use crate::curl_gateway::models::CurlCommand;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum FileType {
    Script,
    Data,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub name: String,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
    #[serde(rename = "isFolder")]
    pub is_folder: bool,
    pub path: String,
    pub file_type: FileType,
}

#[derive(Serialize, Deserialize)]
pub struct FileList {
    pub path: String,
    pub files: Vec<FileInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct ScriptDetails {
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "curlCommand")]
    pub curl_command: CurlCommand,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ScriptDetailsCreate {
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "curlCommand")]
    pub curl_command: CurlCommand,
}
