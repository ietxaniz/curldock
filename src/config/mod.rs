mod config;
mod singleton;

pub use config::Config;
pub use singleton::{initialize_config, get_config};
