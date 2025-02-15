use libglacierdisk::attribute::dump_attributes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  sudo::escalate_if_needed().expect("Failed to escalate privileges");

  let disks = libglacierdisk::list_disks().unwrap();

  for mut disk in disks {
    dump_attributes(&mut disk.raw_disk());
  }

  Ok(())
}
