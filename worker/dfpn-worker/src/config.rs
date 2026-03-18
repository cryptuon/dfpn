//! Worker configuration

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Network to connect to
    pub network: Network,
    /// Solana RPC URL
    pub rpc_url: String,
    /// Indexer API URL
    pub indexer_url: String,
    /// Path to wallet keypair
    pub wallet_path: String,
    /// Worker-specific settings
    pub worker: WorkerConfig,
    /// Model configurations
    pub models: Vec<ModelConfig>,
    /// Inference settings
    pub inference: InferenceConfig,
    /// Storage settings
    pub storage: StorageConfig,
    /// Monitoring settings
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    Devnet,
    Testnet,
    Mainnet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    /// Supported modalities
    pub modalities: Vec<String>,
    /// Minimum fee to accept (in lamports)
    pub min_fee: u64,
    /// Maximum concurrent tasks
    pub max_concurrent: usize,
    /// Task timeout in seconds
    pub task_timeout: u64,
    /// Poll interval in milliseconds
    pub poll_interval_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Model identifier
    pub id: String,
    /// Path to model files
    pub path: String,
    /// Supported modalities
    pub modalities: Vec<String>,
    /// Whether GPU is required
    pub gpu_required: bool,
    /// Runtime to use (onnx, candle, external)
    #[serde(default = "default_runtime")]
    pub runtime: String,
    /// On-chain model account pubkey (optional)
    #[serde(default)]
    pub on_chain_id: Option<String>,
}

fn default_runtime() -> String {
    "external".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    /// Device to use (cuda, cpu)
    pub device: String,
    /// Batch size for inference
    pub batch_size: usize,
    /// Precision (fp32, fp16, int8)
    pub precision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Temporary directory for media files
    pub temp_dir: String,
    /// Maximum file size in MB
    pub max_file_size_mb: u64,
    /// Cleanup after seconds
    pub cleanup_after_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Prometheus metrics port
    pub metrics_port: u16,
    /// Health check port
    pub health_port: u16,
}

impl Config {
    /// Load configuration from a YAML file
    pub fn load(path: &str) -> Result<Self> {
        let path = Path::new(path);

        if path.exists() {
            let contents = std::fs::read_to_string(path)?;
            let config: Config = serde_yaml::from_str(&contents)?;
            Ok(config)
        } else {
            // Return default config if file doesn't exist
            Ok(Self::default())
        }
    }

    /// Save configuration to a YAML file
    pub fn save(&self, path: &str) -> Result<()> {
        let contents = serde_yaml::to_string(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: Network::Devnet,
            rpc_url: "https://api.devnet.solana.com".to_string(),
            indexer_url: "https://indexer.devnet.dfpn.network".to_string(),
            wallet_path: "~/.config/solana/id.json".to_string(),
            worker: WorkerConfig {
                modalities: vec!["image_authenticity".to_string()],
                min_fee: 1_000_000,
                max_concurrent: 4,
                task_timeout: 300,
                poll_interval_ms: 5000,
            },
            models: vec![],
            inference: InferenceConfig {
                device: "cpu".to_string(),
                batch_size: 1,
                precision: "fp32".to_string(),
            },
            storage: StorageConfig {
                temp_dir: "/tmp/dfpn".to_string(),
                max_file_size_mb: 500,
                cleanup_after_seconds: 3600,
            },
            monitoring: MonitoringConfig {
                metrics_port: 9090,
                health_port: 8080,
            },
        }
    }
}
