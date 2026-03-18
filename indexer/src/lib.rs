//! DFPN Indexer - Tantivy-based event indexer for DFPN protocol
//!
//! This crate provides:
//! - Solana event subscription for DFPN programs
//! - Full-text search indexing using Tantivy
//! - REST API for querying indexed data

pub mod api;
pub mod indexer;
pub mod schema;
pub mod subscriber;

pub use api::{create_router, AppState};
pub use indexer::{DfpnIndexer, IndexEvent};
pub use schema::{
    build_model_schema, build_request_schema, build_worker_schema,
    ModelFields, RequestFields, WorkerFields,
};
pub use subscriber::{EventSubscriber, ProgramIds};
