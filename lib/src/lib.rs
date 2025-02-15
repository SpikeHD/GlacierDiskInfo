#![doc = include_str!("../README.md")]

use std::{error::Error, path::PathBuf};

use disk::{get_disk_paths, Disk};

pub mod ata;
pub mod attribute;
pub mod disk;
pub mod kind;
pub mod sysfs;

// Re-export libatasmart
pub use libatasmart;
pub use libatasmart_sys;

#[cfg(target_os = "linux")]
static DEV_PATH: &str = "/dev";

/// List all disks on the system
pub fn list_disks() -> Result<Vec<Disk>, Box<dyn Error>> {
  let mut list = vec![];
  let disks = get_disk_paths()?;

  for disk in disks {
    let d = Disk::new(PathBuf::from(format!("{}/{}", DEV_PATH, disk)));

    if let Ok(d) = d {
      list.push(d);
    }
  }

  Ok(list)
}
