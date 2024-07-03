use std::fs;
use std::path::PathBuf;
use chrono::DateTime;
use crate::script_manager::models::ScriptDetails;
use crate::script_manager::ScriptManager;
use std::io::Read;

impl ScriptManager {
    pub fn get_script_details(&self, path: &str, name: &str) -> Result<ScriptDetails, std::io::Error> {
        let full_path: PathBuf = if path.is_empty() {
            self.scripts_dir().join(name)
        } else {
            self.scripts_dir().join(path).join(name)
        };
        
        if !full_path.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Script not found"));
        }

        let metadata = fs::metadata(&full_path)?;
        let mut file = fs::File::open(&full_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(ScriptDetails {
            name: name.to_string(),
            path: path.to_string(),
            content,
            created_at: DateTime::from(metadata.created()?),
            updated_at: DateTime::from(metadata.modified()?),
        })
    }
}