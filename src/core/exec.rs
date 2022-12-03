//! Command to execute

use std::{
    io,
    process::{Child, Command},
};

pub struct Exec {
    pub command: Command,
    pub exec_string: String,
}

impl Exec {
    pub fn new(exec: String) -> Self {
        let mut exec_vec = exec.split(" ").collect::<Vec<&str>>();
        let mut command = Command::new(exec_vec[0]);
        exec_vec.remove(0);
        command.args(exec_vec);
        Exec {
            command,
            exec_string: exec,
        }
    }

    pub fn run_as_child(&mut self) -> Result<Child, io::Error> {
        self.command.spawn()
    }
}
