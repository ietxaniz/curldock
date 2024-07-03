use std::fs;
use std::path::PathBuf;
use std::process::Command;
use crate::script_manager::models::ScriptError;
use crate::script_manager::ScriptManager;
use crate::curl_gateway::operations::{parse_curl_command, generate_curl_command_result};
use crate::curl_gateway::models::{CurlCommand, CurlCommandResult};

impl ScriptManager {
    pub fn execute_script(&self, path: &str, name: &str) -> Result<CurlCommandResult, ScriptError> {
        let full_path: PathBuf = self.scripts_dir().join(path).join(name);
        
        if !full_path.exists() {
            return Err(ScriptError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Script not found"
            )));
        }

        let content = fs::read_to_string(&full_path)?;

        let curl_command: CurlCommand = parse_curl_command(&content)?;

        let output = Command::new("sh")
            .arg("-c")
            .arg(&content)
            .output()
            .map_err(|e| ScriptError::IoError(e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(generate_curl_command_result(curl_command, &stdout))
    }
}
