use serde::{Serialize, Deserialize};
use thiserror::Error;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurlCommand {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub data: Option<String>,
    pub options: CurlOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CurlOptions {
    pub verbose: bool,
    pub insecure: bool,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Missing URL in curl command")]
    MissingUrl,
    #[error("Invalid HTTP method: {0}")]
    InvalidMethod(String),
    #[error("No curl command found in script")]
    MissingCurlCommand,
    // Add other error types as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurlCommandResult {
    pub request: CurlCommand,
    #[serde(rename = "responseHeaders")]
    pub response_headers: HashMap<String, String>,
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub date: String,
    pub body: String,
}
