/// Aligns `address` up to the boundary denoted by `align`.
pub fn align_up(address: usize, align: usize) -> usize {
    align_down(address + align - 1, align)
}

/// Aligns `address` down to the boundary denoted by `align`.
pub fn align_down(address: usize, align: usize) -> usize {
    assert!(align.is_power_of_two(), "`align` must be a power of two");
    address & !(align - 1)
}
