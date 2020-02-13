//! Abstractions over the HDCP KFUSE Control registers of the Tegra X1.
//!
//! See Chapter 27.9 in the Tegra X1 Technical Reference Manual
//! for details.

use register::{mmio::*, register_bitfields, register_structs};

use crate::memory_map::KFUSE;

/// A pointer to the KFUSE register block that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = KFUSE as *const Registers;

register_bitfields! {
    u32,

    /// Bitfields of the `KFUSE_STATE_0` register.
    pub KFUSE_STATE_0 [
        SOFTRESET OFFSET(31) NUMBITS(1) [],

        /// Set this bit to abort decoding in progress, then wait for STATE=IDLE.
        STOP OFFSET(25) NUMBITS(1) [],

        /// Set this bit to restart decoding, similar to deassertion of reset.
        RESTART OFFSET(24) NUMBITS(1) [],

        /// After DONE, this bit indicates CRC pass/fail.
        ///
        /// NOTE: This bit is read-only.
        CRCPASS OFFSET(17) NUMBITS(1) [],

        /// This bit is set when decoding is completed.
        ///
        /// NOTE: This bit is read-only.
        DONE OFFSET(16) NUMBITS(1) [],

        /// If any `ERR_*` bits are set, this contains the offset of the first errored block.
        ///
        /// NOTE: These bits are read-only.
        ERRBLOCK OFFSET(8) NUMBITS(6) [],

        /// Counter of the current block during decoding. Useful for debugging.
        ///
        /// NOTE: These bits are read-only.
        CURBLOCK OFFSET(0) NUMBITS(6) []
    ],

    /// Bitfields of the `KFUSE_ERRCOUNT_0` register.
    pub KFUSE_ERRCOUNT_0 [
        /// Number of uncorrectable errors.
        ERR_FATAL OFFSET(24) NUMBITS(7) [],

        /// Number of correctable 3-bit errors.
        ERR_3 OFFSET(16) NUMBITS(7) [],

        /// Number of correctable 2-bit errors.
        ERR_2 OFFSET(8) NUMBITS(7) [],

        /// Number of correctable 1-bit errors.
        ERR_1 OFFSET(0) NUMBITS(7) []
    ],

    /// Bitfields of the `KFUSE_KEYADDR_0` register.
    pub KFUSE_KEYADDR_0 [
        /// When set, `ADDR` is incremented by 1 after each read of `KEYS`.
        AUTOINC OFFSET(16) NUMBITS(1) [],

        /// Word address (0..144).
        ADDR OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `KFUSE_KEYS_0` register.
    pub KFUSE_KEYS_0 [
        /// The contents of the register.
        DATA OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `KFUSE_PD_0` register.
    pub KFUSE_PD_0 [
        /// Indicates the final value of PD going to the fuse macro.
        ///
        /// NOTE: This bit is read-only.
        STATUS OFFSET(1) NUMBITS(1) [
            KfuseCellPwrup = 0,
            KfuseCellPwrdown = 1
        ],

        /// Asserts kfusecell_pd pin to kfusecell macro and puts it in low leakage mode.
        CTRL OFFSET(0) NUMBITS(1) [
            Powerup = 0,
            Powerdown = 1
        ]
    ]
}

register_structs! {
    /// Representation of the HDCP KFUSE registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => _reserved0: [ReadWrite<u8>; 0x24]),
        (0x24 => pub KFUSE_PD_0: ReadWrite<u32, KFUSE_PD_0::Register>),
        (0x28 => _reserved1: [ReadWrite<u8>; 0x58]),
        (0x80 => pub KFUSE_STATE_0: ReadWrite<u32, KFUSE_STATE_0::Register>),
        (0x84 => pub KFUSE_ERRCOUNT_0: ReadOnly<u32, KFUSE_ERRCOUNT_0::Register>),
        (0x88 => pub KFUSE_KEYADDR_0: ReadWrite<u32, KFUSE_KEYADDR_0::Register>),
        (0x8C => pub KFUSE_KEYS_0: ReadOnly<u32, KFUSE_KEYS_0::Register>),
        (0x90 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x90]);
