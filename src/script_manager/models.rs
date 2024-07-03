use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct ScriptInfo {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_folder: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ScriptList {
    pub path: String,
    pub scripts: Vec<ScriptInfo>,
}