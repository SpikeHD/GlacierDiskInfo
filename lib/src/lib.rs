use std::{error::Error, path::{Path, PathBuf}};

use disk::get_disks;

pub mod ata;
pub mod attribute;
mod disk;
pub mod kind;
pub mod sysfs;

// Re-export libatasmart
pub use libatasmart;
// Re-export libatasmart-sys
pub use libatasmart_sys;

#[cfg(target_os = "linux",)]
static DEV_PATH: &str = "/dev";

pub fn get_disks_info() -> Result<Vec<libatasmart::Disk>, Box<dyn Error>> {
  let mut list = vec![];
  // Get disks
  let disks = get_disks()?;

  for disk in disks {
    let path = PathBuf::from(format!("{}/{}", DEV_PATH, disk));
    let disk = libatasmart::Disk::new(&path)?;

    list.push(disk);
  }

  Ok(list)
}

pub fn get_disk_info(disk: &Path) -> Result<libatasmart::Disk, Box<dyn Error>> {
  let disk = libatasmart::Disk::new(disk)?;
  Ok(disk)
}

pub fn list_disks() -> Result<Vec<PathBuf>, Box<dyn Error>> {
  let mut list = vec![];
  let disks = get_disks()?;

  for disk in disks {
    // TODO other platforms
    list.push(PathBuf::from(format!("{}/{}", DEV_PATH, disk)));
  }

  Ok(list)
}
