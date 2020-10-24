//! Abstractions over the Tegra X1 Clock and Reset Controller functionality.
//!
//! See Chapter 5 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! The Clock and Reset (CAR) block contains all the logic needed to control most of the clocks
//! and resets to the Tegra X1 device. It takes care of most clock source programming and most
//! of the clock dividers.
//!
//! # Clocks
//!
//! Clocks provide the logic to control the operation frequency of hardware blocks, which
//! include certain SoC devices, such as system buses, timers, the RTC, GPIO and CoreSight.
//! The [`Clock`] structure provides an abstraction layer to control the power-on (POR) and
//! reset states of each block individually.
//!
//! Instances of [`Clock`] should never be created by the user, as malformed configurations bring
//! a certain risk of damaging the hardware. That's why the structure holds globally-accessible
//! constant instances of itself, which represent most of the known device clocks without
//! non-standard programming guidelines.
//!
//! [`Clock`]: struct.Clock.html

mod clock;
mod registers;

pub use crate::car::clock::*;
pub use crate::car::registers::*;
