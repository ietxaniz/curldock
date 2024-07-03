use crate::curl_gateway::models::{CurlCommand, CurlCommandResult};
use std::collections::HashMap;

pub fn generate_curl_command_result(command: CurlCommand, stdout: &str) -> CurlCommandResult {
    let mut response_headers = HashMap::new();
    let mut status_code = 0;
    let mut date = String::new();
    let mut body = String::new();
    let mut in_body = false;

    for line in stdout.lines() {
        if line.starts_with("< HTTP/") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(code) = parts[1].parse::<u16>() {
                    status_code = code;
                }
            }
        } else if line.starts_with("< ") {
            let header = &line[2..];
            let parts: Vec<&str> = header.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                response_headers.insert(key.clone(), value.clone());
                if key.to_lowercase() == "date" {
                    date = value;
                }
            }
        } else if line.is_empty() {
            in_body = true;
        } else if in_body {
            body.push_str(line);
            body.push('\n');
        }
    }

    body = body.trim().to_string();

    CurlCommandResult {
        request: command,
        response_headers,
        status_code,
        date,
        body,
    }
}
