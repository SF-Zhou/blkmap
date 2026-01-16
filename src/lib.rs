//! # blkmap
//!
//! Query file physical extents (FIEMAP) for a given range on disk.
//!
//! This crate provides a Rust interface to the Linux FIEMAP ioctl, which allows
//! querying the physical block mapping of files on disk.
//!
//! ## Example
//!
//! ```no_run
//! use blkmap::Fiemap;
//! use std::fs::File;
//!
//! let file = File::open("/path/to/file").unwrap();
//! let extents = file.fiemap().unwrap();
//! for extent in extents {
//!     println!("{:?}", extent);
//! }
//! ```
//!
//! ## Features
//!
//! - Query full file extent mapping
//! - Query extent mapping for a specific byte range
//! - Works on both x86 and ARM Linux platforms
//! - Bitflags for extent properties (shared, encoded, encrypted, etc.)

#![cfg(target_os = "linux")]
#![deny(missing_docs)]

mod error;
mod extent;
mod fiemap;
mod ioctl;
mod traits;

pub use error::{Error, Result};
pub use extent::{ExtentFlags, FiemapExtent};
pub use traits::Fiemap;
