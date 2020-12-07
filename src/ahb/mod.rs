//! Driver for the Tegra X1 AMBA Advanced High-performance Bus.
//!
//! See Chapter 19 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! The AHB Bus conforms to the AMBA Specification (Rev 2.0) Advanced High-
//! performance Bus architecture as published by ARM.
//!
//! AHB is a 32-bit multi-master bus. Despite what the name implies, it is a
//! second tier bus in the Tegra X1 processor, slower and less flexible than
//! the AXI bus used by the CPU, or the memory client interface used by the
//! high-speed devices.
//!
//! It supports secure access to memory, using the same secure bit mechanism
//! used by the CPU TrustZone security mechanism through the Security Engine.

mod registers;

pub use registers::*;
