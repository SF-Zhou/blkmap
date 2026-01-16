//! Fiemap trait and implementations for common types.

use crate::error::Result;
use crate::extent::FiemapExtent;
use crate::fiemap::{query_fiemap, query_fiemap_full};

use std::fs::File;
use std::path::{Path, PathBuf};

/// A trait for querying file extent maps (FIEMAP).
///
/// This trait provides methods to query the physical block mapping of files,
/// either for the entire file or for a specific byte range.
///
/// # Example
///
/// ```no_run
/// use blkmap::Fiemap;
/// use std::fs::File;
///
/// let file = File::open("/path/to/file").unwrap();
///
/// // Get all extents
/// let extents = file.fiemap().unwrap();
///
/// // Get extents for a specific range
/// let range_extents = file.fiemap_range(0, 4096).unwrap();
/// ```
pub trait Fiemap {
    /// Returns the complete extent map for the file.
    ///
    /// This queries the physical block mapping for the entire file from
    /// offset 0 to the end of the file.
    ///
    /// # Returns
    ///
    /// A vector of `FiemapExtent` describing each extent in the file's
    /// physical layout.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The filesystem does not support FIEMAP
    /// - An I/O error occurs during the ioctl call
    fn fiemap(&self) -> Result<Vec<FiemapExtent>>;

    /// Returns the extent map for a specific byte range in the file.
    ///
    /// This queries the physical block mapping for bytes in the range
    /// `[offset, offset + length)`.
    ///
    /// # Arguments
    ///
    /// * `offset` - Starting byte offset in the file
    /// * `length` - Number of bytes to query
    ///
    /// # Returns
    ///
    /// A vector of `FiemapExtent` describing extents that overlap with
    /// the specified range.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The range is invalid (e.g., offset + length overflows)
    /// - The filesystem does not support FIEMAP
    /// - An I/O error occurs during the ioctl call
    fn fiemap_range(&self, offset: u64, length: u64) -> Result<Vec<FiemapExtent>>;
}

impl Fiemap for File {
    fn fiemap(&self) -> Result<Vec<FiemapExtent>> {
        query_fiemap_full(self)
    }

    fn fiemap_range(&self, offset: u64, length: u64) -> Result<Vec<FiemapExtent>> {
        query_fiemap(self, offset, length)
    }
}

impl Fiemap for Path {
    fn fiemap(&self) -> Result<Vec<FiemapExtent>> {
        let file = File::open(self)?;
        file.fiemap()
    }

    fn fiemap_range(&self, offset: u64, length: u64) -> Result<Vec<FiemapExtent>> {
        let file = File::open(self)?;
        file.fiemap_range(offset, length)
    }
}

impl Fiemap for PathBuf {
    fn fiemap(&self) -> Result<Vec<FiemapExtent>> {
        self.as_path().fiemap()
    }

    fn fiemap_range(&self, offset: u64, length: u64) -> Result<Vec<FiemapExtent>> {
        self.as_path().fiemap_range(offset, length)
    }
}

impl Fiemap for str {
    fn fiemap(&self) -> Result<Vec<FiemapExtent>> {
        Path::new(self).fiemap()
    }

    fn fiemap_range(&self, offset: u64, length: u64) -> Result<Vec<FiemapExtent>> {
        Path::new(self).fiemap_range(offset, length)
    }
}

impl Fiemap for String {
    fn fiemap(&self) -> Result<Vec<FiemapExtent>> {
        self.as_str().fiemap()
    }

    fn fiemap_range(&self, offset: u64, length: u64) -> Result<Vec<FiemapExtent>> {
        self.as_str().fiemap_range(offset, length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_fiemap() {
        let temp = NamedTempFile::new().unwrap();
        let result = temp.as_file().fiemap();
        match result {
            Ok(_) | Err(Error::NotSupported) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_file_fiemap_range() {
        let mut temp = NamedTempFile::new().unwrap();
        temp.write_all(&[0u8; 4096]).unwrap();
        temp.flush().unwrap();
        temp.as_file().sync_all().unwrap();

        let result = temp.as_file().fiemap_range(0, 2048);
        match result {
            Ok(_) | Err(Error::NotSupported) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_path_fiemap() {
        let temp = NamedTempFile::new().unwrap();
        let path = temp.path();

        let result = path.fiemap();
        match result {
            Ok(_) | Err(Error::NotSupported) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_pathbuf_fiemap() {
        let temp = NamedTempFile::new().unwrap();
        let path: PathBuf = temp.path().to_path_buf();

        let result = path.fiemap();
        match result {
            Ok(_) | Err(Error::NotSupported) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_str_fiemap() {
        let temp = NamedTempFile::new().unwrap();
        let path_str = temp.path().to_str().unwrap();

        let result = path_str.fiemap();
        match result {
            Ok(_) | Err(Error::NotSupported) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_string_fiemap() {
        let temp = NamedTempFile::new().unwrap();
        let path_string = temp.path().to_str().unwrap().to_string();

        let result = path_string.fiemap();
        match result {
            Ok(_) | Err(Error::NotSupported) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_nonexistent_path() {
        let path = "/nonexistent/path/to/file";
        let result = path.fiemap();
        match result {
            Err(Error::Io(e)) => {
                assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
            }
            _ => panic!("Expected NotFound error"),
        }
    }
}
