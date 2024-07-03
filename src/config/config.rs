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
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv::dotenv().ok();
        
        let mode = match env::var("MODE").unwrap_or_else(|_| "DEVEL".to_string()).as_str() {
            "PROD" => Mode::PROD,
            _ => Mode::DEVEL,
        };

        let devfrontport = env::var("DEVFRONTPORT")
            .unwrap_or_else(|_| "5174".to_string())
            .parse()
            .unwrap_or(5174);

        let scripts_folder = PathBuf::from(env::var("SCRIPTSFOLDER")
            .unwrap_or_else(|_| "rest-examples".to_string()));

        let port = env::var("PORT")
            .unwrap_or_else(|_| "2080".to_string())
            .parse()
            .unwrap_or(2080);

        Ok(Config {
            mode,
            devfrontport,
            scripts_folder,
            port,
        })
    }

    pub fn is_development(&self) -> bool {
        matches!(self.mode, Mode::DEVEL)
    }

    pub fn dev_server_url(&self) -> String {
        format!("http://localhost:{}", self.devfrontport)
    }
}