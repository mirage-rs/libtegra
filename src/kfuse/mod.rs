//! Driver for the Tegra X1 for the HDCP KFUSE Controller.
//!
//! See Chapter 27.5 in the Tegra X1 Technical Reference Manual
//! for details.
//!
//! # Description
//!
//! The KFUSE stores upstream and downstream HDCP keys which
//! are used by the HDMI module.
//!
//! ## Initialization
//!
//! In certain cases, the KFUSE is busy initializing data and
//! thus cannot be used, which may lead to unexpected errors
//! (for example with the TSEC). The [`wait_until_ready`]
//! function serves the purpose of preventing that from
//! happening.
//!
//! ## Key Copying
//!
//! HDCP keys are stored encrypted in the KFUSE module. Before
//! starting HDCP, software must copy the keys (576 bytes) from
//! the KFUSE to the HDMI registers.
//!
//! ```no_run
//! use libtegra::kfuse;
//!
//! // Copy the keys.
//! let mut buffer = [0; kfuse::MAX_WORD_LENGTH];
//! kfuse::read(&mut buffer).unwrap();
//!
//! // Process them...
//! ```
//!
//! [`wait_until_ready`]: fn.wait_until_ready.html

use crate::car::Clock;

pub use registers::*;

mod registers;

/// Maximum word length of a KFUSE address.
pub const MAX_WORD_LENGTH: usize = 144;

/// Waits until KFUSE is ready to be used.
///
/// NOTE: This function expects the KFUSE [`Clock`] to be up
/// before calling to it.
///
/// [`Clock`]: ../car/struct.Clock.html
pub fn wait_until_ready() -> Result<(), ()> {
    let kfuse = unsafe { &*REGISTERS };

    while !kfuse.KFUSE_STATE_0.is_set(KFUSE_STATE_0::DONE) {
        // Wait for KFUSE to finish initialization and verification of data.
    }

    // Ensure that CRC passes.
    if !kfuse.KFUSE_STATE_0.is_set(KFUSE_STATE_0::CRCPASS) {
        return Err(());
    }

    Ok(())
}

/// Reads the encrypted HDCP keys from the KFUSE into a buffer.
///
/// [`Clock`]: ../car/struct.Clock.html
#[optimize(size)]
pub fn read(buffer: &mut [u32]) -> Result<(), ()> {
    let kfuse = unsafe { &*REGISTERS };
    let mut result = Err(());

    if buffer.len() > MAX_WORD_LENGTH {
        return result;
    }

    Clock::KFUSE.enable();

    if wait_until_ready().is_ok() {
        kfuse.KFUSE_KEYADDR_0.modify(KFUSE_KEYADDR_0::AUTOINC::SET);
        for i in buffer.iter_mut() {
            *i = kfuse.KFUSE_KEYS_0.get();
        }

        result = Ok(());
    }

    Clock::KFUSE.disable();

    result
}
