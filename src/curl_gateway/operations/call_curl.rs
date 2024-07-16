use crate::curl_gateway::errors::{CurlGatewayError, ErrorKind};
use std::collections::HashMap;
use std::process::Command;

pub fn call_curl(
    args: &[String],
    env_vars: &HashMap<String, String>,
) -> Result<(String, String), CurlGatewayError> {
    let curl_command_str = format!("curl {}", args.join(" "));
    let output: std::process::Output = Command::new("sh")
        .arg("-c")
        .arg(&curl_command_str)
        .envs(env_vars)
        .output()
        .map_err(|e| {
            CurlGatewayError::with_source(
                ErrorKind::Execution,
                "call_curl",
                "Failed to execute curl command",
                Box::new(e)
            )
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok((stdout, stderr))
}