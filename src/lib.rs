#![no_std]
#![recursion_limit = "512"]
#![feature(const_fn)]
#![feature(optimize_attribute)]

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
pub mod gpio;
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
