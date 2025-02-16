use shared::{
  config::{self, Config},
  theme, App,
};

pub mod menu;

pub fn scaffold_folders() {
  let app = App::GlacierDiskMark;
  let theme_path = theme::theme_path(app.clone());
  let config_file = config::config_file(app.clone());

  if !config_file.exists() {
    config::save_config(app, &Config::default()).unwrap_or_default();
  }

  if !theme_path.exists() {
    std::fs::create_dir_all(theme_path).unwrap_or_default();
  }
}
