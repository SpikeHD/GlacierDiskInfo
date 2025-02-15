fn main() {
  sudo::escalate_if_needed().expect("Failed to escalate privileges");

  let disks = libglacierdisk::list_disks().unwrap();
  let first = disks.first().unwrap().clone();
  let temp = first.raw_disk().get_temperature().unwrap_or(0);

  println!("{:?} mkelvin", temp);
  println!("{:?} celsius", celsius(temp));
}

fn celsius(mkelvin: u64) -> f32 {
  (mkelvin as f32 / 1000.) - 273.15
}
