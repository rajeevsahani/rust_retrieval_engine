use tantivy::schema::{Field, Schema, TEXT, STORED};
use tracing::info;

pub fn build_schema() -> (Schema, Field) {
    info!("Building schema...");
    let mut schema_builder = Schema::builder();
    let text_field = schema_builder.add_text_field("text", TEXT | STORED);
    let schema = schema_builder.build();
    info!("Schema built successfully");
    (schema, text_field)
}
