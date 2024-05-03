extern crate toml;
use serde_derive::Deserialize; // Import Deserialize trait
use std::fs::File;
use std::io::{self, prelude::*};
// Define a struct to hold your configuration
#[derive(Debug, Deserialize)] // Derive Deserialize trait
pub struct Config {
    // Define your configuration fields
    pub user: String,
    pub api_key: String,
}

pub fn deserialized() -> io::Result<Config> {
    // Read the TOML string from the file
    let mut file = File::open("config.toml")?;
    let mut toml_string = String::new();
    file.read_to_string(&mut toml_string)?;

    // Deserialize the TOML string into a Config struct
    let config: Config = toml::from_str(&toml_string).unwrap();
    Ok(config)
}
