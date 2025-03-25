use data::{disk_cache::DiskCache, drives_and_status, status::Status};
use dialog::DialogBox;
use dioxus::{
  desktop::{
    tao::{dpi::LogicalSize, window::WindowBuilder},
    Config,
  },
  prelude::*,
};
use dioxus_desktop::muda::MenuId;
use shared::{
  config::{self, load_config},
  theme::{self, read_theme_contents},
  App,
};
use ui::{drive::Drive, drive_tabs::DriveTabs};
use util::menu;

use crate::assets::CSS;

mod assets;
mod data;
mod ui;
mod util;

pub static DRIVES: Global<Vec<(DiskCache, Status)>> = Global::new(drives_and_status);

fn main() {
  util::scaffold_folders();

  match sudo::check() {
    sudo::RunningAs::Root => (),
    sudo::RunningAs::User => shared::root::pk_reopen(),
    sudo::RunningAs::Suid => {
      sudo::escalate_if_needed().expect("Failed to escalate privileges");
    }
  };

  // let window = WindowBuilder::new()
  //   .with_title("GlacierDiskInfo")
  //   .with_resizable(true)
  //   .with_inner_size(LogicalSize::new(1500, 800))
  //   .with_min_inner_size(LogicalSize::new(1500, 800));

  // let config = Config::default()
  //   .with_menu(Some(menu::create_menu()))
  //   .with_window(window);

  //dioxus::LaunchBuilder::new().with_cfg(config).launch(Root);

  dioxus_native::launch(Root);
}

#[component]
fn Root() -> Element {
  let mut theme_css = use_signal(|| {
    let config = load_config(App::GlacierDiskInfo).unwrap_or_default();
    let theme = config.get_theme();

    match theme {
      Some(t) => read_theme_contents(&t).unwrap_or_default(),
      None => "".to_string(),
    }
  });

  // Submenu handler
  // dioxus::desktop::use_muda_event_handler(move |e| {
  //   let id = match e.id() {
  //     MenuId(s) => s,
  //   }
  //   .to_owned();

  //   if id.starts_with("apply-") {
  //     let mut config = config::load_config(App::GlacierDiskInfo).unwrap_or_default();
  //     let name = id.strip_prefix("apply-").unwrap_or_default();

  //     println!("Applying theme: {name}");

  //     config.theme = name.to_string();
  //     config::save_config(App::GlacierDiskInfo, &config).unwrap_or_default();

  //     // Apply the CSS
  //     if let Some(theme) = config.get_theme() {
  //       let contents = read_theme_contents(&theme).unwrap_or_default();
  //       theme_css.set(contents);
  //     } else {
  //       theme_css.set("".to_string());
  //     }
  //   } else if id == "add-theme" {
  //     let theme_path = theme::theme_path(App::GlacierDiskInfo);
  //     open::that_detached(theme_path).unwrap_or_default();
  //   } else if id == "about" {
  //     let version = env!("CARGO_PKG_VERSION");
  //     let git_sha = option_env!("GIT_SHA").unwrap_or("unknown revision");

  //     // This can block
  //     std::thread::spawn(move || {
  //       dialog::Message::new(format!("GlacierDiskInfo GUI v{version} ({git_sha})\n\nhttps://github.com/SpikeHD/GlacierDiskInfo\n\nCreated by SpikeHD, inspired by CrystalDiskInfo"))
  //         .title("About")
  //         .show()
  //         .expect("Failed to show dialog");
  //     });
  //   }
  // });

  let mut selected_drive = use_signal(|| DRIVES.resolve()[0].0.clone());

  rsx! {
      style {
        r#"{CSS.join("\n")}"#
      }

      style {
        "{theme_css}"
      }

      DriveTabs {
        selected_drive: selected_drive(),
        on_select: move |disk: DiskCache| {
          println!("selected drive: {:?}", disk.path());
          selected_drive.set(disk);
        }
      }

      Drive {
        selected_drive: selected_drive(),
      }
  }
}
