use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "apiKey")]
    pub api_key: Option<String>,

    #[serde(rename = "systemContext")]
    pub system_context: Option<String>,
}

impl Config {
    pub fn new() -> Result<Config> {
        read_config_by_json_file()
    }

    pub fn default() -> Config {
        Config {
            api_key: None,
            system_context: None,
        }
    }
}

fn read_config_by_json_file() -> Result<Config> {
    let path = get_config_filepath();
    let config_string = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_string)?;
    Ok(config)
}

// ~/.config/chatgpt-repl/config.json
pub fn get_config_filepath() -> PathBuf {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".config");
    path.push("chatgpt-repl");
    path.push("config.json");
    path
}

fn write_config(config: Config) -> Result<()> {
    let path = get_config_filepath();
    let config_string = serde_json::to_string_pretty(&config)?;

    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, config_string)?;
    Ok(())
}

pub fn write_api_key(api_key: String) -> Result<()> {
    let mut config = Config::new().unwrap_or(Config::default());
    config.api_key = Some(api_key);
    write_config(config)
}
