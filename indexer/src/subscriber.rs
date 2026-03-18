//! Solana event subscriber for DFPN programs

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::EncodedTransactionWithStatusMeta;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::indexer::IndexEvent;

/// Program IDs to monitor
pub struct ProgramIds {
    pub content_registry: Pubkey,
    pub analysis_marketplace: Pubkey,
    pub model_registry: Pubkey,
    pub worker_registry: Pubkey,
    pub rewards: Pubkey,
}

impl Default for ProgramIds {
    fn default() -> Self {
        // Default program IDs (can be overridden in config)
        Self {
            content_registry: Pubkey::from_str("DFPNcontent111111111111111111111111111111111").unwrap(),
            analysis_marketplace: Pubkey::from_str("DFPNmarket1111111111111111111111111111111111").unwrap(),
            model_registry: Pubkey::from_str("DFPNmodel11111111111111111111111111111111111").unwrap(),
            worker_registry: Pubkey::from_str("DFPNworker1111111111111111111111111111111111").unwrap(),
            rewards: Pubkey::from_str("DFPNrewards111111111111111111111111111111111").unwrap(),
        }
    }
}

/// Event subscriber that watches Solana for DFPN program events
pub struct EventSubscriber {
    rpc_client: Arc<RpcClient>,
    program_ids: ProgramIds,
    event_tx: mpsc::Sender<IndexEvent>,
    last_slot: u64,
}

impl EventSubscriber {
    /// Create a new event subscriber
    pub fn new(
        rpc_url: &str,
        program_ids: ProgramIds,
        event_tx: mpsc::Sender<IndexEvent>,
    ) -> Self {
        let rpc_client = Arc::new(RpcClient::new(rpc_url.to_string()));

        Self {
            rpc_client,
            program_ids,
            event_tx,
            last_slot: 0,
        }
    }

    /// Start the subscription loop
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting event subscriber");

        // Get initial slot
        self.last_slot = self.rpc_client.get_slot()?;
        info!("Starting from slot {}", self.last_slot);

        loop {
            if let Err(e) = self.poll_new_blocks().await {
                error!("Error polling blocks: {}", e);
            }

            // Poll every 400ms (Solana slot time)
            tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
        }
    }

    /// Poll for new blocks and process transactions
    async fn poll_new_blocks(&mut self) -> Result<()> {
        let current_slot = self.rpc_client.get_slot()?;

        if current_slot <= self.last_slot {
            return Ok(());
        }

        // Process slots we haven't seen
        for slot in (self.last_slot + 1)..=current_slot {
            if let Err(e) = self.process_slot(slot).await {
                warn!("Error processing slot {}: {}", slot, e);
            }
        }

        self.last_slot = current_slot;
        Ok(())
    }

    /// Process a single slot
    async fn process_slot(&self, slot: u64) -> Result<()> {
        // Get block with full transaction details
        let block = match self.rpc_client.get_block(slot) {
            Ok(block) => block,
            Err(e) => {
                // Slot might be skipped, that's OK
                debug!("Could not get block for slot {}: {}", slot, e);
                return Ok(());
            }
        };

        // Process each transaction
        for tx in block.transactions.iter() {
            if tx.meta.is_some() {
                // Check if transaction involves our programs
                if self.transaction_involves_programs(tx) {
                    self.process_transaction(tx, slot).await?;
                }
            }
        }

        Ok(())
    }

    /// Check if a transaction involves our programs
    fn transaction_involves_programs(
        &self,
        tx: &EncodedTransactionWithStatusMeta,
    ) -> bool {
        // Extract account keys from transaction
        // Try to decode the transaction
        if let Some(decoded) = tx.transaction.decode() {
            let account_keys = decoded.message.static_account_keys();

            let program_ids = [
                &self.program_ids.content_registry,
                &self.program_ids.analysis_marketplace,
                &self.program_ids.model_registry,
                &self.program_ids.worker_registry,
                &self.program_ids.rewards,
            ];

            for key in account_keys {
                if program_ids.contains(&key) {
                    return true;
                }
            }
        }

        false
    }

    /// Process a transaction and emit events
    async fn process_transaction(
        &self,
        tx: &EncodedTransactionWithStatusMeta,
        slot: u64,
    ) -> Result<()> {
        let meta = tx.meta.as_ref().ok_or_else(|| {
            anyhow::anyhow!("Transaction has no meta")
        })?;

        // Parse program logs for events
        // log_messages uses OptionSerializer, we convert to Option
        let log_messages: Option<Vec<String>> = meta.log_messages.clone().into();
        if let Some(logs) = log_messages {
            for log in logs {
                if let Some(event) = self.parse_log_message(&log, slot)? {
                    if let Err(e) = self.event_tx.send(event).await {
                        error!("Failed to send event: {}", e);
                    }
                }
            }
        }

        Ok(())
    }

    /// Parse a log message for Anchor events
    fn parse_log_message(&self, log: &str, slot: u64) -> Result<Option<IndexEvent>> {
        // Anchor events are logged with "Program data:" prefix
        if !log.contains("Program data:") {
            return Ok(None);
        }

        let data_start = log.find("Program data:").unwrap() + 13;
        let data_str = log[data_start..].trim();

        // Decode base64 data
        let data = match base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            data_str,
        ) {
            Ok(data) => data,
            Err(_) => return Ok(None),
        };

        // First 8 bytes are discriminator
        if data.len() < 8 {
            return Ok(None);
        }

        let discriminator = &data[0..8];
        let event_data = &data[8..];

        // Try to parse known events
        self.parse_event(discriminator, event_data, slot)
    }

    /// Parse event based on discriminator
    fn parse_event(
        &self,
        discriminator: &[u8],
        data: &[u8],
        slot: u64,
    ) -> Result<Option<IndexEvent>> {
        // Event discriminators would need to be computed from event names
        // For now, we'll use a simplified approach

        // RequestCreated event
        if discriminator == [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08] {
            return Ok(Some(IndexEvent::RequestCreated {
                request_id: parse_pubkey(data, 0)?,
                slot,
            }));
        }

        // WorkerRegistered event
        if discriminator == [0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18] {
            return Ok(Some(IndexEvent::WorkerRegistered {
                worker_id: parse_pubkey(data, 0)?,
                slot,
            }));
        }

        // ModelRegistered event
        if discriminator == [0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28] {
            return Ok(Some(IndexEvent::ModelRegistered {
                model_id: parse_pubkey(data, 0)?,
                slot,
            }));
        }

        // RequestFinalized event
        if discriminator == [0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38] {
            return Ok(Some(IndexEvent::RequestFinalized {
                request_id: parse_pubkey(data, 0)?,
                slot,
            }));
        }

        Ok(None)
    }
}

/// Parse a pubkey from bytes at offset
fn parse_pubkey(data: &[u8], offset: usize) -> Result<String> {
    if data.len() < offset + 32 {
        anyhow::bail!("Not enough data for pubkey");
    }

    let pubkey_bytes: [u8; 32] = data[offset..offset + 32].try_into()?;
    Ok(Pubkey::new_from_array(pubkey_bytes).to_string())
}
