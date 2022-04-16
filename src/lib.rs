#![no_std]
#![recursion_limit = "1024"]

#[macro_use]
extern crate enum_primitive;

#[macro_use]
extern crate static_assertions;

pub mod actmon;
pub mod ahb;
pub mod apb;
pub mod arm;
pub mod atomic;
pub mod bpmp;
pub mod car;
pub mod dsi;
pub mod flow;
pub mod fuse;
pub mod gpio;
#[cfg(feature = "hal")]
pub mod hal;
pub mod i2c;
pub mod kfuse;
pub mod mc;
pub mod memory_map;
pub mod pinmux;
pub mod pmc;
pub mod pwm;
pub mod se;
pub mod spi;
pub mod system;
pub mod timer;
pub mod tsec;
pub mod uart;
pub mod vic;
