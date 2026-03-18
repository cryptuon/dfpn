//! DFPN Worker Client
//!
//! Reference implementation for node operators running deepfake detection
//! models on their own infrastructure.

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod client;
mod task;
mod inference;
mod commit_reveal;

use config::Config;

#[derive(Parser)]
#[command(name = "dfpn-worker")]
#[command(about = "DFPN Worker Client - Run deepfake detection for the DFPN network")]
#[command(version)]
struct Cli {
    /// Path to configuration file
    #[arg(short, long, default_value = "config.yaml")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the worker daemon
    Start,
    /// Register as a new worker
    Register {
        /// Initial stake amount in DFPN
        #[arg(long)]
        stake: u64,
        /// Supported modalities (comma-separated)
        #[arg(long)]
        modalities: String,
    },
    /// Check worker status
    Status,
    /// View pending tasks
    Tasks {
        /// Maximum number of tasks to show
        #[arg(long, default_value = "10")]
        limit: usize,
    },
    /// View earned rewards
    Rewards,
    /// Claim accumulated rewards
    ClaimRewards,
    /// Update worker configuration
    Update {
        /// New modalities (comma-separated)
        #[arg(long)]
        modalities: Option<String>,
        /// Additional stake to add
        #[arg(long)]
        add_stake: Option<u64>,
    },
    /// Request stake withdrawal
    Unstake {
        /// Amount to unstake
        #[arg(long)]
        amount: u64,
    },
    /// Withdraw unstaked tokens
    Withdraw,
    /// Test a model locally
    TestModel {
        /// Model identifier
        #[arg(long)]
        model: String,
        /// Path to test media file
        #[arg(long)]
        input: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();

    // Load configuration
    let config = Config::load(&cli.config)?;

    match cli.command {
        Commands::Start => {
            info!("Starting DFPN worker...");
            start_worker(config).await?;
        }
        Commands::Register { stake, modalities } => {
            info!("Registering worker with stake {} DFPN", stake);
            register_worker(config, stake, &modalities).await?;
        }
        Commands::Status => {
            show_status(config).await?;
        }
        Commands::Tasks { limit } => {
            show_tasks(config, limit).await?;
        }
        Commands::Rewards => {
            show_rewards(config).await?;
        }
        Commands::ClaimRewards => {
            claim_rewards(config).await?;
        }
        Commands::Update { modalities, add_stake } => {
            update_worker(config, modalities, add_stake).await?;
        }
        Commands::Unstake { amount } => {
            request_unstake(config, amount).await?;
        }
        Commands::Withdraw => {
            withdraw_stake(config).await?;
        }
        Commands::TestModel { model, input } => {
            test_model(config, &model, &input).await?;
        }
    }

    Ok(())
}

async fn start_worker(config: Config) -> Result<()> {
    info!("Worker configuration loaded");
    info!("  Network: {:?}", config.network);
    info!("  RPC URL: {}", config.rpc_url);
    info!("  Modalities: {:?}", config.worker.modalities);
    info!("  Max concurrent tasks: {}", config.worker.max_concurrent);

    // Initialize client
    let client = client::DfpnClient::new(&config)?;

    // Main worker loop
    let mut task_manager = task::TaskManager::new(config.clone(), client);
    task_manager.run().await?;

    Ok(())
}

async fn register_worker(config: Config, stake: u64, modalities: &str) -> Result<()> {
    let client = client::DfpnClient::new(&config)?;
    let modality_bits = parse_modalities(modalities)?;

    info!("Registering worker...");
    info!("  Stake: {} DFPN", stake);
    info!("  Modalities: {}", modalities);

    client.register_worker(stake, modality_bits).await?;
    info!("Worker registered successfully!");

    Ok(())
}

async fn show_status(config: Config) -> Result<()> {
    let client = client::DfpnClient::new(&config)?;
    let status = client.get_worker_status().await?;

    println!("Worker Status:");
    println!("  Operator: {}", status.operator);
    println!("  Stake: {} DFPN", status.stake);
    println!("  Reputation: {}%", status.reputation_score as f64 / 100.0);
    println!("  Status: {:?}", status.status);
    println!("  Tasks completed: {}", status.tasks_completed);
    println!("  Tasks failed: {}", status.tasks_failed);

    Ok(())
}

async fn show_tasks(config: Config, limit: usize) -> Result<()> {
    let client = client::DfpnClient::new(&config)?;
    let tasks = client.get_available_tasks(limit).await?;

    println!("Available Tasks ({}):", tasks.len());
    for task in tasks {
        println!("  Request: {}", task.request_id);
        println!("    Fee: {} DFPN", task.fee_amount);
        println!("    Deadline: {}", task.deadline);
        println!("    Modalities: {:?}", task.modalities);
        println!();
    }

    Ok(())
}

async fn show_rewards(config: Config) -> Result<()> {
    let client = client::DfpnClient::new(&config)?;
    let rewards = client.get_pending_rewards().await?;

    println!("Rewards:");
    println!("  Pending: {} DFPN", rewards.pending);
    println!("  Total claimed: {} DFPN", rewards.total_claimed);

    Ok(())
}

async fn claim_rewards(config: Config) -> Result<()> {
    let client = client::DfpnClient::new(&config)?;
    let amount = client.claim_rewards().await?;

    info!("Claimed {} DFPN in rewards", amount);

    Ok(())
}

async fn update_worker(
    config: Config,
    modalities: Option<String>,
    add_stake: Option<u64>,
) -> Result<()> {
    let client = client::DfpnClient::new(&config)?;

    let modality_bits = modalities.map(|m| parse_modalities(&m)).transpose()?;

    client.update_worker(modality_bits, add_stake).await?;
    info!("Worker updated successfully");

    Ok(())
}

async fn request_unstake(config: Config, amount: u64) -> Result<()> {
    let client = client::DfpnClient::new(&config)?;
    client.request_unstake(amount).await?;

    info!("Unstake request submitted for {} DFPN", amount);
    info!("Tokens will be available after the unbonding period");

    Ok(())
}

async fn withdraw_stake(config: Config) -> Result<()> {
    let client = client::DfpnClient::new(&config)?;
    let amount = client.withdraw_stake().await?;

    info!("Withdrew {} DFPN", amount);

    Ok(())
}

async fn test_model(config: Config, model: &str, input: &str) -> Result<()> {
    info!("Testing model {} with input {}", model, input);

    let detector = inference::create_detector(&config, model)?;
    let result = detector.analyze(std::path::Path::new(input)).await?;

    println!("Analysis Result:");
    println!("  Verdict: {:?}", result.verdict);
    println!("  Confidence: {}%", result.confidence);
    println!("  Detections: {}", result.detections.len());
    for detection in &result.detections {
        println!("    - {}: {}%", detection.detection_type, detection.confidence);
    }

    Ok(())
}

fn parse_modalities(input: &str) -> Result<u8> {
    let mut bits: u8 = 0;

    for modality in input.split(',') {
        match modality.trim().to_lowercase().as_str() {
            "image" | "image_authenticity" => bits |= 1 << 0,
            "video" | "video_authenticity" => bits |= 1 << 1,
            "audio" | "audio_authenticity" => bits |= 1 << 2,
            "face" | "face_manipulation" => bits |= 1 << 3,
            "voice" | "voice_cloning" => bits |= 1 << 4,
            "generated" | "generated_content" => bits |= 1 << 5,
            other => anyhow::bail!("Unknown modality: {}", other),
        }
    }

    Ok(bits)
}
