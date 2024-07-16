use crate::script_manager::errors::{ScriptManagerError, ErrorKind};
use crate::script_manager::models::{FileInfo, FileList, FileType};
use crate::script_manager::ScriptManager;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::UNIX_EPOCH;
use crate::curl_gateway::operations::parse_curl_command;

impl ScriptManager {
    pub fn list_files_recursive(self: &Arc<Self>) -> Result<FileList, ScriptManagerError> {
        let base_path = self.scripts_dir();
        let files = self.collect_files_recursive(base_path, "")?;

        Ok(FileList {
            path: "".to_string(),
            files,
        })
    }

    fn collect_files_recursive(
        &self,
        dir: &Path,
        relative_path: &str,
    ) -> Result<Vec<FileInfo>, ScriptManagerError> {
        let mut files = Vec::new();

        for entry in fs::read_dir(dir).map_err(|e| ScriptManagerError::with_source(
            ErrorKind::Io,
            "collect_files_recursive",
            "Failed to read directory",
            Box::new(e),
        ))? {
            let entry = entry.map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "collect_files_recursive",
                "Failed to read directory entry",
                Box::new(e),
            ))?;
            let path = entry.path();
            let metadata = fs::metadata(&path).map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "collect_files_recursive",
                "Failed to read metadata",
                Box::new(e),
            ))?;
            let is_folder = metadata.is_dir();
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();

            let new_relative_path = if relative_path.is_empty() {
                file_name.clone()
            } else {
                format!("{}/{}", relative_path, file_name)
            };

            let file_type = if is_folder {
                FileType::Data // Folders are considered as data
            } else {
                // Try to parse as a curl command to determine if it's a script
                let content = fs::read_to_string(&path).map_err(|e| ScriptManagerError::with_source(
                    ErrorKind::Io,
                    "collect_files_recursive",
                    "Failed to read file",
                    Box::new(e),
                ))?;
                match parse_curl_command(&content) {
                    Ok(_) => FileType::Script,
                    Err(_) => FileType::Data,
                }
            };

            let created_at = metadata
                .created()
                .map(|time| time.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64)
                .unwrap_or(0);
            let updated_at = metadata
                .modified()
                .map(|time| time.duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64)
                .unwrap_or(0);

            files.push(FileInfo {
                name: file_name,
                created_at,
                updated_at,
                is_folder,
                path: new_relative_path.clone(),
                file_type,
            });

            if is_folder {
                let mut sub_files = self.collect_files_recursive(&path, &new_relative_path)?;
                files.append(&mut sub_files);
            }
        }

        Ok(files)
    }
}
