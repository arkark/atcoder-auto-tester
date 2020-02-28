use std::fs;
use std::path;
use std::process;

use regex::Regex;

use crate::args::Args;
use crate::command::Command;
use crate::config::Config;
use crate::error::ErrorHandleable;
use crate::message::Messageable;

pub struct Runner {
    args: Args,
    config: Config,
    file_regex: Regex,
}

impl Runner {
    pub fn new(args: Args, config: Config) -> Self {
        let pattern = config.file_name.replace("{}", r"([^\.]+)");
        let file_regex = regex::Regex::new(&pattern).handle_error(&format!(
            "Failed to parse a regular expression: {}",
            pattern
        ));
        Runner {
            args,
            config,
            file_regex,
        }
    }

    pub fn run(&self, file_name: &str) -> Option<process::ExitStatus> {
        self.match_file(&file_name).map(|task_name| {
            Self::notify_detection(&file_name);
            Self::create_test_directory(&self.args.test_directory);

            let path = self.args.test_directory.join(task_name);
            let url = self.config.task_url.replace("{}", task_name);
            let command = format!(
                "timeout {} {}",
                self.args.timeout,
                self.config.command.replace("{}", task_name)
            );

            if !path.exists() {
                let exit_status = Self::download(&url, &path);
                if !exit_status.success() {
                    return exit_status;
                }
            }
            Self::test(&command, &path)
        })
    }

    fn notify_detection(file_name: &str) {
        println!();
        format!("Detected change in file '{}' ", file_name)
            .info_message()
            .show();
    }

    fn match_file<'a>(&self, file_name: &'a str) -> Option<&'a str> {
        self.file_regex
            .captures(file_name)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
    }

    fn create_test_directory(path: &path::Path) {
        fs::create_dir_all(path).handle_error(&format!(
            "Failed to create a directory: {}",
            path.to_string_lossy()
        ));
    }

    fn download(url: &str, path: &path::Path) -> process::ExitStatus {
        Command::new("oj")
            .arg("download")
            .arg("--directory")
            .arg(&path.to_string_lossy())
            .arg(url)
            .show()
            .run()
    }

    fn test(command: &str, path: &path::Path) -> process::ExitStatus {
        Command::new("oj")
            .arg("test")
            .arg("--command")
            .arg(command)
            .arg("--directory")
            .arg(&path.to_string_lossy())
            .show()
            .run()
    }
}
