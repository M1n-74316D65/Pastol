use dirs::config_dir;
use serde_derive::Serialize;
use std::fs;
use std::fs::File;
use std::io::{self, prelude::*};
#[derive(Debug, Serialize)]
pub struct Config {
    pub user: String,
    pub api_key: String,
    pub unlist: bool,
}
pub fn serialize(user: String, api_key: String, unlist: bool) -> io::Result<Config> {
    let config = Config {
        user,
        api_key,
        unlist,
    };

    let toml_string = toml::to_string(&config).unwrap();

    let dir_path = config_dir().unwrap().join("omg.paste.cli");
    fs::create_dir_all(&dir_path).expect("Failed to create directory");
    let file_path = dir_path.join("config.toml");
    let mut file = File::create(file_path)?;
    file.write_all(toml_string.as_bytes())?;

    Ok(config)
}
