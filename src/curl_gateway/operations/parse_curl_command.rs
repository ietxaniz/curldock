use crate::curl_gateway::models::{
    CurlCommand, CurlOptions, HttpMethod, LoadCurl, StoreCurlBody, StoreCurlCookie,
};
use crate::debug_err;
use anyhow::{anyhow, Error, Result};
use regex::Regex;
use std::str::FromStr;

pub fn parse_curl_command(script: &str) -> Result<CurlCommand> {
    let preprocessed_script = preprocess_script(script);
    let curl_command = preprocessed_script.replace("\\\n", " ").replace("\n", " ");

    if !curl_command.starts_with("curl ") {
        return Err(anyhow!("No curl command found in script"));
    }

    let method = parse_method(&curl_command)?;
    let url = parse_url(&curl_command)?;
    let headers = parse_headers(&curl_command);
    let data = parse_data(&curl_command);
    let cookies = parse_cookies(&curl_command);
    let options = parse_options(&curl_command);
    let store_curl_body = parse_store_curl_body(script);
    let store_curl_cookie = parse_store_curl_cookie(script);
    let load_curl = parse_load_curl(script);

    Ok(CurlCommand {
        method,
        url,
        headers,
        data,
        cookies,
        options,
        store_curl_body,
        store_curl_cookie,
        load_curl,
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

fn parse_method(command: &str) -> Result<HttpMethod> {
    let method_regex = debug_err!(Regex::new(r"-X\s+(\w+)"))?;
    match method_regex.captures(command) {
        Some(cap) => debug_err!(
            HttpMethod::from_str(&cap[1].to_uppercase()),
            "Invalid HTTP method {}",
            &cap[1].to_uppercase()
        ),
        None => Ok(HttpMethod::GET),
    }
}

fn parse_url(command: &str) -> Result<String> {
    let url_regex = debug_err!(Regex::new(r"(https?://\S+)"))?;
    match url_regex.captures(command) {
        Some(cap) => Ok(cap[1].to_string()),
        None => debug_err!(
            Err(anyhow::anyhow!("Missing URL in curl command")),
            "Failed to parse URL"
        )?,
    }
}

fn parse_headers(command: &str) -> Vec<(String, String)> {
    let header_regex = Regex::new(r#"-H\s+('.*?'|".*?"|\S+)"#).unwrap();
    header_regex
        .captures_iter(command)
        .filter_map(|cap| parse_quoted_string(&cap[1]))
        .filter_map(|header| {
            let parts: Vec<&str> = header.splitn(2, ':').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect()
}

fn parse_data(command: &str) -> Option<String> {
    let data_regex = Regex::new(r#"-d\s+('.*?'|".*?"|\S+)"#).unwrap();
    data_regex
        .captures(command)
        .and_then(|cap| parse_quoted_string(&cap[1]))
}

fn parse_cookies(command: &str) -> Vec<(String, String)> {
    let cookie_regex = Regex::new(r#"--cookie\s+('.*?'|".*?"|\S+)"#).unwrap();
    cookie_regex
        .captures_iter(command)
        .filter_map(|cap| parse_quoted_string(&cap[1]))
        .flat_map(|cookie_str| {
            cookie_str
                .split(';')
                .filter_map(|cookie| {
                    let parts: Vec<&str> = cookie.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(String, String)>>()
        })
        .collect()
}

fn parse_options(command: &str) -> CurlOptions {
    CurlOptions {
        insecure: Some(Regex::new(r"\s-k\b").unwrap().is_match(command)),
        follow_redirects: Some(Regex::new(r"\s-L\b").unwrap().is_match(command)),
        compressed: Some(Regex::new(r"\s--compressed\b").unwrap().is_match(command)),
        fail: Some(Regex::new(r"\s-f\b").unwrap().is_match(command)),
        max_redirects: parse_numeric_option(command, r"--max-redirs\s+(\d+)"),
        timeout: parse_numeric_option(command, r"--max-time\s+(\d+)"),
        connect_timeout: parse_numeric_option(command, r"--connect-timeout\s+(\d+)"),
        max_time: parse_numeric_option(command, r"--max-time\s+(\d+)"),
        proxy: parse_string_option(command, r"-x\s+'([^']+)'"),
        output_file: parse_string_option(command, r"-o\s+'([^']+)'"),
        cert: parse_string_option(command, r"--cert\s+'([^']+)'"),
        key: parse_string_option(command, r"--key\s+'([^']+)'"),
        key_password: parse_string_option(command, r"--pass\s+'([^']+)'"),
        retry: parse_numeric_option(command, r"--retry\s+(\d+)"),
        retry_delay: parse_numeric_option(command, r"--retry-delay\s+(\d+)"),
        interface: parse_string_option(command, r"--interface\s+'([^']+)'"),
        dns_servers: parse_string_list_option(command, r"--dns-servers\s+(\S+)"),
        ipv4_only: Some(Regex::new(r"\s-4\b").unwrap().is_match(command)),
        ipv6_only: Some(Regex::new(r"\s-6\b").unwrap().is_match(command)),
        rate_limit: parse_numeric_option(command, r"--limit-rate\s+(\d+)k"),
    }
}

fn parse_numeric_option(command: &str, pattern: &str) -> Option<u32> {
    Regex::new(pattern)
        .unwrap()
        .captures(command)
        .and_then(|cap| cap[1].parse().ok())
}

fn parse_string_option(command: &str, pattern: &str) -> Option<String> {
    Regex::new(pattern)
        .unwrap()
        .captures(command)
        .map(|cap| cap[1].to_string())
}

fn parse_string_list_option(command: &str, pattern: &str) -> Option<Vec<String>> {
    Regex::new(pattern)
        .unwrap()
        .captures(command)
        .map(|cap| cap[1].split(',').map(String::from).collect())
}

fn parse_store_curl_body(script: &str) -> Vec<StoreCurlBody> {
    let regex = Regex::new(r"#\s*storebody\s+(\S+)\s+(\S+)\s+(\S+)").unwrap();
    regex
        .captures_iter(script)
        .map(|cap| StoreCurlBody {
            source: cap[1].to_string(),
            destination: cap[2].to_string(),
            filename: cap[3].to_string(),
        })
        .collect()
}

fn parse_store_curl_cookie(script: &str) -> Vec<StoreCurlCookie> {
    let regex = Regex::new(r"#\s*storecookie\s+(\S+)\s+(\S+)\s+(\S+)").unwrap();
    regex
        .captures_iter(script)
        .map(|cap| StoreCurlCookie {
            source: cap[1].to_string(),
            destination: cap[2].to_string(),
            filename: cap[3].to_string(),
        })
        .collect()
}

fn parse_load_curl(script: &str) -> Vec<LoadCurl> {
    let regex = Regex::new(r"#\s*loaddata\s+(\S+)\s+(\S+)\s+(\S+)").unwrap();
    regex
        .captures_iter(script)
        .map(|cap| LoadCurl {
            filename: cap[1].to_string(),
            data_name: cap[2].to_string(),
            env_variable: cap[3].to_string(),
        })
        .collect()
}

fn parse_quoted_string(s: &str) -> Option<String> {
    let s = s.trim();
    if (s.starts_with('\'') && s.ends_with('\'')) || (s.starts_with('"') && s.ends_with('"')) {
        Some(s[1..s.len() - 1].to_string())
    } else {
        Some(s.to_string())
    }
}

impl FromStr for HttpMethod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            _ => Err(anyhow!("Invalid HTTP method: {}", s)),
        }
    }
}
