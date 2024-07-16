use crate::curl_gateway::operations::generate_curl_command;
use crate::script_manager::errors::{ErrorKind, ScriptManagerError};
use crate::script_manager::models::{ScriptDetails, ScriptDetailsCreate};
use crate::script_manager::ScriptManager;
use std::fs;
use std::io::Write;
use std::sync::Arc;
use std::time::UNIX_EPOCH;

impl ScriptManager {
    pub fn update_script(
        self: &Arc<Self>,
        script_details: ScriptDetailsCreate,
    ) -> Result<ScriptDetails, ScriptManagerError> {
        let _lock = self.lock();

        let full_path = self.get_full_path(&script_details.path)?.join(&script_details.name);

        if !full_path.exists() {
            return Err(ScriptManagerError::new(
                ErrorKind::ScriptNotFound,
                "update_script",
                "Script does not exist",
            ));
        }

        let curl_command_str = generate_curl_command(&script_details.curl_command)
            .map_err(|e| ScriptManagerError::from_curl_gateway_error("update_script", e))?;

        let mut file = fs::File::create(&full_path).map_err(|e| {
            ScriptManagerError::with_source(
                ErrorKind::Io,
                "update_script",
                "Failed to create file",
                Box::new(e),
            )
        })?;
        file.write_all(curl_command_str.as_bytes()).map_err(|e| {
            ScriptManagerError::with_source(
                ErrorKind::Io,
                "update_script",
                "Failed to write to file",
                Box::new(e),
            )
        })?;

        let metadata = fs::metadata(&full_path).map_err(|e| {
            ScriptManagerError::with_source(
                ErrorKind::Io,
                "update_script",
                "Failed to get file metadata",
                Box::new(e),
            )
        })?;

        Ok(ScriptDetails {
            name: script_details.name,
            path: script_details.path,
            curl_command: script_details.curl_command,
            created_at: metadata
                .created()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
                .unwrap_or(0),
            updated_at: metadata
                .modified()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
                .unwrap_or(0),
        })
    }
}
