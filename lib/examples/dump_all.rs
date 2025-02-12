use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  sudo::escalate_if_needed().expect("Failed to escalate privileges");

  let disks = libglacierdisk::list_disks().unwrap();
  
  for disk in disks {
    let path = PathBuf::from(disk.clone());
    let mut disk = libglacierdisk::get_disk_info(path).unwrap();
    disk.dump().unwrap();
  }

  Ok(())
}
