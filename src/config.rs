use eyre::Result;
use serde::Deserialize;
use xdg::BaseDirectories;
use std::{fs::{self}, path::PathBuf};

use crate::cli::APP_NAME;

#[derive(Deserialize)]
pub struct Handler {
    pub pattern: String,
    pub script: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub handlers: Vec<Handler>,
}

fn get_config_dir() -> Result<BaseDirectories> {
    Ok(xdg::BaseDirectories::with_prefix(APP_NAME)?)
}

impl Config {
    pub fn from_file() -> Result<Config> {
        let config_dir = get_config_dir()?;
        let config_path = config_dir.find_config_file("config.toml");
        match config_path {
            None => Ok(Config { handlers: vec![] }),
            Some(path) => {
                let config_str = fs::read_to_string(path)?;
                let config: Config = toml::from_str(&config_str)?;
                Ok(config)
            }
        }
    }
    
    pub fn get_script_path(script_name: &String) -> Result<Option<PathBuf>> {
        let config_dir = get_config_dir()?;
        let script_path = config_dir.find_config_file(format!("scripts/{}", script_name));
        Ok(script_path)
    }
}
