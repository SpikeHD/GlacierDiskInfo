use std::path::PathBuf;

use dioxus::prelude::*;

use crate::util::conversion::bytes_to_readable;

static CSS: Asset = asset!("/assets/drive.css");

#[derive(Props, PartialEq, Clone)]
pub struct DriveProps {
  pub selected_drive: String,
}

#[component]
pub fn Drive(props: DriveProps) -> Element {
  let mut drive = libminidisk::get_disk_info(PathBuf::from(props.selected_drive.clone())).expect("Failed to get disk info");
  let identity = drive.identify_parse().expect("Failed to get identify info");
  let size = drive.get_disk_size().expect("Failed to get disk size");
  let size = bytes_to_readable(size);

  rsx! {
    document::Link { rel: "stylesheet", href: CSS },

    div {
      class: "drive",

      div {
        class: "drive-name",
        "{identity.model} {size} ({props.selected_drive})"
      }
    }
  }
}