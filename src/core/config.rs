//! Chobs configuration

use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Chobs configuration structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Disables/enables chobs logs
    pub verbose: Option<bool>,
    /// Directories and files to ignore
    pub ignore: Option<Vec<String>>,
    /// Observation delay
    pub delay: Option<u64>,
    /// Root folder to observe
    pub root_folder: Option<PathBuf>,
}

impl Default for Config {
    /// Creates default config instance
    fn default() -> Self {
        Self {
            verbose: Some(true),
            ignore: Some(vec!["target".to_string(), ".git".to_string()]),
            delay: Some(1000),
            root_folder: Some(PathBuf::from(".")),
        }
    }
}

impl Config {
    /// Creates default chobs.json config file
    pub fn init() -> Result<(), io::Error> {
        let config = Config::default();
        let path = Path::new("chobs.json");
        let json = serde_json::to_string(&config).unwrap();
        fs::write(path, json)?;
        Ok(())
    }

    /// Creates config instance
    pub fn new(folder_path: Option<PathBuf>) -> Self {
        let mut config: Config;
        let path = Path::new("chobs.json");
        if folder_path.is_some() {
            std::env::set_current_dir(folder_path.as_ref().unwrap()).unwrap();
        }
        if !path.exists() {
            config = Config::default();
        } else {
            let json_str = std::fs::read_to_string(path).unwrap();
            config = Config::from(&json_str[..]);
        }
        if folder_path.is_some() {
            config.root_folder = folder_path;
        }
        config
    }

    /// Checks if ignored vec contains path
    pub fn is_path_ignored(&self, entry_path: PathBuf) -> bool {
        let mut ignored = false;
        if self.ignore.is_some() {
            for path in self.ignore.as_ref().unwrap() {
                let mut path = PathBuf::from(path);
                if path.is_dir() {
                    let root = self.root_folder.as_ref().unwrap();
                    path = root.join(path);
                }
                if entry_path
                    .to_str()
                    .unwrap()
                    .contains(path.to_str().unwrap())
                {
                    ignored = true;
                    break;
                }
            }
        }
        ignored
    }
}

impl From<&str> for Config {
    /// Creates config instance from json string
    fn from(value: &str) -> Self {
        let mut config: Config = serde_json::from_str(value).unwrap();
        config.delay = Some(config.delay.get_or_insert(1000).to_owned());
        config.verbose = Some(config.verbose.get_or_insert(true).to_owned());
        config.ignore = Some(config.ignore.get_or_insert(vec![]).to_owned());
        config.root_folder = Some(
            config
                .root_folder
                .get_or_insert(PathBuf::from("."))
                .to_owned(),
        );
        config
    }
}
