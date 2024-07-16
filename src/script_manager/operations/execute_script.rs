use crate::curl_gateway::models::CurlCommandResult;
use crate::curl_gateway::operations::{execute_curl_command, parse_curl_command};
use crate::script_manager::errors::{ErrorKind, ScriptManagerError};
use crate::script_manager::ScriptManager;
use std::fs;

impl ScriptManager {
    pub fn execute_script(
        &self,
        path: &str,
        name: &str,
    ) -> Result<CurlCommandResult, ScriptManagerError> {
        let full_path = self.get_full_path(path)?.join(name);

        if !full_path.exists() {
            return Err(ScriptManagerError::new(
                ErrorKind::ScriptNotFound,
                "execute_script",
                format!("Script not found at path: {}", full_path.display()),
            ));
        }

        let content = fs::read_to_string(&full_path).map_err(|e| {
            ScriptManagerError::with_source(
                ErrorKind::Io,
                "execute_script",
                "Failed to read script file",
                Box::new(e),
            )
        })?;

        let curl_command = parse_curl_command(&content)
            .map_err(|e| ScriptManagerError::from_curl_gateway_error("parse_curl_command", e))?;

        execute_curl_command(curl_command)
            .map_err(|e| ScriptManagerError::from_curl_gateway_error("execute_curl_command", e))
    }
}
