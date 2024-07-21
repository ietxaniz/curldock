use crate::curl_gateway::models::CurlCommandResult;
use crate::curl_gateway::operations::{execute_curl_command, parse_curl_command};
use crate::debug_err;
use crate::script_manager::ScriptManager;
use anyhow::{anyhow, Result};
use std::fs;

impl ScriptManager {
    pub fn execute_script(&self, full_path: &str) -> Result<CurlCommandResult> {
        let full_path = self.get_full_path(full_path)?;

        if !full_path.exists() {
            return Err(anyhow!("Script not found at path: {}", full_path.display()));
        }

        let content = debug_err!(fs::read_to_string(&full_path), "Failed to read script file")?;

        let curl_command = debug_err!(parse_curl_command(&content))?;

        debug_err!(execute_curl_command(curl_command))
    }
}
