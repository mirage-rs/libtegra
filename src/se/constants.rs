// Security Engine buffer sizes.
pub const CTX_BUFFER_SIZE: usize = 1072;
pub const CTX_DRBG_BUFFER_SIZE: usize = 2112;

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
    pub const KEY_SLOT_PART_COUNT: usize = 2;
    pub const KEY_SLOT_COUNT: usize = 16;

    pub const KEY_SIZE_256: usize = 32;
    pub const KEY_SIZE_192: usize = 24;
    pub const KEY_SIZE_128: usize = 16;

    pub const BLOCK_SIZE: usize = 16;

    pub const MIN_KEY_SIZE: usize = 16;
    pub const MAX_KEY_SIZE: usize = 32;

    pub const IV_SIZE: usize = 16;
}

/// Data sizes related to RNG.
pub mod rng {
    pub const IV_SIZE: usize = 16;
    pub const DT_SIZE: usize = 16;
    pub const KEY_SIZE: usize = 16;

    pub const SEED_SIZE: usize = IV_SIZE + DT_SIZE + KEY_SIZE;

    pub const RESEED_INTERVAL: u32 = 70_000 + 1;
}

/// Data sizes related to RSA.
pub mod rsa {
    pub const KEY_SLOT_PART_COUNT: usize = 2;
    pub const KEY_SLOT_COUNT: usize = 2;

    pub const MAX_EXP_BIT_SIZE: usize = 2048;
    pub const MAX_EXP_SIZE_32: usize = MAX_EXP_BIT_SIZE >> 5;
    pub const MAX_MOD_BIT_SIZE: usize = 2048;
    pub const MAX_MOD_SIZE_32: usize = MAX_MOD_BIT_SIZE >> 5;

    pub const DIGEST_SIZE_512: usize = 64;
    pub const DIGEST_SIZE_1024: usize = 128;
    pub const DIGEST_SIZE_1536: usize = 192;
    pub const DIGEST_SIZE_2048: usize = 256;
}
