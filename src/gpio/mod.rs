//! Driver for the Tegra X1 General-Purpose Input/Output controller.
//!
//! See `9.10 GPIO Controller` in the Tegra X1 Technical
//! Reference Manual for details.
//!
//! # Description
//!
//! The GPIO controller for Tegra X1 devices provides the tools for
//! configuring each MPIO for use as a software-controlled GPIO.
//!
//! The GPIO controller is divided into 8 banks. Each bank handles
//! the GPIO functionality for up to 32 MPIOs. Within a bank, the
//! GPIOs are arranged as four ports of 8 bits each. The ports are
//! labeled consecutively from A through Z and then AA through FF.
//! In total, there are approximately 170 GPIOs, (however, approximately
//! 170 physical GPIOs are available in the chip) and the banking
//! and numbering conventions will have some break in between but
//! will maintain backwards compatibility in register configurations
//! for the GPIOs as that of previous generation chips.
//!
//! Each GPIO port has 8 available pins, numbered from 0 through 7.
//!
//! Each GPIO can be individually configured as Output/Input/Interrupt
//! sources with level/edge controls.
//!
//! GPIO configuration has a lock bit controlling every bit separately.
//! When the `LOCK` bit is set, the associated control aspects of the bits
//! (for example, whether it is an Output/Input or used as GPIO or SFIO
//! or values driven) cannot be modified (locked). The `LOCK` bit gets
//! cleared only by system reset; it is sticky. This bit can be used for
//! security-related functionality where an authorized entity owning the
//! GPIO can set the configuration and lock it. The lock bit also covers
//! the GPIO output value, so this may not be varied dynamically once
//! `LOCK` is enabled.
//!
//! The GPIO controller also has masked-write registers.
//! Values written to these registers specify both a mask of bits to be
//! updated in the underlying state (the mask bits are not sticky) as well
//! as new values for that state. Individual bits of the state can be
//! updated without the need for a read-modify-write sequence. Thus different
//! portions of software can modify the GPIO controller state without
//! coordination.

pub use paste::expr;

pub use controller::CONTROLLER;

use enum_primitive::FromPrimitive;
use register::mmio::ReadWrite;

mod controller;

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
/// use libtegra::*;
///
/// let gpio = gpio::Gpio {
///     port: gpio::Port::X,
///     pin: gpio::Pin::P7,
/// };
///
/// assert_eq!(gpio, gpio!(X, 7));
/// ```
#[macro_export]
macro_rules! gpio {
    ($port:ident, $pin:tt) => {
        $crate::gpio::Gpio {
            port: $crate::gpio::Port::$port,
            pin: $crate::gpio::expr!($crate::gpio::Pin::[<P $pin>]),
        }
    }
}

impl Gpio {
    /// Calculates the numeric representation of the wrapped GPIO.
    #[inline(always)]
    fn get_gpio_value(&self) -> usize {
        self.port as usize * 8 + self.pin as usize
    }

    /// Calculates the numeric representation of the wrapped GPIO port.
    #[inline(always)]
    fn get_port(&self) -> usize {
        self.port as usize & 3
    }

    /// Calculates the bank where the GPIO is located.
    #[inline(always)]
    fn get_bank(&self) -> usize {
        self.get_gpio_value() >> 5
    }

    /// Calculates the GPIO mask to be used for register writes.
    #[inline(always)]
    fn get_mask(&self) -> u32 {
        1 << self.pin as u32
    }

    /// Reads the flag of a GPIO register.
    #[inline]
    fn read_flag(&self, register: &ReadWrite<u32>) -> u32 {
        (register.get() >> self.pin as u32) & 1
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

        // Read the flag and wrap it into the corresponding enum member.
        Mode::from_u32(self.read_flag(config_reg)).unwrap()
    }

    /// Sets the pin to a given GPIO mode.
    pub fn set_mode(&self, mode: Mode) {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to write to.
        let config_reg = &controller.banks[self.get_bank()].GPIO_CONFIG[self.get_port()];

        // Read the value to be modified and the mask to be used.
        let mut value = config_reg.get();
        let mask = self.get_mask();

        // Set or clear the bit, as appropriate.
        match mode {
            Mode::Sfio => {
                value &= !mask;
            }
            Mode::Gpio => {
                value |= mask;
            }
        }

        // Write the new value to the register.
        config_reg.set(value);

        // Dummy read.
        config_reg.get();
    }

    /// Reads the GPIO direction the pin is currently set to.
    pub fn get_direction(&self) -> Direction {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to read from.
        let direction_reg = &controller.banks[self.get_bank()].GPIO_OUTPUT_ENABLE[self.get_port()];

        // Read the flag and wrap it into the corresponding enum member.
        Direction::from_u32(self.read_flag(direction_reg)).unwrap()
    }

    /// Sets the pin to a given GPIO direction.
    pub fn set_direction(&self, direction: Direction) {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to write to.
        let direction_reg = &controller.banks[self.get_bank()].GPIO_OUTPUT_ENABLE[self.get_port()];

        // Read the value to be modified and the mask to be used.
        let mut value = direction_reg.get();
        let mask = self.get_mask();

        // Set or clear the bit, as appropriate.
        match direction {
            Direction::Input => {
                value &= !mask;
            }
            Direction::Output => {
                value |= mask;
            }
        }

        // Write the new value to the register.
        direction_reg.set(value);

        // Dummy read.
        direction_reg.get();
    }

    /// Reads the GPIO level the pin is currently set to.
    pub fn read(&self) -> Level {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to read from.
        let in_reg = &controller.banks[self.get_bank()].GPIO_IN[self.get_port()];

        // Read the flag and wrap it into the corresponding enum member.
        Level::from_u32(self.read_flag(in_reg)).unwrap()
    }

    /// Writes the given GPIO level to the pin.
    pub fn write(&self, level: Level) {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to write to.
        let out_reg = &controller.banks[self.get_bank()].GPIO_OUT[self.get_port()];

        // Read the value to be modified and the mask to be used.
        let mut value = out_reg.get();
        let mask = self.get_mask();

        // Set or clear the bit, as appropriate.
        match level {
            Level::Low => {
                value &= !mask;
            }
            Level::High => {
                value |= mask;
            }
        }

        // Write the new value to the register.
        out_reg.set(value);

        // Dummy read.
        out_reg.get();
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

        // Read the value to be modified and the mask to be used.
        let mut value = int_enb_reg.get();
        let mask = self.get_mask();

        // Set or clear the bit, as appropriate.
        if enable {
            value |= mask;
        } else {
            value &= !mask;
        }

        // Write the new value to the register.
        int_enb_reg.set(value);

        // Dummy read.
        int_enb_reg.get();
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

        // Read the value to be modified and the mask to be used.
        let mut value = int_clr_reg.get();
        let mask = self.get_mask();

        // Set the bit.
        value |= mask;

        // Write the new value to the register.
        int_clr_reg.set(value);

        // Dummy read.
        int_clr_reg.get();
    }

    /// Raises a given interrupt on the GPIO.
    pub fn set_interrupt(&self, interrupt: InterruptType) {
        let controller = unsafe { &*CONTROLLER };

        // Figure out the register to write to.
        let int_lvl_reg = &controller.banks[self.get_bank()].GPIO_INT_LEVEL[self.get_port()];

        // Read the value to be modified and the mask to be used.
        let mut value = int_lvl_reg.get();
        let mask = self.get_mask();

        // Configure the interrupt.
        value &= !(0x010101 << mask);
        value |= (interrupt as u32) << mask;

        // Write the new value to the register.
        int_lvl_reg.set(value);

        // Dummy read.
        int_lvl_reg.get();
    }
}
