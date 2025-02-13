
fn main() -> Result<(), Box<dyn std::error::Error>> {
  sudo::escalate_if_needed().expect("Failed to escalate privileges");

  let disks = libglacierdisk::list_disks().unwrap();
  // Dump the first disk
  let path = disks[0].clone();
  let mut disk = match libglacierdisk::get_disk_info(&path) {
    Ok(d) => d,
    Err(e) => {
      eprintln!("Failed to get disk info: {e}");
      return Ok(());
    }
  };

  disk.dump().unwrap();

  Ok(())
}
