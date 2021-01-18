use register::{mmio::*, register_bitfields, register_structs};

use crate::memory_map::{DSIA, DSIB};

/// A pointer to the DSIA register block that can be accessed by dereferencing it.
pub const DSIA_REGISTERS: *const Registers = DSIA as *const Registers;

/// A pointer to the DSIB register block that can be accessed by dereferencing it.
pub const DSIB_REGISTERS: *const Registers = DSIB as *const Registers;

register_bitfields! {
    u32,

    /// Bitfields of the `DSI_INCR_SYNCPT_0` register.
    pub DSI_INCR_SYNCPT_0 [
        /// Condition mapped from raise/wait.
        COND OFFSET(8) NUMBITS(8) [
            Immediate = 0,
            OpDone = 1,
            RdDone = 2,
            RegWrSafe = 3,
            Cond4 = 4,
            Cond5 = 5,
            Cond6 = 6,
            Cond7 = 7,
            Cond8 = 8,
            Cond9 = 9,
            Cond10 = 10,
            Cond11 = 11,
            Cond12 = 12,
            Cond13 = 13,
            Cond14 = 14,
            Cond15 = 15,
            Cond16 = 16,
            Cond17 = 17,
            Cond18 = 18,
            Cond19 = 19,
            Cond20 = 20,
            Cond21 = 21,
            Cond22 = 22,
            Cond23 = 23,
            Cond24 = 24,
            Cond25 = 25,
            Cond26 = 26,
            Cond27 = 27,
            Cond28 = 28,
            Cond29 = 29,
            Cond30 = 30,
            Cond31 = 31
        ],

        /// Syncpt index value.
        INDX OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `DSI_INCR_SYNCPT_CNTRL_0` register.
    pub DSI_INCR_SYNCPT_CNTRL_0 [
        /// Prevent stalling the client host interface when FIFOs are full and instead drop
        /// the INCR_SYNCPT methods.
        INCR_SYNCPT_NO_STALL OFFSET(8) NUMBITS(1) [],

        /// Resets all internal states of the client syncpt.
        INCR_SYNCPT_SOFT_RESET OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `DSI_INCR_SYNCPT_ERROR_0` register.
    pub DSI_INCR_SYNCPT_ERROR_0 [
        /// Set if FIFO hits COND overflows with stalling disabled. This bit is sticky.
        COND_STATUS OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `DSI_CTXSW_0` register.
    pub DSI_CTXSW_0 [
        /// The next requested channel.
        ///
        /// NOTE: This field is read-only.
        NEXT_CHANNEL OFFSET(28) NUMBITS(31) [],

        /// The next requested class.
        ///
        /// NOTE: This field is read-only.
        NEXT_CLASS OFFSET(16) NUMBITS(10) [],

        /// The current working channel.
        CURR_CHANNEL OFFSET(12) NUMBITS(4) [],

        /// Whether incoming context switch requests should be acknowledged automatically.
        AUTO_ACK OFFSET(11) NUMBITS(1) [
            Manual = 0,
            AutoAck = 1
        ],

        /// The current working class.
        CURR_CLASS OFFSET(0) NUMBITS(10) []
    ],

    /// Bitfields of the `DSI_DSI_POWER_CONTROL_0` register.
    pub DSI_DSI_POWER_CONTROL_0 [
        /// DSI interface Enable.
        LEG_DSI_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `DSI_INT_ENABLE_0` register.
    pub DSI_INT_ENABLE_0 [
        /// Context Switch Interrupt Enable.
        CTXSW_INT_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `DSI_INT_STATUS_0` register.
    pub DSI_INT_STATUS_0 [
        /// Context switch interrupt status.
        CTXSW_INT OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `DSI_INT_MASK_0` register.
    pub DSI_INT_MASK_0 [
        /// Context Switch Interrupt Mask.
        CTXSW_INT_MASK OFFSET(0) NUMBITS(1) [
            Masked = 0,
            Unmasked = 1
        ]
    ],

    /// Bitfields of the `DSI_HOST_DSI_CONTROL_0` register.
    pub DSI_HOST_DSI_CONTROL_0 [
        /// Clears FIFO underflow/overflow flags.
        ///
        /// NOTE: This field is write-only.
        FIFO_STAT_RESET OFFSET(21) NUMBITS(1) [],

        /// Whether the Verification CRC should be reset to `0xFFFF_FFFF`.
        ///
        /// NOTE: This field is write-only.
        CRC_RESET OFFSET(20) NUMBITS(1) [],

        /// Physical clock divider value for byte clock.
        DSI_PHY_CLK_DIV OFFSET(16) NUMBITS(3) [
            Div1 = 0,
            Div2 = 1
        ],

        /// Controls the source of the trigger to start sending packets.
        HOST_TX_TRIG_SRC OFFSET(12) NUMBITS(2) [
            /// Start of Line signal from Display Controller.
            Sol = 0,
            /// How full the FIFO is. Level determined elsewhere.
            FifoLevel = 1,
            /// Determined by a write to the `DSI_HOST_TRIGGER` field of the
            /// `DSI_TRIGGER` register.
            Immediate = 2
        ],

        /// Ultra Low Power mode.
        DSI_ULTRA_LOW_POWER OFFSET(8) NUMBITS(2) [
            Normal = 0,
            EnterUlpm = 1,
            ExitUlpm = 2
        ],

        /// Initiate an Escape Mode Peripheral Reset.
        PERIPH_RESET OFFSET(7) NUMBITS(1) [],

        /// Host raw data mode. In this mode, all data is sent exactly as written
        /// without any attempt to decode packet headers.
        RAW_DATA OFFSET(6) NUMBITS(1) [],

        /// DSI high speed transmission of packets.
        DSI_HIGH_SPEED_TRANS OFFSET(5) NUMBITS(1) [
            /// Low speed - Unlikely to ever be used.
            Low = 0,
            /// High speed.
            High = 1
        ],

        /// Host Write FIFO Select. In video mode, software shall not program this
        /// to `Video`.
        PKT_WR_FIFO_SEL OFFSET(4) NUMBITS(1) [
            /// Write data to the small host data FIFO only.
            Host = 0,
            /// Write data to both the host and video line store FIFO, in series.
            Video = 1
        ],

        /// Generate BTA immediately, e.g. for Tearing Effect reporting.
        IMM_BTA OFFSET(3) NUMBITS(1) [],

        /// Generate BTA at the end of Host packets.
        PKT_BTA OFFSET(2) NUMBITS(1) [],

        /// Enable hardware Check Sum (CS) for Host packets.
        CS_ENABLE OFFSET(1) NUMBITS(1) [],

        /// Enable hardware Error Code Correction (ECC) for Host packets.
        ECC_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `DSI_DSI_CONTROL_0` register.
    pub DSI_DSI_CONTROL_0 [
        /// Control signal to turn off clock monitoring when enabled for debug, on
        /// every DSI byte clock debug signal toggle.
        DSI_DBG_ENABLE OFFSET(31) NUMBITS(1) [],

        /// DSI specification supports different bit orderings (only 16 bpp) in
        /// command mode.
        DFMT_16BPP_SWAP_EN OFFSET(30) NUMBITS(1) [],

        /// Control for the HS clock lane.
        DSI_HS_CLK_CTRL OFFSET(20) NUMBITS(1) [
            /// HS clock is on all the time.
            Continuos = 0,
            /// HS clock is only active during HS transmissions.
            TxOnly = 1
        ],

        /// Virtual channel ID. Sent as part of the packet header and used to
        /// distinguish multiple displays.
        DSI_VIRTUAL_CHANNEL OFFSET(16) NUMBITS(2) [],

        /// Pixel Data format transmitted. Only this information for constructing
        /// RGB data packets in hardware.
        DSI_DATA_FORMAT OFFSET(12) NUMBITS(2) [
            /// 16 bpp RGB Packed. 2 bytes used per pixel.
            Bit16P = 0,
            /// 18 bpp RGB Not-packed. 3 bytes used per pixel.
            Bit18NP = 1,
            /// 18 bpp RGB Packed. 2.25 bytes used per pixel.
            Bit18P = 2,
            /// 24 bpp RGB Packed. 3 bytes used per pixel.
            Bit24P = 3
        ],

        /// Controls the source of the trigger to start sending packets.
        VID_TX_TRIG_SRC OFFSET(8) NUMBITS(2) [
            /// Start of Line signal from the Display Controller.
            Sol = 0,
            /// How full the FIFO is. Level determined elsewhere.
            FifoLevel = 1,
            /// Determined by a write to the `DSI_VID_TRIGGER` field of the
            /// `DSI_TRIGGER` register.
            Immediate = 2
        ],

        /// Number of D-PHY data lanes used by Display for HS transmission.
        DSI_NUM_DATA_LANES OFFSET(4) NUMBITS(2) [
            /// 1 data lane.
            One = 0,
            /// 2 data lanes.
            Two = 1,
            /// 3 data lanes.
            Three = 2,
            /// 4 data lanes.
            Four = 3
        ],

        /// Allows insertion of DCS commands during Display Controller generated
        /// packets.
        VID_DCS_ENABLE OFFSET(3) NUMBITS(1) [],

        /// Source of video pixels.
        DSI_VID_SOURCE OFFSET(2) NUMBITS(1) [
            /// Pixels come from Display A.
            Display0 = 0,
            /// Pixels come from Display B.
            Display1 = 1
        ],

        /// Video DSI Interface Enable.
        DSI_VID_ENABLE OFFSET(1) NUMBITS(1) [],

        /// Host DSI Interface Enable.
        DSI_HOST_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `DSI_DSI_SOL_DELAY_0` register.
    pub DSI_DSI_SOL_DELAY_0 [
        /// Start Of Line before generating output packets.
        SOL_DELAY OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `DSI_DSI_MAX_THRESHOLD_0` register.
    pub DSI_DSI_MAX_THRESHOLD_0 [
        /// Start draining FIFO once this threshold is met. This register can be used
        /// for DBI mode when line packet data exceeds the size of the data FIFO.
        MAX_THRESHOLD OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `DSI_DSI_TRIGGER_0` register.
    pub DSI_DSI_TRIGGER_0 [
        /// Triggers host transmission in hardware immediately.
        DSI_HOST_TRIGGER OFFSET(1) NUMBITS(1) [],

        /// Triggers video transmission in hardware immediately.
        DSI_VID_TRIGGER OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `DSI_DSI_STATUS_0` register.
    pub DSI_DSI_STATUS_0 [
        /// Indicates that the DSI is IDLE.
        DSI_IDLE OFFSET(10) NUMBITS(1) [],

        /// Indicates that a Line buffer underflow event happened.
        LB_UNDERFLOW OFFSET(9) NUMBITS(1) [],

        /// Indicates that a Line buffer overflow event happened.
        LB_OVERFLOW OFFSET(8) NUMBITS(1) [],

        /// Count of how many data words are left in the Host Read Data Return FIFO.
        RD_FIFO_COUNT OFFSET(0) NUMBITS(5) []
    ]
}

register_structs! {
    /// Representation of the MIPI-DSI registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => pub DSI_INCR_SYNCPT_0: ReadWrite<u32, DSI_INCR_SYNCPT_0::Register>),
        (0x04 => pub DSI_INCR_SYNCPT_CNTRL_0: ReadWrite<u32, DSI_INCR_SYNCPT_CNTRL_0::Register>),
        (0x08 => pub DSI_INCR_SYNCPT_ERROR_0: ReadWrite<u32, DSI_INCR_SYNCPT_ERROR_0::Register>),
        (0x0C => _reserved),
        (0x20 => pub DSI_CTXSW_0: ReadWrite<u32, DSI_CTXSW_0::Register>),
        (0x24 => pub DSI_DSI_RD_DATA_0: ReadOnly<u32>),
        (0x28 => pub DSI_DSI_WR_DATA_0: ReadWrite<u32>),
        (0x2C => pub DSI_DSI_POWER_CONTROL_0: ReadWrite<u32, DSI_DSI_POWER_CONTROL_0::Register>),
        (0x30 => pub DSI_INT_ENABLE_0: ReadWrite<u32, DSI_INT_ENABLE_0::Register>),
        (0x34 => pub DSI_INT_STATUS_0: ReadWrite<u32, DSI_INT_STATUS_0::Register>),
        (0x38 => pub DSI_INT_MASK_0: ReadWrite<u32, DSI_INT_MASK_0::Register>),
        (0x3C => pub DSI_HOST_DSI_CONTROL_0: ReadWrite<u32, DSI_HOST_DSI_CONTROL_0::Register>),
        (0x40 => pub DSI_DSI_CONTROL_0: ReadWrite<u32, DSI_DSI_CONTROL_0::Register>),
        (0x44 => pub DSI_DSI_SOL_DELAY_0: ReadWrite<u32, DSI_DSI_SOL_DELAY_0::Register>),
        (0x48 => pub DSI_DSI_MAX_THRESHOLD_0: ReadWrite<u32, DSI_DSI_MAX_THRESHOLD_0::Register>),
        (0x4C => pub DSI_DSI_TRIGGER_0: ReadWrite<u32, DSI_DSI_TRIGGER_0::Register>),
        (0x50 => pub DSI_DSI_TX_CRC_0: ReadOnly<u32>),
        (0x54 => pub DSI_DSI_STATUS_0: ReadOnly<u32, DSI_DSI_STATUS_0::Register>),
        (0x58 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x58]);
