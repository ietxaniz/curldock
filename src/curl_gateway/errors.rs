// In curl_gateway/errors.rs

use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurlGatewayError {
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
    Parsing,
    Execution,
    DataHandling,
    InvalidInput,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Parsing => write!(f, "Parsing"),
            ErrorKind::Execution => write!(f, "Execution"),
            ErrorKind::DataHandling => write!(f, "Data handling"),
            ErrorKind::InvalidInput => write!(f, "Invalid input"),
        }
    }
}

impl CurlGatewayError {
    pub fn new(kind: ErrorKind, operation: impl Into<String>, message: impl Into<String>) -> Self {
        CurlGatewayError::OperationError {
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
        CurlGatewayError::OperationError {
            kind,
            operation: operation.into(),
            message: message.into(),
            source: Some(source),
        }
    }
}