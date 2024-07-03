use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Deserialize, Clone, Debug)]
pub enum Mode {
    DEVEL,
    PROD,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub mode: Mode,
    pub devfrontport: u16,
    pub scripts_folder: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv::dotenv().ok(); // This line loads the .env file if it exists
        
        let mode = match env::var("MODE").unwrap_or_else(|_| "DEVEL".to_string()).as_str() {
            "PROD" => Mode::PROD,
            _ => Mode::DEVEL,
        };

        let devfrontport = env::var("DEVFRONTPORT")
            .unwrap_or_else(|_| "7153".to_string())
            .parse()
            .unwrap_or(7153);

        let scripts_folder = PathBuf::from(env::var("SCRIPTSFOLDER")
          .unwrap_or_else(|_| "rest-examples".to_string()));

        Ok(Config {
            mode,
            devfrontport,
            scripts_folder,
        })
    }

    pub fn is_development(&self) -> bool {
        matches!(self.mode, Mode::DEVEL)
    }

    pub fn dev_server_url(&self) -> String {
        format!("http://localhost:{}", self.devfrontport)
    }
}
