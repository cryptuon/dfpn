//! Task management for the worker

use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::client::{AvailableTask, DfpnClient};
use crate::commit_reveal::CommitReveal;
use crate::config::Config;
use crate::inference::{create_detector, AnalysisResult, Detector};

/// Maximum retry attempts for RPC operations
const MAX_RETRIES: u32 = 3;

/// Base delay for exponential backoff (in milliseconds)
const BASE_RETRY_DELAY_MS: u64 = 1000;

/// Pending reveal data
struct PendingReveal {
    result: AnalysisResult,
    salt: [u8; 16],
    commit_deadline: i64,
    model_id: String,
    model_pubkey: Pubkey,
    retry_count: u32,
}

/// Task manager handles polling, execution, and submission
pub struct TaskManager {
    config: Config,
    client: DfpnClient,
    pending_reveals: HashMap<String, PendingReveal>,
    detectors: HashMap<String, Box<dyn Detector>>,
    model_pubkeys: HashMap<String, Pubkey>,
    semaphore: Arc<Semaphore>,
}

impl TaskManager {
    /// Create a new task manager
    pub fn new(config: Config, client: DfpnClient) -> Self {
        let max_concurrent = config.worker.max_concurrent;
        Self {
            config,
            client,
            pending_reveals: HashMap::new(),
            detectors: HashMap::new(),
            model_pubkeys: HashMap::new(),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    /// Initialize detectors and model pubkeys from configuration
    fn init_detectors(&mut self) -> Result<()> {
        for model_config in &self.config.models {
            // Create detector
            let detector = create_detector(&self.config, &model_config.id)?;
            self.detectors.insert(model_config.id.clone(), detector);

            // Parse model pubkey from config (if available)
            if let Some(ref pubkey_str) = model_config.on_chain_id {
                if let Ok(pubkey) = pubkey_str.parse::<Pubkey>() {
                    self.model_pubkeys.insert(model_config.id.clone(), pubkey);
                }
            }
        }
        Ok(())
    }

    /// Get model pubkey for a given model ID
    fn get_model_pubkey(&self, model_id: &str) -> Pubkey {
        self.model_pubkeys
            .get(model_id)
            .copied()
            .unwrap_or_default()
    }

    /// Main worker loop
    pub async fn run(&mut self) -> Result<()> {
        // Initialize detectors
        self.init_detectors()?;

        let poll_interval = Duration::from_millis(self.config.worker.poll_interval_ms);

        info!("Starting task manager main loop");
        info!(
            "Worker configured with {} max concurrent tasks",
            self.config.worker.max_concurrent
        );

        loop {
            // Process pending reveals first (time-critical)
            if let Err(e) = self.process_pending_reveals().await {
                error!("Error processing reveals: {}", e);
            }

            // Check if we have capacity for new tasks
            let available_permits = self.semaphore.available_permits();
            if available_permits > 0 {
                // Poll for new tasks
                match self.poll_tasks().await {
                    Ok(tasks) => {
                        for task in tasks.into_iter().take(available_permits) {
                            // Clone references for spawned task
                            let request_id = task.request_id.clone();

                            if let Err(e) = self.process_task(task).await {
                                error!("Failed to process task {}: {}", request_id, e);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to poll tasks: {}", e);
                    }
                }
            }

            // Sleep before next poll
            sleep(poll_interval).await;
        }
    }

    /// Poll for available tasks
    async fn poll_tasks(&self) -> Result<Vec<AvailableTask>> {
        let capacity = self.semaphore.available_permits();
        self.client.get_available_tasks(capacity).await
    }

    /// Process a single task with retry logic
    async fn process_task(&mut self, task: AvailableTask) -> Result<()> {
        let _permit = self.semaphore.acquire().await?;
        info!("Processing task: {}", task.request_id);

        // Check deadline before starting
        let current_time = get_current_timestamp()?;
        if current_time >= task.commit_deadline {
            warn!(
                "Task {} commit deadline has passed, skipping",
                task.request_id
            );
            return Ok(());
        }

        // 1. Fetch media from storage URI with retry
        let media_path = retry_async(
            || self.fetch_media(&task),
            MAX_RETRIES,
            "fetch_media",
        )
        .await?;

        // 2. Verify content hash
        self.verify_content_hash(&media_path, &task.content_hash_bytes())?;

        // 3. Select appropriate model
        let model_id = self.select_model(task.modalities)?;
        let model_pubkey = self.get_model_pubkey(&model_id);

        // 4. Run inference
        let detector = self
            .detectors
            .get(&model_id)
            .ok_or_else(|| anyhow::anyhow!("Model {} not found", model_id))?;

        let result = detector.analyze(&media_path).await?;

        info!(
            "Inference complete for {}: verdict={:?}, confidence={}",
            task.request_id, result.verdict, result.confidence
        );

        // 5. Generate salt and compute commitment
        let salt = CommitReveal::generate_salt();
        let commitment = CommitReveal::compute_commitment(
            &result,
            &salt,
            &self.client.pubkey(),
            &task.request_id.parse()?,
        );

        // 6. Submit commit with retry
        let request_pubkey = task.request_id.parse()?;
        retry_async(
            || async { self.client.submit_commit(&request_pubkey, commitment).await },
            MAX_RETRIES,
            "submit_commit",
        )
        .await?;

        info!("Commit submitted for task {}", task.request_id);

        // 7. Store for reveal phase
        self.pending_reveals.insert(
            task.request_id.clone(),
            PendingReveal {
                result,
                salt,
                commit_deadline: task.commit_deadline,
                model_id,
                model_pubkey,
                retry_count: 0,
            },
        );

        // 8. Cleanup media file
        self.cleanup_media(&media_path)?;

        Ok(())
    }

    /// Process reveals for committed tasks past commit deadline
    async fn process_pending_reveals(&mut self) -> Result<()> {
        let current_time = get_current_timestamp()?;

        let mut to_remove = Vec::new();
        let mut failed_reveals: Vec<(String, u32)> = Vec::new();

        for (request_id, pending) in &self.pending_reveals {
            // Only reveal after commit window closes
            if current_time >= pending.commit_deadline {
                debug!("Revealing result for request {}", request_id);

                let request_pubkey: Pubkey = match request_id.parse() {
                    Ok(pk) => pk,
                    Err(e) => {
                        error!("Invalid request pubkey {}: {}", request_id, e);
                        to_remove.push(request_id.clone());
                        continue;
                    }
                };

                match self
                    .client
                    .submit_reveal(
                        &request_pubkey,
                        pending.result.verdict as u8,
                        pending.result.confidence,
                        pending.result.detections_hash,
                        pending.salt,
                        &pending.model_pubkey,
                    )
                    .await
                {
                    Ok(_) => {
                        info!("Successfully revealed result for {}", request_id);
                        to_remove.push(request_id.clone());
                    }
                    Err(e) => {
                        let new_retry_count = pending.retry_count + 1;
                        if new_retry_count >= MAX_RETRIES {
                            error!(
                                "Failed to reveal for {} after {} attempts: {}",
                                request_id, MAX_RETRIES, e
                            );
                            to_remove.push(request_id.clone());
                        } else {
                            warn!(
                                "Reveal failed for {} (attempt {}/{}): {}",
                                request_id, new_retry_count, MAX_RETRIES, e
                            );
                            failed_reveals.push((request_id.clone(), new_retry_count));
                        }
                    }
                }
            }
        }

        // Update retry counts for failed reveals
        for (request_id, retry_count) in failed_reveals {
            if let Some(pending) = self.pending_reveals.get_mut(&request_id) {
                pending.retry_count = retry_count;
            }
        }

        // Remove completed or failed reveals
        for request_id in to_remove {
            self.pending_reveals.remove(&request_id);
        }

        Ok(())
    }

    /// Fetch media from storage URI
    async fn fetch_media(&self, task: &AvailableTask) -> Result<std::path::PathBuf> {
        let temp_dir = std::path::Path::new(&self.config.storage.temp_dir);
        std::fs::create_dir_all(temp_dir)?;

        let file_name = format!("{}.media", task.request_id);
        let file_path = temp_dir.join(&file_name);

        // Download media
        let response = reqwest::get(&task.storage_uri).await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch media: HTTP {}", response.status());
        }

        let bytes = response.bytes().await?;

        // Check file size
        let size_mb = bytes.len() as u64 / (1024 * 1024);
        if size_mb > self.config.storage.max_file_size_mb {
            anyhow::bail!(
                "File size {} MB exceeds maximum {} MB",
                size_mb,
                self.config.storage.max_file_size_mb
            );
        }

        std::fs::write(&file_path, bytes)?;
        debug!("Downloaded media to {:?} ({} bytes)", file_path, size_mb);

        Ok(file_path)
    }

    /// Verify content hash matches
    fn verify_content_hash(
        &self,
        path: &std::path::Path,
        expected_hash: &[u8; 32],
    ) -> Result<()> {
        use sha2::{Digest, Sha256};

        let data = std::fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let computed_hash: [u8; 32] = hasher.finalize().into();

        if &computed_hash != expected_hash {
            anyhow::bail!(
                "Content hash mismatch: expected {}, got {}",
                hex_encode(expected_hash),
                hex_encode(&computed_hash)
            );
        }

        debug!("Content hash verified: {}", hex_encode(expected_hash));
        Ok(())
    }

    /// Select appropriate model for given modalities
    fn select_model(&self, modalities: u8) -> Result<String> {
        for model_config in &self.config.models {
            let model_modalities = parse_modality_bits(&model_config.modalities);
            if model_modalities & modalities != 0 {
                return Ok(model_config.id.clone());
            }
        }

        anyhow::bail!("No model found for modalities {}", modalities)
    }

    /// Cleanup media file after processing
    fn cleanup_media(&self, path: &std::path::Path) -> Result<()> {
        if path.exists() {
            std::fs::remove_file(path)?;
            debug!("Cleaned up media file: {:?}", path);
        }
        Ok(())
    }
}

/// Get current Unix timestamp
fn get_current_timestamp() -> Result<i64> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64)
}

/// Retry an async operation with exponential backoff
async fn retry_async<F, Fut, T>(
    operation: F,
    max_retries: u32,
    operation_name: &str,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut last_error = None;

    for attempt in 0..max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                let delay = BASE_RETRY_DELAY_MS * 2u64.pow(attempt);
                warn!(
                    "{} failed (attempt {}/{}): {}. Retrying in {}ms",
                    operation_name,
                    attempt + 1,
                    max_retries,
                    e,
                    delay
                );
                last_error = Some(e);
                sleep(Duration::from_millis(delay)).await;
            }
        }
    }

    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("{} failed after {} retries", operation_name, max_retries)))
}

/// Encode bytes as hex string
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Parse modality strings to bits
fn parse_modality_bits(modalities: &[String]) -> u8 {
    let mut bits: u8 = 0;
    for modality in modalities {
        match modality.to_lowercase().as_str() {
            "image" | "image_authenticity" => bits |= 1 << 0,
            "video" | "video_authenticity" => bits |= 1 << 1,
            "audio" | "audio_authenticity" => bits |= 1 << 2,
            "face" | "face_manipulation" => bits |= 1 << 3,
            "voice" | "voice_cloning" => bits |= 1 << 4,
            "generated" | "generated_content" => bits |= 1 << 5,
            _ => {}
        }
    }
    bits
}
