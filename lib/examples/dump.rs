use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  sudo::escalate_if_needed().expect("Failed to escalate privileges");

  let disks = libminidisk::list_disks().unwrap();
  // Dump the first disk
  let path = PathBuf::from(disks[0].clone());
  let mut disk = libminidisk::get_disk_info(path).unwrap();
  
  disk.dump().unwrap();

  Ok(())
}