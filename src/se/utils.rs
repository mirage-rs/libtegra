use cortex_a::barrier;

use super::LinkedList;

const DATA_CACHE_LINE_SIZE: usize = 64;

fn align_up(address: usize, align: usize) -> usize {
    assert!(align.is_power_of_two(), "`align` must be a power of two");

    let align_mask = align - 1;
    if addr & align_mask == 0 {
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
pub unsafe fn flush_data_cache<T>(obj: &T, size: usize) {
    let start = obj as *const T as usize;
    let end = align_up(start + size, DATA_CACHE_LINE_SIZE);

    barrier::dmb(barrier::SY);
    for line in (start..end).step_by(DATA_CACHE_LINE_SIZE) {
        // Flush all cache lines within the given area.
        flush_data_cache_line(line);
    }
    barrier::dmb(barrier::SY);
}
