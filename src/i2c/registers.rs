//! Abstractions over the registers of the Tegra X1 I2C controllers.
//!
//! See Chapter 35.6 in the Tegra X1 Technical Reference Manual
//! for details.

use register::{mmio::*, register_structs};

/// Base address for I2C registers.
const I2C_BASE: u32 = 0x7000_C000;

/// A pointer to the I2C 1 register block that can be accessed by dereferencing it.
pub const I2C_1_REGISTERS: *const Registers = (I2C_BASE + 0x0000)  as *const Registers;
/// A pointer to the I2C 2 register block that can be accessed by dereferencing it.
pub const I2C_2_REGISTERS: *const Registers = (I2C_BASE + 0x0400)  as *const Registers;
/// A pointer to the I2C 3 register block that can be accessed by dereferencing it.
pub const I2C_3_REGISTERS: *const Registers = (I2C_BASE + 0x0500)  as *const Registers;
/// A pointer to the I2C 4 register block that can be accessed by dereferencing it.
pub const I2C_4_REGISTERS: *const Registers = (I2C_BASE + 0x0700)  as *const Registers;
/// A pointer to the I2C 5 register block that can be accessed by dereferencing it.
pub const I2C_5_REGISTERS: *const Registers = (I2C_BASE + 0x1000)  as *const Registers;
/// A pointer to the I2C 6 register block that can be accessed by dereferencing it.
pub const I2C_6_REGISTERS: *const Registers = (I2C_BASE + 0x1100)  as *const Registers;

// TODO: Bitfields.

register_structs! {
    /// Representation of the I2C registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => pub I2C_I2C_CNFG_0: ReadWrite<u32>),
        (0x04 => pub I2C_I2C_CMD_ADDR0_0: ReadWrite<u32>),
        (0x08 => pub I2C_I2C_CMD_ADDR1_0: ReadWrite<u32>),
        (0x0C => pub I2C_I2C_CMD_DATA1_0: ReadWrite<u32>),
        (0x10 => pub I2C_I2C_CMD_DATA2_0: ReadWrite<u32>),
        (0x14 => reserved0: [ReadWrite<u32>; 0x2]),
        (0x1C => pub I2C_I2C_STATUS_0: ReadOnly<u32>),
        (0x20 => pub I2C_I2C_SL_CNFG_0: ReadWrite<u32>),
        (0x24 => pub I2C_I2C_SL_RCVD_0: ReadWrite<u32>),
        (0x28 => pub I2C_I2C_SL_STATUS_0: ReadWrite<u32>),
        (0x2C => pub I2C_I2C_SL_ADDR1_0: ReadWrite<u32>),
        (0x30 => pub I2C_I2C_SL_ADDR2_0: ReadWrite<u32>),
        (0x34 => pub I2C_I2C_TLOW_SEXT_0: ReadWrite<u32>),
        (0x38 => reserved1: ReadWrite<u32>),
        (0x3C => pub I2C_I2C_SL_DELAY_COUNT_0: ReadWrite<u32>),
        (0x40 => pub I2C_I2C_SL_INT_MASK_0: ReadWrite<u32>),
        (0x44 => pub I2C_I2C_SL_INT_SOURCE_0: ReadOnly<u32>),
        (0x48 => pub I2C_I2C_SL_INT_SET_0: ReadWrite<u32>),
        (0x4C => reserved2: ReadWrite<u32>),
        (0x50 => pub I2C_I2C_TX_PACKET_FIFO_0: ReadWrite<u32>),
        (0x54 => pub I2C_I2C_RX_FIFO_0: ReadOnly<u32>),
        (0x58 => pub I2C_PACKET_TRANSFER_STATUS_0: ReadOnly<u32>),
        (0x5C => pub I2C_FIFO_CONTROL_0: ReadWrite<u32>),
        (0x60 => pub I2C_FIFO_STATUS_0: ReadOnly<u32>),
        (0x64 => pub I2C_INTERRUPT_MASK_REGISTER_0: ReadWrite<u32>),
        (0x68 => pub I2C_INTERRUPT_STATUS_REGISTER_0: ReadWrite<u32>),
        (0x6C => pub I2C_I2C_CLK_DIVISOR_REGISTER_0: ReadWrite<u32>),
        (0x70 => pub I2C_I2C_INTERRUPT_SOURCE_REGISTER_0: ReadOnly<u32>),
        (0x74 => pub I2C_I2C_INTERRUPT_SET_REGISTER_0: ReadWrite<u32>),
        (0x78 => pub I2C_I2C_SLV_TX_PACKET_FIFO_0: ReadWrite<u32>),
        (0x7C => pub I2C_I2C_SLV_RX_FIFO_0: ReadOnly<u32>),
        (0x80 => pub I2C_I2C_SLV_PACKET_STATUS_0: ReadOnly<u32>),
        (0x84 => pub I2C_I2C_BUS_CLEAR_CONFIG_0: ReadWrite<u32>),
        (0x88 => pub I2C_I2C_BUS_CLEAR_STATUS_0: ReadOnly<u32>),
        (0x8C => pub I2C_I2C_CONFIG_LOAD_0: ReadWrite<u32>),
        (0x90 => reserved3: ReadWrite<u32>),
        (0x94 => pub I2C_I2C_INTERFACE_TIMING_0_0: ReadWrite<u32>),
        (0x98 => pub I2C_I2C_INTERFACE_TIMING_1_0: ReadWrite<u32>),
        (0x9C => pub I2C_I2C_HS_INTERFACE_TIMING_0_0: ReadWrite<u32>),
        (0xA0 => pub I2C_I2C_HS_INTERFACE_TIMING_1_0: ReadWrite<u32>),
        (0xA4 => @END),
    }
}
