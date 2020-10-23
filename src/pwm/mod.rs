//! Driver for the PWM Controller of the Tegra X1.
//!
//! See Chapter 39 in the Tegra X1 Technical Reference Manual
//! for details.
//!
//! # Description
//! //! The Pulse Width Modulator (PWM) Controller is a four-channel
//! frequency divider whose pulse width varies. Each channel has
//! a programmable frequency divider and a programmable pulse
//! width generator.

mod registers;

#[cfg(feature = "hal")]
mod hal;

pub use crate::pwm::registers::*;

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

        controller
            .PWM_CONTROLLER_PWM_CSR_0
            .modify(PWM_CONTROLLER_PWM_CSR_0::ENB::SET);
    }

    /// Disables pulse generation through this channel.
    pub fn disable(&self) {
        let controller = unsafe { &*self.registers };

        controller
            .PWM_CONTROLLER_PWM_CSR_0
            .modify(PWM_CONTROLLER_PWM_CSR_0::ENB::CLEAR);
    }

    /// Configures the pulse width of the channel.
    ///
    /// The argument is the desired duty cycle as a float value,
    /// representing a percentage ranging from 0.0 (0%) to 1.0 (100%).
    pub fn set_pulse_width(&self, duty: f32) -> Result<(), ()> {
        let controller = unsafe { &*self.registers };

        if duty < 0.0 || duty > 1.0 {
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
