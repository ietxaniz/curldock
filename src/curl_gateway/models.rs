use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurlCommand {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub data: Option<String>,
    pub cookies: Vec<(String, String)>,
    pub options: CurlOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

/// Note: HTTP-specific options such as User-Agent, Authorization, Content-Type, etc.,
/// should be set using the `headers` field in the `CurlCommand` struct.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CurlOptions {
    pub verbose: Option<bool>,
    pub insecure: Option<bool>,
    #[serde(rename = "followRedirects")]
    pub follow_redirects: Option<bool>,
    #[serde(rename = "maxRedirects")]
    pub max_redirects: Option<u32>,
    pub timeout: Option<u32>,
    #[serde(rename = "connectTimeout")]
    pub connect_timeout: Option<u32>,
    pub proxy: Option<String>,
    #[serde(rename = "outputFile")]
    pub output_file: Option<String>,
    pub cert: Option<String>,
    pub key: Option<String>,
    #[serde(rename = "keyPassword")]
    pub key_password: Option<String>,
    pub compressed: Option<bool>,
    pub retry: Option<u32>,
    #[serde(rename = "retryDelay")]
    pub retry_delay: Option<u32>,
    pub fail: Option<bool>,
    #[serde(rename = "interface")]
    pub interface: Option<String>,
    #[serde(rename = "dnsServers")]
    pub dns_servers: Option<Vec<String>>,
    #[serde(rename = "ipv4Only")]
    pub ipv4_only: Option<bool>,
    #[serde(rename = "ipv6Only")]
    pub ipv6_only: Option<bool>,
    #[serde(rename = "maxTime")]
    pub max_time: Option<u32>,
    #[serde(rename = "rateLimit")]
    pub rate_limit: Option<u32>,
    #[serde(rename = "timeNamelookup")]
    pub time_namelookup: Option<bool>,
    #[serde(rename = "timeConnect")]
    pub time_connect: Option<bool>,
    #[serde(rename = "timeAppconnect")]
    pub time_appconnect: Option<bool>,
    #[serde(rename = "timePretransfer")]
    pub time_pretransfer: Option<bool>,
    #[serde(rename = "timeStarttransfer")]
    pub time_starttransfer: Option<bool>,
    #[serde(rename = "timeTotal")]
    pub time_total: Option<bool>,
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
    pub cookies: HashMap<String, String>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(rename = "redirectCount")]
    pub redirect_count: Option<u32>,
    #[serde(rename = "effectiveUrl")]
    pub effective_url: Option<String>,
    // Integrated timing information
    #[serde(rename = "timeNamelookup")]
    pub time_namelookup: Option<u32>,
    #[serde(rename = "timeConnect")]
    pub time_connect: Option<u32>,
    #[serde(rename = "timeAppconnect")]
    pub time_appconnect: Option<u32>,
    #[serde(rename = "timePretransfer")]
    pub time_pretransfer: Option<u32>,
    #[serde(rename = "timeStarttransfer")]
    pub time_starttransfer: Option<u32>,
    #[serde(rename = "timeTotal")]
    pub time_total: Option<u32>,
}

#[derive(Error, Debug)]
pub enum CommandExecutionError {
    #[error("Failed to generate curl command: {0}")]
    CommandGenerationError(String),
    #[error("Failed to execute curl command: {0}")]
    ExecutionError(String),
    #[error("Failed to capture curl output: {0}")]
    OutputCaptureError(String),
    #[error("Failed to parse curl output: {0}")]
    OutputParseError(String),
    #[error("Invalid HTTP status code: {0}")]
    InvalidStatusCode(String),
}
