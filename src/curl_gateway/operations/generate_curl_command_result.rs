use crate::curl_gateway::models::{CurlCommand, CurlCommandResult};
use crate::script_manager::models::ScriptError;
use std::collections::HashMap;

pub fn generate_curl_command_result(command: CurlCommand, stdout: &str) -> Result<CurlCommandResult, ScriptError> {
    let mut response_headers = HashMap::new();
    let mut cookies = HashMap::new();
    let mut status_code = 0;
    let mut date = String::new();
    let mut body = String::new();
    let mut content_type = None;
    let mut redirect_count = None;
    let mut effective_url = None;
    let mut in_body = false;
    let mut time_namelookup = None;
    let mut time_connect = None;
    let mut time_appconnect = None;
    let mut time_pretransfer = None;
    let mut time_starttransfer = None;
    let mut time_total = None;

    for line in stdout.lines() {
        if line.starts_with("< HTTP/") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                status_code = parts[1].parse::<u16>()
                    .map_err(|_| ScriptError::OutputParseError("Failed to parse status code".to_string()))?;
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
                } else if key.to_lowercase() == "content-type" {
                    content_type = Some(value);
                } else if key.to_lowercase() == "set-cookie" {
                    parse_cookie(&value, &mut cookies);
                }
            }
        } else if line.starts_with("* Redirect") {
            redirect_count = Some(redirect_count.unwrap_or(0) + 1);
        } else if line.starts_with("* Effective URL:") {
            effective_url = Some(line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string());
        } else if line.is_empty() {
            in_body = true;
        } else if in_body {
            body.push_str(line);
            body.push('\n');
        } else if line.starts_with("time_namelookup:") {
            time_namelookup = parse_time(line);
        } else if line.starts_with("time_connect:") {
            time_connect = parse_time(line);
        } else if line.starts_with("time_appconnect:") {
            time_appconnect = parse_time(line);
        } else if line.starts_with("time_pretransfer:") {
            time_pretransfer = parse_time(line);
        } else if line.starts_with("time_starttransfer:") {
            time_starttransfer = parse_time(line);
        } else if line.starts_with("time_total:") {
            time_total = parse_time(line);
        }
    }

    body = body.trim().to_string();

    if status_code == 0 {
        return Err(ScriptError::OutputParseError("No valid status code found in response".to_string()));
    }

    Ok(CurlCommandResult {
        request: command,
        response_headers,
        status_code,
        date,
        body,
        cookies,
        content_type,
        redirect_count,
        effective_url,
        time_namelookup,
        time_connect,
        time_appconnect,
        time_pretransfer,
        time_starttransfer,
        time_total,
    })
}

fn parse_cookie(cookie_str: &str, cookies: &mut HashMap<String, String>) {
    let parts: Vec<&str> = cookie_str.split(';').collect();
    if let Some(main_part) = parts.first() {
        let cookie_parts: Vec<&str> = main_part.splitn(2, '=').collect();
        if cookie_parts.len() == 2 {
            cookies.insert(cookie_parts[0].trim().to_string(), cookie_parts[1].trim().to_string());
        }
    }
}

fn parse_time(line: &str) -> Option<u32> {
    line.split(':')
        .nth(1)
        .and_then(|s| s.trim().parse::<f64>().ok())
        .map(|f| (f * 1000.0) as u32)
}