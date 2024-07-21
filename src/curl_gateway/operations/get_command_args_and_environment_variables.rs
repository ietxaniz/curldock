use crate::curl_gateway::models::{CurlCommand, HttpMethod};
use crate::debug_err;
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

pub fn get_command_args_and_environment_variables(
    command: &CurlCommand,
) -> Result<(Vec<String>, HashMap<String, String>)> {
    let mut args = Vec::new();
    let mut env_vars = HashMap::new();

    // Extract curl arguments
    args.push("-v".to_string()); // Always include verbose mode

    // Always include timing information
    args.push("--write-out".to_string());
    args.push("'\\nNamelookup: %{time_namelookup}\\nConnect: %{time_connect}\\nAppconnect: %{time_appconnect}\\nPretransfer: %{time_pretransfer}\\nStarttransfer: %{time_starttransfer}\\nTotal: %{time_total}'".to_string());

    // Add method if not GET
    if command.method != HttpMethod::GET {
        args.push("-X".to_string());
        args.push(command.method.to_string());
    }

    // Add headers
    for (key, value) in &command.headers {
        args.push("-H".to_string());
        args.push(format!("'{}: {}'", key, value));
    }

    // Add data if present
    if let Some(data) = &command.data {
        args.push("-d".to_string());
        args.push(format!("'{}'", data));
    }

    // Add cookies
    for (key, value) in &command.cookies {
        args.push("--cookie".to_string());
        args.push(format!("'{}={}'", key, value));
    }

    // Add other options
    if let Some(true) = command.options.insecure {
        args.push("-k".to_string());
    }
    if let Some(true) = command.options.follow_redirects {
        args.push("-L".to_string());
    }
    if let Some(max_redirects) = command.options.max_redirects {
        args.push("--max-redirs".to_string());
        args.push(max_redirects.to_string());
    }
    if let Some(timeout) = command.options.timeout {
        args.push("--max-time".to_string());
        args.push(timeout.to_string());
    }
    if let Some(connect_timeout) = command.options.connect_timeout {
        args.push("--connect-timeout".to_string());
        args.push(connect_timeout.to_string());
    }
    if let Some(ref proxy) = command.options.proxy {
        args.push("-x".to_string());
        args.push(proxy.clone());
    }
    if let Some(ref output_file) = command.options.output_file {
        args.push("-o".to_string());
        args.push(output_file.clone());
    }
    if let Some(ref cert) = command.options.cert {
        args.push("--cert".to_string());
        args.push(cert.clone());
    }
    if let Some(ref key) = command.options.key {
        args.push("--key".to_string());
        args.push(key.clone());
    }
    if let Some(ref key_password) = command.options.key_password {
        args.push("--pass".to_string());
        args.push(key_password.clone());
    }
    if let Some(true) = command.options.compressed {
        args.push("--compressed".to_string());
    }
    if let Some(retry) = command.options.retry {
        args.push("--retry".to_string());
        args.push(retry.to_string());
    }
    if let Some(retry_delay) = command.options.retry_delay {
        args.push("--retry-delay".to_string());
        args.push(retry_delay.to_string());
    }
    if let Some(true) = command.options.fail {
        args.push("-f".to_string());
    }
    if let Some(ref interface) = command.options.interface {
        args.push("--interface".to_string());
        args.push(interface.clone());
    }
    if let Some(ref dns_servers) = command.options.dns_servers {
        args.push("--dns-servers".to_string());
        args.push(dns_servers.join(","));
    }
    if let Some(true) = command.options.ipv4_only {
        args.push("-4".to_string());
    }
    if let Some(true) = command.options.ipv6_only {
        args.push("-6".to_string());
    }
    if let Some(max_time) = command.options.max_time {
        args.push("--max-time".to_string());
        args.push(max_time.to_string());
    }
    if let Some(rate_limit) = command.options.rate_limit {
        args.push("--limit-rate".to_string());
        args.push(format!("{}k", rate_limit));
    }

    // Add URL
    args.push(command.url.clone());

    // Process loaddata instructions
    for load in &command.load_curl {
        let file_content = debug_err!(
            fs::read_to_string(&load.filename),
            "Failed to load data from {}", load.filename
        )?;

        let json: Value = debug_err!(
            serde_json::from_str(&file_content),
            "Failed to parse JSON from {}", load.filename
        )?;
        

        if let Some(value) = json.get(&load.data_name) {
            let env_value = value
                .as_str()
                .map(|s| s.to_string())
                .or_else(|| value.as_u64().map(|n| n.to_string()))
                .or_else(|| value.as_f64().map(|n| n.to_string()))
                .unwrap_or_else(|| value.to_string());
            env_vars.insert(load.env_variable.clone(), env_value);
        } else {
            return Err(anyhow!("Key '{}' not found in {}", load.data_name, load.filename));
        }
    }

    Ok((args, env_vars))
}
