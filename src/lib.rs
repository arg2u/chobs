//! Chobs library entry points

pub mod core;

use crate::core::config::Config;
use crate::core::watcher::Watcher;
use std::path::PathBuf;

/// Start watching process
/// `folder_path` - directory to observe
/// `exec` - command for process execution
/// ```
/// use chobs::watch;
/// use std::path::PathBuf;
///
/// fn main() {
///     watch(Some(PathBuf::from(".")), "cargo run".to_string());
/// }
/// ```
pub fn watch(root_folder: Option<PathBuf>, exec: String) {
    let mut watcher = Watcher::new(root_folder, exec);
    watcher.watch();
}

/// Create chobs.json config file.
/// If you already have this file, it will be overwritten.
/// ```
/// use chobs::init_config;
///
/// fn main() {
///     init_config();
/// }
/// ```
pub fn init_config() {
    log_info!("Creating chobs.json ...");
    Config::init().unwrap();
}
