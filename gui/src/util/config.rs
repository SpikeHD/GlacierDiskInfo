use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::dot_config;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
  // Path to the theme
  pub theme: String,
}

pub fn config_file() -> PathBuf {
  let cfg = dot_config();
  cfg.join("glacierdisk").join("config.toml")
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
  let path = config_file();
  let contents = std::fs::read_to_string(path)?;
  let config: Config = toml::from_str(&contents)?;
  Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
  let path = config_file();
  let contents = toml::to_string(config)?;
  std::fs::write(path, contents)?;
  Ok(())
}