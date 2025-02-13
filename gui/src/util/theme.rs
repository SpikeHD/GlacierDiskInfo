use std::path::{Path, PathBuf};

use regex::Regex;

use super::dot_config;

#[derive(Debug)]
pub struct Theme {
  pub path: PathBuf,
  pub name: String,
}

pub fn themes() -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let mut themes = vec![];
  let themes_path = theme_path();

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

pub fn theme_path() -> PathBuf {
  let cfg = dot_config();
  cfg.join("glacierdisk").join("themes")
}

/**
 * Themes should have a comment with a line like this:
 * // @NAME: MyTheme
 */
pub fn read_theme_data(filename: String) -> Result<Theme, Box<dyn std::error::Error>> {
  let root = theme_path();
  let path = root.join(&filename);
  let contents = std::fs::read_to_string(&path)?;
  let reg = Regex::new(r"\/\/( |)@NAME:( |)(.*)").unwrap();
  let mut name = String::new();

  for cap in reg.captures_iter(&contents) {
    if cap.len() > 3 {
      name = cap[3].to_string();
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