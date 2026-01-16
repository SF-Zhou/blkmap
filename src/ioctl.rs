//! Low-level FIEMAP ioctl structures and constants.
//!
//! This module contains the raw C structures and ioctl definitions
//! needed to interface with the Linux kernel's FIEMAP ioctl.

/// FIEMAP ioctl request code.
///
/// This is the ioctl number for the FIEMAP operation.
/// The value is `_IOWR('f', 11, struct fiemap)` which evaluates to the same
/// value on both x86 and ARM platforms.
pub const FS_IOC_FIEMAP: libc::c_ulong = 0xC020660B;

/// Maximum number of extents to request in a single ioctl call.
pub const FIEMAP_MAX_EXTENTS: u32 = 256;

/// Raw FIEMAP extent structure as returned by the kernel.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FiemapExtentRaw {
    /// Logical offset in bytes for the start of the extent.
    pub fe_logical: u64,
    /// Physical offset in bytes for the start of the extent.
    pub fe_physical: u64,
    /// Length in bytes for the extent.
    pub fe_length: u64,
    /// Reserved fields.
    pub fe_reserved64: [u64; 2],
    /// Flags for this extent.
    pub fe_flags: u32,
    /// Reserved fields.
    pub fe_reserved: [u32; 3],
}

/// Raw FIEMAP structure used for the ioctl call.
#[repr(C)]
#[derive(Debug)]
pub struct FiemapRaw {
    /// Logical offset (in bytes) at which to start mapping.
    pub fm_start: u64,
    /// Logical length (in bytes) of mapping which userspace wants.
    pub fm_length: u64,
    /// Flags for request.
    pub fm_flags: u32,
    /// Number of extents that were mapped.
    pub fm_mapped_extents: u32,
    /// Size of fm_extents array.
    pub fm_extent_count: u32,
    /// Reserved field.
    pub fm_reserved: u32,
    /// Extent array (variable size, but we allocate a fixed buffer).
    pub fm_extents: [FiemapExtentRaw; FIEMAP_MAX_EXTENTS as usize],
}

impl Default for FiemapRaw {
    fn default() -> Self {
        Self {
            fm_start: 0,
            fm_length: u64::MAX,
            fm_flags: 0,
            fm_mapped_extents: 0,
            fm_extent_count: FIEMAP_MAX_EXTENTS,
            fm_reserved: 0,
            fm_extents: [FiemapExtentRaw::default(); FIEMAP_MAX_EXTENTS as usize],
        }
    }
}

impl FiemapRaw {
    /// Creates a new FIEMAP request for the given range.
    pub fn new(start: u64, length: u64) -> Self {
        Self {
            fm_start: start,
            fm_length: length,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fiemap_raw_default() {
        let fiemap = FiemapRaw::default();
        assert_eq!(fiemap.fm_start, 0);
        assert_eq!(fiemap.fm_length, u64::MAX);
        assert_eq!(fiemap.fm_extent_count, FIEMAP_MAX_EXTENTS);
    }

    #[test]
    fn test_fiemap_raw_new() {
        let fiemap = FiemapRaw::new(1024, 4096);
        assert_eq!(fiemap.fm_start, 1024);
        assert_eq!(fiemap.fm_length, 4096);
    }

    #[test]
    fn test_fiemap_extent_raw_size() {
        // Ensure the extent structure has the expected size (56 bytes)
        assert_eq!(std::mem::size_of::<FiemapExtentRaw>(), 56);
    }
}
