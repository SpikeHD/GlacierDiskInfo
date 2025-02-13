use std::path::{Path, PathBuf};

pub enum DiskKind {
  SSD,
  HDD,
  NVME,
  USB,
}

impl ToString for DiskKind {
  fn to_string(&self) -> String {
    match self {
      Self::SSD => "SSD",
      Self::HDD => "HDD",
      Self::NVME => "NVMe",
      Self::USB => "USB",
    }.to_string()
  }
}

pub fn disk_class(disk: &Path) -> DiskKind {
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
