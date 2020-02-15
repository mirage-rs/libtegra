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
//! use libtegra::{make_gpio, gpio};
//!
//! // The following line...
//! make_gpio!(D, 4).config(gpio::Config::OutputHigh);
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

#[doc(hidden)]
pub use paste::expr;

pub use controller::CONTROLLER;

use enum_primitive::FromPrimitive;
use register::mmio::ReadWrite;

mod controller;

/// The GPIO ports that are supported by the platform.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port {
    /// Port A.
    A = 0,
    /// Port B.
    B,
    /// Port C.
    C,
    /// Port D.
    D,
    /// Port E.
    E,
    /// Port F.
    F,
    /// Port G.
    G,
    /// Port H.
    H,
    /// Port I.
    I,
    /// Port J.
    J,
    /// Port K.
    K,
    /// Port L.
    L,
    /// Port M.
    M,
    /// Port N.
    N,
    /// Port O.
    O,
    /// Port P.
    P,
    /// Port Q.
    Q,
    /// Port R.
    R,
    /// Port S.
    S,
    /// Port T.
    T,
    /// Port U.
    U,
    /// Port V.
    V,
    /// Port W.
    W,
    /// Port X.
    X,
    /// Port Y.
    Y,
    /// Port Z.
    Z,
    /// Port AA.
    AA,
    /// Port BB.
    BB,
    /// Port CC.
    CC,
    /// Port DD.
    DD,
    /// Port EE.
    EE,
    /// Port FF.
    FF,
}

/// The GPIO pins that are provided for each port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin {
    /// Pin 0.
    P0 = 0,
    /// Pin 1.
    P1,
    /// Pin 2.
    P2,
    /// Pin 3.
    P3,
    /// Pin 4.
    P4,
    /// Pin 5.
    P5,
    /// Pin 6.
    P6,
    /// Pin 7.
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
/// use libtegra::{make_gpio, gpio};
///
/// let some_gpio = gpio::Gpio {
///     port: gpio::Port::X,
///     pin: gpio::Pin::P7,
/// };
///
/// assert_eq!(some_gpio, make_gpio!(X, 7));
/// ```
#[macro_export]
macro_rules! make_gpio {
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
