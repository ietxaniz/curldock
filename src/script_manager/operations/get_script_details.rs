use crate::curl_gateway::operations::parse_curl_command;
use crate::script_manager::errors::{ScriptManagerError, ErrorKind};
use crate::script_manager::models::ScriptDetails;
use crate::script_manager::ScriptManager;
use std::fs;
use std::time::UNIX_EPOCH;

impl ScriptManager {
    pub fn get_script_details(&self, full_path: &str) -> Result<ScriptDetails, ScriptManagerError> {
        let local_full_path = self.get_full_path(full_path)?;

        if !local_full_path.exists() {
            return Err(ScriptManagerError::new(
                ErrorKind::ScriptNotFound,
                "get_script_details",
                format!("Script not found: {}", local_full_path.display()),
            ));
        }

        let metadata = fs::metadata(&local_full_path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "get_script_details",
                "Failed to read metadata",
                Box::new(e),
            ))?;

        let content = fs::read_to_string(&local_full_path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "get_script_details",
                "Failed to read script file",
                Box::new(e),
            ))?;

        let curl_command = parse_curl_command(&content)
            .map_err(|e| ScriptManagerError::from_curl_gateway_error("parse_curl_command", e))?;

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
