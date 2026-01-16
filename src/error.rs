//! Error types for the blkmap crate.

use std::io::{Error, ErrorKind};

/// Result type for blkmap operations.
pub type Result<T> = std::io::Result<T>;

/// Creates an error indicating that FIEMAP is not supported on this filesystem.
#[inline]
pub fn not_supported() -> Error {
    Error::new(
        ErrorKind::Unsupported,
        "FIEMAP not supported on this filesystem",
    )
}

/// Creates an error for an invalid range (e.g., offset + length overflows).
#[inline]
pub fn invalid_range(offset: u64, length: u64) -> Error {
    Error::new(
        ErrorKind::InvalidInput,
        format!("Invalid range: offset={}, length={}", offset, length),
    )
}
