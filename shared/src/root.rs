use std::{env, process::Command};

use dialog::{DialogBox, Message};

pub fn pk_reopen() {
  if !is_pkexec_available() {
    Message::new("GlacierDiskInfo must be run as root, and pkexec was not found. Please run as root or install pkexec.")
      .title("Error")
      .show()
      .expect("Failed to show dialog");

    std::process::exit(1);
  }

  let args: Vec<String> = env::args().skip(1).collect();
  let mut status = Command::new("pkexec");
  status
    .arg("env")
    .arg(format!(
      "DISPLAY={}",
      env::var("DISPLAY").unwrap_or_default()
    ))
    .arg(format!(
      "XAUTHORITY={}",
      env::var("XAUTHORITY").unwrap_or_default()
    ))
    .arg(env::current_exe().unwrap())
    .args(args);

  println!("running: {:?}", status);

  let status = status.status().expect("Failed to run pkexec");

  std::process::exit(status.code().unwrap_or(1));
}

pub fn is_pkexec_available() -> bool {
  let status = Command::new("pkexec").arg("--version").status();

  match status {
    Ok(status) => status.success(),
    Err(_) => false,
  }
}
