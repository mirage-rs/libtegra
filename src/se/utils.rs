use crate::arm;
use crate::se::constants::*;
use crate::se::core::*;
use crate::se::registers::*;

pub fn trigger_single_block_operation(
    engine: &Registers,
    source: &[u8],
    destination: &mut [u8],
) -> Result<(), OperationError> {
    assert!(source.len() <= aes::BLOCK_SIZE);
    assert!(destination.len() <= aes::BLOCK_SIZE);
    if source.is_empty() && destination.is_empty() {
        return Ok(());
    }

    // Create a cache-aligned buffer wrapping the data.
    let pad = unsafe {
        // Create the pad and copy in the data.
        let pad = arm::cache::CachePad::<u8, { aes::BLOCK_SIZE }>::from(source);

        // Make the data coherent so it is seen correctly by CPU and SE.
        arm::cache::flush_data_cache(&pad[..], aes::BLOCK_SIZE);
        #[cfg(target_arch = "aarch64")]
        cortex_a::barrier::dsb(cortex_a::barrier::ISH);

        pad.into_inner()
    };

    // Configure a hardware operation on a single block.
    engine.SE_CRYPTO_LAST_BLOCK_0.set(0);

    // Prepare the linked lists and kick off the operation.
    let source_ll = LinkedList::from(&pad[..]);
    let mut destination_ll = LinkedList::from(&pad[..]);
    start_normal_operation(engine, &source_ll, &mut destination_ll)?;

    // Ensure data cache coherency so that CPU sees the correct data.
    unsafe {
        #[cfg(target_arch = "aarch64")]
        cortex_a::barrier::dsb(cortex_a::barrier::ISH);
        arm::cache::flush_data_cache(&pad[..], aes::BLOCK_SIZE);
        #[cfg(target_arch = "aarch64")]
        cortex_a::barrier::dsb(cortex_a::barrier::ISH);
    }

    // Copy back the result to the output buffer.
    {
        let len = destination.len();
        destination.copy_from_slice(&pad[..len]);
    }

    Ok(())
}
