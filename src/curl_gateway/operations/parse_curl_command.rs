use crate::curl_gateway::models::{CurlCommand, HttpMethod, CurlOptions, ParseError};
use regex::Regex;

pub fn parse_curl_command(script: &str) -> Result<CurlCommand, ParseError> {
    // Preprocess the script
    let preprocessed_script = preprocess_script(script);

    // Join all lines into a single string, preserving only necessary whitespace
    let curl_command = preprocessed_script.replace("\\\n", " ").replace("\n", " ");

    if !curl_command.starts_with("curl ") {
        return Err(ParseError::MissingCurlCommand);
    }

    let method_regex = Regex::new(r"-X\s+(\w+)").unwrap();
    let url_regex = Regex::new(r"(https?://\S+)\s*$").unwrap();
    let header_regex = Regex::new(r"-H\s+'([^']+)'").unwrap();
    let data_regex = Regex::new(r"-d\s+'([^']*)'").unwrap();
    let cookie_regex = Regex::new(r"--cookie\s+'([^']+)'").unwrap();

    // Boolean option regexes
    let verbose_regex = Regex::new(r"\s-v\b").unwrap();
    let insecure_regex = Regex::new(r"\s-k\b").unwrap();
    let follow_redirects_regex = Regex::new(r"\s-L\b").unwrap();
    let compressed_regex = Regex::new(r"\s--compressed\b").unwrap();
    let fail_regex = Regex::new(r"\s-f\b").unwrap();

    // Other option regexes
    let max_redirects_regex = Regex::new(r"--max-redirs\s+(\d+)").unwrap();
    let timeout_regex = Regex::new(r"--max-time\s+(\d+)").unwrap();
    let connect_timeout_regex = Regex::new(r"--connect-timeout\s+(\d+)").unwrap();
    let proxy_regex = Regex::new(r"-x\s+'([^']+)'").unwrap();
    let output_file_regex = Regex::new(r"-o\s+'([^']+)'").unwrap();
    let cert_regex = Regex::new(r"--cert\s+'([^']+)'").unwrap();
    let key_regex = Regex::new(r"--key\s+'([^']+)'").unwrap();
    let key_password_regex = Regex::new(r"--pass\s+'([^']+)'").unwrap();
    let retry_regex = Regex::new(r"--retry\s+(\d+)").unwrap();
    let retry_delay_regex = Regex::new(r"--retry-delay\s+(\d+)").unwrap();
    let interface_regex = Regex::new(r"--interface\s+'([^']+)'").unwrap();
    let dns_servers_regex = Regex::new(r"--dns-servers\s+([^

 ]+)").unwrap();
    let ipv4_regex = Regex::new(r"\s-4\b").unwrap();
    let ipv6_regex = Regex::new(r"\s-6\b").unwrap();
    let rate_limit_regex = Regex::new(r"--limit-rate\s+(\d+)k").unwrap();
    let write_out_regex = Regex::new(r"--write-out\s+'([^']+)'").unwrap();

    let method = method_regex
        .captures(&curl_command)
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
        .captures(&curl_command)
        .ok_or(ParseError::MissingUrl)?[1]
        .to_string();

    let headers = header_regex
        .captures_iter(&curl_command)
        .map(|cap| {
            let parts: Vec<&str> = cap[1].splitn(2, ':').collect();
            (parts[0].trim().to_string(), parts[1].trim().to_string())
        })
        .collect();

    let data = data_regex
        .captures(&curl_command)
        .map(|cap| cap[1].to_string());

    let cookies = cookie_regex
        .captures_iter(&curl_command)
        .map(|cap| {
            let parts: Vec<&str> = cap[1].splitn(2, '=').collect();
            (parts[0].trim().to_string(), parts[1].trim().to_string())
        })
        .collect();

    let write_out = write_out_regex.captures(&curl_command).map(|cap| cap[1].to_string());

    let options = CurlOptions {
        verbose: Some(verbose_regex.is_match(&curl_command)),
        insecure: Some(insecure_regex.is_match(&curl_command)),
        follow_redirects: Some(follow_redirects_regex.is_match(&curl_command)),
        compressed: Some(compressed_regex.is_match(&curl_command)),
        fail: Some(fail_regex.is_match(&curl_command)),
        max_redirects: max_redirects_regex.captures(&curl_command).and_then(|cap| cap[1].parse().ok()),
        timeout: timeout_regex.captures(&curl_command).and_then(|cap| cap[1].parse().ok()),
        connect_timeout: connect_timeout_regex.captures(&curl_command).and_then(|cap| cap[1].parse().ok()),
        max_time: timeout_regex.captures(&curl_command).and_then(|cap| cap[1].parse().ok()),
        proxy: proxy_regex.captures(&curl_command).map(|cap| cap[1].to_string()),
        output_file: output_file_regex.captures(&curl_command).map(|cap| cap[1].to_string()),
        cert: cert_regex.captures(&curl_command).map(|cap| cap[1].to_string()),
        key: key_regex.captures(&curl_command).map(|cap| cap[1].to_string()),
        key_password: key_password_regex.captures(&curl_command).map(|cap| cap[1].to_string()),
        retry: retry_regex.captures(&curl_command).and_then(|cap| cap[1].parse().ok()),
        retry_delay: retry_delay_regex.captures(&curl_command).and_then(|cap| cap[1].parse().ok()),
        interface: interface_regex.captures(&curl_command).map(|cap| cap[1].to_string()),
        dns_servers: dns_servers_regex.captures(&curl_command).map(|cap| cap[1].split(',').map(String::from).collect()),
        ipv4_only: Some(ipv4_regex.is_match(&curl_command)),
        ipv6_only: Some(ipv6_regex.is_match(&curl_command)),
        rate_limit: rate_limit_regex.captures(&curl_command).and_then(|cap| cap[1].parse().ok()),
        time_namelookup: Some(write_out.as_ref().map_or(false, |w| w.contains("Namelookup:"))),
        time_connect: Some(write_out.as_ref().map_or(false, |w| w.contains("Connect:"))),
        time_appconnect: Some(write_out.as_ref().map_or(false, |w| w.contains("Appconnect:"))),
        time_pretransfer: Some(write_out.as_ref().map_or(false, |w| w.contains("Pretransfer:"))),
        time_starttransfer: Some(write_out.as_ref().map_or(false, |w| w.contains("Starttransfer:"))),
        time_total: Some(write_out.as_ref().map_or(false, |w| w.contains("Total:"))),
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

fn preprocess_script(script: &str) -> String {
    script
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}