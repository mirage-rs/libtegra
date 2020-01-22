use crate::car::Clock;

use registers::*;

mod registers;

/// Representation of a UART.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Uart {
    /// A reference to the device clock that corresponds to the UART.
    clock: &'static Clock,
    /// A pointer to the [`Registers`] of the UART.
    ///
    /// [`Registers`]: struct.Registers.html
    registers: *const Registers,
}

// Definitions of known UARTs.

impl Uart {
    /// Representation of UART A.
    pub const A: Self = Uart {
        clock: &Clock::UART_A,
        registers: UART_A_REGISTERS,
    };

    /// Representation of UART B.
    pub const B: Self = Uart {
        clock: &Clock::UART_B,
        registers: UART_B_REGISTERS,
    };

    /// Representation of UART C.
    pub const C: Self = Uart {
        clock: &Clock::UART_C,
        registers: UART_C_REGISTERS,
    };

    /// Representation of UART D.
    pub const D: Self = Uart {
        clock: &Clock::UART_D,
        registers: UART_D_REGISTERS,
    };

    /// Representation of UART E.
    pub const E: Self = Uart {
        clock: &Clock::UART_APE,
        registers: UART_E_REGISTERS,
    };
}
