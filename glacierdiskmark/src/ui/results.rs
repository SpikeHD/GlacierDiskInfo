use dioxus::prelude::*;

// #[derive(Props, PartialEq, Clone)]
// pub struct ResultsProps {}

#[component]
pub fn Results(/* props: ResultsProps */) -> Element {
  rsx! {
    div {
      class: "bench-results",

      // TODO configuration row
      div {
        class: "bench-result-row",
      }

      // SEQ1M
      Result {
        read: "0.00 MB/s",
        write: "0.00 MB/s",
      }

      // SEQ128K
      Result {
        read: "0.00 MB/s",
        write: "0.00 MB/s",
      }

      // RAND4K
      Result {
        read: "0.00 MB/s",
        write: "0.00 MB/s",
      }
    }
  }
}

#[derive(Props, PartialEq, Clone)]
pub struct ResultRowProps {
  pub read: String,
  pub write: String,
}

#[component]
fn Result(props: ResultRowProps) -> Element {
  rsx! {
    div {
      class: "bench-result-row",

      span {
        class: "bench-result-value",
        "{props.read}"
      }

      span {
        class: "bench-result-value",
        "{props.read}"
      }
    }
  }
}
