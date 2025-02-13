use dioxus::prelude::*;

use crate::{assets::{ico_to_data_uri, BAD_ICO, CAUTION_ICO, GOOD_ICO}, data::{smart::DriveStatus, status::Status}};

#[derive(Props, PartialEq, Clone)]
pub struct DriveTabsProps {
  pub drives: Vec<(String, Status)>,
  pub selected_drive: String,
  pub on_select: EventHandler<String>,
}

#[component]
pub fn DriveTabs(props: DriveTabsProps) -> Element {
  let tab_renders = props.drives.iter().map(|(name, status)| {
    let evt_name = name.clone();
    let temp = if status.temp == 0. { "--".into() } else { status.temp.to_string() };
    let status_class = match DriveStatus::from_smart(status.state.as_str()) {
      DriveStatus::Good => "good",
      DriveStatus::Caution => "caution",
      DriveStatus::Bad => "bad",
    };
    let ico = match status.state.as_str() {
      "Good" => GOOD_ICO,
      "Bad Attribute In The Past" => CAUTION_ICO,
      "Bad Sector" => BAD_ICO,
      "Bad Attribute Now" => CAUTION_ICO,
      "Bad Sector Many" => BAD_ICO,
      "Bad Status" => BAD_ICO,
      _ => BAD_ICO,
    };

    rsx! {
      div {
        class: "drive-tab ".to_owned() + status_class + " " + (if name == &props.selected_drive { "selected" } else { "" }),
        onclick: move |_| props.on_select.call(evt_name.clone()),

        div {
          class: "drive-tab-status",

          img {
            class: "drive-tab-icon",
            src: ico_to_data_uri(ico),
          }
        }
        div {
          class: "drive-tab-info",

          span {
            class: "drive-tab-state",
            "{status.state}"
          }

          span {
            class: "drive-tab-temp",
            "{temp} °C"
          }

          span {
            class: "drive-tab-name",
            "{name}"
          }
        }
      }
    }
  });

  rsx! {
    div {
      id: "drive-tabs",

      {tab_renders}
    }
  }
}
