use std::{env, path::PathBuf};

use uzers::User;

pub mod config;
pub mod convert;
pub mod theme;

#[derive(Clone, Debug)]
pub enum App {
  GlacierDiskInfo,
  GlacierDiskMark,
}

impl Default for App {
  fn default() -> Self {
    App::GlacierDiskInfo
  }
}

impl ToString for App {
  fn to_string(&self) -> String {
    match self {
      App::GlacierDiskInfo => "glacierdiskinfo".to_string(),
      App::GlacierDiskMark => "glacierdiskmark".to_string(),
    }
  }
}

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