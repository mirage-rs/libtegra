//! Abstractions over the PWM Controller registers of the Tegra X1.
//!
//! See Chapter 39.2 in the Tegra X1 Technical Reference Manual for
//! details.

use register::{mmio::ReadWrite, register_bitfields, register_structs};

use crate::memory_map::PWM;

/// A pointer to the PWM_0 register block that can be accessed by dereferencing it.
pub const PWM_0_REGISTERS: *const Registers = (PWM + 0x00) as *const Registers;
/// A pointer to the PWM_1 register block that can be accessed by dereferencing it.
pub const PWM_1_REGISTERS: *const Registers = (PWM + 0x10) as *const Registers;
/// A pointer to the PWM_2 register block that can be accessed by dereferencing it.
pub const PWM_2_REGISTERS: *const Registers = (PWM + 0x20) as *const Registers;
/// A pointer to the PWM_3 register block that can be accessed by dereferencing it.
pub const PWM_3_REGISTERS: *const Registers = (PWM + 0x30) as *const Registers;

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
        (0x0 => pub PWM_CONTROLLER_PWM_CSR_0: ReadWrite<u32, PWM_CONTROLLER_PWM_CSR_0::Register>),
        (0x4 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x4]);
