//! DFPN Indexer - Main entry point

use anyhow::Result;
use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use dfpn_indexer::{
    api::{create_router, AppState},
    indexer::DfpnIndexer,
    schema::{build_model_schema, build_request_schema, build_worker_schema},
    subscriber::{EventSubscriber, ProgramIds},
};

/// DFPN Protocol Indexer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Solana RPC URL
    #[arg(short, long, default_value = "http://localhost:8899")]
    rpc_url: String,

    /// Path to store Tantivy indexes
    #[arg(short, long, default_value = "./data/indexes")]
    index_path: PathBuf,

    /// API server bind address
    #[arg(short, long, default_value = "127.0.0.1:3030")]
    bind: SocketAddr,

    /// Content Registry program ID
    #[arg(long)]
    content_registry: Option<String>,

    /// Analysis Marketplace program ID
    #[arg(long)]
    analysis_marketplace: Option<String>,

    /// Model Registry program ID
    #[arg(long)]
    model_registry: Option<String>,

    /// Worker Registry program ID
    #[arg(long)]
    worker_registry: Option<String>,

    /// Rewards program ID
    #[arg(long)]
    rewards: Option<String>,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    let log_level = match args.log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting DFPN Indexer");
    info!("RPC URL: {}", args.rpc_url);
    info!("Index path: {}", args.index_path.display());
    info!("API bind address: {}", args.bind);

    // Build program IDs
    let program_ids = build_program_ids(&args)?;

    // Create event channel
    let (event_tx, event_rx) = mpsc::channel(1000);

    // Create indexer
    let mut indexer = DfpnIndexer::new(&args.index_path, &args.rpc_url, event_rx)?;

    // Create shared state for API
    let (request_schema, request_fields) = build_request_schema();
    let (worker_schema, worker_fields) = build_worker_schema();
    let (model_schema, model_fields) = build_model_schema();

    // Open indexes for API queries
    let request_path = args.index_path.join("requests");
    let worker_path = args.index_path.join("workers");
    let model_path = args.index_path.join("models");

    std::fs::create_dir_all(&request_path)?;
    std::fs::create_dir_all(&worker_path)?;
    std::fs::create_dir_all(&model_path)?;

    let request_index = tantivy::Index::open_or_create(
        tantivy::directory::MmapDirectory::open(&request_path)?,
        request_schema,
    )?;
    let worker_index = tantivy::Index::open_or_create(
        tantivy::directory::MmapDirectory::open(&worker_path)?,
        worker_schema,
    )?;
    let model_index = tantivy::Index::open_or_create(
        tantivy::directory::MmapDirectory::open(&model_path)?,
        model_schema,
    )?;

    let app_state = Arc::new(RwLock::new(AppState {
        request_index,
        worker_index,
        model_index,
        request_fields,
        worker_fields,
        model_fields,
    }));

    // Create API router
    let app = create_router(app_state)
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Create event subscriber
    let mut subscriber = EventSubscriber::new(&args.rpc_url, program_ids, event_tx);

    // Spawn indexer task
    let indexer_handle = tokio::spawn(async move {
        if let Err(e) = indexer.run().await {
            tracing::error!("Indexer error: {}", e);
        }
    });

    // Spawn subscriber task
    let subscriber_handle = tokio::spawn(async move {
        if let Err(e) = subscriber.run().await {
            tracing::error!("Subscriber error: {}", e);
        }
    });

    // Start API server
    info!("Starting API server on {}", args.bind);
    let listener = tokio::net::TcpListener::bind(args.bind).await?;
    axum::serve(listener, app).await?;

    // Wait for background tasks (they run forever)
    indexer_handle.await?;
    subscriber_handle.await?;

    Ok(())
}

fn build_program_ids(args: &Args) -> Result<ProgramIds> {
    use std::str::FromStr;
    use solana_sdk::pubkey::Pubkey;

    let mut ids = ProgramIds::default();

    if let Some(id) = &args.content_registry {
        ids.content_registry = Pubkey::from_str(id)?;
    }
    if let Some(id) = &args.analysis_marketplace {
        ids.analysis_marketplace = Pubkey::from_str(id)?;
    }
    if let Some(id) = &args.model_registry {
        ids.model_registry = Pubkey::from_str(id)?;
    }
    if let Some(id) = &args.worker_registry {
        ids.worker_registry = Pubkey::from_str(id)?;
    }
    if let Some(id) = &args.rewards {
        ids.rewards = Pubkey::from_str(id)?;
    }

    Ok(ids)
}
