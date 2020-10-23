//! Driver for the Tegra X1 General-Purpose Input/Output controller.
//!
//! See Chapter 9.10 in the Tegra X1 Technical Reference Manual for
//! details.
//!
//! # Description
//!
//! The GPIO controller for Tegra X1 devices provides the tools for
//! configuring each MPIO for use as a software-controlled GPIO.
//!
//! ## Controller Structure
//!
//! The controller is divided into 8 banks. Each bank contains 4 GPIO
//! ports, which in turn provide 8 available pins, numbered from 0
//! through 7. GPIO ports a labeled consecutively from A through Z
//! and then AA through FF. Ports A through E are in bank 0, E through
//! H in bank 1, and so on.
//!
//! In conclusion, 256 GPIOs in total are therefore available,
//! approximately 170 of them being physical pins.
//! Each of them can be identified uniquely through a port and a pin.
//!
//! ## Configuration
//!
//! Each GPIO can be configured individually as Input/Output/Interrupt
//! sources with level/edge controls.
//!
//! ```no_run
//! use libtegra::{tegra_gpio, gpio};
//!
//! // The following line...
//! tegra_gpio!(D, 4).config(gpio::Config::OutputHigh);
//!
//! // ...is equivalent to:
//! let pin = gpio::Gpio {
//!     port: gpio::Port::D,
//!     pin: gpio::Pin::P4,
//! };
//! pin.set_mode(gpio::Mode::Gpio);
//! pin.set_direction(gpio::Direction::Output);
//! pin.write(gpio::Level::High);
//! ```

mod controller;

#[cfg(feature = "hal")]
mod hal;

pub use crate::gpio::controller::*;

use enum_primitive::FromPrimitive;
#[doc(hidden)]
pub use paste::expr;
use register::mmio::ReadWrite;

/// The GPIO ports that are supported by the platform.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    AA,
    BB,
    CC,
    DD,
    EE,
    FF,
}

/// The GPIO pins that are provided for each port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin {
    P0 = 0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
}

enum_from_primitive! {
    /// Possible GPIO modes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Mode {
        /// SFIO mode.
        Sfio = 0,
        /// GPIO mode.
        Gpio,
    }
}

enum_from_primitive! {
    /// Possible GPIO directions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Direction {
        /// Input direction.
        Input = 0,
        /// Output direction.
        Output,
    }
}

enum_from_primitive! {
    /// Possible GPIO levels.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Level {
        /// Low level.
        Low = 0,
        /// High level.
        High,
    }
}

/// Supported GPIO configurations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Config {
    /// Configures a GPIO for input.
    Input,
    /// Configures a GPIO for output with the level set to low.
    OutputLow,
    /// Configures a GPIO for output with the level set to high.
    OutputHigh,
}

/// Supported interrupt types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterruptType {
    /// Rising edge interrupt.
    RisingEdge = 0,
    /// Falling edge interrupt.
    FallingEdge,
    /// Both edges interrupt.
    BothEdge,
    /// High level interrupt.
    HighLevel,
    /// Low level interrupt.
    LowLevel,
}

/// Representation of a Tegra X1 GPIO.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gpio {
    /// The GPIO port.
    pub port: Port,
    /// The GPIO pin.
    pub pin: Pin,
}

/// A macro to facilitate the creation of a GPIO given port and pin.
///
/// # Example
///
/// ```
/// use libtegra::{tegra_gpio, gpio};
///
/// let some_gpio = gpio::Gpio {
///     port: gpio::Port::X,
///     pin: gpio::Pin::P7,
/// };
///
/// assert_eq!(some_gpio, tegra_gpio!(X, 7));
/// ```
#[macro_export]
macro_rules! tegra_gpio {
    ($port:ident, $pin:tt) => {
        $crate::gpio::Gpio {
            port: $crate::gpio::Port::$port,
            pin: $crate::gpio::expr!($crate::gpio::Pin::[<P $pin>]),
        }
    }
}

impl Gpio {
    #[inline(always)]
    fn get_gpio_value(&self) -> usize {
        self.port as usize * 8 + self.pin as usize
    }

    #[inline(always)]
    fn get_port(&self) -> usize {
        self.port as usize & 3
    }

    #[inline(always)]
    fn get_bank(&self) -> usize {
        self.get_gpio_value() >> 5
    }

    #[inline(always)]
    fn get_mask(&self) -> u32 {
        1 << self.pin as u32
    }

    #[inline]
    fn read_flag(&self, register: &ReadWrite<u32>) -> u32 {
        (register.get() >> self.pin as u32) & 1
    }

    /// Reads a GPIO register and returns the enum representation of the result.
    #[inline]
    fn read_gpio<T>(&self, register: &ReadWrite<u32>) -> T
    where
        T: FromPrimitive,
    {
        // Read the flag and wrap it into the corresponding enum member.
        T::from_u32(self.read_flag(register)).unwrap()
    }

    /// Writes to a GPIO register and respectively toggles or clears the bit.
    #[inline]
    fn write_gpio(&self, register: &ReadWrite<u32>, set_bit: bool) {
        // Read the value to be modified and the mask to be used.
        let mut value = register.get();
        let mask = self.get_mask();

        // Toggle the bit appropriately.
        if set_bit {
            value |= mask;
        } else {
            value &= !mask;
        }

        // Write the new value to the register.
        register.set(value);

        // Commit the write.
        register.get();
    }

    /// Configures a GPIO with a given configuration.
    pub fn config(&self, config: Config) {
        self.set_mode(Mode::Gpio);

        match config {
            Config::Input => {
                self.set_direction(Direction::Input);
            }
            Config::OutputLow => {
                self.set_direction(Direction::Output);
                self.write(Level::Low);
            }
            Config::OutputHigh => {
                self.set_direction(Direction::Output);
                self.write(Level::High);
            }
        }
    }

    /// Reads the GPIO mode the pin is currently set to.
    pub fn get_mode(&self) -> Mode {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to read from.
        let config_reg = &controller.banks[self.get_bank()].GPIO_CONFIG[self.get_port()];

        // Read the register and return the result.
        self.read_gpio::<Mode>(config_reg)
    }

    /// Sets the pin to a given GPIO mode.
    pub fn set_mode(&self, mode: Mode) {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to write to.
        let config_reg = &controller.banks[self.get_bank()].GPIO_CONFIG[self.get_port()];

        // Update the register.
        self.write_gpio(config_reg, mode == Mode::Gpio);
    }

    /// Reads the GPIO direction the pin is currently set to.
    pub fn get_direction(&self) -> Direction {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to read from.
        let direction_reg = &controller.banks[self.get_bank()].GPIO_OUTPUT_ENABLE[self.get_port()];

        // Read the register and return the result.
        self.read_gpio::<Direction>(direction_reg)
    }

    /// Sets the pin to a given GPIO direction.
    pub fn set_direction(&self, direction: Direction) {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to write to.
        let direction_reg = &controller.banks[self.get_bank()].GPIO_OUTPUT_ENABLE[self.get_port()];

        // Update the register.
        self.write_gpio(direction_reg, direction == Direction::Output);
    }

    /// Reads the GPIO level the pin is currently set to.
    pub fn read(&self) -> Level {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to read from.
        let in_reg = &controller.banks[self.get_bank()].GPIO_IN[self.get_port()];

        // Read the register and return the result.
        self.read_gpio::<Level>(in_reg)
    }

    /// Writes the given GPIO level to the pin.
    pub fn write(&self, level: Level) {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to write to.
        let out_reg = &controller.banks[self.get_bank()].GPIO_OUT[self.get_port()];

        // Update the register.
        self.write_gpio(out_reg, level == Level::High);
    }

    /// Whether the pin is currently set to high.
    pub fn is_high(&self) -> bool {
        self.read() == Level::High
    }

    /// Whether the pin is currently set to low.
    pub fn is_low(&self) -> bool {
        self.read() == Level::Low
    }

    /// Whether interrupts are enabled for the GPIO.
    pub fn interrupts_enabled(&self) -> bool {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to read from.
        let int_enb_reg = &controller.banks[self.get_bank()].GPIO_INT_ENABLE[self.get_port()];

        // Read the flag and check whether interrupts are enabled.
        self.read_flag(int_enb_reg) == 1
    }

    /// Enables or disables interrupts.
    fn set_enable_interrupts(&self, enable: bool) {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to write to.
        let int_enb_reg = &controller.banks[self.get_bank()].GPIO_INT_ENABLE[self.get_port()];

        // Update the register.
        self.write_gpio(int_enb_reg, enable);
    }

    /// Enables interrupts for the GPIO.
    #[inline]
    pub fn enable_interrupts(&self) {
        self.set_enable_interrupts(true)
    }

    /// Disables interrupts for the GPIO.
    #[inline]
    pub fn disable_interrupts(&self) {
        self.set_enable_interrupts(false)
    }

    /// Clears the interrupts that are set for the GPIO.
    pub fn clear_interrupts(&self) {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to write to.
        let int_clr_reg = &controller.banks[self.get_bank()].GPIO_INT_CLEAR[self.get_port()];

        // Update the register.
        self.write_gpio(int_clr_reg, true);
    }

    /// Raises a given interrupt on the GPIO.
    pub fn set_interrupt(&self, interrupt: InterruptType) {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to write to.
        let int_lvl_reg = &controller.banks[self.get_bank()].GPIO_INT_LEVEL[self.get_port()];

        // Read the value to be modified.
        let mut value = int_lvl_reg.get();

        // Configure the interrupt.
        value &= !(0x010101 << self.pin as u32);
        value |= (interrupt as u32) << self.pin as u32;

        // Write the new value to the register.
        int_lvl_reg.set(value);

        // Dummy read.
        int_lvl_reg.get();
    }
}
