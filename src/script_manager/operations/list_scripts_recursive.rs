use crate::script_manager::models::{ScriptError, ScriptInfo, ScriptList};
use crate::script_manager::ScriptManager;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::sync::Arc;

impl ScriptManager {
    pub fn list_scripts_recursive(self: &Arc<Self>) -> Result<ScriptList, ScriptError> {
        let base_path = self.scripts_dir();
        let scripts = self.collect_scripts_recursive(base_path, "")?;

        Ok(ScriptList {
            path: "".to_string(),
            scripts,
        })
    }

    fn collect_scripts_recursive(
        &self,
        _dir: &Path,
        relative_path: &str,
    ) -> Result<Vec<ScriptInfo>, ScriptError> {
        let mut scripts = Vec::new();

        let full_path = self.get_full_path(relative_path)?;

        let entries = fs::read_dir(&full_path).map_err(|e| {
            ScriptError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read directory '{}': {}", full_path.display(), e),
            ))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                ScriptError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Failed to read directory entry in '{}': {}",
                        full_path.display(),
                        e
                    ),
                ))
            })?;

            let path = entry.path();
            let metadata = fs::metadata(&path).map_err(|e| {
                ScriptError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to read metadata for '{}': {}", path.display(), e),
                ))
            })?;

            let is_folder = metadata.is_dir();
            let file_name = path
                .file_name()
                .ok_or_else(|| {
                    ScriptError::InvalidPath(format!("Invalid file name for '{}'", path.display()))
                })?
                .to_string_lossy()
                .into_owned();

            let new_relative_path = if relative_path.is_empty() {
                file_name.clone()
            } else {
                format!("{}/{}", relative_path, file_name)
            };

            let created_at = metadata
                .created()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
                .unwrap_or(0);
            let updated_at = metadata
                .modified()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
                .unwrap_or(0);

            scripts.push(ScriptInfo {
                name: file_name,
                created_at,
                updated_at,
                is_folder,
                path: relative_path.to_string(),
            });

            if is_folder {
                let mut sub_scripts = self.collect_scripts_recursive(&path, &new_relative_path)?;
                scripts.append(&mut sub_scripts);
            }
        }

        Ok(scripts)
    }
}
