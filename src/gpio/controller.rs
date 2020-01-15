use register::mmio::ReadWrite;

/// Base address for the GPIO controller registers.
pub const GPIO_BASE: u32 = 0x6000_D000;

/// A pointer to the GPIO controller that can be accessed by dereferencing it.
pub const CONTROLLER: *const GpioController = GPIO_BASE as *const GpioController;

/// The amount of GPIO ports per bank.
const GPIO_PORTS_COUNT: usize = 4;

/// The amount of available GPIO banks.
const GPIO_BANKS_COUNT: usize = 8;

// TODO: Bitfields for the registers?

/// Representation of a GPIO bank.
#[allow(non_snake_case)]
#[repr(C)]
pub struct GpioBank {
    /// The `GPIO_CNF_*` register.
    pub GPIO_CONFIG: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_OE_*` register.
    pub GPIO_OUTPUT_ENABLE: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_OUT_*` register.
    pub GPIO_OUT: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_IN_*` register.
    pub GPIO_IN: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_INT_STA_*` register.
    pub GPIO_INT_STATUS: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_INT_ENB_*` register.
    pub GPIO_INT_ENABLE: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_INT_LVL_*` register.
    pub GPIO_INT_LEVEL: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_INT_CLR_*` register.
    pub GPIO_INT_CLEAR: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_MSK_CNF` register.
    pub GPIO_MASKED_CONFIG: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_MSK_OE_*` register.
    pub GPIO_MASKED_OUTPUT_ENABLE: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_MSK_OUT_*` register.
    pub GPIO_MASKED_OUT: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_MSK_IN_*` register.
    pub GPIO_MASKED_IN: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_MSK_INT_STA_*` register.
    pub GPIO_MASKED_INT_STATUS: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_MSK_INT_ENB_*` register.
    pub GPIO_MASKED_INT_ENABLE: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_MSK_INT_LVL_*` register.
    pub GPIO_MASKED_INT_LEVEL: [ReadWrite<u32>; GPIO_PORTS_COUNT],
    /// The `GPIO_MSK_INT_CLR_*` register.
    pub GPIO_MASKED_INT_CLEAR: [ReadWrite<u32>; GPIO_PORTS_COUNT],
}

/// Representation of the GPIO controller.
#[repr(C)]
pub struct GpioController {
    /// The GPIO banks that are part of the controller
    pub banks: [GpioBank; GPIO_BANKS_COUNT],
}
