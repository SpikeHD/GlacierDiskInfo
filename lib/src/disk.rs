use std::{
  error::Error,
  fmt::{Debug, Display},
  fs,
  os::unix::fs::FileTypeExt,
  path::{Path, PathBuf},
  rc::Rc,
  sync::{Mutex, MutexGuard},
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
  disk: Option<Rc<Mutex<libatasmart::Disk>>>,
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

impl ShallowDisk {
  /// Create a new Disk from the path (e.g. `"/dev/sda"`)
  pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
    let kind = disk_class(&path);
    let ata_link = DiskAtaLink::for_disk(&path).unwrap_or_default();

    Ok(Self {
      path,
      kind,
      ata_link,
    })
  }
}

impl Disk {
  /// Create a new Disk from the path (e.g. `"/dev/sda"`)
  pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
    // Special case for USB devices
    if disk_class(&path) == DiskKind::USB {
      return Ok(Self {
        path,
        kind: DiskKind::USB,
        ata_link: DiskAtaLink::default(),
        disk: None,
      });
    }

    let disk = libatasmart::Disk::new(&path)?;
    let kind = disk_class(&path);
    let ata_link = DiskAtaLink::for_disk(&path).unwrap_or_default();

    Ok(Self {
      path,
      kind,
      ata_link,
      disk: Some(Rc::new(Mutex::new(disk))),
    })
  }

  /// Get a SMART attribute from the disk
  pub fn get_attribute(&mut self, name: impl AsRef<str>) -> Option<Attribute> {
    if let Some(mut disk) = self.raw_disk() {
      get_attribute(&mut disk, name)
    } else {
      None
    }
  }

  /// Get all SMART attributes from the disk
  pub fn get_all_attributes(&mut self) -> Vec<Attribute> {
    if let Some(mut disk) = self.raw_disk() {
      get_all_attributes(&mut disk)
    } else {
      vec![]
    }
  }

  /// Dump all SMART attributes to stdout
  pub fn dump_attributes(&mut self) {
    if let Some(mut disk) = self.raw_disk() {
      disk.dump().unwrap_or_default();
    }
  }

  /// Get a reference to the raw [`libatasmart::Disk`] struct
  pub fn raw_disk(&self) -> Option<MutexGuard<libatasmart::Disk>> {
    self.disk.as_ref().map(|disk| disk.lock().unwrap())
  }

  /// Get the mount locations of the disk. A disk may have multiple if there are multiple partitions, or a disk may have none if it is not mounted.
  pub fn mounts(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    get_mounts(&self)
  }

  /// Read the `model` via sysfs
  pub fn model(&self) -> Result<String, Box<dyn Error>> {
    let model = fs::read_to_string(format!("/sys/block/{}/device/model", self.path.file_name().unwrap_or_default().to_str().unwrap_or_default()))?;
    Ok(model)
  }

  /// Read the `vendor` via sysfs
  pub fn vendor(&self) -> Result<String, Box<dyn Error>> {
    let vendor = fs::read_to_string(format!("/sys/block/{}/device/vendor", self.path.file_name().unwrap_or_default().to_str().unwrap_or_default()))?;
    Ok(vendor)
  }

  /// Read size from either SMART or sysfs
  pub fn size(&self) -> Result<u64, Box<dyn Error>> {
    let size = self.raw_disk().map(|mut disk| disk.get_disk_size().unwrap_or(0)).unwrap_or(0);

    // If size is zero, try to read from sysfs
    if size == 0 {
      let size = fs::read_to_string(format!("/sys/block/{}/size", self.path.file_name().unwrap_or_default().to_str().unwrap_or_default()))?;
      // 512 is standard block size
      let size = size.trim().parse::<u64>().unwrap_or(0) * 512;
      return Ok(size);
    }

    Ok(size)
  }
}

impl ShallowDisk {
  /// Get the mount locations of the disk. A disk may have multiple if there are multiple partitions, or a disk may have none if it is not mounted.
  pub fn mounts(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    get_mounts(&self)
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

fn get_mounts(path: &impl AsRef<Path>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
  let path = path.as_ref().to_string_lossy().to_string();
  let mounts = fs::read_to_string("/proc/mounts")?;
  let mounts = mounts.split("\n");

  let mounts = mounts
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
    .collect();

  Ok(mounts)
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
