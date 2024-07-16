use serde::{Serialize, Deserialize};
use crate::curl_gateway::models::CurlCommand;

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
    pub name: String,
    pub path: String,
    #[serde(rename = "curlCommand")]
    pub curl_command: CurlCommand,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ScriptDetailsCreate {
    pub name: String,
    pub path: String,
    #[serde(rename = "curlCommand")]
    pub curl_command: CurlCommand,
}
