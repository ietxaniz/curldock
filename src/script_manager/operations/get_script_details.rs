use std::fs;
use std::path::PathBuf;
use chrono::DateTime;
use crate::script_manager::models::{ScriptDetails, ScriptError};
use crate::script_manager::ScriptManager;
use crate::curl_gateway::operations::parse_curl_command;

impl ScriptManager {
    pub fn get_script_details(&self, path: &str, name: &str) -> Result<ScriptDetails, ScriptError> {
        let full_path: PathBuf = self.scripts_dir().join(path).join(name);
        
        if !full_path.exists() {
            return Err(ScriptError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Script not found"
            )));
        }

        let metadata = fs::metadata(&full_path)?;
        let content = fs::read_to_string(&full_path)?;

        let curl_command = parse_curl_command(&content)?;

        Ok(ScriptDetails {
            name: name.to_string(),
            path: path.to_string(),
            curl_command,
            created_at: DateTime::from(metadata.created()?),
            updated_at: DateTime::from(metadata.modified()?),
        })
    }
}