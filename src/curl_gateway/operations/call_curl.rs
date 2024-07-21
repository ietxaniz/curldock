use crate::debug_err;
use anyhow::Result;
use std::collections::HashMap;
use std::process::Command;

pub fn call_curl(args: &[String], env_vars: &HashMap<String, String>) -> Result<(String, String)> {
    let curl_command_str = format!("curl {}", args.join(" "));
    let output: std::process::Output = debug_err!(
        Command::new("sh")
            .arg("-c")
            .arg(&curl_command_str)
            .envs(env_vars)
            .output(),
        "Failed to execute curl command"
    )?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok((stdout, stderr))
}
