use tantivy::schema::{Field, Schema};
use tantivy::{doc, Index, IndexWriter};
use tracing::{debug, info};

pub fn build_index(schema: Schema, text_field: Field) -> Index {
    info!("Creating in-RAM index...");

    let index = Index::create_in_ram(schema);
    let mut writer: IndexWriter = index.writer(50_000_000)
        .expect("Failed to create index writer");

    let docs = vec![
        "Rust is fast and memory safe.",
        "Axum is a modern Rust web framework.",
        "Tantivy is a full-text search engine in Rust.",
        "Tokio is an async runtime for Rust.",
        "Serde is a serialization framework for Rust.",
    ];

    for content in &docs {
        debug!("Indexing document: {}", content);
        writer
            .add_document(doc!(text_field => *content))
            .expect("Failed to add document");
    }

    writer.commit().expect("Failed to commit index");
    info!("Indexed {} documents successfully", docs.len());

    index
}
