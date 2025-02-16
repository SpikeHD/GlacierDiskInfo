use std::{
  fmt::Debug,
  fs::{self, File, OpenOptions},
  io::{Read, Seek, SeekFrom, Write},
  path::PathBuf,
  time::{Duration, Instant},
};

use rand::seq::SliceRandom;

use crate::disk::ShallowDisk;

const FILENAME: &str = "glacierdisk-test.bin";

#[derive(Clone, Debug)]
pub enum BenchmarkType {
  Read,
  Write,
}

#[derive(Clone, Debug)]
pub struct BenchmarkResult {
  pub elapsed: Duration,
  pub avg_speed: f64,
}

#[derive(Clone, Debug)]
pub struct BenchmarkConfig {
  /// Whether this is a read or write benchmark
  pub kind: BenchmarkType,
  /// Block size in bytes
  pub block_size: usize,
  /// Block count
  pub block_count: usize,
  /// Set a custom file path
  pub file_path: Option<PathBuf>,
  /// Delete the created test file after the benchmark is complete
  pub delete_after: bool,
  /// Whether this is a sequential benchmark or a random benchmark
  pub random: bool,
}

impl Default for BenchmarkConfig {
  fn default() -> Self {
    Self {
      kind: BenchmarkType::Read,
      // 4kb blocks
      block_size: 4 * 1024,
      // Amount of blocks
      block_count: 1024 * 1024,
      file_path: None,
      delete_after: true,
      random: false,
    }
  }
}

impl BenchmarkConfig {
  pub fn total_size(&self) -> usize {
    self.block_size * self.block_count
  }
}

pub trait Benchmark {
  fn new(
    disk: impl Into<ShallowDisk>,
    mount: usize,
    block_config: BenchmarkConfig,
  ) -> Result<Self, Box<dyn std::error::Error>>
  where
    Self: Sized;
  /// Run the benchmark. When the benchmark is done, it will both emit and return the final progress.
  fn run(&mut self) -> Result<BenchmarkResult, Box<dyn std::error::Error>>;
}
/// A sequential-read benchmark
pub struct GlacierDiskBenchmark {
  pub disk: ShallowDisk,
  pub mount: PathBuf,
  pub running: bool,

  // Benchmark block configuration
  pub bench_config: BenchmarkConfig,
}

impl Debug for GlacierDiskBenchmark {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ReadBenchmark")
      .field("disk", &self.disk)
      .field("mount", &self.mount)
      .field("running", &self.running)
      .field("block_config", &self.bench_config)
      .finish()
  }
}

impl Benchmark for GlacierDiskBenchmark {
  fn new(
    disk: impl Into<ShallowDisk>,
    mount: usize,
    bench_config: BenchmarkConfig,
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

      bench_config,
    })
  }

  fn run(&mut self) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
    // Create file
    let (path, mut file) = get_file(
      self.bench_config.kind.clone(),
      &self.bench_config,
      &self.disk,
    )?;

    self.running = true;

    let elapsed = match self.bench_config.kind {
      BenchmarkType::Read => perform_read_benchmark(&self.bench_config, &mut file)?,
      BenchmarkType::Write => perform_write_benchmark(&self.bench_config, &mut file)?,
    };

    // Cleanup
    if self.bench_config.delete_after {
      fs::remove_file(path).unwrap_or_default();
    }

    self.running = false;

    Ok(BenchmarkResult {
      elapsed,
      avg_speed: self.bench_config.total_size() as f64 / elapsed.as_secs_f64(),
    })
  }
}

fn get_file(
  bench_type: BenchmarkType,
  bench_config: &BenchmarkConfig,
  disk: &ShallowDisk,
) -> Result<(PathBuf, File), Box<dyn std::error::Error>> {
  let existing_path = bench_config.file_path.clone();
  let actual_file = if let Some(path) = &existing_path {
    path
  } else {
    let mounts = disk.mounts()?;
    let mount = mounts.first().ok_or(format!(
      "No mount found at index {0} for disk {1:?}",
      0, disk
    ))?;
    let mount = mount.to_path_buf();
    let file_name = format!("{}/{}", mount.to_str().unwrap(), FILENAME);

    &mount.join(file_name)
  };

  match bench_type {
    BenchmarkType::Read => {
      let mut f = OpenOptions::new()
        .write(existing_path.is_none())
        .create(existing_path.is_none())
        .read(true)
        .open(actual_file)?;

      // Fill with random data
      if existing_path.is_none() {
        random_fill(&mut f, bench_config.total_size())?;
      }

      // Seek to start
      f.seek(SeekFrom::Start(0))?;

      Ok((actual_file.to_path_buf(), f))
    }
    BenchmarkType::Write => {
      let f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(actual_file)?;

      Ok((actual_file.to_path_buf(), f))
    }
  }
}

fn perform_read_benchmark(
  bench_config: &BenchmarkConfig,
  file: &mut File,
) -> Result<Duration, Box<dyn std::error::Error>> {
  let mut buf = vec![0; bench_config.block_size];

  let start = Instant::now();

  // Benchmark
  if bench_config.random {
    let mut rng = rand::rng();
    let mut block_positions = (0..bench_config.block_count).collect::<Vec<usize>>();
    block_positions.shuffle(&mut rng);

    for (total_reads, _) in (0..bench_config.block_count).enumerate() {
      let pos = block_positions[total_reads];

      file.seek(SeekFrom::Start(pos as u64 * bench_config.block_size as u64))?;
      let n = file.read(&mut buf)?;

      if n == 0 {
        break;
      }
    }
  } else {
    loop {
      let n = file.read(&mut buf)?;
      if n == 0 {
        break;
      }
    }
  }

  Ok(start.elapsed())
}

fn perform_write_benchmark(
  bench_config: &BenchmarkConfig,
  file: &mut File,
) -> Result<Duration, Box<dyn std::error::Error>> {
  let mut urand = File::open("/dev/urandom")?;
  let mut buf = vec![0; bench_config.block_size];

  let start = Instant::now();

  // Benchmark
  if bench_config.random {
    let mut rng = rand::rng();
    let mut block_positions = (0..bench_config.block_count).collect::<Vec<usize>>();
    block_positions.shuffle(&mut rng);

    for (total_writes, _) in (0..bench_config.block_count).enumerate() {
      let pos = block_positions[total_writes];

      urand.read_exact(&mut buf)?;

      file.seek(SeekFrom::Start(pos as u64 * bench_config.block_size as u64))?;
      file.write_all(&buf)?;
    }
  } else {
    for _ in 0..bench_config.block_count {
      urand.read_exact(&mut buf)?;
      file.write_all(&buf)?;
    }
  }

  Ok(start.elapsed())
}

fn random_fill(file: &mut File, size: usize) -> Result<(), Box<dyn std::error::Error>> {
  let mut urand = File::open("/dev/urandom")?;

  // Calculate a reasonable chunk size for the buffer
  let chunk_size = size / 1024;
  let mut buf = vec![0; chunk_size];

  for _ in 0..(size / chunk_size) {
    urand.read_exact(&mut buf)?;
    file.write_all(&buf[0..chunk_size])?;
  }

  Ok(())
}
