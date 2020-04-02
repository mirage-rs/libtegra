//! Driver for the PWM Controller of the Tegra X1.
//!
//! See Chapter 39 in the Tegra X1 Technical Reference Manual
//! for details.
//!
//! # Description
//!
//! The Pulse Width Modulator (PWM) Controller is a four-channel
//! frequency divider whose pulse width varies. Each channel has
//! a programmable frequency divider and a programmable pulse
//! width generator.

use crate::car::Clock;

pub use registers::*;

mod registers;

/// Representation of a Pulse Width Modulator channel.
///
/// NOTE: It is expected that the PWM [`Clock`] is up
/// before using any of the functionality of any channel.
///
/// [`Clock`]: ../car/struct.Clock.html
#[derive(Debug)]
pub struct PwmChannel {
    /// A pointer to the PWM [`Registers`] block.
    ///
    /// [`Registers`]: struct.Registers.html
    registers: *const Registers,
}

// Definitions of known PWM channels.

impl PwmChannel {
    pub const PWM_0: Self = PwmChannel {
        registers: PWM_0_REGISTERS,
    };

    pub const PWM_1: Self = PwmChannel {
        registers: PWM_0_REGISTERS,
    };

    pub const PWM_2: Self = PwmChannel {
        registers: PWM_0_REGISTERS,
    };

    pub const PWM_3: Self = PwmChannel {
        registers: PWM_0_REGISTERS,
    };
}

impl PwmChannel {
    /// Enables pulse generation through this channel.
    pub fn enable(&self) {
        let controller = unsafe { &*self.registers };

        controller.PWM_CONTROLLER_PWM_CSR_0.modify(PWM_CONTROLLER_PWM_CSR_0::ENB::SET);
    }

    /// Disables pulse generation through this channel.
    pub fn disable(&self) {
        let controller = unsafe { &*self.registers };

        controller.PWM_CONTROLLER_PWM_CSR_0.modify(PWM_CONTROLLER_PWM_CSR_0::ENB::CLEAR);
    }
}
