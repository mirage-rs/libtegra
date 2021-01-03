//! Abstractions for the Atomic Registers of the Tegra X1.
//!
//! See Chapter 7.4 for more information.
//!
//! To maintain atomicity, the Tegra X1 splits the Atomic Operation
//! process into three phases:
//!
//! - Setup Phase: Parameters in the setup are prepared.
//! - Trigger Phase: The operation is started by configuring a trigger register.
//!   The trigger register specifies the operation to perform,
//!   and the index of the target register.
//! - Result Phase: The results of the operation are made available in the result register.

use crate::memory_map::ATOMICS;
use register::{mmio::*, register_bitfields, register_structs};

/// A pointer to the Atomic register block.
pub const REGISTERS: *const Registers = ATOMICS as *const Registers;

register_bitfields! {
    u32,

    pub TRIGGER [
        CMD OFFSET(0) NUMBITS(4) [
            EXCHANGE = 0x0,
            COMPARE_EXCHANGE = 0b0001,
            INCREMENT = 0b0010,
            DECREMENT = 0b0011,
            GET = 0b0100,
            PUT = 0b0101,
            TEST_AND_SET = 0b0110,
            TEST_AND_CLEAR = 0b0111,
            TEST_AND_INVERT = 0b1000
        ],

        WIDTH64 OFFSET(4) NUMBITS(1) [],

        ID OFFSET(16) NUMBITS(7) []
    ]
}

register_structs! {
    /// Representation of the Atomic registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x0000 => pub ATOMICS_AP0_TRIGGER_0: ReadWrite<u32, TRIGGER::Register>),
        (0x0004 => pub ATOMICS_AP1_TRIGGER_0: ReadWrite<u32, TRIGGER::Register>),
        (0x0008 => _reserved0: [ReadWrite<u8>; 0x3F8]),

        (0x0400 => pub ATOMICS_AP0_SETUP_V_0: [ReadWrite<u32>; 128]),
        (0x0600 => _reserved1: [ReadWrite<u8>; 0x200]),
        (0x0800 => pub ATOMICS_AP0_SETUP_C_0: [ReadWrite<u32>; 128]),
        (0x0A00 => _reserved2: [ReadWrite<u8>; 0x200]),
        (0x0C00 => pub ATOMICS_AP0_RESULT_0: [ReadOnly<u32>; 128]),
        (0x0E00 => _reserved3: [ReadWrite<u8>; 0x200]),

        (0x1000 => pub ATOMICS_AP1_SETUP_V_0: [ReadWrite<u32>; 128]),
        (0x0E00 => _reserved4: [ReadWrite<u8>; 0x200]),
        (0x1400 => pub ATOMICS_AP1_SETUP_C_0: [ReadWrite<u32>; 128]),
        (0x1600 => _reserved5: [ReadWrite<u8>; 0x200]),
        (0x1800 => pub ATOMICS_AP1_RESULT_0: [ReadOnly<u32>; 128]),
        (0x1A00 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x1A00]);
