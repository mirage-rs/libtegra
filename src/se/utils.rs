use core::ops::{Deref, DerefMut};

use cortex_a::barrier;

const DATA_CACHE_LINE_SIZE: usize = 64;

/// A padding to align an inner value to the length of a cache line.
#[repr(align(64))]
pub struct CachePad<T: Default, const N: usize>([T; N]);

impl<T: Copy + Default, const N: usize> CachePad<T, { N }> {
    /// Constructs a new cache line padding that aligns the given data.
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
    #[inline(always)]
    fn from(data: &[T]) -> Self {
        assert!(data.len() <= N);

        // Construct a new cache pad and copy the supplied data into it.
        let mut pad = CachePad::new([T::default(); N]);
        pad.copy_from_slice(data);

        // Make the data coherent so it is seen correctly by the SE and the CPU.
        unsafe {
            flush_data_cache_line(&pad as *const CachePad<T, N> as usize);
            barrier::dsb(barrier::ISH);
        }

        pad
    }
}

impl<T: Copy + Default, const N: usize> Deref for CachePad<T, { N }> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Copy + Default, const N: usize> DerefMut for CachePad<T, { N }> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn align_up(address: usize, align: usize) -> usize {
    assert!(align.is_power_of_two(), "`align` must be a power of two");

    let align_mask = align - 1;
    if address & align_mask == 0 {
        address // Already aligned.
    } else {
        (address | align_mask) + 1
    }
}

/// Flushes the data cache line starting from the given address.
pub unsafe fn flush_data_cache_line(line: usize) {
    asm!("dc civac, {}", in(reg) line);
}

/// Flushes the entire data cache area that is covered by the given object.
pub unsafe fn flush_data_cache<T>(obj: &T, size: usize)
where
    T: ?Sized,
{
    let start = &obj as *const _ as usize;
    let end = align_up(start + size, DATA_CACHE_LINE_SIZE);

    barrier::dmb(barrier::SY);
    for line in (start..end).step_by(DATA_CACHE_LINE_SIZE) {
        // Flush all cache lines within the given area.
        flush_data_cache_line(line);
    }
    barrier::dmb(barrier::SY);
}
