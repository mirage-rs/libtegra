//! Abstractions over the SPI Controller registers of the Tegra X1.
//!
//! See Chapter 37.3 in the Tegra X1 Technical Reference Manual
//! for details.

use register::{mmio::*, register_bitfields, register_structs};

use crate::memory_map::spi::*;

/// A pointer to the SPI 2B-1 register block that can be accessed by dereferencing it.
pub const SPI_1_REGISTERS: *const Registers = SPI_1 as *const Registers;
/// A pointer to the SPI 2B-2 register block that can be accessed by dereferencing it.
pub const SPI_2_REGISTERS: *const Registers = SPI_2 as *const Registers;
/// A pointer to the SPI 2B-3 register block that can be accessed by dereferencing it.
pub const SPI_3_REGISTERS: *const Registers = SPI_3 as *const Registers;
/// A pointer to the SPI 2B-4 register block that can be accessed by dereferencing it.
pub const SPI_4_REGISTERS: *const Registers = SPI_4 as *const Registers;

register_bitfields! {
    u32,

    /// Bitfields of the `SPI_COMMAND_0` register.
    pub SPI_COMMAND_0 [
        /// Starts the transfer.
        ///
        /// NOTE: This is the last bit that should be programmed in this register.
        PIO OFFSET(31) NUMBITS(1) [
            Stop = 0,
            Go = 1
        ],

        /// Master/Slave Mode select.
        MS OFFSET(30) NUMBITS(1) [
            SlaveMode = 0,
            MasterMode = 1
        ],

        /// The SPI interface clock mode according to the device with which it is communicating.
        MODE OFFSET(28) NUMBITS(2) [
            Mode0 = 0,
            Mode1 = 1,
            Mode2 = 2,
            Mode3 = 3
        ],

        /// In Master Mode, these bits are used to select a slave in the multi-slave environment.
        CS_SEL OFFSET(26) NUMBITS(2) [
            /// Selects CS0.
            Cs0 = 0,
            /// Selects CS1.
            Cs1 = 1,
            /// Selects CS2.
            Cs2 = 2,
            /// Selects CS3.
            Cs3 = 3
        ],

        /// The inactive value of the external device's CS value, which is connected to CS3.
        CS_POL_INACTIVE_3 OFFSET(25) NUMBITS(1) [],

        /// The inactive value of the external device's CS value, which is connected to CS2.
        CS_POL_INACTIVE_2 OFFSET(24) NUMBITS(1) [],

        /// The inactive value of the external device's CS value, which is connected to CS1.
        CS_POL_INACTIVE_1 OFFSET(23) NUMBITS(1) [],

        /// The inactive value of the external device's CS value, which is connected to CS0.
        CS_POL_INACTIVE_0 OFFSET(22) NUMBITS(1) [],

        /// Software control of the SPI_CS signal in Master Mode.
        CS_SW_HW OFFSET(21) NUMBITS(1) [],

        /// CS signal value in Master Mode.
        CS_SW_VAL OFFSET(20) NUMBITS(1) [
            Low = 0,
            High = 1
        ],

        /// Inactive data signal format.
        IDLE_SDA OFFSET(18) NUMBITS(2) [
            DriveLow = 0,
            DriveHigh = 1,
            ExternalPullDown = 2,
            ExternalPullHigh = 3
        ],

        /// Bidirectional Transfer Control Bit.
        BIDIR OFFSET(17) NUMBITS(1) [],

        /// Whether Little Endian Bit should be enabled.
        EN_LE_BIT OFFSET(16) NUMBITS(1) [],

        /// Whether Little Endian Byte should be enabled.
        EN_LE_BYTE OFFSET(15) NUMBITS(1) [],

        /// Both MISO and MOSI can also be used to transfer at the same time when this bit is set.
        BOTH_EN_BIT OFFSET(14) NUMBITS(1) [],

        /// Receive enable.
        RX_EN OFFSET(12) NUMBITS(1) [],

        /// Transmit enable.
        TX_EN OFFSET(11) NUMBITS(1) [],

        /// Reserved for future use.
        ///
        /// NOTE: Always write `0`.
        RESERVED OFFSET(6) NUMBITS(5) [],

        /// Packed mode enable bit.
        PACKED OFFSET(5) NUMBITS(1) [],

        /// Represents the number of bits transferred in either Packed or Unpacked Mode.
        BIT_LEN OFFSET(0) NUMBITS(5) []
    ],

    /// Bitfields of the `SPI_COMMAND2_0` register.
    pub SPI_COMMAND2_0 [
        /// Reserved for future use.
        ///
        /// NOTE: Always write `0`.
        RESERVED OFFSET(12) NUMBITS(20) [],

        /// Delays the clock going out to the external device with these tap values.
        TX_CLK_TAP_DELAY OFFSET(6) NUMBITS(6) [],

        /// Delays the clock coming from the external device with these tap values.
        RX_CLK_TAP_DELAY OFFSET(0) NUMBITS(6) []
    ],

    /// Bitfields of the `SPI_TIMING_REG1_0` register.
    pub SPI_TIMING_REG1_0 [
        /// Specifies the setup time of the chip select to the data being transferred on CS#3.
        CS_SETUP_TIME_3 OFFSET(28) NUMBITS(4) [],

        /// Specifies the hold time of the chip select to the data being transferred on CS#3.
        CS_HOLD_TIME_3 OFFSET(24) NUMBITS(4) [],

        /// Specifies the setup time of the chip select to the data being transferred on CS#2.
        CS_SETUP_TIME_2 OFFSET(20) NUMBITS(4) [],

        /// Specifies the hold time of the chip select to the data being transferred on CS#2.
        CS_HOLD_TIME_2 OFFSET(16) NUMBITS(4) [],

        /// Specifies the setup time of the chip select to the data being transferred on CS#1.
        CS_SETUP_TIME_1 OFFSET(12) NUMBITS(4) [],

        /// Specifies the hold time of the chip select to the data being transferred on CS#1.
        CS_HOLD_TIME_1 OFFSET(8) NUMBITS(4) [],

        /// Specifies the setup time of the chip select to the data being transferred on CS#0.
        CS_SETUP_TIME_0 OFFSET(4) NUMBITS(4) [],

        /// Specifies the hold time of the chip select to the data being transferred on CS#0.
        CS_HOLD_TIME_0 OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `SPI_TIMING_REG2_0` register.
    pub SPI_TIMING_REG2_0 [
        /// Reserved for future use.
        ///
        /// NOTE: Always write `0`.
        RESERVED0 OFFSET(30) NUMBITS(2) [],

        /// Specifies if CS stays active between two packets on CS#3.
        CS_ACTIVE_BETWEEN_PACKETS_3 OFFSET(29) NUMBITS(1) [],

        /// Specifies the number of cycles between two packets for communication on CS#3.
        CYCLES_BETWEEN_PACKETS_3 OFFSET(24) NUMBITS(5) [],

        /// Reserved for future use.
        ///
        /// NOTE: Always write `0`.
        RESERVED1 OFFSET(22) NUMBITS(2) [],

        /// Specifies if CS stays active between two packets on CS#2.
        CS_ACTIVE_BETWEEN_PACKETS_2 OFFSET(21) NUMBITS(1) [],

        /// Specifies the number of cycles between two packets for communication on CS#2.
        CYCLES_BETWEEN_PACKETS_2 OFFSET(16) NUMBITS(5) [],

        /// Reserved for future use.
        ///
        /// NOTE: Always write `0`.
        RESERVED2 OFFSET(14) NUMBITS(2) [],

        /// Specifies if CS stays active between two packets on CS#1.
        CS_ACTIVE_BETWEEN_PACKETS_1 OFFSET(13) NUMBITS(1) [],

        /// Specifies the number of cycles between two packets for communication on CS#1.
        CYCLES_BETWEEN_PACKETS_1 OFFSET(8) NUMBITS(5) [],

        /// Reserved for future use.
        ///
        /// NOTE: Always write `0`.
        RESERVED3 OFFSET(6) NUMBITS(2) [],

        /// Specifies if CS stays active between two packets on CS#0.
        CS_ACTIVE_BETWEEN_PACKETS_0 OFFSET(5) NUMBITS(1) [],

        /// Specifies the number of cycles between two packets for communication on CS#0.
        CYCLES_BETWEEN_PACKETS_0 OFFSET(0) NUMBITS(5) []
    ],

    /// Bitfields of the `SPI_TRANSFER_STATUS_0` register.
    pub SPI_TRANSFER_STATUS_0 [
        /// Reserved for future use.
        ///
        /// NOTE: Always write `0`.
        RESERVED0 OFFSET(31) NUMBITS(1) [],

        /// Ready bit.
        RDY OFFSET(30) NUMBITS(1) [],

        /// Reserved for future use.
        ///
        /// NOTE: Always write `0`.
        RESERVED1 OFFSET(24) NUMBITS(6) [],

        /// Slave Continuous Mode.
        SLV_IDLE_COUNT OFFSET(16) NUMBITS(8) [],

        /// Counts the number of packets in a transaction in DMA/PIO Mode.
        BLOCK_COUNT OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `SPI_FIFO_STATUS_0` register.
    pub SPI_FIFO_STATUS_0 [
        /// Whether CS is deasserted when SPI is in Slave Mode.
        CS_INACTIVE OFFSET(31) NUMBITS(1) [],

        /// Status bit that is set during Slave Continuos Mode, in case of an Sclk mismatch.
        FRAME_END OFFSET(30) NUMBITS(1) [],

        /// Indicates the number of slots in the receive FIFO remaining before the FIFO is full.
        RX_FIFO_FULL_COUNT OFFSET(23) NUMBITS(7) [],

        /// Indicates the number of slots in the transmit FIFO remaining before the FIFO is full.
        TX_FIFO_EMPTY_COUNT OFFSET(16) NUMBITS(7) [],

        /// Whether the RX FIFO should be flushed.
        RX_FIFO_FLUSH OFFSET(15) NUMBITS(1) [],

        /// Whether the TX FIFO should be flushed.
        TX_FIFO_FLUSH OFFSET(14) NUMBITS(1) [],

        /// Reserved for future use.
        ///
        /// NOTE: Always write `0`.
        RESERVED OFFSET(9) NUMBITS(5) [],

        /// Will be set to 1 when errors occur.
        ERR OFFSET(8) NUMBITS(1) [
            Ok = 0,
            Error = 1
        ],

        /// TX FIFO Overflow.
        TX_FIFO_OVF OFFSET(7) NUMBITS(1) [],

        /// TX FIFO Underrun.
        TX_FIFO_UNR OFFSET(6) NUMBITS(1) [],

        /// RX FIFO Overflow.
        RX_FIFO_OVF OFFSET(5) NUMBITS(1) [],

        /// RX FIFO Underrun.
        RX_FIFO_UNR OFFSET(4) NUMBITS(1) [],

        /// TX FIFO Full Status.
        ///
        /// NOTE: This bit is read-only.
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],

        /// TX FIFO Empty Status.
        ///
        /// NOTE: This bit is read-only.
        TX_FIFO_EMPTY OFFSET(2) NUMBITS(1) [],

        /// RX FIFO Full Status.
        ///
        /// NOTE: This bit is read-only.
        RX_FIFO_FULL OFFSET(1) NUMBITS(1) [],

        /// RX FIFO Empty Status.
        ///
        /// NOTE: This bit is read-only.
        RX_FiFO_EMPTY OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `SPI_TX_DATA_0` register.
    pub SPI_TX_DATA_0 [
        /// Holds the last data that was transmitted by the SPI controller.
        SPI_TX_DATA OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `SPI_RX_DATA_0` register.
    pub SPI_RX_DATA_0 [
        /// Holds the last data that was received by the SPI controller.
        SPI_RX_DATA OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `SPI_DMA_CTL_0` register.
    pub SPI_DMA_CTL_0 [
        /// Whether DMA Mode transfer should be enabled.
        DMA OFFSET(31) NUMBITS(1) [],

        /// Whether Continous Mode transfer should be enabled.
        CONT OFFSET(30) NUMBITS(1) [],

        /// Reserved for future use.
        ///
        /// NOTE: Always write `0`.
        RESERVED0 OFFSET(21) NUMBITS(7) [],

        /// Receive FIFO trigger level.
        RX_TRIG OFFSET(19) NUMBITS(2) [
            /// DMA trigger is asserted when there is at least 1 packet in RX FIFO.
            OneWord = 00,
            /// DMA trigger is asserted when there are at least 4 packets in RX FIFO.
            FourWords = 01,
            /// DMA trigger is asserted when there are at least 8 packets in RX FIFO.
            EightWords = 10,
            /// DMA trigger is asserted when there are at least 16 packets in RX FIFO.
            SixteenWords = 11
        ],

        /// Reserved for future use.
        ///
        /// NOTE: Always write `0`.
        RESERVED1 OFFSET(17) NUMBITS(2) [],

        /// Transmit FIFO trigger level.
        TX_TRIG OFFSET(15) NUMBITS(2) [
            /// DMA trigger is asserted whenever there is space for at least 1 packet in TX FIFO.
            OneWord = 00,
            /// DMA trigger is asserted whenever there is space for at least 4 packets in TX FIFO.
            FourWords = 01,
            /// DMA trigger is asserted whenever there is space for at least 8 packets in TX FIFO.
            EightWords = 10,
            /// DMA trigger is asserted whenever there is space for at least 16 packets in TX FIFO.
            SixteenWords = 11
        ]
    ],

    /// Bitfields of the `SPI_DMA_BLK_SIZE_0` register.
    pub SPI_DMA_BLK_SIZE_0 [
        /// Size of data blocks to be transferred in PIO/DMA Mode.
        BLOCK_SIZE OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `SPI_TX_FIFO_0` register.
    pub SPI_TX_FIFO_0 [
        /// The data to be inserted into TX FIFO.
        SPI_TX_FIFO OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `SPI_RX_FIFO_0` register.
    pub SPI_RX_FIFO_0 [
        /// The data to be read from RX FIFO.
        SPI_RX_FIFO OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `SPI_INTR_MASK_0` register.
    pub SPI_INTR_MASK_0 [
        /// Interrupt enable mask bit for CS deassertion in Slave Mode.
        CS_INTR_MASK OFFSET(31) NUMBITS(1) [],

        /// Interrupt enable mask bit for Frame End in Slave Mode.
        FRAME_END_INTR_MASK OFFSET(30) NUMBITS(1) [],

        /// Interrupt enable mask bit for `RDY`.
        RDY_INTR_MASK OFFSET(29) NUMBITS(1) [],

        /// Interrupt enable mask bit for `TX_FIFO_OVF`.
        TX_FIFO_OVF_INTR_MASK OFFSET(28) NUMBITS(1) [],

        /// Interrupt enable mask bit for `TX_FIFO_UNF`.
        TX_FIFO_UNF_INTR_MASK OFFSET(27) NUMBITS(1) [],

        /// Interrupt enable mask bit for `RX_FIFO_OVF`.
        RX_FIFO_OVF_INTR_MASK OFFSET(26) NUMBITS(1) [],

        /// Interrupt enable mask bit for `RX_FIFO_UNF`.
        RX_FIFO_UNF_INTR_MASK OFFSET(25) NUMBITS(1) []
    ],

    /// Bitfields of the `SPI_SPARE_CTLR` register.
    pub SPI_SPARE_CTLR [
        /// Reserved for future use.
        RESERVED0 OFFSET(11) NUMBITS(21) [],

        /// Can be used to adjust internal clock delay in Master Mode.
        SPI_SPARE_CONTROL_REGISTER_BYTE2 OFFSET(8) NUMBITS(3) [],

        /// Reserved for future use.
        RESERVED1 OFFSET(0) NUMBITS(8) []
    ]
}

register_structs! {
    /// Representation of the SPI registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => pub SPI_COMMAND_0: ReadWrite<u32, SPI_COMMAND_0::Register>),
        (0x04 => pub SPI_COMMAND2_0: ReadWrite<u32, SPI_COMMAND2_0::Register>),
        (0x08 => pub SPI_TIMING_REG1_0: ReadWrite<u32, SPI_TIMING_REG1_0::Register>),
        (0x0C => pub SPI_TIMING_REG2_0: ReadWrite<u32, SPI_TIMING_REG2_0::Register>),
        (0x10 => pub SPI_TRANSFER_STATUS_0: ReadWrite<u32, SPI_TRANSFER_STATUS_0::Register>),
        (0x14 => pub SPI_FIFO_STATUS_0: ReadWrite<u32, SPI_FIFO_STATUS_0::Register>),
        (0x18 => pub SPI_TX_DATA_0: ReadOnly<u32, SPI_TX_DATA_0::Register>),
        (0x1C => pub SPI_RX_DATA_0: ReadOnly<u32, SPI_RX_DATA_0::Register>),
        (0x20 => pub SPI_DMA_CTL_0: ReadWrite<u32, SPI_DMA_CTL_0::Register>),
        (0x24 => pub SPI_DMA_BLK_SIZE_0: ReadWrite<u32, SPI_DMA_BLK_SIZE_0::Register>),
        (0x28 => _reserved0: [ReadWrite<u8>; 0xE0]),
        (0x108 => pub SPI_TX_FIFO_0: ReadWrite<u32, SPI_TX_FIFO_0::Register>),
        (0x10C => _reserved1: [ReadWrite<u8>; 0x7C]),
        (0x188 => pub SPI_RX_FIFO_0: ReadWrite<u32, SPI_RX_FIFO_0::Register>),
        (0x18C => pub SPI_INTR_MASK_0: ReadWrite<u32, SPI_INTR_MASK_0::Register>),
        (0x190 => pub SPI_SPARE_CTLR: ReadWrite<u32, SPI_SPARE_CTLR::Register>),
        (0x194 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x194]);
