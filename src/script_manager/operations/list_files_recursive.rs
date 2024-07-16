use crate::script_manager::errors::{ScriptManagerError, ErrorKind};
use crate::script_manager::models::{FileInfo, FileList, FileType};
use crate::script_manager::ScriptManager;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::UNIX_EPOCH;
use crate::curl_gateway::operations::parse_curl_command;
use serde_json::from_str as json_from_str;

impl ScriptManager {
    pub fn list_files_recursive(self: &Arc<Self>) -> Result<FileList, ScriptManagerError> {
        let base_path = self.scripts_dir();
        let files = self.collect_files_recursive(&base_path, &base_path)?;

        Ok(FileList {
            path: "".to_string(),
            files,
        })
    }

    fn collect_files_recursive(
        &self,
        dir: &Path,
        base_path: &Path,
    ) -> Result<Vec<FileInfo>, ScriptManagerError> {
        let mut files = Vec::new();
        let mut entries: Vec<_> = fs::read_dir(dir)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "collect_files_recursive",
                "Failed to read directory",
                Box::new(e),
            ))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "collect_files_recursive",
                "Failed to collect directory entries",
                Box::new(e),
            ))?;

        // Sort entries: directories first, then files
        entries.sort_by(|a, b| {
            let a_metadata = a.metadata().unwrap();
            let b_metadata = b.metadata().unwrap();
            b_metadata.is_dir().cmp(&a_metadata.is_dir())
                .then(a.file_name().cmp(&b.file_name()))
        });

        for entry in entries {
            let path = entry.path();
            let metadata = fs::metadata(&path).map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "collect_files_recursive",
                "Failed to read metadata",
                Box::new(e),
            ))?;
            let is_folder = metadata.is_dir();
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();

            let relative_path = path.strip_prefix(base_path)
                .map_err(|e| ScriptManagerError::with_source(
                    ErrorKind::Internal,
                    "collect_files_recursive",
                    "Failed to strip base path",
                    Box::new(e),
                ))?
                .to_string_lossy()
                .into_owned();

            let file_type = if is_folder {
                FileType::Folder
            } else {
                if let Ok(content) = fs::read_to_string(&path) {
                    if parse_curl_command(&content).is_ok() {
                        FileType::Script
                    } else if json_from_str::<serde_json::Value>(&content).is_ok() {
                        FileType::Data
                    } else {
                        FileType::Unknown
                    }
                } else {
                    FileType::Unknown
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

            let file_info = FileInfo {
                name: file_name,
                created_at,
                updated_at,
                is_folder,
                path: relative_path,
                file_type,
            };

            files.push(file_info);

            if is_folder {
                let mut sub_files = self.collect_files_recursive(&path, base_path)?;
                files.append(&mut sub_files);
            }
        }

        Ok(files)
    }
}