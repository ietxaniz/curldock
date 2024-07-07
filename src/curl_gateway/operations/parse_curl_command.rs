use crate::curl_gateway::models::{CurlCommand, HttpMethod, CurlOptions, ParseError};
use regex::Regex;

pub fn parse_curl_command(script: &str) -> Result<CurlCommand, ParseError> {
    let curl_line = script.lines()
        .find(|line| line.trim().starts_with("curl"))
        .ok_or(ParseError::MissingCurlCommand)?;

    let method_regex = Regex::new(r"-X\s+(\w+)").unwrap();
    let url_regex = Regex::new(r"curl.*?\s(https?://\S+)").unwrap();
    let header_regex = Regex::new(r"-H\s+'([^']+)'").unwrap();
    let data_regex = Regex::new(r"-d\s+'([^']+)'").unwrap();
    let cookie_regex = Regex::new(r"-b\s+'([^']+)'").unwrap();

    // Boolean option regexes
    let verbose_regex = Regex::new(r"\s-v\b|\s--verbose\b").unwrap();
    let no_verbose_regex = Regex::new(r"\s--no-verbose\b").unwrap();
    let insecure_regex = Regex::new(r"\s-k\b|\s--insecure\b").unwrap();
    let no_insecure_regex = Regex::new(r"\s--no-insecure\b").unwrap();
    let follow_redirects_regex = Regex::new(r"\s-L\b|\s--location\b").unwrap();
    let no_follow_redirects_regex = Regex::new(r"\s--no-location\b").unwrap();
    let compressed_regex = Regex::new(r"\s--compressed\b").unwrap();
    let no_compressed_regex = Regex::new(r"\s--no-compressed\b").unwrap();
    let fail_regex = Regex::new(r"\s-f\b|\s--fail\b").unwrap();
    let no_fail_regex = Regex::new(r"\s--no-fail\b").unwrap();

    // Other option regexes
    let max_redirects_regex = Regex::new(r"--max-redirs\s+(\d+)").unwrap();
    let timeout_regex = Regex::new(r"--max-time\s+(\d+)").unwrap();
    let max_time_regex = Regex::new(r"--max-time\s+(\d+)").unwrap();
    let connect_timeout_regex = Regex::new(r"--connect-timeout\s+(\d+)").unwrap();
    let proxy_regex = Regex::new(r"-x\s+(\S+)").unwrap();
    let output_file_regex = Regex::new(r"-o\s+(\S+)").unwrap();
    let cert_regex = Regex::new(r"--cert\s+(\S+)").unwrap();
    let key_regex = Regex::new(r"--key\s+(\S+)").unwrap();
    let key_password_regex = Regex::new(r"--key-type\s+(\S+)").unwrap();
    let retry_regex = Regex::new(r"--retry\s+(\d+)").unwrap();
    let retry_delay_regex = Regex::new(r"--retry-delay\s+(\d+)").unwrap();
    let interface_regex = Regex::new(r"--interface\s+(\S+)").unwrap();
    let dns_servers_regex = Regex::new(r"--dns-servers\s+(\S+)").unwrap();
    let ipv4_regex = Regex::new(r"-4\b|\s--ipv4\b").unwrap();
    let ipv6_regex = Regex::new(r"-6\b|\s--ipv6\b").unwrap();
    let rate_limit_regex = Regex::new(r"--limit-rate\s+(\d+)").unwrap();

    let method = method_regex
        .captures(curl_line)
        .map(|cap| match cap[1].to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            m => Err(ParseError::InvalidMethod(m.to_string())),
        })
        .unwrap_or(Ok(HttpMethod::GET))?;

    let url = url_regex
        .captures(curl_line)
        .ok_or(ParseError::MissingUrl)?[1]
        .to_string();

    let headers = header_regex
        .captures_iter(curl_line)
        .map(|cap| {
            let parts: Vec<&str> = cap[1].splitn(2, ':').collect();
            (parts[0].trim().to_string(), parts[1].trim().to_string())
        })
        .collect();

    let data = data_regex
        .captures(curl_line)
        .map(|cap| cap[1].to_string());

    let cookies = cookie_regex
        .captures_iter(curl_line)
        .map(|cap| {
            let parts: Vec<&str> = cap[1].splitn(2, '=').collect();
            (parts[0].trim().to_string(), parts[1].trim().to_string())
        })
        .collect();

    let options = CurlOptions {
        verbose: parse_bool_option(curl_line, &verbose_regex, &no_verbose_regex),
        insecure: parse_bool_option(curl_line, &insecure_regex, &no_insecure_regex),
        follow_redirects: parse_bool_option(curl_line, &follow_redirects_regex, &no_follow_redirects_regex),
        max_redirects: max_redirects_regex.captures(curl_line).and_then(|cap| cap[1].parse().ok()),
        timeout: timeout_regex.captures(curl_line).and_then(|cap| cap[1].parse().ok()),
        connect_timeout: connect_timeout_regex.captures(curl_line).and_then(|cap| cap[1].parse().ok()),
        max_time: max_time_regex.captures(curl_line).and_then(|cap| cap[1].parse().ok()),
        proxy: proxy_regex.captures(curl_line).map(|cap| cap[1].to_string()),
        output_file: output_file_regex.captures(curl_line).map(|cap| cap[1].to_string()),
        cert: cert_regex.captures(curl_line).map(|cap| cap[1].to_string()),
        key: key_regex.captures(curl_line).map(|cap| cap[1].to_string()),
        key_password: key_password_regex.captures(curl_line).map(|cap| cap[1].to_string()),
        compressed: parse_bool_option(curl_line, &compressed_regex, &no_compressed_regex),
        retry: retry_regex.captures(curl_line).and_then(|cap| cap[1].parse().ok()),
        retry_delay: retry_delay_regex.captures(curl_line).and_then(|cap| cap[1].parse().ok()),
        fail: parse_bool_option(curl_line, &fail_regex, &no_fail_regex),
        interface: interface_regex.captures(curl_line).map(|cap| cap[1].to_string()),
        dns_servers: dns_servers_regex.captures(curl_line).map(|cap| cap[1].split(',').map(String::from).collect()),
        ipv4_only: Some(ipv4_regex.is_match(curl_line)),
        ipv6_only: Some(ipv6_regex.is_match(curl_line)),
        rate_limit: rate_limit_regex.captures(curl_line).and_then(|cap| cap[1].parse().ok()),
        time_namelookup: None,
        time_connect: None,
        time_appconnect: None,
        time_pretransfer: None,
        time_starttransfer: None,
        time_total: None,
    };

    Ok(CurlCommand {
        method,
        url,
        headers,
        data,
        cookies,
        options,
    })
}

fn parse_bool_option(curl_line: &str, positive_regex: &Regex, negative_regex: &Regex) -> Option<bool> {
    if positive_regex.is_match(curl_line) {
        Some(true)
    } else if negative_regex.is_match(curl_line) {
        Some(false)
    } else {
        None
    }
}
