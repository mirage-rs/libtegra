//! Abstractions over the Tegra X1 Clock and Reset Controller functionality.
//!
//! See Chapter 5 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! The Clock and Reset (CAR) block contains all the logic needed to control
//! most of the clocks and resets to the Tegra X1 device. It takes care of
//!most clock source programming, and most of the clock dividers.
//!
//! ## Clocks
//!
//! [`Clock`]s provide the logic to control so-called blocks, which include
//! certain SoC devices, such as system buses, timers, the RTC, GPIO and
//! CoreSight. The [`Clock`] structure provides an abstraction layer to
//! control the power-on (POR) and reset states of this block, allowing for
//! enabling, disabling or rebooting certain hardware devices.
//!
//! Instances of [`Clock`] should never be created by the user, as malformed
//! configurations bring a certain risk of damaging the hardware. That's why
//! the structure holds globally-accessible constant instances of itself,
//! which represent most of the known device clocks without exceptional control
//! routines.
//!
//! ```no_run
//! use libtegra::{car::Clock, i2c::I2C_5_REGISTERS};
//!
//! // Certain logic, such as I2C, hangs the SoC if the respective
//! // clock is down. That's why it needs to be enabled first.
//!
//! // Enable the I2C 5 controller.
//! Clock::I2C_5.enable();
//!
//! // Some I2C logic here...
//! let registers = unsafe { &*I2C_5_REGISTERS };
//! // (If the device clock wasn't enabled, the following line would hang the SoC.)
//! while (registers.I2C_I2C_STATUS_0.get() & 0x100) != 0 {
//!     // This bus is busy.
//! }
//!
//! // We're done. Disable the controller.
//! Clock::I2C_5.disable();
//! ```
//!
//! [`Clock`]: struct.Clock.html

pub use clock::Clock;
pub use registers::*;

mod clock;
mod registers;
