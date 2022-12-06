//! Watcher

use std::{
    path::PathBuf,
    process::Child,
    sync::{mpsc::channel, Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::{
    core::{config::Config, exec::Exec, walker::Walker},
    log_info,
};

pub struct Watcher {
    pub config: Arc<Config>,
    pub command: Arc<Mutex<Exec>>,
    pub walker: Arc<Mutex<Walker>>,
}

impl Clone for Watcher {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            command: self.command.clone(),
            walker: self.walker.clone(),
        }
    }
}

impl Watcher {
    pub fn new(folder_path: Option<PathBuf>, exec: Option<String>) -> Self {
        let config = Arc::new(Config::new(folder_path, exec));
        let command = Arc::new(Mutex::new(Exec::new(
            config.exec.as_ref().unwrap().to_string(),
        )));
        let walker = Arc::new(Mutex::new(Walker::new()));
        Self {
            config,
            command,
            walker,
        }
    }

    pub fn watch(&mut self) {
        let (sndr, recv) = channel::<bool>();
        if self.config.verbose.unwrap() {
            log_info!("Starts watching ... ");
        }

        let command_config = self.config.clone();
        let command_exec = self.command.clone();
        let command_thread = std::thread::spawn(move || {
            let mut command = command_exec.lock().unwrap();
            let mut prev: Option<Child> = None;
            while let Ok(_) = recv.recv() {
                if prev.is_some() {
                    prev.unwrap().kill().ok();
                }
                if command_config.verbose.unwrap() {
                    log_info!("Executing command: {}", command.exec_string);
                }
                prev = Some(command.run_as_child().expect(
                    format!("Can't execute such a process: {}", command.exec_string).as_str(),
                ));
            }
        });

        let watcher_config = self.config.clone();
        let watcher_walker = self.walker.clone();
        let watcher_thread = std::thread::spawn(move || {
            let mut is_first_run = true;
            loop {
                let mut walker = watcher_walker.try_lock().unwrap();
                walker.walk(
                    watcher_config.root_folder.as_ref().unwrap(),
                    watcher_config.clone(),
                    sndr.clone(),
                );
                if is_first_run {
                    sndr.send(true).unwrap();
                    is_first_run = false;
                } else {
                    walker.check();
                }
                sleep(Duration::from_millis(watcher_config.delay.unwrap()));
            }
        });

        command_thread.join().unwrap();
        watcher_thread.join().unwrap();
    }
}
