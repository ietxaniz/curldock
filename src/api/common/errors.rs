use std::fmt;
use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use anyhow::{Error as AnyhowError, Result as AnyhowResult};
use serde::Serialize;

#[derive(Debug, Error, Serialize)]
pub struct ApiError {
    operation: String,
    message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stack: Vec<String>,
}

impl ApiError {
    pub fn new(operation: impl Into<String>, message: impl Into<String>) -> Self {
        ApiError {
            operation: operation.into(),
            message: message.into(),
            stack: Vec::new(),
        }
    }

    pub fn from_debug_error(operation: impl Into<String>, error: AnyhowError) -> Self {
        let message = error.to_string();
        let stack = error.chain()
            .map(|e| e.to_string())
            .collect();

        ApiError {
            operation: operation.into(),
            message,
            stack,
        }
    }

    pub fn new_error(operation: impl Into<String>, result: AnyhowResult<()>) -> Self {
        match result {
            Ok(_) => panic!("ApiError::new_error called with Ok value"),
            Err(e) => Self::from_debug_error(operation, e),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in {}: {}", self.operation, self.message)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let mut error_json = serde_json::json!({
            "error": {
                "operation": self.operation,
                "message": self.message,
            }
        });

        if !self.stack.is_empty() {
            error_json["error"]["stack"] = serde_json::to_value(&self.stack).unwrap();
        }

        HttpResponse::InternalServerError().json(error_json)
    }
}
