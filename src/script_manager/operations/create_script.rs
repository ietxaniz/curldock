use crate::curl_gateway::operations::generate_curl_command;
use crate::script_manager::models::{ScriptDetails, ScriptDetailsCreate, ScriptError};
use crate::script_manager::ScriptManager;
use std::fs;
use std::io::Write;
use std::sync::Arc;
use std::time::UNIX_EPOCH;

impl ScriptManager {
    pub fn create_script(
        self: &Arc<Self>,
        script_details: ScriptDetailsCreate,
    ) -> Result<ScriptDetails, ScriptError> {
        let _lock = self.lock();

        let base_path = self.scripts_dir();
        println!("base path: {:?}", base_path);
        println!("script_details path: {:?}", &script_details.path);
        let full_path = base_path
            .join(&script_details.path)
            .join(&script_details.name);

        println!("Creating script at: {:?}", full_path);

        if full_path.exists() {
            return Err(ScriptError::IoError(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Script already exists",
            )));
        }

        let curl_command_str = generate_curl_command(&script_details.curl_command);

        println!("Generated curl command: {}", curl_command_str);

        let mut file = fs::File::create(&full_path)?;
        file.write_all(curl_command_str.as_bytes())?;

        let metadata = fs::metadata(&full_path)?;

        Ok(ScriptDetails {
            name: script_details.name,
            path: script_details.path,
            curl_command: script_details.curl_command,
            created_at: metadata
                .created()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
                .unwrap_or(0),
            updated_at: metadata
                .modified()
                .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
                .unwrap_or(0),
        })
    }
}
