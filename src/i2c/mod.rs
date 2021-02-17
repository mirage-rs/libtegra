//! Driver for the Tegra X1 Inter-Integrated Circuit Controller and its Devices.
//!
//! See Chapter 35 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! The I²C controller (I2C) provides a specification-compliant I²C 3.0 driver that
//! supports multiple controllers and devices.
//!
//! There are six instances of the I2C controller within Tegra X1 devices, all of
//! them providing the same functionality. These six instances are represented as
//! publicly exposed constants of the [`I2c`] structure.
//!
//! # Transmission speed
//!
//! By default, the I2C controller instances which are exposed by this module have
//! pre-defined clocks, which are configured for 100KHz data transfer rate over the
//! I²C protocol.
//!
//! If customized clocks are needed, one must create a custom instance of an I2C
//! controller with the desired [`Clock`] settings.
//!
//! # Implementation details
//!
//! The implementation of the I2c controller provides lower-level and higher-level
//! methods for driving the host MMIO driver in Normal Mode with 7-bit addressing
//! transactions. That being said, if one wishes to use Packet Mode (which may be
//! added in the future) or 10-bit addressing transactions, one would have to
//! implement the functionality themselves.
//!
//! [`I2c`]: struct.I2c.html\
//! [`Clock`]: ../car/struct.Clock.html

mod controller;
mod registers;

#[cfg(feature = "hal")]
mod hal;

pub use controller::*;
pub use registers::*;

// TODO: I2C Device implementation.
