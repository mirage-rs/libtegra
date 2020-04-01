//! Driver for the Tegra X1 Serial Peripheral Interface Controller.

use core::{convert::TryInto, marker::Sync};

use crate::timer::usleep;

pub use registers::*;

mod registers;

/// Representation of an SPI.
///
/// NOTE: Instances of this structure should never be created manually.
/// Refer to the public constants this structure holds, which represent
/// the controllers 1 through 4.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spi {
    // TODO: SPI device clocks?

    /// A pointer to the [`Registers`] of the device.
    ///
    /// [`Registers`]: struct.Registers.html
    registers: *const Registers,
}

// Definitions of known SPIs.

impl Spi {
    /// Representation of the SPI 1 controller.
    pub const SPI_1: Self = Spi {
        registers: SPI_1_REGISTERS,
    };

    /// Representation of the SPI 2 controller.
    pub const SPI_2: Self = Spi {
        registers: SPI_2_REGISTERS,
    };

    /// Representation of the SPI 3 controller.
    pub const SPI_3: Self = Spi {
        registers: SPI_3_REGISTERS,
    };

    /// Representation of the SPI 4 controller.
    pub const SPI_4: Self = Spi {
        registers: SPI_4_REGISTERS,
    };

    /// Representation of the SPI 5 controller.
    pub const SPI_5: Self = Spi {
        registers: SPI_5_REGISTERS,
    };

    /// Representation of the SPI 6 controller.
    pub const SPI_6: Self = Spi {
        registers: SPI_6_REGISTERS,
    };

    /// Representation of the QSPI controller.
    pub const QSPI: Self = Spi {
        registers: QSPI_REGISTERS,
    };
}

impl Spi {
    /// Waits for the SPI Controller to complete all transactions.
    fn wait_until_ready(&self) {
        let controller = unsafe { &*self.registers };

        while !controller.SPI_TRANSFER_STATUS_0.is_set(SPI_TRANSFER_STATUS_0::RDY) {
            // Wait until all transactions are completed.
        }
    }

    /// Clears the error status bits of the [`SPI_FIFO_STATUS_0`] register.
    ///
    /// [`SPI_FIFO_STATUS_0`]: ./SPI_FIFO_STATUS_0/index.html
    fn clear_fifo_status(&self) {
        let controller = unsafe { &*self.registers };

        // Clear the relevant bits.
        controller.SPI_FIFO_STATUS_0.modify(
            SPI_FIFO_STATUS_0::ERR::CLEAR
            + SPI_FIFO_STATUS_0::TX_FIFO_OVF::CLEAR
            + SPI_FIFO_STATUS_0::TX_FIFO_UNR::CLEAR
            + SPI_FIFO_STATUS_0::RX_FIFO_OVF::CLEAR
            + SPI_FIFO_STATUS_0::RX_FIFO_UNR::CLEAR
        );
    }

    /// Transmits data over SPI in PIO mode.
    ///
    /// NOTE: This method is a low-level implementation
    /// of the SPI transmit flow and doesn't validate any
    /// buffer boundaries. This task is delegated to the
    /// caller.
    fn pio_send_packet(&self, data: &[u8; 4]) -> Result<(), ()> {
        let controller = unsafe { &*self.registers };

        // Flush the FIFOs.
        self.flush_fifos();

        // Set 8-bit transfers, unpacked mode, most significant bit first.
        controller.SPI_COMMAND_0.modify(
            SPI_COMMAND_0::PACKED::CLEAR
            + SPI_COMMAND_0::BIT_LEN.val(7)
        );

        // Set the size of data blocks to be transferred.
        controller.SPI_DMA_BLK_SIZE_0.set(0);

        // Clear SPI_TRANSFER_STATUS RDY bit.
        controller.SPI_TRANSFER_STATUS_0.modify(SPI_TRANSFER_STATUS_0::RDY::CLEAR);

        // Set the transmit enable bit.
        controller.SPI_COMMAND_0.modify(SPI_COMMAND_0::TX_EN::SET);

        // Load in the data to write.
        controller.SPI_TX_FIFO_0.set(u32::from_le_bytes(*data));

        // Make sure that the register is stabilized before setting the PIO bit.
        usleep(2);

        // Set the PIO bit to start transaction.
        controller.SPI_COMMAND_0.modify(SPI_COMMAND_0::PIO::Go);

        // Delay for a few CPU cycles to process the data.
        usleep(1);

        // Dummy read.
        controller.SPI_COMMAND_0.get();

        // Wait for the transaction to complete.
        self.wait_until_ready();

        // Clear the transmit enable bit.
        controller.SPI_COMMAND_0.modify(SPI_COMMAND_0::TX_EN::CLEAR);

        // Check for errors.
        if controller.SPI_FIFO_STATUS_0.is_set(SPI_FIFO_STATUS_0::ERR) {
            self.clear_fifo_status();
            return Err(());
        }

        Ok(())
    }

    /// Receives data over SPI in PIO mode.
    ///
    /// NOTE: This method is a low-level implementation
    /// of the SPI receive flow and doesn't validate any
    /// buffer boundaries. This task is delegated to the
    /// caller.
    fn pio_receive_packet(&self, data: &mut [u8; 4]) -> Result<(), ()> {
        let controller = unsafe { &*self.registers };

        // Flush the FIFOs.
        self.flush_fifos();

        // Set 8-bit transfers, unpacked mode, most significant bit first.
        controller.SPI_COMMAND_0.modify(
            SPI_COMMAND_0::PACKED::CLEAR
            + SPI_COMMAND_0::BIT_LEN.val(7)
        );

        // Set the size of data blocks to be transferred.
        controller.SPI_DMA_BLK_SIZE_0.set(0);

        // Clear SPI_TRANSFER_STATUS RDY bit.
        controller.SPI_TRANSFER_STATUS_0.modify(SPI_TRANSFER_STATUS_0::RDY::CLEAR);

        // Set the receive enable bit.
        controller.SPI_COMMAND_0.modify(SPI_COMMAND_0::RX_EN::SET);

        // Make sure that the register is stabilized before setting the PIO bit.
        usleep(2);

        // Set the PIO bit to start transaction.
        controller.SPI_COMMAND_0.modify(SPI_COMMAND_0::PIO::Go);

        // Delay for a few CPU cycles to process the data.
        usleep(1);

        // Dummy read.
        controller.SPI_COMMAND_0.get();

        // Wait for the transaction to complete.
        self.wait_until_ready();

        // Clear the receive enable bit.
        controller.SPI_COMMAND_0.modify(SPI_COMMAND_0::RX_EN::CLEAR);

        // Check for errors.
        if controller.SPI_FIFO_STATUS_0.is_set(SPI_FIFO_STATUS_0::ERR) {
            self.clear_fifo_status();
            return Err(());
        }

        // Read the data bytes into the buffer.
        *data = controller.SPI_RX_FIFO_0.get().to_le_bytes();

        Ok(())
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

        // Set chip-select value to high, 8-bit transfers,
        // unpacked mode and most significant bit first.
        controller.SPI_COMMAND_0.modify(
            SPI_COMMAND_0::CS_SW_HW::SET
            + SPI_COMMAND_0::CS_SW_VAL::SET
            + SPI_COMMAND_0::PACKED::CLEAR
            + SPI_COMMAND_0::BIT_LEN.val(7)
        );

        // Flush the FIFOs.
        self.flush_fifos();

        // Enforce chip-select line 0 for now and drive chip-select low.
        controller.SPI_COMMAND_0.modify(
            SPI_COMMAND_0::CS_SEL.val(0)
            + SPI_COMMAND_0::CS_SW_VAL::CLEAR
        );
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
        controller.SPI_FIFO_STATUS_0.modify(
            SPI_FIFO_STATUS_0::RX_FIFO_FLUSH::SET
            + SPI_FIFO_STATUS_0::TX_FIFO_FLUSH::SET
        );

        while controller.SPI_FIFO_STATUS_0.is_set(SPI_FIFO_STATUS_0::RX_FIFO_FLUSH)
            && controller.SPI_FIFO_STATUS_0.is_set(SPI_FIFO_STATUS_0::TX_FIFO_FLUSH)
        {
            // Wait for the changes to take effect.
        }
    }

    /// Reads a buffer of data from a slave over SPI.
    ///
    /// NOTE: Currently, only PIO mode transfers are supported.
    pub fn read(&self, data: &mut [u8]) -> Result<(), ()> {
        if data.len() % 4 != 0 {
            return Err(());
        }

        // TODO: Add DMA support.

        for chunk in data.chunks_mut(4) {
            self.pio_receive_packet(chunk.try_into().unwrap())?;
        }

        Ok(())
    }

    /// Writes a buffer of data to a slave over SPI.
    ///
    /// NOTE: Currently, only PIO mode transfers are supported.
    pub fn write(&self, data: &[u8]) -> Result<(), ()> {
        if data.len() % 4 != 0 {
            return Err(());
        }

        // TODO: Add DMA support.

        for chunk in data.chunks(4) {
            self.pio_send_packet(chunk.try_into().unwrap())?;
        }

        Ok(())
    }
}

unsafe impl Sync for Spi {}
