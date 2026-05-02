mod index;
mod logger;
mod models;
mod schema;
mod search;
mod server;

use tracing::info;

#[tokio::main]
async fn main() {
    logger::init_logger();

    info!("Starting rust_retrieval_engine...");

    let (schema, text_field) = schema::build_schema();
    let index = std::sync::Arc::new(index::build_index(schema, text_field));
    let app = server::build_router(std::sync::Arc::clone(&index), text_field);

    server::start_server(app).await;
}
