use std::fs;
use std::io::Write;
use std::sync::Arc;
use crate::script_manager::models::{ScriptDetails, ScriptError, ScriptDetailsCreate};
use crate::script_manager::ScriptManager;
use crate::curl_gateway::operations::generate_curl_command;

impl ScriptManager {
    pub fn update_script(self: &Arc<Self>, script_details: ScriptDetailsCreate) -> Result<ScriptDetails, ScriptError> {
        let _lock = self.lock();
        
        let base_path = self.scripts_dir();
        println!("base path: {:?}", base_path);
        println!("script_details path: {:?}", &script_details.path);
        let full_path = base_path.join(&script_details.path).join(&script_details.name);
        
        if !full_path.exists() {
            return Err(ScriptError::IoError(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Script does not exist"
            )));
        }
        
        let curl_command_str = generate_curl_command(&script_details.curl_command);
        
        let mut file = fs::File::create(&full_path)?;
        file.write_all(curl_command_str.as_bytes())?;
        
        let metadata = fs::metadata(&full_path)?;
        
        Ok(ScriptDetails {
            name: script_details.name,
            path: script_details.path,
            curl_command: script_details.curl_command,
            created_at: chrono::DateTime::from(metadata.created()?),
            updated_at: chrono::DateTime::from(metadata.modified()?),
        })
    }
}
