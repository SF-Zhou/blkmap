//! Error types for the blkmap crate.

use thiserror::Error;

/// Result type for blkmap operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when querying file extent maps.
#[derive(Error, Debug)]
pub enum Error {
    /// I/O error from the underlying system call.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The file system does not support FIEMAP.
    #[error("FIEMAP not supported on this file system")]
    NotSupported,

    /// Invalid range specified (e.g., start + length overflows).
    #[error("Invalid range: offset={offset}, length={length}")]
    InvalidRange {
        /// The starting offset of the range.
        offset: u64,
        /// The length of the range.
        length: u64,
    },
}
