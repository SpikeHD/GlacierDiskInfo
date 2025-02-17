use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonsProps {}

#[component]
pub fn Buttons(props: ButtonsProps) -> Element {
  rsx! {
    div {
      class: "bench-buttons",

      Button {
        label: "All",
        on_click: move |_| {},
      }

      Button {
        label: "SEQ1M",
        on_click: move |_| {},
      }

      Button {
        label: "SEQ128K",
        on_click: move |_| {},
      }

      Button {
        label: "RAND4K",
        on_click: move |_| {},
      },
    }
  }
}

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
  pub label: String,
  pub on_click: EventHandler,
}

#[component]
fn Button(props: ButtonProps) -> Element {
  rsx! {
    div {
      class: "bench-button",
      onclick: move |_| props.on_click.call(()),

      span {
        "{props.label}"
      }
    }
  }
}
