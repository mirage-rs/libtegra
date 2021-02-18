use register::{mmio::*, register_bitfields, register_structs};

use crate::memory_map::{TSEC, TSEC2};

/// A pointer to the TSEC-A register block that can be accessed by dereferencing it.
pub const TSEC_A_REGISTERS: *const Registers = TSEC as *const Registers;

/// A pointer to the TSEC-B register block that can be accessed by dereferencing it.
pub const TSEC_B_REGISTERS: *const Registers = TSEC2 as *const Registers;

register_bitfields! {
    u32,

    /// Bitfields of the `TSEC_THI_INCR_SYNCPT` register.
    pub TSEC_THI_INCR_SYNCPT [
        COND OFFSET(10) NUMBITS(8) [],

        INDX OFFSET(0) NUMBITS(10) []
    ],

    /// Bitfields of the `TSEC_THI_INCR_SYNCPT_CTRL` register.
    pub TSEC_THI_INCR_SYNCPT_CTRL [
        NO_STALL_4 OFFSET(25) NUMBITS(1) [],

        SOFT_RESET_4 OFFSET(24) NUMBITS(1) [],

        NO_STALL_3 OFFSET(23) NUMBITS(1) [],

        SOFT_RESET_3 OFFSET(22) NUMBITS(1) [],

        NO_STALL_2 OFFSET(21) NUMBITS(1) [],

        SOFT_RESET_2 OFFSET(20) NUMBITS(1) [],

        NO_STALL_1 OFFSET(19) NUMBITS(1) [],

        SOFT_RESET_1 OFFSET(18) NUMBITS(1) [],

        NO_STALL_0 OFFSET(17) NUMBITS(1) [],

        SOFT_RESET_0 OFFSET(16) NUMBITS(1) [],

        NO_STALL OFFSET(8) NUMBITS(1) [],

        SOFT_RESET OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_THI_INCR_SYNCPT_ERR` register.
    pub TSEC_THI_INCR_SYNCPT_ERR [
        COND_STS_ENGINE_IDLE OFFSET(4) NUMBITS(1) [],

        COND_STS_REG_WR_SAFE OFFSET(3) NUMBITS(1) [],

        COND_STS_RD_DONE OFFSET(2) NUMBITS(1) [],

        COND_STS_OPDONE OFFSET(1) NUMBITS(1) [],

        COND_STS_IMM OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_THI_CTXSW_INCR_SYNCPT` register.
    pub TSEC_THI_CTXSW_INCR_SYNCPT [
        INDX OFFSET(0) NUMBITS(10) []
    ],

    /// Bitfields of the `TSEC_THI_CTXSW` register.
    pub TSEC_THI_CTXSW [
        CURR_CHANNEL OFFSET(11) NUMBITS(10) [],

        AUTO_ACK OFFSET(10) NUMBITS(1) [],

        CURR_CLASS OFFSET(0) NUMBITS(10) []
    ],

    /// Bitfields of the `TSEC_THI_CTXSW_NEXT` register.
    pub TSEC_THI_CTXSW_NEXT [
        NEXT_CHANNEL OFFSET(10) NUMBITS(10) [],

        NEXT_CLASS OFFSET(0) NUMBITS(10) []
    ],

    /// Bitfields of the `TSEC_THI_CONT_SYNCPT_EOF` register.
    pub TSEC_THI_CONT_SYNCPT_EOF [
        COND OFFSET(10) NUMBITS(1) [],

        INDEX OFFSET(0) NUMBITS(10) []
    ],

    /// Bitfields of the `TSEC_THI_CONT_SYNCPT_L1` register.
    pub TSEC_THI_CONT_SYNCPT_L1 [
        COND OFFSET(10) NUMBITS(1) [],

        INDEX OFFSET(0) NUMBITS(10) []
    ],

    /// Bitfields of the `TSEC_THI_STREAMIDX` register.
    pub TSEC_THI_STREAMID [
        ID OFFSET(0) NUMBITS(7) []
    ],

    /// Bitfields of the `TSEC_THI_THI_SEC` register.
    pub TSEC_THI_THI_SEC [
        CH_LOCK OFFSET(8) NUMBITS(1) [],

        TZ_AUTH OFFSET(4) NUMBITS(1) [],

        TZ_LOCK OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_THI_METHOD0` register.
    pub TSEC_THI_METHOD0 [
        /// Encodes a method's ID that is sent to the TSEC over Host1X.
        METHOD OFFSET(0) NUMBITS(12) [
            Nop = 0x100,
            PmTrigger = 0x140,
            SetApplicationId = 0x200,
            SetWatchdogTimer = 0x204,
            SemaphoreA = 0x240,
            SemaphoreB = 0x244,
            SemaphoreC = 0x248,
            CtxSaveArea = 0x24C,
            CtxSwitch = 0x250,
            Execute = 0x300,
            SemaphoreD = 0x304,
            HdcpInit = 0x500,
            HdcpCreateSession = 0x504,
            HdcpVerifyCertRx = 0x508,
            HdcpGenerateEkm = 0x50C,
            HdcpRevocationCheck = 0x510,
            HdcpVerifyHprime = 0x514,
            HdcpEncryptPairingInfo = 0x518,
            HdcpDecryptPairingInfo = 0x51C,
            HdcpUpdateSession = 0x520,
            HdcpGenerateLcInit = 0524,
            HdcpVerifyLprime = 0x528,
            HdcpGenerateSkeInit = 0x52C,
            HdcpVerifyVprime = 0x530,
            HdcpEncryptionRunCtrl = 0x534,
            HdcpSessionCtrl = 0x538,
            HdcpComputeSprime = 0x53C,
            HdcpGetCertRx = 0x540,
            HdcpExchangeInfo = 0x544,
            HdcpDecryptKm = 0x548,
            HdcpGetHprime = 0x54C,
            HdcpGenerateEkhKm = 0x550,
            HdcpVerifyRttChallenge = 0x554,
            HdcpGetLprime = 0x558,
            HdcpDecryptKs = 0x55C,
            HdcpDecrypt = 0x560,
            HdcpGetRrx = 0x564,
            HdcpDecryptReencrypt = 0x568,
            HdcpDecryptStoredKm = 0x574,
            HdcpGetCurrentResolution = 0x578,
            HdcpGetCurrentVersion = 0x57C,
            HdcpValidateSrm = 0x700,
            HdcpValidateStream = 0x704,
            HdcpTestSecureStatus = 0x708,
            HdcpSetDcpKpub = 0x70C,
            HdcpSetRxKpub = 0x710,
            HdcpSetCertRx = 0x714,
            HdcpSetScratchBuffer = 0x718,
            HdcpSetSrm = 0x71C,
            HdcpSetReceiverIdList = 0x720,
            HdcpSetSprime = 0x724,
            HdcpSetEncInputBuffer = 0x728,
            HdcpSetEncOuptutBuffer = 0x72C,
            HdcpGetRttChallenge = 0x730,
            HdcpStreamManage = 0x734,
            HdcpReadCaps = 0x738,
            HdcpEncrypt = 0x73C,
            HdcpGetCurrentNonce = 0x740,
            PmTriggerEnd = 0x1114
        ]
    ],

    /// Bitfields of the `TSEC_THI_CONTEXT_SWITCH` register.
    pub TSEC_THI_CONTEXT_SWITCH [
        TARGET OFFSET(30) NUMBITS(2) [],

        PTR OFFSET(0) NUMBITS(28) []
    ],

    /// Bitfields of the `TSEC_THI_INT_STATUS` register.
    pub TSEC_THI_INT_STATUS [
        FALCON_INT OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_THI_INT_MASK` register.
    pub TSEC_THI_INT_MASK [
        FALCON_INT OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_THI_CONFIG0` register.
    pub TSEC_THI_CONFIG0 [
        IDLE_SYNCPT_INC_ENG OFFSET(4) NUMBITS(1) [],

        RETURN_SYNCPT_ON_ERR OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_THI_DBG_MISC` register.
    pub TSEC_THI_DBG_MISC [
        THI_IDLE_EN OFFSET(3) NUMBITS(1) [],

        THI_SYNCPT_PENDING_STATUS OFFSET(2) NUMBITS(1) [],

        THI_IDLE_STATUS OFFSET(1) NUMBITS(1) [],

        CLIENT_IDLE_STATUS OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_THI_SLCG_OVERRIDE_HIGH_A` register.
    pub TSEC_THI_SLCG_OVERRIDE_HIGH_A [
        REG OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `TSEC_FALCON_IRQXXXX` registers.
    pub TSEC_FALCON_IRQS [
        DMA OFFSET(16) NUMBITS(1) [],

        EXT OFFSET(8) NUMBITS(8) [],

        SWGEN1 OFFSET(7) NUMBITS(1) [],

        SWGEN0 OFFSET(6) NUMBITS(1) [],

        EXTERR OFFSET(5) NUMBITS(1) [],

        HALT OFFSET(4) NUMBITS(1) [],

        CTXSW OFFSET(3) NUMBITS(1) [],

        MTHD OFFSET(2) NUMBITS(1) [],

        WDTMR OFFSET(1) NUMBITS(1) [],

        GPTMR OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_IRQDEST` register.
    pub TSEC_FALCON_IRQDEST [
        TARGET_EXT OFFSET(24) NUMBITS(8) [],

        TARGET_SWGEN1 OFFSET(23) NUMBITS(1) [],

        TARGET_SWGEN0 OFFSET(22) NUMBITS(1) [],

        TARGET_EXTERR OFFSET(21) NUMBITS(1) [],

        TARGET_HALT OFFSET(20) NUMBITS(1) [],

        TARGET_CTXSW OFFSET(19) NUMBITS(1) [],

        TARGET_MTHD OFFSET(18) NUMBITS(1) [],

        TARGET_WDTMR OFFSET(17) NUMBITS(1) [],

        TARGET_GPTMR OFFSET(16) NUMBITS(1) [],

        HOST_EXT OFFSET(8) NUMBITS(8) [],

        HOST_SWGEN1 OFFSET(7) NUMBITS(1) [],

        HOST_SWGEN0 OFFSET(6) NUMBITS(1) [],

        HOST_EXTERR OFFSET(5) NUMBITS(1) [],

        HOST_HALT OFFSET(4) NUMBITS(1) [],

        HOST_CTXSW OFFSET(3) NUMBITS(1) [],

        HOST_MTHD OFFSET(2) NUMBITS(1) [],

        HOST_WDTMR OFFSET(1) NUMBITS(1) [],

        HOST_GPTMR OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_WDTMRCTL` register.
    pub TSEC_FALCON_WDTMRCTL [
        WDTMREN OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_IRQDEST2` register.
    pub TSEC_FALCON_IRQDEST2 [
        TARGET_DMA OFFSET(16) NUMBITS(1) [],

        HOST_DMA OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_ITFEN` register.
    pub TSEC_FALCON_ITFEN [
        MTHDEN OFFSET(1) NUMBITS(1) [],

        CTXEN OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_IDLESTATE` register.
    pub TSEC_FALCON_IDLESTATE [
        EXT_BUSY OFFSET(1) NUMBITS(15) [],

        FALCON_BUSY OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_CURCTX` register.
    pub TSEC_FALCON_CURCTX [
        CTXVLD OFFSET(30) NUMBITS(1) [],

        CTXTGT OFFSET(28) NUMBITS(2) [],

        CTXPTR OFFSET(0) NUMBITS(28) []
    ],

    /// Bitfields of the `TSEC_FALCON_NXTCTX` register.
    pub TSEC_FALCON_NXTCTX [
        CTXVLD OFFSET(30) NUMBITS(1) [],

        CTXTGT OFFSET(28) NUMBITS(2) [],

        CTXPTR OFFSET(0) NUMBITS(28) []
    ],

    /// Bitfields of the `TSEC_FALCON_CTXACK` register.
    pub TSEC_FALCON_CTXACK [
        REST_ACK OFFSET(1) NUMBITS(1) [],

        SAVE_ACK OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_FHSTATE` register.
    pub TSEC_FALCON_FHSTATE [
        STALL_REQ OFFSET(17) NUMBITS(1) [],

        ENGINE_FAULTED OFFSET(16) NUMBITS(1) [],

        EXT_HALTED OFFSET(1) NUMBITS(15) [],

        FALCON_HALTED OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_PRIVSTATE` register.
    pub TSEC_FALCON_PRIVSTATE [
        PRIV OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_MTHDID` register.
    pub TSEC_FALCON_MTHDID [
        WPEND OFFSET(16) NUMBITS(1) [],

        PRIV OFFSET(15) NUMBITS(1) [],

        SUBCH OFFSET(12) NUMBITS(3) [],

        ID OFFSET(0) NUMBITS(12) []
    ],

    /// Bitfields of the `TSEC_FALCON_MTHDCOUNT` register.
    pub TSEC_FALCON_MTHDCOUNT [
        COUNT OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `TSEC_FALCON_MTHDPOP` register.
    pub TSEC_FALCON_MTHDPOP [
        POP OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_MTHDRAMSZ` register.
    pub TSEC_FALCON_MTHDRAMSZ [
        RAMSZ OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `TSEC_FALCON_SFTRESET` register.
    pub TSEC_FALCON_SFTRESET [
        EXT OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_SOFT_PM` register.
    pub TSEC_FALCON_SOFT_PM [
        TRIGGER_START OFFSET(17) NUMBITS(1) [],

        TRIGGER_END OFFSET(16) NUMBITS(1) [],

        PROBE OFFSET(0) NUMBITS(6) []
    ],

    /// Bitfields of the `TSEC_FALCON_SOFT_MODE` register.
    pub TSEC_FALCON_SOFT_MODE [
        PROBE OFFSET(0) NUMBITS(6) []
    ],

    /// Bitfields of the `TSEC_FALCON_DEBUG1` register.
    pub TSEC_FALCON_DEBUG1 [
        TRACE_FORMAT OFFSET(17) NUMBITS(1) [],

        CTXSW_MODE OFFSET(16) NUMBITS(1) [],

        MTHD_DRAIN_TIME OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `TSEC_FALCON_IBRKPTX` registers.
    pub TSEC_FALCON_IBRKPT [
        EN OFFSET(31) NUMBITS(1) [],

        SKIP OFFSET(30) NUMBITS(1) [],

        SUPPRESS OFFSET(29) NUMBITS(1) [],

        PC OFFSET(0) NUMBITS(24) []
    ],

    /// Bitfields of the `TSEC_FALCON_CGCTL` register.
    pub TSEC_FALCON_CGCTL [
        CG_OVERRIDE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_ENGCTL` register.
    pub TSEC_FALCON_ENGCTL [
        STALLACK OFFSET(9) NUMBITS(1) [],

        STALLREQ OFFSET(8) NUMBITS(1) [],

        SWITCH_CONTEXT OFFSET(3) NUMBITS(1) [],

        CLR_STALLREQ OFFSET(2) NUMBITS(1) [],

        SET_STALLREQ OFFSET(1) NUMBITS(1) [],

        INV_CONTEXT OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_PMM` register.
    pub TSEC_FALCON_PMM [
        TFBIF_STALL2_SEL OFFSET(28) NUMBITS(4) [
            RDatQFull = 0
        ],

        TFBIF_STALL1_SEL OFFSET(24) NUMBITS(4) [
            RDatQFull = 0
        ],

        TFBIF_STALL0_SEL OFFSET(20) NUMBITS(4) [
            RDatQFull = 0,
            RAckQFull = 1,
            WReqQFull = 2,
            WDatQFull = 3,
            WAckQFull = 4,
            MReqQFull = 5,
            RReqPend = 6,
            WReqPend = 7,
            RDatQFullSc = 8,
            RAckQFullSc = 9,
            WReqQFullSc = 10,
            WDatQFullSc = 11,
            WAckQFullSc = 12,
            MReqQFullSc = 13,
            RReqPendSc = 14,
            WReqPendSc = 15
        ],

        TFBIF_DSTAT_SEL OFFSET(17) NUMBITS(3) [
            OneKTransfer = 0,
            RReq = 1,
            WReq = 2,
            TWReq = 3,
            OneKTransferSc = 4,
            RReqSc = 5,
            WReqSc = 6,
            TWReqSc = 7
        ],

        FALCON_SOFTPM1_SEL OFFSET(12) NUMBITS(4) [],

        FALCON_SOFTPM0_SEL OFFSET(8) NUMBITS(4) [
            Zero = 0,
            One = 1,
            Two = 2,
            Three = 3,
            Four = 4,
            Five = 5,
            ZeroSc = 6,
            OneSc = 7,
            TwoSc = 8,
            ThreeSc = 9,
            FourSc = 10,
            FiveSc = 11
        ],

        FALCON_IDLE_SEL OFFSET(5) NUMBITS(3) [
            Waiting = 0,
            EngIdle = 1,
            MThdFull = 2,
            WaitingSc = 3,
            EngIdleSc = 4,
            MThdFullSc = 5
        ],

        FALCON_STALL_SEL OFFSET(0) NUMBITS(5) [
            Any = 0,
            Code = 1,
            DmaQ = 2,
            DmFence = 3,
            DmWait = 4,
            ImWait = 5,
            Ipnd = 6,
            Ldstq = 7,
            Sb = 8,
            AnySc = 9,
            CodeSc = 10,
            DmaQSc = 11,
            DmFenceSc = 12,
            DmWaitSc = 13,
            ImWaitSc = 14,
            IpndSc = 15,
            LdstqSc = 16,
            SbSc = 17
        ]
    ],

    /// Bitfields of the `TSEC_FALCON_ADDR` register.
    pub TSEC_FALCON_ADDR [
        MSB OFFSET(6) NUMBITS(6) [],

        LSB OFFSET(0) NUMBITS(6) []
    ],

    /// Bitfields of the `TSEC_FALCON_EXCI` register.
    pub TSEC_FALCON_EXCI [
        EXCAUSE OFFSET(20) NUMBITS(4) [
            Trap0 = 0,
            Trap1 = 1,
            Trap2 = 2,
            Trap3 = 3,
            IllIns = 8,
            InvIns = 9,
            MissIns = 10,
            DhitIns = 11,
            BrkptIns = 15
        ],

        EXPC OFFSET(0) NUMBITS(20) []
    ],

    /// Bitfields of the `TSEC_FALCON_SVEC_SPR` register.
    pub TSEC_FALCON_SVEC_SPR [
        SIGPASS OFFSET(18) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_CPUCTL` register.
    pub TSEC_FALCON_CPUCTL [
        ALIAS_EN OFFSET(6) NUMBITS(1) [],

        STOPPED OFFSET(5) NUMBITS(1) [],

        HALTED OFFSET(4) NUMBITS(1) [],

        HRESET OFFSET(3) NUMBITS(1) [],

        SRESET OFFSET(2) NUMBITS(1) [],

        STARTCPU OFFSET(1) NUMBITS(1) [],

        IINVAL OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_HWCFG` register.
    pub TSEC_FALCON_HWCFG [
        DMAQUEUE_DEPTH OFFSET(27) NUMBITS(4) [],

        METHODFIFO_DEPTH OFFSET(18) NUMBITS(9) [],

        DMEM_SIZE OFFSET(9) NUMBITS(9) [],

        IMEM_SIZE OFFSET(0) NUMBITS(9) []
    ],

    /// Bitfields of the `TSEC_FALCON_DMACTL` register.
    pub TSEC_FALCON_DMACTL [
        SECURE_STAT OFFSET(7) NUMBITS(1) [],

        DMAQ_NUM OFFSET(3) NUMBITS(4) [],

        IMEM_SCRUBBING OFFSET(2) NUMBITS(1) [],

        DMEM_SCRUBBING OFFSET(1) NUMBITS(1) [],

        REQUIRE_CTX OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_DMATRFMOFFS` register.
    pub TSEC_FALCON_DMATRFMOFFS [
        OFFS OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `TSEC_FALCON_DMATRFCMD` register.
    pub TSEC_FALCON_DMATRFCMD [
        CTXDMA OFFSET(12) NUMBITS(3) [],

        SIZE OFFSET(8) NUMBITS(3) [],

        WRITE OFFSET(5) NUMBITS(1) [],

        IMEM OFFSET(4) NUMBITS(1) [],

        SEC OFFSET(2) NUMBITS(2) [],

        IDLE OFFSET(1) NUMBITS(1) [],

        FULL OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_DMAPOLL_FB` register.
    pub TSEC_FALCON_DMAPOLL [
        RCOUNT OFFSET(24) NUMBITS(8) [],

        WCOUNT OFFSET(16) NUMBITS(8) [],

        CFG_W_FENCE OFFSET(5) NUMBITS(1) [],

        CFG_R_FENCE OFFSET(4) NUMBITS(1) [],

        DMA_ACTIVE OFFSET(1) NUMBITS(1) [],

        FENCE_ACTIVE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_HWCFG1` register.
    pub TSEC_FALCON_HWCFG1 [
        IMEM_AUTOFILL OFFSET(31) NUMBITS(1) [],

        DMEM_APERTURES OFFSET(30) NUMBITS(1) [],

        PRIV_DIRECT OFFSET(29) NUMBITS(1) [],

        CSB_SIZE_16M OFFSET(28) NUMBITS(1) [],

        DBG_PRIV_BUS OFFSET(27) NUMBITS(1) [],

        TAG_WIDTH OFFSET(16) NUMBITS(5) [],

        DMEM_PORTS OFFSET(12) NUMBITS(4) [],

        IMEM_PORTS OFFSET(8) NUMBITS(4) [],

        CORE_REV_SUBVERSION OFFSET(6) NUMBITS(2) [],

        SECURITY_MODEL OFFSET(4) NUMBITS(2) [],

        CORE_REV OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `TSEC_FALCON_CPUCTL_ALIAS` register.
    pub TSEC_FALCON_CPUCTL_ALIAS [
        STARTCPU OFFSET(1) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_STACKCFG` register.
    pub TSEC_FALCON_STACKCFG [
        SPEXC OFFSET(31) NUMBITS(1) [],

        BOTTOM OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `TSEC_FALCON_IMCTL` register.
    pub TSEC_FALCON_IMCTL [
        CMD OFFSET(24) NUMBITS(3) [
            Nop = 0,
            ImInv = 1,
            ImBlk = 2,
            ImTag = 3,
            ImTagSetVld = 4
        ],

        ADDR_BLK OFFSET(0) NUMBITS(24) []
    ],

    /// Bitfields of the `TSEC_FALCON_TRACEIDX` register.
    pub TSEC_FALCON_TRACEIDX [
        CNT OFFSET(24) NUMBITS(8) [],

        MAXIDX OFFSET(16) NUMBITS(8) [],

        IDX OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `TSEC_FALCON_TRACEPC` register.
    pub TSEC_FALCON_TRACEPC [
        PC OFFSET(0) NUMBITS(24) []
    ],

    /// Bitfields of the `TSEC_FALCON_IMFILLRNGX` registers.
    pub TSEC_FALCON_IMFILLRNG [
        TAG_HI OFFSET(16) NUMBITS(16) [],

        TAG_LO OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `TSEC_FALCON_IMFILLCTL` register.
    pub TSEC_FALCON_IMFILLCTL [
        NBLOCKS OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `TSEC_FALCON_IMCTL_DEBUG` register.
    pub TSEC_FALCON_IMCTL_DEBUG [
        CMD OFFSET(24) NUMBITS(3) [
            Nop = 0,
            ImBlk = 2,
            ImTag = 3
        ],

        ADDR_BLK OFFSET(0) NUMBITS(24) []
    ],

    /// Bitfields of the `TSEC_FALCON_CMEMBASE` register.
    pub TSEC_FALCON_CMEMBASE [
        VAL OFFSET(18) NUMBITS(14) []
    ],

    /// Bitfields of the `TSEC_FALCON_DMEMAPERT` register.
    pub TSEC_FALCON_DMEMAPERT [
        LDSTQ_NUM OFFSET(17) NUMBITS(3) [],

        ENABLE OFFSET(16) NUMBITS(1) [],

        TIME_UNIT OFFSET(8) NUMBITS(4) [],

        TIME_OUT OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `TSEC_FALCON_EXTERRSTAT` register.
    pub TSEC_FALCON_EXTERRSTAT [
        VALID OFFSET(31) NUMBITS(1) [],

        STAT OFFSET(24) NUMBITS(4) [],

        PC OFFSET(0) NUMBITS(24) []
    ],

    /// Bitfields of the `TSEC_FALCON_CG2` register.
    pub TSEC_FALCON_CG2 [
        SLCG_FBIF OFFSET(17) NUMBITS(1) [],

        SLCG_FALCON_TOP OFFSET(16) NUMBITS(1) [],

        SLCG_FALCON_IRQSTAT OFFSET(15) NUMBITS(1) [],

        SLCG_FALCON_WDTMR OFFSET(14) NUMBITS(1) [],

        SLCG_FALCON_GPTMR OFFSET(13) NUMBITS(1) [],

        SLCG_FALCON_TSYNC OFFSET(12) NUMBITS(1) [],

        SLCG_FALCON_LDST OFFSET(11) NUMBITS(1) [],

        SLCG_FALCON_MUL OFFSET(10) NUMBITS(1) [],

        SLCG_FALCON_RF OFFSET(9) NUMBITS(1) [],

        SLCG_FALCON_PMB OFFSET(8) NUMBITS(1) [],

        SLCG_FALCON_CTXSW OFFSET(7) NUMBITS(1) [],

        SLCG_FALCON_CFG OFFSET(6) NUMBITS(1) [],

        SLCG_FALCON_ICD OFFSET(5) NUMBITS(1) [],

        SLCG_FALCON_DIV OFFSET(4) NUMBITS(1) [],

        SLCG_FALCON_PIPE OFFSET(3) NUMBITS(1) [],

        SLCG_FALCON_GC6_SR_FSM OFFSET(2) NUMBITS(1) [],

        SLCG_FALCON_DMA OFFSET(1) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_IMEMC` register.
    pub TSEC_FALCON_IMEMC [
        SEC_LOCK OFFSET(31) NUMBITS(1) [],

        SEC_WR_VIO OFFSET(30) NUMBITS(1) [],

        SEC_ATOMIC OFFSET(29) NUMBITS(1) [],

        SECURE OFFSET(28) NUMBITS(1) [],

        AINCR OFFSET(25) NUMBITS(1) [],

        AINCW OFFSET(24) NUMBITS(1) [],

        BLK OFFSET(8) NUMBITS(8) [],

        OFFS OFFSET(2) NUMBITS(6) []
    ],

    /// Bitfields of the `TSEC_FALCON_IMEMT` register.
    pub TSEC_FALCON_IMEMT [
        TAG OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `TSEC_FALCON_DMEMC` register.
    pub TSEC_FALCON_DMEMC [
        AINCR OFFSET(25) NUMBITS(1) [],

        AINCW OFFSET(24) NUMBITS(1) [],

        BLK OFFSET(8) NUMBITS(8) [],

        OFFS OFFSET(2) NUMBITS(6) []
    ],

    /// Bitfields of the `TSEC_FALCON_ICD_CMD` register.
    pub TSEC_FALCON_ICD_CMD [
        PARM OFFSET(16) NUMBITS(16) [
            EMaskTrap0 = 0x1,
            EMaskTrap1 = 0x2,
            EMaskTrap2 = 0x4,
            EMaskTrap3 = 0x8,
            EMaskExcUnimp = 0x10,
            EMaskExcIMiss = 0x20,
            EMaskExcIMhit = 0x40,
            EMaskExcIBreak = 0x80,
            EMaskIv0 = 0x100,
            EMaskIv1 = 0x200,
            EMaskIv2 = 0x400,
            EMaskExt0 = 0x800,
            EMaskExt1 = 0x1000,
            EMaskExt2 = 0x2000,
            EMaskExt3 = 0x4000,
            EMaskExt4 = 0x8000
        ],

        RDVLD OFFSET(15) NUMBITS(1) [],

        ERROR OFFSET(14) NUMBITS(1) [],

        IDX OFFSET(8) NUMBITS(5) [
            Reg0 = 0x0,
            Reg1 = 0x1,
            Reg2 = 0x2,
            Reg3 = 0x3,
            Reg4 = 0x4,
            Reg5 = 0x5,
            Reg6 = 0x6,
            Reg7 = 0x7,
            Reg8 = 0x8,
            Reg9 = 0x9,
            Reg10 = 0xA,
            Reg11 = 0xB,
            Reg12 = 0xC,
            Reg13 = 0xD,
            Reg14 = 0xE,
            Reg15 = 0xF,
            Iv0 = 0x10,
            Iv1 = 0x11,
            Undefined = 0x12,
            Ev = 0x13,
            Sp = 0x14,
            Pc = 0x15,
            Imb = 0x16,
            Dmb = 0x17,
            Csw = 0x18,
            Ccr = 0x19,
            Sec = 0x1A,
            Ctx = 0x1B,
            Exci = 0x1C,
            Sec1 = 0x1D,
            Imb1 = 0x1E,
            Dmb1 = 0x1F
        ],

        SZ OFFSET(6) NUMBITS(2) [
            Byte = 0,
            HalfWord = 1,
            Word = 2
        ],

        OPC OFFSET(0) NUMBITS(4) [
            Stop = 0x0,
            Run = 0x1,
            JRun = 0x2,
            RunB = 0x3,
            JRunB = 0x4,
            Step = 0x5,
            JStep = 0x6,
            EMask = 0x7,
            RReg = 0x8,
            WReg = 0x9,
            Rdm = 0xA,
            Wdm = 0xB,
            Rcm = 0xC,
            Wcm = 0xD,
            Rstat = 0xE,
            Sbu = 0xF
        ]
    ],

    /// Bitfields of the `TSEC_FALCON_SCTL` register.
    pub TSEC_FALCON_SCTL [
        TRANSIST_LSMODE OFFSET(14) NUMBITS(1) [],

        HSMODE OFFSET(1) NUMBITS(1) [],

        LSMODE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_SSTAT` register.
    pub TSEC_FALCON_SSTAT [
        MEMPROT_VIO OFFSET(31) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_FALCON_SPROT_X` registers.
    pub TSEC_FALCON_SPROT [
        ACCESS_WRITE OFFSET(4) NUMBITS(4) [],

        ACCESS_READ OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `TSEC_FALCON_DMAINFO_CTL` register.
    pub TSEC_FALCON_DMAINFO_CTL [
        CLR_FBWR OFFSET(1) NUMBITS(1) [],

        CLR_FBRD OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_CTL0` register.
    pub TSEC_SCP_CTL0 [
        EN_CTL OFFSET(20) NUMBITS(1) [],

        EN_SEQ OFFSET(16) NUMBITS(1) [],

        EN_CMD OFFSET(14) NUMBITS(1) [],

        EN_STORE OFFSET(12) NUMBITS(1) [],

        EN_LOAD OFFSET(10) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_CTL1` register.
    pub TSEC_SCP_CTL1 [
        EN_STORE_BYPASS OFFSET(24) NUMBITS(1) [],

        EN_LOAD_BYPASS OFFSET(20) NUMBITS(1) [],

        EN_LOAD_DUMMY_MODE OFFSET(16) NUMBITS(1) [],

        EN_RNG OFFSET(12) NUMBITS(1) [],

        EN_RNG_TEST_MODE OFFSET(11) NUMBITS(1) [],

        CLR_SCP_PIPELINE OFFSET(8) NUMBITS(1) [],

        CLR_SEQ_PIPELINE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_CTL_STAT` register.
    pub TSEC_SCP_CTL_STAT [
        DEBUG_MODE OFFSET(20) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_CTL_LOCK` register.
    pub TSEC_SCP_CTL_LOCK [
        LOCK_SCP OFFSET(4) NUMBITS(1) [],

        EN_LOCKDOWN OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_CFG` register.
    pub TSEC_SCP_CFG [
        TIMEOUT_VAL OFFSET(16) NUMBITS(16) [],

        CARRY_CHAIN_SIZE OFFSET(12) NUMBITS(2) [
            _32Bits = 0,
            _64Bits = 1,
            _96Bits = 2,
            _128Bits = 3
        ],

        FLUSH_CMD_PIPELINE OFFSET(8) NUMBITS(1) [],

        AES_BLOCK_ENDIAN OFFSET(4) NUMBITS(1) [
            Little = 0,
            Big = 1
        ]
    ],

    /// Bitfields of the `TSEC_SCP_CTL_SCP` register.
    pub TSEC_SCP_CTL_SCP [
        CUR_SCP_MASTER OFFSET(1) NUMBITS(1) [
            Falcon = 0,
            External = 1
        ],

        SWAP_SCP_MASTER OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_CTL_PKEY` register.
    pub TSEC_SCP_CTL_PKEY [
        LOADED OFFSET(1) NUMBITS(1) [],

        REQUEST_RELOAD OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_DBG0` register.
    pub TSEC_SCP_DBG0 [
        STORE_IN_HSMODE OFFSET(31) NUMBITS(1) [],

        STORE_CUR_OP_VALID OFFSET(30) NUMBITS(1) [],

        STORE_PIPELINE_SIZE OFFSET(25) NUMBITS(2) [],

        LOAD_IN_HSMODE OFFSET(24) NUMBITS(1) [],

        LOAD_CUR_OP_VALID OFFSET(23) NUMBITS(1) [],

        LOAD_PIPELINE_SIZE OFFSET(19) NUMBITS(4) [],

        SEQ_IN_HSMODE OFFSET(18) NUMBITS(1) [],

        SEQ_CUR_INS_VALID OFFSET(17) NUMBITS(1) [],

        SEQ_CUR_INS_ADDR OFFSET(13) NUMBITS(4) [],

        SEQ_CUR_SEQ_SIZE OFFSET(8) NUMBITS(5) [],

        TARGET OFFSET(5) NUMBITS(2) [
            None = 0,
            Store = 1,
            Load = 2,
            Seq = 3
        ],

        AINC OFFSET(4) NUMBITS(1) [],

        INDEX OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `TSEC_SCP_DBG1` register.
    pub TSEC_SCP_DBG1 [
        SEQ_CUR_INS_OPCODE OFFSET(10) NUMBITS(5) [],

        SEQ_CUR_INS_SEC_OP OFFSET(4) NUMBITS(6) [],

        SEQ_CUR_INS_FIR_OP OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `TSEC_SCP_DBG2` register.
    pub TSEC_SCP_DBG2 [
        ACTIVE_KEYREG_IDX OFFSET(12) NUMBITS(4) [],

        SEQ_INS_NUM_LEFT OFFSET(4) NUMBITS(4) [],

        SEQ_STATE OFFSET(0) NUMBITS(2) [
            Idle = 0,
            Recording = 1
        ]
    ],

    /// Bitfields of the `TSEC_SCP_CMD` register.
    pub TSEC_SCP_CMD [
        CMD_IN_HSMODE OFFSET(31) NUMBITS(1) [],

        CMD_CUR_INS_VALID OFFSET(28) NUMBITS(1) [],

        OPCODE OFFSET(20) NUMBITS(5) [
            Nop = 0x0,
            Mov = 0x1,
            Xsin = 0x2,
            Xsout = 0x3,
            Rnd = 0x4,
            S0Begin = 0x5,
            S0Exec = 0x6,
            S1Begin = 0x7,
            S1Exec = 0x8,
            Invalid = 0x9,
            Chmod = 0xA,
            Xor = 0xB,
            Add = 0xC,
            And = 0xD,
            Rev = 0xE,
            Gfmul = 0xF,
            Secret = 0x10,
            Keyreg = 0x11,
            Kexp = 0x12,
            Krexp = 0x13,
            Enc = 0x14,
            Dec = 0x15,
            Sigcmp = 0x16,
            Sigenc = 0x17,
            Sigclr = 0x18
        ],

        SOURCE_OP OFFSET(8) NUMBITS(6) [],

        DEST_OP OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `TSEC_SCP_STAT0` register.
    pub TSEC_SCP_STAT0 [
        RNG_ACTIVE OFFSET(16) NUMBITS(1) [],

        AES_ACTIVE OFFSET(14) NUMBITS(1) [],

        LOAD_ACTIVE OFFSET(10) NUMBITS(1) [],

        CTL_ACTIVE OFFSET(8) NUMBITS(1) [],

        SEQ_ACTIVE OFFSET(6) NUMBITS(1) [],

        STORE_ACTIVE OFFSET(4) NUMBITS(1) [],

        CMD_ACTIVE OFFSET(2) NUMBITS(1) [],

        SCP_ACTIVE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_STAT1` register.
    pub TSEC_SCP_STAT1 [
        CMD_RECV_VALID_INS OFFSET(14) NUMBITS(1) [],

        CMD_IN_HSMODE OFFSET(12) NUMBITS(1) [],

        STORE_RECV_VALID_OP OFFSET(10) NUMBITS(1) [],

        STORE_IN_HSMODE OFFSET(8) NUMBITS(1) [],

        LOAD_READY OFFSET(6) NUMBITS(1) [],

        LOAD_IN_HSMODE OFFSET(4) NUMBITS(1) [],

        SIGCMP_RESULT OFFSET(0) NUMBITS(2) [
            None = 0,
            Running = 1,
            Failed = 2,
            Succeeded = 3
        ]
    ],

    /// Bitfields of the `TSEC_SCP_STAT2` register.
    pub TSEC_SCP_STAT2 [
        AES_STALLED OFFSET(29) NUMBITS(1) [],

        RNG_STALLED OFFSET(27) NUMBITS(1) [],

        LOAD_STALLED OFFSET(26) NUMBITS(1) [],

        STORE_STALLED OFFSET(25) NUMBITS(1) [],

        CUR_AES_OP OFFSET(15) NUMBITS(2) [
            Encryption = 0,
            Decryption = 1,
            KeyExpansion = 2,
            KeyReverseExpansion = 3
        ],

        CMD_PENDING_OPCODE OFFSET(10) NUMBITS(5) [],

        CUR_CMD_OPCODE OFFSET(5) NUMBITS(5) [],

        CUR_SEQ_OPCODE OFFSET(0) NUMBITS(5) []
    ],

    /// Bitfields of the `TSEC_SCP_RNG_STAT0` register.
    pub TSEC_SCP_RNG_STAT0 [
        RND_READY OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_IRQSTAT` register.
    pub TSEC_SCP_IRQSTAT [
        TIMEOUT OFFSET(28) NUMBITS(1) [],

        RND_OP OFFSET(24) NUMBITS(1) [],

        SINGLE_STEP OFFSET(20) NUMBITS(1) [],

        CMD_ERROR OFFSET(16) NUMBITS(1) [],

        SEC_ERROR OFFSET(12) NUMBITS(1) [],

        ACL_ERROR OFFSET(8) NUMBITS(1) [],

        RND_READY OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_IRQMASK` register.
    pub TSEC_SCP_IRQMASK [
        TIMEOUT OFFSET(28) NUMBITS(1) [],

        RND_OP OFFSET(24) NUMBITS(1) [],

        SINGLE_STEP OFFSET(20) NUMBITS(1) [],

        CMD_ERROR OFFSET(16) NUMBITS(1) [],

        SEC_ERROR OFFSET(12) NUMBITS(1) [],

        ACL_ERROR OFFSET(8) NUMBITS(1) [],

        RND_READY OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_ACL_ERR` register.
    pub TSEC_SCP_ACL_ERR [
        ACL_ERROR OFFSET(31) NUMBITS(1) [],

        INV_ACL_RANGE OFFSET(8) NUMBITS(1) [],

        INV_ACL_READ OFFSET(4) NUMBITS(1) [],

        INV_ACL_WRITE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_SEC_ERR` register.
    pub TSEC_SCP_SEC_ERR [
        ERROR OFFSET(31) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_SCP_CMD_ERR` register.
    pub TSEC_SCP_CMD_ERR [
        INV_ACL_CHANGE OFFSET(24) NUMBITS(1) [],

        INV_HSMODE_SIG_OP OFFSET(20) NUMBITS(1) [],

        INV_NSMODE_SIG_OP OFFSET(16) NUMBITS(1) [],

        SEQ_UNFINISHED OFFSET(12) NUMBITS(1) [],

        SEQ_TOO_LONG OFFSET(8) NUMBITS(1) [],

        SEQ_EMPTY OFFSET(4) NUMBITS(1) [],

        INV_CMD OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_TFBIF_CTL` register.
    pub TSEC_TFBIF_CTL [
        VPR OFFSET(12) NUMBITS(1) [],

        SRTOVAL OFFSET(8) NUMBITS(4) [],

        CLR_SRTOUT OFFSET(7) NUMBITS(1) [],

        SRTOUT OFFSET(6) NUMBITS(1) [],

        IDLEWDERR OFFSET(5) NUMBITS(1) [],

        IDLE OFFSET(4) NUMBITS(1) [],

        RESET OFFSET(3) NUMBITS(1) [],

        CLR_IDLEWDERR OFFSET(2) NUMBITS(1) [],

        ENABLE OFFSET(1) NUMBITS(1) [],

        CLR_BWCOUNT OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_TFBIF_MCCIF_FIFOCTRL` register.
    pub TSEC_TFBIF_MCCIF_FIFOCTRL [
        WCLK_OVR_MODE OFFSET(8) NUMBITS(1) [],

        RCLK_OVR_MODE OFFSET(7) NUMBITS(1) [],

        CCLK_OVERRIDE OFFSET(6) NUMBITS(1) [],

        RDCL_RDFAST OFFSET(5) NUMBITS(1) [],

        WRMC_CLLE2X OFFSET(4) NUMBITS(1) [],

        RDMC_RDFAST OFFSET(3) NUMBITS(1) [],

        WRCL_MCLE2X OFFSET(2) NUMBITS(1) [],

        WCLK_OVERRIDE OFFSET(1) NUMBITS(1) [],

        RCLK_OVERRIDE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_TFBIF_THROTTLE` register.
    pub TSEC_TFBIF_THROTTLE [
        LEAK_SIZE OFFSET(30) NUMBITS(2) [],

        LEAK_COUNT OFFSET(16) NUMBITS(12) [],

        BUCKET_SIZE OFFSET(0) NUMBITS(12) []
    ],

    /// Bitfields of the `TSEC_TFBIF_DBG_STAT0` register.
    pub TSEC_TFBIF_DBG_STAT0 [
        UNWEIGHT_ACTMON_MCB OFFSET(20) NUMBITS(1) [],

        UNWEIGHT_ACTMON_ACTIVE OFFSET(19) NUMBITS(1) [],

        WU_IDLE OFFSET(17) NUMBITS(1) [],

        RU_IDLE OFFSET(16) NUMBITS(1) [],

        CSB_IDLE OFFSET(15) NUMBITS(1) [],

        WMCCIF_IDLE OFFSET(14) NUMBITS(1) [],

        RMCCIF_IDLE OFFSET(13) NUMBITS(1) [],

        ENGINE_IDLE OFFSET(12) NUMBITS(1) [],

        STALL_MREQ OFFSET(11) NUMBITS(1) [],

        STALL_WREQ_PENDING OFFSET(10) NUMBITS(1) [],

        STALL_RREQ_PENDING OFFSET(9) NUMBITS(1) [],

        STALL_WACKQ OFFSET(8) NUMBITS(1) [],

        STALL_WDATQ OFFSET(7) NUMBITS(1) [],

        STALL_WREQQ OFFSET(6) NUMBITS(1) [],

        STALL_RACKQ OFFSET(5) NUMBITS(1) [],

        STALL_RDATQ OFFSET(4) NUMBITS(1) [],

        TAGQ_ISSUED OFFSET(3) NUMBITS(1) [],

        WREQ_ISSUED OFFSET(2) NUMBITS(1) [],

        RREQ_ISSUED OFFSET(1) NUMBITS(1) [],

        ONEK_TRANSFER OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_TFBIF_MCCIF_FIFOCTRL1` register.
    pub TSEC_TFBIF_MCCIF_FIFOCTRL1 [
        SWR2MC_REORDER_DEPTH_LIMIT OFFSET(16) NUMBITS(16) [],

        SRD2MC_REORDER_DEPTH_LIMIT OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `TSEC_TFBIF_WRR_RDP` register.
    pub TSEC_TFBIF_WRR_RDP [
        INT_WEIGHT OFFSET(16) NUMBITS(16) [],

        EXT_WEIGHT OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `TSEC_TFBIF_SPROT_EMEM` register.
    pub TSEC_TFBIF_SPROT_EMEM [
        ACCESS_WRITE OFFSET(4) NUMBITS(4) [],

        ACCESS_READ OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `TSEC_TFBIF_TRANSCFG` register.
    pub TSEC_TFBIF_TRANSCFG [
        ATT7_SWID OFFSET(28) NUMBITS(1) [],

        ATT6_SWID OFFSET(24) NUMBITS(1) [],

        ATT5_SWID OFFSET(20) NUMBITS(1) [],

        ATT4_SWID OFFSET(16) NUMBITS(1) [],

        ATT3_SWID OFFSET(12) NUMBITS(1) [],

        ATT2_SWID OFFSET(8) NUMBITS(1) [],

        ATT1_SWID OFFSET(4) NUMBITS(1) [],

        ATT0_SWID OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_TFBIF_REGIONCFG` register.
    pub TSEC_TFBIF_REGIONCFG [
        T7_VPR OFFSET(31) NUMBITS(1) [],

        T7_APERT_ID OFFSET(28) NUMBITS(3) [],

        T6_VPR OFFSET(27) NUMBITS(1) [],

        T6_APERT_ID OFFSET(24) NUMBITS(3) [],

        T5_VPR OFFSET(23) NUMBITS(1) [],

        T5_APERT_ID OFFSET(20) NUMBITS(3) [],

        T4_VPR OFFSET(19) NUMBITS(1) [],

        T4_APERT_ID OFFSET(16) NUMBITS(3) [],

        T3_VPR OFFSET(15) NUMBITS(1) [],

        T3_APERT_ID OFFSET(12) NUMBITS(3) [],

        T2_VPR OFFSET(11) NUMBITS(1) [],

        T2_APERT_ID OFFSET(8) NUMBITS(3) [],

        T1_VPR OFFSET(7) NUMBITS(1) [],

        T1_APERT_ID OFFSET(4) NUMBITS(3) [],

        T0_VPR OFFSET(3) NUMBITS(1) [],

        T0_APERT_ID OFFSET(0) NUMBITS(3) []
    ],

    /// Bitfields of the `TSEC_TFBIF_THI_TRANSPROP` register.
    pub TSEC_TFBIF_THI_TRANSPROP [
        TZ_AUTH OFFSET(16) NUMBITS(1) [],

        STREAMID1 OFFSET(8) NUMBITS(7) [],

        STREAMID0 OFFSET(0) NUMBITS(7) []
    ],

    /// Bitfields of the `TSEC_CG` register.
    pub TSEC_CG [
        WAKEUP_DLY_EN OFFSET(19) NUMBITS(1) [],

        WAKEUP_DLY_CNT OFFSET(16) NUMBITS(3) [],

        IDLE_CG_EN OFFSET(6) NUMBITS(1) [],

        IDLE_CG_DLY_CNT OFFSET(0) NUMBITS(6) []
    ],

    /// Bitfields of the `TSEC_BAR0_CTL` register.
    pub TSEC_BAR0_CTL [
        CTL_INIT OFFSET(31) NUMBITS(1) [],

        SEC_MODE OFFSET(16) NUMBITS(2) [
            None = 0,
            Invalid = 1,
            LightSecure = 2,
            HeavySecure = 3
        ],

        STATUS OFFSET(12) NUMBITS(2) [
            Idle = 0,
            Busy = 1,
            Error = 2,
            Disabled = 3
        ],

        BYTE_MASK OFFSET(4) NUMBITS(4) [],

        WRITE OFFSET(1) NUMBITS(1) [],

        READ OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `TSEC_TEGRA_CTL` register.
    pub TSEC_TEGRA_CTL [
        TMPI_DISABLE_OUTPUT_I2C OFFSET(27) NUMBITS(1) [],

        TMPI_RESTART_FSM_APB OFFSET(26) NUMBITS(1) [],

        TMPI_RESTART_FSM_HOST1X OFFSET(25) NUMBITS(1) [],

        TMPI_FORCE_IDLE_INPUTS_I2C OFFSET(24) NUMBITS(1) [],

        TKFI_RESTART_FSM_KFUSE OFFSET(17) NUMBITS(1) [],

        TKFI_KFUSE OFFSET(16) NUMBITS(1) []
    ]
}

register_structs! {
    /// Representation of the TSEC registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x0000 => pub TSEC_THI_INCR_SYNCPT: ReadWrite<u32, TSEC_THI_INCR_SYNCPT::Register>),
        (0x0004 => pub TSEC_THI_INCR_SYNCPT_CTRL: ReadWrite<u32, TSEC_THI_INCR_SYNCPT_CTRL::Register>),
        (0x0008 => pub TSEC_THI_INCR_SYNCPT_ERR: ReadOnly<u32, TSEC_THI_INCR_SYNCPT_ERR::Register>),
        (0x000C => pub TSEC_THI_CTXSW_INCR_SYNCPT: ReadWrite<u32, TSEC_THI_CTXSW_INCR_SYNCPT::Register>),
        (0x0010 => _reserved0),
        (0x0020 => pub TSEC_THI_CTXSW: ReadWrite<u32, TSEC_THI_CTXSW::Register>),
        (0x0024 => pub TSEC_THI_CTXSW_NEXT: ReadWrite<u32, TSEC_THI_CTXSW_NEXT::Register>),
        (0x0028 => pub TSEC_THI_CONT_SYNCPT_EOF: ReadWrite<u32, TSEC_THI_CONT_SYNCPT_EOF::Register>),
        (0x002C => pub TSEC_THI_CONT_SYNCPT_L1: ReadWrite<u32, TSEC_THI_CONT_SYNCPT_L1::Register>),
        (0x0030 => pub TSEC_THI_STREAMID0: ReadWrite<u32, TSEC_THI_STREAMID::Register>),
        (0x0034 => pub TSEC_THI_STREAMID1: ReadWrite<u32, TSEC_THI_STREAMID::Register>),
        (0x0038 => pub TSEC_THI_THI_SEC: ReadOnly<u32, TSEC_THI_THI_SEC::Register>),
        (0x003C => _reserved1),
        (0x0040 => pub TSEC_THI_METHOD0: ReadWrite<u32, TSEC_THI_METHOD0::Register>),
        (0x0044 => pub TSEC_THI_METHOD1: ReadWrite<u32>),
        (0x0048 => _reserved2),
        (0x0060 => pub TSEC_THI_CONTEXT_SWITCH: ReadWrite<u32, TSEC_THI_CONTEXT_SWITCH::Register>),
        (0x0064 => _reserved3),
        (0x0078 => pub TSEC_THI_INT_STATUS: ReadOnly<u32, TSEC_THI_INT_STATUS::Register>),
        (0x007C => pub TSEC_THI_INT_MASK: ReadWrite<u32, TSEC_THI_INT_MASK::Register>),
        (0x0080 => pub TSEC_THI_CONFIG0: ReadWrite<u32, TSEC_THI_CONFIG0::Register>),
        (0x0084 => pub TSEC_THI_DBG_MISC: ReadWrite<u32, TSEC_THI_DBG_MISC::Register>),
        (0x0088 => pub TSEC_THI_SLCG_OVERRIDE_HIGH_A: ReadWrite<u32, TSEC_THI_SLCG_OVERRIDE_HIGH_A::Register>),
        (0x008C => pub TSEC_THI_SLCG_OVERRIDE_LOW_A: ReadWrite<u32>),
        (0x0090 => _reserved4),
        (0x0E00 => pub TSEC_THI_CLK_OVERRIDE: ReadWrite<u32>),
        (0x0E04 => _reserved5),
        (0x1000 => pub TSEC_FALCON_IRQSSET: ReadWrite<u32, TSEC_FALCON_IRQS::Register>),
        (0x1004 => pub TSEC_FALCON_IRQSCLR: ReadWrite<u32, TSEC_FALCON_IRQS::Register>),
        (0x1008 => pub TSEC_FALCON_IRQSTAT: ReadWrite<u32, TSEC_FALCON_IRQS::Register>),
        (0x100C => pub TSEC_FALCON_IRQMODE: ReadWrite<u32, TSEC_FALCON_IRQS::Register>),
        (0x1010 => pub TSEC_FALCON_IRQMSET: ReadWrite<u32, TSEC_FALCON_IRQS::Register>),
        (0x1014 => pub TSEC_FALCON_IRQMCLR: ReadWrite<u32, TSEC_FALCON_IRQS::Register>),
        (0x1018 => pub TSEC_FALCON_IRQMASK: ReadWrite<u32, TSEC_FALCON_IRQS::Register>),
        (0x101C => pub TSEC_FALCON_IRQDEST: ReadWrite<u32, TSEC_FALCON_IRQDEST::Register>),
        (0x1020 => pub TSEC_FALCON_GPTMRINT: ReadWrite<u32>),
        (0x1024 => pub TSEC_FALCON_GPTMRVAL: ReadWrite<u32>),
        (0x1028 => pub TSEC_FALCON_GPTMRCTL: ReadWrite<u32>),
        (0x102C => pub TSEC_FALCON_PTIMER0: ReadWrite<u32>),
        (0x1030 => pub TSEC_FALCON_PTIMER1: ReadWrite<u32>),
        (0x1034 => pub TSEC_FALCON_WDTMRVAL: ReadWrite<u32>),
        (0x1038 => pub TSEC_FALCON_WDTMRCTL: ReadWrite<u32, TSEC_FALCON_WDTMRCTL::Register>),
        (0x103C => pub TSEC_FALCON_IRQDEST2: ReadWrite<u32, TSEC_FALCON_IRQDEST2::Register>),
        (0x1040 => pub TSEC_FALCON_MAILBOX0: ReadWrite<u32>),
        (0x1044 => pub TSEC_FALCON_MAILBOX1: ReadWrite<u32>),
        (0x1048 => pub TSEC_FALCON_ITFEN: ReadWrite<u32, TSEC_FALCON_ITFEN::Register>),
        (0x104C => pub TSEC_FALCON_IDLESTATE: ReadOnly<u32, TSEC_FALCON_IDLESTATE::Register>),
        (0x1050 => pub TSEC_FALCON_CURCTX: ReadWrite<u32, TSEC_FALCON_CURCTX::Register>),
        (0x1054 => pub TSEC_FALCON_NXTCTX: ReadWrite<u32, TSEC_FALCON_NXTCTX::Register>),
        (0x1058 => pub TSEC_FALCON_CTXACK: ReadWrite<u32, TSEC_FALCON_CTXACK::Register>),
        (0x105C => pub TSEC_FALCON_FHSTATE: ReadOnly<u32, TSEC_FALCON_FHSTATE::Register>),
        (0x1060 => pub TSEC_FALCON_PRIVSTATE: ReadWrite<u32, TSEC_FALCON_PRIVSTATE::Register>),
        (0x1064 => pub TSEC_FALCON_MTHDDATA: ReadWrite<u32>),
        (0x1068 => pub TSEC_FALCON_MTHDID: ReadWrite<u32, TSEC_FALCON_MTHDID::Register>),
        (0x106C => pub TSEC_FALCON_MTHDWDAT: ReadWrite<u32>),
        (0x1070 => pub TSEC_FALCON_MTHDCOUNT: ReadWrite<u32, TSEC_FALCON_MTHDCOUNT::Register>),
        (0x1074 => pub TSEC_FALCON_MTHDPOP: ReadWrite<u32, TSEC_FALCON_MTHDPOP::Register>),
        (0x1078 => pub TSEC_FALCON_MTHDRAMSZ: ReadWrite<u32, TSEC_FALCON_MTHDRAMSZ::Register>),
        (0x107C => pub TSEC_FALCON_SFTRESET: ReadWrite<u32, TSEC_FALCON_SFTRESET::Register>),
        (0x1080 => pub TSEC_FALCON_OS: ReadWrite<u32>),
        (0x1084 => pub TSEC_FALCON_RM: ReadWrite<u32>),
        (0x1088 => pub TSEC_FALCON_SOFT_PM: ReadWrite<u32, TSEC_FALCON_SOFT_PM::Register>),
        (0x108C => pub TSEC_FALCON_SOFT_MODE: ReadWrite<u32, TSEC_FALCON_SOFT_MODE::Register>),
        (0x1090 => pub TSEC_FALCON_DEBUG1: ReadWrite<u32, TSEC_FALCON_DEBUG1::Register>),
        (0x1094 => pub TSEC_FALCON_DEBUGINFO: ReadWrite<u32>),
        (0x1098 => pub TSEC_FALCON_IBRKPT1: ReadWrite<u32, TSEC_FALCON_IBRKPT::Register>),
        (0x109C => pub TSEC_FALCON_IBRKPT2: ReadWrite<u32, TSEC_FALCON_IBRKPT::Register>),
        (0x10A0 => pub TSEC_FALCON_CGCTL: ReadWrite<u32, TSEC_FALCON_CGCTL::Register>),
        (0x10A4 => pub TSEC_FALCON_ENGCTL: ReadWrite<u32, TSEC_FALCON_ENGCTL::Register>),
        (0x10A8 => pub TSEC_FALCON_PMM: ReadWrite<u32, TSEC_FALCON_PMM::Register>),
        (0x10AC => pub TSEC_FALCON_ADDR: ReadWrite<u32, TSEC_FALCON_ADDR::Register>),
        (0x10B0 => pub TSEC_FALCON_IBRKPT3: ReadWrite<u32, TSEC_FALCON_IBRKPT::Register>),
        (0x10B4 => pub TSEC_FALCON_IBRKPT4: ReadWrite<u32, TSEC_FALCON_IBRKPT::Register>),
        (0x10B8 => pub TSEC_FALCON_IBRKPT5: ReadWrite<u32, TSEC_FALCON_IBRKPT::Register>),
        (0x10BC => _reserved6),
        (0x10D0 => pub TSEC_FALCON_EXCI: ReadOnly<u32, TSEC_FALCON_EXCI::Register>),
        (0x10D4 => pub TSEC_FALCON_SVEC_SPR: ReadOnly<u32, TSEC_FALCON_SVEC_SPR::Register>),
        (0x10D8 => pub TSEC_FALCON_RSTAT0: ReadOnly<u32>),
        (0x10DC => pub TSEC_FALCON_RSTAT3: ReadOnly<u32>),
        (0x10E0 => _reserved7),
        (0x1100 => pub TSEC_FALCON_CPUCTL: ReadWrite<u32, TSEC_FALCON_CPUCTL::Register>),
        (0x1104 => pub TSEC_FALCON_BOOTVEC: ReadWrite<u32>),
        (0x1108 => pub TSEC_FALCON_HWCFG: ReadOnly<u32, TSEC_FALCON_HWCFG::Register>),
        (0x110C => pub TSEC_FALCON_DMACTL: ReadWrite<u32, TSEC_FALCON_DMACTL::Register>),
        (0x1110 => pub TSEC_FALCON_DMATRFBASE: ReadWrite<u32>),
        (0x1114 => pub TSEC_FALCON_DMATRFMOFFS: ReadWrite<u32, TSEC_FALCON_DMATRFMOFFS::Register>),
        (0x1118 => pub TSEC_FALCON_DMATRFCMD: ReadWrite<u32, TSEC_FALCON_DMATRFCMD::Register>),
        (0x111C => pub TSEC_FALCON_DMATRFFBOFFS: ReadWrite<u32>),
        (0x1120 => pub TSEC_FALCON_DMAPOLL_FB: ReadOnly<u32, TSEC_FALCON_DMAPOLL::Register>),
        (0x1124 => pub TSEC_FALCON_DMAPOLL_CP: ReadOnly<u32, TSEC_FALCON_DMAPOLL::Register>),
        (0x1128 => _reserved8),
        (0x112C => pub TSEC_FALCON_HWCFG1: ReadOnly<u32, TSEC_FALCON_HWCFG1::Register>),
        (0x1130 => pub TSEC_FALCON_CPUCTL_ALIAS: ReadWrite<u32, TSEC_FALCON_CPUCTL_ALIAS::Register>),
        (0x1134 => _reserved9),
        (0x1138 => pub TSEC_FALCON_STACKCFG: ReadWrite<u32, TSEC_FALCON_STACKCFG::Register>),
        (0x113C => _reserved10),
        (0x1140 => pub TSEC_FALCON_IMCTL: ReadWrite<u32, TSEC_FALCON_IMCTL::Register>),
        (0x1144 => pub TSEC_FALCON_IMSTAT: ReadOnly<u32>),
        (0x1148 => pub TSEC_FALCON_TRACEIDX: ReadWrite<u32, TSEC_FALCON_TRACEIDX::Register>),
        (0x114C => pub TSEC_FALCON_TRACEPC: ReadOnly<u32, TSEC_FALCON_TRACEPC::Register>),
        (0x1150 => pub TSEC_FALCON_IMFILLRNG0: ReadWrite<u32, TSEC_FALCON_IMFILLRNG::Register>),
        (0x1154 => pub TSEC_FALCON_IMFILLRNG1: ReadWrite<u32, TSEC_FALCON_IMFILLRNG::Register>),
        (0x1158 => pub TSEC_FALCON_IMFILLCTL: ReadWrite<u32, TSEC_FALCON_IMFILLCTL::Register>),
        (0x115C => pub TSEC_FALCON_IMCTL_DEBUG: ReadWrite<u32, TSEC_FALCON_IMCTL_DEBUG::Register>),
        (0x1160 => pub TSEC_FALCON_CMEMBASE: ReadWrite<u32, TSEC_FALCON_CMEMBASE::Register>),
        (0x1164 => pub TSEC_FALCON_DMEMAPERT: ReadWrite<u32, TSEC_FALCON_DMEMAPERT::Register>),
        (0x1168 => pub TSEC_FALCON_EXTERRADDR: ReadWrite<u32>),
        (0x116C => pub TSEC_FALCON_EXTERRSTAT: ReadOnly<u32, TSEC_FALCON_EXTERRSTAT::Register>),
        (0x1170 => _reserved11),
        (0x117C => pub TSEC_FALCON_CG2: ReadWrite<u32, TSEC_FALCON_CG2::Register>),
        (0x1180 => pub TSEC_FALCON_IMEMC0: ReadWrite<u32, TSEC_FALCON_IMEMC::Register>),
        (0x1184 => pub TSEC_FALCON_IMEMD0: ReadWrite<u32>),
        (0x1188 => pub TSEC_FALCON_IMEMT0: ReadWrite<u32, TSEC_FALCON_IMEMT::Register>),
        (0x118C => _reserved12),
        (0x1190 => pub TSEC_FALCON_IMEMC1: ReadWrite<u32, TSEC_FALCON_IMEMC::Register>),
        (0x1194 => pub TSEC_FALCON_IMEMD1: ReadWrite<u32>),
        (0x1198 => pub TSEC_FALCON_IMEMT1: ReadWrite<u32, TSEC_FALCON_IMEMT::Register>),
        (0x119C => _reserved13),
        (0x11A0 => pub TSEC_FALCON_IMEMC2: ReadWrite<u32, TSEC_FALCON_IMEMC::Register>),
        (0x11A4 => pub TSEC_FALCON_IMEMD2: ReadWrite<u32>),
        (0x11A8 => pub TSEC_FALCON_IMEMT2: ReadWrite<u32, TSEC_FALCON_IMEMT::Register>),
        (0x11AC => _reserved14),
        (0x11B0 => pub TSEC_FALCON_IMEMC3: ReadWrite<u32, TSEC_FALCON_IMEMC::Register>),
        (0x11B4 => pub TSEC_FALCON_IMEMD3: ReadWrite<u32>),
        (0x11B8 => pub TSEC_FALCON_IMEMT3: ReadWrite<u32, TSEC_FALCON_IMEMT::Register>),
        (0x11BC => _reserved15),
        (0x11C0 => pub TSEC_FALCON_DMEMC0: ReadWrite<u32, TSEC_FALCON_DMEMC::Register>),
        (0x11C4 => pub TSEC_FALCON_DMEMD0: ReadWrite<u32>),
        (0x11C8 => pub TSEC_FALCON_DMEMC1: ReadWrite<u32, TSEC_FALCON_DMEMC::Register>),
        (0x11CC => pub TSEC_FALCON_DMEMD1: ReadWrite<u32>),
        (0x11D0 => pub TSEC_FALCON_DMEMC2: ReadWrite<u32, TSEC_FALCON_DMEMC::Register>),
        (0x11D4 => pub TSEC_FALCON_DMEMD2: ReadWrite<u32>),
        (0x11D8 => pub TSEC_FALCON_DMEMC3: ReadWrite<u32, TSEC_FALCON_DMEMC::Register>),
        (0x11DC => pub TSEC_FALCON_DMEMD3: ReadWrite<u32>),
        (0x11E0 => pub TSEC_FALCON_DMEMC4: ReadWrite<u32, TSEC_FALCON_DMEMC::Register>),
        (0x11E4 => pub TSEC_FALCON_DMEMD4: ReadWrite<u32>),
        (0x11E8 => pub TSEC_FALCON_DMEMC5: ReadWrite<u32, TSEC_FALCON_DMEMC::Register>),
        (0x11EC => pub TSEC_FALCON_DMEMD5: ReadWrite<u32>),
        (0x11F0 => pub TSEC_FALCON_DMEMC6: ReadWrite<u32, TSEC_FALCON_DMEMC::Register>),
        (0x11F4 => pub TSEC_FALCON_DMEMD6: ReadWrite<u32>),
        (0x11F8 => pub TSEC_FALCON_DMEMC7: ReadWrite<u32, TSEC_FALCON_DMEMC::Register>),
        (0x11FC => pub TSEC_FALCON_DMEMD7: ReadWrite<u32>),
        (0x1200 => pub TSEC_FALCON_ICD_CMD: ReadWrite<u32, TSEC_FALCON_ICD_CMD::Register>),
        (0x1204 => pub TSEC_FALCON_ICD_ADDR: ReadWrite<u32>),
        (0x1208 => pub TSEC_FALCON_ICD_WDATA: WriteOnly<u32>),
        (0x120C => pub TSEC_FALCON_ICD_RDATA: ReadOnly<u32>),
        (0x1210 => _reserved16),
        (0x1240 => pub TSEC_FALCON_SCTL: ReadWrite<u32, TSEC_FALCON_SCTL::Register>),
        (0x1244 => pub TSEC_FALCON_SSTAT: ReadOnly<u32, TSEC_FALCON_SSTAT::Register>),
        (0x1248 => _reserved17),
        (0x1280 => pub TSEC_FALCON_SPROT_IMEM: ReadWrite<u32, TSEC_FALCON_SPROT::Register>),
        (0x1284 => pub TSEC_FALCON_SPROT_DMEM: ReadWrite<u32, TSEC_FALCON_SPROT::Register>),
        (0x1288 => pub TSEC_FALCON_SPROT_CPUCTL: ReadWrite<u32, TSEC_FALCON_SPROT::Register>),
        (0x128C => pub TSEC_FALCON_SPROT_MISC: ReadWrite<u32, TSEC_FALCON_SPROT::Register>),
        (0x1290 => pub TSEC_FALCON_SPROT_IRQ: ReadWrite<u32, TSEC_FALCON_SPROT::Register>),
        (0x1294 => pub TSEC_FALCON_SPROT_MTHD: ReadWrite<u32, TSEC_FALCON_SPROT::Register>),
        (0x1298 => pub TSEC_FALCON_SPROT_SCTL: ReadWrite<u32, TSEC_FALCON_SPROT::Register>),
        (0x129C => pub TSEC_FALCON_SPROT_WDTMR: ReadWrite<u32, TSEC_FALCON_SPROT::Register>),
        (0x12A0 => _reserved18),
        (0x12C0 => pub TSEC_FALCON_DMAINFO_FINISHED_FBRD_LOW: ReadWrite<u32>),
        (0x12C4 => pub TSEC_FALCON_DMAINFO_FINISHED_FBRD_HIGH: ReadWrite<u32>),
        (0x12C8 => pub TSEC_FALCON_DMAINFO_FINISHED_FBWR_LOW: ReadWrite<u32>),
        (0x12CC => pub TSEC_FALCON_DMAINFO_FINISHED_FBWR_HIGH: ReadWrite<u32>),
        (0x12D0 => pub TSEC_FALCON_DMAINFO_CURRENT_FBRD_LOW: ReadWrite<u32>),
        (0x12D4 => pub TSEC_FALCON_DMAINFO_CURRENT_FBRD_HIGH: ReadWrite<u32>),
        (0x12D8 => pub TSEC_FALCON_DMAINFO_CURRENT_FBWR_LOW: ReadWrite<u32>),
        (0x12DC => pub TSEC_FALCON_DMAINFO_CURRENT_FBWR_HIGH: ReadWrite<u32>),
        (0x12E0 => pub TSEC_FALCON_DMAINFO_CTL: ReadWrite<u32, TSEC_FALCON_DMAINFO_CTL::Register>),
        (0x12E4 => _reserved19),
        (0x1400 => pub TSEC_SCP_CTL0: ReadWrite<u32, TSEC_SCP_CTL0::Register>),
        (0x1404 => pub TSEC_SCP_CTL1: ReadWrite<u32, TSEC_SCP_CTL1::Register>),
        (0x1408 => pub TSEC_SCP_CTL_STAT: ReadOnly<u32, TSEC_SCP_CTL_STAT::Register>),
        (0x140C => pub TSEC_SCP_CTL_LOCK: ReadWrite<u32, TSEC_SCP_CTL_LOCK::Register>),
        (0x1410 => pub TSEC_SCP_CFG: ReadWrite<u32, TSEC_SCP_CFG::Register>),
        (0x1414 => pub TSEC_SCP_CTL_SCP: ReadWrite<u32, TSEC_SCP_CTL_SCP::Register>),
        (0x1418 => pub TSEC_SCP_CTL_PKEY: ReadWrite<u32, TSEC_SCP_CTL_PKEY::Register>),
        (0x141C => pub TSEC_SCP_CTL_DBG: ReadWrite<u32>),
        (0x1420 => pub TSEC_SCP_DBG0: ReadWrite<u32, TSEC_SCP_DBG0::Register>),
        (0x1424 => pub TSEC_SCP_DBG1: ReadWrite<u32, TSEC_SCP_DBG1::Register>),
        (0x1428 => pub TSEC_SCP_DBG2: ReadWrite<u32, TSEC_SCP_DBG2::Register>),
        (0x142C => _reserved20),
        (0x1430 => pub TSEC_SCP_CMD: ReadOnly<u32, TSEC_SCP_CMD::Register>),
        (0x1434 => _reserved21),
        (0x1450 => pub TSEC_SCP_STAT0: ReadOnly<u32, TSEC_SCP_STAT0::Register>),
        (0x1454 => pub TSEC_SCP_STAT1: ReadOnly<u32, TSEC_SCP_STAT1::Register>),
        (0x1458 => pub TSEC_SCP_STAT2: ReadOnly<u32, TSEC_SCP_STAT2::Register>),
        (0x145C => _reserved22),
        (0x1470 => pub TSEC_SCP_RNG_STAT0: ReadOnly<u32, TSEC_SCP_RNG_STAT0::Register>),
        (0x1474 => pub TSEC_SCP_RNG_STAT1: ReadOnly<u32>),
        (0x1478 => _reserved23),
        (0x1480 => pub TSEC_SCP_IRQSTAT: ReadOnly<u32, TSEC_SCP_IRQSTAT::Register>),
        (0x1484 => pub TSEC_SCP_IRQMASK: ReadOnly<u32, TSEC_SCP_IRQMASK::Register>),
        (0x1488 => _reserved24),
        (0x1490 => pub TSEC_SCP_ACL_ERR: ReadOnly<u32, TSEC_SCP_ACL_ERR::Register>),
        (0x1494 => pub TSEC_SCP_SEC_ERR: ReadOnly<u32, TSEC_SCP_SEC_ERR::Register>),
        (0x1498 => pub TSEC_SCP_CMD_ERR: ReadOnly<u32, TSEC_SCP_CMD_ERR::Register>),
        (0x149C => _reserved25),
        (0x1500 => pub TSEC_SCP_RND_CTL0: ReadWrite<u32>),
        (0x1504 => pub TSEC_SCP_RND_CTL1: ReadWrite<u32>),
        (0x1508 => pub TSEC_SCP_RND_CTL2: ReadWrite<u32>),
        (0x150C => pub TSEC_SCP_RND_CTL3: ReadWrite<u32>),
        (0x1510 => pub TSEC_SCP_RND_CTL4: ReadWrite<u32>),
        (0x1514 => pub TSEC_SCP_RND_CTL5: ReadWrite<u32>),
        (0x1518 => pub TSEC_SCP_RND_CTL6: ReadWrite<u32>),
        (0x151C => pub TSEC_SCP_RND_CTL7: ReadWrite<u32>),
        (0x1520 => pub TSEC_SCP_RND_CTL8: ReadWrite<u32>),
        (0x1524 => pub TSEC_SCP_RND_CTL9: ReadWrite<u32>),
        (0x1528 => pub TSEC_SCP_RND_CTL10: ReadWrite<u32>),
        (0x152C => pub TSEC_SCP_RND_CTL11: ReadWrite<u32>),
        (0x1530 => _reserved26),
        (0x1600 => pub TSEC_TFBIF_CTL: ReadWrite<u32, TSEC_TFBIF_CTL::Register>),
        (0x1604 => pub TSEC_TFBIF_MCCIF_FIFOCTRL: ReadWrite<u32, TSEC_TFBIF_MCCIF_FIFOCTRL::Register>),
        (0x1608 => pub TSEC_TFBIF_THROTTLE: ReadWrite<u32, TSEC_TFBIF_THROTTLE::Register>),
        (0x160C => pub TSEC_TFBIF_DBG_STAT0: ReadOnly<u32, TSEC_TFBIF_DBG_STAT0::Register>),
        (0x1610 => pub TSEC_TFBIF_DBG_STAT1: ReadOnly<u32>),
        (0x1614 => pub TSEC_TFBIF_DBG_RDCOUNT_LO: ReadOnly<u32>),
        (0x1618 => pub TSEC_TFBIF_DBG_RDCOUNT_HI: ReadOnly<u32>),
        (0x161C => pub TSEC_TFBIF_DBG_WRCOUNT_LO: ReadOnly<u32>),
        (0x1620 => pub TSEC_TFBIF_DBG_WRCOUNT_HI: ReadOnly<u32>),
        (0x1624 => pub TSEC_TFBIF_DBG_R32COUNT: ReadOnly<u32>),
        (0x1628 => pub TSEC_TFBIF_DBG_R64COUNT: ReadOnly<u32>),
        (0x162C => pub TSEC_TFBIF_DBG_R128COUNT: ReadOnly<u32>),
        (0x1630 => _reserved27),
        (0x1634 => pub TSEC_TFBIF_MCCIF_FIFOCTRL1: ReadWrite<u32, TSEC_TFBIF_MCCIF_FIFOCTRL1::Register>),
        (0x1638 => pub TSEC_TFBIF_WRR_RDP: ReadWrite<u32, TSEC_TFBIF_WRR_RDP::Register>),
        (0x163C => _reserved28),
        (0x1640 => TSEC_TFBIF_SPROT_EMEM: ReadWrite<u32, TSEC_TFBIF_SPROT_EMEM::Register>),
        (0x1644 => pub TSEC_TFBIF_TRANSCFG: ReadWrite<u32, TSEC_TFBIF_TRANSCFG::Register>),
        (0x1648 => pub TSEC_TFBIF_REGIONCFG: ReadWrite<u32, TSEC_TFBIF_REGIONCFG::Register>),
        (0x164C => pub TSEC_TFBIF_ACTMON_ACTIVE_MASK: ReadWrite<u32>),
        (0x1650 => pub TSEC_TFBIF_ACTMON_ACTIVE_BORPS: ReadWrite<u32>),
        (0x1654 => pub TSEC_TFBIF_ACTMON_ACTIVE_WEIGHT: ReadWrite<u32>),
        (0x1658 => _reserved29),
        (0x1660 => pub TSEC_TFBIF_ACTMON_MCB_MASK: ReadWrite<u32>),
        (0x1664 => pub TSEC_TFBIF_ACTMON_MCB_BORPS: ReadWrite<u32>),
        (0x1668 => pub TSEC_TFBIF_ACTMON_MCB_WEIGHT: ReadWrite<u32>),
        (0x166C => _reserved30),
        (0x1670 => pub TSEC_TFBIF_THI_TRANSPROP: ReadWrite<u32, TSEC_TFBIF_THI_TRANSPROP::Register>),
        (0x1674 => _reserved31),
        (0x16D0 => pub TSEC_CG: ReadWrite<u32, TSEC_CG::Register>),
        (0x16D4 => _reserved32),
        (0x1700 => pub TSEC_BAR0_CTL: ReadWrite<u32, TSEC_BAR0_CTL::Register>),
        (0x1704 => pub TSEC_BAR0_ADDR: ReadWrite<u32>),
        (0x1708 => pub TSEC_BAR0_DATA: ReadWrite<u32>),
        (0x170C => pub TSEC_BAR0_TIMEOUT: ReadWrite<u32>),
        (0x1710 => _reserved33),
        (0x1838 => pub TSEC_TEGRA_CTL: ReadWrite<u32, TSEC_TEGRA_CTL::Register>),
        (0x183C => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x183C]);
