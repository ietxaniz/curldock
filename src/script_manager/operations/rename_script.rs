use crate::debug_err;
use anyhow::{Result, anyhow};
use std::fs;
use std::sync::Arc;
use crate::script_manager::models::ScriptDetails;
use crate::script_manager::ScriptManager;

impl ScriptManager {
    pub fn rename_script(
        self: &Arc<Self>, 
        old_full_name: &str, 
        new_full_name: &str
    ) -> Result<ScriptDetails> {
        let _lock = self.lock();
        
        let old_full_path = self.get_full_path(old_full_name)?;
        let new_full_path = self.get_full_path(new_full_name)?;
        
        if !old_full_path.exists() {
            return Err(anyhow!("Script not found: {}", old_full_path.display()));
        }
        
        if new_full_path.exists() {
            return debug_err!(Err(anyhow!("A script with the name '{}' already exists", new_full_name)));
        }
        
        // Create the directory structure for the new path if it doesn't exist
        if let Some(parent) = new_full_path.parent() {
            debug_err!(fs::create_dir_all(parent), "Failed to create directory structure for the new path")?;
        }
        
        debug_err!(fs::rename(&old_full_path, &new_full_path), "Failed to rename script")?;
        
        // After renaming, get the updated details
        debug_err!(self.get_script_details(new_full_name))
    }
}
