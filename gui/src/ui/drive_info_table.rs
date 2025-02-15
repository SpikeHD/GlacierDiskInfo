use std::path::PathBuf;

use dioxus::prelude::*;
use libglacierdisk::{
  ata::DiskAtaLink, attribute::{get_attribute, Convertable}, disk::Disk, kind::disk_class
};

use crate::util::conversion::{bytes_to_readable, ms_to_readable};

#[derive(Props, PartialEq, Clone)]
pub struct DriveInfoTableProps {
  pub selected_drive: Disk,
}

#[component]
pub fn DriveInfoTable(mut props: DriveInfoTableProps) -> Element {
  let ata = props.selected_drive.ata_link.clone();
  let mut drive = props.selected_drive.raw_disk().clone();
  let identity = drive.identify_parse().expect("Failed to get identify info");
  let lbas_read = get_attribute(&mut drive, "total-lbas-read").unwrap_or_default();
  let lbas_written = get_attribute( &mut drive, "total-lbas-written").unwrap_or_default();

  let left_values = [
    ("Firmware", identity.firmware),
    ("Serial", identity.serial),
    ("Model", identity.model),
    ("Drive Path", props.selected_drive.path().to_string_lossy().to_string()),
    ("SATA Speed", ata.speed),
    ("Kind", props.selected_drive.kind.to_string())
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
