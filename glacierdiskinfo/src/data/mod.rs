use disk_cache::DiskCache;
use libglacierdisk::disk::Disk;
use smart::smart_to_string;
use status::Status;

pub mod disk_cache;
pub mod smart;
pub mod status;

pub fn drives_and_status() -> Vec<(DiskCache, Status)> {
  let mut drives = libglacierdisk::list_disks().expect("Failed to list disks");

  let drives: Vec<(DiskCache, Status)> = drives
    .iter_mut()
    .filter_map(|d| {
      let mut disk = d.raw_disk();
      let smart = match disk.smart_get_overall() {
        Ok(s) => s,
        Err(e) => {
          eprintln!("Error fetching smart status: {e}");
          return None;
        }
      };
      let state = smart_to_string(smart);

      let temp = disk.get_temperature().unwrap_or(0);

      // convert mkelvin to celsius
      let temp = (temp as f32 / 1000.) - 273.15;

      // Drop the mutex guard, preventing deadlock
      drop(disk);

      let disk_cache = DiskCache::new(d.clone());

      Some((disk_cache, Status { temp, state }))
    })
    .collect();

  drives
}
