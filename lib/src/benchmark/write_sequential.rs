use std::{
  fmt::Debug,
  fs::{self, File},
  io::{Read, Seek, SeekFrom, Write},
  path::PathBuf,
  time::Instant,
};

use crate::disk::ShallowDisk;

use super::{Benchmark, BenchmarkProgress};

const FILENAME: &str = "glacierdisk-test.bin";

/// A sequential-read benchmark
pub struct WriteSequentialBenchmark {
  pub disk: ShallowDisk,
  pub mount: PathBuf,
  pub running: bool,
  pub progress: super::BenchmarkProgress,

  // Benchmark block configuration
  pub bench_config: super::BenchmarkConfig,

  pub on_progress: Option<Box<dyn FnMut(super::BenchmarkProgress) + Send + 'static>>,
}

impl Debug for WriteSequentialBenchmark {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("WriteSequentialBenchmark")
      .field("disk", &self.disk)
      .field("mount", &self.mount)
      .field("running", &self.running)
      .field("progress", &self.progress)
      .field("bench_config", &self.bench_config)
      .finish()
  }
}

impl Benchmark for WriteSequentialBenchmark {
  fn new(
    disk: impl Into<ShallowDisk>,
    mount: usize,
    bench_config: super::BenchmarkConfig,
  ) -> Result<Self, Box<dyn std::error::Error>> {
    let disk = disk.into();
    let mounts = disk.mounts()?;
    let mount = mounts
      .get(mount)
      .ok_or(format!("No mount found at index {mount} for disk {disk:?}"))?;

    Ok(Self {
      disk: disk.clone(),
      mount: mount.to_path_buf(),
      running: false,
      progress: super::BenchmarkProgress::default(),

      bench_config,

      on_progress: None,
    })
  }

  fn run(&mut self) -> Result<BenchmarkProgress, Box<dyn std::error::Error>> {
    let file_path = self.mount.join(FILENAME);

    // Delete file if it exists
    if fs::metadata(&file_path).is_ok() {
      fs::remove_file(&file_path).unwrap_or_default();
    }

    // Create file
    let actual = if let Some(path) = &self.bench_config.file_path {
      path
    } else {
      &file_path
    };
    let mut file = fs::OpenOptions::new()
      .write(true)
      .create(true)
      .read(true)
      .truncate(true)
      .open(actual)?;

    self.running = true;

    let start = Instant::now();

    // Benchmark
    let mut urand = File::open("/dev/urandom")?;
    let mut buf = vec![0; self.bench_config.block_size];

    for (total_writes, _) in (0..self.bench_config.block_count).enumerate() {
      urand.read_exact(&mut buf)?;
      file.write_all(&buf)?;

      self.progress = BenchmarkProgress {
        elapsed: start.elapsed().as_secs_f64(),
        avg_speed: self.bench_config.total_size() as f64 / start.elapsed().as_secs_f64(),
        pct: total_writes as f64 / self.bench_config.block_count as f64,
      };

      if let Some(f) = self.on_progress.as_mut() {
        f(self.progress.clone());
      }
    }

    // Seek to start
    file.seek(SeekFrom::Start(0))?;
    let elapsed = start.elapsed();

    self.progress = BenchmarkProgress {
      elapsed: elapsed.as_secs_f64(),
      avg_speed: self.bench_config.total_size() as f64 / elapsed.as_secs_f64(),
      pct: 1.0,
    };

    if let Some(f) = self.on_progress.as_mut() {
      f(self.progress.clone());
    }

    // Cleanup
    if self.bench_config.delete_after {
      fs::remove_file(file_path).unwrap_or_default();
    }

    self.running = false;

    Ok(self.progress.clone())
  }

  fn on_progress(&mut self, f: impl FnMut(super::BenchmarkProgress) + Send + 'static) {
    self.on_progress = Some(Box::new(f));
  }
}
