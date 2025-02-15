use std::{
  error::Error,
  fmt::{Debug, Display},
  fs,
  os::unix::fs::FileTypeExt,
  path::{Path, PathBuf},
  sync::{Arc, Mutex, MutexGuard},
};

use crate::{
  ata::DiskAtaLink,
  attribute::{get_all_attributes, get_attribute, Attribute},
  kind::{disk_class, DiskKind},
};

// TODO other platforms (eg. FreeBSD)
#[cfg(target_os = "linux")]
static FILTER: [&str; 3] = ["sd", "hd", "nvme"];

static NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

#[derive(Clone)]
pub struct Disk {
  pub path: PathBuf,
  pub kind: DiskKind,
  pub ata_link: DiskAtaLink,
  disk: Arc<Mutex<libatasmart::Disk>>,
}

#[derive(Clone, Debug)]
pub struct ShallowDisk {
  pub path: PathBuf,
  pub kind: DiskKind,
  pub ata_link: DiskAtaLink,
}

impl Debug for Disk {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // Write struct without the disk field
    let mut s = f.debug_struct("Disk");
    s.field("kind", &self.kind);
    s.field("ata_link", &self.ata_link);
    s.finish()
  }
}

impl AsRef<Path> for Disk {
  fn as_ref(&self) -> &Path {
    &self.path
  }
}

impl PartialEq for Disk {
  fn eq(&self, other: &Self) -> bool {
    self.path == other.path
  }
}

impl Display for Disk {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.path.display())
  }
}

impl Disk {
  /// Create a new Disk from the path (e.g. `"/dev/sda"`)
  pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
    let disk = libatasmart::Disk::new(&path)?;
    let kind = disk_class(&path);
    let ata_link = DiskAtaLink::for_disk(&path).unwrap_or_default();

    Ok(Self {
      path,
      kind,
      ata_link,
      disk: Arc::new(Mutex::new(disk)),
    })
  }

  /// Get a SMART attribute from the disk
  pub fn get_attribute(&mut self, name: impl AsRef<str>) -> Option<Attribute> {
    get_attribute(&mut self.raw_disk(), name)
  }

  /// Get all SMART attributes from the disk
  pub fn get_all_attributes(&mut self) -> Vec<Attribute> {
    get_all_attributes(&mut self.raw_disk())
  }

  /// Dump all SMART attributes to stdout
  pub fn dump_attributes(&mut self) {
    self.raw_disk().dump().unwrap_or_default();
  }

  /// Get a reference to the raw [`libatasmart::Disk`] struct
  pub fn raw_disk(&self) -> MutexGuard<libatasmart::Disk> {
    self.disk.lock().unwrap()
  }

  /// Get the mount locations of the disk. A disk may have multiple if there are multiple partitions, or a disk may have none if it is not mounted.
  pub fn mounts(&self) -> Vec<PathBuf> {
    let path = self.path.to_string_lossy().to_string();
    let mounts = fs::read_to_string("/proc/mounts").unwrap_or_default();
    let mounts = mounts.split("\n");

    mounts
      .filter_map(|mount| {
        // We use starts_with as it's possible for multiple mounts to use the same disk, eg:
        // /dev/sda1
        // /dev/sda2
        if !mount.starts_with(&path) {
          return None;
        }

        let mount = mount.split_whitespace().collect::<Vec<&str>>();

        mount[1].parse::<PathBuf>().ok()
      })
      .collect()
  }
}

impl AsRef<Path> for ShallowDisk {
  fn as_ref(&self) -> &Path {
    &self.path
  }
}

impl PartialEq for ShallowDisk {
  fn eq(&self, other: &Self) -> bool {
    self.path == other.path
  }
}

impl Display for ShallowDisk {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.path.display())
  }
}

impl From<Disk> for ShallowDisk {
  fn from(disk: Disk) -> Self {
    Self {
      path: disk.path,
      kind: disk.kind,
      ata_link: disk.ata_link,
    }
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
