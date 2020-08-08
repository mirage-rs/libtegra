//! Constants for interfacing with the Security Engine.

// Security Engine buffer sizes.
pub const CTX_BUFFER_SIZE: usize = 1072;
pub const CTX_DRBG_BUFFER_SIZE: u32 = 2112;

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
