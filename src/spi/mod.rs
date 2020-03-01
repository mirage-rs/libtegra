//! Driver for the Tegra X1 Serial Peripheral Interface Controller.

pub use registers::*;

mod registers;

/// Representation of an SPI.
///
/// NOTE: Instances of this structure should never be created manually.
/// Refer to the public constants this structure holds, which represent
/// the controllers 1 through 4.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spi {
    /// A pointer to the [`Registers`] of the device.
    ///
    /// [`Registers`]: struct.Registers.html
    registers: *const Registers,
}

impl Spi {
    /// Waits for the SPI Controller to complete all transactions.
    fn wait_until_ready(&self) {
        let controller = unsafe { &*self.registers };

        while !controller.SPI_TRANSFER_STATUS_0.is_set(SPI_TRANSFER_STATUS_0::RDY) {
            // Wait until all transactions are completed.
        }
    }

    /// Initializes the SPI controller.
    ///
    /// NOTE: This method must be called once before an SPI device is usable.
    /// Further, it is required to do the respective [`pinmux`] configuration
    /// before calling this method.
    ///
    /// [`pinmux`]: ../pinmux
    pub fn init(&self) {
        let controller = unsafe { &*self.registers };

        // TODO: Use the register bitflags instead of toggling bits manually.

        let mut command1 = controller.SPI_COMMAND_0.get();

        // Software drives chip-select, set value to high.
        command1 |= (1 << 21) | (1 << 20);

        // Enable 8-bit transfers, unpacked mode and most significant bit first.
        command1 &= !((31 << 0) | (1 << 5));
        command1 |= 7 << 0;

        // Initialize the controller.
        controller.SPI_COMMAND_0.set(command1);

        // Flush the FIFOs.
        self.flush_fifos();

        // Force chip select 0 for now.
        let cs = 0;

        command1 = controller.SPI_COMMAND_0.get();

        // Select appropriate chip-select line.
        command1 &= !(3 << 26);
        command1 |= cs << 26;

        // Drive chip-select low.
        command1 &= !(1 << 20);

        // Start the controller.
        controller.SPI_COMMAND_0.set(command1);
    }

    /// Flushes the underlying FIFOs of the UART.
    ///
    /// NOTE: This method flushes both, TX FIFO and RX FIFO,
    /// so be careful when you use it.
    pub fn flush_fifos(&self) {
        let controller = unsafe { &*self.registers };

        // Make sure the controller is in idle state.
        self.wait_until_ready();

        // Issue flush requests for TX FIFO and RX FIFO.
        controller
            .SPI_FIFO_STATUS_0
            .modify(SPI_FIFO_STATUS_0::RX_FIFO_FLUSH::SET + SPI_FIFO_STATUS_0::TX_FIFO_FLUSH::SET);

        while controller.SPI_FIFO_STATUS_0.is_set(SPI_FIFO_STATUS_0::RX_FIFO_FLUSH)
            && controller.SPI_FIFO_STATUS_0.is_set(SPI_FIFO_STATUS_0::TX_FIFO_FLUSH)
        {
            // Wait for the changes to take effect.
        }
    }
}
