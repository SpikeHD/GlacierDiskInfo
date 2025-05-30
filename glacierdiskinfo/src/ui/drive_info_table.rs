use dioxus::prelude::*;
use shared::convert::{bytes_to_readable, ms_to_readable};

use crate::data::disk_cache::DiskCache;

#[derive(PartialEq, Clone)]
enum SerialState {
  Hidden,
  Shown,
  AlwaysShown,
}

#[derive(Props, PartialEq, Clone)]
pub struct DriveInfoTableProps {
  pub selected_drive: DiskCache,
}

#[component]
pub fn DriveInfoTable(props: DriveInfoTableProps) -> Element {
  let mut serial_state = use_signal(|| SerialState::Hidden);

  let drive = props.selected_drive;
  let ata = drive.ata_link();
  let identity = drive.identity();
  let lbas_read = drive.total_read();
  let lbas_written = drive.total_write();

  let left_values = [
    ("Firmware", identity.firmware),
    ("Serial", identity.serial),
    ("Model", identity.model),
    ("Drive Path", drive.path().to_string_lossy().to_string()),
    ("SATA Speed", ata.speed.clone()),
    ("Kind", drive.kind().to_string()),
  ];
  let right_values = [
    ("Total Read", bytes_to_readable(lbas_read)),
    ("Total Write", bytes_to_readable(lbas_written)),
    ("Powered On", ms_to_readable(drive.power_on())),
    ("Power On Count", drive.power_cycle_count().to_string()),
    (
      "Average Power On Time",
      ms_to_readable(if drive.power_cycle_count() > 0 { drive.power_on() / drive.power_cycle_count() } else { 0 }),
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

        if *name == "Serial" {
          span {
            onmouseenter: move |_| if serial_state() != SerialState::AlwaysShown { serial_state.set(SerialState::Shown) },
            onmouseleave: move |_| if serial_state() != SerialState::AlwaysShown { serial_state.set(SerialState::Hidden) },
            onmousedown: move |_| serial_state.set(SerialState::AlwaysShown),
            class: "drive-info-value",
            
            if serial_state() == SerialState::Shown || serial_state() == SerialState::AlwaysShown {
              "{value}"
            } else {
              "•••••••••••••••••••••••"
            }
          }
        } else {
          span {
            class: "drive-info-value",
            "{value}"
          }
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
