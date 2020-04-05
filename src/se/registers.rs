//! Abstractions over the Tegra X1 Security Engine registers.

use register::mmio::ReadWrite;

use crate::memory_map::SE;

/// A pointer to the SE register block that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = SE as *const Registers;

// TODO: Bitfields.

// TODO: Convert this struct to register_structs! format.

// FIXME: Are really all of them ReadWrite?

/// Representation of the Security Engine registers.
#[allow(non_snake_case)]
#[repr(C)]
pub struct Registers {
    pub SE_SE_SECURITY_0: ReadWrite<u32>,
    pub SE_TZRAM_SECURITY_0: ReadWrite<u32>,
    pub SE_OPERATION_0: ReadWrite<u32>,
    pub SE_INT_ENABLE_0: ReadWrite<u32>,
    pub SE_INT_STATUS_0: ReadWrite<u32>,
    pub SE_CONFIG_0: ReadWrite<u32>,
    pub SE_IN_LL_ADDR_0: ReadWrite<u32>,
    pub SE_IN_CUR_BYTE_ADDR_0: ReadWrite<u32>,
    pub SE_IN_CUR_LL_ID_0: ReadWrite<u32>,
    pub SE_OUT_LL_ADDR_0: ReadWrite<u32>,
    pub SE_OUT_CUR_BYTE_ADDR_0: ReadWrite<u32>,
    pub SE_OUT_CUR_LL_ID_0: ReadWrite<u32>,
    pub SE_HASH_RESULT_0: [ReadWrite<u32>; 0x10],
    pub SE_CTX_SAVE_CONFIG_0: ReadWrite<u32>,
    _reserved0: [ReadWrite<u8>; 0x18C],
    pub SE_SHA_CONFIG_0: ReadWrite<u32>,
    pub SE_SHA_MSG_LENGTH_0: [ReadWrite<u32>; 0x4],
    pub SE_SHA_MSG_LEFT_0: [ReadWrite<u32>; 0x4],
    _reserved1: [ReadWrite<u8>; 0x5C],
    pub SE_CRYPTO_SECURITY_PERKEY_0: ReadWrite<u32>,
    pub SE_CRYPTO_KEYTABLE_ACCESS_0: [ReadWrite<u32>; 0x10],
    _reserved2: [ReadWrite<u8>; 0x40],
    pub SE_CRYPTO_CONFIG_0: ReadWrite<u32>,
    pub SE_CRYPTO_LINEAR_CTR_0: [ReadWrite<u32>; 0x4],
    pub SE_CRYPTO_LAST_BLOCK_0: ReadWrite<u32>,
    pub SE_CRYPTO_KEYTABLE_ADDR_0: ReadWrite<u32>,
    pub SE_CRYPTO_KEYTABLE_DATA_0: ReadWrite<u32>,
    _reserved3: [ReadWrite<u8>; 0xC],
    pub SE_CRYPTO_KEYTABLE_DST_0: ReadWrite<u32>,
    _reserved4: [ReadWrite<u8>; 0xC],
    pub SE_RNG_CONFIG_0: ReadWrite<u32>,
    pub SE_RNG_SRC_CONFIG_0: ReadWrite<u32>,
    pub SE_RNG_RESEED_INTERVAL_0: ReadWrite<u32>,
    _reserved5: [ReadWrite<u8>; 0xB4],
    pub SE_RSA_CONFIG_0: ReadWrite<u32>,
    pub SE_RSA_KEY_SIZE_0: ReadWrite<u32>,
    pub SE_RSA_EXP_SIZE_0: ReadWrite<u32>,
    pub SE_RSA_SECURITY_PERKEY_0: ReadWrite<u32>,
    pub SE_RSA_KEYTABLE_ACCESS_0: [ReadWrite<u32>; 0x2],
    _reserved6: [ReadWrite<u8>; 0x8],
    pub SE_RSA_KEYTABLE_ADDR_0: ReadWrite<u32>,
    pub SE_RSA_KEYTABLE_DATA_0: ReadWrite<u32>,
    pub SE_RSA_OUTPUT_0: [ReadWrite<u32>; 0x40],
    _reserved7: [ReadWrite<u8>; 0x2D8],
    pub SE_STATUS_0: ReadWrite<u32>,
    pub SE_ERR_STATUS_0: ReadWrite<u32>,
    pub SE_MISC_0: ReadWrite<u32>,
    pub SE_SPARE_0: ReadWrite<u32>,
    pub SE_ENTROPY_DEBUG_COUNTER_0: ReadWrite<u32>,
    _reserved8: [ReadWrite<u8>; 0x17EC],
}
