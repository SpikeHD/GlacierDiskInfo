use std::{error::Error, path::PathBuf};

use disk::get_disks;

mod disk;

#[cfg(any(target_os = "linux"))]
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

pub fn get_disk_info(disk: PathBuf) -> Result<libatasmart::Disk, Box<dyn Error>> {
  let disk = libatasmart::Disk::new(&disk)?;
  Ok(disk)
}

pub fn list_disks() -> Result<Vec<String>, Box<dyn Error>> {
  let mut list = vec![];
  let disks = get_disks()?;

  for disk in disks {
    list.push(
      PathBuf::from(format!("{}/{}", DEV_PATH, disk))
        .to_str()
        .unwrap_or_default()
        .to_string(),
    );
  }

  Ok(list)
}
