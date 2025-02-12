pub fn bytes_to_readable(bytes: u64) -> String {
  if bytes < 1024 {
    format!("{} B", bytes)
  } else if bytes < 1024 * 1024 {
    return format!("{:.2} KB", bytes as f64 / 1024.0);
  } else if bytes < 1024 * 1024 * 1024 {
    return format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0);
  } else {
    return format!("{:.2} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0);
  }
}

pub fn ms_to_readable(ms: u64) -> String {
  const SECONDS_IN_MINUTE: f32 = 60.;

  let ms = ms as f32;

  if ms < 1000. {
    return format!("{} ms", ms);
  }

  let seconds = ms / 1000.;

  if seconds < SECONDS_IN_MINUTE {
    return format!("{:.2} s", seconds);
  }

  let minutes = seconds / SECONDS_IN_MINUTE;
  if minutes < 60. {
    return format!("{:.2} m", minutes);
  }

  let hours = minutes / 60.;
  format!("{:.2} h", hours)
}
