use libglacierdisk::{
  benchmark::{Benchmark, BenchmarkConfig, BenchmarkResult, BenchmarkType, GlacierDiskBenchmark},
  disk::ShallowDisk,
};

#[derive(Clone, Debug)]
pub enum BenchKind {
  ALL,
  SEQ1M,
  SEQ128K,
  RAND4K,
}

pub fn run_rw(
  configs: &(BenchKind, Vec<BenchmarkConfig>),
  disk: ShallowDisk,
) -> Result<Vec<(BenchKind, BenchmarkResult)>, Box<dyn std::error::Error>> {
  let mut results = vec![];
  let mounts = disk.mounts()?;
  // TODO use better methodology that "whatever mount is first"
  let mount = mounts
    .first()
    .ok_or(format!("No mounts found for disk {disk:?}",))?;
  let (kind, configs) = configs;

  for config in configs {
    let w_config = BenchmarkConfig {
      kind: BenchmarkType::Write,
      file_path: Some(mount.join("glacier-disk-mark.bin")),
      delete_after: false,
      ..config.clone()
    };
    let r_config = BenchmarkConfig {
      kind: BenchmarkType::Read,
      file_path: Some(mount.join("glacier-disk-mark.bin")),
      delete_after: true,
      ..config.clone()
    };

    // TODO use better methodology that "whatever mount is first"
    let mut w = GlacierDiskBenchmark::new(disk.clone(), 0, w_config)?;
    let mut r = GlacierDiskBenchmark::new(disk.clone(), 0, r_config)?;

    // Run the benchmark
    let w_result = w.run()?;
    let r_result = r.run()?;

    results.push((kind.clone(), w_result));
    results.push((kind.clone(), r_result));
  }

  Ok(results)
}
