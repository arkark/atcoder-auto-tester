use std::fmt;
use std::process;

use crate::error::ErrorHandleable;
use crate::message::Messageable;

#[derive(Debug)]
pub struct Command {
    program: String,
    args: Vec<String>,
}

impl Command {
    pub fn new(program: &str) -> Self {
        Command {
            program: String::from(program),
            args: vec![],
        }
    }

    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(String::from(arg));
        self
    }

    pub fn run(&self) -> process::ExitStatus {
        process::Command::new(&self.program)
            .args(&self.args)
            .stdout(process::Stdio::piped())
            .spawn()
            .handle_error("Failed to execute child process")
            .wait()
            .handle_error("Failed to wait child process")
    }

    pub fn show(self) -> Self {
        format!("{}", self).command_message().show();
        self
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.args
                .iter()
                .map(|arg| format!("{:?}", arg))
                .fold(format!("{:?}", self.program), |acc, arg| acc + " " + &arg)
        )
    }
}
