use std::path::{Path, PathBuf};

#[derive(Clone, Default, Debug)]
pub struct DiskStat {
  pub read_io: u64,
  pub write_io: u64,
  pub discard_io: u64,
  pub read_merge: u64,
  pub write_merge: u64,
  pub discard_merge: u64,
  pub read_sectors: u64,
  pub write_sectors: u64,
  pub discard_sectors: u64,
  pub read_ticks: u64,
  pub write_ticks: u64,
  pub discard_ticks: u64,
  pub in_flight: u64,
  pub io_ticks: u64,
  pub time_in_queue: u64,
}

impl DiskStat {
  pub fn from_disk(disk: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    let disk = disk
      .file_name()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default();
    let path = PathBuf::from(format!("/sys/block/{disk}/stat"));
    let mut stat = Self::default();
    let contents = std::fs::read_to_string(path)?;
    let values = contents.split_whitespace().collect::<Vec<&str>>();

    for (i, value) in values.into_iter().enumerate() {
      match i {
        0 => {
          stat.read_io = value.parse::<u64>().unwrap_or_default();
        }
        1 => {
          stat.read_merge = value.parse::<u64>().unwrap_or_default();
        }
        2 => {
          stat.read_sectors = value.parse::<u64>().unwrap_or_default();
        }
        3 => {
          stat.read_ticks = value.parse::<u64>().unwrap_or_default();
        }
        4 => {
          stat.write_io = value.parse::<u64>().unwrap_or_default();
        }
        5 => {
          stat.write_merge = value.parse::<u64>().unwrap_or_default();
        }
        6 => {
          stat.write_sectors = value.parse::<u64>().unwrap_or_default();
        }
        7 => {
          stat.write_ticks = value.parse::<u64>().unwrap_or_default();
        }
        8 => {
          stat.in_flight = value.parse::<u64>().unwrap_or_default();
        }
        9 => {
          stat.io_ticks = value.parse::<u64>().unwrap_or_default();
        }
        10 => {
          stat.time_in_queue = value.parse::<u64>().unwrap_or_default();
        }
        11 => {
          stat.discard_io = value.parse::<u64>().unwrap_or_default();
        }
        12 => {
          stat.discard_merge = value.parse::<u64>().unwrap_or_default();
        }
        13 => {
          stat.discard_sectors = value.parse::<u64>().unwrap_or_default();
        }
        14 => {
          stat.discard_ticks = value.parse::<u64>().unwrap_or_default();
        }
        _ => {}
      };
    }

    Ok(stat)
  }
}

pub fn sector_size(disk: &Path) -> u64 {
  let drive = disk
    .file_name()
    .unwrap_or_default()
    .to_str()
    .unwrap_or_default();
  let path = PathBuf::from(format!("/sys/block/{drive}/queue/hw_sector_size"));
  let size = std::fs::read_to_string(path).unwrap_or_default();
  let size = size.trim();

  size.parse::<u64>().unwrap_or_default()
}
