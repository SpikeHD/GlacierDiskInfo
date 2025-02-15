# libglacierdisk

This is the underlying library that powers the [GlacierDiskInfo](https://github.com/SpikeHD/GlacierDiskInfo) project. It is a linux-only library for interfacing with and reading SMART (and other) data from disks.

## Usage

Run the following:

```bash
cargo add libglacierdisk
```

# Examples

## List and log disks
```rust
 use libglacierdisk;

 let disks = libglacierdisk::list_disks()?;
 for disk in disks {
   println!("{:?}", disk);
 }
```

## Get a specific disk
```rust
use libglacierdisk::{ disk::Disk };

let disk = Disk::new("/dev/sda").unwrap();
println!("{:?}", disk);
```

## Get the temperature of a disk

```rust
use libglacierdisk;

let disks = libglacierdisk::list_disks()?;
let first = disks.first()?;

// This will be in mkelvin
println!("{:?}", disk.raw_disk().get_temperature());
```

## Get a specific SMART attribute

```rust
use libglacierdisk;

let disks = libglacierdisk::list_disks()?;
let first = disks.first()?;

let attribute = first.get_attribute("total-lbas-read")?;
println!("{:?}", attribute);
```
