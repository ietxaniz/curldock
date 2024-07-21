use crate::curl_gateway::models::StoreData;
use crate::debug_err;
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};

pub fn store_data(store_data: Vec<StoreData>) -> Result<()> {
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
            debug_err!(
                file.read_to_string(&mut file_content),
                "Failed to read file: {}",
                filename
            )?;
        }

        let mut json_data: Value = if file_content.is_empty() {
            json!({})
        } else {
            debug_err!(
                serde_json::from_str(&file_content),
                "Failed to parse JSON from file: {}",
                filename
            )?
        };

        // Update the JSON object with the new data
        for entry in entries {
            json_data[entry.parameter] = json!(entry.data);
        }

        // Write the updated JSON back to the file
        let json_string = debug_err!(
            serde_json::to_string_pretty(&json_data),
            "Failed to serialize JSON data"
        )?;

        let mut file = debug_err!(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&filename),
            "Failed to open file for writing: {}",
            filename
        )?;

        debug_err!(
            file.write_all(json_string.as_bytes()),
            "Failed to write data to file: {}",
            filename
        )?;
    }

    Ok(())
}
