use register::{mmio::*, register_bitfields, register_structs};

use crate::memory_map::VIC;

/// A pointer to the Video Image Compositor registers that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = VIC as *const Registers;

register_bitfields! {
    u32,

    /// Bitfields of the `NV_PVIC_THI_INCR_SYNCPT` register.
    pub NV_PVIC_THI_INCR_SYNCPT [
        /// Indicates sync point condition at which THI has to return the index value
        /// back to Host1x.
        NV_PVIC_THI_INCR_SYNCPT_COND OFFSET(8) NUMBITS(8) [
            CondImmediate = 0,
            CondOpDone = 1
        ],

        /// Indicates the sync point index value, THI will return this index back to
        /// Host1x when the particular sync point condition is done.
        NV_PVIC_THI_INCR_SYNCPT_INDX OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `NV_PVIC_THI_INCR_SYNCPT_ERR` register.
    pub NV_PVIC_THI_INCR_SYNCPT_ERR [
        NV_PVIC_THI_INCR_SYNCPT_ERR_COND_STS_OPDONE OFFSET(1) NUMBITS(1) [
            CondStsOpdoneInit = 0,
            CondStsOpdoneClear = 1
        ],

        NV_PVIC_THI_INCR_SYNCPT_ERR_COND_STS_IMM OFFSET(0) NUMBITS(1) [
            CondStsImmInit = 0,
            CondStsImmClear = 1
        ]
    ],

    /// Bitfields of the `NV_PVIC_THI_CTXSW_INCR_SYNCPT` register.
    pub NV_PVIC_THI_CTXSW_INCR_SYNCPT [
        /// Indicates the sync point index value, THI will return this index value back
        /// to Host1x when the context save/restore operation is done.
        NV_PVIC_THI_CTXSW_INCR_SYNCPT_INDX OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `NV_PVIC_THI_CTXSW` register.
    pub NV_PVIC_THI_CTXSW [
        /// Indicates next requested channel of engine.
        NV_PVIC_THI_CTXSW_NEXT_CHANNEL OFFSET(28) NUMBITS(4) [],

        /// Indicates next requested class of engine.
        NV_PVIC_THI_CTXSW_NEXT_CLASS OFFSET(16) NUMBITS(10) [],

        /// Indicates current working channel of engine. Reset to invalid.
        NV_PVIC_THI_CTXSW_CURR_CHANNEL OFFSET(12) NUMBITS(4) [],

        /// Tells the module to automatically acknowledge any incoming context switch
        /// requests without triggering an interrupt.
        ///
        /// NOTE: This bit is read-only.
        NV_PVIC_THI_CTXSW_AUTO_ACK OFFSET(11) NUMBITS(1) [],

        /// Indicates current working class of engine.
        NV_PVIC_THI_CTXSW_CURR_CLASS OFFSET(0) NUMBITS(10) []
    ],

    /// Bitfields of the `NV_PVIC_THI_CONT_SYNCPT_EOF` register.
    pub NV_PVIC_THI_CONT_SYNCPT_EOF [
        /// Sync Point Condition Control: Can be used to enable or disable generation of
        /// continuous sync point increment.
        NV_PVIC_THI_CONT_SYNCPT_EOF_COND OFFSET(8) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Sync Point Counter Index: Specifies the index of the sync point counter that
        /// are returned to host when continuous sync point is enabled and whenever a
        /// FRAME_DONE happens.
        NV_PVIC_THI_CONT_SYNCPT_EOF_INDEX OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `NV_PVIC_THI_METHOD0` register.
    pub NV_PVIC_THI_METHOD0 [
        /// Contains the method ID which is to be sent to Falcon over the method interface.
        /// THI waits for writes to the `METHOD_DATA` register before triggering any write
        /// over the Falcon method interface.
        NV_PVIC_THI_METHOD0_OFFSET OFFSET(0) NUMBITS(12) []
    ],

    /// Bitfields of the `NV_PVIC_THI_INT_STATUS` register.
    pub NV_PVIC_THI_INT_STATUS [
        /// Implies if there is any pending Falcon interrupt corresponding to an error
        /// condition.
        NV_PVIC_THI_INT_STATUS_FALCON_INT OFFSET(0) NUMBITS(1) [
            FalconIntInit = 0,
            FalconIntClear = 1
        ]
    ],

    /// Bitfields of the `NV_PVIC_THI_INT_MASK` register.
    pub NV_PVIC_THI_INT_MASK [
        /// When set, this enables generation of interrupts corresponding to Falcon error
        /// conditions.
        NV_PVIC_THI_INT_MASK_FALCON_INT OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `NV_PVIC_FALCON_ITFEN` register.
    pub NV_PVIC_FALCON_ITFEN [
        /// Indicates whether to use a post write on the main priv interface. By default,
        /// all writes to the Control and Status Bus (CSB) from the main priv interface
        /// are non-posted to support error reporting.
        NV_PVIC_FALCON_ITFEN_PRIV_POSTWR OFFSET(2) NUMBITS(1) [
            PrivPostwrFalse = 0,
            PrivPostwrTrue = 1
        ],

        /// Method interface enable. When set, allows the host to push methods into the
        /// method FIFO.
        NV_PVIC_FALCON_ITFEN_MTHDEN OFFSET(1) NUMBITS(1) [
            MthdenDisable = 0,
            MthdenEnable = 1
        ],

        /// Context switch interface enable. When set, allows the host context switch state
        /// machine to react to incoming context switch requests from the host.
        NV_PVIC_FALCON_ITFEN_CTXEN OFFSET(0) NUMBITS(1) [
            CtxenDisable = 0,
            CtxenEnable = 1
        ]
    ],

    /// Bitfields of the `NV_PVIC_FALCON_CPUCTL` register.
    pub NV_PVIC_FALCON_CPUCTL [
        /// Indicates whether the CPU is currently in the stopped state. Falcon exits this
        /// state if a `1` is written to the `STARTCPU` bit or if an interrupt arrives at
        /// one of its 2 inputs and the corresponding IE bit in CSW is set.
        ///
        /// NOTE: This bit is read-only.
        NV_PVIC_FALCON_CPUCTL_STOPPED OFFSET(5) NUMBITS(1) [],

        /// Indicates whether the CPU is currently in the halted state. Falcon can only
        /// exit this state when a `1` is written to the `STARTCPU` bit.
        ///
        /// NOTE: This bit is read-only.
        NV_PVIC_FALCON_CPUCTL_HALTED OFFSET(4) NUMBITS(1) [],

        /// Set to apply hard reset. This bit will auto-clear.
        NV_PVIC_FALCON_CPUCTL_HRESET OFFSET(3) NUMBITS(1) [],

        /// Set to apply soft reset. This bit will auto-clear.
        NV_PVIC_FALCON_CPUCTL_SRESET OFFSET(2) NUMBITS(1) [],

        /// Set this bit to start CPU execution while in a halted state. If a start request
        /// is still pending, clearing this bit will cancel the start request. Writing any
        /// value has no effect while the CPU is running.
        NV_PVIC_FALCON_CPUCTL_STARTCPU OFFSET(1) NUMBITS(1) [],

        /// Set this bit to mark all blocks in IMEM except block 0 as INVALID. This bit will
        /// auto-clear.
        NV_PVIC_FALCON_CPUCTL_IINVAL OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `NV_PVIC_FALCON_DMACTL` register.
    pub NV_PVIC_FALCON_DMACTL [
        /// Indicates valid request number at DMA request queue.
        NV_PVIC_FALCON_DMACTL_DMAQ_NUM OFFSET(3) NUMBITS(4) [],

        /// When set, a valid context must be loaded before any DMA request can be serviced.
        /// Pending requests without a valid current context remain pending, and do not
        /// prevent the engine from reporting idle. When clear, DMA requests are serviced
        /// regardless of the current context.
        ///
        /// Once a request is issued, it must complete before the engine can report idle, as
        /// needed for example to process WFI context switch requests.
        NV_PVIC_FALCON_DMACTL_REQUIRE_CTX OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `NV_PVIC_FALCON_DMATRFMOFFS` register.
    pub NV_PVIC_FALCON_DMATRFMOFFS [
        NV_PVIC_FALCON_DMATRFMOFFS_OFFS OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `NV_PVIC_FALCON_DMATRFCMD` register.
    pub NV_PVIC_FALCON_DMATRFCMD [
        NV_PVIC_FALCON_DMATRFCMD_CTXDMA OFFSET(12) NUMBITS(3) [],

        NV_PVIC_FALCON_DMATRFCMD_SIZE OFFSET(8) NUMBITS(3) [
            Size4B = 0,
            Size8B = 1,
            Size16B = 2,
            Size32B = 3,
            Size64B = 4,
            Size128B = 5,
            Size256B = 6
        ],

        NV_PVIC_FALCON_DMATRFCMD_WRITE OFFSET(5) NUMBITS(1) [],

        NV_PVIC_FALCON_DMATRFCMD_IMEM OFFSET(4) NUMBITS(1) [],

        /// Indicates that the DMA engine is still busy with a transfer or has more
        /// transfers pending in the queue.
        ///
        /// NOTE: This bit is read-only.
        NV_PVIC_FALCON_DMATRFCMD_IDLE OFFSET(1) NUMBITS(1) [],

        /// Indicates that the DMA request queue is full and a valid request is still
        /// needed to move into the queue.
        ///
        /// NOTE: This bit is read-only.
        NV_PVIC_FALCON_DMATRFCMD_FULL OFFSET(0) NUMBITS(1) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub Registers {
        (0x0000 => pub NV_PVIC_THI_INCR_SYNCPT: ReadWrite<u32, NV_PVIC_THI_INCR_SYNCPT::Register>),
        (0x0004 => _reserved0),
        (0x0008 => pub NV_PVIC_THI_INCR_SYNCPT_ERR: ReadWrite<u32, NV_PVIC_THI_INCR_SYNCPT_ERR::Register>),
        (0x000C => pub NV_PVIC_THI_CTXSW_INCR_SYNCPT: ReadWrite<u32, NV_PVIC_THI_CTXSW_INCR_SYNCPT::Register>),
        (0x0010 => _reserved1),
        (0x0020 => pub NV_PVIC_THI_CTXSW: ReadWrite<u32, NV_PVIC_THI_CTXSW::Register>),
        (0x0024 => _reserved2),
        (0x0028 => pub NV_PVIC_THI_CONT_SYNCPT_EOF: ReadWrite<u32, NV_PVIC_THI_CONT_SYNCPT_EOF::Register>),
        (0x002C => _reserved3),
        (0x0040 => pub NV_PVIC_THI_METHOD0: ReadWrite<u32, NV_PVIC_THI_METHOD0::Register>),
        (0x0044 => pub NV_PVIC_THI_METHOD1: ReadWrite<u32>),
        (0x0048 => _reserved4),
        (0x0078 => pub NV_PVIC_THI_INT_STATUS: ReadWrite<u32, NV_PVIC_THI_INT_STATUS::Register>),
        (0x007C => pub NV_PVIC_THI_INT_MASK: ReadWrite<u32, NV_PVIC_THI_INT_MASK::Register>),
        (0x0080 => _reserved5),
        (0x1048 => pub NV_PVIC_FALCON_ITFEN: ReadWrite<u32, NV_PVIC_FALCON_ITFEN::Register>),
        (0x104C => _reserved6),
        (0x1100 => pub NV_PVIC_FALCON_CPUCTL: ReadWrite<u32, NV_PVIC_FALCON_CPUCTL::Register>),
        (0x1104 => pub NV_PVIC_FALCON_BOOTVEC: ReadWrite<u32>),
        (0x1108 => _reserved7),
        (0x110C => pub NV_PVIC_FALCON_DMACTL: ReadWrite<u32, NV_PVIC_FALCON_DMACTL::Register>),
        (0x1110 => pub NV_PVIC_FALCON_DMATRFBASE: ReadWrite<u32>),
        (0x1114 => pub NV_PVIC_FALCON_DMATRFMOFFS: ReadWrite<u32, NV_PVIC_FALCON_DMATRFMOFFS::Register>),
        (0x1118 => pub NV_PVIC_FALCON_DMATRFCMD: ReadWrite<u32, NV_PVIC_FALCON_DMATRFCMD::Register>),
        (0x111C => pub NV_PVIC_FALCON_DMATRFFBOFFS: ReadWrite<u32>),
        (0x1120 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x1120]);
