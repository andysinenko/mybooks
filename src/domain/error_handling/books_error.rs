use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum BooksError {
    NotFound(String),
    Database(String),
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for BooksError {
    fn into_response(self) -> Response {
        let (status, message) = match self { 
            BooksError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            BooksError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            BooksError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ErrorResponse { error: message });

        (status, body).into_response()
    }
}