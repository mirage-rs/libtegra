//! Constants for interfacing with the Security Engine.

// Security Engine buffer sizes.
pub const CTX_BUFFER_SIZE: usize = 1072;
pub const CTX_DRBG_BUFFER_SIZE: u32 = 2112;

/// Configuration flags for different crypto algorithms.
pub mod alg {
    pub const ENC_NOP: u32 = 0 << 12;
    pub const DEC_NOP: u32 = 0 << 8;
    pub const ENC_AES: u32 = 1 << 12;
    pub const DEC_AES: u32 = 1 << 8;
    pub const ENC_RNG: u32 = 2 << 12;
    pub const ENC_SHA: u32 = 3 << 12;
    pub const ENC_RSA: u32 = 4 << 12;
}

/// Configuration flags for the destination of the crypto output.
pub mod destination {
    pub const MEMORY: u32 = 0 << 2;
    pub const HASHREG: u32 = 1 << 2;
    pub const KEYTAB: u32 = 2 << 2;
    pub const SRK: u32 = 3 << 2;
    pub const RSAREG: u32 = 4 << 2;
}

/// Configuration flags for the encryption mode to be used.
pub mod enc_mode {
    pub const KEY128_ENC: u32 = 0 << 24;
    pub const KEY128_DEC: u32 = 0 << 16;
    pub const KEY192_ENC: u32 = 1 << 24;
    pub const KEY192_DEC: u32 = 1 << 16;
    pub const KEY256_ENC: u32 = 2 << 24;
    pub const KEY256_DEC: u32 = 2 << 16;
    pub const SHA1_ENC: u32 = 0 << 24;
    pub const SHA1_DEC: u32 = 0 << 16;
    pub const SHA224_ENC: u32 = 4 << 24;
    pub const SHA224_DEC: u32 = 4 << 16;
    pub const SHA256_ENC: u32 = 5 << 24;
    pub const SHA256_DEC: u32 = 5 << 16;
    pub const SHA384_ENC: u32 = 6 << 24;
    pub const SHA384_DEC: u32 = 6 << 16;
    pub const SHA512_ENC: u32 = 7 << 24;
    pub const SHA512_DEC: u32 = 7 << 16;
}

/// DRBG mode configuration flags for RNG.
pub mod drbg_mode {
    pub const NORMAL: u32 = 0 << 0;
    pub const FORCE_INSTANTION: u32 = 1 << 0;
    pub const FORCE_RESEED: u32 = 2 << 0;
}

/// DRBG source configuration flags for RNG.
pub mod drbg_src {
    pub const NONE: u32 = 0 << 2;
    pub const ENTROPY: u32 = 1 << 2;
    pub const LFSR: u32 = 2 << 2;
}

/// Control opcodes for Security Engine operations.
pub mod opcodes {
    pub const ABORT: u32 = 0;
    pub const START: u32 = 1;
    pub const RESTART_OUT: u32 = 2;
    pub const CTX_SAVE: u32 = 3;
    pub const RESTART_IN: u32 = 4;
}

/// Data sizes related to AES.
pub mod aes {
    pub const KEY_SLOT_COUNT: usize = 16;
    pub const KEY_SIZE_256: u32 = 32;
    pub const KEY_SIZE_192: u32 = 24;
    pub const KEY_SIZE_128: u32 = 16;
    pub const BLOCK_SIZE: u32 = 16;
    pub const MIN_KEY_SIZE: u32 = 16;
    pub const MAX_KEY_SIZE: u32 = 32;
    pub const IV_SIZE: u32 = 16;
}

/// Data sizes related to RNG.
pub mod rng {
    pub const IV_SIZE: u32 = 16;
    pub const DT_SIZE: u32 = 16;
    pub const KEY_SIZE: u32 = 16;

    pub const SEED_SIZE: u32 = IV_SIZE + DT_SIZE + KEY_SIZE;
}

// Security Engine blobs size in bytes.
pub const CTX_SAVE_RSA_KEY_LENGTH: u32 = 1024;
pub const CTX_SAVE_RANDOM_DATA_SIZE: u32 = 16;
pub const CTX_SAVE_STICKY_BITS_SIZE: u32 = 16;
pub const CTX_KNOWN_PATTERN_SIZE: u32 = 16;
pub const CTX_KNOWN_PATTERN_SIZE_WORDS: u32 = CTX_KNOWN_PATTERN_SIZE / 4;

/// Data sizes related to RSA.
pub mod rsa {
    pub const KEY_SLOT_COUNT: usize = 2;
    pub const MAX_EXP_BIT_SIZE: u32 = 2048;
    pub const MAX_EXP_SIZE_32: u32 = MAX_EXP_BIT_SIZE >> 5;
    pub const MAX_MOD_BIT_SIZE: u32 = 2048;
    pub const MAX_MOD_SIZE_32: u32 = MAX_MOD_BIT_SIZE >> 5;

    pub const DIGEST_SIZE_512: u32 = 64;
    pub const DIGEST_SIZE_1024: u32 = 128;
    pub const DIGEST_SIZE_1536: u32 = 192;
    pub const DIGEST_SIZE_2048: u32 = 256;
}
