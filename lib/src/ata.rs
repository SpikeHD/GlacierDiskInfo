use std::{fs, path::Path};

#[derive(Clone, PartialEq, Debug)]
pub struct DiskAtaLink {
  pub port: u8,
  pub speed: String,
}

impl Default for DiskAtaLink {
  fn default() -> Self {
    Self {
      port: 0,
      speed: "unknown".to_string(),
    }
  }
}

impl DiskAtaLink {
  /// Get the link details for a disk
  pub fn for_disk(disk: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
    let disk = disk.as_ref();
    let disk = disk
      .file_name()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default();
    // Read links in /dev/disk/by-path, find the one that matches the disk
    let links = std::fs::read_dir("/dev/disk/by-path")?;

    for link in links {
      let link = link?;
      let link_path = link.path();
      let link = fs::read_link(&link_path)
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_string();

      // The link should be something like ../../sdX
      if link.ends_with(format!("/{disk}").as_str()) {
        // Get the port from the links filename
        let port = link_path
          .file_name()
          .unwrap_or_default()
          .to_str()
          .unwrap_or_default();

        if !port.contains("ata") {
          continue;
        }

        let port = port.split("ata-").collect::<Vec<&str>>()[1];

        // No parts, no extras, just the base links
        if port.contains(".") || port.contains("part") {
          continue;
        }

        let port = port.parse::<u8>().unwrap_or_default();

        // Now we get the link via the port
        let ata_spd = fs::read_to_string(format!("/sys/class/ata_link/link{port}/sata_spd"))?;

        return Ok(DiskAtaLink {
          port,
          speed: ata_spd,
        });
      }
    }

    Err("No link found".into())
  }
}
