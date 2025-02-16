use libglacierdisk::{
  benchmark::{
    read::ReadBenchmark, write::WriteBenchmark, Benchmark, BenchmarkConfig
  },
  disk::Disk,
};

fn main() {
  // Generally the benchmark wouldn't require sudo, but getting the Disk does, so we still have to escalate
  sudo::escalate_if_needed().expect("Failed to escalate privileges");

  let disk = Disk::new("/dev/sda".into()).expect("Failed to get Disk");
  // Assume the first mount is fine
  let mounts = disk.mounts().expect("Failed to get mounts");
  let mount = mounts.first().unwrap();
  let bin_file = format!("{}/test.bin", mount.to_path_buf().to_str().unwrap());

  // Write benchmark
  let mut write_benchmark = WriteBenchmark::new(
    disk.clone(),
    0,
    BenchmarkConfig {
      file_path: Some(bin_file.clone().into()),
      delete_after: false,
      random: true,
      ..BenchmarkConfig::default()
    },
  )
  .unwrap();

  println!("Running write benchmark");
  println!(
    "Total benchmark file size: {:?}mb",
    byte_to_mb(write_benchmark.bench_config.total_size())
  );

  let write_result = write_benchmark
    .run()
    .expect("Failed to run write benchmark");

  // Read benchmark
  let mut read_benchmark = ReadBenchmark::new(
    disk,
    0,
    BenchmarkConfig {
      file_path: Some(bin_file.into()),
      delete_after: true,
      random: true,
      ..BenchmarkConfig::default()
    },
  )
  .expect("Failed to create read benchmark");

  println!("Running read benchmark");
  println!(
    "Total benchmark file size: {:?}mb",
    byte_to_mb(read_benchmark.bench_config.total_size())
  );
  let read_result = read_benchmark.run().expect("Failed to run read benchmark");

  println!("=== Benchmark results ===");

  println!(
    "Write speed: {:.2}MB/s (took {:.2}s)",
    speed_to_mb(write_result.avg_speed),
    write_result.elapsed
  );
  println!(
    "Read speed: {:.2}MB/s (took {:.2}s)",
    speed_to_mb(read_result.avg_speed),
    read_result.elapsed
  );
}

fn byte_to_mb(bytes: usize) -> f64 {
  bytes as f64 / 1024.0 / 1024.0
}

fn speed_to_mb(bytes: f64) -> f64 {
  bytes / 1024.0 / 1024.0
}
