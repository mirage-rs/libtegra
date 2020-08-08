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

use byteorder::{ByteOrder, BE};

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
    /// Locks the SE down.
    ///
    /// Only TrustZone clients can access the SE anymore afterwards.
    pub fn lock(&self) {
        let engine = unsafe { &*self.registers };

        let mut value = engine.SE_SE_SECURITY_0.get();
        value |= 0 << 16; // Clear no-lockdown bit.
        engine.SE_SE_SECURITY_0.set(value);

        // Confirm the write.
        engine.SE_SE_SECURITY_0.get();
    }

    /// Unlocks the SE.
    ///
    /// All clients can access the SE afterwards.
    pub fn unlock(&self) {
        let engine = unsafe { &*self.registers };

        let mut value = engine.SE_SE_SECURITY_0.get();
        value |= 1 << 16; // Set no-lockdown bit.
        engine.SE_SE_SECURITY_0.set(value);

        // Confirm the write.
        engine.SE_SE_SECURITY_0.get();
    }

    /// Locks down the Security Engine per-key.
    pub fn lock_per_key(&self) {
        let engine = unsafe { &*self.registers };

        engine.SE_CRYPTO_SECURITY_PERKEY_0.set(0);

        let mut value = engine.SE_SE_SECURITY_0.get();
        value |= 0 << 2; // Set per-key secure setting.
        engine.SE_SE_SECURITY_0.set(value);

        // Confirm the write.
        engine.SE_SE_SECURITY_0.get();
    }

    /// Disables the Security Engine.
    pub fn disable(&self) {
        let engine = unsafe { &*self.registers };

        // Lock access to the AES key slots.
        for i in 0..aes::KEY_SLOT_COUNT {
            engine.SE_CRYPTO_KEYTABLE_ACCESS_0[i].set(0);
        }

        // Lock access to the RSA key slots.
        for i in 0..rsa::KEY_SLOT_COUNT {
            engine.SE_RSA_KEYTABLE_ACCESS_0[i].set(0);
        }

        // Set Per Key lockdown.
        self.lock_per_key();

        // Adjust lockdown settings.
        engine.SE_SE_SECURITY_0.set(2);
    }

    /// Initializes the RNG (Random Numer Generator).
    ///
    /// Calling this function is a prerequisite for all functions that use random
    /// input sources to perform cryptographic operations.
    pub fn initialize_rng(&self) -> Result<(), OperationError> {
        let engine = unsafe { &*self.registers };

        // Lock the entropy source.
        engine.SE_RNG_SRC_CONFIG_0.modify(
            SE_RNG_SRC_CONFIG_0::RO_ENTROPY_SOURCE::SET
                + SE_RNG_SRC_CONFIG_0::RO_ENTROPY_SOURCE_LOCK::SET,
        );

        // Set the reseed interval to force a reseed every 70.000 blocks.
        engine.SE_RNG_RESEED_INTERVAL_0.set(70_001);

        // Configure the hardware to force DRBG instantiation.
        engine.SE_CONFIG_0.modify(
            SE_CONFIG_0::ENC_MODE::Aes128
                + SE_CONFIG_0::DEC_MODE::Aes128
                + SE_CONFIG_0::ENC_ALG::Rng
                + SE_CONFIG_0::DEC_ALG::Nop
                + SE_CONFIG_0::DESTINATION::Memory,
        );

        engine.SE_CRYPTO_CONFIG_0.modify(
            SE_CRYPTO_CONFIG_0::MEMIF::Ahb
                + SE_CRYPTO_CONFIG_0::CTR_CNTN::CLEAR
                + SE_CRYPTO_CONFIG_0::KEYSCH_BYPASS::CLEAR
                + SE_CRYPTO_CONFIG_0::CORE_SEL::Encrypt
                + SE_CRYPTO_CONFIG_0::IV_SELECT::Original
                + SE_CRYPTO_CONFIG_0::VCTRAM_SEL::Memory
                + SE_CRYPTO_CONFIG_0::INPUT_SEL::Random
                + SE_CRYPTO_CONFIG_0::XOR_POS::Bypass
                + SE_CRYPTO_CONFIG_0::HASH_ENB::CLEAR,
        );

        // Configure the RNG to use Entropy as source.
        engine
            .SE_RNG_CONFIG_0
            .modify(SE_RNG_CONFIG_0::SOURCE::Entropy + SE_RNG_CONFIG_0::MODE::ForceInstantiation);

        // Only process a single RNG block to trigger DRBG initialization.
        engine.SE_CRYPTO_LAST_BLOCK_0.set(0);

        // Construct dummy Security Engine Linked Lists.
        let buffer = [0; aes::BLOCK_SIZE as usize];
        let source_ll = LinkedList::from(buffer.as_ref());
        let mut destination_ll = LinkedList::default();

        // Kick off the hardware operation.
        start_normal_operation(engine, &source_ll, &mut destination_ll)
    }

    /// Performs a hardware operation to generate the Storage Root Key (SRK).
    ///
    /// NOTE: Different entropy sources will lead to different results.
    pub fn generate_srk(&self) -> Result<(), OperationError> {
        let engine = unsafe { &*self.registers };

        // Configure the hardware to do RNG encryption.
        engine.SE_CONFIG_0.modify(
            SE_CONFIG_0::ENC_MODE::Aes128
                + SE_CONFIG_0::DEC_MODE::Aes128
                + SE_CONFIG_0::ENC_ALG::Rng
                + SE_CONFIG_0::DEC_ALG::Nop
                + SE_CONFIG_0::DESTINATION::Srk,
        );

        engine.SE_CRYPTO_CONFIG_0.modify(
            SE_CRYPTO_CONFIG_0::MEMIF::Ahb
                + SE_CRYPTO_CONFIG_0::CTR_CNTN::CLEAR
                + SE_CRYPTO_CONFIG_0::KEYSCH_BYPASS::CLEAR
                + SE_CRYPTO_CONFIG_0::CORE_SEL::Encrypt
                + SE_CRYPTO_CONFIG_0::IV_SELECT::Original
                + SE_CRYPTO_CONFIG_0::VCTRAM_SEL::Memory
                + SE_CRYPTO_CONFIG_0::INPUT_SEL::Random
                + SE_CRYPTO_CONFIG_0::XOR_POS::Bypass
                + SE_CRYPTO_CONFIG_0::HASH_ENB::CLEAR,
        );

        // Configure the RNG to use Entropy as source.
        engine
            .SE_RNG_CONFIG_0
            .modify(SE_RNG_CONFIG_0::SOURCE::Entropy + SE_RNG_CONFIG_0::MODE::ForceReseed);

        // Only process a single RNG block to trigger DRBG initialization.
        engine.SE_CRYPTO_LAST_BLOCK_0.set(0);

        // Construct dummy Security Engine Linked Lists.
        let source_ll = LinkedList::default();
        let mut destination_ll = LinkedList::default();

        // Kick off the hardware operation.
        start_normal_operation(engine, &source_ll, &mut destination_ll)
    }

    /// Performs a hashing operation on a given buffer of data using the SHA256 algorithm.
    pub fn calculate_sha256(&self, source: &[u8]) -> Result<[u8; 32], OperationError> {
        let engine = unsafe { &*self.registers };
        let mut output = [0; 32];

        // Configure the hardware to perform a SHA256 hashing operation.
        engine.SE_CONFIG_0.modify(
            SE_CONFIG_0::HASH_MODE::Sha256
                + SE_CONFIG_0::DEC_MODE::Aes128
                + SE_CONFIG_0::ENC_ALG::Sha
                + SE_CONFIG_0::DEC_ALG::Nop
                + SE_CONFIG_0::DESTINATION::HashReg,
        );
        engine
            .SE_SHA_CONFIG_0
            .modify(SE_SHA_CONFIG_0::HW_INIT_HASH::SET);

        // Set the message size.
        engine.SE_SHA_MSG_LENGTH_0[0].set((source.len() << 3) as u32);
        engine.SE_SHA_MSG_LENGTH_0[1].set(0);
        engine.SE_SHA_MSG_LENGTH_0[2].set(0);
        engine.SE_SHA_MSG_LENGTH_0[3].set(0);

        // Set the message remaining size.
        engine.SE_SHA_MSG_LEFT_0[0].set((source.len() << 3) as u32);
        engine.SE_SHA_MSG_LEFT_0[1].set(0);
        engine.SE_SHA_MSG_LEFT_0[2].set(0);
        engine.SE_SHA_MSG_LEFT_0[3].set(0);

        // Construct the Security Engine Linked Lists.
        let source_ll = LinkedList::from(source);
        let mut destination_ll = LinkedList::default();

        // Kick off the operation.
        start_normal_operation(engine, &source_ll, &mut destination_ll)?;

        // Read and copy back the resulting hash.
        for i in 0..8 {
            BE::write_u32(&mut output[i << 2..], engine.SE_HASH_RESULT_0[i].get());
        }

        Ok(output)
    }
}

// Safety: The driver waits until previous operations have completed unconditionally
// before querying a new one.
unsafe impl Sync for SecurityEngine {}
