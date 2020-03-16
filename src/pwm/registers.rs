//! Abstractions over the PWM Controller registers of the Tegra X1.
//!
//! See Chapter 39.2 in the Tegra X1 Technical Reference Manual for
//! details.

use register::{mmio::ReadWrite, register_bitfields, register_structs};

use crate::memory_map::PWM;

/// A pointer to the PWM register block that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = PWM as *const Registers;

register_bitfields! {
    u32,

    /// Bitfields of the `PWM_CONTROLLER_PWM_CSR_<x>_0` register.
    pub PWM_CONTROLLER_PWM_CSR_0 [
        /// Whether pulse should be enabled with modulator.
        ENB OFFSET(31) NUMBITS(1) [],

        /// Pulse width that needs to be programmed.
        ///
        /// As a rule per thumb:
        ///
        /// 0 = Always low.
        /// N = N / 256 Pulse high.
        PWM_0 OFFSET(16) NUMBITS(15) [],

        /// Frequency divider that needs to be programmed.
        PFM OFFSET(0) NUMBITS(13) []
    ]
}

register_structs! {
    /// Representation of the PWM Controller registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => pub PWM_CONTROLLER_PWM_CSR_0_0: ReadWrite<u32, PWM_CONTROLLER_PWM_CSR_0::Register>),
        (0x04 => _reserved0: [ReadWrite<u8>; 0xC]),
        (0x10 => pub PWM_CONTROLLER_PWM_CSR_1_0: ReadWrite<u32, PWM_CONTROLLER_PWM_CSR_0::Register>),
        (0x14 => _reserved1: [ReadWrite<u8>; 0xC]),
        (0x20 => pub PWM_CONTROLLER_PWM_CSR_2_0: ReadWrite<u32, PWM_CONTROLLER_PWM_CSR_0::Register>),
        (0x24 => _reserved2: [ReadWrite<u8>; 0xC]),
        (0x30 => pub PWM_CONTROLLER_PWM_CSR_3_0: ReadWrite<u32, PWM_CONTROLLER_PWM_CSR_0::Register>),
        (0x34 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x34]);
