use crate::curl_gateway::errors::{CurlGatewayError, ErrorKind};
use crate::curl_gateway::models::StoreData;
use std::collections::HashMap;
use std::io::{Read, Write};
use serde_json::{json, Value};
use std::fs::OpenOptions;

pub fn store_data(store_data: Vec<StoreData>) -> Result<(), CurlGatewayError> {
    // Group entries by filename
    let mut grouped_data: HashMap<String, Vec<StoreData>> = HashMap::new();
    for entry in store_data {
        grouped_data
            .entry(entry.filename.clone())
            .or_insert_with(Vec::new)
            .push(entry);
    }

    // Process each file
    for (filename, entries) in grouped_data {
        // Read the existing file content or create a new JSON object
        let mut file_content = String::new();
        if let Ok(mut file) = OpenOptions::new().read(true).open(&filename) {
            file.read_to_string(&mut file_content).map_err(|e| {
                CurlGatewayError::with_source(
                    ErrorKind::DataHandling,
                    "store_data",
                    format!("Failed to read file: {}", filename),
                    Box::new(e)
                )
            })?;
        }

        let mut json_data: Value = if file_content.is_empty() {
            json!({})
        } else {
            serde_json::from_str(&file_content).map_err(|e| {
                CurlGatewayError::with_source(
                    ErrorKind::DataHandling,
                    "store_data",
                    format!("Failed to parse JSON from file: {}", filename),
                    Box::new(e)
                )
            })?
        };

        // Update the JSON object with the new data
        for entry in entries {
            json_data[entry.parameter] = json!(entry.data);
        }

        // Write the updated JSON back to the file
        let json_string = serde_json::to_string_pretty(&json_data).map_err(|e| {
            CurlGatewayError::with_source(
                ErrorKind::DataHandling,
                "store_data",
                "Failed to serialize JSON data",
                Box::new(e)
            )
        })?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&filename)
            .map_err(|e| {
                CurlGatewayError::with_source(
                    ErrorKind::DataHandling,
                    "store_data",
                    format!("Failed to open file for writing: {}", filename),
                    Box::new(e)
                )
            })?;

        file.write_all(json_string.as_bytes()).map_err(|e| {
            CurlGatewayError::with_source(
                ErrorKind::DataHandling,
                "store_data",
                format!("Failed to write data to file: {}", filename),
                Box::new(e)
            )
        })?;
    }

    Ok(())
}