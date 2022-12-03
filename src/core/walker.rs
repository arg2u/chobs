//! Directory walker

use std::{
    borrow::BorrowMut,
    collections::HashMap,
    fs::read_dir,
    path::PathBuf,
    sync::{mpsc::Sender, Arc},
    time::SystemTime,
};

use crate::core::config::Config;

type DirStructure = HashMap<PathBuf, SystemTime>;

pub struct Walker {
    pub dir_struct: DirStructure,
}

impl Walker {
    pub fn new() -> Self {
        let dir_struct: DirStructure = HashMap::new();
        Self { dir_struct }
    }

    pub fn walk(&mut self, path: &PathBuf, config: Arc<Config>, sndr: Sender<bool>) {
        if path.exists() {
            for entry in read_dir(path).unwrap() {
                let entry = entry.unwrap();
                if !config.is_path_ignored(entry.path()) {
                    if let Some(time) = self.dir_struct.get(&entry.path()) {
                        let cur_time = entry.path().metadata().unwrap().modified().unwrap();
                        if *time != cur_time {
                            self.dir_struct.insert(entry.path(), cur_time);
                            sndr.send(true).unwrap();
                        }
                    } else {
                        self.dir_struct
                            .insert(entry.path(), entry.metadata().unwrap().modified().unwrap());
                    }
                    if entry.path().is_dir() {
                        self.walk(&entry.path(), config.clone(), sndr.clone());
                    }
                }
            }
        } else {
            panic!("You've provided the wrong directory to observe!")
        }
    }

    pub fn check(&mut self) {
        self.dir_struct.borrow_mut().retain(|key, _| key.exists());
    }
}
