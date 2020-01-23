use core::{
    cell::Cell,
    fmt::{Write, Error},
    marker::Sync,
};

use crate::{car::Clock, timer::usleep};

pub use registers::*;

mod registers;

/// Representation of a UART.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Uart {
    /// The baud rate this [`Uart`] is configured with.
    ///
    /// Use [`Uart::get_baud_rate`] to obtain the "real" rate
    /// the Uart is configured with.
    ///
    /// NOTE: This value will be overridden by [`Uart::init`],
    /// so it is safe to initialize instances of this struct
    /// with a dummy value, such as `Cell::new(0)`.
    ///
    /// [`Uart`]: struct.Uart.html
    /// [`Uart::get_baud_rate`]: struct.Uart.html#method.get_baud_rate
    /// [`Uart::init`]: struct.Uart.html#method.init
    baud: Cell<u32>,
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
        baud: Cell::new(0),
        clock: &Clock::UART_A,
        registers: UART_A_REGISTERS,
    };

    /// Representation of UART B.
    pub const B: Self = Uart {
        baud: Cell::new(0),
        clock: &Clock::UART_B,
        registers: UART_B_REGISTERS,
    };

    /// Representation of UART C.
    pub const C: Self = Uart {
        baud: Cell::new(0),
        clock: &Clock::UART_C,
        registers: UART_C_REGISTERS,
    };

    /// Representation of UART D.
    pub const D: Self = Uart {
        baud: Cell::new(0),
        clock: &Clock::UART_D,
        registers: UART_D_REGISTERS,
    };

    /// Representation of UART E.
    pub const E: Self = Uart {
        baud: Cell::new(0),
        clock: &Clock::UART_APE,
        registers: UART_E_REGISTERS,
    };
}

impl Uart {
    #[inline(always)]
    fn wait_cycles(&self, amount: u32) {
        let baud_rate = self.baud.get();
        usleep((amount * 1_000_000 + 16 * baud_rate - 1) / (16 * baud_rate));
    }

    #[inline(always)]
    fn wait_symbols(&self, amount: u32) {
        let baud_rate = self.baud.get();
        usleep((amount * 1_000_000 + baud_rate - 1) / baud_rate);
    }

    pub fn get_baud_rate(&self) -> u32 {
        let baud_rate = self.baud.get();

        (8 * baud_rate + 408_000_000) / (16 * baud_rate)
    }

    pub fn init(&self, baud_rate: u32) {
        let controller = unsafe { &*self.registers };

        // Store the provided baud rate.
        self.baud.set(baud_rate);

        // Enable the device clock.
        self.clock.enable();

        while !controller.UART_LSR_0.is_set(UART_LSR_0::TMTY) {
            // Wait for idle state.
        }

        // Calculate the baud rate, rounded to nearest.
        let real_baud_rate = self.get_baud_rate();

        // Setup UART in FIFO mode.

        // Disable interrupts.
        controller.UART_IER_DLAB_0_0.set(0);
        // Disable hardware flow control.
        controller.UART_MCR_0.set(0);
        // Enable DLAB and set word length to 8.
        controller.UART_LCR_0.modify(UART_LCR_0::DLAB::SET + UART_LCR_0::WD_SIZE::WordLength8);
        // Divisor latch LSB.
        controller.UART_THR_DLAB_0_0.set(real_baud_rate);
        // Divisor latch MSB.
        controller.UART_IER_DLAB_0_0.set(real_baud_rate >> 8);
        // Disable DLAB.
        controller.UART_LCR_0.modify(UART_LCR_0::DLAB::CLEAR);
        // Dummy read.
        controller.UART_SPR_0.get();
        // Wait 3 symbols for the new baud rate.
        self.wait_symbols(3);

        // Enable FIFO with default settings.

        // Enable FIFO mode.
        controller.UART_IIR_FCR_0.write(UART_IIR_FCR_0::EN_FIFO::Mode16550);
        // Dummy read.
        controller.UART_SPR_0.get();
        // Wait for 3 baud cycles.
        self.wait_cycles(3);

        // Flush the FIFOs and wait until they are ready.
        self.flush();
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

    pub fn flush(&self) {
        let controller = unsafe { &*self.registers };

        while !controller.UART_LSR_0.is_set(UART_LSR_0::TMTY) {
            // Make sure there is no data being written to TX FIFO.
        }

        // Disable hardware control flow.
        controller.UART_MCR_0.set(0);
        // Dummy read.
        controller.UART_SPR_0.get();
        // Wait for 1 character time.
        // XXX: Figure out how to calculate this from code.
        usleep(96);

        // Issue flush requests for TX FIFO and RX FIFO.
        controller.UART_IIR_FCR_0.modify(UART_IIR_FCR_0::TX_CLR::SET + UART_IIR_FCR_0::RX_CLR::SET);
        // Dummy read.
        controller.UART_SPR_0.get();
        // Wait for 32 baud cycles.
        self.wait_cycles(32);

        while !controller.UART_LSR_0.is_set(UART_LSR_0::TMTY)
            && controller.UART_LSR_0.is_set(UART_LSR_0::RDR)
        {
            // Wait until the FIFOs are ready.
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
