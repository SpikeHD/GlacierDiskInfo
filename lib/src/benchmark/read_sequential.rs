use std::{
  fmt::Debug,
  fs,
  io::{Read, Seek, SeekFrom},
  path::PathBuf,
  sync::{Arc, Mutex},
  thread,
  time::Instant,
};

use crate::disk::ShallowDisk;

use super::{random_fill, Benchmark, BenchmarkProgress};

const FILENAME: &str = "glacierdisk-test.bin";

/// A sequential-read benchmark
pub struct ReadSequentialBenchmark {
  pub disk: ShallowDisk,
  pub mount: PathBuf,
  pub running: bool,
  pub progress: super::BenchmarkProgress,

  // Benchmark block configuration
  pub block_config: super::BlockConfig,

  pub on_progress: Option<Box<dyn FnMut(super::BenchmarkProgress) + Send + 'static>>,
}

impl Debug for ReadSequentialBenchmark {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ReadSequentialBenchmark")
      .field("disk", &self.disk)
      .field("mount", &self.mount)
      .field("running", &self.running)
      .field("progress", &self.progress)
      .field("block_config", &self.block_config)
      .finish()
  }
}

impl Benchmark for ReadSequentialBenchmark {
  fn new(
    disk: crate::Disk,
    mount: usize,
    block_config: super::BlockConfig,
  ) -> Result<Self, Box<dyn std::error::Error>> {
    let mounts = disk.mounts();
    let mount = mounts
      .get(mount)
      .ok_or(format!("No mount found at index {mount} for disk {disk:?}"))?;

    Ok(Self {
      disk: disk.into(),
      mount: mount.to_path_buf(),
      running: false,
      progress: super::BenchmarkProgress::default(),

      block_config,

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
    let mut file = fs::OpenOptions::new()
      .write(true)
      .create(true)
      .read(true)
      .truncate(true)
      .open(&file_path)?;

    // Fill with random data
    random_fill(
      &mut file,
      self.block_config.total_size(),
    )?;

    // Seek to start
    file.seek(SeekFrom::Start(0))?;

    self.running = true;

    let start = Instant::now();

    let mut buf = vec![0; self.block_config.block_size];
    let mut total_reads = 0;

    // Benchmark
    loop {
      let n = file.read(&mut buf)?;
      self.progress = BenchmarkProgress {
        elapsed: start.elapsed().as_secs_f64(),
        avg_speed: self.block_config.total_size() as f64 / start.elapsed().as_secs_f64(),
        pct: total_reads as f64 / self.block_config.block_count as f64,
      };

      if let Some(f) = self.on_progress.as_mut() {
        f(self.progress.clone());
      }

      total_reads += 1;

      if n == 0 {
        break;
      }
    }

    let elapsed = start.elapsed();

    self.progress = BenchmarkProgress {
      elapsed: elapsed.as_secs_f64(),
      avg_speed: self.block_config.total_size() as f64 / elapsed.as_secs_f64(),
      pct: 1.0,
    };

    if let Some(f) = self.on_progress.as_mut() {
      f(self.progress.clone());
    }

    // Cleanup
    // fs::remove_file(file_path).unwrap_or_default();

    self.running = false;

    Ok(self.progress.clone())
  }

  fn on_progress(&mut self, f: impl FnMut(super::BenchmarkProgress) + Send + 'static) {
    self.on_progress = Some(Box::new(f));
  }
}
