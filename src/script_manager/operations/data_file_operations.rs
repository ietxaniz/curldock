use crate::curl_gateway::models::DataFileDetails;
use crate::script_manager::errors::{ScriptManagerError, ErrorKind};
use crate::script_manager::ScriptManager;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

impl ScriptManager {
    pub fn store_data_file(
        self: &Arc<Self>,
        path: &str,
        name: &str,
        content: &HashMap<String, String>,
    ) -> Result<DataFileDetails, ScriptManagerError> {
        let _lock = self.lock();

        let full_path = self.get_full_path(path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "store_data_file",
                "Failed to get full path",
                Box::new(e),
            ))?
            .join(name);

        if full_path.exists() {
            return Err(ScriptManagerError::new(
                ErrorKind::Io,
                "store_data_file",
                "Data file already exists",
            ));
        }

        let json_content = serde_json::to_string_pretty(content)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::InvalidInput,
                "store_data_file",
                "Failed to serialize content to JSON",
                Box::new(e),
            ))?;

        let mut file = fs::File::create(&full_path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "store_data_file",
                "Failed to create data file",
                Box::new(e),
            ))?;

        file.write_all(json_content.as_bytes())
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "store_data_file",
                "Failed to write to data file",
                Box::new(e),
            ))?;

        let metadata = fs::metadata(&full_path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "store_data_file",
                "Failed to read metadata",
                Box::new(e),
            ))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Ok(DataFileDetails {
            name: name.to_string(),
            path: path.to_string(),
            content: content.clone(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn load_data_file(self: &Arc<Self>, path: &str) -> Result<DataFileDetails, ScriptManagerError> {
        let full_path = self.get_full_path(path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "load_data_file",
                "Failed to get full path",
                Box::new(e),
            ))?;

        if !full_path.exists() {
            return Err(ScriptManagerError::new(
                ErrorKind::Io,
                "load_data_file",
                "Data file not found",
            ));
        }

        let content_str = fs::read_to_string(&full_path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "load_data_file",
                "Failed to read data file",
                Box::new(e),
            ))?;

        let content: HashMap<String, String> = serde_json::from_str(&content_str)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::InvalidInput,
                "load_data_file",
                "Failed to deserialize JSON content",
                Box::new(e),
            ))?;

        let metadata = fs::metadata(&full_path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "load_data_file",
                "Failed to read metadata",
                Box::new(e),
            ))?;

        let created_at = metadata
            .created()
            .map(|time| time.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64)
            .unwrap_or(0);

        let updated_at = metadata
            .modified()
            .map(|time| time.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64)
            .unwrap_or(0);

        Ok(DataFileDetails {
            name: full_path.file_name().unwrap().to_string_lossy().into_owned(),
            path: path.to_string(),
            content,
            created_at,
            updated_at,
        })
    }

    pub fn update_data_file(
        self: &Arc<Self>,
        path: &str,
        content: &HashMap<String, String>,
    ) -> Result<DataFileDetails, ScriptManagerError> {
        let _lock = self.lock();

        let full_path = self.get_full_path(path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "update_data_file",
                "Failed to get full path",
                Box::new(e),
            ))?;

        if !full_path.exists() {
            return Err(ScriptManagerError::new(
                ErrorKind::Io,
                "update_data_file",
                "Data file not found",
            ));
        }

        let json_content = serde_json::to_string_pretty(content)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::InvalidInput,
                "update_data_file",
                "Failed to serialize content to JSON",
                Box::new(e),
            ))?;

        fs::write(&full_path, json_content)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "update_data_file",
                "Failed to write to data file",
                Box::new(e),
            ))?;

        let metadata = fs::metadata(&full_path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "update_data_file",
                "Failed to read metadata",
                Box::new(e),
            ))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let created_at = metadata
            .created()
            .map(|time| time.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64)
            .unwrap_or(0);

        Ok(DataFileDetails {
            name: full_path.file_name().unwrap().to_string_lossy().into_owned(),
            path: path.to_string(),
            content: content.clone(),
            created_at,
            updated_at: now,
        })
    }

    pub fn delete_data_file(self: &Arc<Self>, path: &str) -> Result<(), ScriptManagerError> {
        let _lock = self.lock();

        let full_path = self.get_full_path(path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "delete_data_file",
                "Failed to get full path",
                Box::new(e),
            ))?;

        if !full_path.exists() {
            return Err(ScriptManagerError::new(
                ErrorKind::Io,
                "delete_data_file",
                "Data file not found",
            ));
        }

        fs::remove_file(full_path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "delete_data_file",
                "Failed to delete data file",
                Box::new(e),
            ))
    }

    pub fn rename_file(
        self: &Arc<Self>,
        old_path: &str,
        new_path: &str,
    ) -> Result<(), ScriptManagerError> {
        let _lock = self.lock();

        let old_full_path = self.get_full_path(old_path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "rename_file",
                "Failed to get full path for old file",
                Box::new(e),
            ))?;

        let new_full_path = self.get_full_path(new_path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "rename_file",
                "Failed to get full path for new file",
                Box::new(e),
            ))?;

        if !old_full_path.exists() {
            return Err(ScriptManagerError::new(
                ErrorKind::Io,
                "rename_file",
                "Old data file not found",
            ));
        }

        if new_full_path.exists() {
            return Err(ScriptManagerError::new(
                ErrorKind::Io,
                "rename_file",
                "New data file path already exists",
            ));
        }

        fs::rename(old_full_path, new_full_path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "rename_file",
                "Failed to rename file",
                Box::new(e),
            ))
    }
}
