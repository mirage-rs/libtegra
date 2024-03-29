use tock_registers::{register_bitfields, register_structs, registers::*};

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
    ],

    /// Bitfields of the `DSI_DSI_INIT_SEQ_CONTROL_0` register.
    pub DSI_DSI_INIT_SEQ_CONTROL_0 [
        /// Frame Initialization Sequence Byte Count. This specifies the number of
        /// frame initialization sequence cycles to send.
        DSI_FRAME_INIT_BYTE_COUNT OFFSET(8) NUMBITS(7) [],

        /// Send Initialization Sequence.
        DSI_SEND_INIT_SEQUENCE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_SEQ_0_LO_0` register.
    pub DSI_DSI_PKT_SEQ_0_LO_0 [
        /// For packet sequence 0, forces the D-PHY to go to LP mode after the last
        /// packet of the sequence has been transmitted.
        SEQ_0_FORCE_LP OFFSET(30) NUMBITS(1) [],

        /// Packet 2 enable.
        PKT_02_EN OFFSET(29) NUMBITS(1) [],

        /// Packet 2 Packet ID.
        PKT_02_ID OFFSET(23) NUMBITS(6) [],

        /// Packet 2 size pointer.
        PKT_02_SIZE OFFSET(20) NUMBITS(3) [],

        /// Packet 1 enable.
        PKT_01_EN OFFSET(19) NUMBITS(1) [],

        /// Packet 1 Packet ID.
        PKT_01_ID OFFSET(13) NUMBITS(6) [],

        /// Packet 1 size pointer.
        PKT_01_SIZE OFFSET(10) NUMBITS(3) [],

        /// Packet 0 enable.
        PKT_00_EN OFFSET(9) NUMBITS(1) [],

        /// Packet 0 Packet ID.
        PKT_00_ID OFFSET(3) NUMBITS(6) [],

        /// Packet 0 size pointer.
        PKT_00_SIZE OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_SEQ_0_HI_0` register.
    pub DSI_DSI_PKT_SEQ_0_HI_0 [
        /// Packet 5 enable.
        PKT_05_EN OFFSET(29) NUMBITS(1) [],

        /// Packet 5 Packet ID.
        PKT_05_ID OFFSET(23) NUMBITS(6) [],

        /// Packet 5 size pointer.
        PKT_05_SIZE OFFSET(20) NUMBITS(3) [],

        /// Packet 4 enable.
        PKT_04_EN OFFSET(19) NUMBITS(1) [],

        /// Packet 4 Packet ID.
        PKT_04_ID OFFSET(13) NUMBITS(6) [],

        /// Packet 4 size pointer.
        PKT_04_SIZE OFFSET(10) NUMBITS(3) [],

        /// Packet 3 enable.
        PKT_03_EN OFFSET(9) NUMBITS(1) [],

        /// Packet 3 Packet ID.
        PKT_03_ID OFFSET(3) NUMBITS(6) [],

        /// Packet 3 size pointer.
        PKT_03_SIZE OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_SEQ_1_LO_0` register.
    pub DSI_DSI_PKT_SEQ_1_LO_0 [
        /// For packet sequence 1, forces the D-PHY to go to LP mode after the last
        /// packet of the sequence has been transmitted.
        SEQ_1_FORCE_LP OFFSET(30) NUMBITS(1) [],

        /// Packet 2 enable.
        PKT_12_EN OFFSET(29) NUMBITS(1) [],

        /// Packet 2 Packet ID.
        PKT_12_ID OFFSET(23) NUMBITS(6) [],

        /// Packet 2 size pointer.
        PKT_12_SIZE OFFSET(20) NUMBITS(3) [],

        /// Packet 1 enable.
        PKT_11_EN OFFSET(19) NUMBITS(1) [],

        /// Packet 1 Packet ID.
        PKT_11_ID OFFSET(13) NUMBITS(6) [],

        /// Packet 1 size pointer.
        PKT_11_SIZE OFFSET(10) NUMBITS(3) [],

        /// Packet 0 enable.
        PKT_10_EN OFFSET(9) NUMBITS(1) [],

        /// Packet 0 Packet ID.
        PKT_10_ID OFFSET(3) NUMBITS(6) [],

        /// Packet 0 size pointer.
        PKT_10_SIZE OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_SEQ_1_HI_0` register.
    pub DSI_DSI_PKT_SEQ_1_HI_0 [
        /// Packet 5 enable.
        PKT_15_EN OFFSET(29) NUMBITS(1) [],

        /// Packet 5 Packet ID.
        PKT_15_ID OFFSET(23) NUMBITS(6) [],

        /// Packet 5 size pointer.
        PKT_15_SIZE OFFSET(20) NUMBITS(3) [],

        /// Packet 4 enable.
        PKT_14_EN OFFSET(19) NUMBITS(1) [],

        /// Packet 4 Packet ID.
        PKT_14_ID OFFSET(13) NUMBITS(6) [],

        /// Packet 4 size pointer.
        PKT_14_SIZE OFFSET(10) NUMBITS(3) [],

        /// Packet 3 enable.
        PKT_13_EN OFFSET(9) NUMBITS(1) [],

        /// Packet 3 Packet ID.
        PKT_13_ID OFFSET(3) NUMBITS(6) [],

        /// Packet 3 size pointer.
        PKT_13_SIZE OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_SEQ_2_LO_0` register.
    pub DSI_DSI_PKT_SEQ_2_LO_0 [
        /// For packet sequence 2, forces the D-PHY to go to LP mode after the last
        /// packet of the sequence has been transmitted.
        SEQ_2_FORCE_LP OFFSET(30) NUMBITS(1) [],

        /// Packet 2 enable.
        PKT_22_EN OFFSET(29) NUMBITS(1) [],

        /// Packet 2 Packet ID.
        PKT_22_ID OFFSET(23) NUMBITS(6) [],

        /// Packet 2 size pointer.
        PKT_22_SIZE OFFSET(20) NUMBITS(3) [],

        /// Packet 1 enable.
        PKT_21_EN OFFSET(19) NUMBITS(1) [],

        /// Packet 1 Packet ID.
        PKT_21_ID OFFSET(13) NUMBITS(6) [],

        /// Packet 1 size pointer.
        PKT_21_SIZE OFFSET(10) NUMBITS(3) [],

        /// Packet 0 enable.
        PKT_20_EN OFFSET(9) NUMBITS(1) [],

        /// Packet 0 Packet ID.
        PKT_20_ID OFFSET(3) NUMBITS(6) [],

        /// Packet 0 size pointer.
        PKT_20_SIZE OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_SEQ_2_HI_0` register.
    pub DSI_DSI_PKT_SEQ_2_HI_0 [
        /// Packet 5 enable.
        PKT_25_EN OFFSET(29) NUMBITS(1) [],

        /// Packet 5 Packet ID.
        PKT_25_ID OFFSET(23) NUMBITS(6) [],

        /// Packet 5 size pointer.
        PKT_25_SIZE OFFSET(20) NUMBITS(3) [],

        /// Packet 4 enable.
        PKT_24_EN OFFSET(19) NUMBITS(1) [],

        /// Packet 4 Packet ID.
        PKT_24_ID OFFSET(13) NUMBITS(6) [],

        /// Packet 4 size pointer.
        PKT_24_SIZE OFFSET(10) NUMBITS(3) [],

        /// Packet 3 enable.
        PKT_23_EN OFFSET(9) NUMBITS(1) [],

        /// Packet 3 Packet ID.
        PKT_23_ID OFFSET(3) NUMBITS(6) [],

        /// Packet 3 size pointer.
        PKT_23_SIZE OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_SEQ_3_LO_0` register.
    pub DSI_DSI_PKT_SEQ_3_LO_0 [
        /// For packet sequence 3, forces the D-PHY to go to LP mode after the last
        /// packet of the sequence has been transmitted.
        SEQ_3_FORCE_LP OFFSET(30) NUMBITS(1) [],

        /// Packet 2 enable.
        PKT_32_EN OFFSET(29) NUMBITS(1) [],

        /// Packet 2 Packet ID.
        PKT_32_ID OFFSET(23) NUMBITS(6) [],

        /// Packet 2 size pointer.
        PKT_32_SIZE OFFSET(20) NUMBITS(3) [],

        /// Packet 1 enable.
        PKT_31_EN OFFSET(19) NUMBITS(1) [],

        /// Packet 1 Packet ID.
        PKT_31_ID OFFSET(13) NUMBITS(6) [],

        /// Packet 1 size pointer.
        PKT_31_SIZE OFFSET(10) NUMBITS(3) [],

        /// Packet 0 enable.
        PKT_30_EN OFFSET(9) NUMBITS(1) [],

        /// Packet 0 Packet ID.
        PKT_30_ID OFFSET(3) NUMBITS(6) [],

        /// Packet 0 size pointer.
        PKT_30_SIZE OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_SEQ_3_HI_0` register.
    pub DSI_DSI_PKT_SEQ_3_HI_0 [
        /// Packet 5 enable.
        PKT_35_EN OFFSET(29) NUMBITS(1) [],

        /// Packet 5 Packet ID.
        PKT_35_ID OFFSET(23) NUMBITS(6) [],

        /// Packet 5 size pointer.
        PKT_35_SIZE OFFSET(20) NUMBITS(3) [],

        /// Packet 4 enable.
        PKT_34_EN OFFSET(19) NUMBITS(1) [],

        /// Packet 4 Packet ID.
        PKT_34_ID OFFSET(13) NUMBITS(6) [],

        /// Packet 4 size pointer.
        PKT_34_SIZE OFFSET(10) NUMBITS(3) [],

        /// Packet 3 enable.
        PKT_33_EN OFFSET(9) NUMBITS(1) [],

        /// Packet 3 Packet ID.
        PKT_33_ID OFFSET(3) NUMBITS(6) [],

        /// Packet 3 size pointer.
        PKT_33_SIZE OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_SEQ_4_LO_0` register.
    pub DSI_DSI_PKT_SEQ_4_LO_0 [
        /// For packet sequence 4, forces the D-PHY to go to LP mode after the last
        /// packet of the sequence has been transmitted.
        SEQ_4_FORCE_LP OFFSET(30) NUMBITS(1) [],

        /// Packet 2 enable.
        PKT_42_EN OFFSET(29) NUMBITS(1) [],

        /// Packet 2 Packet ID.
        PKT_42_ID OFFSET(23) NUMBITS(6) [],

        /// Packet 2 size pointer.
        PKT_42_SIZE OFFSET(20) NUMBITS(3) [],

        /// Packet 1 enable.
        PKT_41_EN OFFSET(19) NUMBITS(1) [],

        /// Packet 1 Packet ID.
        PKT_41_ID OFFSET(13) NUMBITS(6) [],

        /// Packet 1 size pointer.
        PKT_41_SIZE OFFSET(10) NUMBITS(3) [],

        /// Packet 0 enable.
        PKT_40_EN OFFSET(9) NUMBITS(1) [],

        /// Packet 0 Packet ID.
        PKT_40_ID OFFSET(3) NUMBITS(6) [],

        /// Packet 0 size pointer.
        PKT_40_SIZE OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_SEQ_4_HI_0` register.
    pub DSI_DSI_PKT_SEQ_4_HI_0 [
        /// Packet 5 enable.
        PKT_45_EN OFFSET(29) NUMBITS(1) [],

        /// Packet 5 Packet ID.
        PKT_45_ID OFFSET(23) NUMBITS(6) [],

        /// Packet 5 size pointer.
        PKT_45_SIZE OFFSET(20) NUMBITS(3) [],

        /// Packet 4 enable.
        PKT_44_EN OFFSET(19) NUMBITS(1) [],

        /// Packet 4 Packet ID.
        PKT_44_ID OFFSET(13) NUMBITS(6) [],

        /// Packet 4 size pointer.
        PKT_44_SIZE OFFSET(10) NUMBITS(3) [],

        /// Packet 3 enable.
        PKT_43_EN OFFSET(9) NUMBITS(1) [],

        /// Packet 3 Packet ID.
        PKT_43_ID OFFSET(3) NUMBITS(6) [],

        /// Packet 3 size pointer.
        PKT_43_SIZE OFFSET(0) NUMBITS(3) []
    ],
    //
    /// Bitfields of the `DSI_DSI_PKT_SEQ_5_LO_0` register.
    pub DSI_DSI_PKT_SEQ_5_LO_0 [
        /// For packet sequence 5, forces the D-PHY to go to LP mode after the last
        /// packet of the sequence has been transmitted.
        SEQ_5_FORCE_LP OFFSET(30) NUMBITS(1) [],

        /// Packet 2 enable.
        PKT_52_EN OFFSET(29) NUMBITS(1) [],

        /// Packet 2 Packet ID.
        PKT_52_ID OFFSET(23) NUMBITS(6) [],

        /// Packet 2 size pointer.
        PKT_52_SIZE OFFSET(20) NUMBITS(3) [],

        /// Packet 1 enable.
        PKT_51_EN OFFSET(19) NUMBITS(1) [],

        /// Packet 1 Packet ID.
        PKT_51_ID OFFSET(13) NUMBITS(6) [],

        /// Packet 1 size pointer.
        PKT_51_SIZE OFFSET(10) NUMBITS(3) [],

        /// Packet 0 enable.
        PKT_50_EN OFFSET(9) NUMBITS(1) [],

        /// Packet 0 Packet ID.
        PKT_50_ID OFFSET(3) NUMBITS(6) [],

        /// Packet 0 size pointer.
        PKT_50_SIZE OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_SEQ_5_HI_0` register.
    pub DSI_DSI_PKT_SEQ_5_HI_0 [
        /// Packet 5 enable.
        PKT_55_EN OFFSET(29) NUMBITS(1) [],

        /// Packet 5 Packet ID.
        PKT_55_ID OFFSET(23) NUMBITS(6) [],

        /// Packet 5 size pointer.
        PKT_55_SIZE OFFSET(20) NUMBITS(3) [],

        /// Packet 4 enable.
        PKT_54_EN OFFSET(19) NUMBITS(1) [],

        /// Packet 4 Packet ID.
        PKT_54_ID OFFSET(13) NUMBITS(6) [],

        /// Packet 4 size pointer.
        PKT_54_SIZE OFFSET(10) NUMBITS(3) [],

        /// Packet 3 enable.
        PKT_53_EN OFFSET(9) NUMBITS(1) [],

        /// Packet 3 Packet ID.
        PKT_53_ID OFFSET(3) NUMBITS(6) [],

        /// Packet 3 size pointer.
        PKT_53_SIZE OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_DSI_DCS_CMDS_0` register.
    pub DSI_DSI_DCS_CMDS_0 [
        /// DCS command for Line Type 5.
        LT5_DCS_CMD OFFSET(8) NUMBITS(8) [],

        /// DCS command for Line Type 3.
        LT3_DCS_CMD OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_LEN_0_1_0` register.
    pub DSI_DSI_PKT_LEN_0_1_0 [
        /// Packet length 1 (in bytes).
        LENGTH_1 OFFSET(16) NUMBITS(16) [],

        /// Packet length 0 (in bytes).
        LENGTH_0 OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_LEN_2_3_0` register.
    pub DSI_DSI_PKT_LEN_2_3_0 [
        /// Packet length 3 (in bytes).
        LENGTH_3 OFFSET(16) NUMBITS(16) [],

        /// Packet length 2 (in bytes).
        LENGTH_2 OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_LEN_4_5_0` register.
    pub DSI_DSI_PKT_LEN_4_5_0 [
        /// Packet length 5 (in bytes).
        LENGTH_5 OFFSET(16) NUMBITS(16) [],

        /// Packet length 4 (in bytes).
        LENGTH_4 OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `DSI_DSI_PKT_LEN_6_7_0` register.
    pub DSI_DSI_PKT_LEN_6_7_0 [
        /// Packet length 7 (in bytes).
        LENGTH_7 OFFSET(16) NUMBITS(16) [],

        /// Packet length 6 (in bytes).
        LENGTH_6 OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `DSI_DSI_PHY_TIMING_0_0` register.
    pub DSI_DSI_PHY_TIMING_0_0 [
        /// Time to drive LP11 after HS.
        DSI_THSDEXIT OFFSET(24) NUMBITS(8) [],

        /// Time to drive HS flipped bit at EOT.
        DSI_THSTRAIL OFFSET(16) NUMBITS(8) [],

        /// Time to drive HS0 before SOT.
        DSI_TDATZERO OFFSET(8) NUMBITS(8) [],

        /// Time to drive LP00 before HS data.
        DSI_THSPREPR OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `DSI_DSI_PHY_TIMING_1_0` register.
    pub DSI_DSI_PHY_TIMING_1_0 [
        /// Time to drive HS0 before the clock goes to LP1.
        DSI_TCLKTRAIL OFFSET(24) NUMBITS(8) [],

        /// Time to drive clock after the last HS data.
        DSI_TCLKPOST OFFSET(16) NUMBITS(8) [],

        /// Time to drive LP00 before HS clock.
        DSI_TCLKZERO OFFSET(8) NUMBITS(8) [],

        /// LP period.
        DSI_TTLPX OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `DSI_DSI_PHY_TIMING_2_0` register.
    pub DSI_DSI_PHY_TIMING_2_0 [
        /// Time to drive LP0 before CLK_ZERO starts off on the clock lane.
        DSI_TCLKPREPARE OFFSET(16) NUMBITS(8) [],

        /// Time to run clock before enabling data lane.
        DSI_TCLKPRE OFFSET(8) NUMBITS(8) [],

        /// LP period when exiting ULPM, in units of 512 byte clocks.
        DSI_TWAKEUP OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `DSI_DSI_BTA_TIMING_0` register.
    pub DSI_DSI_BTA_TIMING_0 [
        /// Programmable Time delay between end of Host packet transmission and
        /// generation of Pkt BTA in PKT_BTA mode.
        DSI_TPKBTA OFFSET(24) NUMBITS(8) [],

        /// Time to Drive LP00 at end of BTA (5 * TTLPX).
        DSI_TTAGET OFFSET(16) NUMBITS(8) [],

        /// Time to Receive LP00 at end of BTA (2 * TTLPX).
        DSI_TTASURE OFFSET(8) NUMBITS(8) [],

        /// Time to drive LP00 at start of BTA (4 * TTLPX).
        DSI_TTAGO OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `DSI_DSI_TIMEOUT_0_0` register.
    pub DSI_DSI_TIMEOUT_0_0 [
        /// Low Power Receive (Host) Time Out terminal count.
        LRXH_TO OFFSET(16) NUMBITS(16) [],

        /// High Speed Transmit Time Out terminal count.
        HTX_TO OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `DSI_DSI_TIMEOUT_1_0` register.
    pub DSI_DSI_TIMEOUT_1_0 [
        /// Peripheral Reset duration.
        PR_TO OFFSET(16) NUMBITS(16) [],

        /// Turn Around Time Out terminal count.
        TA_TO OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `DSI_DSI_TO_TALLY_0` register.
    pub DSI_DSI_TO_TALLY_0 [
        /// Peripheral Reset time out status.
        ///
        /// NOTE: This field is read-only.
        P_RESET_STATUS OFFSET(24) NUMBITS(1) [
            InReset = 0,
            Ready = 1
        ],

        /// Turn Around time out tally.
        TA_TALLY OFFSET(16) NUMBITS(8) [],

        /// LP Rx time out tally.
        LRXH_TALLY OFFSET(8) NUMBITS(8) [],

        /// HS Tx time out tally.
        HTX_TALLY OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `DSI_PAD_CONTROL_0` register.
    pub DSI_PAD_CONTROL_0 [
        /// Enable pad pulldown for clock bit at power on.
        DSI_PAD_PULLDN_CLK_ENAB OFFSET(24) NUMBITS(1) [],

        /// Enable pad pulldown for data bits at power on.
        DSI_PAD_PULLDN_ENAB OFFSET(16) NUMBITS(4) [],

        /// Power down for clock bit, drivers, receivers and contention detectors.
        DSI_PAD_PDIO_CLK OFFSET(8) NUMBITS(1) [],

        /// Power down for data bit, drivers, receivers, and contention detectors.
        DSI_PAD_PDIO OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `DSI_PAD_CONTROL_CD_0` register.
    pub DSI_PAD_CONTROL_CD_0 [
        /// Level adjust on low limit of detection.
        DSI_PAD_CDDNADJ OFFSET(16) NUMBITS(3) [],

        /// Clock bit contention detector enable.
        DSI_PAD_CD_EN_CLK OFFSET(8) NUMBITS(1) [],

        /// Data bits contention detector enable.
        DSI_PAD_CD_EN OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `DSI_PAD_CD_STATUS_0` register.
    pub DSI_PAD_CD_STATUS_0 [
        DSI_PAD_CDN_CLK OFFSET(18) NUMBITS(1) [],

        DSI_PAD_CDP_CLK OFFSET(16) NUMBITS(1) [],

        DSI_PAD_CDN OFFSET(8) NUMBITS(4) [],

        DSI_PAD_CDP OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `DSI_DSI_VID_MODE_CONTROL_0` register.
    pub DSI_DSI_VID_MODE_CONTROL_0 [
        /// Vertical blank. LINE TYPE on which the host command packet is to be
        /// transmitted.
        DSI_LINE_TYPE OFFSET(1) NUMBITS(3) [],

        /// Whether host command packet during video mode should be enabled.
        DSI_CMD_PKT_VID_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `DSI_PAD_CONTROL_1_0` register.
    pub DSI_PAD_CONTROL_1_0 [
        /// Input delay trimmer for data bit 3. Each tap delays 40 ps.
        DSI_PAD_OUTADJ3 OFFSET(12) NUMBITS(3) [],

        /// Input delay trimmer for data bit 2. Each tap delays 40 ps.
        DSI_PAD_OUTADJ2 OFFSET(8) NUMBITS(3) [],

        /// Input delay trimmer for data bit 1. Each tap delays 40 ps.
        DSI_PAD_OUTADJ1 OFFSET(4) NUMBITS(3) [],

        /// Input delay trimmer for data bit 0. Each tap delays 40 ps.
        DSI_PAD_OUTADJ0 OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_PAD_CONTROL_2_0` register.
    pub DSI_PAD_CONTROL_2_0 [
        /// Pull-up slew rate adjust.
        DSI_PAD_SLEWUPADJ OFFSET(16) NUMBITS(3) [],

        /// Pull-down slew rate adjust.
        DSI_PAD_SLEWDNADJ OFFSET(12) NUMBITS(3) [],

        /// Driver pull-up impedance control.
        DSI_PAD_LPUPADJ OFFSET(8) NUMBITS(3) [],

        /// Driver pull-down impedance control.
        DSI_PAD_LPDNADJ OFFSET(4) NUMBITS(3) [],

        /// Output trimmer delay for clock bit. Each tap delays 40 ps.
        DSI_PAD_OUTADJCLK OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `DSI_PAD_CONTROL_3_0` register.
    pub DSI_PAD_CONTROL_3_0 [
        /// Power down regulator which supplies current to serializer/deserializer
        /// logic.
        DSI_PAD_PDVCLAMP OFFSET(28) NUMBITS(1) [],

        /// Increase bandwidth of differential receiver.
        DSI_PAD_BANDWD_IN OFFSET(16) NUMBITS(1) [],

        /// Clock bit HS driver pull up pre-emphasis.
        DSI_PAD_PREEMP_PD_CLK OFFSET(12) NUMBITS(2) [],

        /// Clock bit HS driver pull up pre-emphasis.
        DSI_PAD_PREEMP_PU_CLK OFFSET(8) NUMBITS(2) [],

        /// Data bit HS driver pull down pre-emphasis.
        DSI_PAD_PREEMP_PD OFFSET(4) NUMBITS(2) [],

        /// Data bit HS driver pull up pre-emphasis.
        DSI_PAD_PREEMP_PU OFFSET(0) NUMBITS(2) []
    ],

    /// Bitfields of the `DSI_PAD_CONTROL_4_0` register.
    pub DSI_PAD_CONTROL_4_0 [
        /// Enables BIAS and power regulators on for HS mode.
        DSI_PAD_HS_BSO_CLK OFFSET(28) NUMBITS(1) [],

        /// Enables BIAS and power regulators on for HS mode.
        DSI_PAD_HS_BSO OFFSET(20) NUMBITS(4) [],

        /// Enables BIAS and power regulators on for LP mode.
        DSI_PAD_LP_BSO_CLK OFFSET(16) NUMBITS(1) [],

        /// Enables BIAS and power regulators on for LP mode.
        DSI_PAD_LP_BSO OFFSET(8) NUMBITS(4) [],

        /// Increase bandwidth of output driver.
        DSI_PAD_TXBW_EN OFFSET(4) NUMBITS(1) [],

        /// Reverse clock polarity.
        DSI_PAD_REV_CLK OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `DSI_DSI_GANGED_MODE_CONTROL_0` register.
    pub DSI_DSI_GANGED_MODE_CONTROL_0 [
        /// Enables dummy pixels on one side of the active pixel stream.
        DUMMY_PIX_LEFT_RIGHT_SIDE OFFSET(1) NUMBITS(2) [
            Left = 1,
            Right = 2
        ],

        /// Whether ganged mode transactions should be enabled.
        DSI_GANGED_MODE_EN OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `DSI_DSI_GANGED_MODE_START_0` register.
    pub DSI_DSI_GANGED_MODE_START_0 [
        /// Start pointer for indicating the start of partial active valid pixel
        /// data to be latched from the valid pixels of the display controller in
        /// Left-Right/Even-Odd ganged mode.
        DSI_GANGED_START_POINTER OFFSET(0) NUMBITS(13) []
    ],

    /// Bitfields of the `DSI_DSI_GANGED_MODE_SIZE_0` register.
    pub DSI_DSI_GANGED_MODE_SIZE_0 [
        /// Host RAW DATA byte count specifies the total number of bytes to send
        /// when raw data control mode is enabled.
        DSI_RAW_DATA_BYTE_COUNT OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `DSI_DSI_RAW_DATA_BYTE_COUNT_0` register.
    pub DSI_DSI_RAW_DATA_BYTE_COUNT_0 [
        /// Host RAW DATA byte count specifies the total number of bytes to send
        /// when raw data control mode is enabled.
        DSI_RAW_DATA_BYTE_COUNT OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `DSI_DSI_ULTRA_LOW_POWER_CONTROL_0` register.
    pub DSI_DSI_ULTRA_LOW_POWER_CONTROL_0 [
        /// Ultra Low Power.
        DSI_ULTRA_LOW_POWER_DATA_LANE3 OFFSET(8) NUMBITS(2) [
            Normal = 0,
            EnterUlpm = 1,
            ExitUlpm = 2
        ],

        /// Ultra Low Power.
        DSI_ULTRA_LOW_POWER_DATA_LANE2 OFFSET(6) NUMBITS(2) [
            Normal = 0,
            EnterUlpm = 1,
            ExitUlpm = 2
        ],

        /// Ultra Low Power.
        DSI_ULTRA_LOW_POWER_DATA_LANE1 OFFSET(4) NUMBITS(2) [
            Normal = 0,
            EnterUlpm = 1,
            ExitUlpm = 2
        ],

        /// Ultra Low Power.
        DSI_ULTRA_LOW_POWER_DATA_LANE0 OFFSET(2) NUMBITS(2) [
            Normal = 0,
            EnterUlpm = 1,
            ExitUlpm = 2
        ],

        /// Ultra Low Power.
        DSI_ULTRA_LOW_POWER_CLK_LANE OFFSET(0) NUMBITS(2) [
            Normal = 0,
            EnterUlpm = 1,
            ExitUlpm = 2
        ]
    ],

    /// Bitfields of the `DSI_DUMMY_PIX_CNT_0` register.
    pub DSI_DUMMY_PIX_CNT_0 [
        /// Number of dummy pixels padded to the right of the active pixel stream.
        RIGHT_DUMMY_PIX_CNT OFFSET(16) NUMBITS(8) [],

        /// Number of dummy pixels padded to the left of the active pixel stream.
        LEFT_DUMMY_PIX_CNT OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `DSI_DSI_DSC_CONTROL_0` register.
    pub DSI_DSI_DSC_CONTROL_0 [
        /// Number of compressed image packets per row between two sync events.
        /// Multiple packets option is for Video mode only. Always program ONE
        /// packet for Command mode.
        NUM_COMPRESS_PKTS_PER_ROW OFFSET(16) NUMBITS(2) [],

        /// Compression bit rate. Only 8/12/16 bpp compression rates are supported.
        COMPRESS_RATE OFFSET(2) NUMBITS(10) [
            Bpp8 = 128,
            Bpp12 = 192,
            Bpp16 = 256
        ],

        /// Enables compressed bitstream transport mode.
        COMPRESS_MODE_EN OFFSET(0) NUMBITS(1) []
    ]
}

register_structs! {
    /// Representation of the MIPI-DSI registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x000 => pub DSI_INCR_SYNCPT_0: ReadWrite<u32, DSI_INCR_SYNCPT_0::Register>),
        (0x004 => pub DSI_INCR_SYNCPT_CNTRL_0: ReadWrite<u32, DSI_INCR_SYNCPT_CNTRL_0::Register>),
        (0x008 => pub DSI_INCR_SYNCPT_ERROR_0: ReadWrite<u32, DSI_INCR_SYNCPT_ERROR_0::Register>),
        (0x00C => _reserved0),
        (0x020 => pub DSI_CTXSW_0: ReadWrite<u32, DSI_CTXSW_0::Register>),
        (0x024 => pub DSI_DSI_RD_DATA_0: ReadOnly<u32>),
        (0x028 => pub DSI_DSI_WR_DATA_0: ReadWrite<u32>),
        (0x02C => pub DSI_DSI_POWER_CONTROL_0: ReadWrite<u32, DSI_DSI_POWER_CONTROL_0::Register>),
        (0x030 => pub DSI_INT_ENABLE_0: ReadWrite<u32, DSI_INT_ENABLE_0::Register>),
        (0x034 => pub DSI_INT_STATUS_0: ReadWrite<u32, DSI_INT_STATUS_0::Register>),
        (0x038 => pub DSI_INT_MASK_0: ReadWrite<u32, DSI_INT_MASK_0::Register>),
        (0x03C => pub DSI_HOST_DSI_CONTROL_0: ReadWrite<u32, DSI_HOST_DSI_CONTROL_0::Register>),
        (0x040 => pub DSI_DSI_CONTROL_0: ReadWrite<u32, DSI_DSI_CONTROL_0::Register>),
        (0x044 => pub DSI_DSI_SOL_DELAY_0: ReadWrite<u32, DSI_DSI_SOL_DELAY_0::Register>),
        (0x048 => pub DSI_DSI_MAX_THRESHOLD_0: ReadWrite<u32, DSI_DSI_MAX_THRESHOLD_0::Register>),
        (0x04C => pub DSI_DSI_TRIGGER_0: ReadWrite<u32, DSI_DSI_TRIGGER_0::Register>),
        (0x050 => pub DSI_DSI_TX_CRC_0: ReadOnly<u32>),
        (0x054 => pub DSI_DSI_STATUS_0: ReadOnly<u32, DSI_DSI_STATUS_0::Register>),
        (0x058 => _reserved1),
        (0x068 => pub DSI_DSI_INIT_SEQ_CONTROL_0: ReadWrite<u32, DSI_DSI_INIT_SEQ_CONTROL_0::Register>),
        (0x06C => pub DSI_DSI_INIT_SEQ_DATA_0_0: ReadWrite<u32>),
        (0x070 => pub DSI_DSI_INIT_SEQ_DATA_1_0: ReadWrite<u32>),
        (0x074 => pub DSI_DSI_INIT_SEQ_DATA_2_0: ReadWrite<u32>),
        (0x078 => pub DSI_DSI_INIT_SEQ_DATA_3_0: ReadWrite<u32>),
        (0x07C => pub DSI_DSI_INIT_SEQ_DATA_4_0: ReadWrite<u32>),
        (0x080 => pub DSI_DSI_INIT_SEQ_DATA_5_0: ReadWrite<u32>),
        (0x084 => pub DSI_DSI_INIT_SEQ_DATA_6_0: ReadWrite<u32>),
        (0x088 => pub DSI_DSI_INIT_SEQ_DATA_7_0: ReadWrite<u32>),
        (0x08C => pub DSI_DSI_PKT_SEQ_0_LO_0: ReadWrite<u32, DSI_DSI_PKT_SEQ_0_LO_0::Register>),
        (0x090 => pub DSI_DSI_PKT_SEQ_0_HI_0: ReadWrite<u32, DSI_DSI_PKT_SEQ_0_HI_0::Register>),
        (0x094 => pub DSI_DSI_PKT_SEQ_1_LO_0: ReadWrite<u32, DSI_DSI_PKT_SEQ_1_LO_0::Register>),
        (0x098 => pub DSI_DSI_PKT_SEQ_1_HI_0: ReadWrite<u32, DSI_DSI_PKT_SEQ_1_HI_0::Register>),
        (0x09C => pub DSI_DSI_PKT_SEQ_2_LO_0: ReadWrite<u32, DSI_DSI_PKT_SEQ_2_LO_0::Register>),
        (0x0A0 => pub DSI_DSI_PKT_SEQ_2_HI_0: ReadWrite<u32, DSI_DSI_PKT_SEQ_2_HI_0::Register>),
        (0x0A4 => pub DSI_DSI_PKT_SEQ_3_LO_0: ReadWrite<u32, DSI_DSI_PKT_SEQ_3_LO_0::Register>),
        (0x0A8 => pub DSI_DSI_PKT_SEQ_3_HI_0: ReadWrite<u32, DSI_DSI_PKT_SEQ_3_HI_0::Register>),
        (0x0AC => pub DSI_DSI_PKT_SEQ_4_LO_0: ReadWrite<u32, DSI_DSI_PKT_SEQ_4_LO_0::Register>),
        (0x0B0 => pub DSI_DSI_PKT_SEQ_4_HI_0: ReadWrite<u32, DSI_DSI_PKT_SEQ_4_HI_0::Register>),
        (0x0B4 => pub DSI_DSI_PKT_SEQ_5_LO_0: ReadWrite<u32, DSI_DSI_PKT_SEQ_5_LO_0::Register>),
        (0x0B8 => pub DSI_DSI_PKT_SEQ_5_HI_0: ReadWrite<u32, DSI_DSI_PKT_SEQ_5_HI_0::Register>),
        (0x0BC => _reserved2),
        (0x0CC => pub DSI_DSI_DCS_CMDS_0: ReadWrite<u32, DSI_DSI_DCS_CMDS_0::Register>),
        (0x0D0 => pub DSI_DSI_PKT_LEN_0_1_0: ReadWrite<u32, DSI_DSI_PKT_LEN_0_1_0::Register>),
        (0x0D4 => pub DSI_DSI_PKT_LEN_2_3_0: ReadWrite<u32, DSI_DSI_PKT_LEN_2_3_0::Register>),
        (0x0D8 => pub DSI_DSI_PKT_LEN_4_5_0: ReadWrite<u32, DSI_DSI_PKT_LEN_4_5_0::Register>),
        (0x0DC => pub DSI_DSI_PKT_LEN_6_7_0: ReadWrite<u32, DSI_DSI_PKT_LEN_6_7_0::Register>),
        (0x0E0 => _reserved3),
        (0x0F0 => pub DSI_DSI_PHY_TIMING_0_0: ReadWrite<u32, DSI_DSI_PHY_TIMING_0_0::Register>),
        (0x0F4 => pub DSI_DSI_PHY_TIMING_1_0: ReadWrite<u32, DSI_DSI_PHY_TIMING_1_0::Register>),
        (0x0F8 => pub DSI_DSI_PHY_TIMING_2_0: ReadWrite<u32, DSI_DSI_PHY_TIMING_2_0::Register>),
        (0x0FC => pub DSI_DSI_BTA_TIMING_0: ReadWrite<u32, DSI_DSI_BTA_TIMING_0::Register>),
        (0x100 => _reserved4),
        (0x110 => pub DSI_DSI_TIMEOUT_0_0: ReadWrite<u32, DSI_DSI_TIMEOUT_0_0::Register>),
        (0x114 => pub DSI_DSI_TIMEOUT_1_0: ReadWrite<u32, DSI_DSI_TIMEOUT_1_0::Register>),
        (0x118 => pub DSI_DSI_TO_TALLY_0: ReadWrite<u32, DSI_DSI_TO_TALLY_0::Register>),
        (0x11C => _reserved5),
        (0x12C => pub DSI_PAD_CONTROL_0: ReadWrite<u32, DSI_PAD_CONTROL_0::Register>),
        (0x130 => pub DSI_PAD_CONTROL_CD_0: ReadWrite<u32, DSI_PAD_CONTROL_CD_0::Register>),
        (0x134 => pub DSI_PAD_CD_STATUS_0: ReadWrite<u32, DSI_PAD_CD_STATUS_0::Register>),
        (0x138 => pub DSI_DSI_VID_MODE_CONTROL_0: ReadWrite<u32, DSI_DSI_VID_MODE_CONTROL_0::Register>),
        (0x13C => pub DSI_PAD_CONTROL_1_0: ReadWrite<u32, DSI_PAD_CONTROL_1_0::Register>),
        (0x140 => pub DSI_PAD_CONTROL_2_0: ReadWrite<u32, DSI_PAD_CONTROL_2_0::Register>),
        (0x144 => pub DSI_PAD_CONTROL_3_0: ReadWrite<u32, DSI_PAD_CONTROL_3_0::Register>),
        (0x148 => pub DSI_PAD_CONTROL_4_0: ReadWrite<u32, DSI_PAD_CONTROL_4_0::Register>),
        (0x14C => pub DSI_DSI_GANGED_MODE_CONTROL_0: ReadWrite<u32, DSI_DSI_GANGED_MODE_CONTROL_0::Register>),
        (0x150 => pub DSI_DSI_GANGED_MODE_START_0: ReadWrite<u32, DSI_DSI_GANGED_MODE_START_0::Register>),
        (0x154 => pub DSI_DSI_GANGED_MODE_SIZE_0: ReadWrite<u32, DSI_DSI_GANGED_MODE_SIZE_0::Register>),
        (0x158 => pub DSI_DSI_RAW_DATA_BYTE_COUNT_0: ReadWrite<u32, DSI_DSI_RAW_DATA_BYTE_COUNT_0::Register>),
        (0x15C => pub DSI_DSI_ULTRA_LOW_POWER_CONTROL_0: ReadWrite<u32, DSI_DSI_ULTRA_LOW_POWER_CONTROL_0::Register>),
        (0x160 => pub DSI_DSI_INIT_SEQ_DATA_8_0: ReadWrite<u32>),
        (0x164 => pub DSI_DSI_INIT_SEQ_DATA_9_0: ReadWrite<u32>),
        (0x168 => pub DSI_DSI_INIT_SEQ_DATA_10_0: ReadWrite<u32>),
        (0x16C => pub DSI_DSI_INIT_SEQ_DATA_11_0: ReadWrite<u32>),
        (0x170 => pub DSI_DSI_INIT_SEQ_DATA_12_0: ReadWrite<u32>),
        (0x174 => pub DSI_DSI_INIT_SEQ_DATA_13_0: ReadWrite<u32>),
        (0x178 => pub DSI_DSI_INIT_SEQ_DATA_14_0: ReadWrite<u32>),
        (0x17C => pub DSI_DSI_INIT_SEQ_DATA_15_0: ReadWrite<u32>),
        (0x180 => pub DSI_DUMMY_PIX_CNT_0: ReadWrite<u32>),
        (0x184 => pub DSI_DSI_DSC_CONTROL_0: ReadWrite<u32>),
        (0x188 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x188]);
