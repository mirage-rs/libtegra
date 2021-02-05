//! Utilities for working with data cache lines.

use core::ops::{Deref, DerefMut};

/// A helper that aligns a block of data to cache line size.
#[cfg(not(target_arch = "aarch64"))]
#[repr(transparent)]
pub struct CachePad<T, const N: usize>([T; N]);

/// A helper that aligns a block of data to cache line size.
#[cfg(target_arch = "aarch64")]
#[repr(align(64), transparent)]
pub struct CachePad<T, const N: usize>([T; N]);

impl<T, const N: usize> CachePad<T, { N }> {
    /// Constructs a new cache pad by consuming the supplied data.
    #[inline(always)]
    pub fn new(data: [T; N]) -> Self {
        CachePad(data)
    }

    /// Returns the inner data.
    pub fn into_inner(self) -> [T; N] {
        self.0
    }
}

impl<T: Copy + Default, const N: usize> From<&[T]> for CachePad<T, { N }> {
    fn from(data: &[T]) -> Self {
        assert!(data.len() <= N);

        // Construct a new cache pad and copy in the data.
        let mut pad = CachePad::new([T::default(); N]);
        pad.copy_from_slice(data);

        pad
    }
}

impl<T, const N: usize> Deref for CachePad<T, { N }> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for CachePad<T, { N }> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Flushes the data cache line starting from the given address.
#[cfg(not(target_arch = "aarch64"))]
pub unsafe fn flush_data_cache_line(_: usize) {
    // Purposefully do nothing...
}

/// Flushes the data cache line starting from the given address.
#[cfg(target_arch = "aarch64")]
pub unsafe fn flush_data_cache_line(line: usize) {
    asm!("dc civac, {}", in(reg) line);
}

/// Flushes all data cache lines which are covered by an object of known size.
pub unsafe fn flush_data_cache<T>(_obj: &T, _size: usize)
where
    T: ?Sized,
{
    #[cfg(target_arch = "aarch64")]
    {
        const DATA_CACHE_LINE_SIZE: usize = 64;

        let start = &_obj as *const _ as usize;
        let end = super::align_up(start + _size, DATA_CACHE_LINE_SIZE);

        cortex_a::barrier::dmb(cortex_a::barrier::SY);
        for line in (start..end).step_by(DATA_CACHE_LINE_SIZE) {
            // Flush all data cache lines within the given area.
            flush_data_cache_line(line);
        }
        cortex_a::barrier::dmb(cortex_a::barrier::SY);
    }
}
