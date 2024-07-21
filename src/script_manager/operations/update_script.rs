use crate::debug_err;
use anyhow::{Result, anyhow};
use crate::curl_gateway::operations::generate_curl_command;
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
    ) -> Result<ScriptDetails> {
        let _lock = self.lock();

        let full_path = self.get_full_path(&script_details.full_name)?;

        if !full_path.exists() {
            return debug_err!(Err(anyhow!("Script does not exist")));
        }

        let curl_command_str = debug_err!(generate_curl_command(&script_details.curl_command))?;

        let mut file = debug_err!(fs::File::create(&full_path), "Failed to create file")?;
        debug_err!(file.write_all(curl_command_str.as_bytes()), "Failed to write to file")?;

        let metadata = debug_err!(fs::metadata(&full_path), "Failed to get file metadata")?;

        Ok(ScriptDetails {
            full_name: script_details.full_name,
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
