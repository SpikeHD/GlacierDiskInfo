use std::path::PathBuf;

use dioxus::prelude::*;
use libglacierdisk::{
  ata::DiskAtaLink,
  attribute::{get_attribute, Convertable}, kind::disk_class,
};

use crate::util::conversion::{bytes_to_readable, ms_to_readable};

#[derive(Props, PartialEq, Clone)]
pub struct DriveInfoTableProps {
  pub selected_drive: String,
}

#[component]
pub fn DriveInfoTable(props: DriveInfoTableProps) -> Element {
  let disk_path = PathBuf::from(props.selected_drive.clone());
  let mut drive = libglacierdisk::get_disk_info(&disk_path).expect("Failed to get disk info");
  let identity = drive.identify_parse().expect("Failed to get identify info");
  let ata = DiskAtaLink::for_disk(&disk_path).unwrap_or_default();
  let lbas_read = get_attribute("total-lbas-read", &mut drive).unwrap_or_default();
  let lbas_written = get_attribute("total-lbas-written", &mut drive).unwrap_or_default();

  let left_values = [
    ("Firmware", identity.firmware),
    ("Serial", identity.serial),
    ("Model", identity.model),
    ("Drive Path", props.selected_drive.clone()),
    ("SATA Speed", ata.speed),
    ("Kind", disk_class(&disk_path).to_string())
  ];
  let right_values = [
    (
      "Total Read",
      bytes_to_readable(
        lbas_read
          .pretty_unit
          .convert_to_base(lbas_read.pretty_value),
      ),
    ),
    (
      "Total Write",
      bytes_to_readable(
        lbas_written
          .pretty_unit
          .convert_to_base(lbas_written.pretty_value),
      ),
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
      ms_to_readable(
        drive.get_power_on().unwrap_or(0) / drive.get_power_cycle_count().unwrap_or(0),
      ),
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
