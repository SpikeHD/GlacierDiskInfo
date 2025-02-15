use std::{
  fs::File,
  io::{Read, Write},
};

pub mod read_sequential;

pub enum BenchmarkMessage {
  Progress(BenchmarkProgress),
  Finished,
}

#[derive(Clone, Default, Debug)]
pub struct BenchmarkProgress {
  /// Amount of time elapsed in the benchmark, in seconds
  pub elapsed: f64,

  /// Average speed in bytes/s
  pub avg_speed: f64,

  /// Percent complete
  pub pct: f64,
}

#[derive(Clone, Debug)]
pub struct BlockConfig {
  /// Block size in bytes
  pub block_size: usize,
  /// Block count
  pub block_count: usize,
}

impl Default for BlockConfig {
  fn default() -> Self {
    Self {
      // 4kb blocks
      block_size: 4 * 1024,
      // Amount of blocks
      block_count: 1024 * 1024,
    }
  }
}

impl BlockConfig {
  pub fn total_size(&self) -> usize {
    self.block_size * self.block_count
  }
}

pub trait Benchmark {
  fn new(
    disk: crate::Disk,
    mount: usize,
    block_config: BlockConfig,
  ) -> Result<Self, Box<dyn std::error::Error>>
  where
    Self: Sized;
  /// Run the benchmark. When the benchmark is done, it will both emit and return the final progress.
  fn run(&mut self) -> Result<BenchmarkProgress, Box<dyn std::error::Error>>;
  /// Provide a function to run when the benchmark progress changes
  fn on_progress(&mut self, f: impl FnMut(BenchmarkProgress) + Send + 'static);
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
