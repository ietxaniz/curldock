use anyhow::{Result, anyhow};
use std::path::PathBuf;
use crate::script_manager::ScriptManager;

impl ScriptManager {
    pub(crate) fn get_full_path(&self, path: &str) -> Result<PathBuf> {
        let full_path = self.scripts_dir().join(path);
        
        // Ensure the resulting path is still within the scripts directory
        if !full_path.starts_with(self.scripts_dir()) {
            return Err(anyhow!("Path is outside of the scripts directory"));
        }
        
        Ok(full_path)
    }
}
