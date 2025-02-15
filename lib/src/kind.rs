use std::{fmt::Display, path::{Path, PathBuf}};

/// A struct representing the kind of disk (SSD, HDD, etc.)
#[derive(Clone, PartialEq, Debug)]
pub enum DiskKind {
  SSD,
  HDD,
  NVME,
  USB,
}

impl Display for DiskKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DiskKind::SSD => write!(f, "SSD"),
      DiskKind::HDD => write!(f, "HDD"),
      DiskKind::NVME => write!(f, "NVME"),
      DiskKind::USB => write!(f, "USB"),
    }
  }
}

/// Get the kind of disk from a path or [`super::Disk`]
pub fn disk_class(disk: impl AsRef<Path>) -> DiskKind {
  let disk = disk.as_ref();
  let drive = disk
    .file_name()
    .unwrap_or_default()
    .to_str()
    .unwrap_or_default();

  // Read disk type
  if drive.starts_with("nvme") {
    return DiskKind::NVME;
  }

  // Check first if it's a USB disk
  let link = PathBuf::from(format!("/sys/block/{drive}/"));
  if link.exists() {
    let link = std::fs::read_link(link).unwrap_or_default();
    if link.starts_with("/usb") {
      return DiskKind::USB;
    }
  }

  let sys_class = PathBuf::from(format!("/sys/class/block/{drive}/queue/rotational"));

  // 1 = HDD, 0 = SSD
  if sys_class.exists() {
    let rotational = std::fs::read_to_string(sys_class).unwrap_or_default();
    let rotational = rotational.trim();
    if rotational == "1" {
      return DiskKind::HDD;
    }
  }

  DiskKind::SSD
}
