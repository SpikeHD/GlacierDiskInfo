use dioxus::prelude::*;

use crate::data::status::Status;

static CSS: Asset = asset!("/assets/drivetabs.css");
static GOOD: Asset = asset!("/assets/img/good.ico");
static CAUTION: Asset = asset!("/assets/img/caution.ico");
static BAD: Asset = asset!("/assets/img/bad.ico");

pub enum DriveStatus {
  Good,
  Caution,
  Bad,
}

impl DriveStatus {
  pub fn from_smart(s: impl AsRef<str>) -> Self {
    match s.as_ref() {
      "Good" => DriveStatus::Good,
      "Bad Attribute In The Past" => DriveStatus::Caution,
      "Bad Sector" => DriveStatus::Bad,
      "Bad Attribute Now" => DriveStatus::Caution,
      "Bad Sector Many" => DriveStatus::Bad,
      "Bad Status" => DriveStatus::Bad,
      _ => DriveStatus::Bad,
    }
  }
}

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

    rsx! {
      div {
        class: "drive-tab ".to_owned() + status_class + " " + (if name == &props.selected_drive { "selected" } else { "" }),
        onclick: move |_| props.on_select.call(evt_name.clone()),

        div {
          class: "drive-tab-status",
          
          img {
            class: "drive-tab-icon",
            src: match status.state.as_str() {
              "Good" => GOOD,
              "Bad Attribute In The Past" => CAUTION,
              "Bad Sector" => BAD,
              "Bad Attribute Now" => CAUTION,
              "Bad Sector Many" => BAD,
              "Bad Status" => BAD,
              _ => BAD,
            },
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
    document::Link { rel: "stylesheet", href: CSS }

    div {
      id: "drive-tabs",

      {tab_renders}
    }
  }
}
