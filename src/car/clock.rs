//! Device clock abstractions.

use register::mmio::ReadWrite;

use crate::timer::usleep;

/// Base address for Clock and Reset Controller registers.
pub const CLOCK_BASE: u32 = 0x6000_6000;

// Some constants for the driver.

const CLK_RST_CONTROLLER_RST_DEVICES_L: u32 = 0x4;
const CLK_RST_CONTROLLER_RST_DEVICES_H: u32 = 0x8;
const CLK_RST_CONTROLLER_RST_DEVICES_U: u32 = 0xC;
const CLK_RST_CONTROLLER_RST_DEVICES_X: u32 = 0x28C;
const CLK_RST_CONTROLLER_RST_DEVICES_Y: u32 = 0x2A4;
const CLK_RST_CONTROLLER_RST_DEVICES_V: u32 = 0x358;
const CLK_RST_CONTROLLER_RST_DEVICES_W: u32 = 0x35C;

const CLK_RST_CONTROLLER_CLK_OUT_ENB_L: u32 = 0x10;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_H: u32 = 0x14;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_U: u32 = 0x18;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_X: u32 = 0x280;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_Y: u32 = 0x298;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_V: u32 = 0x360;
const CLK_RST_CONTROLLER_CLK_OUT_ENB_W: u32 = 0x364;

const CLK_NO_SOURCE: u32 = 0;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_A: u32 = 0x178;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_B: u32 = 0x17C;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_C: u32 = 0x1A0;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_D: u32 = 0x1C0;
const CLK_RST_CONTROLLER_CLK_SOURCE_UART_APE: u32 = 0x710;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_1: u32 = 0x124;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_2: u32 = 0x198;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_3: u32 = 0x1B8;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_4: u32 = 0x3C4;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_5: u32 = 0x128;
const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_6: u32 = 0x65C;
const CLK_RST_CONTROLLER_CLK_SOURCE_SE: u32 = 0x42C;
const CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X: u32 = 0x180;
const CLK_RST_CONTROLLER_CLK_SOURCE_TSEC: u32 = 0x1F4;
const CLK_RST_CONTROLLER_CLK_SOURCE_SOR1: u32 = 0x410;
const CLK_RST_CONTROLLER_CLK_SOURCE_CSITE: u32 = 0x1D4;
const CLK_RST_CONTROLLER_CLK_SOURCE_PWM: u32 = 0x11;

/// Representation of a device clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clock {
    /// The reset register offset for the clock device.
    reset: u32,
    /// The enable register offset for the clock device.
    enable: u32,
    /// The source register offset for the clock device.
    source: u32,
    /// The clock device index.
    index: u8,
    /// The clock source value.
    clock_source: u32,
    /// The clock divider value.
    clock_divider: u32,
}

// Definitions for known device clocks.
impl Clock {
    /// Representation of the UART A clock.
    pub const UART_A: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_A,
        index: 0x6,
        clock_source: 0,
        clock_divider: 0,
    };

    /// Representation of the UART B clock.
    pub const UART_B: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_B,
        index: 0x7,
        clock_source: 0,
        clock_divider: 0,
    };

    /// Representation of the UART C clock.
    pub const UART_C: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_C,
        index: 0x17,
        clock_source: 0,
        clock_divider: 0,
    };

    /// Representation of the UART D clock.
    pub const UART_D: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_D,
        index: 0x1,
        clock_source: 0,
        clock_divider: 0,
    };

    /// Representation of the UART APE clock.
    pub const UART_APE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_APE,
        index: 0x14,
        clock_source: 0,
        clock_divider: 0,
    };

    /// Representation of the I²C 1 clock.
    pub const I2C_1: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_1,
        index: 0xC,
        clock_source: 0x6,
        clock_divider: 0,
    };

    /// Representation of the I²C 2 clock.
    pub const I2C_2: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_2,
        index: 0x16,
        clock_source: 0x6,
        clock_divider: 0,
    };

    /// Representation of the I²C 3 clock.
    pub const I2C_3: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_3,
        index: 0x3,
        clock_source: 0x6,
        clock_divider: 0,
    };

    /// Representation of the I²C 4 clock.
    pub const I2C_4: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_4,
        index: 0x7,
        clock_source: 0x6,
        clock_divider: 0,
    };

    /// Representation of the I²C 5 clock.
    pub const I2C_5: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_5,
        index: 0xF,
        clock_source: 0x6,
        clock_divider: 0,
    };

    /// Representation of the I²C 6 clock.
    pub const I2C_6: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_6,
        index: 0x6,
        clock_source: 0x6,
        clock_divider: 0,
    };

    /// Representation of the Security Engine clock.
    pub const SE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_SE,
        index: 0x1F,
        clock_source: 0,
        clock_divider: 0,
    };

    /// Representation of the TZRAM clock.
    pub const TZRAM: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_NO_SOURCE,
        index: 0x1E,
        clock_source: 0,
        clock_divider: 0,
    };

    /// Representation of the HOST1X clock.
    pub const HOST1X: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X,
        index: 0x1C,
        clock_source: 0x4,
        clock_divider: 0x3,
    };

    /// Representation of the TSEC clock.
    pub const TSEC: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_TSEC,
        index: 0x13,
        clock_source: 0,
        clock_divider: 0x2,
    };

    /// Representation of the SOR_SAFE clock.
    pub const SOR_SAFE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_NO_SOURCE,
        index: 0x1E,
        clock_source: 0,
        clock_divider: 0,
    };

    /// Representation of the SOR0 clock.
    pub const SOR0: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_NO_SOURCE,
        index: 0x16,
        clock_source: 0,
        clock_divider: 0,
    };

    /// Representation of the SOR1 clock.
    pub const SOR1: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_SOR1,
        index: 0x17,
        clock_source: 0,
        clock_divider: 0x2,
    };

    /// Representation of the KFUSE clock.
    pub const KFUSE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_NO_SOURCE,
        index: 0x8,
        clock_source: 0,
        clock_divider: 0,
    };

    /// Representation of the CL-DVFS clock.
    pub const CL_DVFS: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_W,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_W,
        source: CLK_NO_SOURCE,
        index: 0x1B,
        clock_source: 0,
        clock_divider: 0,
    };

    /// Representation of the CSITE clock.
    pub const CORESIGHT: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_CSITE,
        index: 0x9,
        clock_source: 0,
        clock_divider: 0x4,
    };

    /// Representation of the PWM clock.
    pub const PWM: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_PWM,
        index: 0x11,
        clock_source: 0x6,
        clock_divider: 0x4,
    };
}

impl Clock {
    /// Calculates the mask to be used for writes to the clock registers.
    #[inline(always)]
    fn get_mask(&self) -> u32 {
        (1 << (self.index & 0x1F)) as u32
    }

    /// Changes the reset state of a clock.
    ///
    /// # Arguments
    ///
    /// Behavior of this method depends on the `reset` argument.
    ///
    /// * `true` - Puts a clock into a reset state.
    ///
    /// * `false` - Takes a clock off the reset state.
    fn set_reset(&self, reset: bool) {
        // Figure out the register to write to.
        let reset_reg = unsafe { &*((CLOCK_BASE + self.reset) as *const ReadWrite<u32>) };

        // Read the value to be modified and the mask to be used.
        let mut value = reset_reg.get();
        let mask = self.get_mask();

        // Set or clear the bit, as appropriate.
        if reset {
            value |= mask;
        } else {
            value &= !mask;
        }

        // Write the new value to the register.
        reset_reg.set(value);
    }

    /// Changes the enabling state of the clock.
    ///
    /// # Arguments
    ///
    /// Behavior of this method depends on the `enable` argument.
    ///
    /// * `true` - Enables the device.
    ///
    /// * `false` - Disables the device.
    fn set_enable(&self, enable: bool) {
        // Figure out the register to write to.
        let enable_reg = unsafe { &*((CLOCK_BASE + self.enable) as *const ReadWrite<u32>) };

        // Read the value to be modified and the mask to be used.
        let mut value = enable_reg.get();
        let mask = self.get_mask();

        // Set or clear the bit, as appropriate.
        if enable {
            value |= mask;
        } else {
            value &= !mask;
        }

        // Write the new value to the register.
        enable_reg.set(value);
    }

    /// Boots up the device.
    pub fn enable(&self) {
        // Disable the clock.
        self.disable();

        // Configure the clock source, if needed.
        if self.source != 0 {
            unsafe {
                (*((CLOCK_BASE + self.source) as *const ReadWrite<u32>))
                    .set((self.clock_source << 29) | self.clock_divider);
            }
        }

        // KFUSE needs more time for the changes to take effect.
        if self == &Self::KFUSE {
            // Enable the clock.
            self.set_enable(true);
            usleep(100);

            // Take clock off reset.
            self.set_reset(false);
            usleep(200);
        } else {
            // Enable the clock.
            self.set_enable(true);
            usleep(2);

            // Take clock off reset.
            self.set_reset(false);
        }
    }

    /// Disables the device.
    pub fn disable(&self) {
        // Put clock into reset.
        self.set_reset(true);
        // Disable.
        self.set_enable(false);
    }

    /// Indicates whether the device is enabled or not.
    pub fn is_enabled(&self) -> bool {
        // Figure out the register to read from.
        let enable_reg = unsafe { &*((CLOCK_BASE + self.enable) as *const ReadWrite<u32>) };

        // Check if the mask bit is set.
        let mask = self.get_mask();
        (enable_reg.get() & mask) == mask
    }
}
