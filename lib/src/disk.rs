use std::{error::Error, fs, os::unix::fs::FileTypeExt, path::PathBuf};

use crate::{ata::DiskAtaLink, attribute::{dump_attributes, get_all_attributes, get_attribute, Attribute}, kind::{disk_class, DiskKind}};

// TODO other platforms (eg. FreeBSD)
#[cfg(target_os = "linux")]
static FILTER: [&str; 3] = ["sd", "hd", "nvme"];

static NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

#[derive(Clone, Debug)]
pub struct Disk {
  pub kind: DiskKind,
  pub ata_link: DiskAtaLink,
  disk: libatasmart::Disk,
}

impl PartialEq for Disk {
  fn eq(&self, other: &Self) -> bool {
    self.disk.disk == other.disk.disk
  }
}

impl ToString for Disk {
  fn to_string(&self) -> String {
    format!("{:?}", self.path())
  }
}

impl Disk {
  pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
    let disk = libatasmart::Disk::new(&path)?;
    let kind = disk_class(&path);
    let ata_link = DiskAtaLink::for_disk(&path).unwrap_or_default();

    Ok(Self { kind, ata_link, disk })
  }

  pub fn path(&self) -> &PathBuf {
    &self.disk.disk
  }

  pub fn get_attribute(&mut self, name: impl AsRef<str>) -> Option<Attribute> {
    get_attribute(&mut self.disk, name)
  }

  pub fn get_all_attributes(&mut self) -> Vec<Attribute> {
    get_all_attributes(&mut self.disk)
  }

  pub fn dump_attributes(&mut self) {
    dump_attributes(&mut self.disk);
  }

  pub fn raw_disk(&mut self) -> &mut libatasmart::Disk {
    &mut self.disk
  }
}

pub fn get_disks() -> Result<Vec<String>, Box<dyn Error>> {
  let mut disks = vec![];

  for entry in fs::read_dir("/dev")? {
    let path = entry?.path();
    let meta = path.metadata()?;
    let filename = path
      .file_name()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default();

    if meta.file_type().is_block_device() && fits_filter(filename) {
      disks.push(filename.to_string());
    }
  }

  Ok(disks)
}

fn fits_filter(disk: &str) -> bool {
  for filter in FILTER.iter() {
    if disk.starts_with(filter) {
      for number in NUMBERS.iter() {
        if disk.contains(number) {
          return false;
        }
      }

      return true;
    }
  }

  false
}
