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
    pub fn create_script(
        self: &Arc<Self>,
        script_details: ScriptDetailsCreate,
    ) -> Result<ScriptDetails> {
        let _lock = self.lock();


        let full_path = self.get_full_path(&script_details.full_name)?;

        println!("Creating script at: {:?}", full_path);

        if full_path.exists() {
            return Err(anyhow!("Script already exists"));
        }

        let curl_command_str = debug_err!(generate_curl_command(&script_details.curl_command))?;

        println!("Generated curl command: {}", curl_command_str);

        let mut file = debug_err!(fs::File::create(&full_path), "Failed to create script file {}", script_details.full_name)?;

        debug_err!(file.write_all(curl_command_str.as_bytes()),"Failed to write to script file")?;

        let metadata = debug_err!(fs::metadata(&full_path), "Failed to read metadata")?;

        Ok(ScriptDetails {
            full_name: script_details.full_name,
            curl_command: script_details.curl_command,
            created_at: metadata
                .created()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64)
                .unwrap_or(0),
            updated_at: metadata
                .modified()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64)
                .unwrap_or(0),
        })
    }
}
