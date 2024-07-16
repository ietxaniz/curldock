use std::fs;
use std::sync::Arc;
use crate::script_manager::errors::{ScriptManagerError, ErrorKind};
use crate::script_manager::models::ScriptDetails;
use crate::script_manager::ScriptManager;

impl ScriptManager {
    pub fn rename_script(
        self: &Arc<Self>, 
        old_path: &str, 
        old_name: &str, 
        new_path: &str, 
        new_name: &str
    ) -> Result<ScriptDetails, ScriptManagerError> {
        let _lock = self.lock();
        
        let old_full_path = self.get_full_path(old_path)?.join(old_name);
        let new_full_path = self.get_full_path(new_path)?.join(new_name);
        
        if !old_full_path.exists() {
            return Err(ScriptManagerError::new(
                ErrorKind::ScriptNotFound,
                "rename_script",
                format!("Script not found: {}", old_full_path.display()),
            ));
        }
        
        if new_full_path.exists() {
            return Err(ScriptManagerError::new(
                ErrorKind::Io,
                "rename_script",
                format!("A script with the name '{}' already exists in the destination path", new_name),
            ));
        }
        
        // Create the directory structure for the new path if it doesn't exist
        if let Some(parent) = new_full_path.parent() {
            fs::create_dir_all(parent).map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "rename_script",
                "Failed to create directory structure for the new path",
                Box::new(e),
            ))?;
        }
        
        fs::rename(&old_full_path, &new_full_path).map_err(|e| ScriptManagerError::with_source(
            ErrorKind::Io,
            "rename_script",
            "Failed to rename script",
            Box::new(e),
        ))?;
        
        // After renaming, get the updated details
        self.get_script_details(new_path, new_name)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "rename_script",
                "Failed to get script details after renaming",
                Box::new(e),
            ))
    }
}
