#![no_std]
#![feature(optimize_attribute)]

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
pub mod memory_map;
pub mod timer;
pub mod uart;
