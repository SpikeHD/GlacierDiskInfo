use dioxus::prelude::*;

use crate::data::status::Status;

static CSS: Asset = asset!("/assets/drivetabs.css");
static GOOD: Asset = asset!("/assets/img/good.ico");
static CAUTION: Asset = asset!("/assets/img/caution.ico");
static BAD: Asset = asset!("/assets/img/bad.ico");

#[derive(Props, PartialEq, Clone)]
pub struct DriveTabsProps {
  pub drives: Vec<(String, Status)>,
}

#[component]
pub fn DriveTabs(props: DriveTabsProps) -> Element {
  println!("Rendering with drives: {:?}", props.drives);
  let tab_renders = props.drives.iter().map(|(name, status)| {
    rsx! {
      div {
        class: "drive-tab",
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
            "{status.temp} °C"
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
