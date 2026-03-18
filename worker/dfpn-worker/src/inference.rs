//! Model inference interface
//!
//! This module provides the detection/inference interface for analyzing media content.
//!
//! ## Detector Types
//!
//! - **ExternalDetector**: Calls an external process (Python script or binary) for inference
//! - **DummyDetector**: Returns dummy results for testing
//!
//! ## Note on ONNX and Candle
//!
//! Native ONNX Runtime and Candle (pure Rust ML) detectors are designed but currently
//! disabled due to dependency version conflicts with the Solana SDK. The recommended
//! approach for production is to use ExternalDetector with a Python inference service
//! or a separate microservice.
//!
//! When dependency conflicts are resolved upstream, the ONNX and Candle implementations
//! can be re-enabled by uncommenting the relevant dependencies in Cargo.toml.

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;

use crate::config::Config;

/// Verdict from analysis
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Verdict {
    Authentic = 0,
    Manipulated = 1,
    Inconclusive = 2,
}

/// Individual detection finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    /// Type of detection (e.g., "face_swap", "audio_splice")
    pub detection_type: String,
    /// Confidence score (0-100)
    pub confidence: u8,
    /// Optional region (bounding box or time range)
    pub region: Option<Region>,
}

/// Region specification for detections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    /// For images/video: x coordinate
    pub x: Option<u32>,
    /// For images/video: y coordinate
    pub y: Option<u32>,
    /// For images/video: width
    pub width: Option<u32>,
    /// For images/video: height
    pub height: Option<u32>,
    /// For audio/video: start time in milliseconds
    pub start_ms: Option<u64>,
    /// For audio/video: end time in milliseconds
    pub end_ms: Option<u64>,
}

/// Analysis result from a detector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Overall verdict
    pub verdict: Verdict,
    /// Confidence score (0-100)
    pub confidence: u8,
    /// Individual detections
    pub detections: Vec<Detection>,
    /// Hash of detailed detection data
    pub detections_hash: [u8; 32],
    /// Processing time in milliseconds
    pub processing_time_ms: u32,
}

impl AnalysisResult {
    /// Compute hash of detections for on-chain storage
    pub fn compute_detections_hash(detections: &[Detection]) -> [u8; 32] {
        let serialized = serde_json::to_vec(detections).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        hasher.finalize().into()
    }
}

/// Trait for detection models
#[async_trait]
pub trait Detector: Send + Sync {
    /// Analyze media and return result
    async fn analyze(&self, media_path: &Path) -> Result<AnalysisResult>;

    /// Get supported modalities
    fn supported_modalities(&self) -> Vec<String>;

    /// Get model identifier
    fn model_id(&self) -> &str;
}

// ============================================================================
// External Process-based Detector
// ============================================================================

/// External process-based detector
///
/// Calls an external command (e.g., Python script) to run inference.
/// The external command should:
/// 1. Accept the media file path as the last argument
/// 2. Output JSON to stdout in the format:
///    ```json
///    {
///      "verdict": "authentic" | "manipulated" | "inconclusive",
///      "confidence": 0-100,
///      "detections": [
///        {
///          "detection_type": "face_swap",
///          "confidence": 95,
///          "region": { "x": 100, "y": 200, "width": 50, "height": 50 }
///        }
///      ]
///    }
///    ```
pub struct ExternalDetector {
    model_id: String,
    command: String,
    args: Vec<String>,
    modalities: Vec<String>,
}

impl ExternalDetector {
    pub fn new(
        model_id: String,
        command: String,
        args: Vec<String>,
        modalities: Vec<String>,
    ) -> Self {
        Self {
            model_id,
            command,
            args,
            modalities,
        }
    }
}

#[async_trait]
impl Detector for ExternalDetector {
    async fn analyze(&self, media_path: &Path) -> Result<AnalysisResult> {
        use std::process::Command;
        use std::time::Instant;

        let start = Instant::now();

        // Build command with media path as argument
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args);
        cmd.arg(media_path);

        // Execute and capture output
        let output = cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Detector command failed: {}", stderr);
        }

        // Parse JSON output
        let result: ExternalResult = serde_json::from_slice(&output.stdout)?;

        let processing_time_ms = start.elapsed().as_millis() as u32;

        let detections = result
            .detections
            .into_iter()
            .map(|d| Detection {
                detection_type: d.detection_type,
                confidence: d.confidence,
                region: d.region,
            })
            .collect::<Vec<_>>();

        let detections_hash = AnalysisResult::compute_detections_hash(&detections);

        Ok(AnalysisResult {
            verdict: match result.verdict.to_lowercase().as_str() {
                "authentic" => Verdict::Authentic,
                "manipulated" => Verdict::Manipulated,
                _ => Verdict::Inconclusive,
            },
            confidence: result.confidence,
            detections,
            detections_hash,
            processing_time_ms,
        })
    }

    fn supported_modalities(&self) -> Vec<String> {
        self.modalities.clone()
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }
}

/// Expected output format from external detector
#[derive(Debug, Deserialize)]
struct ExternalResult {
    verdict: String,
    confidence: u8,
    detections: Vec<ExternalDetection>,
}

#[derive(Debug, Deserialize)]
struct ExternalDetection {
    detection_type: String,
    confidence: u8,
    region: Option<Region>,
}

// ============================================================================
// HTTP-based Detector (for microservice inference)
// ============================================================================

/// HTTP-based detector that calls a REST API for inference
///
/// This is useful for running inference in a separate microservice that can
/// have different dependencies than the Solana SDK.
pub struct HttpDetector {
    model_id: String,
    endpoint: String,
    modalities: Vec<String>,
    timeout_ms: u64,
}

impl HttpDetector {
    pub fn new(
        model_id: String,
        endpoint: String,
        modalities: Vec<String>,
        timeout_ms: u64,
    ) -> Self {
        Self {
            model_id,
            endpoint,
            modalities,
            timeout_ms,
        }
    }
}

#[async_trait]
impl Detector for HttpDetector {
    async fn analyze(&self, media_path: &Path) -> Result<AnalysisResult> {
        use std::time::Instant;

        let start = Instant::now();

        // Read media file
        let media_bytes = std::fs::read(media_path)?;
        let file_name = media_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("media");

        // Create multipart form
        let part = reqwest::multipart::Part::bytes(media_bytes)
            .file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new().part("file", part);

        // Send request
        let client = reqwest::Client::new();
        let response = client
            .post(&self.endpoint)
            .multipart(form)
            .timeout(std::time::Duration::from_millis(self.timeout_ms))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Inference service returned error: {}",
                response.status()
            );
        }

        // Parse response
        let result: ExternalResult = response.json().await?;

        let processing_time_ms = start.elapsed().as_millis() as u32;

        let detections = result
            .detections
            .into_iter()
            .map(|d| Detection {
                detection_type: d.detection_type,
                confidence: d.confidence,
                region: d.region,
            })
            .collect::<Vec<_>>();

        let detections_hash = AnalysisResult::compute_detections_hash(&detections);

        Ok(AnalysisResult {
            verdict: match result.verdict.to_lowercase().as_str() {
                "authentic" => Verdict::Authentic,
                "manipulated" => Verdict::Manipulated,
                _ => Verdict::Inconclusive,
            },
            confidence: result.confidence,
            detections,
            detections_hash,
            processing_time_ms,
        })
    }

    fn supported_modalities(&self) -> Vec<String> {
        self.modalities.clone()
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }
}

// ============================================================================
// Dummy Detector (for testing)
// ============================================================================

/// Dummy detector for testing
pub struct DummyDetector {
    model_id: String,
    modalities: Vec<String>,
}

impl DummyDetector {
    pub fn new(model_id: String, modalities: Vec<String>) -> Self {
        Self {
            model_id,
            modalities,
        }
    }
}

#[async_trait]
impl Detector for DummyDetector {
    async fn analyze(&self, _media_path: &Path) -> Result<AnalysisResult> {
        // Return a dummy result for testing
        let detections = vec![Detection {
            detection_type: "test_detection".to_string(),
            confidence: 75,
            region: None,
        }];

        let detections_hash = AnalysisResult::compute_detections_hash(&detections);

        Ok(AnalysisResult {
            verdict: Verdict::Inconclusive,
            confidence: 50,
            detections,
            detections_hash,
            processing_time_ms: 100,
        })
    }

    fn supported_modalities(&self) -> Vec<String> {
        self.modalities.clone()
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }
}

// ============================================================================
// Detector Factory
// ============================================================================

/// Create a detector from configuration
pub fn create_detector(config: &Config, model_id: &str) -> Result<Box<dyn Detector>> {
    // Find model config
    let model_config = config
        .models
        .iter()
        .find(|m| m.id == model_id)
        .ok_or_else(|| anyhow::anyhow!("Model {} not found in configuration", model_id))?;

    let model_path = Path::new(&model_config.path);

    // Select detector based on runtime configuration
    match model_config.runtime.as_str() {
        "http" => {
            // HTTP-based inference service
            tracing::info!(
                "Using HTTP detector for model {} at {}",
                model_id,
                model_config.path
            );
            return Ok(Box::new(HttpDetector::new(
                model_id.to_string(),
                model_config.path.clone(), // Path is actually the endpoint URL
                model_config.modalities.clone(),
                30000, // 30 second timeout
            )));
        }

        "external" | _ => {
            if model_path.exists() {
                // Look for inference script
                let inference_script = model_path.join("inference.py");
                if inference_script.exists() {
                    tracing::info!(
                        "Using external Python detector from {:?}",
                        inference_script
                    );
                    return Ok(Box::new(ExternalDetector::new(
                        model_id.to_string(),
                        "python".to_string(),
                        vec![inference_script.to_string_lossy().to_string()],
                        model_config.modalities.clone(),
                    )));
                }

                // Look for executable
                let executable = model_path.join("detect");
                if executable.exists() {
                    tracing::info!("Using external detector from {:?}", executable);
                    return Ok(Box::new(ExternalDetector::new(
                        model_id.to_string(),
                        executable.to_string_lossy().to_string(),
                        vec![],
                        model_config.modalities.clone(),
                    )));
                }
            }
            tracing::warn!(
                "External detector not found at {:?}, falling back to dummy",
                model_path
            );
        }
    }

    // Fall back to dummy detector
    tracing::warn!(
        "Using dummy detector for model {} (runtime: {})",
        model_id,
        model_config.runtime
    );
    Ok(Box::new(DummyDetector::new(
        model_id.to_string(),
        model_config.modalities.clone(),
    )))
}

// ============================================================================
// Example Python Inference Script
// ============================================================================

/// Example Python inference script that can be used with ExternalDetector:
///
/// ```python
/// #!/usr/bin/env python3
/// """
/// Example deepfake detection inference script.
///
/// Usage: python inference.py <media_path>
/// Output: JSON to stdout
/// """
/// import sys
/// import json
/// import torch
/// from PIL import Image
/// from torchvision import transforms
///
/// def load_model():
///     # Load your trained model here
///     model = torch.jit.load("model.pt")
///     model.eval()
///     return model
///
/// def preprocess(image_path):
///     transform = transforms.Compose([
///         transforms.Resize((224, 224)),
///         transforms.ToTensor(),
///         transforms.Normalize(
///             mean=[0.485, 0.456, 0.406],
///             std=[0.229, 0.224, 0.225]
///         )
///     ])
///     img = Image.open(image_path).convert('RGB')
///     return transform(img).unsqueeze(0)
///
/// def main():
///     if len(sys.argv) != 2:
///         print("Usage: python inference.py <media_path>", file=sys.stderr)
///         sys.exit(1)
///
///     media_path = sys.argv[1]
///     model = load_model()
///     input_tensor = preprocess(media_path)
///
///     with torch.no_grad():
///         output = model(input_tensor)
///         prob = torch.softmax(output, dim=1)
///         authentic_prob = prob[0][0].item()
///         manipulated_prob = prob[0][1].item()
///
///     if manipulated_prob > 0.5:
///         verdict = "manipulated"
///         confidence = int(manipulated_prob * 100)
///     elif authentic_prob > 0.5:
///         verdict = "authentic"
///         confidence = int(authentic_prob * 100)
///     else:
///         verdict = "inconclusive"
///         confidence = 50
///
///     result = {
///         "verdict": verdict,
///         "confidence": confidence,
///         "detections": [
///             {
///                 "detection_type": "deepfake_detection",
///                 "confidence": confidence,
///                 "region": None
///             }
///         ]
///     }
///
///     print(json.dumps(result))
///
/// if __name__ == "__main__":
///     main()
/// ```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verdict_serialization() {
        assert_eq!(Verdict::Authentic as u8, 0);
        assert_eq!(Verdict::Manipulated as u8, 1);
        assert_eq!(Verdict::Inconclusive as u8, 2);
    }

    #[test]
    fn test_detections_hash() {
        let detections = vec![Detection {
            detection_type: "test".to_string(),
            confidence: 50,
            region: None,
        }];

        let hash1 = AnalysisResult::compute_detections_hash(&detections);
        let hash2 = AnalysisResult::compute_detections_hash(&detections);

        assert_eq!(hash1, hash2);
    }
}
