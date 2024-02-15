use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub(crate) enum BdfError {
    #[error("failed to read the key file")]
    SocketError(#[source] std::io::Error),
    #[error("invalid token")]
    InvalidToken,
    #[error("wrong credentials")]
    InvalidCredentials,
    #[error("web token creation failed")]
    TokenCreationError,
}

impl IntoResponse for BdfError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            BdfError::SocketError(e) => (StatusCode::BAD_REQUEST, format!("Socket not available: {}", e)),
            BdfError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()),
            BdfError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            BdfError::TokenCreationError => (StatusCode::UNAUTHORIZED, "creation of web token failed".to_string())
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}