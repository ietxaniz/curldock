use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug)]
pub struct ScriptManager {
    scripts_dir: PathBuf,
    #[allow(dead_code)]
    // TODO: This attribute suppreses the warning, but should be removed once we implement some write lock.
    write_lock: Mutex<()>,
}

impl ScriptManager {
    pub fn new(scripts_dir: PathBuf) -> Arc<Self> {
        Arc::new(Self {
            scripts_dir,
            write_lock: Mutex::new(()),
        })
    }

    pub fn scripts_dir(&self) -> &PathBuf {
        &self.scripts_dir
    }

    pub fn lock(&self) -> MutexGuard<()> {
        self.write_lock.lock().unwrap()
    }
}
