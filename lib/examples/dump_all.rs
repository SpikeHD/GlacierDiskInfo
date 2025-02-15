fn main() -> Result<(), Box<dyn std::error::Error>> {
  sudo::escalate_if_needed().expect("Failed to escalate privileges");

  let disks = libglacierdisk::list_disks().unwrap();

  for disk in disks {
    disk.raw_disk().dump().unwrap();
  }

  Ok(())
}
