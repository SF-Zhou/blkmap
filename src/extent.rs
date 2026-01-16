//! FIEMAP extent structures and flags.

use bitflags::bitflags;

bitflags! {
    /// Flags describing the properties of a file extent.
    ///
    /// These flags are returned by the FIEMAP ioctl and describe various
    /// properties of each extent in the file's block mapping.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ExtentFlags: u32 {
        /// This is the last extent in the file.
        const LAST = 0x00000001;
        /// The extent is not yet allocated (for pre-allocated or sparse files).
        const UNKNOWN = 0x00000002;
        /// The extent is delayed allocation and not yet written to disk.
        const DELALLOC = 0x00000004;
        /// The extent data is stored inline in the inode or extent tree.
        const ENCODED = 0x00000008;
        /// The extent is encrypted.
        const DATA_ENCRYPTED = 0x00000080;
        /// Extent offsets are in units relative to the file's logical offset.
        const NOT_ALIGNED = 0x00000100;
        /// Data in this extent is stored inline in the metadata.
        const DATA_INLINE = 0x00000200;
        /// Data in this extent is tail-packed with other files.
        const DATA_TAIL = 0x00000400;
        /// The extent represents a hole (unallocated space).
        const UNWRITTEN = 0x00000800;
        /// The extent is merged from multiple underlying extents.
        const MERGED = 0x00001000;
        /// The extent is shared with other files (e.g., reflinked or deduplicated).
        const SHARED = 0x00002000;
    }
}

impl ExtentFlags {
    /// Returns true if this is the last extent in the file.
    #[inline]
    pub fn is_last(&self) -> bool {
        self.contains(Self::LAST)
    }

    /// Returns true if the extent location is unknown (e.g., delayed allocation).
    #[inline]
    pub fn is_unknown(&self) -> bool {
        self.contains(Self::UNKNOWN)
    }

    /// Returns true if the extent is delayed allocation.
    #[inline]
    pub fn is_delalloc(&self) -> bool {
        self.contains(Self::DELALLOC)
    }

    /// Returns true if the extent data is encoded/compressed.
    #[inline]
    pub fn is_encoded(&self) -> bool {
        self.contains(Self::ENCODED)
    }

    /// Returns true if the extent data is encrypted.
    #[inline]
    pub fn is_encrypted(&self) -> bool {
        self.contains(Self::DATA_ENCRYPTED)
    }

    /// Returns true if the extent data is stored inline.
    #[inline]
    pub fn is_inline(&self) -> bool {
        self.contains(Self::DATA_INLINE)
    }

    /// Returns true if the extent represents unwritten/preallocated space.
    #[inline]
    pub fn is_unwritten(&self) -> bool {
        self.contains(Self::UNWRITTEN)
    }

    /// Returns true if the extent is shared with other files.
    #[inline]
    pub fn is_shared(&self) -> bool {
        self.contains(Self::SHARED)
    }
}

/// A single extent in a file's physical block mapping.
///
/// Each extent represents a contiguous range of physical blocks that store
/// a portion of the file's data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FiemapExtent {
    /// Logical offset in the file where this extent starts (in bytes).
    pub logical: u64,
    /// Physical offset on the block device where this extent starts (in bytes).
    pub physical: u64,
    /// Length of this extent (in bytes).
    pub length: u64,
    /// Flags describing this extent's properties.
    pub flags: ExtentFlags,
}

impl FiemapExtent {
    /// Creates a new FiemapExtent from raw values.
    #[inline]
    pub(crate) fn new(logical: u64, physical: u64, length: u64, flags: u32) -> Self {
        Self {
            logical,
            physical,
            length,
            flags: ExtentFlags::from_bits_truncate(flags),
        }
    }

    /// Returns true if this is the last extent in the file.
    #[inline]
    pub fn is_last(&self) -> bool {
        self.flags.is_last()
    }

    /// Returns true if the extent is shared with other files.
    #[inline]
    pub fn is_shared(&self) -> bool {
        self.flags.is_shared()
    }

    /// Returns true if the extent represents unwritten/preallocated space.
    #[inline]
    pub fn is_unwritten(&self) -> bool {
        self.flags.is_unwritten()
    }
}

impl std::fmt::Display for FiemapExtent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Extent {{ logical: {:#x}, physical: {:#x}, length: {:#x}, flags: {:?} }}",
            self.logical, self.physical, self.length, self.flags
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extent_flags_last() {
        let flags = ExtentFlags::LAST;
        assert!(flags.is_last());
        assert!(!flags.is_shared());
    }

    #[test]
    fn test_extent_flags_multiple() {
        let flags = ExtentFlags::LAST | ExtentFlags::SHARED;
        assert!(flags.is_last());
        assert!(flags.is_shared());
        assert!(!flags.is_unwritten());
    }

    #[test]
    fn test_extent_display() {
        let extent = FiemapExtent::new(0, 4096, 8192, 0x00000001);
        let s = extent.to_string();
        assert!(s.contains("logical: 0x0"));
        assert!(s.contains("physical: 0x1000"));
        assert!(s.contains("length: 0x2000"));
    }

    #[test]
    fn test_extent_flags_from_bits() {
        let flags = ExtentFlags::from_bits_truncate(0x00002001);
        assert!(flags.contains(ExtentFlags::LAST));
        assert!(flags.contains(ExtentFlags::SHARED));
    }
}
