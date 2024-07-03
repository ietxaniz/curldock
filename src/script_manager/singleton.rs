use std::sync::{Arc, OnceLock};
use super::ScriptManager;

static SCRIPT_MANAGER: OnceLock<Arc<ScriptManager>> = OnceLock::new();

pub fn initialize_script_manager() {
    let config = crate::config::get_config();
    let manager = ScriptManager::new(config.scripts_folder.clone());
    SCRIPT_MANAGER.set(manager).expect("Failed to set ScriptManager");
}

pub fn get_script_manager() -> &'static Arc<ScriptManager> {
    SCRIPT_MANAGER.get().expect("ScriptManager not initialized")
}