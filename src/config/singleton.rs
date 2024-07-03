use std::sync::OnceLock;
use super::config::Config;

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn initialize_config() {
    let config = Config::from_env().expect("Failed to load configuration");
    CONFIG.set(config).expect("Failed to set global config");
}

pub fn get_config() -> &'static Config {
    CONFIG.get().expect("Config not initialized")
}
