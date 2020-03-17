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

mod registers;
