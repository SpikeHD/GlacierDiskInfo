use std::path::PathBuf;

use dioxus::prelude::*;
use libglacierdisk::{attribute, disk::Disk};

use crate::assets::{ico_to_data_uri, CAUTION_ICO, GOOD_ICO};

#[derive(Props, PartialEq, Clone)]
pub struct DriveAttrTableProps {
  pub selected_drive: Disk,
}

#[component]
pub fn DriveAttrTable(mut props: DriveAttrTableProps) -> Element {
  let attrs = props.selected_drive.get_all_attributes();
  let rows = attrs.iter().map(|attr| {
    let ico = if attr.warn { CAUTION_ICO } else { GOOD_ICO }; 

    rsx! {
      div {
        class: "drive-attr-row",
        // Status
        span {
          class: "drive-attr-status",

          img {
            src: ico_to_data_uri(ico),
          }
        },
        span {
          class: "drive-attr-id",
          "{attr.id}"
        },
        span {
          class: "drive-attr-name",
          "{attr.name}"
        },
        span {
          class: "drive-attr-current",
          "{attr.current}"
        },
        span {
          class: "drive-attr-worst",
          "{attr.worst}"
        },
        span {
          class: "drive-attr-threshold",
          "{attr.threshold}"
        },
        span {
          class: "drive-attr-raw",
          "{attribute::raw_to_string(attr.raw)}"
        }
      }
    }
  });

  rsx! {
    div {
      class: "drive-attr-table",

      div {
        class: "drive-attr-table-header",
        span {
          class: "drive-attr-status",
          ""
        },
        span {
          class: "drive-attr-id",
          "ID"
        },
        span {
          class: "drive-attr-name",
          "Name"
        },
        span {
          class: "drive-attr-current",
          "Current"
        },
        span {
          class: "drive-attr-worst",
          "Worst"
        },
        span {
          class: "drive-attr-threshold",
          "Threshold"
        },
        span {
          class: "drive-attr-raw",
          "Raw"
        },
      },

      div {
        class: "drive-attr-table-body",
        {rows}
      }
    }
  }
}
