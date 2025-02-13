use std::path::PathBuf;

use dioxus::desktop::muda::{Menu, MenuItem, Submenu, SubmenuBuilder, IsMenuItem, PredefinedMenuItem};

use crate::util::theme;

use super::theme::Theme;

pub fn create_menu() -> Menu {
  let menu = Menu::new();
  let submenu = Submenu::new("Theme", true);
  let items = generate_theme_items();

  for item in items {
    submenu.append(&item).unwrap_or_else(|e| {
      eprintln!("Failed to append menu item: {e}");
    });
  }

  submenu.append(
    &PredefinedMenuItem::separator()
  ).unwrap_or_else(|e| {
    eprintln!("Failed to append menu item: {e}");
  });

  submenu.append(
    &MenuItem::new("Add theme", true, None)
  ).unwrap_or_else(|e| {
    eprintln!("Failed to append menu item: {e}");
  });

  menu.append(&submenu).unwrap_or_else(|e| {
    eprintln!("Failed to append menu item: {e}");
  });
  
  menu
}

fn generate_theme_items() -> Vec<Submenu> {
  let mut items = vec![];
  let themes = match theme::themes() {
    Ok(t) => t,
    Err(e) => {
      eprintln!("Error reading themes: {e}");
      return vec![];
    }
  };

  for theme in themes {
    let theme = theme::read_theme_data(theme);
    let theme = match theme {
      Ok(t) => t,
      Err(e) => {
        eprintln!("Error reading theme: {e}");
        continue;
      }
    };

    println!("Theme data: {:?}", theme);

    let submenu = Submenu::new(theme.name.clone(), true);
    let controls = generate_theme_controls(theme);

    for item in controls {
      submenu.append(&item).unwrap_or_else(|e| {
        eprintln!("Failed to append menu item: {e}");
      });
    }

    items.push(submenu);
  }

  items
}

fn generate_theme_controls(theme: Theme) -> Vec<MenuItem> {
  let mut items = vec![];

  items.push(MenuItem::new("Apply", true, None));
  items.push(MenuItem::new("Delete", true, None));

  items
}