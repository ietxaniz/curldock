use crate::curl_gateway::models::{CurlCommand, CurlCommandResult, CommandExecutionError};
use crate::curl_gateway::operations::{generate_curl_command, generate_curl_command_result};
use std::process::Command;

pub fn execute_curl_command(command: CurlCommand) -> Result<CurlCommandResult, CommandExecutionError> {
    let curl_command_str = generate_curl_command(&command)
        .map_err(|e| CommandExecutionError::CommandGenerationError(e.to_string()))?;

    println!("Executing curl command:\n{}", curl_command_str);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&curl_command_str)
        .output()
        .map_err(|e| CommandExecutionError::ExecutionError(format!("Failed to execute curl command: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    println!("Curl command stdout:\n{}", stdout);
    println!("Curl command stderr:\n{}", stderr);

    let result = generate_curl_command_result(command, &stdout, &stderr)
        .map_err(|e| {
            println!("Error parsing curl output: {:?}", e);
            CommandExecutionError::OutputParseError(format!("Failed to parse curl output: {}", e))
        })?;

    Ok(result)
}