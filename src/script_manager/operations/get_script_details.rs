use std::fs;
use std::time::UNIX_EPOCH;
use crate::script_manager::models::{ScriptDetails, ScriptError};
use crate::script_manager::ScriptManager;
use crate::curl_gateway::operations::parse_curl_command;

impl ScriptManager {
    pub fn get_script_details(&self, path: &str, name: &str) -> Result<ScriptDetails, ScriptError> {
        let full_path = self.get_full_path(path)?.join(name);
        
        if !full_path.exists() {
            return Err(ScriptError::ScriptNotFound(format!("Script not found: {}", full_path.display())));
        }

        let metadata = fs::metadata(&full_path)?;
        let content = fs::read_to_string(&full_path)?;

        let curl_command = parse_curl_command(&content)?;

        let created_at = metadata.created()
            .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
            .unwrap_or(0);
        let updated_at = metadata.modified()
            .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
            .unwrap_or(0);

        Ok(ScriptDetails {
            name: name.to_string(),
            path: path.to_string(),
            curl_command,
            created_at,
            updated_at,
        })
    }
}