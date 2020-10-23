//! Miscellaneous `embedded-hal` trait implementations.
//!
//! This module consists of a collection of trait implementations from the `embedded-hal` crate
//! where the actual functionality described by these traits is not tied to a specific device.
//!
//! This module is only included when either the `hal` or `hal-unproven` features are enabled.

mod delay;

pub use crate::hal::delay::*;
