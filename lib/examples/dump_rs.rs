use libglacierdisk::attribute::dump_attributes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  sudo::escalate_if_needed().expect("Failed to escalate privileges");

  let disks = libglacierdisk::list_disks().unwrap();

  for disk in disks {
    let path = disk.clone();
    let mut disk = match libglacierdisk::get_disk_info(&path) {
      Ok(d) => d,
      Err(e) => {
        eprintln!("Failed to get disk info: {e}");
        continue;
      }
    };
    
    dump_attributes(&mut disk);
  }

  Ok(())
}
