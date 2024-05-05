extern crate toml;
use dirs::config_dir;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::{self, prelude::*};
// Define a struct to hold your configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub user: String,
    pub api_key: String,
    pub unlist: bool,
}

pub fn deserialized() -> io::Result<Config> {
    // Read the TOML string from the file
    let dir_path = config_dir().unwrap().join("omg.paste.cli");
    let file_path = dir_path.join("config.toml");
    let mut file = File::open(file_path)?;
    let mut toml_string = String::new();
    file.read_to_string(&mut toml_string)?;

    // Deserialize the TOML string into a Config struct
    let config: Config = toml::from_str(&toml_string).unwrap();
    Ok(config)
}
