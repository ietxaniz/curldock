use crate::curl_gateway::models::{CurlCommand, HttpMethod, CommandExecutionError};

pub fn generate_curl_command(command: &CurlCommand) -> Result<String, CommandExecutionError> {
    let mut parts = vec!["#!/bin/sh".to_string(), "curl".to_string()];

    // Boolean options
    add_flag(&mut parts, command.options.verbose, "-v");
    add_flag(&mut parts, command.options.insecure, "-k");
    add_flag(&mut parts, command.options.follow_redirects, "-L");
    add_flag(&mut parts, command.options.compressed, "--compressed");
    add_flag(&mut parts, command.options.fail, "-f");

    // HTTP method
    match command.method {
        HttpMethod::GET => {},  // GET is default, so we don't need to specify it
        _ => parts.push(format!("-X {}", http_method_to_string(&command.method))),
    }

    // Headers and data
    for (key, value) in &command.headers {
        parts.push(format!("-H '{}: {}'", key, value));
    }
    if let Some(data) = &command.data {
        if !data.is_empty() {
            parts.push(format!("-d '{}'", data));
        }
    }

    // Cookies
    for (key, value) in &command.cookies {
        parts.push(format!("--cookie '{}={}'", key, value));
    }

    // Numeric options
    add_option(&mut parts, &command.options.max_redirects, "--max-redirs");
    add_option(&mut parts, &command.options.timeout, "--max-time");
    add_option(&mut parts, &command.options.connect_timeout, "--connect-timeout");
    add_option(&mut parts, &command.options.retry, "--retry");
    add_option(&mut parts, &command.options.retry_delay, "--retry-delay");
    add_option(&mut parts, &command.options.max_time, "--max-time");

    // String options
    add_string_option(&mut parts, &command.options.proxy, "-x");
    add_string_option(&mut parts, &command.options.output_file, "-o");
    add_string_option(&mut parts, &command.options.cert, "--cert");
    add_string_option(&mut parts, &command.options.key, "--key");
    add_string_option(&mut parts, &command.options.key_password, "--pass");
    add_string_option(&mut parts, &command.options.interface, "--interface");

    // DNS servers
    if let Some(dns_servers) = &command.options.dns_servers {
        parts.push(format!("--dns-servers {}", dns_servers.join(",")));
    }

    // IP version
    if command.options.ipv4_only == Some(true) {
        parts.push("-4".to_string());
    } else if command.options.ipv6_only == Some(true) {
        parts.push("-6".to_string());
    }

    // Rate limit
    if let Some(rate) = &command.options.rate_limit {
        parts.push(format!("--limit-rate {}k", rate));
    }

    // Timing options
    let mut time_format = String::new();
    if command.options.time_namelookup == Some(true) { time_format.push_str("\\nNamelookup: %{time_namelookup}"); }
    if command.options.time_connect == Some(true) { time_format.push_str("\\nConnect: %{time_connect}"); }
    if command.options.time_appconnect == Some(true) { time_format.push_str("\\nAppconnect: %{time_appconnect}"); }
    if command.options.time_pretransfer == Some(true) { time_format.push_str("\\nPretransfer: %{time_pretransfer}"); }
    if command.options.time_starttransfer == Some(true) { time_format.push_str("\\nStarttransfer: %{time_starttransfer}"); }
    if command.options.time_total == Some(true) { time_format.push_str("\\nTotal: %{time_total}"); }

    if !time_format.is_empty() {
        parts.push(format!("--write-out '{}'", time_format));
    }

    // URL (should be last)
    parts.push(command.url.clone());

    // Join parts with line breaks and indentation, but don't add a backslash to the shebang line
    let result = parts
    .iter()
    .enumerate()
    .map(|(i, part)| {
        if part.starts_with('#') {
            part.to_string()
        } else if i == parts.len() - 1 {
            format!("  {}", part)
        } else {
            format!("  {} \\", part)
        }
    })
    .collect::<Vec<String>>()
    .join("\n");

Ok(result)
}

fn http_method_to_string(method: &HttpMethod) -> &'static str {
    match method {
        HttpMethod::GET => "GET",
        HttpMethod::POST => "POST",
        HttpMethod::PUT => "PUT",
        HttpMethod::DELETE => "DELETE",
        HttpMethod::PATCH => "PATCH",
        HttpMethod::HEAD => "HEAD",
        HttpMethod::OPTIONS => "OPTIONS",
    }
}

fn add_flag(parts: &mut Vec<String>, option: Option<bool>, flag: &str) {
    if option == Some(true) {
        parts.push(flag.to_string());
    }
}

fn add_option<T: std::fmt::Display>(parts: &mut Vec<String>, option: &Option<T>, flag: &str) {
    if let Some(value) = option {
        parts.push(format!("{} {}", flag, value));
    }
}

fn add_string_option(parts: &mut Vec<String>, option: &Option<String>, flag: &str) {
    if let Some(value) = option {
        parts.push(format!("{} '{}'", flag, value));
    }
}