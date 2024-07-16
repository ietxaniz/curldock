use crate::curl_gateway::errors::{CurlGatewayError, ErrorKind};
use crate::curl_gateway::models::{CurlCommand, CurlCommandResult, StoreData};
use crate::curl_gateway::utils::extract_data;
use regex::Regex;
use std::collections::HashMap;

pub fn generate_curl_command_result(
    command: CurlCommand,
    stdout: &str,
    stderr: &str,
) -> Result<CurlCommandResult, CurlGatewayError> {
    let mut response_headers = HashMap::new();
    let mut cookies = HashMap::new();
    let mut status_code = 0;
    let mut date = String::new();
    let mut content_type = None;
    let mut redirect_count = None;
    let mut effective_url = None;
    let mut time_namelookup = None;
    let mut time_connect = None;
    let mut time_appconnect = None;
    let mut time_pretransfer = None;
    let mut time_starttransfer = None;
    let mut time_total = None;

    // Parse stdout: separate timing info from body
    let (body, timing_info) = split_body_and_timing(stdout);

    // Parse timing info
    for line in timing_info.lines() {
        if line.starts_with("Namelookup:") {
            parse_timing_line(line, &mut time_namelookup);
        } else if line.starts_with("Connect:") {
            parse_timing_line(line, &mut time_connect);
        } else if line.starts_with("Appconnect:") {
            parse_timing_line(line, &mut time_appconnect);
        } else if line.starts_with("Pretransfer:") {
            parse_timing_line(line, &mut time_pretransfer);
        } else if line.starts_with("Starttransfer:") {
            parse_timing_line(line, &mut time_starttransfer);
        } else if line.starts_with("Total:") {
            parse_timing_line(line, &mut time_total);
        }
    }

    // Parse status code, headers, and other info from stderr
    let status_regex = Regex::new(r"< HTTP/\d+\.\d+\s+(\d+)").map_err(|e| {
        CurlGatewayError::with_source(
            ErrorKind::Parsing,
            "generate_curl_command_result",
            "Failed to create status regex",
            Box::new(e)
        )
    })?;
    let header_regex = Regex::new(r"< ([^:]+):\s*(.+)").map_err(|e| {
        CurlGatewayError::with_source(
            ErrorKind::Parsing,
            "generate_curl_command_result",
            "Failed to create header regex",
            Box::new(e)
        )
    })?;

    for line in stderr.lines() {
        if let Some(cap) = status_regex.captures(line) {
            status_code = cap[1].parse().unwrap_or(0);
        } else if let Some(cap) = header_regex.captures(line) {
            let key = cap[1].trim().to_string();
            let value = cap[2].trim().to_string();
            response_headers.insert(key.clone(), value.clone());
            if key.to_lowercase() == "date" {
                date = value;
            } else if key.to_lowercase() == "content-type" {
                content_type = Some(value);
            } else if key.to_lowercase() == "set-cookie" {
                parse_cookie(&value, &mut cookies);
            }
        } else if line.starts_with("* Redirect") {
            redirect_count = Some(redirect_count.unwrap_or(0) + 1);
        } else if line.starts_with("* Effective URL:") {
            effective_url = Some(line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string());
        }
    }

    let mut store_data = Vec::new();

    for store_curl in &command.store_curl_body {
        match extract_data(&body, &store_curl.source) {
            Ok(extracted_data) => {
                store_data.push(StoreData {
                    parameter: store_curl.destination.clone(),
                    filename: store_curl.filename.clone(),
                    data: extracted_data,
                });
            }
            Err(e) => {
                return Err(CurlGatewayError::with_source(
                    ErrorKind::DataHandling,
                    "generate_curl_command_result",
                    format!("Failed to extract data from {}", store_curl.source),
                    Box::new(e)
                ));
            }
        }
    }

    for store_cookie in &command.store_curl_cookie {
        if let Some(cookie_value) = cookies.get(&store_cookie.source) {
            store_data.push(StoreData {
                parameter: store_cookie.destination.clone(),
                filename: store_cookie.filename.clone(),
                data: cookie_value.clone(),
            });
        } else {
            return Err(CurlGatewayError::new(
                ErrorKind::DataHandling,
                "generate_curl_command_result",
                format!("Cookie '{}' not found", store_cookie.source)
            ));
        }
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
        store_data,
    })
}

fn split_body_and_timing(stdout: &str) -> (String, String) {
    if let Some(timing_start) = stdout.rfind("\nNamelookup:") {
        let (body, timing) = stdout.split_at(timing_start);
        (body.trim().to_string(), timing.trim().to_string())
    } else {
        (stdout.trim().to_string(), String::new())
    }
}

fn parse_cookie(cookie_str: &str, cookies: &mut HashMap<String, String>) {
    let parts: Vec<&str> = cookie_str.split(';').collect();
    if let Some(main_part) = parts.first() {
        let cookie_parts: Vec<&str> = main_part.splitn(2, '=').collect();
        if cookie_parts.len() == 2 {
            cookies.insert(
                cookie_parts[0].trim().to_string(),
                cookie_parts[1].trim().to_string(),
            );
        }
    }
}

fn parse_timing_line(line: &str, time: &mut Option<u32>) {
    if let Some(value_str) = line.split(':').nth(1) {
        if let Ok(value) = value_str.trim().parse::<f64>() {
            *time = Some((value * 1_000_000.0) as u32);
        }
    }
}
