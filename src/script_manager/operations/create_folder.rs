use std::fs;
use std::sync::Arc;
use crate::script_manager::models::ScriptError;
use crate::script_manager::ScriptManager;

impl ScriptManager {
    pub fn create_folder(self: &Arc<Self>, folder_path: &str) -> Result<(), ScriptError> {
        let _lock = self.lock();
        
        let full_path = self.get_full_path(folder_path)?;
        
        if full_path.exists() {
            if full_path.is_dir() {
                return Ok(());  // Folder already exists, consider this a success
            } else {
                return Err(ScriptError::IoError(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "A file with this name already exists"
                )));
            }
        }
        
        fs::create_dir_all(&full_path).map_err(ScriptError::IoError)
    }
}
