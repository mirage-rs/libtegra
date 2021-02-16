//! Utilities related to the ARM processors found on Tegra X1 SoC.
//!
//! This covers both, the ARM7TDMI used as the [bpmp] and the Cortex-A53/A57 processors
//! forming the main CPU Complex (CCPLEX).

pub mod cache;
pub mod gic;
mod utils;

pub use utils::*;
