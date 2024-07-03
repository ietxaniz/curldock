use crate::curl_gateway::models::{CurlCommand, HttpMethod};

pub fn generate_curl_command(command: &CurlCommand) -> String {
    let mut parts = vec!["curl".to_string()];

    if command.options.verbose {
        parts.push("-v".to_string());
    }

    if command.options.insecure {
        parts.push("-k".to_string());
    }

    match command.method {
        HttpMethod::GET => {}
        _ => parts.push(format!("-X {}", http_method_to_string(&command.method))),
    }

    for (key, value) in &command.headers {
        parts.push(format!("-H '{}: {}'", key, value));
    }

    if let Some(data) = &command.data {
        parts.push(format!("-d '{}'", data));
    }

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
