#![no_std]
#![recursion_limit = "512"]
#![feature(optimize_attribute)]
#![feature(min_const_generics)]

pub extern crate cortex_a;

extern crate byteorder;

extern crate embedded_hal;

#[macro_use]
extern crate enum_primitive;

extern crate paste;

extern crate register;

#[macro_use]
extern crate static_assertions;

pub mod ahb;
pub mod apb;
pub mod bpmp;
pub mod car;
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
pub mod timer;
pub mod tsec;
pub mod uart;
pub mod utils;
