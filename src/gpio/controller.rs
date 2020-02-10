//! Abstractions over the registers of the Tegra X1 GPIO Controller.
//!
//! See Chapter 9.13 in the Tegra X1 Technical Reference Manual for
//! details.

use register::mmio::ReadWrite;

use crate::memory_map::gpio::BASE;

/// A pointer to the GPIO controller that can be accessed by dereferencing it.
pub const CONTROLLER: *const GpioController = BASE as *const GpioController;

/// The amount of GPIO ports per bank.
const GPIO_PORTS_COUNT: usize = 4;

/// The amount of available GPIO banks.
const GPIO_BANKS_COUNT: usize = 8;

// TODO: Bitfields for the registers?

/// Representation of a GPIO bank.
#[allow(non_snake_case)]
#[repr(C)]
pub struct GpioBank {
    pub GPIO_CONFIG: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_OUTPUT_ENABLE: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_OUT: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_IN: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_INT_STATUS: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_INT_ENABLE: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_INT_LEVEL: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_INT_CLEAR: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_MASKED_CONFIG: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_MASKED_OUTPUT_ENABLE: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_MASKED_OUT: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_MASKED_IN: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_MASKED_INT_STATUS: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_MASKED_INT_ENABLE: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_MASKED_INT_LEVEL: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    pub GPIO_MASKED_INT_CLEAR: [ReadWrite<u32>; GPIO_PORTS_COUNT],
}

assert_eq_size!(GpioBank, [u8; 0x100]);

/// Representation of the GPIO controller.
#[repr(C)]
pub struct GpioController {
    /// The GPIO banks that are part of the controller
    pub banks: [GpioBank; GPIO_BANKS_COUNT],
}

assert_eq_size!(GpioController, [u8; 0x800]);
