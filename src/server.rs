use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use tantivy::schema::Field;
use tantivy::Index;
use tracing::{error, info, warn};

use crate::search::search_handler;

// ============================================================
// ERROR TYPES
// ============================================================
#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub status: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: StatusCode, message: &str) -> Self {
        ErrorResponse {
            code: code.as_u16(),
            status: code.canonical_reason().unwrap_or("Unknown").to_string(),
            message: message.to_string(),
        }
    }
}

// ============================================================
// ERROR HANDLERS
// ============================================================
pub async fn handle_404() -> impl IntoResponse {
    warn!("404 - Route not found");
    let body = ErrorResponse::new(StatusCode::NOT_FOUND, "Route not found");
    (StatusCode::NOT_FOUND, Json(body))
}

#[allow(dead_code)]
pub async fn handle_500(message: &str) -> impl IntoResponse {
    error!("500 - Internal server error: {}", message);
    let body = ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, message);
    (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
}

// ============================================================
// LOGGING MIDDLEWARE
// ============================================================
pub async fn logging_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    info!("--> {} {}", method, uri);

    let response = next.run(req).await;
    let status = response.status();

    if status.is_success() {
        info!("<-- {} {} {}", method, uri, status.as_u16());
    } else if status.is_client_error() {
        warn!("<-- {} {} {}", method, uri, status.as_u16());
    } else if status.is_server_error() {
        error!("<-- {} {} {}", method, uri, status.as_u16());
    }

    response
}

// ============================================================
// ROUTER
// ============================================================
pub fn build_router(index: Arc<Index>, text_field: Field) -> Router {
    info!("Building router...");
    Router::new()
        .route("/search", post(search_handler))
        .with_state((index, text_field))
        .layer(middleware::from_fn(logging_middleware))
        .fallback(handle_404)
}

// ============================================================
// SERVER
// ============================================================
pub async fn start_server(app: Router) {
    //let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server crashed");
}
