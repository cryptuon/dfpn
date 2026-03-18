//! Tantivy schema definitions for DFPN indexer

use tantivy::schema::{
    Field, Schema, SchemaBuilder, FAST, STORED, STRING, TEXT,
};

/// Fields for the request index
pub struct RequestFields {
    pub id: Field,
    pub requester: Field,
    pub content_hash: Field,
    pub storage_uri: Field,
    pub modalities: Field,
    pub status: Field,
    pub fee_amount: Field,
    pub deadline: Field,
    pub commit_deadline: Field,
    pub created_at: Field,
    pub commit_count: Field,
    pub reveal_count: Field,
}

/// Fields for the worker index
pub struct WorkerFields {
    pub id: Field,
    pub operator: Field,
    pub stake: Field,
    pub reputation: Field,
    pub modalities: Field,
    pub status: Field,
    pub tasks_completed: Field,
    pub tasks_failed: Field,
    pub last_active_slot: Field,
}

/// Fields for the model index
pub struct ModelFields {
    pub id: Field,
    pub developer: Field,
    pub name: Field,
    pub version: Field,
    pub modalities: Field,
    pub model_uri: Field,
    pub status: Field,
    pub score: Field,
    pub total_uses: Field,
    pub created_at: Field,
}

/// Build the request schema
pub fn build_request_schema() -> (Schema, RequestFields) {
    let mut builder = SchemaBuilder::new();

    let fields = RequestFields {
        id: builder.add_text_field("id", STRING | STORED),
        requester: builder.add_text_field("requester", STRING | STORED),
        content_hash: builder.add_text_field("content_hash", STRING | STORED),
        storage_uri: builder.add_text_field("storage_uri", TEXT | STORED),
        modalities: builder.add_u64_field("modalities", FAST | STORED),
        status: builder.add_text_field("status", STRING | STORED),
        fee_amount: builder.add_u64_field("fee_amount", FAST | STORED),
        deadline: builder.add_i64_field("deadline", FAST | STORED),
        commit_deadline: builder.add_i64_field("commit_deadline", FAST | STORED),
        created_at: builder.add_i64_field("created_at", FAST | STORED),
        commit_count: builder.add_u64_field("commit_count", FAST | STORED),
        reveal_count: builder.add_u64_field("reveal_count", FAST | STORED),
    };

    (builder.build(), fields)
}

/// Build the worker schema
pub fn build_worker_schema() -> (Schema, WorkerFields) {
    let mut builder = SchemaBuilder::new();

    let fields = WorkerFields {
        id: builder.add_text_field("id", STRING | STORED),
        operator: builder.add_text_field("operator", STRING | STORED),
        stake: builder.add_u64_field("stake", FAST | STORED),
        reputation: builder.add_u64_field("reputation", FAST | STORED),
        modalities: builder.add_u64_field("modalities", FAST | STORED),
        status: builder.add_text_field("status", STRING | STORED),
        tasks_completed: builder.add_u64_field("tasks_completed", FAST | STORED),
        tasks_failed: builder.add_u64_field("tasks_failed", FAST | STORED),
        last_active_slot: builder.add_u64_field("last_active_slot", FAST | STORED),
    };

    (builder.build(), fields)
}

/// Build the model schema
pub fn build_model_schema() -> (Schema, ModelFields) {
    let mut builder = SchemaBuilder::new();

    let fields = ModelFields {
        id: builder.add_text_field("id", STRING | STORED),
        developer: builder.add_text_field("developer", STRING | STORED),
        name: builder.add_text_field("name", TEXT | STORED),
        version: builder.add_text_field("version", STRING | STORED),
        modalities: builder.add_u64_field("modalities", FAST | STORED),
        model_uri: builder.add_text_field("model_uri", TEXT | STORED),
        status: builder.add_text_field("status", STRING | STORED),
        score: builder.add_u64_field("score", FAST | STORED),
        total_uses: builder.add_u64_field("total_uses", FAST | STORED),
        created_at: builder.add_i64_field("created_at", FAST | STORED),
    };

    (builder.build(), fields)
}
