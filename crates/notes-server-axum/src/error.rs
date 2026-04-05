use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use notes_core::{AuthError, StorageError};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Forbidden,
    Unauthorized(String),
    Conflict(String),
    UnprocessableEntity(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found".into()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "forbidden".into()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::UnprocessableEntity(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<StorageError> for AppError {
    fn from(e: StorageError) -> Self {
        match e {
            StorageError::NotFound => AppError::NotFound,
            StorageError::AlreadyExists => AppError::UnprocessableEntity("already exists".into()),
            StorageError::PermissionDenied => AppError::Forbidden,
            StorageError::VaultNotEmpty => {
                AppError::UnprocessableEntity("vault is not empty".into())
            }
            StorageError::IoError(e) => AppError::Internal(e.to_string()),
            StorageError::ParseError(msg) => AppError::Internal(format!("parse error: {msg}")),
        }
    }
}

impl From<AuthError> for AppError {
    fn from(e: AuthError) -> Self {
        match e {
            AuthError::InvalidToken => AppError::Unauthorized("invalid token".into()),
            AuthError::UserNotFound => AppError::Unauthorized("user not found".into()),
            AuthError::InvalidCredentials => AppError::Unauthorized("invalid credentials".into()),
            AuthError::Forbidden => AppError::Forbidden,
            AuthError::Conflict(msg) => AppError::Conflict(msg),
            AuthError::Validation(msg) => AppError::UnprocessableEntity(msg),
            AuthError::ServiceError(msg) => AppError::Internal(msg),
        }
    }
}
