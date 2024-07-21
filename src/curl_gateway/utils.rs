use crate::debug_err;
use anyhow::{Result, anyhow};
use serde_json::Value;

pub fn extract_data(body: &str, source: &str) -> Result<String> {
    let json: Value = if body.is_empty() {
        serde_json::json!({})
    } else {
        debug_err!(serde_json::from_str(body), "failed to parse json")?
    };

    let parts: Vec<&str> = source.split('.').collect();
    let mut current = &json;

    for part in parts {
        current = current.get(part).ok_or_else(|| anyhow!("Data not found at path: {}", source))?;
    }

    current
        .as_str()
        .map(|s| s.to_string())
        .or_else(|| current.as_u64().map(|n| n.to_string()))
        .or_else(|| current.as_f64().map(|n| n.to_string()))
        .ok_or_else(|| anyhow!("Invalid source path: {}", source))
}
