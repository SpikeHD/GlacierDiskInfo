use std::{error::Error, fmt::Display, fs, os::unix::fs::FileTypeExt, path::{Path, PathBuf}};

use crate::{ata::DiskAtaLink, attribute::{get_all_attributes, get_attribute, Attribute}, kind::{disk_class, DiskKind}};

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

impl AsRef<Path> for Disk {
  fn as_ref(&self) -> &Path {
    self.path()
  }
}

impl PartialEq for Disk {
  fn eq(&self, other: &Self) -> bool {
    self.disk.disk == other.disk.disk
  }
}

impl Display for Disk {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.path().display())
  }
}

impl Disk {
  /// Create a new Disk from the path (e.g. `"/dev/sda"`) 
  pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
    let disk = libatasmart::Disk::new(&path)?;
    let kind = disk_class(&path);
    let ata_link = DiskAtaLink::for_disk(&path).unwrap_or_default();

    Ok(Self { kind, ata_link, disk })
  }

  /// Get the path of the disk
  pub fn path(&self) -> &PathBuf {
    &self.disk.disk
  }

  /// Get a SMART attribute from the disk
  pub fn get_attribute(&mut self, name: impl AsRef<str>) -> Option<Attribute> {
    get_attribute(&mut self.disk, name)
  }

  /// Get all SMART attributes from the disk
  pub fn get_all_attributes(&mut self) -> Vec<Attribute> {
    get_all_attributes(&mut self.disk)
  }

  /// Dump all SMART attributes to stdout
  pub fn dump_attributes(&mut self) {
    self.disk.dump().unwrap_or_default();
  }

  /// Get a reference to the raw [`libatasmart::Disk`] struct
  pub fn raw_disk(&mut self) -> &mut libatasmart::Disk {
    &mut self.disk
  }
}

/// Get a list of all disk paths on the system
pub fn get_disk_paths() -> Result<Vec<String>, Box<dyn Error>> {
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
