use std::{env, path::PathBuf};

use config::Config;
use uzers::User;

pub mod config;
pub mod conversion;
pub mod menu;
pub mod root;
pub mod theme;

pub fn dot_config() -> PathBuf {
  // We run as sudo so get the OG user
  let mut user = env::var("SUDO_USER").unwrap_or_default();

  // If we aren't running as SUDO, it could be that we ran with pkexec
  if user.is_empty() {
    // Get the PKEXEC_UID
    let uid = env::var("PKEXEC_UID")
      .unwrap_or_default()
      .parse::<u32>()
      .unwrap_or_default();
    // Then get the username
    user = uzers::get_user_by_uid(uid)
      .unwrap_or(User::new(0, "root", 0))
      .name()
      .to_string_lossy()
      .to_string();
  }

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
