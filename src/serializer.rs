use serde_derive::Serialize;
use std::fs::File;
use std::io::{self, prelude::*};
#[derive(Debug, Serialize)]
pub struct Config {
    // Define your configuration fields
    pub user: String,
    pub api_key: String,
}
pub fn serialize(user: String, api_key: String) -> io::Result<Config> {
    // Instantiate your configuration
    let config = Config {
        user: user,
        api_key: api_key,
    };

    // Serialize the configuration to a TOML string
    let toml_string = toml::to_string(&config).unwrap();

    // Write the TOML string to a file
    let mut file = File::create("config.toml")?;
    file.write_all(toml_string.as_bytes())?;

    Ok(config)
}
