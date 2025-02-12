use std::path::PathBuf;

use dioxus::prelude::*;

use crate::util::conversion::ms_to_readable;

static CSS: Asset = asset!("/assets/driveinfotable.css");

#[derive(Props, PartialEq, Clone)]
pub struct DriveInfoTableProps {
  pub selected_drive: String,
}

#[component]
pub fn DriveInfoTable(props: DriveInfoTableProps) -> Element {
  let mut drive = libglacierdisk::get_disk_info(PathBuf::from(props.selected_drive.clone()))
    .expect("Failed to get disk info");
  let identity = drive.identify_parse().expect("Failed to get identify info");
  let table_values = vec![
    ("Firmware", identity.firmware),
    ("Serial", identity.serial),
    ("Model", identity.model),
    ("Drive Path", props.selected_drive.clone()),
    ("Powered On", ms_to_readable(drive.get_power_on().unwrap_or(0))),
    ("Power On Count", drive.get_power_cycle_count().unwrap_or(0).to_string()),
  ];
  let rows = table_values.iter().map(|(name, value)| {
    rsx! {
      div {
        class: "drive-info-row",
        span {
          class: "drive-info-name",
          "{name}"
        }

        span {
          class: "drive-info-value",
          "{value}"
        }
      }
    }
  });

  rsx! {
    document::Link { rel: "stylesheet", href: CSS },

    div {
      class: "drive-info-table",

      {rows}
    }
  }
}
