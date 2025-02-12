use std::path::PathBuf;

use dioxus::prelude::*;

use crate::data::attributes;

static CSS: Asset = asset!("/assets/driveattrtable.css");

#[derive(Props, PartialEq, Clone)]
pub struct DriveAttrTableProps {
  pub selected_drive: String,
}

#[component]
pub fn DriveAttrTable(props: DriveAttrTableProps) -> Element {
  let mut drive = libminidisk::get_disk_info(PathBuf::from(props.selected_drive.clone())).expect("Failed to get disk info");
  let attrs = attributes::get_all_attributes(&mut drive);
  let rows = attrs.iter().map(|attr| {
    rsx! {
      tr {
        // Status
        td { "{attr.name}" },
        td { "{attr.id}" },
        td { "{attr.name}" },
        td { "{attr.current}" },
        td { "{attr.worst}" },
        td { "{attr.threshold}" },
        td { "{attributes::raw_to_string(attr.raw)}" },
      }
    }
  });

  rsx! {
    document::Link { rel: "stylesheet", href: CSS },

    table {
      class: "drive-attr-table",

      thead {
        tr {
          // Status
          th { "" },
          th { "ID" },
          th { "Attribute Name" },
          th { "Current" },
          th { "Worst" },
          th { "Threshold" },
          th { "Raw" },
        }
      },

      tbody {
        {rows}
      }
    }
  }
}