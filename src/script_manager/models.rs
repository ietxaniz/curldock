use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use thiserror::Error;
use crate::curl_gateway::models::ParseError;
use crate::curl_gateway::models::CurlCommand;
use crate::curl_gateway::models::CommandExecutionError;

#[derive(Serialize, Deserialize)]
pub struct ScriptInfo {
    pub name: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "isFolder")]
    pub is_folder: bool,
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct ScriptList {
    pub path: String,
    pub scripts: Vec<ScriptInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct ScriptDetails {
    pub name: String,
    pub path: String,
    #[serde(rename = "curlCommand")]
    pub curl_command: CurlCommand,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ScriptDetailsCreate {
    pub name: String,
    pub path: String,
    #[serde(rename = "curlCommand")]
    pub curl_command: CurlCommand,
}

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Curl parse error: {0}")]
    CurlParseError(#[from] ParseError),

    #[error("Script execution error: {0}")]
    ExecutionError(String),

    #[error("Command generation error: {0}")]
    CommandGenerationError(String),

    #[error("Output capture error: {0}")]
    OutputCaptureError(String),

    #[error("Output parse error: {0}")]
    OutputParseError(String),

    #[error("Command execution error: {0}")]
    CommandExecutionError(#[from] CommandExecutionError),

    #[error("Script not found: {0}")]
    ScriptNotFound(String),
}
