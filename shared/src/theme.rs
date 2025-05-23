use std::path::PathBuf;

use regex::Regex;

use crate::App;

use super::dot_config;

#[derive(Debug)]
pub struct Theme {
  pub path: PathBuf,
  pub name: String,
}

pub fn themes(app: App) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let mut themes = vec![];
  let themes_path = theme_path(app);

  if themes_path.exists() {
    for entry in std::fs::read_dir(themes_path)? {
      let entry = entry?;
      let path = entry.path();
      let name = path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_string();
      themes.push(name);
    }
  }

  Ok(themes)
}

pub fn theme_path(app: App) -> PathBuf {
  let cfg = dot_config();
  cfg.join(app.to_string()).join("themes")
}

/**
 * Themes should have a comment with a line like this:
 * /* ThemeName */
 */
pub fn read_theme_data(app: App, filename: String) -> Result<Theme, Box<dyn std::error::Error>> {
  let root = theme_path(app);
  let path = root.join(&filename);
  let contents = std::fs::read_to_string(&path)?;
  let reg = Regex::new(r"\/\*( |)(.*)( |)\*\/").unwrap();
  let mut name = String::new();

  for cap in reg.captures_iter(&contents) {
    if cap.len() > 2 {
      name = cap[2].to_string();
    }
  }

  // If there is no name, make it the filename
  if name.is_empty() {
    name = filename;
  }

  Ok(Theme {
    path: path.to_path_buf(),
    name,
  })
}

pub fn read_theme_contents(theme: &Theme) -> Result<String, Box<dyn std::error::Error>> {
  if theme.name == "none" {
    return Ok("".to_string());
  }

  let contents = std::fs::read_to_string(&theme.path)?;
  Ok(contents)
}
