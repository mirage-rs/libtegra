//! Driver for the Tegra X1 Security Engine.
//!
//! # Description
//!
//! The Security Engine is responsible for performing, cryptographic operations
//! in a secure, hardware-based environment. Communication is done over [`LinkedList`]s,
//! a data structure defined by the SE interface, which provides I/O buffers to
//! operate on.
//!
//! ## Hardware Operations
//!
//! As mentioned previously, the Security Engine performs hardware-based operations
//! to process DMA buffers of data. These operations can be performed through the
//! [`trigger_operation`] function.
//!
//! As this is a low-level interface that should only be used if no other possibility
//! exists, there are higher-level wrappers around commonly used operations available:
//!
//! ```no_run
//! // TODO
//! ```
//!
//! [`LinkedList`]: struct.LinkedList.html
//! [`trigger_operation`]: fn.trigger_operation.html

use ::core::marker::Sync;

use byteorder::{BE, ByteOrder};

pub use self::core::*;
pub use registers::*;

use constants::*;

#[allow(dead_code)]
mod constants;
mod core;
mod registers;

/// Representation of the Security Engine used for cryptographic operations.
pub struct SecurityEngine {
    /// A pointer to the Security Engine [`Registers`].
    ///
    /// [`Registers`]: struct.Registers.html
    registers: *const Registers,
}

// Definitions of known SE instances.

impl SecurityEngine {
    /// A pointer to the first Security Engine instance.
    pub const SE1: Self = SecurityEngine {
        registers: SE1_REGISTERS,
    };

    #[cfg(feature = "mariko")]
    /// A pointer to the second Security Engine instance.
    ///
    /// NOTE: Only available with the `mariko` feature enabled.
    pub const SE2: Self = SecurityEngine {
        registers: SE2_REGISTERS,
    };
}

impl SecurityEngine {
    /// Peforms a hardware operation to generate the Storage Root Key (SRK).
    ///
    /// NOTE: Different entropy sources will lead to different results.
    pub fn srk(&self) -> Result<(), OperationError> {
        let engine = unsafe { &*self.registers };

        // Prepare an SRK operation.
        engine.SE_CONFIG_0.set(alg::ENC_RNG | alg::DEC_NOP | destination::SRK);

        // Configure the RNG.
        engine.SE_RNG_CONFIG_0.set(drbg_mode::FORCE_RESEED | drbg_src::LFSR);

        // Construct the Security Engine Linked Lists.
        let source_ll = LinkedList::default();
        let mut destination_ll = LinkedList::default();

        // Kick off the hardware operation.
        start_normal_operation(engine, &source_ll, &mut destination_ll, 0)?;

        Ok(())
    }

    /// Performs a hashing operation on a given buffer of data using the SHA256 algorithm.
    pub fn sha256(&self, source: &[u8]) -> Result<[u8; 32], OperationError> {
        let engine = unsafe { &*self.registers };
        let mut output = [0; 32];

        // Prepare a SHA256 hardware operation.
        engine.SE_CONFIG_0.set(enc_mode::SHA256_ENC | alg::SHA | destination::HASHREG);
        engine.SE_SHA_CONFIG_0.set(1);
        engine.SE_SHA_MSG_LENGTH_0[0].set((source.len() << 3) as u32);
        engine.SE_SHA_MSG_LENGTH_0[1].set(0);
        engine.SE_SHA_MSG_LENGTH_0[2].set(0);
        engine.SE_SHA_MSG_LENGTH_0[3].set(0);
        engine.SE_SHA_MSG_LEFT_0[0].set((source.len() << 3) as u32);
        engine.SE_SHA_MSG_LEFT_0[1].set(0);
        engine.SE_SHA_MSG_LEFT_0[2].set(0);
        engine.SE_SHA_MSG_LEFT_0[3].set(0);

        // Construct the Security Engine Linked Lists.
        let source_ll = LinkedList::from(source);
        let mut destination_ll = LinkedList::default();

        // Kick off the hardware operation.
        start_normal_operation(engine, &source_ll, &mut destination_ll, 0)?;

        // Read and copy back the resulting hash.
        for i in 0..8 {
            BE::write_u32(&mut output[i << 2..], engine.SE_HASH_RESULT_0[i].get());
        }

        Ok(output)
    }
}

unsafe impl Sync for SecurityEngine {}
