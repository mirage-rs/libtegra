//! Abstractions over the Fixed Time Base Registers.
//!
//! See Chapter 8.8 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! The USEC_CFG/CNTR_1US registers provide a fixed time base (in microseconds)
//! to be used by the rest of the system regardless of the clk_m frequency
//! (i.e., 12 MHz or 38.4 MHz).

use tock_registers::{register_bitfields, register_structs, registers::*};

use crate::memory_map::TMR;

/// A pointer to the Fixed Time Base register block that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = (TMR + 0x10) as *const Registers;

register_bitfields! {
    u32,

    /// Bitfields of the `TIMERUS_USEC_CFG_0` register.
    pub TIMERUS_CNTR_1US_0 [
        /// Elapsed time in microseconds.
        HIGH_VALUE OFFSET(16) NUMBITS(16) [],

        /// Elapsed time in microseconds.
        LOW_VALUE OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `TIMERUS_USEC_CFG_0` register.
    pub TIMERUS_USEC_CFG_0 [
        /// Microsecond dividend. (n+1)
        USEC_DIVIDEND OFFSET(8) NUMBITS(8) [
            /// 12 MHz clk_m frequency - 0x00 / 0x0B.
            ClkMFreq12 = 0x00,
            /// 38.4 Mhz clk_m frequency - 0x04 / 0xBF.
            ClkMFreq384 = 0x04
        ],

        /// Microsecond divisor. (n+1)
        USEC_DIVISOR OFFSET(0) NUMBITS(8) [
            /// 12 MHz clk_m frequency - 0x00 / 0x0B.
            ClkMFreq12 = 0x0B,
            /// 38.4 Mhz clk_m frequency - 0x04 / 0xBF.
            ClkMFreq384 = 0xBF
        ]
    ],

    /// Bitfields of the `TIMERUS_CNTR_FREEZE_0` register.
    pub TIMERUS_CNTR_FREEZE_0 [
        /// Whether timers should be freezed when COP is in debug state.
        DBG_FREEZE_COP OFFSET(4) NUMBITS(1) [
            /// No freeze.
            NoFreeze = 0,
            /// Freeze.
            Freeze = 1
        ],

        /// Whether timers should be freezed when CPU3 is in debug state.
        DBG_FREEZE_CPU3 OFFSET(3) NUMBITS(1) [
            /// No freeze.
            NoFreeze = 0,
            /// Freeze.
            Freeze = 1
        ],

        /// Whether timers should be freezed when CPU2 is in debug state.
        DBG_FREEZE_CPU2 OFFSET(2) NUMBITS(1) [
            /// No freeze.
            NoFreeze = 0,
            /// Freeze.
            Freeze = 1
        ],

        /// Whether timers should be freezed when CPU1 is in debug state.
        DBG_FREEZE_CPU1 OFFSET(1) NUMBITS(1) [
            /// No freeze.
            NoFreeze = 0,
            /// Freeze.
            Freeze = 1
        ],

        /// Whether timers should be freezed when CPU0 is in debug state.
        DBG_FREEZE_CPU0 OFFSET(0) NUMBITS(1) [
            /// No freeze.
            NoFreeze = 0,
            /// Freeze.
            Freeze = 1
        ]
    ]
}

register_structs! {
    /// Representation of the Fixed Time Base registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => pub TIMERUS_CNTR_1US_0: ReadWrite<u32, TIMERUS_CNTR_1US_0::Register>),
        (0x04 => pub TIMERUS_USEC_CFG_0: ReadWrite<u32, TIMERUS_USEC_CFG_0::Register>),
        (0x08 => _reserved: [ReadWrite<u32>; 0xD]),
        (0x3C => pub TIMERUS_CNTR_FREEZE_0: ReadWrite<u32, TIMERUS_CNTR_FREEZE_0::Register>),
        (0x40 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x40]);
