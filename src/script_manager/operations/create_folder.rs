use crate::debug_err;
use anyhow::{Result, anyhow};
use std::fs;
use std::sync::Arc;
use crate::script_manager::ScriptManager;


impl ScriptManager {
    pub fn create_folder(self: &Arc<Self>, folder_path: &str) -> Result<()> {
        let _lock = self.lock();
        
        let full_path = debug_err!(self.get_full_path(folder_path), "Failed to get full path")?;
        
        if full_path.exists() {
            if full_path.is_dir() {
                return Ok(());  // Folder already exists, consider this a success
            } else {
                return Err(anyhow!("A file with this name already exists"));
            }
        }
        
        debug_err!(fs::create_dir_all(&full_path), "Failed to create directory")
    }
}
