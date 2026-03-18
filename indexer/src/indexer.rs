//! Tantivy indexer for DFPN events

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use tantivy::{
    directory::MmapDirectory, doc, Index, IndexWriter,
};
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::schema::{
    build_model_schema, build_request_schema, build_worker_schema,
    ModelFields, RequestFields, WorkerFields,
};

/// Events that can be indexed
#[derive(Debug, Clone)]
pub enum IndexEvent {
    RequestCreated { request_id: String, slot: u64 },
    RequestFinalized { request_id: String, slot: u64 },
    WorkerRegistered { worker_id: String, slot: u64 },
    WorkerUpdated { worker_id: String, slot: u64 },
    ModelRegistered { model_id: String, slot: u64 },
    ModelUpdated { model_id: String, slot: u64 },
}

/// DFPN Indexer managing multiple Tantivy indexes
pub struct DfpnIndexer {
    request_index: Index,
    worker_index: Index,
    model_index: Index,
    request_fields: RequestFields,
    worker_fields: WorkerFields,
    model_fields: ModelFields,
    request_writer: IndexWriter,
    worker_writer: IndexWriter,
    model_writer: IndexWriter,
    rpc_client: Arc<RpcClient>,
    event_rx: mpsc::Receiver<IndexEvent>,
}

impl DfpnIndexer {
    /// Create a new indexer
    pub fn new(
        index_path: &Path,
        rpc_url: &str,
        event_rx: mpsc::Receiver<IndexEvent>,
    ) -> Result<Self> {
        // Create index directories
        let request_path = index_path.join("requests");
        let worker_path = index_path.join("workers");
        let model_path = index_path.join("models");

        std::fs::create_dir_all(&request_path)?;
        std::fs::create_dir_all(&worker_path)?;
        std::fs::create_dir_all(&model_path)?;

        // Build schemas
        let (request_schema, request_fields) = build_request_schema();
        let (worker_schema, worker_fields) = build_worker_schema();
        let (model_schema, model_fields) = build_model_schema();

        // Create or open indexes
        let request_index = Index::open_or_create(
            MmapDirectory::open(&request_path)?,
            request_schema,
        )?;
        let worker_index = Index::open_or_create(
            MmapDirectory::open(&worker_path)?,
            worker_schema,
        )?;
        let model_index = Index::open_or_create(
            MmapDirectory::open(&model_path)?,
            model_schema,
        )?;

        // Create writers (50MB heap size)
        let request_writer = request_index.writer(50_000_000)?;
        let worker_writer = worker_index.writer(50_000_000)?;
        let model_writer = model_index.writer(50_000_000)?;

        let rpc_client = Arc::new(RpcClient::new(rpc_url.to_string()));

        Ok(Self {
            request_index,
            worker_index,
            model_index,
            request_fields,
            worker_fields,
            model_fields,
            request_writer,
            worker_writer,
            model_writer,
            rpc_client,
            event_rx,
        })
    }

    /// Run the indexer event loop
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting DFPN indexer");

        while let Some(event) = self.event_rx.recv().await {
            if let Err(e) = self.process_event(event).await {
                error!("Error processing event: {}", e);
            }
        }

        Ok(())
    }

    /// Process a single event
    async fn process_event(&mut self, event: IndexEvent) -> Result<()> {
        match event {
            IndexEvent::RequestCreated { request_id, slot } => {
                self.index_request(&request_id, slot).await?;
            }
            IndexEvent::RequestFinalized { request_id, slot: _ } => {
                self.update_request_status(&request_id, "Finalized").await?;
            }
            IndexEvent::WorkerRegistered { worker_id, slot } => {
                self.index_worker(&worker_id, slot).await?;
            }
            IndexEvent::WorkerUpdated { worker_id, slot } => {
                self.index_worker(&worker_id, slot).await?;
            }
            IndexEvent::ModelRegistered { model_id, slot } => {
                self.index_model(&model_id, slot).await?;
            }
            IndexEvent::ModelUpdated { model_id, slot } => {
                self.index_model(&model_id, slot).await?;
            }
        }

        Ok(())
    }

    /// Index a request by fetching its on-chain data
    async fn index_request(&mut self, request_id: &str, _slot: u64) -> Result<()> {
        let pubkey = Pubkey::from_str(request_id)?;
        let account_info = self.rpc_client.get_account(&pubkey)?;

        // Parse account data (simplified - would need proper borsh deserialization)
        let data = &account_info.data;

        if data.len() < 100 {
            anyhow::bail!("Invalid request account data");
        }

        // Extract fields from account data
        let requester = Pubkey::new_from_array(data[8..40].try_into()?).to_string();
        let content_hash = hex::encode(&data[40..72]);
        let modalities = data[72] as u64;
        let status = match data[73] {
            0 => "Open",
            1 => "CommitClosed",
            2 => "Finalized",
            3 => "Expired",
            4 => "Cancelled",
            5 => "Disputed",
            _ => "Unknown",
        };

        // Create document
        let doc = doc!(
            self.request_fields.id => request_id,
            self.request_fields.requester => requester,
            self.request_fields.content_hash => content_hash,
            self.request_fields.storage_uri => "",
            self.request_fields.modalities => modalities,
            self.request_fields.status => status,
            self.request_fields.fee_amount => 0u64,
            self.request_fields.deadline => 0i64,
            self.request_fields.commit_deadline => 0i64,
            self.request_fields.created_at => chrono::Utc::now().timestamp(),
            self.request_fields.commit_count => 0u64,
            self.request_fields.reveal_count => 0u64
        );

        self.request_writer.add_document(doc)?;
        self.request_writer.commit()?;

        info!("Indexed request {}", request_id);
        Ok(())
    }

    /// Update request status
    async fn update_request_status(&mut self, request_id: &str, _status: &str) -> Result<()> {
        // Delete existing document
        let term = tantivy::Term::from_field_text(self.request_fields.id, request_id);
        self.request_writer.delete_term(term);

        // Re-index with new status (will fetch fresh data from chain)
        self.index_request(request_id, 0).await
    }

    /// Index a worker by fetching its on-chain data
    async fn index_worker(&mut self, worker_id: &str, _slot: u64) -> Result<()> {
        let pubkey = Pubkey::from_str(worker_id)?;
        let account_info = self.rpc_client.get_account(&pubkey)?;

        let data = &account_info.data;

        if data.len() < 80 {
            anyhow::bail!("Invalid worker account data");
        }

        // Extract fields
        let operator = Pubkey::new_from_array(data[8..40].try_into()?).to_string();
        let stake = u64::from_le_bytes(data[40..48].try_into()?);
        let reputation = u32::from_le_bytes(data[48..52].try_into()?) as u64;
        let modalities = data[52] as u64;
        let status = match data[76] {
            0 => "Active",
            1 => "Inactive",
            2 => "Slashed",
            3 => "Unbonding",
            _ => "Unknown",
        };

        let doc = doc!(
            self.worker_fields.id => worker_id,
            self.worker_fields.operator => operator,
            self.worker_fields.stake => stake,
            self.worker_fields.reputation => reputation,
            self.worker_fields.modalities => modalities,
            self.worker_fields.status => status,
            self.worker_fields.tasks_completed => 0u64,
            self.worker_fields.tasks_failed => 0u64,
            self.worker_fields.last_active_slot => 0u64
        );

        // Delete existing and add new
        let term = tantivy::Term::from_field_text(self.worker_fields.id, worker_id);
        self.worker_writer.delete_term(term);
        self.worker_writer.add_document(doc)?;
        self.worker_writer.commit()?;

        info!("Indexed worker {}", worker_id);
        Ok(())
    }

    /// Index a model by fetching its on-chain data
    async fn index_model(&mut self, model_id: &str, _slot: u64) -> Result<()> {
        let pubkey = Pubkey::from_str(model_id)?;
        let account_info = self.rpc_client.get_account(&pubkey)?;

        let data = &account_info.data;

        if data.len() < 100 {
            anyhow::bail!("Invalid model account data");
        }

        // Extract fields (simplified)
        let developer = Pubkey::new_from_array(data[8..40].try_into()?).to_string();
        let modalities = data[64] as u64;
        let status = match data[65] {
            0 => "Pending",
            1 => "Active",
            2 => "Retired",
            3 => "Suspended",
            _ => "Unknown",
        };

        let doc = doc!(
            self.model_fields.id => model_id,
            self.model_fields.developer => developer,
            self.model_fields.name => "",
            self.model_fields.version => "",
            self.model_fields.modalities => modalities,
            self.model_fields.model_uri => "",
            self.model_fields.status => status,
            self.model_fields.score => 0u64,
            self.model_fields.total_uses => 0u64,
            self.model_fields.created_at => chrono::Utc::now().timestamp()
        );

        let term = tantivy::Term::from_field_text(self.model_fields.id, model_id);
        self.model_writer.delete_term(term);
        self.model_writer.add_document(doc)?;
        self.model_writer.commit()?;

        info!("Indexed model {}", model_id);
        Ok(())
    }

    /// Get the request index for queries
    pub fn request_index(&self) -> &Index {
        &self.request_index
    }

    /// Get the worker index for queries
    pub fn worker_index(&self) -> &Index {
        &self.worker_index
    }

    /// Get the model index for queries
    pub fn model_index(&self) -> &Index {
        &self.model_index
    }

    /// Get request fields
    pub fn request_fields(&self) -> &RequestFields {
        &self.request_fields
    }

    /// Get worker fields
    pub fn worker_fields(&self) -> &WorkerFields {
        &self.worker_fields
    }

    /// Get model fields
    pub fn model_fields(&self) -> &ModelFields {
        &self.model_fields
    }
}
