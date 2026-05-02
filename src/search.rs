
use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, TantivyDocument, Value};
use tantivy::{DocAddress, Index, Score};
use tracing::{debug, error, info, warn};

use crate::models::{SearchQuery, SearchResult};

// Change return type from Json to Response
pub async fn search_handler(
    State((index, text_field)): State<(Arc<Index>, Field)>,
    Json(payload): Json<SearchQuery>,
) -> Response {
    let query_str = payload.q.clone();
    info!("Received search query: '{}'", query_str);

    let reader = match index.reader() {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to get reader: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "code": 500,
                    "status": "Internal Server Error",
                    "message": format!("Index reader failed: {}", e)
                })),
            ).into_response();
        }
    };

    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![text_field]);

    let query = match query_parser.parse_query(&query_str) {
        Ok(q) => q,
        Err(e) => {
            warn!("Bad query '{}': {}", query_str, e);
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "code": 400,
                    "status": "Bad Request",
                    "message": format!("Invalid query: {}", e)
                })),
            ).into_response();
        }
    };

    let top_docs: Vec<(Score, DocAddress)> =
        match searcher.search(&query, &TopDocs::with_limit(3)) {
            Ok(docs) => {
                info!("Search returned {} results", docs.len());
                docs
            }
            Err(e) => {
                error!("Search failed: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "code": 500,
                        "status": "Internal Server Error",
                        "message": format!("Search failed: {}", e)
                    })),
                ).into_response();
            }
        };

    let mut results = vec![];
    for (score, doc_address) in top_docs {
        let retrieved: TantivyDocument = searcher.doc(doc_address).unwrap();
        let text = retrieved
            .get_first(text_field)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        debug!("Result (score: {:.4}): {}", score, text);
        results.push(text);
    }

    if results.is_empty() {
        warn!("No results found for query: '{}'", query_str);
    }

    (StatusCode::OK, Json(SearchResult { results })).into_response()
}
