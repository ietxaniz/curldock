use crate::curl_gateway::models::{CurlCommand, HttpMethod};

pub fn generate_curl_command(command: &CurlCommand) -> String {
    let mut parts = vec!["#!/bin/sh\n\ncurl".to_string()];

    add_boolean_option(&mut parts, command.options.verbose, "-v", "--no-verbose");
    add_boolean_option(&mut parts, command.options.insecure, "-k", "--no-insecure");
    add_boolean_option(&mut parts, command.options.follow_redirects, "-L", "--no-location");
    add_boolean_option(&mut parts, command.options.compressed, "--compressed", "--no-compressed");
    add_boolean_option(&mut parts, command.options.fail, "-f", "--no-fail");

    if command.method != HttpMethod::GET {
        parts.push(format!("-X {}", http_method_to_string(&command.method)));
    }

    for (key, value) in &command.headers {
        parts.push(format!("-H '{}: {}'", key, value));
    }

    if let Some(data) = &command.data {
        parts.push(format!("-d '{}'", data));
    }

    for (key, value) in &command.cookies {
        parts.push(format!("-b '{}={}'", key, value));
    }

    add_option(&mut parts, &command.options.max_redirects, "--max-redirs");
    add_option(&mut parts, &command.options.timeout, "--connect-timeout");
    add_option(&mut parts, &command.options.connect_timeout, "--connect-timeout");
    add_option(&mut parts, &command.options.max_time, "--max-time");
    add_string_option(&mut parts, &command.options.proxy, "-x");
    add_string_option(&mut parts, &command.options.output_file, "-o");
    add_string_option(&mut parts, &command.options.cert, "--cert");
    add_string_option(&mut parts, &command.options.key, "--key");
    add_string_option(&mut parts, &command.options.key_password, "--key-type");
    add_option(&mut parts, &command.options.retry, "--retry");
    add_option(&mut parts, &command.options.retry_delay, "--retry-delay");
    add_string_option(&mut parts, &command.options.interface, "--interface");
    
    if let Some(dns_servers) = &command.options.dns_servers {
        parts.push(format!("--dns-servers {}", dns_servers.join(",")));
    }
    
    if command.options.ipv4_only == Some(true) {
        parts.push("-4".to_string());
    }
    
    if command.options.ipv6_only == Some(true) {
        parts.push("-6".to_string());
    }
    
    add_option(&mut parts, &command.options.rate_limit, "--limit-rate");

    parts.push(command.url.clone());

    parts.join(" ")
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

fn add_boolean_option(parts: &mut Vec<String>, option: Option<bool>, true_flag: &str, false_flag: &str) {
    match option {
        Some(true) => parts.push(true_flag.to_string()),
        Some(false) => parts.push(false_flag.to_string()),
        None => {}
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