//! Abstractions over the registers of the Tegra X1 I2C controllers.
//!
//! See Chapter 35.6 in the Tegra X1 Technical Reference Manual
//! for details.

use register::{mmio::*, register_bitfields, register_structs};

use crate::memory_map::i2c::*;

/// A pointer to the I2C 1 register block that can be accessed by dereferencing it.
pub const I2C_1_REGISTERS: *const Registers = I2C_1 as *const Registers;
/// A pointer to the I2C 2 register block that can be accessed by dereferencing it.
pub const I2C_2_REGISTERS: *const Registers = I2C_2 as *const Registers;
/// A pointer to the I2C 3 register block that can be accessed by dereferencing it.
pub const I2C_3_REGISTERS: *const Registers = I2C_3 as *const Registers;
/// A pointer to the I2C 4 register block that can be accessed by dereferencing it.
pub const I2C_4_REGISTERS: *const Registers = I2C_4 as *const Registers;
/// A pointer to the I2C 5 register block that can be accessed by dereferencing it.
pub const I2C_5_REGISTERS: *const Registers = I2C_5 as *const Registers;
/// A pointer to the I2C 6 register block that can be accessed by dereferencing it.
pub const I2C_6_REGISTERS: *const Registers = I2C_6 as *const Registers;

register_bitfields! {
    u32,

    /// Bitfields of the `I2C_I2C_CNFG_0` register.
    pub I2C_I2C_CNFG_0 [
        /// Used to select single or multi master mode.
        MULTI_MASTER_MODE OFFSET(17) NUMBITS(1) [
            /// Single master mode.
            ///
            /// NOTE: No arbitration checks are done in this mode.
            SingleMaster = 0,
            /// Multi master mode.
            MultiMaster = 1
        ],

        /// If set, the master will put a `STOP` condition on the bus.
        MSTR_CLR_BUS_ON_TIMEOUT OFFSET(15) NUMBITS(1) [],

        /// Debounce period for SDA and SCL lines.
        DEBOUNCE_CNT OFFSET(12) NUMBITS(3) [
            /// No debounce.
            None = 0,
            /// Debounce period of 2T.
            TwoT = 1,
            /// Debounce period of 4T.
            FourT = 2,
            /// Debounce period of 6T.
            SixT = 3,
            /// Debounce period of 8T.
            EightT = 4,
            /// Debounce period of 10T.
            TenT = 5,
            /// Debounce period of 12T.
            TwelveT = 6,
            /// Debounce period of 14T.
            FourteenT = 7
        ],

        /// Whether the new Master FSM should be enabled.
        ///
        /// NOTE: This is maintained for backwards compatibility.
        NEW_MASTER_FSM OFFSET(11) NUMBITS(1) [],

        /// Whether transfer should be done in packet mode.
        PACKET_MODE_EN OFFSET(10) NUMBITS(1) [],

        /// Setting this bit makes the master initialize transaction in normal mode.
        ///
        /// NOTE: Firmware should program all other registers and the bits `[0:8]`
        /// before setting this bit.
        SEND OFFSET(9) NUMBITS(1) [],

        /// Whether an ACK at the end of the disable should be ignored.
        NOACK OFFSET(8) NUMBITS(1) [],

        /// Read/Write Command for Slave 2.
        CMD2 OFFSET(7) NUMBITS(1) [
            /// Write Transaction.
            Write = 0,
            /// Read Transaction.
            Read = 1
        ],

        /// Read/Write Command for Slave 1.
        CMD1 OFFSET(6) NUMBITS(1) [
            /// Write Transaction.
            Write = 0,
            /// Read Transaction.
            Read = 1
        ],

        /// Whether sending a Start byte is required.
        START OFFSET(5) NUMBITS(1) [],

        /// Whether two-slave transaction should be enabled.
        SLV2 OFFSET(4) NUMBITS(1) [],

        /// The number of bytes to be transmitted per transaction.
        ///
        /// 000 = 1 byte, 111 = 8 bytes, ...
        ///
        /// NOTE: In a two-slave transaction, the number of bytes
        /// should be programmed to be less than 011.
        LENGTH OFFSET(1) NUMBITS(3) [],

        /// The address mode, defining how the slave address should be programmed.
        ///
        /// Setting this bit means that a 10-bit slave address will be
        /// programmed, otherwise a 7-bit slave address.
        A_MOD OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_I2C_CMD_ADDR0_0` register.
    pub I2C_I2C_CMD_ADDR0_0 [
        /// Configures the slave 1 address and the transaction mode.
        ///
        /// For 7-bit mode, the address is written in `I2C_CMD_ADDR0[7:1]`, and `I2C_CMD_ADDR0[0]`
        /// indicates the read/write transaction. The `I2C_CMD_ADDR0[0]` bit must match the
        /// `I2C_CNFG[6]` bit.
        ///
        /// For 10-bit mode, the address is written in `I2C_CMD_ADDR0[9:0]`, and `I2C_CNFG[6]`
        /// indicates the read/write transaction.
        ADDR0 OFFSET(0) NUMBITS(10) []
    ],

    /// Bitfields of the `I2C_I2C_CMD_ADDR1_0` register.
    pub I2C_I2C_CMD_ADDR1_0 [
        /// Configures the slave 2 address and the transaction mode.
        ///
        /// For 7-bit mode, the address is written in `I2C_CMD_ADDR1[7:1]`, and `I2C_CMD_ADDR1[0]`
        /// indicates the read/write transaction. The `I2C_CMD_ADDR1[0]` bit must match the
        /// `I2C_CNFG[7]` bit.
        ///
        /// For 10-bit mode, the address is written in `I2C_CMD_ADDR1[9:0]`, and `I2C_CNFG[7]`
        /// indicates the read/write transaction.
        ADDR0 OFFSET(0) NUMBITS(10) []
    ],

    /// Bitfields of the `I2C_I2C_CMD_DATA1_0` register.
    pub I2C_I2C_CMD_DATA1_0 [
        /// Fourth data byte to send/receive.
        DATA4 OFFSET(24) NUMBITS(8) [],

        /// Third data byte to send/receive.
        DATA3 OFFSET(16) NUMBITS(8) [],

        /// Second data byte to send/receive.
        DATA2 OFFSET(8) NUMBITS(8) [],

        /// First data byte to send/receive.
        DATA1 OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `I2C_I2C_CMD_DATA2_0` register.
    pub I2C_I2C_CMD_DATA2_0 [
        /// Eighth data byte to send/receive.
        DATA8 OFFSET(24) NUMBITS(8) [],

        /// Seventh data byte to send/receive.
        DATA7 OFFSET(16) NUMBITS(8) [],

        /// Sixth data byte to send/receive.
        DATA6 OFFSET(8) NUMBITS(8) [],

        /// Fifth data byte to send/receive.
        DATA5 OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `I2C_I2C_STATUS_0` register.
    pub I2C_I2C_STATUS_0 [
        /// Whether the master is currently busy.
        BUSY OFFSET(8) NUMBITS(1) [],

        /// Slave 2 transaction status.
        CMD2_STAT OFFSET(4) NUMBITS(4) [
            /// Transaction succeeded.
            Success = 0,
            /// Byte 1 was not acknowledged.
            Byte1NoAck = 1,
            /// Byte 2 was not acknowledged.
            Byte2NoAck = 2,
            /// Byte 3 was not acknowledged.
            Byte3NoAck = 3,
            /// Byte 4 was not acknowledged.
            Byte4NoAck = 4,
            /// Byte 5 was not acknowledged.
            Byte5NoAck = 5,
            /// Byte 6 was not acknowledged.
            Byte6NoAck = 6,
            /// Byte 7 was not acknowledged.
            Byte7NoAck = 7,
            /// Byte 8 was not acknowledged.
            Byte8NoAck = 8,
            /// Byte 9 was not acknowledged.
            Byte9NoAck = 9,
            /// Byte 10 was not acknowledged.
            Byte10NoAck = 10
        ],

        /// Slave 1 transaction status.
        CMD1_STAT OFFSET(0) NUMBITS(4) [
            /// Transaction succeeded.
            Success = 0,
            /// Byte 1 was not acknowledged.
            Byte1NoAck = 1,
            /// Byte 2 was not acknowledged.
            Byte2NoAck = 2,
            /// Byte 3 was not acknowledged.
            Byte3NoAck = 3,
            /// Byte 4 was not acknowledged.
            Byte4NoAck = 4,
            /// Byte 5 was not acknowledged.
            Byte5NoAck = 5,
            /// Byte 6 was not acknowledged.
            Byte6NoAck = 6,
            /// Byte 7 was not acknowledged.
            Byte7NoAck = 7,
            /// Byte 8 was not acknowledged.
            Byte8NoAck = 8,
            /// Byte 9 was not acknowledged.
            Byte9NoAck = 9,
            /// Byte 10 was not acknowledged.
            Byte10NoAck = 10
        ]
    ],

    /// Bitfields of the `I2C_I2C_SL_CNFG_0` register.
    pub I2C_I2C_SL_CNFG_0 [
        /// Whether data should be communicated through the FIFOs.
        FIFO_XFER_EN OFFSET(20) NUMBITS(1) [],

        /// The payload size in bytes.
        BUFFER_SIZE OFFSET(8) NUMBITS(12) [],

        /// Whether the last byte should be acknowledged valid.
        ACK_LAST_BYTE_VALID OFFSET(7) NUMBITS(1) [],

        /// Whether the last byte should be acknowledged.
        ACK_LAST_BYTE OFFSET(6) NUMBITS(1) [],

        /// Whether the ACK Withhold feature should be enabled.
        ACK_WITHHOLD_EN OFFSET(5) NUMBITS(1) [],

        /// Whether Packet Mode should be enabled.
        PKT_MODE_EN OFFSET(4) NUMBITS(1) [],

        /// By writing zero to this field, the slave can be turned off.
        ENABLE_SL OFFSET(3) NUMBITS(1) [],

        /// Whether a new slave should be used.
        NEWSL OFFSET(2) NUMBITS(1) [],

        /// Whether Slave ACK should be disabled.
        NACK OFFSET(1) NUMBITS(1) [],

        /// Whether slave responses should be sent to the general call address (zero address).
        RESP OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_I2C_SL_RCVD_0` register.
    pub I2C_I2C_SL_RCVD_0 [
        /// The data received by the slave.
        SL_DATA OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `I2C_I2C_SL_STATUS_0` register.
    pub I2C_I2C_SL_STATUS_0 [
        /// Hardware master address received via general call address.
        ///
        /// This field is only meaningful if `HW_MSTR_ADR` is set.
        ///
        /// NOTE: These bits are read-only.
        HW_MSTR_ADR OFFSET(8) NUMBITS(7) [],

        /// Whether Hardware Master Access was received.
        HW_MSTR_INT OFFSET(7) NUMBITS(1) [],

        /// Whether the slave address should be reprogrammed.
        REPROG_SL OFFSET(6) NUMBITS(1) [],

        /// Whether the slave address should be reset and reprogrammed.
        RST_SL OFFSET(5) NUMBITS(1) [],

        /// Whether a transaction has completed.
        END_TRANS OFFSET(4) NUMBITS(1) [],

        /// Whether an interrupt has been generated by the slave.
        SL_IRQ OFFSET(3) NUMBITS(1) [],

        /// New Transaction Received status.
        RCVD OFFSET(2) NUMBITS(1) [
            /// No transaction occurred.
            NoTransactionOccurred = 0,
            /// Transaction occurred.
            TransactionOccurred = 1
        ],

        /// Slave Transaction status.
        ///
        /// NOTE: This bit is read-only.
        RNW OFFSET(1) NUMBITS(1) [
            /// Write Transaction.
            Write = 0,
            /// Read Transaction.
            Read = 1
        ],

        /// Zero Address status.
        ZA OFFSET(0) NUMBITS(1) [
            /// Slave did not respond.
            NoSlaveResponse = 0,
            /// Slave responded.
            SlaveResponse = 1
        ]
    ],

    /// Bitfields of the `I2C_I2C_SL_ADDR1_0` register.
    pub I2C_I2C_SL_ADDR1_0 [
        /// For a 10-bit slave address, this field is the least significant 8 bits.
        SL_ADDR1 OFFSET(8) NUMBITS(8) [],

        /// For a 10-bit slave address, this field is the most significant 8 bits.
        SL_ADDR0 OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `I2C_I2C_SL_ADDR2_0` register.
    pub I2C_I2C_SL_ADDR2_0 [
        /// Selection of the slave address.
        SELECT_SLAVE OFFSET(16) NUMBITS(1) [
            /// Use slave addr0.
            Addr0 = 0,
            /// Use slave addr1.
            Addr1 = 1
        ],

        /// In 10-bit address mode, these bits represent the 2 MSBs of the address.
        SL1_ADDR_HI OFFSET(9) NUMBITS(2) [],

        SL1_VLD OFFSET(8) NUMBITS(1) [
            /// 7-bit addressing.
            SevenBitAddrMode = 0,
            /// 10-bit addressing.
            TenBitAddrMode = 1
        ],

        /// In 10-bit address mode, these bits represent the 2 MSBs of the address.
        SL_ADDR_HI OFFSET(1) NUMBITS(2) [],

        VLD OFFSET(0) NUMBITS(1) [
            /// 7-bit addressing.
            SevenBitAddrMode = 0,
            /// 10-bit addressing.
            TenBitAddrMode = 1
        ]
    ],

    /// Bitfields of the `I2C_I2C_TLOW_SEXT_0` register.
    pub I2C_I2C_TLOW_SEXT_0 [
        /// Reset slave state machine on timeout.
        RST_SL_ON_TIMEOUT OFFSET(27) NUMBITS(1) [],

        /// Enable `TLOW_MEXT` counter.
        TLOW_MEXT_EN OFFSET(26) NUMBITS(1) [],

        /// Enable `TLOW_SEXT` counter.
        TLOW_SEXT_EN OFFSET(25) NUMBITS(1) [],

        /// Enable `TIMEOUT` counter.
        TIMOUT_EN OFFSET(24) NUMBITS(1) [],

        /// Transfer interval of a master device in milliseconds.
        TLOW_MEXT OFFSET(16) NUMBITS(8) [],

        /// Transfer interval of a slave device in milliseconds.
        TLOW_SEXT OFFSET(8) NUMBITS(8) [],

        /// Clock low timeout period in milliseconds.
        TIMEOUT OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `I2C_I2C_SL_DELAY_COUNT_0` register.
    pub I2C_I2C_SL_DELAY_COUNT_0 [
        /// The value determines the timing of various cycles on the bus.
        SL_DELAY_COUNT OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `I2C_I2C_SL_INT_MASK_0` register.
    pub I2C_I2C_SL_INT_MASK_0 [
        HW_MSTR_INT OFFSET(7) NUMBITS(1) [],

        REPROG_SL OFFSET(6) NUMBITS(1) [],

        RST_SL OFFSET(5) NUMBITS(1) [],

        END_TRANS OFFSET(4) NUMBITS(1) [],

        SL_IRQ OFFSET(3) NUMBITS(1) [],

        RCVD OFFSET(2) NUMBITS(1) [],

        ZA OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_I2C_SL_INT_SOURCE_0` register.
    pub I2C_I2C_SL_INT_SOURCE_0 [
        HW_MSTR_INT OFFSET(7) NUMBITS(1) [],

        REPROG_SL OFFSET(6) NUMBITS(1) [],

        RST_SL OFFSET(5) NUMBITS(1) [],

        END_TRANS OFFSET(4) NUMBITS(1) [],

        SL_IRQ OFFSET(3) NUMBITS(1) [],

        RCVD OFFSET(2) NUMBITS(1) [],

        ZA OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_I2C_SL_INT_SET_0` register.
    pub I2C_I2C_SL_INT_SET_0 [
        HW_MSTR_INT OFFSET(7) NUMBITS(1) [],

        REPROG_SL OFFSET(6) NUMBITS(1) [],

        RST_SL OFFSET(5) NUMBITS(1) [],

        END_TRANS OFFSET(4) NUMBITS(1) [],

        SL_IRQ OFFSET(3) NUMBITS(1) [],

        RCVD OFFSET(2) NUMBITS(1) [],

        ZA OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_I2C_TX_PACKET_FIFO_0` register.
    pub I2C_I2C_TX_PACKET_FIFO_0 [
        /// Software writes packets into this register.
        ///
        /// A packet may contain generic information.
        TX_PACKET OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `I2C_I2C_RX_FIFO_0` register.
    pub I2C_I2C_RX_FIFO_0 [
        /// Software reads data from this register, causes a pop.
        RD_DATA OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `I2C_PACKET_TRANSFER_STATUS_0` register.
    pub I2C_PACKET_TRANSFER_STATUS_0 [
        /// The packet transfer for which the last packet is set has been completed.
        TRANSFER_COMPLETE OFFSET(24) NUMBITS(1) [],

        /// The current packet ID for which the transaction is happening on the bus.
        TRANSFER_PKT_ID OFFSET(16) NUMBITS(8) [],

        /// The number of bytes transferred in the current packet.
        TRANSFER_BYTENUM OFFSET(4) NUMBITS(12) [],

        /// No ACK received for the address byte.
        NOACK_FOR_ADDR OFFSET(3) NUMBITS(1) [],

        /// No ACK received for the data byte.
        NOACK_FOR_DATA OFFSET(2) NUMBITS(1) [],

        /// Arbitration lost for the current byte.
        ARB_LOST OFFSET(1) NUMBITS(1) [],

        /// Controller is busy.
        CONTROLLER_BUSY OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_FIFO_CONTROL_0` register.
    pub I2C_FIFO_CONTROL_0 [
        /// Slave Transmit FIFO trigger level.
        SLV_TX_FIFO_TRIG OFFSET(13) NUMBITS(3) [
            /// Asserted when at least one word is empty in the FIFO.
            OneWord = 000,
            /// Asserted when at least 2 words are empty in the FIFO.
            TwoWords = 001
        ],

        /// Slave Receive FIFO trigger level.
        SLV_RX_FIFO_TRIG OFFSET(10) NUMBITS(3) [
            /// Asserted when at least one word is full in the FIFO.
            OneWord = 000,
            /// Asserted when at least 2 words are full in the FIFO.
            TwoWords = 001
        ],

        /// Flush the TX FIFO.
        ///
        /// Cleared after the FIFO is flushed.
        SLV_TX_FIFO_FLUSH OFFSET(9) NUMBITS(1) [],

        /// Flush the RX FIFO.
        ///
        /// Cleared after the FIFO is flushed.
        SLV_RX_FIFO_FLUSH OFFSET(8) NUMBITS(1) [],

        /// Transmit FIFO trigger level.
        TX_FIFO_TRIG OFFSET(5) NUMBITS(3) [
            /// Asserted when at least one word is empty in the FIFO.
            OneWord = 000,
            /// Asserted when at least 2 words are empty in the FIFO.
            TwoWords = 001
        ],

        /// Receive FIFO trigger level.
        RX_FIFO_TRIG OFFSET(2) NUMBITS(3) [
            /// Asserted when at least one word is full in the FIFO.
            OneWord = 000,
            /// Asserted when at least 2 words are full in the FIFO.
            TwoWords = 001
        ],

        /// Flush the TX FIFO.
        ///
        /// Cleared after the FIFO is flushed.
        TX_FIFO_FLUSH OFFSET(1) NUMBITS(1) [],

        /// Flush the RX FIFO.
        ///
        /// Cleared after the FIFO is flushed.
        RX_FIFO_FLUSH OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_FIFO_STATUS_0` register.
    pub I2C_FIFO_STATUS_0 [
        /// Whether the transaction was terminated by the master or slave.
        ///
        /// It is only meaningful if `PKT_XFER_ERR` is set.
        SLV_XFER_ERR_REASON OFFSET(25) NUMBITS(1) [],

        /// The number of slots that can be written to the TX FIFO.
        SLV_TX_FIFO_EMPTY_CNT OFFSET(20) NUMBITS(4) [
            /// The FIFO is full.
            FifoFull = 0000,
            /// One FIFO slot is empty.
            OneSlotEmpty = 0001,
            /// Two FIFO slots are empty.
            TwoSlotsEmpty = 0002
        ],

        /// The number of slots to be read from the RX FIFO.
        SLV_RX_FIFO_FULL_CNT OFFSET(16) NUMBITS(4) [
            /// The FIFO is empty.
            FifoEmpty = 0000,
            /// One FIFO slot is full.
            OneSlotFull = 0001,
            /// Two FIFO slots are full.
            TwoSlotsFull = 0002
        ],

        /// The number of slots that can be written to the TX FIFO.
        TX_FIFO_EMPTY_CNT OFFSET(4) NUMBITS(4) [
            /// The FIFO is full.
            FifoFull = 0000,
            /// One FIFO slot is empty.
            OneSlotEmpty = 0001,
            /// Two FIFO slots are empty.
            TwoSlotsEmpty = 0002
        ],

        /// The number of slots to be read from the RX FIFO.
        RX_FIFO_FULL_CNT OFFSET(0) NUMBITS(4) [
            /// The FIFO is empty.
            FifoEmpty = 0000,
            /// One FIFO slot is full.
            OneSlotFull = 0001,
            /// Two FIFO slots are full.
            TwoSlotsFull = 0002
        ]
    ],

    /// Bitfields of the `I2C_INTERRUPT_MASK_REGISTER_0` register.
    pub I2C_INTERRUPT_MASK_REGISTER_0 [
        SLV_ACK_WITHHELD_INT_EN OFFSET(28) NUMBITS(1) [],

        SLV_RD2WR_INT_EN OFFSET(27) NUMBITS(1) [],

        SLV_WR2RD_INT_EN OFFSET(26) NUMBITS(1) [],

        SLV_PKT_XFER_ERR_INT_EN OFFSET(25) NUMBITS(1) [],

        SLV_TX_BUFFER_REQ_INT_EN OFFSET(24) NUMBITS(1) [],

        SLV_RX_BUFFER_FILLED_INT_EN OFFSET(23) NUMBITS(1) [],

        SLV_PACKET_XFER_COMPLETE_INT_EN OFFSET(22) NUMBITS(1) [],

        SLV_TFIFO_OVF_REQ_INT_EN OFFSET(21) NUMBITS(1) [],

        SLV_RFIFO_UNF_REQ_INT_EN OFFSET(20) NUMBITS(1) [],

        SLV_TFIFO_DATA_REQ_INT_EN OFFSET(17) NUMBITS(1) [],

        SLV_RFIFO_DATA_REQ_INT_EN OFFSET(16) NUMBITS(1) [],

        BUS_CLEAR_DONE_INT_EN OFFSET(11) NUMBITS(1) [],

        TLOW_MEXT_TIMEOUT_EN OFFSET(10) NUMBITS(1) [],

        TLOW_SEXT_TIMEOUT_EN OFFSET(9) NUMBITS(1) [],

        TIMEOUT_INT_EN OFFSET(8) NUMBITS(1) [],

        PACKET_XFER_COMPLETE_INT_EN OFFSET(7) NUMBITS(1) [],

        ALL_PACKETS_XFER_COMPLETE_INT_EN OFFSET(6) NUMBITS(1) [],

        TFIFO_OVF_INT_EN OFFSET(5) NUMBITS(1) [],

        RFIFO_UNF_INT_EN OFFSET(4) NUMBITS(1) [],

        NOACK_INT_EN OFFSET(3) NUMBITS(1) [],

        ARB_LOST_INT_EN OFFSET(2) NUMBITS(1) [],

        TFIFO_DATA_REQ_INT_EN OFFSET(1) NUMBITS(1) [],

        RFIFO_DATA_REQ_INT_EN OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_INTERRUPT_STATUS_REGISTER_0` register.
    pub I2C_INTERRUPT_STATUS_REGISTER_0 [
        SLV_ACK_WITHHELD OFFSET(28) NUMBITS(1) [],

        SLV_RD2WR OFFSET(27) NUMBITS(1) [],

        SLV_WR2RD OFFSET(26) NUMBITS(1) [],

        SLV_PKT_XFER_ERR OFFSET(25) NUMBITS(1) [],

        SLV_TX_BUFFER_REQ OFFSET(24) NUMBITS(1) [],

        SLV_RX_BUFFER_FILLED OFFSET(23) NUMBITS(1) [],

        SLV_PACKET_XFER_COMPLETE OFFSET(22) NUMBITS(1) [],

        SLV_TFIFO_OVF_REQ OFFSET(21) NUMBITS(1) [],

        SLV_RFIFO_UNF_REQ OFFSET(20) NUMBITS(1) [],

        SLV_TFIFO_DATA_REQ OFFSET(17) NUMBITS(1) [],

        SLV_RFIFO_DATA_REQ OFFSET(16) NUMBITS(1) [],

        BUS_CLEAR_DONE OFFSET(11) NUMBITS(1) [],

        TLOW_MEXT_TIMEOUT OFFSET(10) NUMBITS(1) [],

        TLOW_SEXT_TIMEOUT OFFSET(9) NUMBITS(1) [],

        TIMEOUT OFFSET(8) NUMBITS(1) [],

        PACKET_XFER_COMPLETE OFFSET(7) NUMBITS(1) [],

        ALL_PACKETS_XFER_COMPLETE OFFSET(6) NUMBITS(1) [],

        TFIFO_OVF OFFSET(5) NUMBITS(1) [],

        RFIFO_UNF OFFSET(4) NUMBITS(1) [],

        NOACK OFFSET(3) NUMBITS(1) [],

        ARB_LOST OFFSET(2) NUMBITS(1) [],

        TFIFO_DATA_REQ OFFSET(1) NUMBITS(1) [],

        RFIFO_DATA_REQ OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_I2C_CLK_DIVISOR_REGISTER_0` register.
    pub I2C_I2C_CLK_DIVISOR_REGISTER_0 [
        I2C_CLK_DIVISOR_STD_FAST_MODE OFFSET(16) NUMBITS(16) [],

        I2C_CLK_DIVISOR_HSMODE OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `I2C_I2C_INTERRUPT_SOURCE_REGISTER_0` register.
    pub I2C_I2C_INTERRUPT_SOURCE_REGISTER_0 [
        SLV_ACK_WITHHELD OFFSET(28) NUMBITS(1) [],

        SLV_RD2WR OFFSET(27) NUMBITS(1) [],

        SLV_WR2RD OFFSET(26) NUMBITS(1) [],

        SLV_PKT_XFER_ERR OFFSET(25) NUMBITS(1) [],

        SLV_TX_BUFFER_REQ OFFSET(24) NUMBITS(1) [],

        SLV_RX_BUFFER_FILLED OFFSET(23) NUMBITS(1) [],

        SLV_PACKET_XFER_COMPLETE OFFSET(22) NUMBITS(1) [],

        SLV_TFIFO_OVF_REQ OFFSET(21) NUMBITS(1) [],

        SLV_RFIFO_UNF_REQ OFFSET(20) NUMBITS(1) [],

        SLV_TFIFO_DATA_REQ OFFSET(17) NUMBITS(1) [],

        SLV_RFIFO_DATA_REQ OFFSET(16) NUMBITS(1) [],

        BUS_CLEAR_DONE OFFSET(11) NUMBITS(1) [],

        TLOW_MEXT_TIMEOUT OFFSET(10) NUMBITS(1) [],

        TLOW_SEXT_TIMEOUT OFFSET(9) NUMBITS(1) [],

        TIMEOUT OFFSET(8) NUMBITS(1) [],

        PACKET_XFER_COMPLETE OFFSET(7) NUMBITS(1) [],

        ALL_PACKETS_XFER_COMPLETE OFFSET(6) NUMBITS(1) [],

        TFIFO_OVF OFFSET(5) NUMBITS(1) [],

        RFIFO_UNF OFFSET(4) NUMBITS(1) [],

        NOACK OFFSET(3) NUMBITS(1) [],

        ARB_LOST OFFSET(2) NUMBITS(1) [],

        TFIFO_DATA_REQ OFFSET(1) NUMBITS(1) [],

        RFIFO_DATA_REQ OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_I2C_INTERRUPT_SET_REGISTER_0` register.
    pub I2C_I2C_INTERRUPT_SET_REGISTER_0 [
        SLV_ACK_WITHHELD OFFSET(28) NUMBITS(1) [],

        SLV_RD2WR OFFSET(27) NUMBITS(1) [],

        SLV_WR2RD OFFSET(26) NUMBITS(1) [],

        SLV_PKT_XFER_ERR OFFSET(25) NUMBITS(1) [],

        SLV_TX_BUFFER_REQ OFFSET(24) NUMBITS(1) [],

        SLV_RX_BUFFER_FILLED OFFSET(23) NUMBITS(1) [],

        SLV_PACKET_XFER_COMPLETE OFFSET(22) NUMBITS(1) [],

        SLV_TFIFO_OVF OFFSET(21) NUMBITS(1) [],

        SLV_RFIFO_UNF OFFSET(20) NUMBITS(1) [],

        BUS_CLEAR_DONE OFFSET(11) NUMBITS(1) [],

        TLOW_MEXT_TIMEOUT OFFSET(10) NUMBITS(1) [],

        TLOW_SEXT_TIMEOUT OFFSET(9) NUMBITS(1) [],

        TIMEOUT OFFSET(8) NUMBITS(1) [],

        PACKET_XFER_COMPLETE OFFSET(7) NUMBITS(1) [],

        ALL_PACKETS_XFER_COMPLETE OFFSET(6) NUMBITS(1) [],

        TFIFO_OVF OFFSET(5) NUMBITS(1) [],

        RFIFO_UNF OFFSET(4) NUMBITS(1) [],

        NOACK OFFSET(3) NUMBITS(1) [],

        ARB_LOST OFFSET(2) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_I2C_SLV_TX_PACKET_FIFO_0` register.
    pub I2C_I2C_SLV_TX_PACKET_FIFO_0 [
        /// Software writes packets into this register.
        ///
        /// A packet may contain generic information.
        TX_PACKET OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `I2C_I2C_SLV_RX_FIFO_0` register.
    pub I2C_I2C_SLV_RX_FIFO_0 [
        /// Software reads data from this register, causes a pop.
        RD_DATA OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `I2C_I2C_SLV_PACKET_STATUS_0` register.
    pub I2C_I2C_SLV_PACKET_STATUS_0 [
        /// Indicates that an ACK is withheld for the last byte and the slave is waiting
        /// for the host to explicitly command the slave to acknowledge the last byte.
        ACK_WITHHELD OFFSET(25) NUMBITS(1) [],

        /// All the packets have been transferred successfully.
        TRANSFER_COMPLETE OFFSET(24) NUMBITS(1) [],

        /// The current packet ID for which the transaction is happening on the bus.
        TRANSFER_PKT_ID OFFSET(16) NUMBITS(8) [],

        /// The number of bytes transferred in the current packet.
        TRANSFER_BYTENUM OFFSET(4) NUMBITS(12) []
    ],

    /// Bitfields of the `I2C_I2C_BUS_CLEAR_CONFIG_0` register.
    pub I2C_I2C_BUS_CLEAR_CONFIG_0 [
        /// Clock pulses should be sent until this threshold is met.
        BC_SCLK_THRESHOLD OFFSET(16) NUMBITS(8) [],

        /// Whether a stop condition at the end of the bus clear operation should be sent.
        BC_STOP_COND OFFSET(2) NUMBITS(1) [],

        BC_TERMINATE OFFSET(1) NUMBITS(1) [
            /// Irrespective of SDA release status during bus clear, terminate the
            /// bus clear only after the threshold is reached.
            Threshold = 0,
            /// Terminate the bus clear operation immediately when SDA is released
            /// or threshold count is reached, whichever is earlier.
            Immediate = 1
        ],

        /// Starts bus clear operation. This bit gets cleared upon bus clear transaction completion.
        BC_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_I2C_BUS_CLEAR_STATUS_0` register.
    pub I2C_I2C_BUS_CLEAR_STATUS_0 [
        /// Whether SDA is released by the slave.
        BC_STATUS OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_I2C_CONFIG_LOAD_0` register.
    pub I2C_I2C_CONFIG_LOAD_0 [
        /// Loads the timeout configuration from the `pclk` domain to the `i2c_slow_clk` domain.
        TIMEOUT_CONFIG_LOAD OFFSET(2) NUMBITS(1) [],

        /// Loads the slave configuration from the `pclk` domain to the `i2c_clk` domain.
        SLV_CONFIG_LOAD OFFSET(1) NUMBITS(1) [],

        /// Loads the master configuration from the `pclk` domain to the `i2c_clk` domain.
        MSTR_CONFIG_LOAD OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `I2C_I2C_INTERFACE_TIMING_0_0` register.
    pub I2C_I2C_INTERFACE_TIMING_0_0 [
        /// High period of the SCL clock.
        THIGH OFFSET(8) NUMBITS(6) [],

        /// Low period of the SCL clock.
        TLOW OFFSET(0) NUMBITS(6) []
    ],

    /// Bitfields of the `I2C_I2C_INTERFACE_TIMING_1_0` register.
    pub I2C_I2C_INTERFACE_TIMING_1_0 [
        /// Bus free time between STOP and START conditions.
        TBUF OFFSET(24) NUMBITS(6) [],

        /// Setup time for STOP condition.
        TSU_STO OFFSET(16) NUMBITS(6) [],

        /// Hold time for a (Repeated) START condition.
        THD_STA OFFSET(8) NUMBITS(6) [],

        /// Setup time for a Repeated START condition.
        TSU_STA OFFSET(0) NUMBITS(6) []
    ],

    /// Bitfields of the `I2C_I2C_HS_INTERFACE_TIMING_0_0` register.
    pub I2C_I2C_HS_INTERFACE_TIMING_0_0 [
        /// High period of the SCL clock.
        HS_THIGH OFFSET(8) NUMBITS(6) [],

        /// Low period of the SCL clock.
        HS_TLOW OFFSET(0) NUMBITS(6) []
    ],

    /// Bitfields of the `I2C_I2C_HS_INTERFACE_TIMING_1_0` register.
    pub I2C_I2C_HS_INTERFACE_TIMING_1_0 [
        /// Setup time for STOP condition.
        HS_TSU_STO OFFSET(16) NUMBITS(6) [],

        /// Hold time for a (Repeated) START condition.
        HS_THD_STA OFFSET(8) NUMBITS(6) [],

        /// Setup time for a Repeated START condition.
        HS_TSU_STA OFFSET(0) NUMBITS(6) []
    ]
}

register_structs! {
    /// Representation of the I2C registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => pub I2C_I2C_CNFG_0: ReadWrite<u32, I2C_I2C_CNFG_0::Register>),
        (0x04 => pub I2C_I2C_CMD_ADDR0_0: ReadWrite<u32, I2C_I2C_CMD_ADDR0_0::Register>),
        (0x08 => pub I2C_I2C_CMD_ADDR1_0: ReadWrite<u32, I2C_I2C_CMD_ADDR1_0::Register>),
        (0x0C => pub I2C_I2C_CMD_DATA1_0: ReadWrite<u32, I2C_I2C_CMD_DATA1_0::Register>),
        (0x10 => pub I2C_I2C_CMD_DATA2_0: ReadWrite<u32, I2C_I2C_CMD_DATA2_0::Register>),
        (0x14 => reserved0: [ReadWrite<u32>; 0x2]),
        (0x1C => pub I2C_I2C_STATUS_0: ReadOnly<u32, I2C_I2C_STATUS_0::Register>),
        (0x20 => pub I2C_I2C_SL_CNFG_0: ReadWrite<u32, I2C_I2C_SL_CNFG_0::Register>),
        (0x24 => pub I2C_I2C_SL_RCVD_0: ReadWrite<u32, I2C_I2C_SL_RCVD_0::Register>),
        (0x28 => pub I2C_I2C_SL_STATUS_0: ReadWrite<u32, I2C_I2C_SL_STATUS_0::Register>),
        (0x2C => pub I2C_I2C_SL_ADDR1_0: ReadWrite<u32, I2C_I2C_SL_ADDR1_0::Register>),
        (0x30 => pub I2C_I2C_SL_ADDR2_0: ReadWrite<u32, I2C_I2C_SL_ADDR2_0::Register>),
        (0x34 => pub I2C_I2C_TLOW_SEXT_0: ReadWrite<u32, I2C_I2C_TLOW_SEXT_0::Register>),
        (0x38 => reserved1: ReadWrite<u32>),
        (0x3C => pub I2C_I2C_SL_DELAY_COUNT_0: ReadWrite<u32, I2C_I2C_SL_DELAY_COUNT_0::Register>),
        (0x40 => pub I2C_I2C_SL_INT_MASK_0: ReadWrite<u32, I2C_I2C_SL_INT_MASK_0::Register>),
        (0x44 => pub I2C_I2C_SL_INT_SOURCE_0: ReadOnly<u32, I2C_I2C_SL_INT_SOURCE_0::Register>),
        (0x48 => pub I2C_I2C_SL_INT_SET_0: ReadWrite<u32, I2C_I2C_SL_INT_SET_0::Register>),
        (0x4C => reserved2: ReadWrite<u32>),
        (0x50 => pub I2C_I2C_TX_PACKET_FIFO_0: ReadWrite<u32, I2C_I2C_TX_PACKET_FIFO_0::Register>),
        (0x54 => pub I2C_I2C_RX_FIFO_0: ReadOnly<u32, I2C_I2C_RX_FIFO_0::Register>),
        (0x58 => pub I2C_PACKET_TRANSFER_STATUS_0: ReadOnly<u32, I2C_PACKET_TRANSFER_STATUS_0::Register>),
        (0x5C => pub I2C_FIFO_CONTROL_0: ReadWrite<u32, I2C_FIFO_CONTROL_0::Register>),
        (0x60 => pub I2C_FIFO_STATUS_0: ReadOnly<u32, I2C_FIFO_STATUS_0::Register>),
        (0x64 => pub I2C_INTERRUPT_MASK_REGISTER_0: ReadWrite<u32, I2C_INTERRUPT_MASK_REGISTER_0::Register>),
        (0x68 => pub I2C_INTERRUPT_STATUS_REGISTER_0: ReadWrite<u32, I2C_INTERRUPT_STATUS_REGISTER_0::Register>),
        (0x6C => pub I2C_I2C_CLK_DIVISOR_REGISTER_0: ReadWrite<u32, I2C_I2C_CLK_DIVISOR_REGISTER_0::Register>),
        (0x70 => pub I2C_I2C_INTERRUPT_SOURCE_REGISTER_0: ReadOnly<u32, I2C_I2C_INTERRUPT_SOURCE_REGISTER_0::Register>),
        (0x74 => pub I2C_I2C_INTERRUPT_SET_REGISTER_0: ReadWrite<u32, I2C_I2C_INTERRUPT_SET_REGISTER_0::Register>),
        (0x78 => pub I2C_I2C_SLV_TX_PACKET_FIFO_0: ReadWrite<u32, I2C_I2C_SLV_TX_PACKET_FIFO_0::Register>),
        (0x7C => pub I2C_I2C_SLV_RX_FIFO_0: ReadOnly<u32, I2C_I2C_SLV_RX_FIFO_0::Register>),
        (0x80 => pub I2C_I2C_SLV_PACKET_STATUS_0: ReadOnly<u32, I2C_I2C_SLV_PACKET_STATUS_0::Register>),
        (0x84 => pub I2C_I2C_BUS_CLEAR_CONFIG_0: ReadWrite<u32, I2C_I2C_BUS_CLEAR_CONFIG_0::Register>),
        (0x88 => pub I2C_I2C_BUS_CLEAR_STATUS_0: ReadOnly<u32, I2C_I2C_BUS_CLEAR_STATUS_0::Register>),
        (0x8C => pub I2C_I2C_CONFIG_LOAD_0: ReadWrite<u32, I2C_I2C_CONFIG_LOAD_0::Register>),
        (0x90 => reserved3: ReadWrite<u32>),
        (0x94 => pub I2C_I2C_INTERFACE_TIMING_0_0: ReadWrite<u32, I2C_I2C_INTERFACE_TIMING_0_0::Register>),
        (0x98 => pub I2C_I2C_INTERFACE_TIMING_1_0: ReadWrite<u32, I2C_I2C_INTERFACE_TIMING_1_0::Register>),
        (0x9C => pub I2C_I2C_HS_INTERFACE_TIMING_0_0: ReadWrite<u32, I2C_I2C_HS_INTERFACE_TIMING_0_0::Register>),
        (0xA0 => pub I2C_I2C_HS_INTERFACE_TIMING_1_0: ReadWrite<u32, I2C_I2C_HS_INTERFACE_TIMING_1_0::Register>),
        (0xA4 => @END),
    }
}
