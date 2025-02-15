
fn main() {
  sudo::escalate_if_needed().expect("Failed to escalate privileges");

  let disks = libglacierdisk::list_disks().expect("Failed to get list of disks");
  for disk in disks {
    println!("{:?}", disk);
  }
}
