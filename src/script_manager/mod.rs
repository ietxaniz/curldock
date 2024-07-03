pub mod models;
mod manager;
mod operations;
mod singleton;

pub use manager::ScriptManager;
pub use singleton::{get_script_manager, initialize_script_manager};