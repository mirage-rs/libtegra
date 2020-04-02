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

pub use registers::*;
use register::mmio::ReadWrite;

mod registers;

type ChannelRegister = ReadWrite<u32, PWM_CONTROLLER_PWM_CSR_0::Register>;

pub struct PwmChannel {
    register: ChannelRegister,
}

impl PwmChannel {
    pub fn new(register: ChannelRegister) -> Self {
        Self { register }
    }

    pub fn enable(&mut self) {
        self.register.modify(PWM_CONTROLLER_PWM_CSR_0::ENB::SET)
    }

    pub fn disable(&mut self) {
        self.register.modify(PWM_CONTROLLER_PWM_CSR_0::ENB::CLEAR)
    }

    pub fn set_duty(&mut self, duty: f32) {
        todo!();
    }

    pub fn get_duty(&mut self) -> f32 {
        todo!()
    }
}
