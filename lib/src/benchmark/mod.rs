use std::{
  fs::File,
  io::{Read, Write},
  path::PathBuf, time::Duration,
};

use crate::disk::ShallowDisk;

pub mod benchmark;

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
