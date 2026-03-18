//! REST API for querying the DFPN indexer

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tantivy::{
    collector::TopDocs,
    query::{AllQuery, QueryParser, TermQuery},
    schema::{IndexRecordOption, Value},
    Index, ReloadPolicy, TantivyDocument,
};
use tokio::sync::RwLock;
use tracing::error;

use crate::schema::{ModelFields, RequestFields, WorkerFields};

/// Shared application state
pub struct AppState {
    pub request_index: Index,
    pub worker_index: Index,
    pub model_index: Index,
    pub request_fields: RequestFields,
    pub worker_fields: WorkerFields,
    pub model_fields: ModelFields,
}

/// Query parameters for request listing
#[derive(Debug, Deserialize)]
pub struct RequestQuery {
    pub status: Option<String>,
    pub requester: Option<String>,
    pub modalities: Option<u64>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Query parameters for worker listing
#[derive(Debug, Deserialize)]
pub struct WorkerQuery {
    pub status: Option<String>,
    pub operator: Option<String>,
    pub min_reputation: Option<u64>,
    pub modalities: Option<u64>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Query parameters for model listing
#[derive(Debug, Deserialize)]
pub struct ModelQuery {
    pub status: Option<String>,
    pub developer: Option<String>,
    pub modalities: Option<u64>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Search query parameters
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<usize>,
}

/// Request response
#[derive(Debug, Serialize)]
pub struct RequestResponse {
    pub id: String,
    pub requester: String,
    pub content_hash: String,
    pub storage_uri: String,
    pub modalities: u64,
    pub status: String,
    pub fee_amount: u64,
    pub deadline: i64,
    pub commit_deadline: i64,
    pub created_at: i64,
    pub commit_count: u64,
    pub reveal_count: u64,
}

/// Worker response
#[derive(Debug, Serialize)]
pub struct WorkerResponse {
    pub id: String,
    pub operator: String,
    pub stake: u64,
    pub reputation: u64,
    pub modalities: u64,
    pub status: String,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub last_active_slot: u64,
}

/// Model response
#[derive(Debug, Serialize)]
pub struct ModelResponse {
    pub id: String,
    pub developer: String,
    pub name: String,
    pub version: String,
    pub modalities: u64,
    pub model_uri: String,
    pub status: String,
    pub score: u64,
    pub total_uses: u64,
    pub created_at: i64,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub request_count: u64,
    pub worker_count: u64,
    pub model_count: u64,
}

/// Create the API router
pub fn create_router(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/requests", get(list_requests))
        .route("/requests/:id", get(get_request))
        .route("/requests/search", get(search_requests))
        .route("/workers", get(list_workers))
        .route("/workers/:operator", get(get_worker))
        .route("/models", get(list_models))
        .route("/models/:id", get(get_model))
        .route("/models/search", get(search_models))
        .with_state(state)
}

/// Health check endpoint
async fn health_check(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Json<HealthResponse> {
    let state = state.read().await;

    let request_count = get_doc_count(&state.request_index);
    let worker_count = get_doc_count(&state.worker_index);
    let model_count = get_doc_count(&state.model_index);

    Json(HealthResponse {
        status: "healthy".to_string(),
        request_count,
        worker_count,
        model_count,
    })
}

/// Get document count for an index
fn get_doc_count(index: &Index) -> u64 {
    match index.reader() {
        Ok(reader) => reader.searcher().num_docs(),
        Err(_) => 0,
    }
}

/// List requests with optional filters
async fn list_requests(
    State(state): State<Arc<RwLock<AppState>>>,
    Query(params): Query<RequestQuery>,
) -> Result<Json<Vec<RequestResponse>>, StatusCode> {
    let state = state.read().await;
    let limit = params.limit.unwrap_or(100).min(1000);
    let offset = params.offset.unwrap_or(0);

    let reader = state
        .request_index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let searcher = reader.searcher();

    // Build query
    let query: Box<dyn tantivy::query::Query> = if let Some(status) = &params.status {
        let term = tantivy::Term::from_field_text(state.request_fields.status, status);
        Box::new(TermQuery::new(term, IndexRecordOption::Basic))
    } else {
        Box::new(AllQuery)
    };

    // Execute search
    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(limit + offset))
        .map_err(|e| {
            error!("Search error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Convert to response
    let mut results = Vec::new();
    for (i, (_score, doc_address)) in top_docs.into_iter().enumerate() {
        if i < offset {
            continue;
        }

        let doc: TantivyDocument = searcher.doc(doc_address).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Some(response) = doc_to_request(&doc, &state.request_fields) {
            results.push(response);
        }
    }

    Ok(Json(results))
}

/// Get a single request by ID
async fn get_request(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<String>,
) -> Result<Json<RequestResponse>, StatusCode> {
    let state = state.read().await;

    let reader = state
        .request_index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let searcher = reader.searcher();

    let term = tantivy::Term::from_field_text(state.request_fields.id, &id);
    let query = TermQuery::new(term, IndexRecordOption::Basic);

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(1))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((_score, doc_address)) = top_docs.into_iter().next() {
        let doc: TantivyDocument = searcher.doc(doc_address).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Some(response) = doc_to_request(&doc, &state.request_fields) {
            return Ok(Json(response));
        }
    }

    Err(StatusCode::NOT_FOUND)
}

/// Search requests by text
async fn search_requests(
    State(state): State<Arc<RwLock<AppState>>>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<RequestResponse>>, StatusCode> {
    let state = state.read().await;
    let limit = params.limit.unwrap_or(100).min(1000);

    let reader = state
        .request_index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(
        &state.request_index,
        vec![state.request_fields.storage_uri],
    );

    let query = query_parser
        .parse_query(&params.q)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(limit))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut results = Vec::new();
    for (_score, doc_address) in top_docs {
        let doc: TantivyDocument = searcher.doc(doc_address).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Some(response) = doc_to_request(&doc, &state.request_fields) {
            results.push(response);
        }
    }

    Ok(Json(results))
}

/// List workers with optional filters
async fn list_workers(
    State(state): State<Arc<RwLock<AppState>>>,
    Query(params): Query<WorkerQuery>,
) -> Result<Json<Vec<WorkerResponse>>, StatusCode> {
    let state = state.read().await;
    let limit = params.limit.unwrap_or(100).min(1000);
    let offset = params.offset.unwrap_or(0);

    let reader = state
        .worker_index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let searcher = reader.searcher();

    let query: Box<dyn tantivy::query::Query> = if let Some(status) = &params.status {
        let term = tantivy::Term::from_field_text(state.worker_fields.status, status);
        Box::new(TermQuery::new(term, IndexRecordOption::Basic))
    } else {
        Box::new(AllQuery)
    };

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(limit + offset))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut results = Vec::new();
    for (i, (_score, doc_address)) in top_docs.into_iter().enumerate() {
        if i < offset {
            continue;
        }

        let doc: TantivyDocument = searcher.doc(doc_address).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Some(response) = doc_to_worker(&doc, &state.worker_fields) {
            results.push(response);
        }
    }

    Ok(Json(results))
}

/// Get a single worker by operator
async fn get_worker(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(operator): Path<String>,
) -> Result<Json<WorkerResponse>, StatusCode> {
    let state = state.read().await;

    let reader = state
        .worker_index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let searcher = reader.searcher();

    let term = tantivy::Term::from_field_text(state.worker_fields.operator, &operator);
    let query = TermQuery::new(term, IndexRecordOption::Basic);

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(1))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((_score, doc_address)) = top_docs.into_iter().next() {
        let doc: TantivyDocument = searcher.doc(doc_address).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Some(response) = doc_to_worker(&doc, &state.worker_fields) {
            return Ok(Json(response));
        }
    }

    Err(StatusCode::NOT_FOUND)
}

/// List models with optional filters
async fn list_models(
    State(state): State<Arc<RwLock<AppState>>>,
    Query(params): Query<ModelQuery>,
) -> Result<Json<Vec<ModelResponse>>, StatusCode> {
    let state = state.read().await;
    let limit = params.limit.unwrap_or(100).min(1000);
    let offset = params.offset.unwrap_or(0);

    let reader = state
        .model_index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let searcher = reader.searcher();

    let query: Box<dyn tantivy::query::Query> = if let Some(status) = &params.status {
        let term = tantivy::Term::from_field_text(state.model_fields.status, status);
        Box::new(TermQuery::new(term, IndexRecordOption::Basic))
    } else {
        Box::new(AllQuery)
    };

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(limit + offset))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut results = Vec::new();
    for (i, (_score, doc_address)) in top_docs.into_iter().enumerate() {
        if i < offset {
            continue;
        }

        let doc: TantivyDocument = searcher.doc(doc_address).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Some(response) = doc_to_model(&doc, &state.model_fields) {
            results.push(response);
        }
    }

    Ok(Json(results))
}

/// Get a single model by ID
async fn get_model(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<String>,
) -> Result<Json<ModelResponse>, StatusCode> {
    let state = state.read().await;

    let reader = state
        .model_index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let searcher = reader.searcher();

    let term = tantivy::Term::from_field_text(state.model_fields.id, &id);
    let query = TermQuery::new(term, IndexRecordOption::Basic);

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(1))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((_score, doc_address)) = top_docs.into_iter().next() {
        let doc: TantivyDocument = searcher.doc(doc_address).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Some(response) = doc_to_model(&doc, &state.model_fields) {
            return Ok(Json(response));
        }
    }

    Err(StatusCode::NOT_FOUND)
}

/// Search models by text
async fn search_models(
    State(state): State<Arc<RwLock<AppState>>>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<ModelResponse>>, StatusCode> {
    let state = state.read().await;
    let limit = params.limit.unwrap_or(100).min(1000);

    let reader = state
        .model_index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(
        &state.model_index,
        vec![state.model_fields.name, state.model_fields.model_uri],
    );

    let query = query_parser
        .parse_query(&params.q)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(limit))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut results = Vec::new();
    for (_score, doc_address) in top_docs {
        let doc: TantivyDocument = searcher.doc(doc_address).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Some(response) = doc_to_model(&doc, &state.model_fields) {
            results.push(response);
        }
    }

    Ok(Json(results))
}

// Helper functions to convert documents to response types

fn doc_to_request(doc: &TantivyDocument, fields: &RequestFields) -> Option<RequestResponse> {
    Some(RequestResponse {
        id: get_text_field(doc, fields.id)?,
        requester: get_text_field(doc, fields.requester)?,
        content_hash: get_text_field(doc, fields.content_hash)?,
        storage_uri: get_text_field(doc, fields.storage_uri).unwrap_or_default(),
        modalities: get_u64_field(doc, fields.modalities).unwrap_or(0),
        status: get_text_field(doc, fields.status)?,
        fee_amount: get_u64_field(doc, fields.fee_amount).unwrap_or(0),
        deadline: get_i64_field(doc, fields.deadline).unwrap_or(0),
        commit_deadline: get_i64_field(doc, fields.commit_deadline).unwrap_or(0),
        created_at: get_i64_field(doc, fields.created_at).unwrap_or(0),
        commit_count: get_u64_field(doc, fields.commit_count).unwrap_or(0),
        reveal_count: get_u64_field(doc, fields.reveal_count).unwrap_or(0),
    })
}

fn doc_to_worker(doc: &TantivyDocument, fields: &WorkerFields) -> Option<WorkerResponse> {
    Some(WorkerResponse {
        id: get_text_field(doc, fields.id)?,
        operator: get_text_field(doc, fields.operator)?,
        stake: get_u64_field(doc, fields.stake).unwrap_or(0),
        reputation: get_u64_field(doc, fields.reputation).unwrap_or(0),
        modalities: get_u64_field(doc, fields.modalities).unwrap_or(0),
        status: get_text_field(doc, fields.status)?,
        tasks_completed: get_u64_field(doc, fields.tasks_completed).unwrap_or(0),
        tasks_failed: get_u64_field(doc, fields.tasks_failed).unwrap_or(0),
        last_active_slot: get_u64_field(doc, fields.last_active_slot).unwrap_or(0),
    })
}

fn doc_to_model(doc: &TantivyDocument, fields: &ModelFields) -> Option<ModelResponse> {
    Some(ModelResponse {
        id: get_text_field(doc, fields.id)?,
        developer: get_text_field(doc, fields.developer)?,
        name: get_text_field(doc, fields.name).unwrap_or_default(),
        version: get_text_field(doc, fields.version).unwrap_or_default(),
        modalities: get_u64_field(doc, fields.modalities).unwrap_or(0),
        model_uri: get_text_field(doc, fields.model_uri).unwrap_or_default(),
        status: get_text_field(doc, fields.status)?,
        score: get_u64_field(doc, fields.score).unwrap_or(0),
        total_uses: get_u64_field(doc, fields.total_uses).unwrap_or(0),
        created_at: get_i64_field(doc, fields.created_at).unwrap_or(0),
    })
}

fn get_text_field(doc: &TantivyDocument, field: tantivy::schema::Field) -> Option<String> {
    doc.get_first(field)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn get_u64_field(doc: &TantivyDocument, field: tantivy::schema::Field) -> Option<u64> {
    doc.get_first(field).and_then(|v| v.as_u64())
}

fn get_i64_field(doc: &TantivyDocument, field: tantivy::schema::Field) -> Option<i64> {
    doc.get_first(field).and_then(|v| v.as_i64())
}
