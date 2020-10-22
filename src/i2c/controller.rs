use core::convert::TryInto;

use crate::{car::Clock, timer};

use super::registers::*;

fn bytes_to_word(buf: &[u8]) -> u32 {
    if buf.len() == 4 {
        u32::from_le_bytes(buf.try_into().unwrap())
    } else {
        let mut padded_buf = [0; 4];
        padded_buf[..buf.len()].copy_from_slice(buf);

        u32::from_le_bytes(buf.try_into().unwrap())
    }
}

fn bytes_to_double_word(buf: &[u8]) -> (u32, u32) {
    if buf.len() > 4 {
        (bytes_to_word(&buf[..4]), bytes_to_word(&buf[4..]))
    } else {
        (bytes_to_word(buf), 0)
    }
}

/// I2C errors that may occur during communication over the I²C protocol.
#[derive(Clone, Copy, Debug)]
pub enum Error {
    /// Generic I²C error. Not closer specified.
    Generic,
    /// An issue with memory organization of I²C data, for example if
    /// a buffer blows the boundaries of an I2C register.
    MemoryError,
    /// An I/O error that occurred during communication
    /// over I²C. Indicated through the MMIOs.
    IoError,
    /// The I²C bus encountered a timeout during a data transfer.
    Timeout,
}

/// Representation of an I2C controller device.
///
/// Tegra X1 boards expose 6 different I2C controllers with identical functionality.
/// These are exposed and can be accessed through the constants held by this structure.
///
/// Before using any of those, make sure that the bus is properly initialized and ready
/// for data transfers by calling [`I2c::init`] with the correct Pin Multiplexing for
/// the controller being done manually in advance.
///
/// Implementations of the blocking I2C traits from the `embedded-hal` crate are provided,
/// if the feature is enabled during build.
///
/// [`I2c::init`]: #fn.init.html
/// [`I2c::receive_normal`]: #fn.receive_normal.html
#[derive(Debug)]
pub struct I2c {
    /// A reference of the device clock that corresponds to the I2C controller.
    clock: &'static Clock,
    /// A pointer to the [`Registers`] of the device.
    ///
    /// [`Registers`]: struct.Registers.html
    registers: *const Registers,
}

// Definitions of known I2C controllers.

impl I2c {
    /// Representation of the I2C1 controller.
    pub const C1: Self = I2c {
        clock: &Clock::I2C_1,
        registers: I2C_1_REGISTERS,
    };

    /// Representation of the I2C2 controller.
    pub const C2: Self = I2c {
        clock: &Clock::I2C_2,
        registers: I2C_2_REGISTERS,
    };

    /// Representation of the I2C3 controller.
    pub const C3: Self = I2c {
        clock: &Clock::I2C_3,
        registers: I2C_3_REGISTERS,
    };

    /// Representation of the I2C4 controller.
    pub const C4: Self = I2c {
        clock: &Clock::I2C_4,
        registers: I2C_4_REGISTERS,
    };

    /// Representation of the I2C5 controller.
    pub const C5: Self = I2c {
        clock: &Clock::I2C_5,
        registers: I2C_5_REGISTERS,
    };

    /// Representation of the I2C6 controller.
    pub const C6: Self = I2c {
        clock: &Clock::I2C_6,
        registers: I2C_6_REGISTERS,
    };
}

impl I2c {
    fn load_config(&self) {
        let i2c = unsafe { &*self.registers };

        i2c.I2C_I2C_CONFIG_LOAD_0.write(
            I2C_I2C_CONFIG_LOAD_0::UNKNOWN::SET
                + I2C_I2C_CONFIG_LOAD_0::TIMEOUT_CONFIG_LOAD::SET
                + I2C_I2C_CONFIG_LOAD_0::MSTR_CONFIG_LOAD::SET,
        );

        // Wait a bit for master configuration to be loaded.
        for _ in 0..20 {
            timer::usleep(1);

            if !i2c
                .I2C_I2C_CONFIG_LOAD_0
                .is_set(I2C_I2C_CONFIG_LOAD_0::MSTR_CONFIG_LOAD)
            {
                break;
            }
        }
    }

    /// A lower-level API to write a buffer of data to a given I2C device.
    ///
    /// Consider using [`I2c::write`] if you have no special reason to use this
    /// method. If you have, make sure that the first byte of the `buffer` argument
    /// is the desired target register of the device and that the contents of the
    /// supplied buffer do not exceed 8 bytes in size.
    ///
    /// This method drives the I2C controller to operate in Normal Mode, using 7-bit
    /// addressing transactions. Devices are implementation-specific, no guarantees
    /// are made that data will be written to actual, valid devices.
    ///
    /// [`I2c::write`]: #fn.write.html
    pub fn send_normal(&self, device: u32, buffer: &[u8]) -> Result<(), Error> {
        assert!(
            buffer.len() <= 8,
            "Normal Mode I2C does not support transfers over 8 bytes"
        );

        let i2c = unsafe { &*self.registers };

        // Set device for 7-bit write mode.
        i2c.I2C_I2C_CMD_ADDR0_0
            .write(I2C_I2C_CMD_ADDR0_0::ADDR0.val((device << 1) | 0));

        // Load in data to transmit.
        let (data1_value, data2_value) = bytes_to_double_word(buffer);
        i2c.I2C_I2C_CMD_DATA1_0.set(data1_value); // Set the LS value.
        if data2_value != 0 {
            i2c.I2C_I2C_CMD_DATA2_0.set(data2_value); // Set the MS value.
        }

        // Set packet size and send mode.
        i2c.I2C_I2C_CNFG_0.write(
            I2C_I2C_CNFG_0::LENGTH.val(buffer.len() as u32 - 1)
                + I2C_I2C_CNFG_0::DEBOUNCE_CNT::FourT
                + I2C_I2C_CNFG_0::NEW_MASTER_FSM::SET
                + I2C_I2C_CNFG_0::CMD1::Write,
        );

        // Load the new configuration onto the bus.
        self.load_config();

        // Kick off the transfer.
        i2c.I2C_I2C_CNFG_0
            .modify(I2C_I2C_CNFG_0::PACKET_MODE_EN::CLEAR + I2C_I2C_CNFG_0::SEND::SET);

        let timeout = timer::get_milliseconds() + 100;
        while i2c.I2C_I2C_STATUS_0.is_set(I2C_I2C_STATUS_0::BUSY) {
            // Wait until not busy or facing a timeout.
            if timer::get_milliseconds() > timeout {
                return Err(Error::Timeout);
            }
        }

        // Check whether the transaction was successful and determine the appropriate Result.
        if i2c
            .I2C_I2C_STATUS_0
            .matches_all(I2C_I2C_STATUS_0::CMD1_STAT::Success)
        {
            Ok(())
        } else {
            Err(Error::IoError)
        }
    }

    /// A lower-level API to read into a buffer of data from a given I2C device.
    ///
    /// Consider using [`I2c::read`] if you have no special reason to use this
    /// method. If you have, make sure to send a buffer consisting of only the
    /// register byte to the desired device with [`I2c::send_normal`] first. After
    /// that, this method can be used to read up to 8 bytes of data into a supplied
    /// buffer.
    ///
    /// This method drives the I2C controller to operate in Normal Mode, using 7-bit
    /// addressing transactions. Devices are implementation-specific, no guarantees
    /// are made that data will be read from actual, valid devices.
    ///
    /// [`I2c::read`]: #fn.read.html
    /// [`I2c::send_normal`]: #fn.send_normal.html
    pub fn receive_normal(&self, device: u32, buffer: &mut [u8]) -> Result<(), Error> {
        assert!(
            buffer.len() <= 8,
            "Normal Mode I2C does not support transfers over 8 bytes"
        );

        let i2c = unsafe { &*self.registers };

        // Set device for 7-bit read mode.
        i2c.I2C_I2C_CMD_ADDR0_0
            .write(I2C_I2C_CMD_ADDR0_0::ADDR0.val((device << 1) | 1));

        // Set size and receive mode.
        i2c.I2C_I2C_CNFG_0.write(
            I2C_I2C_CNFG_0::LENGTH.val(buffer.len() as u32 - 1)
                + I2C_I2C_CNFG_0::DEBOUNCE_CNT::FourT
                + I2C_I2C_CNFG_0::NEW_MASTER_FSM::SET
                + I2C_I2C_CNFG_0::CMD1::Read,
        );

        // Load the new configuration onto the bus.
        self.load_config();

        // Kick off the transfer.
        i2c.I2C_I2C_CNFG_0
            .modify(I2C_I2C_CNFG_0::PACKET_MODE_EN::CLEAR + I2C_I2C_CNFG_0::SEND::SET);

        let timeout = timer::get_milliseconds() + 100;
        while i2c.I2C_I2C_STATUS_0.is_set(I2C_I2C_STATUS_0::BUSY) {
            // Wait until not busy or facing a timeout.
            if timer::get_milliseconds() > timeout {
                return Err(Error::Timeout);
            }
        }

        // Check whether the transaction was successful and determine the appropriate Result.
        if i2c
            .I2C_I2C_STATUS_0
            .matches_all(I2C_I2C_STATUS_0::CMD1_STAT::Success)
        {
            // Read and copy back the result.
            let data1 = i2c.I2C_I2C_CMD_DATA1_0.get().to_le_bytes();
            if buffer.len() > 4 {
                let data2 = i2c.I2C_I2C_CMD_DATA2_0.get().to_le_bytes();

                // Copy both, LS and MS values.
                buffer[..4].copy_from_slice(&data1);
                {
                    let size = buffer.len() - 4;
                    buffer[4..].copy_from_slice(&data2[..size]);
                }
            } else {
                // Only copy the LS value.
                buffer.copy_from_slice(&data1[..buffer.len()]);
            }

            Ok(())
        } else {
            Err(Error::IoError)
        }
    }

    /// Initializes the I2C device for data transfers.
    ///
    /// NOTE: This method must be called once before the I2C device is usable.
    /// Further, it is required to do the respective [`pin`] configuration before
    /// calling this method.
    ///
    /// [`pin`]: ../pinmux
    pub fn init(&self) {
        let i2c = unsafe { &*self.registers };

        // Enable the device clock.
        self.clock.enable();

        // Setup divisor and clear the bus.
        i2c.I2C_I2C_CLK_DIVISOR_REGISTER_0.write(
            I2C_I2C_CLK_DIVISOR_REGISTER_0::I2C_CLK_DIVISOR_STD_FAST_MODE.val(5)
                + I2C_I2C_CLK_DIVISOR_REGISTER_0::I2C_CLK_DIVISOR_HSMODE::SET,
        );
        i2c.I2C_I2C_BUS_CLEAR_CONFIG_0.write(
            I2C_I2C_BUS_CLEAR_CONFIG_0::BC_SCLK_THRESHOLD.val(9)
                + I2C_I2C_BUS_CLEAR_CONFIG_0::BC_TERMINATE::SET
                + I2C_I2C_BUS_CLEAR_CONFIG_0::BC_ENABLE::SET,
        );

        // Load hardware configuration.
        self.load_config();

        // Wait a while until BUS_CLEAR_DONE is set.
        for _ in 0..10 {
            timer::usleep(20_000);

            if i2c
                .I2C_INTERRUPT_STATUS_REGISTER_0
                .is_set(I2C_INTERRUPT_STATUS_REGISTER_0::BUS_CLEAR_DONE)
            {
                break;
            }
        }

        // Dummy read.
        i2c.I2C_I2C_BUS_CLEAR_STATUS_0.get();

        // Read and set the Interrupt Status.
        i2c.I2C_INTERRUPT_STATUS_REGISTER_0
            .set(i2c.I2C_INTERRUPT_STATUS_REGISTER_0.get());
    }

    /// Writes a buffer of data to a device register over I²C.
    ///
    /// This method drives the I2C controller to operate in Normal Mode, using 7-bit
    /// addressing transactions. Devices and their registers are implementation-specific, no
    /// guarantees are made that data will be written to actual, valid devices.
    pub fn write(&self, device: u32, register: u8, data: &[u8]) -> Result<(), Error> {
        // Boundary checks, since a buffer cannot exceed 8 bytes
        // and one byte is always reserved for the register.
        if data.len() > 7 {
            return Err(Error::MemoryError);
        }

        // Prepare the I2C data.
        let mut packet = [0; 8];
        packet[0] = register;
        packet[1..=data.len()].copy_from_slice(data);

        // Write the data to the device.
        self.send_normal(device, &packet[..])
    }

    /// Reads the contents of a device register over I²C.
    ///
    /// This method drives the I2C controller to operate in Normal Mode, using 7-bit
    /// addressing transactions. Devices and their registers are implementation-specific, no
    /// guarantees are made that data will be read from actual, valid devices.
    pub fn read(&self, device: u32, register: u8, buffer: &mut [u8]) -> Result<(), Error> {
        // Limit output buffer size to 8 bytes, as one
        // cannot read a higher number of bytes anyway.
        if buffer.len() > 8 {
            return Err(Error::MemoryError);
        }

        // Write single byte register ID to device.
        self.send_normal(device, &[register])?;

        // Read data and copy them into the buffer.
        self.receive_normal(device, buffer)
    }

    /// Writes a single data byte to a device register over I²C.
    #[inline(always)]
    pub fn write_byte(&self, device: u32, register: u8, byte: u8) -> Result<(), Error> {
        self.write(device, register, &byte.to_le_bytes())
    }

    /// Reads a single data byte from a device register over I²C.
    #[inline(always)]
    pub fn read_byte(&self, device: u32, register: u8) -> Result<u8, Error> {
        let mut buffer = [0; 1];
        self.read(device, register, &mut buffer)?;

        Ok(u8::from_le_bytes(buffer.try_into().unwrap()))
    }
}
