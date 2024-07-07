use crate::curl_gateway::models::{CurlCommand, CurlCommandResult, CommandExecutionError};
use crate::curl_gateway::operations::{generate_curl_command, generate_curl_command_result};
use std::process::Command;

pub fn execute_curl_command(command: CurlCommand) -> Result<CurlCommandResult, CommandExecutionError> {
    // Generate the curl command string
    let curl_command_str = generate_curl_command(&command)
        .strip_prefix("#!/bin/sh\n\n")
        .ok_or_else(|| CommandExecutionError::CommandGenerationError("Failed to strip prefix from curl command".to_string()))?
        .to_string();

    // Execute the curl command
    let output = Command::new("sh")
        .arg("-c")
        .arg(&curl_command_str)
        .output()
        .map_err(|e| CommandExecutionError::ExecutionError(format!("Failed to execute curl command: {}", e)))?;

    // Capture the stdout
    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| CommandExecutionError::OutputCaptureError(format!("Failed to capture curl output: {}", e)))?;

    // Generate the CurlCommandResult
    let mut result = generate_curl_command_result(command, &stdout)
        .map_err(|e| CommandExecutionError::OutputParseError(format!("Failed to parse curl output: {}", e)))?;

    // Check if the command was successful
    if !output.status.success() {
        if result.status_code == 0 {
            return Err(CommandExecutionError::InvalidStatusCode(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        // If we have a valid status code, we'll keep it in the result
        // but also include the error message in the body
        result.body = format!("Error: {}\n\n{}", String::from_utf8_lossy(&output.stderr), result.body);
    }

    Ok(result)
}
