use std::{env, path::PathBuf};

use config::Config;

pub mod config;
pub mod conversion;
pub mod menu;
pub mod theme;

pub fn dot_config() -> PathBuf {
  // We run as sudo so get the OG user
  let user = env::var("SUDO_USER").unwrap_or_default();
  let path = format!("/home/{user}/.config");
  PathBuf::from(path)
}

pub fn scaffold_folders() {
  let theme_path = theme::theme_path();
  let config_file = config::config_file();

  if !config_file.exists() {
    config::save_config(&Config::default()).unwrap_or_default();
  }

  if !theme_path.exists() {
    std::fs::create_dir_all(theme_path).unwrap_or_default();
  }
}