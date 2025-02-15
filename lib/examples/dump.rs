fn main() -> Result<(), Box<dyn std::error::Error>> {
  sudo::escalate_if_needed().expect("Failed to escalate privileges");

  let disks = libglacierdisk::list_disks().unwrap();
  // Dump the first disk
  let disk = disks[0].clone();

  disk.raw_disk().dump().unwrap();

  Ok(())
}
