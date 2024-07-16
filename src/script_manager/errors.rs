// In script_manager/errors.rs

use std::fmt;
use thiserror::Error;
use crate::curl_gateway::errors::CurlGatewayError;

#[derive(Debug, Error)]
pub enum ScriptManagerError {
    #[error("{kind} error in {operation}: {message}")]
    OperationError {
        kind: ErrorKind,
        operation: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    Io,
    ScriptNotFound,
    CurlGateway,
    InvalidInput,
    Internal,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Io => write!(f, "I/O"),
            ErrorKind::ScriptNotFound => write!(f, "Script not found"),
            ErrorKind::CurlGateway => write!(f, "Curl gateway"),
            ErrorKind::InvalidInput => write!(f, "Invalid input"),
            ErrorKind::Internal => write!(f, "Internal error"),
        }
    }
}

impl ScriptManagerError {
    pub fn new(kind: ErrorKind, operation: impl Into<String>, message: impl Into<String>) -> Self {
        ScriptManagerError::OperationError {
            kind,
            operation: operation.into(),
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(
        kind: ErrorKind,
        operation: impl Into<String>,
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        ScriptManagerError::OperationError {
            kind,
            operation: operation.into(),
            message: message.into(),
            source: Some(source),
        }
    }

    pub fn from_curl_gateway_error(operation: impl Into<String>, e: CurlGatewayError) -> Self {
        ScriptManagerError::OperationError {
            kind: ErrorKind::CurlGateway,
            operation: operation.into(),
            message: format!("{}", e),
            source: Some(Box::new(e)),
        }
    }
}
