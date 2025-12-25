use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum DomainError {
    ValidationError(String),
    NotFound(String),
    AlreadyExists(String),
    Unauthorized(String),
    Forbidden(String),
    InternalError(String),
    DatabaseError(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Self::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl IntoResponse for DomainError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            DomainError::ValidationError(msg) => {
                (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg.clone())
            }
            DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            DomainError::AlreadyExists(msg) => (StatusCode::CONFLICT, "ALREADY_EXISTS", msg.clone()),
            DomainError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone())
            }
            DomainError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone()),
            DomainError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", msg.clone())
            }
            DomainError::DatabaseError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", msg.clone())
            }
        };

        let body = Json(ErrorResponse {
            success: false,
            message,
            code: Some(code.to_string()),
        });

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for DomainError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DomainError::NotFound("Resource not found".into()),
            sqlx::Error::Database(db_err) => {
                if let Some(code) = db_err.code() {
                    if code == "23505" {
                        return DomainError::AlreadyExists("Resource already exists".into());
                    }
                }
                DomainError::InternalError(format!("Database error: {}", db_err))
            }
            _ => DomainError::InternalError(format!("Database error: {}", err)),
        }
    }
}
