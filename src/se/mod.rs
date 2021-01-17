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
//! # Hardware Operations
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
//! # Cryptographic APIs
//!
//! The following cryptographic APIs are exposed by the Security Engine and can be used
//! from TrustZone clients with no regrets:
//!
//! ## Access Management
//!
//! In secure systems, it is undesirable to have untrusted pieces of code access the Security
//! Engine to perform arbitrary cryptographic operations. For that reason, the SE offers
//! various features for locking its functionality down to Secure World clients which run in
//! the ARM TrustZone. Great flexibility is given in how this is achieved: Full lockdown,
//! lockdown per key, TZRAM lockdown, restriction of context save operations, full Security
//! Engine shutdown. While not directly cryptographic, still an important aspect of retaining
//! cryptographic security in applications.
//!
//! - [`SecurityEngine::lock`]
//!
//! - [`SecurityEngine::unlock`]
//!
//! - [`SecurityEngine::lock_per_key`]
//!
//! - [`SecurityEngine::lock_tzram`]
//!
//! - [`SecurityEngine::lock_context_save`]
//!
//! ## RNG
//!
//! The Security Engine implements a Random Number Generator which can be used to implement
//! secure random data generation with a high entropy for applications.
//!
//! - [`SecurityEngine::initialize_rng`]
//!
//! - [`SecurityEngine::generate_random`]
//!
//! - [`SecurityEngine::set_random_key`]
//!
//! - [`SecurityEngine::generate_srk`]
//!
//! ## Hashing
//!
//! The Security Engine supports various hashing algorithms from the SHA1 and SHA2 family
//! to calculate a fixed-size digest over a given buffer of data using the following methods.
//!
//! - [`SecurityEngine::calculate_sha1`]
//!
//! - [`SecurityEngine::calculate_sha224`]
//!
//! - [`SecurityEngine::calculate_sha256`]
//!
//! - [`SecurityEngine::calculate_sha384`]
//!
//! - [`SecurityEngine::calculate_sha512`]
//!
//! [`trigger_operation`]: fn.trigger_operation.html
//! [`SecurityEngine::initialize_rng`]: struct.SecurityEngine.html#method.initialize_rng
//! [`SecurityEngine::generate_random`]: struct.SecurityEngine.html#method.generate_random
//! [`SecurityEngine::set_random_key`]: struct.SecurityEngine.html#method.set_random_key
//! [`SecurityEngine::generate_srk`]: struct.SecurityEngine.html#method.generate_srk
//! [`SecurityEngine::calculate_sha1`]: struct.SecurityEngine.html#method.calculate_sha1
//! [`SecurityEngine::calculate_sha224`]: struct.SecurityEngine.html#method.calculate_sha224
//! [`SecurityEngine::calculate_sha256`]: struct.SecurityEngine.html#method.calculate_sha256
//! [`SecurityEngine::calculate_sha384`]: struct.SecurityEngine.html#method.calculate_sha384
//! [`SecurityEngine::calculate_sha512`]: struct.SecurityEngine.html#method.calculate_sha512
//! [`SecurityEngine::lock`]: struct.SecurityEngine.html#method.lock
//! [`SecurityEngine::unlock`]: struct.SecurityEngine.html#method.unlock
//! [`SecurityEngine::lock_per_key`]: struct.SecurityEngine.html#method.lock_per_key
//! [`SecurityEngine::lock_tzram`]: struct.SecurityEngine.html#method.lock_tzram
//! [`SecurityEngine::lock_context_save`]: struct.SecurityEngine.html#method.lock_context_save

#[allow(dead_code)]
mod constants;
mod core;
#[macro_use]
mod hash;
mod registers;
mod rng;
#[cfg(target_arch = "aarch64")]
mod utils;

use ::core::marker::Sync;

pub use self::core::*;
pub use registers::*;

use constants::*;

/// Representation of the Security Engine used for cryptographic operations.
pub struct SecurityEngine {
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
    /// Locks the SE down for use from the Secure World.
    ///
    /// Only TrustZone clients can access the SE anymore afterwards.
    pub fn lock(&self) {
        let engine = unsafe { &*self.registers };

        // Configure the hardware to restrict SE access from non-TZ clients.
        engine
            .SE_SE_SECURITY_0
            .modify(SE_SE_SECURITY_0::SOFT_SECURITY::Secure);
        engine.SE_SE_SECURITY_0.get(); // Confirm the write.
    }

    /// Unlocks the SE for use from the Non-Secure World.
    ///
    /// All clients can access the SE afterwards.
    pub fn unlock(&self) {
        let engine = unsafe { &*self.registers };

        // Configure the hardware to extend SE access to all clients.
        engine
            .SE_SE_SECURITY_0
            .modify(SE_SE_SECURITY_0::SOFT_SECURITY::NonSecure);
        engine.SE_SE_SECURITY_0.get(); // Confirm the write.
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

    /// Locks down the TZRAM to access from Secure World only.
    pub fn lock_tzram(&self) {
        let engine = unsafe { &*self.registers };

        // Configure the hardware to restrict access to TZRAM.
        engine
            .SE_TZRAM_SECURITY_0
            .modify(SE_TZRAM_SECURITY_0::LOCKDOWN::Secure);
        engine.SE_TZRAM_SECURITY_0.get(); // Confirm the write.
    }

    /// Locks down the Security Engine to restrict context save operations to
    /// Secure World clients.
    ///
    /// NOTE: This is available for SE1 and SE2, but only on T210B01 either way.
    #[cfg(feature = "mariko")]
    pub fn lock_context_save(&self) {
        let engine = unsafe { &*self.registers };

        // Configure the hardware to only allow context save operations from Secure World.
        engine
            .SE_SE_SECURITY_0
            .modify(SE_SE_SECURITY_0::TZ_CONTEXT_SAVE_LOCK::Secure);
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
        rng::initialize(engine)
    }

    /// Uses the RNG to fill the given buffer with random bytes.
    pub fn generate_random(&self, output: &mut [u8]) -> Result<(), OperationError> {
        // Opt out if the buffer has no capacity for data.
        if output.is_empty() {
            return Ok(());
        }

        let engine = unsafe { &*self.registers };
        rng::generate_random(engine, output)
    }

    /// Fills a given key slot with a random key generated by the RNG.
    pub fn set_random_key(&self, slot: u32) -> Result<(), OperationError> {
        let engine = unsafe { &*self.registers };
        rng::set_random_key(engine, slot)
    }

    /// Performs a hardware operation to generate the Storage Root Key (SRK).
    ///
    /// NOTE: Different entropy sources will lead to different results.
    pub fn generate_srk(&self) -> Result<(), OperationError> {
        let engine = unsafe { &*self.registers };
        rng::generate_srk(engine)
    }

    // Generate the Hashing API.
    gen_sha_impl!(1, 20);
    gen_sha_impl!(224);
    gen_sha_impl!(256);
    gen_sha_impl!(384);
    gen_sha_impl!(512);

    #[cfg(not(target_arch = "aarch64"))]
    unsafe fn trigger_context_save_operation(
        &self,
        source: &[u8],
        destination: &mut [u8],
    ) -> Result<(), OperationError> {
        assert_eq!(source.len() % aes::BLOCK_SIZE, 0);
        assert_eq!(destination.len() % aes::BLOCK_SIZE, 0);

        let engine = &*self.registers;

        // Construct the Security Engine Linked Lists.
        let source_ll = LinkedList::from(source);
        let mut destination_ll = LinkedList::from(&destination[..]);

        // Execute the context save operation.
        start_context_save_operation(engine, &source_ll, &mut destination_ll)
    }

    #[cfg(target_arch = "aarch64")]
    unsafe fn trigger_context_save_operation(
        &self,
        source: &[u8],
        destination: &mut [u8],
    ) -> Result<(), OperationError> {
        use cortex_a::barrier;

        let engine = &*self.registers;

        // Make sure that the destination buffer does not exceed 1 AES block in size.
        if destination.len() > aes::BLOCK_SIZE {
            return Err(OperationError::MalformedBuffer);
        }

        // Construct a cache padding for output data and make it coherent.
        let pad = {
            let pad = utils::CachePad::<u8, { aes::BLOCK_SIZE }>::new([0; aes::BLOCK_SIZE]);
            if destination.len() > 0 {
                utils::flush_data_cache_line(&pad as *const _ as usize);
                barrier::dsb(barrier::ISH);
            }

            pad.into_inner()
        };

        // Make the source data coherent, if necessary.
        if source.len() > 0 {
            utils::flush_data_cache(source, source.len());
            barrier::dsb(barrier::ISH);
        }

        // Construct the Security Engine Linked Lists.
        let source_ll = LinkedList::from(source);
        let mut destination_ll = LinkedList::from(&pad[..]);

        // Execute the context save operation.
        start_context_save_operation(engine, &source_ll, &mut destination_ll)?;

        // Copy back the resulting output data, if necessary.
        if destination.len() > 0 {
            barrier::dsb(barrier::ISH);
            utils::flush_data_cache_line(pad.as_ptr() as usize);
            barrier::dsb(barrier::ISH);

            for (x, y) in destination.iter_mut().zip(pad.iter().cycle()) {
                *x = *y;
            }
        }

        Ok(())
    }
}

// Safety: The driver waits until previous operations have completed unconditionally
// before querying a new one.
unsafe impl Sync for SecurityEngine {}
