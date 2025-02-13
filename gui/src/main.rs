use data::{smart::smart_to_string, status::Status};
use dioxus::{desktop::{
  Config,
  tao::window::WindowBuilder,
  tao::dpi::LogicalSize
}, prelude::*};
use ui::{drive::Drive, drive_tabs::DriveTabs};

mod data;
mod ui;
mod util;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
  sudo::escalate_if_needed().expect("Failed to escalate privileges");
  let window = WindowBuilder::new()
    .with_title("GlacierDiskInfo")
    .with_resizable(true)
    .with_min_inner_size(LogicalSize::new(1000, 600));
  dioxus::LaunchBuilder::new()
    .with_cfg(Config::default().with_menu(None).with_window(window))
    .launch(App);
}

#[component]
fn App() -> Element {
  let drives = libglacierdisk::list_disks().expect("Failed to list disks");
  let mut drives: Vec<(String, Status)> = drives
    .iter()
    .filter_map(|d| {
      let mut status = match libglacierdisk::get_disk_info(d) {
        Ok(d) => d,
        Err(e) => {
          eprintln!("Error fetching disk at {:?}: {e}", d);
          return None;
        }
      };
      let smart = match status
        .smart_get_overall() {
          Ok(s) => s,
          Err(e) => {
            eprintln!("Error fetching smart status: {e}");
            return None;
          }
        };
      let state = smart_to_string(smart);

      let temp = status.get_temperature().unwrap_or(0);

      // convert mkelvin to celsius
      let temp = (temp as f32 / 1000.) - 273.15;

      Some((d.to_string_lossy().to_string(), Status { temp, state }))
    })
    .collect();

  // If drives is empty, we have to create a dummy
  if drives.is_empty() {
    drives.push(("No Disks Found".to_string(), Status { temp: 0., state: "Good".to_string() }));
  }

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
