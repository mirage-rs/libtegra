//! Driver for the Tegra X1 Universal Asynchronous Receiver/Transmitter Controller.
//!
//! See Chapter 36 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! There are five UARTs available in total. The UARTs A through D, which are
//! identical, are built into Tegra X1 devices and the fifth UART is located
//! in the Audio Processing Engine.
//!
//! These UARTs support both, 16450 and 16550 compatible modes, although this
//! implementation specifically targets the 16550 mode.
//!
//! # Transmission speed
//!
//! UART controllers support [`Clock`]s up to 200MHz with a baud rate of 12.5M.
//!
//! The default supported and recommended baud rate is `115_200`. If one wishes to
//! use a custom baud rate, they may have to alter the clock divisor configuration of
//! the corresponding UART device clock.
//!
//! # Initialization
//!
//! [`Uart`]s need to be initialized with a given baud rate before they can be used.
//!
//! ```no_run
//! use libtegra::uart::{Uart, BAUD_115200};
//!
//! Uart::A.init(BAUD_115200);
//! ```
//!
//! # Communication
//!
//! After a [`Uart`] was initialized, it can be used like this:
//!
//! ```no_run
//! use core::fmt::Write;
//!
//! use libtegra::uart::Uart;
//!
//! writeln!(&mut Uart::A, "I got {} problems, but UART logging ain't one!", 99);
//! ```
//!
//! Reading data is also supported:
//!
//! ```no_run
//! use libtegra::uart::Uart;
//!
//! let mut uart = Uart::A; // Less typing...
//!
//! // Read a single byte.
//! let byte = uart.read_byte();
//!
//! // Read 10 bytes into a buffer.
//! let mut buffer = [0; 10];
//! uart.read(&mut buffer);
//! ```
//!
//! # Flushing
//!
//! In some cases, you may want to flush the underlying FIFOs:
//!
//! ```no_run
//! use libtegra::uart::Uart;
//!
//! Uart::A.flush();
//! ```
//!
//! [`Uart`]: struct.Uart.html

mod registers;

#[cfg(feature = "hal")]
mod hal;

use core::{
    fmt::{self, Error},
    marker::Sync,
};

pub use crate::uart::registers::*;
use crate::{car::Clock, timer::usleep};

/// The default baud rate that can be used to intiialize UARTs.
pub const BAUD_115200: u32 = 115_200;

/// Representation of a UART.
///
/// NOTE: Instances of this struct should never be created manually.
/// Refer to the public constants the struct holds, which represent
/// the UARTs A through E.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Uart {
    baud: u32,
    clock: &'static Clock,
    registers: *const Registers,
}

// Definitions of known UARTs.

impl Uart {
    /// Representation of UART A.
    pub const A: Self = Uart {
        baud: 0,
        clock: &Clock::UART_A,
        registers: UART_A_REGISTERS,
    };

    /// Representation of UART B.
    pub const B: Self = Uart {
        baud: 0,
        clock: &Clock::UART_B,
        registers: UART_B_REGISTERS,
    };

    /// Representation of UART C.
    pub const C: Self = Uart {
        baud: 0,
        clock: &Clock::UART_C,
        registers: UART_C_REGISTERS,
    };

    /// Representation of UART D.
    pub const D: Self = Uart {
        baud: 0,
        clock: &Clock::UART_D,
        registers: UART_D_REGISTERS,
    };

    /// Representation of UART E.
    pub const E: Self = Uart {
        baud: 0,
        clock: &Clock::UART_APE,
        registers: UART_E_REGISTERS,
    };
}

impl Uart {
    #[inline(always)]
    fn wait_cycles(&self, amount: u32) {
        let baud_rate = self.baud;
        usleep((amount * 1_000_000 + 16 * baud_rate - 1) / (16 * baud_rate));
    }

    #[inline(always)]
    fn wait_symbols(&self, amount: u32) {
        let baud_rate = self.baud;
        usleep((amount * 1_000_000 + baud_rate - 1) / baud_rate);
    }

    fn round_baud_rate(&self) -> u32 {
        let baud_rate = self.baud;

        (8 * baud_rate + 408_000_000) / (16 * baud_rate)
    }

    /// Initializes the UART with a given baud rate.
    ///
    /// NOTE: This method needs to be called once before a [`Uart`] can actually
    /// send and receive data. Further, it is required to do the respective
    /// [`pinmux`] configuration before calling this method.
    ///
    /// [`Uart`]: struct.Uart.html
    /// [`pinmux`]: ../pinmux
    pub fn init(&mut self, baud_rate: u32) {
        let uart = unsafe { &*self.registers };

        // Store the provided baud rate.
        self.baud = baud_rate;

        // Bring up the device clock.
        self.clock.enable();

        while !uart.UART_LSR_0.is_set(UART_LSR_0::TMTY) {
            // Wait for TX FIFO idle state.
        }

        // Calculate the baud rate, rounded to nearest.
        let rounded_baud_rate = self.round_baud_rate();

        // Setup UART in FIFO mode.

        // Disable interrupts.
        uart.UART_IER_DLAB_0_0.set(0);
        // Disable hardware flow control.
        uart.UART_MCR_0.set(0);
        // Enable DLAB and set word length to 8.
        uart.UART_LCR_0
            .write(UART_LCR_0::DLAB::SET + UART_LCR_0::WD_SIZE::WordLength8);
        // Divisor latch LSB.
        uart.UART_THR_DLAB_0_0.set(rounded_baud_rate & 0xFF);
        // Divisor latch MSB.
        uart.UART_IER_DLAB_0_0.set((rounded_baud_rate >> 8) & 0xFF);
        // Disable DLAB.
        uart.UART_LCR_0.modify(UART_LCR_0::DLAB::CLEAR);
        // Dummy read.
        uart.UART_SPR_0.get();
        // Wait 3 symbols for the new baud rate.
        self.wait_symbols(3);

        // Enable FIFO with default settings.

        // Enable FIFO mode.
        uart.UART_IIR_FCR_0
            .write(UART_IIR_FCR_0::EN_FIFO::Mode16550);
        // Dummy read.
        uart.UART_SPR_0.get();
        // Wait for 3 baud cycles.
        self.wait_cycles(3);

        // Flush the FIFOs and wait until they are ready.
        self.flush();
    }

    /// Reads a singly byte over UART and returns it.
    ///
    /// This method blocks until data is available to read.
    pub fn read_byte(&self) -> u8 {
        let uart = unsafe { &*self.registers };

        while !uart.UART_LSR_0.is_set(UART_LSR_0::RDR) {
            // Wait until it is possible to read data.
        }

        // Read the byte.
        uart.UART_THR_DLAB_0_0.get() as u8
    }

    /// Fills a mutable buffer of data with bytes read over UART.
    ///
    /// This method blocks until the buffer is filled.
    pub fn read(&self, data: &mut [u8]) {
        // Read the bytes one by one into the buffer.
        for i in data.iter_mut() {
            *i = self.read_byte();
        }
    }

    /// Writes a single byte over UART.
    ///
    /// This method blocks until data can be transferred.
    pub fn write_byte(&self, byte: u8) {
        let uart = unsafe { &*self.registers };

        while !uart.UART_LSR_0.is_set(UART_LSR_0::THRE) {
            // Wait until it is possible to write data.
        }

        // Transmit the byte.
        uart.UART_THR_DLAB_0_0.set(byte as u32);
    }

    /// Writes a buffer of bytes over UART.
    ///
    /// This method blocks until everything was transferred.
    pub fn write(&self, data: &[u8]) {
        // Write the bytes from the buffer.
        for byte in data.iter() {
            self.write_byte(*byte);
        }
    }

    /// Enables or disables inversion of the UART signal with the desired bitmask.
    ///
    /// See the documentation of the [`UART_IRDA_CSR_0`] bitfield for instructions
    /// on the structure of the expected bitmask.
    ///
    /// [`UART_IRDA_CSR_0`]: ./UART_IRDA_CSR_0/index.html
    pub fn invert(&self, mask: u32, enable: bool) {
        let uart = unsafe { &*self.registers };

        // Set or clear the mask on the inversion setting bits.
        if enable {
            uart.UART_IRDA_CSR_0.set(uart.UART_IRDA_CSR_0.get() | mask);
        } else {
            uart.UART_IRDA_CSR_0.set(uart.UART_IRDA_CSR_0.get() & !mask);
        }

        // Dummy read.
        uart.UART_SPR_0.get();
    }

    /// Flushes the underlying FIFOs of the Uart.
    ///
    /// This wipes out the data to read and the data that should be written, so be careful when
    /// you use it. In most cases, this method won't be needed.
    pub fn flush(&self) {
        let uart = unsafe { &*self.registers };

        while !uart.UART_LSR_0.is_set(UART_LSR_0::TMTY) {
            // Make sure there is no data being written to TX FIFO.
        }

        // Disable hardware control flow.
        uart.UART_MCR_0.set(0);
        // Dummy read.
        uart.UART_SPR_0.get();
        // Wait for 1 character time.
        // XXX: Figure out how to calculate this from code.
        usleep(96);

        // Issue flush requests for TX FIFO and RX FIFO.
        uart.UART_IIR_FCR_0.write(
            UART_IIR_FCR_0::EN_FIFO::Mode16550
                + UART_IIR_FCR_0::TX_CLR::SET
                + UART_IIR_FCR_0::RX_CLR::SET,
        );
        // Dummy read.
        uart.UART_SPR_0.get();
        // Wait for 32 baud cycles.
        self.wait_cycles(32);

        while !uart.UART_LSR_0.is_set(UART_LSR_0::TMTY) && uart.UART_LSR_0.is_set(UART_LSR_0::RDR) {
            // Wait until the FIFOs are ready.
        }

        // Re-enable hardware control flow.
        uart.UART_MCR_0.modify(UART_MCR_0::RTS_EN::SET);
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        // Write the string in its bytes representation.
        self.write(s.as_bytes());

        Ok(())
    }
}

// Safety: Whenever UARTs carry out an operation on the MMIOs, they
// wait until it is safe to modify the registers to avoid race conditions.
unsafe impl Sync for Uart {}
