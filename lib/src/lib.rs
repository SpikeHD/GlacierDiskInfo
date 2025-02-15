//! `libglacierdisk` is a linux-only library for interfacing with and reading SMART (and other) data from disks.
//! 
//! # Examples
//! 
//! ## List and log disks
//! ```rust
//!  use libglacierdisk;
//! 
//!  let disks = libglacierdisk::list_disks()?;
//!  for disk in disks {
//!    println!("{:?}", disk);
//!  }
//! ```
//! 
//! ## Get a specific disk
//! ```rust
//! use libglacierdisk;
//! 
//! let disk = libglacierdisk::disk::Disk::new("/dev/sda").unwrap();
//! println!("{:?}", disk);
//! ```
//! 
//! ## Get the temperature of a disk
//! 
//! ```rust
//! use libglacierdisk;
//! 
//! let disks = libglacierdisk::list_disks()?;
//! let first = disks.first()?;
//! 
//! // This will be in mkelvin
//! println!("{:?}", disk.raw_disk().get_temperature());
//! ```
//! 
//! ## Get a specific SMART attribute
//! 
//! ```rust
//! use libglacierdisk;
//! 
//! let disks = libglacierdisk::list_disks()?;
//! let first = disks.first()?;
//! 
//! let attribute = first.get_attribute("total-lbas-read")?;
//! println!("{:?}", attribute);
//! ```

use std::{
  error::Error,
  path::PathBuf,
};

use disk::{get_disk_paths, Disk};

pub mod ata;
pub mod attribute;
pub mod disk;
pub mod kind;
pub mod sysfs;

// Re-export libatasmart
pub use libatasmart;

#[cfg(target_os = "linux")]
static DEV_PATH: &str = "/dev";

/// List all disks on the system
pub fn list_disks() -> Result<Vec<Disk>, Box<dyn Error>> {
  let mut list = vec![];
  let disks = get_disk_paths()?;

  for disk in disks {
    let d = Disk::new(PathBuf::from(format!("{}/{}", DEV_PATH, disk)));

    if let Ok(d) = d {
      list.push(d);
    }
  }

  Ok(list)
}
