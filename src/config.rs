use std::fs;
use std::path;

use serde_derive::Deserialize;
use toml;

use crate::error::ErrorHandleable;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub command: String,
    pub file_name: String,
    pub task_url: String,
}

impl Config {
    pub fn load(file: &path::Path) -> Config {
        let file_name = file.to_string_lossy();
        let text = fs::read_to_string(file)
            .handle_error(&format!("Failed to read a config file '{}'", file_name));
        toml::from_str(&text)
            .handle_error(&format!("Failed to parse a config file '{}'", file_name))
    }
}
