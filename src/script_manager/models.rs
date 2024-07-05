use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use thiserror::Error;
use crate::curl_gateway::models::ParseError;
use crate::curl_gateway::models::CurlCommand;

#[derive(Serialize, Deserialize)]
pub struct ScriptInfo {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
    pub curl_command: CurlCommand,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ScriptDetailsCreate {
    pub name: String,
    pub path: String,
    pub curl_command: CurlCommand,
}

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Curl parse error: {0}")]
    CurlParseError(#[from] ParseError),
}
