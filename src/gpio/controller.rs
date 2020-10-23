use register::{mmio::ReadWrite, register_structs};

use crate::memory_map::gpio::BASE;

/// A pointer to the GPIO controller that can be accessed by dereferencing it.
pub const CONTROLLER: *const GpioController = BASE as *const GpioController;

const GPIO_PORTS_COUNT: usize = 4;
const GPIO_BANKS_COUNT: usize = 8;

// TODO: Bitfields for the registers?

register_structs! {
    /// Representation of a GPIO bank.
    #[allow(non_snake_case)]
    pub GpioBank {
        (0x000 => pub GPIO_CONFIG: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x010 => pub GPIO_OUTPUT_ENABLE: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x020 => pub GPIO_OUT: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x030 => pub GPIO_IN: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x040 => pub GPIO_INT_STATUS: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x050 => pub GPIO_INT_ENABLE: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x060 => pub GPIO_INT_LEVEL: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x070 => pub GPIO_INT_CLEAR: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x080 => pub GPIO_MASKED_CONFIG: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x090 => pub GPIO_MASKED_OUTPUT_ENABLE: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x0A0 => pub GPIO_MASKED_OUT: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x0B0 => pub GPIO_MASKED_IN: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x0C0 => pub GPIO_MASKED_INT_STATUS: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x0D0 => pub GPIO_MASKED_INT_ENABLE: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x0E0 => pub GPIO_MASKED_INT_LEVEL: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x0F0 => pub GPIO_MASKED_INT_CLEAR: [ReadWrite<u32>; GPIO_PORTS_COUNT]),
        (0x100 => @END),
    }
}

assert_eq_size!(GpioBank, [u8; 0x100]);

/// Representation of the GPIO controller.
#[repr(C)]
pub struct GpioController {
    /// The GPIO banks that are part of the controller.
    pub banks: [GpioBank; GPIO_BANKS_COUNT],
}

assert_eq_size!(GpioController, [u8; 0x800]);
