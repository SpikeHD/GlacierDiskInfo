use std::{
  fmt::Debug,
  fs,
  io::{Read, Seek, SeekFrom},
  path::PathBuf,
  time::Instant,
};

use rand::seq::SliceRandom;

use crate::disk::ShallowDisk;

use super::{random_fill, Benchmark, BenchmarkProgress, FILENAME};

/// A sequential-read benchmark
pub struct ReadBenchmark {
  pub disk: ShallowDisk,
  pub mount: PathBuf,
  pub running: bool,
  pub progress: super::BenchmarkProgress,

  // Benchmark block configuration
  pub bench_config: super::BenchmarkConfig,

  pub on_progress: Option<Box<dyn FnMut(super::BenchmarkProgress) + Send + 'static>>,
}

impl Debug for ReadBenchmark {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ReadBenchmark")
      .field("disk", &self.disk)
      .field("mount", &self.mount)
      .field("running", &self.running)
      .field("progress", &self.progress)
      .field("block_config", &self.bench_config)
      .finish()
  }
}

impl Benchmark for ReadBenchmark {
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
    let mut file = if let Some(path) = &self.bench_config.file_path {
      fs::OpenOptions::new().read(true).open(path)?
    } else {
      let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .truncate(true)
        .open(&file_path)?;

      // Fill with random data
      random_fill(&mut f, self.bench_config.total_size())?;
      // Seek to start
      f.seek(SeekFrom::Start(0))?;

      f
    };

    self.running = true;

    let start = Instant::now();

    let mut buf = vec![0; self.bench_config.block_size];

    // Benchmark
    if self.bench_config.random {
      let mut rng = rand::rng();
      let mut block_positions = (0..self.bench_config.block_count).collect::<Vec<usize>>();
      block_positions.shuffle(&mut rng);

      for (total_reads, _) in (0..self.bench_config.block_count).enumerate() {
        let pos = block_positions[total_reads];

        file.seek(SeekFrom::Start(pos as u64 * self.bench_config.block_size as u64))?;
        let n = file.read(&mut buf)?;

        self.progress = BenchmarkProgress {
          elapsed: start.elapsed().as_secs_f64(),
          avg_speed: self.bench_config.total_size() as f64 / start.elapsed().as_secs_f64(),
          pct: total_reads as f64 / self.bench_config.block_count as f64,
        };

        if let Some(f) = self.on_progress.as_mut() {
          f(self.progress.clone());
        }

        if n == 0 {
          break;
        }
      }
    } else {
      let mut total_reads = 0;

      loop {
        let n = file.read(&mut buf)?;
        self.progress = BenchmarkProgress {
          elapsed: start.elapsed().as_secs_f64(),
          avg_speed: self.bench_config.total_size() as f64 / start.elapsed().as_secs_f64(),
          pct: total_reads as f64 / self.bench_config.block_count as f64,
        };
  
        if let Some(f) = self.on_progress.as_mut() {
          f(self.progress.clone());
        }

        total_reads += 1;
  
        if n == 0 {
          break;
        }
      }
    }

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
