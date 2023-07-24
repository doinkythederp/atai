use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub chatgpt: ChatgptConfig,
}

#[derive(Debug, Deserialize)]
pub struct ChatgptConfig {
    pub token: String,
    pub url: Option<String>,
}

impl Config {
    pub fn open() -> Self {
        let path = home::home_dir().unwrap().join(".config/atai/config.toml");
        let config = fs::read_to_string(path).unwrap();
        toml::from_str(&config).unwrap()
    }
}
