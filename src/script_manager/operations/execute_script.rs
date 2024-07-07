use std::fs;
use std::path::PathBuf;
use crate::script_manager::models::ScriptError;
use crate::script_manager::ScriptManager;
use crate::curl_gateway::operations::{parse_curl_command, execute_curl_command};
use crate::curl_gateway::models::{CurlCommand, CurlCommandResult, CommandExecutionError};

impl ScriptManager {
    pub fn execute_script(&self, path: &str, name: &str) -> Result<CurlCommandResult, ScriptError> {
        let full_path: PathBuf = self.scripts_dir().join(path).join(name);
        
        if !full_path.exists() {
            return Err(ScriptError::ScriptNotFound(format!("Script not found at path: {}", full_path.display())));
        }

        let content = fs::read_to_string(&full_path)
            .map_err(ScriptError::IoError)?;

        let curl_command: CurlCommand = parse_curl_command(&content)
            .map_err(ScriptError::CurlParseError)?;

        execute_curl_command(curl_command)
            .map_err(|e| match e {
                CommandExecutionError::ExecutionError(msg) => ScriptError::ExecutionError(msg),
                CommandExecutionError::CommandGenerationError(msg) => ScriptError::CommandGenerationError(msg),
                CommandExecutionError::OutputCaptureError(msg) => ScriptError::OutputCaptureError(msg),
                CommandExecutionError::OutputParseError(msg) => ScriptError::OutputParseError(msg),
                CommandExecutionError::InvalidStatusCode(msg) => ScriptError::ExecutionError(format!("Invalid status code: {}", msg)),
            })
    }
}