use std::path::PathBuf;

use dioxus::prelude::*;

use crate::{
  data::smart::{smart_to_string, DriveStatus},
  ui::{drive_attr_table::DriveAttrTable, drive_info_table::DriveInfoTable},
  util::conversion::bytes_to_readable,
};

static CSS: Asset = asset!("/assets/drive.css");

#[derive(Props, PartialEq, Clone)]
pub struct DriveProps {
  pub selected_drive: String,
}

#[component]
pub fn Drive(props: DriveProps) -> Element {
  let mut drive = match libglacierdisk::get_disk_info(&PathBuf::from(props.selected_drive.clone()))
  {
    Ok(drive) => drive,
    Err(_) => {
      return rsx! {
        div {
          "Failed to get drive info"
        }
      }
    }
  };
  let identity = drive.identify_parse().expect("Failed to get identify info");
  let size = drive.get_disk_size().expect("Failed to get disk size");
  let size = bytes_to_readable(size);
  let status = smart_to_string(
    drive
      .smart_get_overall()
      .expect("Failed to get smart status"),
  );
  let status_class = match DriveStatus::from_smart(status.as_str()) {
    DriveStatus::Good => "good",
    DriveStatus::Caution => "caution",
    DriveStatus::Bad => "bad",
  };
  let temp = drive.get_temperature().expect("Failed to get temperature");
  let temp = (temp as f32 / 1000.) - 273.15;
  let temp = if temp == 0. {
    "--".into()
  } else {
    temp.to_string()
  };

  rsx! {
    document::Link { rel: "stylesheet", href: CSS },

    div {
      class: "drive",

      div {
        class: "drive-name",
        "{identity.model} {size} ({props.selected_drive})"
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
