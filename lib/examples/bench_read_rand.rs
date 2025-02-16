use libglacierdisk::{
  benchmark::{benchmark::GlacierDiskBenchmark, Benchmark, BenchmarkConfig, BenchmarkType},
  disk::Disk,
};

fn main() {
  // Generally the benchmark wouldn't require sudo, but getting the Disk does, so we still have to escalate
  sudo::escalate_if_needed().expect("Failed to escalate privileges");

  let disk = Disk::new("/dev/sda".into()).unwrap();
  let mut benchmark = GlacierDiskBenchmark::new(disk, 0, BenchmarkConfig {
    random: true,
    kind: BenchmarkType::Read,
    ..BenchmarkConfig::default()
  }).unwrap();
  let file_size = benchmark.bench_config.total_size();

  println!("Running benchmark");
  println!(
    "Total benchmark file size: {:?}mb",
    byte_to_mb(file_size)
  );

  let result = benchmark.run().unwrap();

  println!("Total time: {:.2}s", result.elapsed.as_secs_f64());
  println!("Average speed: {:.2}MB/s", speed_to_mb(result.avg_speed));
}

fn byte_to_mb(bytes: usize) -> f64 {
  bytes as f64 / 1024.0 / 1024.0
}

fn speed_to_mb(bytes: f64) -> f64 {
  bytes / 1024.0 / 1024.0
}
