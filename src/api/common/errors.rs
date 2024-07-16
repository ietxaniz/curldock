use std::fmt;
use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use crate::script_manager::errors::ScriptManagerError;
use serde::Serialize;

#[derive(Debug, Error, Serialize)]
pub enum ApiError {
    #[error("{kind} error in {operation}: {message}")]
    OperationError {
        kind: ErrorKind,
        operation: String,
        message: String,
        #[serde(skip_serializing)]
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ErrorKind {
    Io,
    NotFound,
    CurlGateway,
    InvalidInput,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Io => write!(f, "I/O"),
            ErrorKind::NotFound => write!(f, "Not found"),
            ErrorKind::CurlGateway => write!(f, "Curl gateway"),
            ErrorKind::InvalidInput => write!(f, "Invalid input"),
        }
    }
}

impl ApiError {
    pub fn new(kind: ErrorKind, operation: impl Into<String>, message: impl Into<String>) -> Self {
        ApiError::OperationError {
            kind,
            operation: operation.into(),
            message: message.into(),
            source: None,
        }
    }


    pub fn from_script_manager_error(operation: impl Into<String>, e: ScriptManagerError) -> Self {
        ApiError::OperationError {
            kind: match e {
                ScriptManagerError::OperationError { kind, .. } => match kind {
                    crate::script_manager::errors::ErrorKind::Io => ErrorKind::Io,
                    crate::script_manager::errors::ErrorKind::ScriptNotFound => ErrorKind::NotFound,
                    crate::script_manager::errors::ErrorKind::CurlGateway => ErrorKind::CurlGateway,
                    crate::script_manager::errors::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
                },
            },
            operation: operation.into(),
            message: format!("{}", e),
            source: Some(Box::new(e)),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::OperationError { kind, operation, message, .. } => {
                let status_code = match kind {
                    ErrorKind::Io => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorKind::NotFound => actix_web::http::StatusCode::NOT_FOUND,
                    ErrorKind::CurlGateway => actix_web::http::StatusCode::BAD_GATEWAY,
                    ErrorKind::InvalidInput => actix_web::http::StatusCode::BAD_REQUEST,
                };

                HttpResponse::build(status_code).json(serde_json::json!({
                    "error": {
                        "kind": kind.to_string(),
                        "operation": operation,
                        "message": message,
                    }
                }))
            }
        }
    }
}
