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

pub use controller::CONTROLLER;

use enum_primitive::FromPrimitive;

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
