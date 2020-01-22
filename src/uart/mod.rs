use core::{fmt::{Write, Error}, marker::Sync};

use crate::{car::Clock, timer::usleep};

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

impl Uart {
    #[inline(always)]
    fn wait_cycles(&self, baud_rate: u32, amount: u32) {
        usleep((amount * 1_000_000 + 16 * baud_rate - 1) / (16 * baud_rate));
    }

    #[inline(always)]
    fn wait_symbols(&self, baud_rate: u32, amount: u32) {
        usleep((amount * 1_000_000 + baud_rate - 1) / baud_rate);
    }

    pub fn init(&self, baud_rate: u32) {
        let controller = unsafe { &*self.registers };

        // Enable the device clock.
        self.clock.enable();

        while !controller.UART_LSR_0.is_set(UART_LSR_0::TMTY) {
            // Wait for idle state.
        }

        // Calculate the baud rate, rounded to nearest.
        let rate = (8 * baud_rate + 408_000_000) / (16 * baud_rate);

        // Setup UART in FIFO mode.

        // Disable interrupts.
        controller.UART_IER_DLAB_0_0.set(0);
        // No hardware flow control.
        controller.UART_MCR_0.set(0);
        // Enable DLAB and set word length to 8.
        controller.UART_LCR_0.modify(UART_LCR_0::DLAB::SET + UART_LCR_0::WD_SIZE::WordLength8);
        // Divisor latch LSB.
        controller.UART_THR_DLAB_0_0.set(rate);
        // Divisor latch MSB.
        controller.UART_IER_DLAB_0_0.set(rate >> 8);
        // Disable DLAB.
        controller.UART_LCR_0.modify(UART_LCR_0::DLAB::CLEAR);
        // Dummy read.
        controller.UART_SPR_0.get();
        // Wait 3 symbols for the new baud rate.
        self.wait_symbols(baud_rate, 3);

        // Enable FIFO with default settings.

        // Enable FIFO mode.
        controller.UART_IIR_FCR_0.write(UART_IIR_FCR_0::EN_FIFO::Mode16550);
        // Dummy read.
        controller.UART_SPR_0.get();
        // Wait for 3 baud cycles.
        self.wait_cycles(baud_rate, 3);

        // Flush the FIFO.

        while !controller.UART_LSR_0.is_set(UART_LSR_0::TMTY) {
            // Make sure there is no data being written to TX FIFO.
        }
        // Clear TX and RX FIFOs.
        controller.UART_IIR_FCR_0.modify(UART_IIR_FCR_0::TX_CLR::SET + UART_IIR_FCR_0::RX_CLR::SET);
        // Wait for 32 baud cycles.
        self.wait_cycles(baud_rate, 32);
        while !controller.UART_LSR_0.is_set(UART_LSR_0::TMTY)
            && !controller.UART_LSR_0.is_set(UART_LSR_0::RDR)
        {
            // Wait until the FIFOs are ready.
        }
    }

    pub fn read_byte(&self) -> u8 {
        let controller = unsafe { &*self.registers };

        while !controller.UART_LSR_0.is_set(UART_LSR_0::RDR) {
            // Wait until it is possible to read data.
        }

        // Read the byte.
        controller.UART_THR_DLAB_0_0.get() as u8
    }

    pub fn read(&self, data: &mut [u8]) {
        let controller = unsafe { &*self.registers };

        // Read the bytes one by one into the buffer.
        for i in data.iter_mut() {
            *i = self.read_byte();
        }

        while !controller.UART_LSR_0.is_set(UART_LSR_0::RDR) {
            // Wait for everything to be read.
        }
    }

    pub fn write_byte(&self, byte: u8) {
        let controller = unsafe { &*self.registers };

        while !controller.UART_LSR_0.is_set(UART_LSR_0::THRE) {
            // Wait until it is possible to write data.
        }

        // Transmit the byte.
        controller.UART_THR_DLAB_0_0.set(byte as u32);
    }

    pub fn write(&self, data: &[u8]) {
        let controller = unsafe { &*self.registers };

        // Write the bytes from the buffer.
        for byte in data {
            self.write_byte(*byte);
        }

        while !controller.UART_LSR_0.is_set(UART_LSR_0::THRE) {
            // Wait for everything to be written.
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        // Write the string in its bytes representation.
        self.write(s.as_bytes());

        Ok(())
    }
}

unsafe impl Sync for Uart {}
