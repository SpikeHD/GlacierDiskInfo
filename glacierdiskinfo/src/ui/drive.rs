use dioxus::prelude::*;
use shared::convert::bytes_to_readable;

use crate::{
  data::{disk_cache::DiskCache, smart::DriveStatus},
  ui::{drive_attr_table::DriveAttrTable, drive_info_table::DriveInfoTable},
};

#[derive(Props, PartialEq, Clone)]
pub struct DriveProps {
  pub selected_drive: DiskCache,
}

#[component]
pub fn Drive(props: DriveProps) -> Element {
  let drive = props.selected_drive.clone();
  let path = drive.path().to_string_lossy().to_string();
  let identity = drive.identity();
  let size = drive.size();
  let size = bytes_to_readable(size);
  let status = drive.smart_overall();
  let status_class = match DriveStatus::from_smart(status) {
    DriveStatus::Good => "good",
    DriveStatus::Caution => "caution",
    DriveStatus::Bad => "bad",
  };
  let temp = drive.temperature();
  let temp = (temp as f32 / 1000.) - 273.15;
  let temp = if temp == 0. {
    "--".into()
  } else {
    temp.to_string()
  };

  rsx! {
    div {
      class: "drive",

      div {
        class: "drive-name",
        "{identity.model} {size} ({path})"
      }

      div {
        class: "drive-info",

        div {
          class: "drive-health-temp",

          div {
            class: "drive-health-elm",
            span {
              "Health Status"
            }
            span {
              class: "drive-health-status ".to_owned() + status_class,
              "{status}"
            }
          }

          div {
            class: "drive-health-elm",
            span {
              "Temperature"
            }
            span {
              class: "drive-temp-status",
              "{temp} °C"
            }
          }
        }

        DriveInfoTable {
          selected_drive: props.selected_drive.clone(),
        }
      },

      DriveAttrTable {
        selected_drive: props.selected_drive.clone(),
      }
    }
  }
}
