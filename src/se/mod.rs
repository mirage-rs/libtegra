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
//! ## AES
//!
//! The AES APIs of the Security Engine expose the primitives to do cryptographic operations
//! with various block and cipher modes. These operations source their keys from a secure
//! hardware key table cache, whose contents can be individually manipulated. Further, access
//! can be restricted under a specific set of circumstances to provide a secure operating
//! environment without leaking keys to the outside.
//!
//! - [`SecurityEngine::fill_aes_keyslot`]
//!
//! - [`SecurityEngine::get_aes_key`]
//!
//! - [`SecurityEngine::clear_aes_keyslot`]
//!
//! - [`SecurityEngine::clear_aes_key_iv`]
//!
//! - [`SecurityEngine::set_encrypted_aes_key`]
//!
//! - [`SecurityEngine::aes_cmac`]
//!
//! - [`SecurityEngine::aes_ecb_encrypt`]
//!
//! - [`SecurityEngine::aes_ecb_decrypt`]
//!
//! - [`SecurityEngine::aes_cbc_encrypt`]
//!
//! - [`SecurityEngine::aes_cbc_decrypt`]
//!
//! - [`SecurityEngine::aes_ctr_encrypt`]
//!
//! - [`SecurityEngine::aes_ctr_decrypt`]
//!
//! ## RSA
//!
//! Similarly to the AES APIs, the Security Engine also features asymmetric encryptions using
//! RSA where a user first feeds the desired exponent and modulus values for the encryption
//! into keyslots and then triggers a modular exponentiation operation.
//!
//! - [`SecurityEngine::fill_rsa_keyslot`]
//!
//! - [`SecurityEngine::clear_rsa_keyslot`]
//!
//! - [`SecurityEngine::rsa_modular_exponentiate`]
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
//! [`SecurityEngine::fill_aes_keyslot`]: struct.SecurityEngine.html#method.fill_aes_keyslot
//! [`SecurityEngine::get_aes_key`]: struct.SecurityEngine.html#method.get_aes_key
//! [`SecurityEngine::clear_aes_keyslot`]: struct.SecurityEngine.html#method.clear_aes_keyslot
//! [`SecurityEngine::clear_aes_key_iv`]: struct.SecurityEngine.html#method.clear_aes_key_iv
//! [`SecurityEngine::set_encrypted_aes_key`]: struct.SecurityEngine.html#method.set_encrypted_aes_key
//! [`SecurityEngine::aes_cmac`]: struct.SecurityEngine.html#method.aes_cmac
//! [`SecurityEngine::aes_ecb_encrypt`]: struct.SecurityEngine.html#method.aes_ecb_encrypt
//! [`SecurityEngine::aes_ecb_decrypt`]: struct.SecurityEngine.html#method.aes_ecb_decrypt
//! [`SecurityEngine::aes_cbc_encrypt`]: struct.SecurityEngine.html#method.aes_cbc_encrypt
//! [`SecurityEngine::aes_cbc_decrypt`]: struct.SecurityEngine.html#method.aes_cbc_decrypt
//! [`SecurityEngine::aes_ctr_encrypt`]: struct.SecurityEngine.html#method.aes_ctr_encrypt
//! [`SecurityEngine::aes_ctr_decrypt`]: struct.SecurityEngine.html#method.aes_ctr_decrypt
//! [`SecurityEngine::fill_rsa_keyslot`]: struct.SecurityEngine.html#method.fill_rsa_keyslot
//! [`SecurityEngine::clear_rsa_keyslot`]: struct.SecurityEngine.html#method.clear_rsa_keyslot
//! [`SecurityEngine::rsa_modular_exponentiate`]: struct.SecurityEngine.html#method.rsa_modular_exponentiate
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

mod aes;
#[allow(dead_code)]
mod constants;
mod core;
#[macro_use]
mod hash;
mod registers;
mod rng;
mod rsa;
mod utils;

use ::core::marker::Sync;

pub use self::core::*;
use crate::arm;
pub use aes::Mode as AesMode;
pub use registers::*;

/// Representation of the Security Engine used for cryptographic operations.
pub struct SecurityEngine {
    registers: *const Registers,
    rsa_keyslot_cache: [rsa::KeyInfo; constants::rsa::KEY_SLOT_COUNT],
}

// Definitions of known SE instances.

impl SecurityEngine {
    /// A pointer to the first Security Engine instance.
    pub const SE1: Self = SecurityEngine {
        registers: SE1_REGISTERS,
        rsa_keyslot_cache: [rsa::KeyInfo::new(); constants::rsa::KEY_SLOT_COUNT],
    };

    #[cfg(feature = "mariko")]
    /// A pointer to the second Security Engine instance.
    ///
    /// NOTE: Only available with the `mariko` feature enabled.
    pub const SE2: Self = SecurityEngine {
        registers: SE2_REGISTERS,
        rsa_keyslot_cache: [rsa::KeyInfo::new(); constants::rsa::KEY_SLOT_COUNT],
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
        for i in 0..constants::aes::KEY_SLOT_COUNT {
            engine.SE_CRYPTO_KEYTABLE_ACCESS_0[i].set(0);
        }

        // Lock access to the RSA key slots.
        for i in 0..constants::rsa::KEY_SLOT_COUNT {
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

    /// Fills a given keyslot with the supplied AES key.
    ///
    /// This must be done prior to any encryptions using this slot.
    pub fn fill_aes_keyslot(&self, slot: u32, key: &[u8]) {
        assert!(slot < constants::aes::KEY_SLOT_COUNT as u32);
        assert_eq!(key.len() % constants::aes::BLOCK_SIZE >> 2, 0);
        assert!(key.len() <= constants::aes::MAX_KEY_SIZE);

        let engine = unsafe { &*self.registers };
        aes::set_key(engine, slot, key)
    }

    /// Copies a previously loaded AES key out of a given keyslot.
    pub fn get_aes_key(&self, slot: u32, key: &mut [u8]) {
        assert!(slot < constants::aes::KEY_SLOT_COUNT as u32);
        assert_eq!(key.len() % constants::aes::BLOCK_SIZE >> 2, 0);
        assert!(key.len() <= constants::aes::MAX_KEY_SIZE);

        let engine = unsafe { &*self.registers };
        aes::get_key(engine, slot, key)
    }

    /// Clears the data out of a given AES keyslot.
    pub fn clear_aes_keyslot(&self, slot: u32) {
        assert!(slot < constants::aes::KEY_SLOT_COUNT as u32);

        let engine = unsafe { &*self.registers };
        aes::clear_keyslot(engine, slot)
    }

    /// Clears the IV out of a given AES keyslot.
    ///
    /// This affects both, the original and the updated IV values.
    pub fn clear_aes_key_iv(&self, slot: u32) {
        assert!(slot < constants::aes::KEY_SLOT_COUNT as u32);

        let engine = unsafe { &*self.registers };
        aes::clear_key_iv(engine, slot)
    }

    /// Loads an encrypted AES key into the given AES keyslot.
    ///
    /// The key in the AES keyslot denoted by `kek_slot` is used to
    /// decrypt the supplied key via AES-ECB before loading it into
    /// the desired `dest_slot` in the key table.
    pub fn set_encrypted_aes_key(
        &self,
        dest_slot: u32,
        kek_slot: u32,
        key: &[u8],
        mode: AesMode,
    ) -> Result<(), OperationError> {
        assert!(dest_slot < constants::aes::KEY_SLOT_COUNT as u32);
        assert!(kek_slot < constants::aes::KEY_SLOT_COUNT as u32);
        assert_eq!(key.len() % constants::aes::BLOCK_SIZE >> 2, 0);
        assert!(key.len() <= constants::aes::MAX_KEY_SIZE);

        let engine = unsafe { &*self.registers };
        aes::set_encrypted_key(engine, dest_slot, kek_slot, key, mode)
    }

    /// Calculates an AES-CMAC from `source` to `destination`.
    pub fn aes_cmac(
        &self,
        slot: u32,
        source: &[u8],
        destination: &mut [u8],
        mode: AesMode,
    ) -> Result<(), OperationError> {
        assert!(slot < constants::aes::KEY_SLOT_COUNT as u32);
        if source.is_empty() {
            return Ok(());
        }

        let engine = unsafe { &*self.registers };
        aes::do_cmac_operation(engine, slot, source, destination, mode)
    }

    /// Encrypts a block of data from `source` to `destination` using AES-ECB.
    pub fn aes_ecb_encrypt(
        &self,
        slot: u32,
        source: &[u8; constants::aes::BLOCK_SIZE],
        destination: &mut [u8; constants::aes::BLOCK_SIZE],
        mode: AesMode,
    ) -> Result<(), OperationError> {
        assert!(slot < constants::aes::KEY_SLOT_COUNT as u32);
        if source.is_empty() {
            return Ok(());
        }

        let engine = unsafe { &*self.registers };
        aes::do_ecb_operation(engine, true, slot, source, destination, mode)
    }

    /// Decrypts a block of data from `source` to `destination` using AES-ECB.
    pub fn aes_ecb_decrypt(
        &self,
        slot: u32,
        source: &[u8; constants::aes::BLOCK_SIZE],
        destination: &mut [u8; constants::aes::BLOCK_SIZE],
        mode: AesMode,
    ) -> Result<(), OperationError> {
        assert!(slot < constants::aes::KEY_SLOT_COUNT as u32);
        if source.is_empty() {
            return Ok(());
        }

        let engine = unsafe { &*self.registers };
        aes::do_ecb_operation(engine, false, slot, source, destination, mode)
    }

    /// Encrypts data from `source` to `destination` using AES-CBC.
    pub fn aes_cbc_encrypt(
        &self,
        slot: u32,
        source: &[u8],
        destination: &mut [u8],
        iv: &[u8; constants::aes::BLOCK_SIZE],
        mode: AesMode,
    ) -> Result<(), OperationError> {
        assert!(slot < constants::aes::KEY_SLOT_COUNT as u32);
        assert_eq!(source.len() % constants::aes::KEY_SLOT_COUNT, 0);
        assert_eq!(source.len(), destination.len());
        if source.is_empty() {
            return Ok(());
        }

        let engine = unsafe { &*self.registers };
        aes::do_cbc_operation(engine, true, slot, source, destination, iv, mode)
    }

    /// Decrypts data from `source` to `destination` using AES-CBC.
    pub fn aes_cbc_decrypt(
        &self,
        slot: u32,
        source: &[u8],
        destination: &mut [u8],
        iv: &[u8; constants::aes::BLOCK_SIZE],
        mode: AesMode,
    ) -> Result<(), OperationError> {
        assert!(slot < constants::aes::KEY_SLOT_COUNT as u32);
        assert_eq!(source.len() % constants::aes::KEY_SLOT_COUNT, 0);
        assert_eq!(source.len(), destination.len());
        if source.is_empty() {
            return Ok(());
        }

        let engine = unsafe { &*self.registers };
        aes::do_cbc_operation(engine, false, slot, source, destination, iv, mode)
    }

    /// Encrypts data from `source` to `destination` using AES-CTR.
    pub fn aes_ctr_encrypt(
        &self,
        slot: u32,
        source: &[u8],
        destination: &mut [u8],
        iv: &[u8; constants::aes::BLOCK_SIZE],
        mode: AesMode,
    ) -> Result<(), OperationError> {
        assert!(slot < constants::aes::KEY_SLOT_COUNT as u32);
        assert_eq!(source.len(), destination.len());
        if source.is_empty() {
            return Ok(());
        }

        let engine = unsafe { &*self.registers };
        aes::do_ctr_operation(engine, true, slot, source, destination, iv, mode)
    }

    /// Decrypts data from `source` to `destination` using AES-CTR.
    pub fn aes_ctr_decrypt(
        &self,
        slot: u32,
        source: &[u8],
        destination: &mut [u8],
        iv: &[u8; constants::aes::BLOCK_SIZE],
        mode: AesMode,
    ) -> Result<(), OperationError> {
        assert!(slot < constants::aes::KEY_SLOT_COUNT as u32);
        assert_eq!(source.len(), destination.len());
        if source.is_empty() {
            return Ok(());
        }

        let engine = unsafe { &*self.registers };
        aes::do_ctr_operation(engine, false, slot, source, destination, iv, mode)
    }

    /// Clears all data out of a given RSA keyslot.
    pub fn clear_rsa_keyslot(&mut self, slot: u32) {
        assert!(slot < constants::rsa::KEY_SLOT_COUNT as u32);

        // Clear the cached keyslot data.
        self.rsa_keyslot_cache[slot as usize].reset();

        let engine = unsafe { &*self.registers };
        rsa::clear_keyslot(engine, slot)
    }

    /// Fills the RSA keyslot using the supplied modulus and exponent data.
    ///
    /// This must be done prior to any RSA operations using the selected slot.
    pub fn fill_rsa_keyslot(&mut self, slot: u32, modulus: &[u8], exponent: &[u8]) {
        assert!(slot < constants::rsa::KEY_SLOT_COUNT as u32);
        assert!(modulus.len() <= constants::rsa::SIZE);
        assert!(exponent.len() <= constants::rsa::SIZE);

        // Cache the infos about the key slot.
        self.rsa_keyslot_cache[slot as usize].update(modulus.len(), exponent.len());

        let engine = unsafe { &*self.registers };
        rsa::fill_keyslot(engine, slot, modulus, exponent)
    }

    /// Computes the modular exponentiation of `source ^ exponent (mod n)`.
    ///
    /// Exponent and modulus should have already been loaded into a keyslot prior
    /// to calling this method.
    pub fn rsa_modular_exponentiate(
        &self,
        slot: u32,
        source: &[u8],
        destination: &mut [u8],
    ) -> Result<(), OperationError> {
        assert!(slot < constants::rsa::KEY_SLOT_COUNT as u32);
        assert!(source.len() <= constants::rsa::SIZE);
        assert!(destination.len() <= constants::rsa::SIZE);
        if source.is_empty() {
            return Ok(());
        }

        // Prepare the RSA context to work with.
        let rev_source = unsafe {
            // Copy source data and reverse the endianness.
            let mut data = [0; constants::rsa::SIZE];
            for i in 0..source.len() {
                data[source.len() - 1 - i] = source[i];
            }

            // Ensure cache coherency so the SE sees the correct data.
            arm::cache::flush_data_cache(&data, data.len());
            #[cfg(target_arch = "aarch64")]
            cortex_a::barrier::dsb(cortex_a::barrier::ISH);

            data
        };

        let engine = unsafe { &*self.registers };
        rsa::encrypt(
            engine,
            &self.rsa_keyslot_cache[slot as usize],
            slot,
            &rev_source[..source.len()],
            destination,
        )
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
}

// Safety: The driver waits until previous operations have completed unconditionally
// before querying a new one.
unsafe impl Sync for SecurityEngine {}
