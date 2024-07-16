pub mod models;
mod manager;
mod operations;
mod singleton;
pub mod errors;

pub use manager::ScriptManager;
pub use singleton::{get_script_manager, initialize_script_manager};
pub use errors::{ScriptManagerError, ErrorKind};
