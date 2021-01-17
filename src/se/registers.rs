use register::{mmio::*, register_bitfields, register_structs};

use crate::memory_map::SE1;
#[cfg(feature = "mariko")]
use crate::memory_map::SE2;

/// A pointer to the SE1 register block that can be accessed by dereferencing it.
pub const SE1_REGISTERS: *const Registers = SE1 as *const Registers;

#[cfg(feature = "mariko")]
/// A pointer to the SE2 register block that can be accessed by dereferencing it.
///
/// NOTE: Only available with the `mariko` feature enabled.
pub const SE2_REGISTERS: *const Registers = SE2 as *const Registers;

register_bitfields! {
    u32,

    pub SE_SE_SECURITY_0 [
        SOFT_SECURITY OFFSET(16) NUMBITS(1) [
            Secure = 0,
            NonSecure = 1
        ],

        TZ_SOFT_LOCK OFFSET(5) NUMBITS(1) [
            Secure = 0,
            NonSecure = 1
        ],

        TZ_CONTEXT_SAVE_LOCK OFFSET(4) NUMBITS(1) [
            Secure = 0,
            NonSecure = 1
        ],

        PERKEY_SECURITY OFFSET(2) NUMBITS(1) [
            Secure = 0,
            NonSecure = 1
        ],

        ENGINE_DISABLE OFFSET(1) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        HARD_SECURITY OFFSET(0) NUMBITS(1) [
            Secure = 0,
            NonSecure = 1
        ]
    ],

    pub SE_TZRAM_SECURITY_0 [
        LOCKDOWN OFFSET(0) NUMBITS(32) [
            Secure = 0,
            NonSecure = 1
        ]
    ],

    pub SE_OPERATION_0 [
        OPCODE OFFSET(0) NUMBITS(3) [
            Abort = 0,
            Start = 1,
            RestartOut = 2,
            ContextSave = 3,
            RestartIn = 4
        ]
    ],

    pub SE_INT_ENABLE_0 [
        ERR_STAT OFFSET(16) NUMBITS(1) [],

        RESEED_CNTR_EXHAUSTED OFFSET(5) NUMBITS(1) [],

        SE_OP_DONE OFFSET(4) NUMBITS(1) [],

        OUT_DONE OFFSET(3) NUMBITS(1) [],

        OUT_LL_BUF_WR OFFSET(2) NUMBITS(1) [],

        IN_DONE OFFSET(1) NUMBITS(1) [],

        IN_LL_BUF_RD OFFSET(0) NUMBITS(1) []
    ],

    pub SE_INT_STATUS_0 [
        ERR_STAT OFFSET(16) NUMBITS(1) [],

        RESEED_CNTR_EXHAUSTED OFFSET(5) NUMBITS(1) [],

        SE_OP_DONE OFFSET(4) NUMBITS(1) [],

        OUT_DONE OFFSET(3) NUMBITS(1) [],

        OUT_LL_BUF_WR OFFSET(2) NUMBITS(1) [],

        IN_DONE OFFSET(1) NUMBITS(1) [],

        IN_LL_BUF_RD OFFSET(0) NUMBITS(1) []
    ],

    pub SE_CONFIG_0 [
        ENC_MODE OFFSET(24) NUMBITS(8) [
            Aes128 = 0,
            Aes192 = 1,
            Aes256 = 2
        ],

        HASH_MODE OFFSET(24) NUMBITS(8) [
            Sha1 = 1,
            Sha224 = 4,
            Sha256 = 5,
            Sha384 = 6,
            Sha512 = 7
        ],

        DEC_MODE OFFSET(16) NUMBITS(8) [
            Aes128 = 0,
            Aes192 = 1,
            Aes256 = 2
        ],

        ENC_ALG OFFSET(12) NUMBITS(4) [
            Nop = 0,
            Aes = 1,
            Rng = 2,
            Sha = 3,
            Rsa = 4
        ],

        DEC_ALG OFFSET(8) NUMBITS(4) [
            Nop = 0,
            Aes = 1
        ],

        DESTINATION OFFSET(2) NUMBITS(3) [
            Memory = 0,
            HashReg = 1,
            KeyTable = 2,
            Srk = 3,
            RsaReg = 4
        ]
    ],

    pub SE_CTX_SAVE_CONFIG_0 [
        SOURCE OFFSET(29) NUMBITS(3) [
            StickyBits = 0,
            RsaKeyTable = 1,
            AesKeyTable = 2,
            Pka1StickyBits = 3,
            Mem = 4,
            Srk = 6,
            Pka1KeyTable = 7
        ],

        STICKY_WORD_QUAD OFFSET(24) NUMBITS(1) [
            Words03 = 0,
            Words47 = 1
        ],

        RSA_KEY_INDEX OFFSET(16) NUMBITS(2) [
            Slot0Exponent = 0,
            Slot0Modulus = 1,
            Slot1Exponent = 2,
            Slot1Modulus = 3
        ],

        PKA1_WORD_QUAD_H OFFSET(12) NUMBITS(4) [],

        RSA_WORD_QUAD OFFSET(12) NUMBITS(4) [],

        AES_KEY_INDEX OFFSET(8) NUMBITS(4) [],

        PKA1_WORD_QUAD_L OFFSET(0) NUMBITS(4) [],

        AES_WORD_QUAD OFFSET(0) NUMBITS(2) [
            Keys03 = 0,
            Keys47 = 1,
            OriginalIvs = 2,
            UpdatedIvs = 3
        ]
    ],

    pub SE_CTX_SAVE_AUTO_0 [
        CURR_CNT OFFSET(16) NUMBITS(10) [],

        LOCK OFFSET(8) NUMBITS(1) [],

        ENABLE OFFSET(0) NUMBITS(1) []
    ],

    pub SE_SHA_CONFIG_0 [
        HW_INIT_HASH OFFSET(0) NUMBITS(1) []
    ],

    pub SE_CRYPTO_CONFIG_0 [
        MEMIF OFFSET(31) NUMBITS(1) [
            Ahb = 0,
            Mccif = 1
        ],

        KEY_INDEX OFFSET(24) NUMBITS(4) [],

        CTR_CNTN OFFSET(11) NUMBITS(8) [],

        KEYSCH_BYPASS OFFSET(10) NUMBITS(1) [],

        CORE_SEL OFFSET(8) NUMBITS(1) [
            Decrypt = 0,
            Encrypt = 1
        ],

        IV_SELECT OFFSET(7) NUMBITS(1) [
            Original = 0,
            Updated = 1
        ],

        VCTRAM_SEL OFFSET(5) NUMBITS(2) [
            Memory = 0,
            InitAesOut = 2,
            InitPrevMem = 3
        ],

        INPUT_SEL OFFSET(3) NUMBITS(2) [
            Memory = 0,
            Random = 1,
            InitAesOut = 2,
            LinearCtr = 3
        ],

        XOR_POS OFFSET(1) NUMBITS(2) [
            Bypass = 0,
            Top = 3,
            Bottom = 4
        ],

        HASH_ENB OFFSET(0) NUMBITS(1) []
    ],

    pub SE_CRYPTO_KEYTABLE_ADDR_0 [
        KEYIV_KEY_SLOT OFFSET(4) NUMBITS(4) [],

        KEYIV_KEYIV_SEL OFFSET(3) NUMBITS(1) [
            Key = 0,
            Iv = 1
        ],

        KEYIV_IV_SEL OFFSET(2) NUMBITS(1) [
            OriginalIv = 0,
            UpdatedIv = 1
        ],

        KEYIV_WORD OFFSET(0) NUMBITS(4) [
            WordKey0 = 0,
            WordKey1 = 1,
            WordKey2 = 2,
            WordKey3 = 3,
            WordKey4 = 4,
            WordKey5 = 5,
            WordKey6 = 6,
            WordKey7 = 7,

            WordOiv0 = 8,
            WordOiv1 = 9,
            WordOiv2 = 10,
            WordOiv3 = 11,

            WordUiv0 = 12,
            WordUiv1 = 13,
            WordUiv2 = 14,
            WordUiv3 = 16
        ],

        KEYIV_IV_WORD OFFSET(0) NUMBITS(2) [],

        KEYIV_KEY_WORD OFFSET(0) NUMBITS(3) []
    ],

    pub SE_CRYPTO_KEYTABLE_DST_0 [
        KEY_INDEX OFFSET(8) NUMBITS(4) [],

        DST_WORD_QUAD OFFSET(0) NUMBITS(2) [
            Keys03 = 0,
            Keys47 = 1,
            OriginalIv = 2,
            UpdatedIv = 3
        ]
    ],

    pub SE_RNG_CONFIG_0 [
        SOURCE OFFSET(2) NUMBITS(2) [
            Nop = 0,
            Entropy = 1,
            Lfsr = 2
        ],

        MODE OFFSET(0) NUMBITS(2) [
            Normal = 0,
            ForceInstantiation = 1,
            ForceReseed = 2
        ]
    ],

    pub SE_RNG_SRC_CONFIG_0 [
        RO_ENTROPY_DATA_FLUSH OFFSET(8) NUMBITS(1) [],

        RO_ENTROPY_SUBSAMPLE OFFSET(4) NUMBITS(3) [],

        HW_DISABLE_CYA OFFSET(2) NUMBITS(1) [],

        RO_ENTROPY_SOURCE OFFSET(1) NUMBITS(1) [],

        RO_ENTROPY_SOURCE_LOCK OFFSET(0) NUMBITS(1) []
    ],

    pub SE_RSA_CONFIG_0 [
        KEY_SLOT OFFSET(24) NUMBITS(1) []
    ],

    pub SE_RSA_KEYTABLE_ACCESS_0 [
        KEY_USE OFFSET(2) NUMBITS(1) [],

        KEY_UPDATE OFFSET(1) NUMBITS(1) [],

        KEY_READ OFFSET(0) NUMBITS(1) []
    ],

    pub SE_RSA_KEYTABLE_ADDR_0 [
        INPUT_MODE OFFSET(8) NUMBITS(1) [
            FromRegister = 0,
            FromMemory = 1
        ],

        KEY_SLOT OFFSET(7) NUMBITS(1) [],

        ADDR_EXPMOD_SEL OFFSET(6) NUMBITS(1) [
            Exponent = 0,
            Modulus = 1
        ],

        WORD_ADDR OFFSET(0) NUMBITS(6) []
    ],

    pub SE_TZRAM_OPERATION_0 [
        CURR_ADDR OFFSET(16) NUMBITS(16) [],

        BUSY OFFSET(2) NUMBITS(1) [
            No = 0,
            Yes = 1
        ],

        MODE OFFSET(1) NUMBITS(1) [
            Save = 0,
            Restore = 1
        ],

        REQ OFFSET(0) NUMBITS(1) [
            Idle = 0,
            Initiate = 1
        ]
    ],

    pub SE_STATUS_0 [
        MEMIF OFFSET(2) NUMBITS(1) [
            Idle = 0,
            Busy = 1
        ],

        STATE OFFSET(0) NUMBITS(2) [
            Idle = 0,
            Busy = 1,
            WaitOut = 2,
            WaitIn = 3
        ]
    ]
}

register_structs! {
    /// Representation of the Security Engine registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x0000 => pub SE_SE_SECURITY_0: ReadWrite<u32, SE_SE_SECURITY_0::Register>),
        (0x0004 => pub SE_TZRAM_SECURITY_0: ReadWrite<u32, SE_TZRAM_SECURITY_0::Register>),
        (0x0008 => pub SE_OPERATION_0: ReadWrite<u32, SE_OPERATION_0::Register>),
        (0x000C => pub SE_INT_ENABLE_0: ReadWrite<u32, SE_INT_ENABLE_0::Register>),
        (0x0010 => pub SE_INT_STATUS_0: ReadWrite<u32, SE_INT_STATUS_0::Register>),
        (0x0014 => pub SE_CONFIG_0: ReadWrite<u32, SE_CONFIG_0::Register>),
        (0x0018 => pub SE_IN_LL_ADDR_0: ReadWrite<u32>),
        (0x001C => pub SE_IN_CUR_BYTE_ADDR_0: ReadWrite<u32>),
        (0x0020 => pub SE_IN_CUR_LL_ID_0: ReadWrite<u32>),
        (0x0024 => pub SE_OUT_LL_ADDR_0: ReadWrite<u32>),
        (0x0028 => pub SE_OUT_CUR_BYTE_ADDR_0: ReadWrite<u32>),
        (0x002C => pub SE_OUT_CUR_LL_ID_0: ReadWrite<u32>),
        (0x0030 => pub SE_HASH_RESULT_0: [ReadWrite<u32>; 0x10]),
        (0x0070 => pub SE_CTX_SAVE_CONFIG_0: ReadWrite<u32, SE_CTX_SAVE_CONFIG_0::Register>),
        (0x0074 => pub SE_CTX_SAVE_AUTO_0: ReadWrite<u32, SE_CTX_SAVE_AUTO_0::Register>),
        (0x0078 => _reserved0),
        (0x0200 => pub SE_SHA_CONFIG_0: ReadWrite<u32, SE_SHA_CONFIG_0::Register>),
        (0x0204 => pub SE_SHA_MSG_LENGTH_0: [ReadWrite<u32>; 0x4]),
        (0x0214 => pub SE_SHA_MSG_LEFT_0: [ReadWrite<u32>; 0x4]),
        (0x0224 => _reserved1),
        (0x0280 => pub SE_CRYPTO_SECURITY_PERKEY_0: ReadWrite<u32>),
        (0x0284 => pub SE_CRYPTO_KEYTABLE_ACCESS_0: [ReadWrite<u32>; 0x10]),
        (0x02C4 => _reserved2),
        (0x0304 => pub SE_CRYPTO_CONFIG_0: ReadWrite<u32, SE_CRYPTO_CONFIG_0::Register>),
        (0x0308 => pub SE_CRYPTO_LINEAR_CTR_0: [ReadWrite<u32>; 0x4]),
        (0x0318 => pub SE_CRYPTO_LAST_BLOCK_0: ReadWrite<u32>),
        (0x031C => pub SE_CRYPTO_KEYTABLE_ADDR_0: ReadWrite<u32, SE_CRYPTO_KEYTABLE_ADDR_0::Register>),
        (0x0320 => pub SE_CRYPTO_KEYTABLE_DATA_0: ReadWrite<u32>),
        (0x0324 => _reserved3),
        (0x0330 => pub SE_CRYPTO_KEYTABLE_DST_0: ReadWrite<u32, SE_CRYPTO_KEYTABLE_DST_0::Register>),
        (0x0334 => _reserved4),
        (0x0340 => pub SE_RNG_CONFIG_0: ReadWrite<u32, SE_RNG_CONFIG_0::Register>),
        (0x0344 => pub SE_RNG_SRC_CONFIG_0: ReadWrite<u32, SE_RNG_SRC_CONFIG_0::Register>),
        (0x0348 => pub SE_RNG_RESEED_INTERVAL_0: ReadWrite<u32>),
        (0x034C => _reserved5),
        (0x0400 => pub SE_RSA_CONFIG_0: ReadWrite<u32, SE_RSA_CONFIG_0::Register>),
        (0x0404 => pub SE_RSA_KEY_SIZE_0: ReadWrite<u32>),
        (0x0408 => pub SE_RSA_EXP_SIZE_0: ReadWrite<u32>),
        (0x040C => pub SE_RSA_SECURITY_PERKEY_0: ReadWrite<u32>),
        (0x0410 => pub SE_RSA_KEYTABLE_ACCESS_0: [ReadWrite<u32, SE_RSA_KEYTABLE_ACCESS_0::Register>; 0x2]),
        (0x0418 => _reserved6),
        (0x0420 => pub SE_RSA_KEYTABLE_ADDR_0: ReadWrite<u32, SE_RSA_KEYTABLE_ADDR_0::Register>),
        (0x0424 => pub SE_RSA_KEYTABLE_DATA_0: ReadWrite<u32>),
        (0x0428 => pub SE_RSA_OUTPUT_0: [ReadWrite<u32>; 0x40]),
        (0x0528 => _reserved7),
        (0x0540 => pub SE_TZRAM_OPERATION_0: ReadWrite<u32, SE_TZRAM_OPERATION_0::Register>),
        (0x0544 => _reserved8),
        (0x0800 => pub SE_STATUS_0: ReadWrite<u32, SE_STATUS_0::Register>),
        (0x0804 => pub SE_ERR_STATUS_0: ReadWrite<u32>),
        (0x0808 => pub SE_MISC_0: ReadWrite<u32>),
        (0x080C => pub SE_SPARE_0: ReadWrite<u32>),
        (0x0810 => pub SE_ENTROPY_DEBUG_COUNTER_0: ReadWrite<u32>),
        (0x0814 => _reserved9),
        (0x2000 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x2000]);
