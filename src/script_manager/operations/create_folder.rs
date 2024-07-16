use std::fs;
use std::sync::Arc;
use crate::script_manager::errors::{ScriptManagerError, ErrorKind};
use crate::script_manager::ScriptManager;

impl ScriptManager {
    pub fn create_folder(self: &Arc<Self>, folder_path: &str) -> Result<(), ScriptManagerError> {
        let _lock = self.lock();
        
        let full_path = self.get_full_path(folder_path)
            .map_err(|e| ScriptManagerError::with_source(
                ErrorKind::Io,
                "create_folder",
                "Failed to get full path",
                Box::new(e),
            ))?;
        
        if full_path.exists() {
            if full_path.is_dir() {
                return Ok(());  // Folder already exists, consider this a success
            } else {
                return Err(ScriptManagerError::new(
                    ErrorKind::Io,
                    "create_folder",
                    "A file with this name already exists",
                ));
            }
        }
        
        fs::create_dir_all(&full_path).map_err(|e| ScriptManagerError::with_source(
            ErrorKind::Io,
            "create_folder",
            "Failed to create directory",
            Box::new(e),
        ))
    }
}
