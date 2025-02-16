use libglacierdisk::{
  benchmark::{write::WriteBenchmark, Benchmark, BenchmarkConfig},
  disk::Disk,
};

fn main() {
  // Generally the benchmark wouldn't require sudo, but getting the Disk does, so we still have to escalate
  sudo::escalate_if_needed().expect("Failed to escalate privileges");

  let disk = Disk::new("/dev/sda".into()).unwrap();
  let mut benchmark = WriteBenchmark::new(disk, 0, BenchmarkConfig {
    random: true,
    ..BenchmarkConfig::default()
  }).unwrap();

  println!("Running benchmark");
  println!(
    "Total benchmark file size: {:?}mb",
    byte_to_mb(benchmark.bench_config.total_size())
  );

  let result = benchmark.run().unwrap();

  println!("Total time: {:.2}s", result.elapsed);
  println!("Average speed: {:.2}MB/s", speed_to_mb(result.avg_speed));
}

fn byte_to_mb(bytes: usize) -> f64 {
  bytes as f64 / 1024.0 / 1024.0
}

fn speed_to_mb(bytes: f64) -> f64 {
  bytes / 1024.0 / 1024.0
}
