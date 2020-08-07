//! Implementation of Tegra X1 I²C master devices.
//!
//! See Chapter 35.7 in the Tegra X1 Technical Reference Manual
//! for details.

use core::convert::TryInto;

use crate::{car::Clock, timer};

use super::registers::*;

/// Enumeration of potential errors that may occur
/// during communication over the I²C protocol.
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

/// Representation of an I2C device.
///
/// NOTE: Instances of this struct should never be created manually.
/// Refer to the public constants this struct holds, which represent
/// the controllers 1 through 6.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    /// Representation of the I2C 1 controller.
    pub const C1: Self = I2c {
        clock: &Clock::I2C_1,
        registers: I2C_1_REGISTERS,
    };

    /// Representation of the I2C 2 controller.
    pub const C2: Self = I2c {
        clock: &Clock::I2C_2,
        registers: I2C_2_REGISTERS,
    };

    /// Representation of the I2C 3 controller.
    pub const C3: Self = I2c {
        clock: &Clock::I2C_3,
        registers: I2C_3_REGISTERS,
    };

    /// Representation of the I2C 4 controller.
    pub const C4: Self = I2c {
        clock: &Clock::I2C_4,
        registers: I2C_4_REGISTERS,
    };

    /// Representation of the I2C 5 controller.
    pub const C5: Self = I2c {
        clock: &Clock::I2C_5,
        registers: I2C_5_REGISTERS,
    };

    /// Representation of the I2C 6 controller.
    pub const C6: Self = I2c {
        clock: &Clock::I2C_6,
        registers: I2C_6_REGISTERS,
    };
}

impl I2c {
    fn load_config(&self) {
        let register_base = unsafe { &*self.registers };

        // Set MSTR_CONFIG_LOAD, TIMEOUT_CONFIG_LOAD, undocumented bit.
        register_base.I2C_I2C_CONFIG_LOAD_0.set(0x25);

        // Wait a bit for master configuration to be loaded.
        for _ in 0..20 {
            timer::usleep(1);

            if register_base.I2C_I2C_CONFIG_LOAD_0.get() & 1 == 0 {
                break;
            }
        }
    }

    fn send_packet(&self, slave: u32, packet: &[u8]) -> Result<(), Error> {
        let register_base = unsafe { &*self.registers };

        // Set device for 7-bit write mode.
        register_base.I2C_I2C_CMD_ADDR0_0.set(slave << 1);

        // Load in data to transmit.
        let (data1, data2) = packet.split_at(4);
        let data1_value = u32::from_le_bytes(data1.try_into().unwrap());
        let data2_value = u32::from_le_bytes(data2.try_into().unwrap());

        register_base.I2C_I2C_CMD_DATA1_0.set(data1_value); // Set the LS value.
        if data2_value != 0 {
            register_base.I2C_I2C_CMD_DATA2_0.set(data2_value); // Set the MS value.
        }

        // Set config with LENGTH = packet.len(), NEW_MASTER_FSM, DEBOUNCE_CNT = 4T.
        register_base
            .I2C_I2C_CNFG_0
            .set((((packet.len() - 1) << 1) | 0x2800) as u32);

        // Kick off the transaction.
        self.load_config();

        // Set CONFIG |= SEND.
        register_base
            .I2C_I2C_CNFG_0
            .set((register_base.I2C_I2C_CNFG_0.get() & 0xFFFF_FDFF) | 0x200);

        let timeout = timer::get_milliseconds() + 1500;
        while (register_base.I2C_I2C_STATUS_0.get() & 0x100) != 0 {
            // Wait until not busy or facing a timeout.
            if timer::get_milliseconds() > timeout {
                return Err(Error::Timeout);
            }
        }

        // Check whether the transaction was successful and determine the appropriate Result.
        if (register_base.I2C_I2C_STATUS_0.get() & 0xF) == 0 {
            Ok(())
        } else {
            Err(Error::IoError)
        }
    }

    fn receive_packet(&self, slave: u32, buffer: &mut [u8]) -> Result<(), Error> {
        let register_base = unsafe { &*self.registers };

        // Set device for 7-bit read mode.
        register_base.I2C_I2C_CMD_ADDR0_0.set((slave << 1) | 1);

        // Set config with LENGTH = buffer.len(), NEW_MASTER_FSM, DEBOUNCE_CNT = 4T.
        register_base
            .I2C_I2C_CNFG_0
            .set((((buffer.len() - 1) << 1) | 0x2840) as u32);

        // Kick off the transaction.
        self.load_config();

        // Set CONFIG |= SEND.
        register_base
            .I2C_I2C_CNFG_0
            .set((register_base.I2C_I2C_CNFG_0.get() & 0xFFFF_FDFF) | 0x200);

        let timeout = timer::get_milliseconds() + 1500;
        while (register_base.I2C_I2C_STATUS_0.get() & 0x100) != 0 {
            // Wait until not busy or facing a timeout.
            if timer::get_milliseconds() > timeout {
                return Err(Error::Timeout);
            }
        }

        // Check whether the transaction was successful and determine the appropriate Result.
        if (register_base.I2C_I2C_STATUS_0.get() & 0xF) == 0 {
            // Read and copy back the result.
            let data1 = register_base.I2C_I2C_CMD_DATA1_0.get().to_le_bytes();
            let data2 = register_base.I2C_I2C_CMD_DATA2_0.get().to_le_bytes();
            if buffer.len() > 4 {
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

    /// Initializes the I2C device.
    ///
    /// NOTE: This method must be called once before the I2C device is usable.
    /// Further, it is required to do the respective [`pinmux`] configuration
    /// before calling this method.
    ///
    /// [`pinmux`]: ../pinmux
    pub fn init(&self) {
        let register_base = unsafe { &*self.registers };

        // Enable the device clock.
        self.clock.enable();

        // Setup divisor and clear the bus.
        register_base.I2C_I2C_CLK_DIVISOR_REGISTER_0.set(0x50001);
        register_base.I2C_I2C_BUS_CLEAR_CONFIG_0.set(0x90003);

        // Load hardware configuration.
        self.load_config();

        // Wait a while until BUS_CLEAR_DONE is set.
        for _ in 0..10 {
            timer::usleep(20_000);

            if (register_base.I2C_INTERRUPT_STATUS_REGISTER_0.get() & 0x800) != 0 {
                break;
            }
        }

        // Dummy read.
        register_base.I2C_I2C_BUS_CLEAR_STATUS_0.get();

        // Read and set the Interrupt Status.
        register_base
            .I2C_INTERRUPT_STATUS_REGISTER_0
            .set(register_base.I2C_INTERRUPT_STATUS_REGISTER_0.get());
    }

    /// Writes a buffer of data to a slave register over I²C.
    pub fn write(&self, slave: u32, register: u8, data: &[u8]) -> Result<(), Error> {
        // Boundary checks, since a buffer cannot exceed 8 bytes
        // and one byte is always reserved for the register.
        if data.len() > 7 {
            return Err(Error::MemoryError);
        }

        // Prepare the I²C packet.
        let mut packet = [0; 8];
        packet[0] = register;
        packet[1..=data.len()].copy_from_slice(data);

        // Write the packet to the device.
        self.send_packet(slave, &packet[..])
    }

    /// Reads the contents of a slave register over I²C.
    pub fn read(&self, slave: u32, register: u8, buffer: &mut [u8]) -> Result<(), Error> {
        // Limit output buffer size to 8 bytes, as one
        // cannot read a higher number of bytes anyway.
        if buffer.len() > 8 {
            return Err(Error::MemoryError);
        }

        // Write single byte register ID to device.
        self.send_packet(slave, &[register])?;

        // Read data and copy them into the buffer.
        self.receive_packet(slave, buffer)
    }

    /// Writes a single byte of data to a slave register over I²C.
    #[inline(always)]
    pub fn write_byte(&self, slave: u32, register: u8, byte: u8) -> Result<(), Error> {
        self.write(slave, register, &byte.to_le_bytes())
    }

    /// Reads a single byte of data from a slave register over I²C.
    #[inline(always)]
    pub fn read_byte(&self, slave: u32, register: u8) -> Result<u8, Error> {
        let mut buffer = [0; 1];
        self.read(slave, register, &mut buffer)?;

        Ok(u8::from_le_bytes(buffer.try_into().unwrap()))
    }
}
