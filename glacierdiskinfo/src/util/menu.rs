use dioxus::desktop::muda::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use shared::App;

use crate::util::theme;

use super::theme::Theme;

pub fn create_menu() -> Menu {
  let menu = Menu::new();
  let app = Submenu::new("App", true);
  let themes = Submenu::new("Theme", true);
  let items = generate_theme_items();

  for item in items {
    themes.append(&item).unwrap_or_else(|e| {
      eprintln!("Failed to append menu item: {e}");
    });
  }

  app.append(&MenuItem::with_id("refresh-disks", "Refresh disks", true, None))
    .unwrap_or_else(|e| {
      eprintln!("Failed to append menu item: {e}");
    });

  themes
    .append(&PredefinedMenuItem::separator())
    .unwrap_or_else(|e| {
      eprintln!("Failed to append menu item: {e}");
    });

  themes
    .append(&MenuItem::with_id("add-theme", "Add theme", true, None))
    .unwrap_or_else(|e| {
      eprintln!("Failed to append menu item: {e}");
    });

  menu.append(&app).unwrap_or_else(|e| {
    eprintln!("Failed to append menu item: {e}");
  });

  menu.append(&themes).unwrap_or_else(|e| {
    eprintln!("Failed to append menu item: {e}");
  });

  menu
    .append(
      &Submenu::with_items(
        "Help",
        true,
        &[&MenuItem::with_id("about", "About", true, None)],
      )
      .expect("Failed to append help menu"),
    )
    .unwrap_or_else(|e| {
      eprintln!("Failed to append menu item: {e}");
    });

  menu
}

fn generate_theme_items() -> Vec<Submenu> {
  let mut items = vec![];
  let themes = match theme::themes(App::GlacierDiskInfo) {
    Ok(t) => t,
    Err(e) => {
      eprintln!("Error reading themes: {e}");
      return vec![];
    }
  };

  for theme in themes {
    let theme = theme::read_theme_data(App::GlacierDiskInfo, theme);
    let theme = match theme {
      Ok(t) => t,
      Err(e) => {
        eprintln!("Error reading theme: {e}");
        continue;
      }
    };

    let submenu = Submenu::new(theme.name.clone(), true);
    let controls = generate_theme_controls(theme);

    for item in controls {
      submenu.append(&item).unwrap_or_else(|e| {
        eprintln!("Failed to append menu item: {e}");
      });
    }

    items.push(submenu);
  }

  // Create a special "None" theme
  let none = Submenu::new("None", true);
  none
    .append(&MenuItem::with_id(
      "apply-none".to_string(),
      "Apply",
      true,
      None,
    ))
    .unwrap_or_default();

  items.push(none);

  items
}

fn generate_theme_controls(theme: Theme) -> Vec<MenuItem> {
  let mut items = vec![];
  let filename = theme
    .path
    .file_name()
    .unwrap_or_default()
    .to_str()
    .unwrap_or_default();

  items.push(MenuItem::with_id(
    format!("apply-{filename}"),
    "Apply",
    true,
    None,
  ));

  items
}
