use std::collections::HashMap;

use dioxus::prelude::*;
use libglacierdisk::benchmark::BenchmarkConfig;

use crate::util::bench::BenchKind;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonsProps {
  // Callback to set the BenchmarkConfigs we are to run
  pub run_configs: EventHandler<(BenchKind, Vec<BenchmarkConfig>)>,

  // Test size in bytes
  pub test_size: usize,
}

#[component]
pub fn Buttons(props: ButtonsProps) -> Element {
  rsx! {
    div {
      class: "bench-buttons",

      Button {
        label: "All",
        on_click: move |_| props.run_configs.call((BenchKind::All, get_configs(props.test_size).get("ALL").unwrap().to_vec())),
      }

      Button {
        label: "SEQ1M",
        on_click: move |_| props.run_configs.call((BenchKind::SEQ1M, get_configs(props.test_size).get("SEQ1M").unwrap().to_vec())),
      }

      Button {
        label: "SEQ128K",
        on_click: move |_| props.run_configs.call((BenchKind::SEQ128K, get_configs(props.test_size).get("SEQ128K").unwrap().to_vec())),
      }

      Button {
        label: "RAND4K",
        on_click: move |_| props.run_configs.call((BenchKind::RAND4K, get_configs(props.test_size).get("RAND4K").unwrap().to_vec())),
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

fn get_configs(test_size: usize) -> HashMap<String, Vec<BenchmarkConfig>> {
  let mut configs = HashMap::new();

  let seq1m = BenchmarkConfig {
    block_size: 1024 * 1024,
    block_count: test_size / 1024 / 1024,
    ..Default::default()
  };
  let seq128k = BenchmarkConfig {
    block_size: 1024 * 128,
    block_count: test_size / 1024 / 128,
    ..Default::default()
  };
  let rand4k = BenchmarkConfig {
    block_size: 1024 * 4,
    block_count: test_size / 1024 / 4,
    ..Default::default()
  };

  configs.insert(
    "ALL".into(),
    vec![seq1m.clone(), seq128k.clone(), rand4k.clone()],
  );
  configs.insert("SEQ1M".into(), vec![seq1m.clone()]);
  configs.insert("SEQ128K".into(), vec![seq128k.clone()]);
  configs.insert("RAND4K".into(), vec![rand4k.clone()]);

  configs
}
