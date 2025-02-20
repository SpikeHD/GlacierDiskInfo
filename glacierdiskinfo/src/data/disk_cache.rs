use std::path::PathBuf;

use libglacierdisk::{
  ata::DiskAtaLink,
  attribute::{Attribute, Convertable},
  disk::Disk,
  kind::DiskKind,
  libatasmart::IdentifyParsedData,
  libatasmart_sys::SkSmartOverall,
};

use super::smart::smart_to_string;

#[derive(Clone, Debug, PartialEq)]
pub struct DiskCache {
  disk: Disk,

  path: PathBuf,

  temperature: u64,
  size: u64,
  // identity: IdentifyParsedData
  attributes: Vec<Attribute>,
  ata_link: DiskAtaLink,

  // Raw disk stuff
  power_on: u64,
  power_cycle_count: u64,

  // Read/write
  total_read: u64,
  total_write: u64,

  kind: DiskKind,

  smart_overall: String,
}

impl DiskCache {
  pub fn new(mut disk: Disk) -> Self {
    let total_write = match disk.get_attribute("total-lbas-written") {
      Some(write) => write.pretty_unit.convert_to_base(write.pretty_value),
      None => 0,
    };
    let total_read = match disk.get_attribute("total-lbas-read") {
      Some(read) => read.pretty_unit.convert_to_base(read.pretty_value),
      None => 0,
    };

    let path = disk.path.clone();
    let temperature = disk.raw_disk().get_temperature().unwrap_or(0);
    let size = disk.raw_disk().get_disk_size().unwrap_or(0);
    let attributes = disk.get_all_attributes();
    let ata_link = disk.ata_link.clone();
    let power_on = disk.raw_disk().get_power_on().unwrap_or(0);
    let power_cycle_count = disk.raw_disk().get_power_cycle_count().unwrap_or(0);
    let kind = disk.kind.clone();
    let smart_overall = smart_to_string(
      disk
        .raw_disk()
        .smart_get_overall()
        .unwrap_or(SkSmartOverall::SK_SMART_OVERALL_GOOD),
    );

    Self {
      disk,
      path,
      temperature,
      size,
      attributes,
      ata_link,
      power_on,
      power_cycle_count,
      total_read,
      total_write,
      kind,
      smart_overall,
    }
  }

  pub fn path(&self) -> &PathBuf {
    &self.path
  }

  // TODO store in struct
  pub fn identity(&self) -> IdentifyParsedData {
    let identity = self
      .disk
      .raw_disk()
      .identify_parse()
      .unwrap_or(IdentifyParsedData {
        firmware: "N/A".into(),
        serial: "N/A".into(),
        model: "N/A".into(),
      });

    identity
  }

  pub fn temperature(&self) -> u64 {
    self.temperature
  }

  pub fn size(&self) -> u64 {
    self.size
  }

  pub fn attributes(&self) -> &Vec<Attribute> {
    &self.attributes
  }

  pub fn ata_link(&self) -> &DiskAtaLink {
    &self.ata_link
  }

  pub fn power_on(&self) -> u64 {
    self.power_on
  }

  pub fn power_cycle_count(&self) -> u64 {
    self.power_cycle_count
  }

  pub fn total_read(&self) -> u64 {
    self.total_read
  }

  pub fn total_write(&self) -> u64 {
    self.total_write
  }

  pub fn kind(&self) -> &DiskKind {
    &self.kind
  }

  pub fn smart_overall(&self) -> &str {
    &self.smart_overall
  }
}
