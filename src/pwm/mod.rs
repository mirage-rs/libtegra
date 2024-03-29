//! Driver for the Pulse Width Modulator Controller of the Tegra X1.
//!
//! See Chapter 39 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! The Pulse Width Modulator (PWM) Controller is a four-channel frequency divider whose pulse
//! width varies. Each channel has a programmable frequency divider and a programmable pulse
//! width generator.

mod registers;

#[cfg(feature = "hal")]
mod hal;

pub use crate::pwm::registers::*;
use tock_registers::interfaces::*;

/// Representation of a Pulse Width Modulator channel.
///
/// NOTE: It is expected that the PWM [`Clock`] is brought up before using any of the functionality
/// of a channel.
///
/// [`Clock`]: ../car/struct.Clock.html
#[derive(Debug)]
pub struct PwmChannel {
    // A pointer to the PWM device registers in memory.
    registers: *const Registers,
}

// Definitions of known PWM channels.
impl PwmChannel {
    /// Representation of the PWM 0 channel.
    pub const PWM_0: Self = PwmChannel {
        registers: PWM_0_REGISTERS,
    };

    /// Representation of the PWM 1 channel.
    pub const PWM_1: Self = PwmChannel {
        registers: PWM_1_REGISTERS,
    };

    /// Representation of the PWM 2 channel.
    pub const PWM_2: Self = PwmChannel {
        registers: PWM_2_REGISTERS,
    };

    /// Representation of the PWM 3 channel.
    pub const PWM_3: Self = PwmChannel {
        registers: PWM_3_REGISTERS,
    };
}

impl PwmChannel {
    /// Enables pulse generation mechanism of this channel.
    pub fn enable(&self) {
        let controller = unsafe { &*self.registers };

        controller
            .PWM_CONTROLLER_PWM_CSR_0
            .modify(PWM_CONTROLLER_PWM_CSR_0::ENB::SET);
    }

    /// Disables pulse generation mechanism of this channel.
    pub fn disable(&self) {
        let controller = unsafe { &*self.registers };

        controller
            .PWM_CONTROLLER_PWM_CSR_0
            .modify(PWM_CONTROLLER_PWM_CSR_0::ENB::CLEAR);
    }

    /// Configures the pulse width of the channel.
    ///
    /// The argument is the desired duty cycle as a float value, representing a percentage
    /// ranging from 0.0 (0%) to 1.0 (100%).
    pub fn set_pulse_width(&self, duty: f32) -> Result<(), ()> {
        let controller = unsafe { &*self.registers };

        if !(0.0..=1.0).contains(&duty) {
            return Err(());
        }

        controller
            .PWM_CONTROLLER_PWM_CSR_0
            .modify(PWM_CONTROLLER_PWM_CSR_0::PWM_0.val((duty * 256.0) as u32));

        Ok(())
    }

    /// Returns the current duty cycle.
    ///
    /// The returned duty cycle is a float value from 0.0 (0%) to 1.0 (100%).
    pub fn get_duty(&self) -> f32 {
        let controller = unsafe { &*self.registers };

        let pulse_width = controller
            .PWM_CONTROLLER_PWM_CSR_0
            .read(PWM_CONTROLLER_PWM_CSR_0::PWM_0) as f32;
        pulse_width / 256.0
    }

    /// Returns the max duty cycle that is possible to set.
    pub fn get_max_duty(&self) -> f32 {
        1.0
    }
}
