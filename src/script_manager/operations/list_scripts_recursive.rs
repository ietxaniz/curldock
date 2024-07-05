use std::fs;
use std::path::Path;
use chrono::DateTime;
use std::sync::Arc;
use crate::script_manager::models::{ScriptInfo, ScriptList};
use crate::script_manager::ScriptManager;

impl ScriptManager {
    pub fn list_scripts_recursive(self: &Arc<Self>) -> Result<ScriptList, String> {
        let base_path = self.scripts_dir();
        println!("collecting {}", base_path.to_string_lossy());
        let scripts = self.collect_scripts_recursive(&base_path, "").map_err(|e| {
            format!("Failed to collect scripts: {}", e)
        })?;

        Ok(ScriptList {
            path: "".to_string(),
            scripts,
        })
    }

    fn collect_scripts_recursive(&self, dir: &Path, relative_path: &str) -> Result<Vec<ScriptInfo>, String> {
        let mut scripts = Vec::new();

        println!("reading {} {}", dir.to_string_lossy(), relative_path);

        let entries = fs::read_dir(dir).map_err(|e| {
            format!("Failed to read directory '{}': {}", dir.display(), e)
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                format!("Failed to read directory entry in '{}': {}", dir.display(), e)
            })?;
            
            let path = entry.path();
            let metadata = fs::metadata(&path).map_err(|e| {
                format!("Failed to read metadata for '{}': {}", path.display(), e)
            })?;
            
            let is_folder = metadata.is_dir();
            let file_name = path.file_name()
                .ok_or_else(|| format!("Invalid file name for '{}'", path.display()))?
                .to_string_lossy()
                .into_owned();
            
            let new_relative_path = if relative_path.is_empty() {
                file_name.clone()
            } else {
                format!("{}/{}", relative_path, file_name)
            };

            let created_at = DateTime::from(metadata.created().map_err(|e| {
                format!("Failed to get creation time for '{}': {}", path.display(), e)
            })?);
            
            let updated_at = DateTime::from(metadata.modified().map_err(|e| {
                format!("Failed to get modification time for '{}': {}", path.display(), e)
            })?);

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