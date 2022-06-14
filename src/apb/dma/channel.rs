use tock_registers::{register_bitfields, register_structs, registers::*};

use crate::memory_map::apb_dma::*;

/// A pointer to the APB DMA Channel 0 register block that can be accessed by dereferencing it.
pub const CHANNEL_0: *const ChannelRegisters = CH0 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 1 register block that can be accessed by dereferencing it.
pub const CHANNEL_1: *const ChannelRegisters = CH1 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 2 register block that can be accessed by dereferencing it.
pub const CHANNEL_2: *const ChannelRegisters = CH2 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 3 register block that can be accessed by dereferencing it.
pub const CHANNEL_3: *const ChannelRegisters = CH3 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 4 register block that can be accessed by dereferencing it.
pub const CHANNEL_4: *const ChannelRegisters = CH4 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 5 register block that can be accessed by dereferencing it.
pub const CHANNEL_5: *const ChannelRegisters = CH5 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 6 register block that can be accessed by dereferencing it.
pub const CHANNEL_6: *const ChannelRegisters = CH6 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 7 register block that can be accessed by dereferencing it.
pub const CHANNEL_7: *const ChannelRegisters = CH7 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 8 register block that can be accessed by dereferencing it.
pub const CHANNEL_8: *const ChannelRegisters = CH8 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 9 register block that can be accessed by dereferencing it.
pub const CHANNEL_9: *const ChannelRegisters = CH9 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 10 register block that can be accessed by dereferencing it.
pub const CHANNEL_10: *const ChannelRegisters = CH10 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 11 register block that can be accessed by dereferencing it.
pub const CHANNEL_11: *const ChannelRegisters = CH11 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 12 register block that can be accessed by dereferencing it.
pub const CHANNEL_12: *const ChannelRegisters = CH12 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 13 register block that can be accessed by dereferencing it.
pub const CHANNEL_13: *const ChannelRegisters = CH13 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 14 register block that can be accessed by dereferencing it.
pub const CHANNEL_14: *const ChannelRegisters = CH14 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 15 register block that can be accessed by dereferencing it.
pub const CHANNEL_15: *const ChannelRegisters = CH15 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 16 register block that can be accessed by dereferencing it.
pub const CHANNEL_16: *const ChannelRegisters = CH16 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 17 register block that can be accessed by dereferencing it.
pub const CHANNEL_17: *const ChannelRegisters = CH17 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 18 register block that can be accessed by dereferencing it.
pub const CHANNEL_18: *const ChannelRegisters = CH18 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 19 register block that can be accessed by dereferencing it.
pub const CHANNEL_19: *const ChannelRegisters = CH19 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 20 register block that can be accessed by dereferencing it.
pub const CHANNEL_20: *const ChannelRegisters = CH20 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 21 register block that can be accessed by dereferencing it.
pub const CHANNEL_21: *const ChannelRegisters = CH21 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 22 register block that can be accessed by dereferencing it.
pub const CHANNEL_22: *const ChannelRegisters = CH22 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 23 register block that can be accessed by dereferencing it.
pub const CHANNEL_23: *const ChannelRegisters = CH23 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 24 register block that can be accessed by dereferencing it.
pub const CHANNEL_24: *const ChannelRegisters = CH24 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 25 register block that can be accessed by dereferencing it.
pub const CHANNEL_25: *const ChannelRegisters = CH25 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 26 register block that can be accessed by dereferencing it.
pub const CHANNEL_26: *const ChannelRegisters = CH26 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 27 register block that can be accessed by dereferencing it.
pub const CHANNEL_27: *const ChannelRegisters = CH27 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 28 register block that can be accessed by dereferencing it.
pub const CHANNEL_28: *const ChannelRegisters = CH28 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 29 register block that can be accessed by dereferencing it.
pub const CHANNEL_29: *const ChannelRegisters = CH29 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 30 register block that can be accessed by dereferencing it.
pub const CHANNEL_30: *const ChannelRegisters = CH30 as *const ChannelRegisters;
/// A pointer to the APB DMA Channel 31 register block that can be accessed by dereferencing it.
pub const CHANNEL_31: *const ChannelRegisters = CH31 as *const ChannelRegisters;

register_bitfields! {
    u32,

    /// Bitfields of the `APBDMACHAN_CHANNEL_CSR_0` register.
    pub APBDMACHAN_CHANNEL_CSR_0 [
        /// Enables DMA channel transfer.
        ENB OFFSET(31) NUMBITS(1) [],

        /// Generates interrupts when DMA block transfer completes.
        IE_EOC OFFSET(30) NUMBITS(1) [],

        /// Holds this Processor until DMA block transfer completes.
        HOLD OFFSET(29) NUMBITS(1) [],

        /// DMA Transfer Direction.
        DIR OFFSET(28) NUMBITS(1) [
            AhbWrite = 0,
            AhbRead = 1
        ],

        /// Run Once or Run Multiple Mode (Allow Retriggering of this Channel).
        ONCE OFFSET(27) NUMBITS(1) [
            MultipleBlock = 0,
            SingleBlock = 1
        ],

        /// Whether Flow Control should be enabled.
        FLOW OFFSET(21) NUMBITS(1) [],

        REQ_SEL OFFSET(16) NUMBITS(5) [
            CntrReq = 0,
            ApbifCh0 = 1,
            ApbifCh1 = 2,
            ApbbifCh2 = 3,
            ApbbifCh3 = 4,
            Qspi = 5,
            ApbifCh4 = 6,
            ApbifCh5 = 7,
            UartA = 8,
            UartB = 9,
            UartC = 10,
            Dtv = 11,
            ApbifCh6 = 12,
            ApbifCh7 = 13,
            AbifCh8 = 14,
            Sl2b1 = 15,
            Sl2b2 = 16,
            Sl2b3 = 17,
            Sl2b4 = 18,
            UartD = 19,
            I2c = 21,
            I2c2 = 22,
            I2c3 = 23,
            DvcI2c = 24,
            I2c4 = 26,
            Sl2b5 = 27,
            Sl2b6 = 28,
            ApbifCh9 = 29,
            I2c6 = 30
        ]
    ],

    /// Bitfields of the `APBDMACHAN_CHANNEL_STA_0` register.
    pub APBDMACHAN_CHANNEL_STA_0 [
        /// Indicates whether DMA Channel Status is active.
        BSY OFFSET(31) NUMBITS(1) [
            Wait = 0,
            Active = 1
        ],

        /// Write `1` to clear the flag.
        ISE_EOC OFFSET(30) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// Holding Status of Processor.
        HALT OFFSET(29) NUMBITS(1) [
            NoHalt = 0,
            Halt = 1
        ],

        /// Whether Ping or Pong buffer transfer has completed.
        PING_PONG_STA OFFSET(28) NUMBITS(1) [
            PingIntrSta = 0,
            PongIntrSta = 1
        ],

        /// Indicates whether the current DMA channel is transferring data.
        DMA_ACTIVITY OFFSET(27) NUMBITS(1) [
            Idle = 0,
            Busy = 1
        ],

        /// Indicates the status of channel pause.
        CHANNEL_PAUSE OFFSET(26) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDMACHAN_CHANNEL_DMA_BYTE_STA_0` register.
    pub APBDMACHAN_CHANNEL_DMA_BYTE_STA_0 [
        /// Indicates the actual DMA Data Transfer Count in bytes.
        DMA_COUNT OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `APBDMACHAN_CHANNEL_CSRE_0` register.
    pub APBDMACHAN_CHANNEL_CSRE_0 [
        /// When enabled, pauses data transfers on the channel.
        CHANNEL_PAUSE OFFSET(31) NUMBITS(1) [
            Resume = 0,
            Pause = 1
        ],

        /// Enable on Non-Zero Value.
        TRIG_SEL OFFSET(14) NUMBITS(5) [
            Na1 = 0,
            Smp24 = 1,
            Smp25 = 2,
            Smp26 = 3,
            Smp27 = 4,
            XrqA = 5,
            XrqB = 6,
            Tmr1 = 7,
            Tmr2 = 8,
            Apb0 = 9,
            Apb1 = 10,
            Apb2 = 11,
            Apb3 = 12,
            Apb4 = 13,
            Apb5 = 14,
            Apb6 = 15,
            Apb7 = 16,
            Apb8 = 17,
            Apb9 = 18,
            Apb10 = 19,
            Apb11 = 20,
            Apb12 = 21,
            Apb13 = 22,
            Apb14 = 23,
            Apb15 = 24,
            Apb16 = 25,
            Apb17 = 26,
            Apb18 = 27,
            Apb19 = 28,
            Apb20 = 29,
            Apb21 = 30,
            Apb22 = 31,
            Apb23 = 32,
            Apb24 = 33,
            Apb25 = 34,
            Apb26 = 35,
            Apb27 = 36,
            Apb28 = 37,
            Apb29 = 38,
            Apb30 = 39,
            Apb31 = 40
        ]
    ],

    /// Bitfields of the `APBDMACHAN_CHANNEL_AHB_PTR_0` register.
    pub APBDMACHAN_CHANNEL_AHB_PTR_0 [
        /// APB-DMA Starting Address for AHB Bus.
        AHB_BASE OFFSET(2) NUMBITS(30) []
    ],

    /// Bitfields of the `APBDMACHAN_CHANNEL_AHB_SEQ_0` register.
    pub APBDMACHAN_CHANNEL_AHB_SEQ_0 [
        /// Where an interrupt should be issued to.
        INTR_ENB OFFSET(31) NUMBITS(1) [
            Cop = 0,
            Cpu = 1
        ],

        /// AHB Bus Width.
        AHB_BUS_WIDTH OFFSET(28) NUMBITS(3) [
            BusWidth8 = 0,
            BusWidth16 = 1,
            BusWidth32 = 2,
            BusWidth64 = 3,
            BusWidth128 = 4
        ],

        /// When enabled, the data going to the AHB is exchanged backwards as 8-bit chunks.
        ///
        /// Example: `[31:0] --> {[7:0], [15:8], [23:16], [31:24]}`.
        AHB_DATA_SWAP OFFSET(27) NUMBITS(1) [],

        /// AHB Burst Size DMA Burst Length (encoded).
        AHB_BURST OFFSET(24) NUMBITS(3) [
            DmaBurst1Words = 4,
            DmaBurst4Words = 5,
            DmaBurst8Words = 6
        ],

        /// 2X Double Buffering Mode (for Run-Multiple Mode with No Wrap Operations).
        DBL_BUF OFFSET(19) NUMBITS(1) [
            ReloadFor1XBlocks = 0,
            ReloadFor2XBlocks = 1
        ],

        /// AHB Address Wrap-around Window.
        AHB_ADDR_WRAP OFFSET(16) NUMBITS(3) [
            NoWrap = 0,
            WrapOn1Words = 1,
            WrapOn2Words = 2,
            WrapOn4Words = 3,
            WrapOn8Words = 4,
            WrapOn16Words = 5,
            WrapOn32Words = 6,
            WrapOn64Words = 7
        ]
    ],

    /// Bitfields of the `APBDMACHAN_CHANNEL_APB_PTR_0` register.
    pub APBDMACHAN_CHANNEL_APB_PTR_0 [
        /// APB-DMA Starting address for APB Bus.
        APB_BASE OFFSET(2) NUMBITS(30) []
    ],

    /// Bitfields of the `APBDMACHAN_CHANNEL_APB_SEQ_0` register.
    pub APBDMACHAN_CHANNEL_APB_SEQ_0 [
        /// APB bus width.
        APB_BUS_WIDTH OFFSET(28) NUMBITS(3) [
            BusWidth8 = 0,
            BusWidth16 = 1,
            BusWidth32 = 2,
            BusWidth64 = 3,
            BusWidth128 = 4
        ],

        /// When enabled, the data going to the APB is exchanged backwards as 8-bit chunks.
        ///
        /// Example: `[31:0] --> {[7:0], [15:8], [23:16], [31:24]}`.
        APB_DATA_SWAP OFFSET(27) NUMBITS(1) [],

        /// APB Address Wrap-around Window.
        APB_ADDR_WRAP OFFSET(16) NUMBITS(3) [
            NoWrap = 0,
            WrapOn1Words = 1,
            WrapOn2Words = 2,
            WrapOn4Words = 3,
            WrapOn8Words = 4,
            WrapOn16Words = 5,
            WrapOn32Words = 6,
            WrapOn64Words = 7
        ]
    ],

    /// Bitfields of the `APBDMACHAN_CHANNEL_WCOUNT_0` register.
    pub APBDMACHAN_CHANNEL_WCOUNT_0 [
        /// Number of 32-bit word cycles.
        WCOUNT OFFSET(2) NUMBITS(28) []
    ],

    /// Bitfields of the `DMACHAN_CHANNEL_WORD_TRANSFER_0` register.
    pub DMACHAN_CHANNEL_WORD_TRANSFER_0 [
        /// APB-Current 32-bit Word Cycles.
        COUNT OFFSET(2) NUMBITS(28) []
    ]
}

register_structs! {
    /// Representation of the APB DMA Channel registers.
    #[allow(non_snake_case)]
    pub ChannelRegisters {
        (0x00 => pub APBDMACHAN_CHANNEL_CSR_0: ReadWrite<u32, APBDMACHAN_CHANNEL_CSR_0::Register>),
        (0x04 => pub APBDMACHAN_CHANNEL_STA_0: ReadWrite<u32, APBDMACHAN_CHANNEL_STA_0::Register>),
        (0x08 => pub APBDMACHAN_CHANNEL_DMA_BYTE_STA_0: ReadOnly<u32, APBDMACHAN_CHANNEL_DMA_BYTE_STA_0::Register>),
        (0x0C => pub APBDMACHAN_CHANNEL_CSRE_0: ReadWrite<u32, APBDMACHAN_CHANNEL_CSRE_0::Register>),
        (0x10 => pub APBDMACHAN_CHANNEL_AHB_PTR_0: ReadWrite<u32, APBDMACHAN_CHANNEL_AHB_PTR_0::Register>),
        (0x14 => pub APBDMACHAN_CHANNEL_AHB_SEQ_0: ReadWrite<u32, APBDMACHAN_CHANNEL_AHB_SEQ_0::Register>),
        (0x18 => pub APBDMACHAN_CHANNEL_APB_PTR_0: ReadWrite<u32, APBDMACHAN_CHANNEL_APB_PTR_0::Register>),
        (0x1C => pub APBDMACHAN_CHANNEL_APB_SEQ_0: ReadWrite<u32, APBDMACHAN_CHANNEL_APB_SEQ_0::Register>),
        (0x20 => pub APBDMACHAN_CHANNEL_WCOUNT_0: ReadWrite<u32, APBDMACHAN_CHANNEL_WCOUNT_0::Register>),
        (0x24 => pub DMACHAN_CHANNEL_WORD_TRANSFER_0: ReadOnly<u32, DMACHAN_CHANNEL_WORD_TRANSFER_0::Register>),
        (0x28 => @END),
    }
}

assert_eq_size!(ChannelRegisters, [u8; 0x28]);
