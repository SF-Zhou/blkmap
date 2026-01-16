//! FIEMAP query implementation.

use crate::error::{Error, Result};
use crate::extent::FiemapExtent;
use crate::ioctl::{FiemapRaw, FS_IOC_FIEMAP};

use std::os::unix::io::AsRawFd;

/// Performs a FIEMAP ioctl on the given file descriptor for the specified range.
///
/// # Arguments
///
/// * `fd` - File descriptor to query
/// * `start` - Starting byte offset in the file
/// * `length` - Length of the range to query in bytes
///
/// # Returns
///
/// A vector of `FiemapExtent` describing the physical layout of the file data.
pub fn query_fiemap<F: AsRawFd>(file: &F, start: u64, length: u64) -> Result<Vec<FiemapExtent>> {
    let fd = file.as_raw_fd();
    let mut extents = Vec::new();
    let mut current_start = start;

    // Handle potential overflow
    let end = start.checked_add(length).ok_or(Error::InvalidRange {
        offset: start,
        length,
    })?;

    loop {
        let remaining_length = end.saturating_sub(current_start);
        if remaining_length == 0 {
            break;
        }

        let mut fiemap = FiemapRaw::new(current_start, remaining_length);

        // SAFETY: We're calling ioctl with a valid file descriptor and properly
        // initialized FIEMAP structure. The kernel will write extent data into
        // our buffer, which we've properly sized.
        let ret = unsafe { libc::ioctl(fd, FS_IOC_FIEMAP, &mut fiemap as *mut FiemapRaw) };

        if ret < 0 {
            let err = std::io::Error::last_os_error();
            // EOPNOTSUPP or ENOTTY indicates FIEMAP is not supported
            if err.raw_os_error() == Some(libc::EOPNOTSUPP)
                || err.raw_os_error() == Some(libc::ENOTTY)
            {
                return Err(Error::NotSupported);
            }
            return Err(Error::Io(err));
        }

        let mapped = fiemap.fm_mapped_extents as usize;
        if mapped == 0 {
            break;
        }

        let mut is_last = false;
        for i in 0..mapped {
            let raw = &fiemap.fm_extents[i];
            let extent =
                FiemapExtent::new(raw.fe_logical, raw.fe_physical, raw.fe_length, raw.fe_flags);

            if extent.is_last() {
                is_last = true;
            }

            extents.push(extent);

            // Update the start position for the next iteration
            current_start = raw.fe_logical.saturating_add(raw.fe_length);
        }

        if is_last || current_start >= end {
            break;
        }
    }

    Ok(extents)
}

/// Queries the complete FIEMAP for a file.
///
/// This is equivalent to calling `query_fiemap(file, 0, u64::MAX)`.
#[inline]
pub fn query_fiemap_full<F: AsRawFd>(file: &F) -> Result<Vec<FiemapExtent>> {
    query_fiemap(file, 0, u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_query_fiemap_empty_file() {
        let temp = NamedTempFile::new().unwrap();
        let result = query_fiemap_full(temp.as_file());
        // Empty files should return either empty extents or an error
        // depending on the filesystem
        match result {
            Ok(extents) => {
                // Empty file should have no extents
                assert!(extents.is_empty());
            }
            Err(Error::NotSupported) => {
                // Some filesystems don't support FIEMAP
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_query_fiemap_with_data() {
        let mut temp = NamedTempFile::new().unwrap();
        // Write some data to create at least one extent
        let data = vec![0u8; 4096];
        temp.write_all(&data).unwrap();
        temp.flush().unwrap();

        // Force data to disk
        temp.as_file().sync_all().unwrap();

        let result = query_fiemap_full(temp.as_file());
        match result {
            Ok(extents) => {
                // File with data should have at least one extent
                // (unless it's delayed allocation)
                for extent in &extents {
                    assert!(extent.length > 0);
                }
            }
            Err(Error::NotSupported) => {
                // Some filesystems don't support FIEMAP (e.g., tmpfs)
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_query_fiemap_range() {
        let mut temp = NamedTempFile::new().unwrap();
        let data = vec![0u8; 8192];
        temp.write_all(&data).unwrap();
        temp.flush().unwrap();
        temp.as_file().sync_all().unwrap();

        // Query a specific range
        let result = query_fiemap(temp.as_file(), 1024, 2048);
        match result {
            Ok(_extents) => {
                // Range query succeeded
            }
            Err(Error::NotSupported) => {
                // Some filesystems don't support FIEMAP
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_invalid_range() {
        let temp = NamedTempFile::new().unwrap();
        // This should cause overflow
        let result = query_fiemap(temp.as_file(), u64::MAX, u64::MAX);
        match result {
            Err(Error::InvalidRange { offset, length }) => {
                assert_eq!(offset, u64::MAX);
                assert_eq!(length, u64::MAX);
            }
            _ => panic!("Expected InvalidRange error"),
        }
    }

    #[test]
    fn test_query_with_file_reference() {
        let temp = NamedTempFile::new().unwrap();
        let file: &File = temp.as_file();
        let _ = query_fiemap_full(file);
    }
}
