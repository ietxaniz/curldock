use crate::curl_gateway::models::DataFileDetails;
use crate::debug_err;
use crate::script_manager::ScriptManager;
use anyhow::{anyhow, Result};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

impl ScriptManager {
    pub fn store_data_file(
        self: &Arc<Self>,
        full_path: &str,
        content: &HashMap<String, String>,
    ) -> Result<DataFileDetails> {
        let _lock = self.lock();

        let inner_full_path = debug_err!(self.get_full_path(full_path), "Failed to get full path")?;

        if inner_full_path.exists() {
            return Err(anyhow!("Data file '{}' already exists", full_path));
        }

        let json_content = debug_err!(
            serde_json::to_string_pretty(content),
            "Failed to serialize content to JSON"
        )?;

        let mut file = debug_err!(
            fs::File::create(&inner_full_path),
            "Failed to create data file"
        )?;

        debug_err!(
            file.write_all(json_content.as_bytes()),
            "Failed to write to data file"
        )?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Ok(DataFileDetails {
            full_name: inner_full_path.to_string_lossy().to_string(),
            content: content.clone(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn load_data_file(self: &Arc<Self>, path: &str) -> Result<DataFileDetails> {
        let full_path = debug_err!(self.get_full_path(path), "Failed to get full path")?;

        if !full_path.exists() {
            return Err(anyhow!("Data file not found"));
        }

        let content_str = debug_err!(fs::read_to_string(&full_path), "Failed to read data file")?;

        let content: HashMap<String, String> = debug_err!(
            serde_json::from_str(&content_str),
            "Failed to deserialize JSON content"
        )?;

        let metadata = debug_err!(fs::metadata(&full_path), "Failed to read metadata")?;

        let created_at = metadata
            .created()
            .map(|time| {
                time.duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64
            })
            .unwrap_or(0);

        let updated_at = metadata
            .modified()
            .map(|time| {
                time.duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64
            })
            .unwrap_or(0);

        Ok(DataFileDetails {
            full_name: full_path.to_string_lossy().to_string(),
            content,
            created_at,
            updated_at,
        })
    }

    pub fn update_data_file(
        self: &Arc<Self>,
        path: &str,
        content: &HashMap<String, String>,
    ) -> Result<DataFileDetails> {
        let _lock = self.lock();

        let full_path = debug_err!(self.get_full_path(path), "Failed to get full path")?;

        if !full_path.exists() {
            return Err(anyhow!("Data file not found"));
        }

        let json_content = debug_err!(
            serde_json::to_string_pretty(content),
            "Failed to serialize content to JSON"
        )?;

        debug_err!(
            fs::write(&full_path, json_content),
            "Failed to write to data file"
        )?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let metadata = debug_err!(fs::metadata(&full_path), "Failed to read metadata")?;

        let created_at = metadata
            .created()
            .map(|time| {
                time.duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64
            })
            .unwrap_or(0);

        Ok(DataFileDetails {
            full_name: full_path.to_string_lossy().to_string(),
            content: content.clone(),
            created_at,
            updated_at: now,
        })
    }

    pub fn delete_asset(self: &Arc<Self>, path: &str) -> Result<()> {
        let _lock = self.lock();

        let full_path = debug_err!(self.get_full_path(path), "Failed to get full path")?;

        if !full_path.exists() {
            return Err(anyhow!("Asset not found"));
        }

        if full_path.is_dir() {
            debug_err!(fs::remove_dir_all(&full_path), "Failed to delete directory")
        } else {
            debug_err!(fs::remove_file(&full_path), "Failed to delete file")
        }
    }

    pub fn rename_file(self: &Arc<Self>, old_path: &str, new_path: &str) -> Result<()> {
        let _lock = self.lock();

        let old_full_path = debug_err!(self.get_full_path(old_path))?;

        let new_full_path = debug_err!(self.get_full_path(new_path))?;

        if !old_full_path.exists() {
            return Err(anyhow!("Old data file not found"));
        }

        if new_full_path.exists() {
            return Err(anyhow!("New data file path already exists"));
        }

        debug_err!(
            fs::rename(old_full_path, new_full_path),
            "Failed to rename file"
        )
    }
}
