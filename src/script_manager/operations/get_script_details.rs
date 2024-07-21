use crate::debug_err;
use anyhow::{anyhow, Result};
use crate::curl_gateway::operations::parse_curl_command;
use crate::script_manager::models::ScriptDetails;
use crate::script_manager::ScriptManager;
use std::fs;
use std::time::UNIX_EPOCH;

impl ScriptManager {
    pub fn get_script_details(&self, full_path: &str) -> Result<ScriptDetails> {
        let local_full_path = self.get_full_path(full_path)?;

        if !local_full_path.exists() {
            return Err(anyhow!("Script not found: {}", local_full_path.display()));
        }

        let metadata = debug_err!(fs::metadata(&local_full_path), "Failed to read metadata")?;

        let content = debug_err!(fs::read_to_string(&local_full_path), "Failed to read script file")?;

        let curl_command = debug_err!(parse_curl_command(&content))?;

        let created_at = metadata
            .created()
            .map(|t| t.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64)
            .unwrap_or(0);
        let updated_at = metadata
            .modified()
            .map(|t| t.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64)
            .unwrap_or(0);

        Ok(ScriptDetails {
            full_name: full_path.to_string(),
            curl_command,
            created_at,
            updated_at,
        })
    }
}
