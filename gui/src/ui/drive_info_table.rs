use std::path::PathBuf;

use dioxus::prelude::*;
use libglacierdisk::{ata::DiskAtaLink, sysfs::{sector_size, DiskStat}};

use crate::util::conversion::{bytes_to_readable, ms_to_readable};

static CSS: Asset = asset!("/assets/driveinfotable.css");

#[derive(Props, PartialEq, Clone)]
pub struct DriveInfoTableProps {
  pub selected_drive: String,
}

#[component]
pub fn DriveInfoTable(props: DriveInfoTableProps) -> Element {
  let disk_path = PathBuf::from(props.selected_drive.clone());
  let mut drive = libglacierdisk::get_disk_info(&disk_path)
    .expect("Failed to get disk info");
  let identity = drive.identify_parse().expect("Failed to get identify info");
  let stats = DiskStat::from_disk(&disk_path).unwrap_or_default();
  let sector_size = sector_size(&disk_path);
  let ata = match DiskAtaLink::for_disk(&disk_path) {
    Ok(ata) => ata,
    Err(_) => DiskAtaLink::default(),
  };
  let left_values = vec![
    ("Firmware", identity.firmware),
    ("Serial", identity.serial),
    ("Model", identity.model),
    ("Drive Path", props.selected_drive.clone()),
    ("SATA Speed", ata.speed),
  ];
  let right_values = vec![
    (
      "Total Read",
      bytes_to_readable(stats.read_sectors * sector_size),
    ),
    (
      "Total Write",
      bytes_to_readable(stats.write_sectors * sector_size),
    ),
    (
      "Powered On",
      ms_to_readable(drive.get_power_on().unwrap_or(0)),
    ),
    (
      "Power On Count",
      drive.get_power_cycle_count().unwrap_or(0).to_string(),
    ),
    (
      "Average Power On Time",
      ms_to_readable(drive.get_power_on().unwrap_or(0) / drive.get_power_cycle_count().unwrap_or(0)),
    ),
  ];
  let left_rows = left_values.iter().map(|(name, value)| {
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
  let right_rows = right_values.iter().map(|(name, value)| {
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

      div {
        class: "drive-info-table-section left",
        {left_rows}
      },

      div {
        class: "drive-info-table-section right",
        {right_rows}
      }
    }
  }
}
