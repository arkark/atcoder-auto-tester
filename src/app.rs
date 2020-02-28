use std::fs;
use std::path;
use std::process;

use crate::args::Args;
use crate::command::Command;
use crate::config::Config;
use crate::detector::Detector;
use crate::runner::Runner;

pub struct App;

impl App {
    pub fn clean(test_dir: &path::Path) -> ! {
        if test_dir.exists() {
            fs::remove_dir_all(&test_dir).expect("Failed to delete a test directory");
        }
        process::exit(0)
    }

    pub fn login() -> ! {
        let exit_status = Command::new("oj")
            .arg("login")
            .arg("https://atcoder.jp/")
            .show()
            .run();
        Self::exit(exit_status)
    }

    pub fn run(args: Args, config: Config) -> ! {
        let detector = Detector::new();
        let runner = Runner::new(args, config);
        for file_name in detector.into_iter() {
            runner.run(&file_name);
        }
        unreachable!();
    }

    fn exit(exit_status: process::ExitStatus) -> ! {
        process::exit(exit_status.code().unwrap_or(1))
    }
}
