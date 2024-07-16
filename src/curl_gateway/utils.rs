use crate::curl_gateway::errors::{CurlGatewayError, ErrorKind};
use serde_json::Value;

pub fn extract_data(body: &str, source: &str) -> Result<String, CurlGatewayError> {
    let json: Value = if body.is_empty() {
        serde_json::json!({})
    } else {
        serde_json::from_str(body).map_err(|e| {
            CurlGatewayError::with_source(
                ErrorKind::Parsing,
                "extract_data",
                "failed to parse json",
                Box::new(e),
            )
        })?
    };

    let parts: Vec<&str> = source.split('.').collect();
    let mut current = &json;

    for part in parts {
        current = current.get(part).ok_or_else(|| {
            CurlGatewayError::new(
                ErrorKind::Parsing,
                "extract_data",
                format!("Data not found at path: {}", source),
            )
        })?;
    }

    current
        .as_str()
        .map(|s| s.to_string())
        .or_else(|| current.as_u64().map(|n| n.to_string()))
        .or_else(|| current.as_f64().map(|n| n.to_string()))
        .ok_or_else(|| {
            CurlGatewayError::new(
                ErrorKind::Parsing,
                "extract_data",
                format!("Invalid source path: {}", source),
            )
        })
}
