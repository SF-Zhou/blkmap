# blkmap

[![CI](https://github.com/SF-Zhou/blkmap/actions/workflows/ci.yml/badge.svg)](https://github.com/SF-Zhou/blkmap/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/blkmap.svg)](https://crates.io/crates/blkmap)
[![Documentation](https://docs.rs/blkmap/badge.svg)](https://docs.rs/blkmap)
[![License](https://img.shields.io/crates/l/blkmap.svg)](https://github.com/SF-Zhou/blkmap#license)

Query file physical extents (FIEMAP) for a given range on disk.

This crate provides a Rust interface to the Linux FIEMAP ioctl, which allows querying the physical block mapping of files on disk. It works on both x86 and ARM Linux platforms.

## Features

- Query complete file extent mapping
- Query extent mapping for a specific byte range
- Cross-platform support for x86 and ARM Linux
- Strongly-typed extent flags using bitflags
- Easy-to-use trait-based API
- CLI tool for quick extent queries

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
blkmap = "0.1"
```

Or install the CLI tool:

```bash
cargo install blkmap
```

## Library Usage

### Query full file extent map

```rust
use blkmap::Fiemap;
use std::fs::File;

let file = File::open("/path/to/file")?;
let extents = file.fiemap()?;

for extent in extents {
    println!("Logical: {:#x}, Physical: {:#x}, Length: {:#x}",
             extent.logical, extent.physical, extent.length);
    println!("Flags: {:?}", extent.flags);
}
```

### Query extent map for a specific range

```rust
use blkmap::Fiemap;
use std::fs::File;

let file = File::open("/path/to/file")?;
// Query extents for bytes 0-4096
let extents = file.fiemap_range(0, 4096)?;
```

### Using with paths

The `Fiemap` trait is also implemented for path types:

```rust
use blkmap::Fiemap;
use std::path::Path;

// Using a Path
let extents = Path::new("/path/to/file").fiemap()?;

// Using a PathBuf
let path = std::path::PathBuf::from("/path/to/file");
let extents = path.fiemap()?;
```

### Checking extent flags

```rust
use blkmap::{Fiemap, ExtentFlags};
use std::fs::File;

let file = File::open("/path/to/file")?;
let extents = file.fiemap()?;

for extent in extents {
    if extent.flags.is_shared() {
        println!("Extent is shared (reflinked/deduplicated)");
    }
    if extent.flags.is_unwritten() {
        println!("Extent is preallocated but not yet written");
    }
    if extent.is_last() {
        println!("This is the last extent in the file");
    }
}
```

## CLI Usage

```bash
# Query all extents for a file
blkmap /path/to/file

# Query extents starting from offset 1024
blkmap /path/to/file --offset 1024

# Query extents for a specific range
blkmap /path/to/file --offset 0 --length 4096

# Short options
blkmap /path/to/file -o 1024 -l 8192
```

Example output:

```
Index  Logical            Physical           Length             Flags
--------------------------------------------------------------------------------
0      0x0000000000000000 0x00000000c8a01000 0x0000000000001000 LAST
--------------------------------------------------------------------------------
Total: 1 extent(s)
```

## Extent Flags

The following extent flags are supported:

| Flag | Description |
|------|-------------|
| `LAST` | This is the last extent in the file |
| `UNKNOWN` | Extent location is not yet known |
| `DELALLOC` | Delayed allocation (not yet written to disk) |
| `ENCODED` | Data is compressed/encoded |
| `DATA_ENCRYPTED` | Data is encrypted |
| `NOT_ALIGNED` | Extent offsets not aligned to block boundary |
| `DATA_INLINE` | Data stored inline in metadata |
| `DATA_TAIL` | Data is tail-packed with other files |
| `UNWRITTEN` | Extent is allocated but not yet written |
| `MERGED` | Extent merged from multiple underlying extents |
| `SHARED` | Extent is shared with other files (reflink) |

## Platform Support

This crate only works on Linux systems. It has been tested on:

- x86_64 (Intel/AMD)
- aarch64 (ARM64)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
