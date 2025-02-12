use std::path::PathBuf;

use data::{smart::smart_to_string, status::Status};
use dioxus::{desktop::Config, prelude::*};
use ui::{drive::Drive, drive_tabs::DriveTabs};

mod data;
mod ui;
mod util;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
  sudo::escalate_if_needed().expect("Failed to escalate privileges");
  let window = dioxus::desktop::tao::window::WindowBuilder::new()
    .with_title("GlacierDiskInfo")
    .with_resizable(true);
  dioxus::LaunchBuilder::new()
    .with_cfg(Config::default().with_menu(None).with_window(window))
    .launch(App);
}

#[component]
fn App() -> Element {
  let drives = libglacierdisk::list_disks().expect("Failed to list disks");
  let drives: Vec<(String, Status)> = drives
    .iter()
    .map(|d| {
      let mut status =
        libglacierdisk::get_disk_info(PathBuf::from(d)).expect("Failed to get disk info");
      let smart = status
        .smart_get_overall()
        .expect("Failed to get smart status");
      let state = smart_to_string(smart);

      let temp = status.get_temperature().unwrap_or(0);

      // convert mkelvin to celsius
      let temp = (temp as f32 / 1000.) - 273.15;

      (d.to_string(), Status { temp, state })
    })
    .collect();

  let mut selected_drive = use_signal(|| drives[0].0.clone());

  rsx! {
      document::Link { rel: "stylesheet", href: MAIN_CSS }

      DriveTabs {
        drives,
        selected_drive: selected_drive(),
        on_select: move |name| {
          println!("selected drive: {}", name);
          selected_drive.set(name);
        }
      }

      Drive {
        selected_drive: selected_drive(),
      }
  }
}
