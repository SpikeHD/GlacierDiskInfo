pub fn bytes_to_readable(bytes: u64) -> String {
  if bytes < 1024 {
    return format!("{} B", bytes);
  }
  else if bytes < 1024 * 1024 {
    return format!("{:.2} KB", bytes as f64 / 1024.0);
  }
  else if bytes < 1024 * 1024 * 1024 {
    return format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0);
  }
  else {
    return format!("{:.2} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0);
  }
}