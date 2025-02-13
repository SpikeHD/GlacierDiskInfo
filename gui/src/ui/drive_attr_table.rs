use std::path::PathBuf;

use dioxus::prelude::*;
use libglacierdisk::attribute;

static CSS: Asset = asset!("/assets/driveattrtable.css");
static GOOD: Asset = asset!("/assets/img/good.ico");
static CAUTION: Asset = asset!("/assets/img/caution.ico");

#[derive(Props, PartialEq, Clone)]
pub struct DriveAttrTableProps {
  pub selected_drive: String,
}

#[component]
pub fn DriveAttrTable(props: DriveAttrTableProps) -> Element {
  let mut drive = libglacierdisk::get_disk_info(&PathBuf::from(props.selected_drive.clone()))
    .expect("Failed to get disk info");
  let attrs = attribute::get_all_attributes(&mut drive);
  let rows = attrs.iter().map(|attr| {
    rsx! {
      div {
        class: "drive-attr-row",
        // Status
        span {
          class: "drive-attr-status",

          img {
            src: match attr.warn {
              true => CAUTION,
              false => GOOD,
            },
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
    document::Link { rel: "stylesheet", href: CSS },

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
