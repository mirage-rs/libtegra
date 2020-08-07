//! Driver for the Tegra X1 Inter-Integrated Circuit master and slave controllers.
//!
//! See Chapter 35 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! The I²C controller (I2C) implements an I²C 3.0 specification-compliant I²C
//! master and slave controller that supports multiple masters and slaves.
//!
//! There are six instances of the I2C controller within Tegra X1 devices, all
//! of them providing the same functionality. These six instances are represented
//! as publicly exposed constants of the [`I2c`] structure, labeled from `C1`
//! through `C6`.
//!
//! [`I2c`]: struct.I2c.html

pub use device::*;
pub use registers::*;

// TODO: I2C Slave implementation.

mod device;
mod registers;
