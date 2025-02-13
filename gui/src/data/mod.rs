use smart::smart_to_string;
use status::Status;

pub mod smart;
pub mod status;

pub fn drives_and_status() -> Vec<(String, Status)> {
  let drives = libglacierdisk::list_disks().expect("Failed to list disks");

  let mut drives: Vec<(String, Status)> = drives
    .iter()
    .filter_map(|d| {
      let mut status = match libglacierdisk::get_disk_info(d) {
        Ok(d) => d,
        Err(e) => {
          eprintln!("Error fetching disk at {:?}: {e}", d);
          return None;
        }
      };
      let smart = match status.smart_get_overall() {
        Ok(s) => s,
        Err(e) => {
          eprintln!("Error fetching smart status: {e}");
          return None;
        }
      };
      let state = smart_to_string(smart);

      let temp = status.get_temperature().unwrap_or(0);

      // convert mkelvin to celsius
      let temp = (temp as f32 / 1000.) - 273.15;

      Some((d.to_string_lossy().to_string(), Status { temp, state }))
    })
    .collect();

  // If drives is empty, we have to create a dummy
  if drives.is_empty() {
    drives.push((
      "No Disks Found".to_string(),
      Status {
        temp: 0.,
        state: "Good".to_string(),
      },
    ));
  }

  drives
}
