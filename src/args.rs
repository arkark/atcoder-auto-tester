use std::path;

use crate::error::ErrorHandleable;

#[derive(Debug)]
pub struct Args {
    pub should_clean: bool,
    pub should_login: bool,
    pub config_file: Box<path::Path>,
    pub test_directory: Box<path::Path>,
    pub timeout: u32,
}

impl Args {
    pub fn load() -> Args {
        let matches = clap::App::new("atcoder-auto-tester")
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .setting(clap::AppSettings::ColoredHelp)
            .setting(clap::AppSettings::ArgRequiredElseHelp)
            .version_short("v")
            .version_message("Print version information")
            .help_message("Print help information")
            .arg(
                clap::Arg::with_name("clean")
                    .long("clean")
                    .help("Remove the test directory")
                    .display_order(1),
            )
            .arg(
                clap::Arg::with_name("login")
                    .long("login")
                    .help("Login to AtCoder")
                    .conflicts_with("clean")
                    .display_order(2),
            )
            .arg(
                clap::Arg::with_name("config-file")
                    .short("f")
                    .long("config-file")
                    .value_name("FILE")
                    .help("Set a config file name")
                    .default_value(".config.toml"),
            )
            .arg(
                clap::Arg::with_name("test-directory")
                    .short("d")
                    .long("test-directory")
                    .value_name("DIRECTORY")
                    .help("Set a directory for saving test cases")
                    .default_value(".test"),
            )
            .arg(
                clap::Arg::with_name("timeout")
                    .short("t")
                    .long("timeout")
                    .value_name("VALUE")
                    .help("Set a time limit for test execution [unit: seconds]")
                    .default_value("5"),
            )
            .get_matches();

        let clean = matches.is_present("clean");
        let login = matches.is_present("login");
        assert!(!(clean && login));

        let config_file = matches.value_of("config-file").unwrap();
        let test_directory = matches.value_of("test-directory").unwrap();
        let timeout = matches.value_of("timeout").unwrap();

        Args {
            should_clean: clean,
            should_login: login,
            config_file: Box::from(path::Path::new(config_file)),
            test_directory: Box::from(path::Path::new(test_directory)),
            timeout: clap::value_t!(matches.value_of("timeout"), u32).handle_error(&format!(
                "The argument '{}' isn't a valid value for 'timeout'. Set a non-negative integer.",
                timeout
            )),
        }
    }
}
