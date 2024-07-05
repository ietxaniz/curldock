use std::fs;
use chrono::DateTime;
use std::sync::Arc;
use crate::script_manager::models::{ScriptInfo, ScriptList};
use crate::script_manager::ScriptManager;

impl ScriptManager {
    pub fn list_scripts(self: &Arc<Self>, path: Option<&str>) -> Result<ScriptList, std::io::Error> {
        let base_path = self.scripts_dir();
        let local_path = match path {
          Some(p) => p,
          None => "",
        };
        let full_path = match path {
            Some(p) => base_path.join(p),
            None => base_path.to_path_buf(),
        };

        let mut scripts = Vec::new();
        for entry in fs::read_dir(&full_path)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::metadata(&path)?;
            let is_folder = metadata.is_dir();
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
            
            scripts.push(ScriptInfo {
                name: file_name,
                created_at: DateTime::from(metadata.created()?),
                updated_at: DateTime::from(metadata.modified()?),
                is_folder,
                path: local_path.to_string(),
            });
        }

        Ok(ScriptList {
            path: path.unwrap_or("").to_string(),
            scripts,
        })
    }
}