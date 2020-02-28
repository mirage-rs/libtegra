#![no_std]
#![recursion_limit = "256"]
#![feature(const_fn)]
#![feature(optimize_attribute)]

pub extern crate cortex_a;

#[macro_use]
extern crate enum_primitive;

extern crate paste;

extern crate register;

#[macro_use]
extern crate static_assertions;

pub mod car;
pub mod gpio;
pub mod i2c;
pub mod kfuse;
pub mod pinmux;
pub mod pmc;
pub mod memory_map;
pub mod timer;
pub mod tsec;
pub mod uart;
