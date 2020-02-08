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
//! ## Initialization (Master)
//!
//! [`I2c`] devices have to pass an initialization routine before they can be used.
//!
//! ```no_run
//! use libtegra::i2c::I2c;
//!
//! I2c::C1.init();
//! ```
//!
//! ## Communication (Master)
//!
//! ```no_run
//! use libtegra::i2c::I2c;
//!
//! const MAX77620_RTC_I2C_ADDR: u32 = 0x68;
//!
//! /// Reads the current time from the Real-Time Clock over I²C.
//! fn get_time() -> (u8, u8, u8) {
//!     let hour = I2c::C5.read_byte(MAX77620_RTC_I2C_ADDR, 0x9).unwrap();
//!     let minute = I2c::C5.read_byte(MAX77620_RTC_I2C_ADDR, 0x8).unwrap() & 0x7F;
//!     let second = I2c::C5.read_byte(MAX77620_RTC_I2C_ADDR, 0x7).unwrap() & 0x7F;
//!
//!     (hour, minute, second)
//! }
//!
//! let time = get_time();
//! println!("The current time is: {}:{}:{}", time.0, time.1, time.2);
//! ```
//!
//! [`I2c`]: struct.I2c.html

pub use device::*;
pub use registers::*;

// TODO: I2C Slave implementation.

mod device;
mod registers;
