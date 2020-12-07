//! Driver for the Tegra X1 Security Engine.
//!
//! # Description
//!
//! The Security Engine is responsible for performing cryptographic operations in
//! a hardware-based environment. Communication with hardware is done over a variety
//! of supported interfaces, including memory and hash registers. It supports most
//! modern cryptographic algorithms and is suited for secure operation as the engine
//! has the ability to restrict access to it from ARM TrustZone clients.
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

        // Configure the hardware to restrict SE access from non-TZ clients.
        engine
            .SE_SE_SECURITY_0
            .modify(SE_SE_SECURITY_0::SOFT_SECURITY::Secure);

        // Confirm the write.
        engine.SE_SE_SECURITY_0.get();
    }

    /// Unlocks the SE.
    ///
    /// All clients can access the SE afterwards.
    pub fn unlock(&self) {
        let engine = unsafe { &*self.registers };

        // Configure the hardware to extend SE access to all clients.
        engine
            .SE_SE_SECURITY_0
            .modify(SE_SE_SECURITY_0::SOFT_SECURITY::NonSecure);

        // Confirm the write.
        engine.SE_SE_SECURITY_0.get();
    }

    /// Locks down the Security Engine per-key.
    pub fn lock_per_key(&self) {
        let engine = unsafe { &*self.registers };

        // Configure the hardware for lockdown.
        engine.SE_CRYPTO_SECURITY_PERKEY_0.set(0);
        engine.SE_CRYPTO_SECURITY_PERKEY_0.get(); // Confirm the write.

        engine.SE_RSA_SECURITY_PERKEY_0.set(0);
        engine.SE_RSA_SECURITY_PERKEY_0.get(); // Confirm the write.

        engine
            .SE_SE_SECURITY_0
            .modify(SE_SE_SECURITY_0::PERKEY_SECURITY::Secure);
        engine.SE_SE_SECURITY_0.get(); // Confirm the write.
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
        engine.SE_SE_SECURITY_0.modify(
            SE_SE_SECURITY_0::HARD_SECURITY::Secure
                + SE_SE_SECURITY_0::ENGINE_DISABLE::Disable
                + SE_SE_SECURITY_0::PERKEY_SECURITY::Secure
                + SE_SE_SECURITY_0::SOFT_SECURITY::Secure,
        );
    }

    /// Initializes the RNG (Random Numer Generator).
    ///
    /// Calling this function is a prerequisite for all functions that use random
    /// input sources to perform cryptographic operations.
    pub fn initialize_rng(&self) -> Result<(), OperationError> {
        let engine = unsafe { &*self.registers };

        // Lock the entropy source.
        engine.SE_RNG_SRC_CONFIG_0.set(
            SE_RNG_SRC_CONFIG_0::RO_ENTROPY_SOURCE::SET
                + SE_RNG_SRC_CONFIG_0::RO_ENTROPY_SOURCE_LOCK::SET,
        );

        // Set the reseed interval to force a reseed every 70.000 blocks.
        engine.SE_RNG_RESEED_INTERVAL_0.set(70_001);

        // Configure the hardware to force DRBG instantiation.
        engine.SE_CONFIG_0.set(
            SE_CONFIG_0::ENC_MODE::Aes128
                + SE_CONFIG_0::DEC_MODE::Aes128
                + SE_CONFIG_0::ENC_ALG::Rng
                + SE_CONFIG_0::DEC_ALG::Nop
                + SE_CONFIG_0::DESTINATION::Memory,
        );

        engine.SE_CRYPTO_CONFIG_0.set(
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
            .set(SE_RNG_CONFIG_0::SOURCE::Entropy + SE_RNG_CONFIG_0::MODE::ForceInstantiation);

        // Only process a single RNG block to trigger DRBG initialization.
        engine.SE_CRYPTO_LAST_BLOCK_0.set(0);

        // Construct dummy Security Engine Linked Lists.
        let buffer = [0; aes::BLOCK_SIZE as usize];
        let source_ll = LinkedList::from(buffer.as_ref());
        let mut destination_ll = LinkedList::default();

        // Kick off the hardware operation.
        start_normal_operation(engine, &source_ll, &mut destination_ll)
    }

    /// Uses the RNG to fill the given buffer with random bytes.
    pub fn generate_random(&self, output: &mut [u8]) -> Result<(), OperationError> {
        // Opt out if the buffer has no capacity for data.
        if output.len() == 0 {
            return Ok(());
        }

        let engine = unsafe { &*self.registers };

        // Determine the amount of blocks to generate.
        let nblocks = output.len() / aes::BLOCK_SIZE as usize;
        let aligned_size = nblocks * aes::BLOCK_SIZE as usize;
        let _fractional = output.len() - aligned_size;

        // Configure the hardware to do RNG encryption.
        engine.SE_CONFIG_0.set(
            SE_CONFIG_0::ENC_MODE::Aes128
                + SE_CONFIG_0::DEC_MODE::Aes128
                + SE_CONFIG_0::ENC_ALG::Rng
                + SE_CONFIG_0::DEC_ALG::Nop
                + SE_CONFIG_0::DESTINATION::Memory,
        );

        engine.SE_CRYPTO_CONFIG_0.set(
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
            .set(SE_RNG_CONFIG_0::SOURCE::Entropy + SE_RNG_CONFIG_0::MODE::Normal);

        // Generate all the aligned blocks first.
        if aligned_size > 0 {
            // Load in the number of blocks to generate.
            engine.SE_CRYPTO_LAST_BLOCK_0.set((nblocks as u32) - 1);

            // Construct the Security Engine Linked Lists.
            let source_ll = LinkedList::default();
            let mut destination_ll = LinkedList::from(&output[..aligned_size]);

            // Execute the operation.
            start_normal_operation(engine, &source_ll, &mut destination_ll)?;
        }

        // TODO: Add support for unaligned blocks.
        Ok(())
    }

    /// Fills a given key slot with a random key generated by the RNG.
    pub fn set_random_key(&self, slot: u32) -> Result<(), OperationError> {
        let engine = unsafe { &*self.registers };

        // Configure the hardware to output to the keytable.
        engine.SE_CONFIG_0.set(
            SE_CONFIG_0::ENC_MODE::Aes128
                + SE_CONFIG_0::DEC_MODE::Aes128
                + SE_CONFIG_0::ENC_ALG::Rng
                + SE_CONFIG_0::DEC_ALG::Nop
                + SE_CONFIG_0::DESTINATION::KeyTable,
        );

        engine.SE_CRYPTO_CONFIG_0.set(
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
            .set(SE_RNG_CONFIG_0::SOURCE::Entropy + SE_RNG_CONFIG_0::MODE::Normal);

        // Configure the keytable to be the low words of the key.
        engine.SE_CRYPTO_KEYTABLE_DST_0.set(
            SE_CRYPTO_KEYTABLE_DST_0::DST_WORD_QUAD::Keys03
                + SE_CRYPTO_KEYTABLE_DST_0::KEY_INDEX.val(slot),
        );

        // Only process a single RNG block to trigger DRBG initialization.
        engine.SE_CRYPTO_LAST_BLOCK_0.set(0);

        // Execute the operation to generate a random chunk of the key.
        start_normal_operation(engine, &LinkedList::default(), &mut LinkedList::default())?;

        // Configure the keytable to be the high words of the key.
        engine.SE_CRYPTO_KEYTABLE_DST_0.set(
            SE_CRYPTO_KEYTABLE_DST_0::DST_WORD_QUAD::Keys47
                + SE_CRYPTO_KEYTABLE_DST_0::KEY_INDEX.val(slot),
        );

        // Execute the operation to generate random chunk of the key.
        start_normal_operation(engine, &LinkedList::default(), &mut LinkedList::default())?;
    }

    /// Performs a hardware operation to generate the Storage Root Key (SRK).
    ///
    /// NOTE: Different entropy sources will lead to different results.
    pub fn generate_srk(&self) -> Result<(), OperationError> {
        let engine = unsafe { &*self.registers };

        // Configure the hardware to do RNG encryption.
        engine.SE_CONFIG_0.set(
            SE_CONFIG_0::ENC_MODE::Aes128
                + SE_CONFIG_0::DEC_MODE::Aes128
                + SE_CONFIG_0::ENC_ALG::Rng
                + SE_CONFIG_0::DEC_ALG::Nop
                + SE_CONFIG_0::DESTINATION::Srk,
        );

        engine.SE_CRYPTO_CONFIG_0.set(
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
            .set(SE_RNG_CONFIG_0::SOURCE::Entropy + SE_RNG_CONFIG_0::MODE::ForceReseed);

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
