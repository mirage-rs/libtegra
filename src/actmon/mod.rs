//! Driver for the Tegra X1 Activity Monitor (ACTMON).
//!
//! See Chapter 10 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! The Activity Monitor acts as a repository for activity indicators from various
//! units, performs averaging of past samplse, and indicates any abnormality in
//! activity levels by sending an interrupt to software. Devices provide an idle/busy
//! signal to the ACTMON block.

mod registers;

pub use registers::*;
