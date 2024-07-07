use std::fs;
use std::time::UNIX_EPOCH;
use std::sync::Arc;
use crate::script_manager::models::{ScriptInfo, ScriptList, ScriptError};
use crate::script_manager::ScriptManager;

impl ScriptManager {
    pub fn list_scripts(self: &Arc<Self>, path: Option<&str>) -> Result<ScriptList, ScriptError> {
        let base_path = match path {
            Some(p) => self.get_full_path(p)?,
            None => self.scripts_dir().to_path_buf(),
        };

        let mut scripts = Vec::new();
        for entry in fs::read_dir(&base_path)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::metadata(&path)?;
            let is_folder = metadata.is_dir();
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
            
            let created_at = metadata.created()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
                .unwrap_or(0);
            let updated_at = metadata.modified()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
                .unwrap_or(0);

            scripts.push(ScriptInfo {
                name: file_name,
                created_at,
                updated_at,
                is_folder,
                path: path.strip_prefix(self.scripts_dir()).unwrap().to_string_lossy().into_owned(),
            });
        }

        Ok(ScriptList {
            path: path.unwrap_or("").to_string(),
            scripts,
        })
    }
}