use std::fs;
use chrono::DateTime;
use std::sync::Arc;
use crate::script_manager::models::ScriptInfo;
use crate::script_manager::ScriptManager;

impl ScriptManager {
    pub fn list_scripts(self: &Arc<Self>) -> Result<Vec<ScriptInfo>, std::io::Error> {
        let mut scripts = Vec::new();
        for entry in fs::read_dir(self.scripts_dir())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("sh") {
                let metadata = fs::metadata(&path)?;
                let created_at = DateTime::from(metadata.created()?);
                let updated_at = DateTime::from(metadata.modified()?);
                let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
                scripts.push(ScriptInfo {
                    file_name,
                    created_at,
                    updated_at,
                });
            }
        }
        Ok(scripts)
    }
}