use crate::curl_gateway::models::{CurlCommand, HttpMethod, CurlOptions,ParseError};
use regex::Regex;


pub fn parse_curl_command(script: &str) -> Result<CurlCommand, ParseError> {
    // Find the first line starting with 'curl'
    let curl_line = script.lines()
        .find(|line| line.trim().starts_with("curl"))
        .ok_or(ParseError::MissingCurlCommand)?;

    let method_regex = Regex::new(r"-X\s+(\w+)").unwrap();
    let url_regex = Regex::new(r"curl.*?\s(https?://\S+)").unwrap();
    let header_regex = Regex::new(r"-H\s+'([^']+)'").unwrap();
    let data_regex = Regex::new(r"-d\s+'([^']+)'").unwrap();
    let verbose_regex = Regex::new(r"\s-v\b").unwrap();
    let insecure_regex = Regex::new(r"\s-k\b").unwrap();

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

    let options = CurlOptions {
        verbose: verbose_regex.is_match(curl_line),
        insecure: insecure_regex.is_match(curl_line),
    };

    Ok(CurlCommand {
        method,
        url,
        headers,
        data,
        options,
    })
}