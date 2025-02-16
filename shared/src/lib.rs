use std::{env, fmt::Display, path::PathBuf};

use uzers::User;

pub mod config;
pub mod convert;
pub mod root;
pub mod theme;

#[derive(Clone, Debug, Default)]
pub enum App {
  #[default]
  GlacierDiskInfo,
  GlacierDiskMark,
}

impl Display for App {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      App::GlacierDiskInfo => write!(f, "glacierdiskinfo"),
      App::GlacierDiskMark => write!(f, "glacierdiskmark"),
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
