//! Abstractions over the Flow Controller registers of the Tegra X1.
//!
//! See Chapter 17.2 in the Tegra X1 Technical Reference Manual for
//! details.

use register::{mmio::*, register_bitfields, register_structs};

use crate::memory_map::FLOW;

/// A pointer to the Flow register block that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = FLOW as *const Registers;

register_bitfields! {
    u32,

    /// Bitfields of the `FLOW_CTLR_HALT_CPU_<x>_EVENTS_0` register.
    pub FLOW_CTLR_HALT_CPU_EVENTS_0 [
        MODE OFFSET(29) NUMBITS(2) [
            FlowModeNone = 0,
            FlowModeRunAndInt = 1,
            FlowModeWaitevent = 2,
            FlowModeWaiteventAndInt = 3,
            FlowModeStopUntilIrq = 4,
            FlowModeStopUntilIrqAndInt = 5,
            FlowModeStopUntilEventAndIrq = 6
        ],

        /// Resume on JTAG activity.
        JTAG OFFSET(28) NUMBITS(1) [],

        /// Resume on SYSCLK cycle ticks.
        SCLK OFFSET(27) NUMBITS(1) [],

        /// Resume on Nth X32K clock input ticks.
        X32K OFFSET(26) NUMBITS(1) [],

        /// Resume on Nth μs clock ticks.
        USEC OFFSET(25) NUMBITS(1) [],

        /// Resume on Nth ms clock ticks.
        MSEC OFFSET(24) NUMBITS(1) [],

        /// Resume on Nth second RTC clock ticks.
        SEC OFFSET(23) NUMBITS(1) [],

        /// Resume on Nth XIO.RDY Ext. IO Ready Events.
        X_RDY OFFSET(22) NUMBITS(1) [],

        /// Resume on SMP.3[1,0] Semaphore Set Events.
        SMP31_0 OFFSET(20) NUMBITS(2) [],

        /// Resume on Nth XRQ.[D,C,B,A] External Trigger Events.
        XRQ_D_C_B_A OFFSET(16) NUMBITS(4) [],

        /// Resume on Nth [O,I]B[E,F] [Outbox, Inbox] [Empty, Full] Events.
        OBE_F_IBE_F OFFSET(12) NUMBITS(4) [],

        /// Resume on Legacy Interrupt Controller IRQ interrupt.
        LIC_IRQN OFFSET(11) NUMBITS(1) [],

        /// Resume on Legacy Interrupt Controller FIQ interrupt.
        LIC_FIQN OFFSET(10) NUMBITS(1) [],

        /// Resume on MPCore GIC IRQ interrupt.
        GIC_IRQN OFFSET(9) NUMBITS(1) [],

        /// Resume on MPCore GIC FIQ interrupt.
        GIC_FIQN OFFSET(8) NUMBITS(1) [],

        /// Initialized, then decremented.
        ZERO OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `FLOW_CTLR_HALT_COP_EVENTS_0` register.
    pub FLOW_CTLR_HALT_COP_EVENTS_0 [
        MODE OFFSET(29) NUMBITS(2) [
            FlowModeNone = 0,
            FlowModeRunAndInt = 1,
            FlowModeWaitevent = 2,
            FlowModeWaiteventAndInt = 3,
            FlowModeStopUntilIrq = 4,
            FlowModeStopUntilIrqAndInt = 5,
            FlowModeStopUntilEventAndIrq = 6
        ],

        JTAG OFFSET(28) NUMBITS(1) [],

        SCLK OFFSET(27) NUMBITS(1) [],

        X32K OFFSET(26) NUMBITS(1) [],

        USEC OFFSET(25) NUMBITS(1) [],

        MSEC OFFSET(24) NUMBITS(1) [],

        SEC OFFSET(23) NUMBITS(1) [],

        X_RDY OFFSET(22) NUMBITS(1) [],

        SMP31 OFFSET(21) NUMBITS(1) [],

        SMP30 OFFSET(20) NUMBITS(1) [],

        XRQ_D OFFSET(19) NUMBITS(1) [],

        XRQ_C OFFSET(18) NUMBITS(1) [],

        XRQ_B OFFSET(17) NUMBITS(1) [],

        XRQ_A OFFSET(16) NUMBITS(1) [],

        OBE OFFSET(15) NUMBITS(1) [],

        OBF OFFSET(14) NUMBITS(1) [],

        IBE OFFSET(13) NUMBITS(1) [],

        IBF OFFSET(12) NUMBITS(1) [],

        LIC_IRQ OFFSET(11) NUMBITS(1) [],

        LIC_FIQ OFFSET(10) NUMBITS(1) [],

        GIC_IRQ OFFSET(9) NUMBITS(1) [],

        GIC_FIQ OFFSET(8) NUMBITS(1) [],

        ZERO OFFSET(0) NUMBITS(9) []
    ],

    /// Bitfields of the `FLOW_CTLR_CPU_<x>_CSR_0` register.
    pub FLOW_CTLR_CPU_CSR_0 [
        /// Current state of the Power-Gate State Machine.
        ///
        /// NOTE: These bits are read-only.
        PWR_STATE OFFSET(24) NUMBITS(4) [],

        /// Whether the CPU is waiting.
        ///
        /// If that is the case, it can be woken up via an event.
        ///
        /// NOTE: This bit is read-only.
        WAIT_EVENT OFFSET(23) NUMBITS(1) [],

        /// Whether the CPU is halted.
        ///
        /// NOTE: This bit is read-only.
        HALT OFFSET(22) NUMBITS(1) [],

        /// `pmc2flow_ack` signal.
        ///
        /// NOTE: This bit is read-only.
        P2F_ACK OFFSET(21) NUMBITS(1) [],

        /// `flow2pmc_pwrup` signal.
        ///
        /// NOTE: This bit is read-only.
        F2P_PWRUP OFFSET(20) NUMBITS(1) [],

        /// `flow2pmc_req` valid.
        ///
        /// NOTE: This bit is read-only.
        F2P_REQ OFFSET(19) NUMBITS(1) [],

        /// Whether a Reset was requested for MPCore.
        ///
        /// NOTE: This bit is read-only.
        F2C_MPCORE_RST OFFSET(17) NUMBITS(1) [],

        /// Whether Interrupt is Active.
        ///
        /// NOTE: To clear this bit, write `1`.
        INTR_FLAG OFFSET(15) NUMBITS(1) [],

        /// Whether Event is Active.
        ///
        /// NOTE: To clear this bit, write `1`.
        EVENT_FLAG OFFSET(14) NUMBITS(1) [],

        /// Specifies what to power off, if `ENABLE` is set.
        ENABLE_EXT OFFSET(12) NUMBITS(2) [],

        /// All cores indicated in bitmap must be in `STANDBY_WFI` before CPU power-gating.
        WAIT_WFI_BITMAP OFFSET(8) NUMBITS(4) [],

        /// All cores indicated in bitmap must be in `STANDBY_WFE` before CpU power-gating.
        WAIT_WFE_BITMAP OFFSET(4) NUMBITS(4) [],

        /// If set, CPU is powered up immediately.
        IMMEDIATE_WAKE OFFSET(3) NUMBITS(1) [],

        /// Switches the active cluster when all indicated CPUs reach `STANDBY_WFI`.
        SWITCH_CLUSTER OFFSET(2) NUMBITS(1) [],

        /// Generates an event when the Flow Controller exits the halted state.
        EVENT_ENABLE OFFSET(1) NUMBITS(1) [],

        /// Power-Gate Enable.
        ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `FLOW_CTLR_COP_CSR_0` register.
    pub FLOW_CTLR_COP_CSR_0 [
        /// Whether Interrupt is Active.
        ///
        /// NOTE: To clear this bit, write `1` to it.
        INTR_FLAG OFFSET(15) NUMBITS(1) []
    ],

    /// Bitfields of the `FLOW_CTLR_XRQ_EVENTS_0` register.
    pub FLOW_CTLR_XRQ_EVENTS_0 [
        /// Enables event triggering for the corresponding bit in GPIO port D.
        XRQ_D7_XRQ_D0 OFFSET(24) NUMBITS(8) [],

        /// Enables event triggering for the corresponding bit in GPIO port C.
        XRQ_C7_XRQ_C0 OFFSET(16) NUMBITS(8) [],

        /// Enables event triggering for the corresponding bit in GPIO port B.
        XRQ_B7_XRQ_B0 OFFSET(8) NUMBITS(8) [],

        /// Enables event triggering for the corresponding bit in GPIO port A.
        XRQ_A7_XRQ_A0 OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `FLOW_CTLR_CLUSTER_CONTROL_0` register.
    pub FLOW_CTLR_CLUSTER_CONTROL_0 [
        /// The post-switch delay.
        POST_SWITCH_DELAY OFFSET(20) NUMBITS(12) [],

        /// The pre-switch delay.
        PRE_SWITCH_DELAY OFFSET(8) NUMBITS(12) [],

        /// Whether Cluster Control is active.
        ACTIVE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `FLOW_CTLR_CPU_PWR_CSR_0` register.
    pub FLOW_CTLR_CPU_PWR_CSR_0 [
        DBG_STS_EN OFFSET(15) NUMBITS(1) [],

        DBG_C1NC_STS OFFSET(13) NUMBITS(2) [],

        DBG_C0NC_STS OFFSET(11) NUMBITS(2) [],

        DBG_RAIL_STS OFFSET(9) NUMBITS(1) [],

        /// Whether to initialize last-CPU PG only when the other CPUs are already power-gated.
        CPU_RG_CFG OFFSET(8) NUMBITS(1) [],

        /// The current status of the C1NC partition.
        C1NC_STS OFFSET(6) NUMBITS(2) [
            PartitionOff = 0,
            PgInProgress = 1,
            PuInProgress = 2,
            PartitionOn = 3
        ],

        /// The current status of the C0NC partition.
        C0NC_STS OFFSET(4) NUMBITS(2) [
            PartitionOff = 0,
            PgInProgress = 1,
            PuInProgress = 2,
            PartitionOn = 3
        ],

        /// Whether C0NC/C1NC/CRAIL domains should be power-gated.
        USE_FLOW_STS OFFSET(3) NUMBITS(1) [],

        /// The current status of the CPU rail.
        RAIL_STS OFFSET(1) NUMBITS(2) [
            RailOff = 0,
            RgInProgress = 1,
            RuInProgress = 2,
            RailOn = 3
        ],

        /// Issues a CPU rail power-on request.
        RAIL_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `FLOW_CTLR_MPID_0` register.
    pub FLOW_CTLR_MPID_0 [
        /// CPU-ID of CPULP.
        CPU_ID OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `FLOW_CTLR_RAM_REPAIR_0` register.
    pub FLOW_CTLR_RAM_REPAIR_0 [
        /// Repair done (acknowledge) from repair logic.
        ///
        /// NOTE: These bits are read-only.
        DBG_STS OFFSET(16) NUMBITS(8) [],

        /// Repair request for individual segments.
        DBG_REQ OFFSET(8) NUMBITS(8) [],

        /// Debug enable to be able to repair individual segments of the Cluster0 repair chain.
        DBG_EN OFFSET(3) NUMBITS(1) [],

        /// RAM repair bypass enable.
        BYPASS_EN OFFSET(2) NUMBITS(1) [],

        /// Indicates Cluster repair chain status.
        ///
        /// NOTE: This bit is read-only.
        STS OFFSET(1) NUMBITS(1) [],

        /// Initializes a Cluster0 RAM repair request to all segments in parallel.
        REQ OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `FLOW_CTLR_FLOW_DBG_SEL_0` register.
    pub FLOW_CTLR_FLOW_DBG_SEL_0 [
        /// Current state of Rail Gate state machine.
        RG_PWR_STATE OFFSET(18) NUMBITS(2) [],

        /// Current state of Rail Ungate state machine.
        RU_PWR_STATE OFFSET(16) NUMBITS(2) [],

        /// Current state of Non CPU Power Gate state machine.
        NC_PG_PWR_STATE OFFSET(14) NUMBITS(2) [],

        /// Current state of Non CPU Power Ungate state machine.
        NC_PU_PWR_STATE OFFSET(12) NUMBITS(2) [],

        /// Activity selection which would be counted by `CNT1`.
        CNT1_SEL OFFSET(8) NUMBITS(4) [
            Idle = 0,
            Cpu0Pg = 1,
            Cpu1Pg = 2,
            Cpu2Pg = 3,
            Cpu3Pg = 4,
            Cpu0123Pg = 5,
            CpulpPg = 6,
            C0ncPg = 7,
            C1ncPg = 8,
            CrailOff = 9,
            C0ToC1Switch = 10,
            C1ToC0Switch = 11,
            HvcEntry = 12,
            RetentionEntry = 13
        ],

        /// Actibity selection which would be counted by `CNT0`.
        CNT0_SEL OFFSET(0) NUMBITS(4) [
            Idle = 0,
            Cpu0Pg = 1,
            Cpu1Pg = 2,
            Cpu2Pg = 3,
            Cpu3Pg = 4,
            Cpu0123Pg = 5,
            CpulpPg = 6,
            C0ncPg = 7,
            C1ncPg = 8,
            CrailOff = 9,
            C0ToC1Switch = 10,
            C1ToC0Switch = 11,
            HvcEntry = 12,
            RetentionEntry = 13
        ]
    ],

    /// Bitfields of the `FLOW_CTLR_FLOW_DBG_CNT0_0` register.
    pub FLOW_CTLR_FLOW_DBG_CNT0_0 [
        /// Activity count value, based on `CNT0_SEL`.
        VALUE OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `FLOW_CTLR_FLOW_DBG_CNT1_0` register.
    pub FLOW_CTLR_FLOW_DBG_CNT1_0 [
        /// Activity count value, based on `CNT1_SEL`.
        VALUE OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `FLOW_CTLR_FLOW_DBG_QUAL_0` register.
    pub FLOW_CTLR_FLOW_DBG_QUAL_0 [
        /// Qualifier for legacy IRQ.
        IRQ2CCPLEX_ENABLE OFFSET(29) NUMBITS(1) [],

        /// Qualifier for legacy FIQ.
        FIQ2CCPLEX_ENABLE OFFSET(28) NUMBITS(1) [],

        /// CCPLEX AXICIF/MCCIF clock gating disable.
        AXICIF_CG_DIS OFFSET(24) NUMBITS(2) [],

        /// CPU PWRUPREQ qualifier.
        PWRUPREQ_QUAL OFFSET(16) NUMBITS(5) [],

        /// CPU NOPWRDWN qualifier.
        NOPWRDWN_QUAL OFFSET(8) NUMBITS(5) [],

        /// CPU DBGPWRDNREQ qualifier.
        PWRDNREQ_QUAL OFFSET(0) NUMBITS(5) []
    ],

    /// Bitfields of the `FLOW_CTLR_FLOW_CTLR_SPARE_0` register.
    pub FLOW_CTLR_FLOW_CTLR_SPARE_0 [
        /// The high bits of this spare register.
        SPARE_HI OFFSET(16) NUMBITS(16) [],

        /// The low bits of this spare register.
        SHARE_LOW OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `FLOW_CTLR_FC_SEQUENCE_INTERCEPT_0` register.
    pub FLOW_CTLR_FC_SEQUENCE_INTERCEPT_0 [
        /// If set, generates an interrupt to LIC before starting HVC sequence.
        INTERCEPT_HVC_ENABLE OFFSET(21) NUMBITS(1) [],

        /// If set, generates an interrupt to LIC after HVC state has been entered.
        INTERCEPT_ENTRY_CC4_ENABLE OFFSET(20) NUMBITS(1) [],

        INTERCEPT_ENTRY_PG_NONCPU_ENABLE OFFSET(19) NUMBITS(1) [],

        INTERCEPT_EXIT_PG_NONCPU_ENABLE OFFSET(18) NUMBITS(1) [],

        INTERCEPT_ENTRY_RG_CPU_ENABLE OFFSET(17) NUMBITS(1) [],

        INTERCEPT_EXIT_RG_CPU_ENABLE OFFSET(16) NUMBITS(1) [],

        INTERCEPT_ENTRY_PG_CORE0_ENABLE OFFSET(15) NUMBITS(1) [],

        INTERCEPT_EXIT_PG_CORE0_ENABLE OFFSET(14) NUMBITS(1) [],

        INTERCEPT_ENTRY_PG_CORE1_ENABLE OFFSET(13) NUMBITS(1) [],

        INTERCEPT_EXIT_PG_CORE1_ENABLE OFFSET(12) NUMBITS(1) [],

        INTERCEPT_ENTRY_PG_CORE2_ENABLE OFFSET(11) NUMBITS(1) [],

        INTERCEPT_EXIT_PG_CORE2_ENABLE OFFSET(10) NUMBITS(1) [],

        INTERCEPT_ENTRY_PG_CORE3_ENABLE OFFSET(9) NUMBITS(1) [],

        INTERCEPT_EXIT_OG_CORE3_ENABLE OFFSET(8) NUMBITS(1) [],

        /// Indicates an FC interrupt pending before starting non-CPU power-gating/ungating.
        INTERRUPT_PENDING_NONCPU OFFSET(7) NUMBITS(1) [],

        /// Indicates an FC interrupt pending before starting CPU rail-gating/ungating.
        INTERRUPT_PENDING_CRAIL OFFSET(6) NUMBITS(1) [],

        /// Indicates an FC interrupt pending from core0.
        INTERRUPT_PENDING_CORE0 OFFSET(5) NUMBITS(1) [],

        /// Indicates an FC interrupt pending from core1.
        INTERRUPT_PENDING_CORE1 OFFSET(4) NUMBITS(1) [],

        /// Indicates an FC interrupt pending from core2.
        INTERRUPT_PENDING_CORE2 OFFSET(3) NUMBITS(1) [],

        /// Indicates an FC interrupt pending from core3.
        INTERRUPT_PENDING_CORE3 OFFSET(2) NUMBITS(1) [],

        /// Indicates an interrupt was issued by FC to LIC after HVC state has entered.
        CC4_INTERRUPT_PENDING OFFSET(1) NUMBITS(1) [],

        /// Indicates an interrupt was issued by FC to LIC before starting/exiting HVC sequence.
        HVC_INTERRUPT_PENDING OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `FLOW_CTLR_CC4_HVC_CONTROL_0` register.
    pub FLOW_CTLR_CC4_HVC_CONTROL_0 [
        /// The programmable time threshold required to enter HVC.
        HVC_RES_TIME_THRESHOLD OFFSET(3) NUMBITS(29) [],

        /// Indicates whether HVC entry/exit sequence has started/completed.
        ///
        /// NOTE: These bits are read-only.
        CC4_HVC_STS OFFSET(1) NUMBITS(2) [
            Cc4HvcNotEntered = 0,
            Cc4HvcEntryInProgress = 1,
            Cc4HvcExitInProgress = 2,
            Cc4HvcEntered = 3
        ],

        /// Enables HVC.
        CC4_HVC_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `FLOW_CTLR_CC4_RETENTION_CONTROL_0` register.
    pub FLOW_CTLR_CC4_RETENTION_CONTROL_0 [
        /// The programmable threshold required to enter Retention in units or microseconds.
        RET_RES_TIME_THRESHOLD OFFSET(3) NUMBITS(29) []
    ],

    /// Bitfields of the `FLOW_CTLR_CC4_FC_STATUS_0` register.
    pub FLOW_CTLR_CC4_FC_STATUS_0 [
        /// Allows LIC interrupts to act as Retention wake events.
        ///
        /// NOTE: For HVC, this should always be `0`.
        CC4_LIC_INTR_EN OFFSET(2) NUMBITS(1) [],

        /// Prevents the FC from processing wake up requests.
        CC4_WAKE_MASK OFFSET(1) NUMBITS(1) [],

        /// Indicates that an interrupt to FC is pending in HVC.
        ///
        /// NOTE: This bit is read-only.
        INT_STATUS OFFSET(0) NUMBITS(1) [
            Pending = 0,
            NotPending = 1
        ]
    ],

    /// Bitfields of the `FLOW_CTLR_CC4_CORE_<x>_CTRL_0` register.
    pub FLOW_CTLR_CC4_CORE_CTRL_0 [
        /// Initializes a countdown that is used to determine whether to enter HVC or Retention.
        CORE_IDLE_TIMER OFFSET(3) NUMBITS(29) [],

        /// Allows the respective core to enter either CC3, HVC4 or CC4 Retention.
        TIMER_COUNTDOWN_VALID OFFSET(2) NUMBITS(1) [],

        /// Allows the respective core to enter Retention and interrupt the BPMP.
        CORE_RET_ENABLE OFFSET(1) NUMBITS(1) [],

        /// Allows the respective core to enter HVC.
        CORE_HVC_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `FLOW_CTLR_CORE_<x>_IDLE_COUNTER_0` register.
    pub FLOW_CTLR_CORE_IDLE_COUNTER_0 [
        /// Captures the current value of the core's idle countdown counter.
        CORE_IDLE_COUNTER OFFSET(3) NUMBITS(29) []
    ],

    /// Bitfields of the `FLOW_CTLR_CC4_HVC_RETRY_0` register.
    pub FLOW_CTLR_CC4_HVC_RETRY_0 [
        /// Represents the threshold to retry HVC entry after QDENY signals are asserted.
        THRESHOLD OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `FLOW_CTLR_L2FLUSH_TIMEOUT_CNTR_0` register.
    pub FLOW_CTLR_L2FLUSH_TIMEOUT_CNTR_0 [
        /// Set in case of L2FLUSHREQUEST timeout.
        ///
        /// NOTE: Software should write `1` to this bit to clear it.
        STATUS OFFSET(31) NUMBITS(1) [],

        /// L2FLUSH asserts timeout once the counter reaches this value.
        THRESHOLD OFFSET(1) NUMBITS(30) [],

        /// Enables L2FLUSH TIMEOUT counter.
        ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `FLOW_CTLR_L2FLUSH_CONTROL_0` register.
    pub FLOW_CTLR_L2FLUSH_CONTROL_0 [
        /// Issues software L2FLUSH request.
        REQ OFFSET(1) NUMBITS(1) [],

        /// Enables hardware L2FLUSH handshake.
        ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `FLOW_CTLR_BPMP_CLUSTER_CONTROL_0` register.
    pub FLOW_CTLR_BPMP_CLUSTER_CONTROL_0 [
        /// This field can only be written by a secure write.
        ACTIVE_CLUSTER_LOCK OFFSET(2) NUMBITS(1) [],

        /// Written by the BPMP prior to allowing master core initiate CC6.
        CLUSTER_SWITCH_ENABLE OFFSET(1) NUMBITS(1) [],

        /// Written by the BPMP during cluster switch sequence.
        ///
        /// NOTE: `ACTIVE_CLUSTER_LOCK` must be set to `0` to
        /// be able to write to this bit.
        ACTIVE_CLUSTER OFFSET(0) NUMBITS(1) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => pub FLOW_CTLR_HALT_CPU0_EVENTS_0: ReadWrite<u32, FLOW_CTLR_HALT_CPU_EVENTS_0::Register>),
        (0x04 => pub FLOW_CTLR_HALT_COP_EVENTS_0: ReadWrite<u32, FLOW_CTLR_HALT_COP_EVENTS_0::Register>),
        (0x08 => pub FLOW_CTLR_CPU0_CSR_0: ReadWrite<u32, FLOW_CTLR_CPU_CSR_0::Register>),
        (0x0C => pub FLOW_CTLR_COP_CSR_0: ReadWrite<u32, FLOW_CTLR_COP_CSR_0::Register>),
        (0x10 => pub FLOW_CTLR_XRQ_EVENTS_0: ReadWrite<u32, FLOW_CTLR_XRQ_EVENTS_0::Register>),
        (0x14 => pub FLOW_CTLR_HALT_CPU1_EVENTS_0: ReadWrite<u32, FLOW_CTLR_HALT_CPU_EVENTS_0::Register>),
        (0x18 => pub FLOW_CTLR_CPU1_CSR_0: ReadWrite<u32, FLOW_CTLR_CPU_CSR_0::Register>),
        (0x1C => pub FLOW_CTLR_HALT_CPU2_EVENTS_0: ReadWrite<u32, FLOW_CTLR_HALT_CPU_EVENTS_0::Register>),
        (0x20 => pub FLOW_CTLR_CPU2_CSR_0: ReadWrite<u32, FLOW_CTLR_CPU_CSR_0::Register>),
        (0x24 => pub FLOW_CTLR_HALT_CPU3_EVENTS_0: ReadWrite<u32, FLOW_CTLR_HALT_CPU_EVENTS_0::Register>),
        (0x28 => pub FLOW_CTLR_CPU3_CSR_0: ReadWrite<u32, FLOW_CTLR_CPU_CSR_0::Register>),
        (0x2C => pub FLOW_CTLR_CLUSTER_CONTROL_0: ReadWrite<u32, FLOW_CTLR_CLUSTER_CONTROL_0::Register>),
        (0x30 => _reserved0: [ReadWrite<u8>; 0x8]),
        (0x38 => pub FLOW_CTLR_CPU_PWR_CSR_0: ReadWrite<u32, FLOW_CTLR_CPU_PWR_CSR_0::Register>),
        (0x3C => pub FLOW_CTLR_MPID_0: ReadWrite<u32, FLOW_CTLR_MPID_0::Register>),
        (0x40 => pub FLOW_CTLR_RAM_REPAIR_0: ReadWrite<u32, FLOW_CTLR_RAM_REPAIR_0::Register>),
        (0x44 => pub FLOW_CTLR_FLOW_DBG_SEL_0: ReadWrite<u32, FLOW_CTLR_FLOW_DBG_SEL_0::Register>),
        (0x48 => pub FLOW_CTLR_FLOW_DBG_CNT0_0: ReadWrite<u32, FLOW_CTLR_FLOW_DBG_CNT0_0::Register>),
        (0x4C => pub FLOW_CTLR_FLOW_DBG_CNT1_0: ReadWrite<u32, FLOW_CTLR_FLOW_DBG_CNT1_0::Register>),
        (0x50 => pub FLOW_CTLR_FLOW_DBG_QUAL_0: ReadWrite<u32, FLOW_CTLR_FLOW_DBG_QUAL_0::Register>),
        (0x54 => pub FLOW_CTLR_FLOW_CTLR_SPARE_0: ReadWrite<u32, FLOW_CTLR_FLOW_CTLR_SPARE_0::Register>),
        (0x58 => _reserved1: [ReadWrite<u8>; 0x4]),
        (0x5C => pub FLOW_CTLR_FC_SEQUENCE_INTERCEPT_0: ReadWrite<u32, FLOW_CTLR_FC_SEQUENCE_INTERCEPT_0::Register>),
        (0x60 => pub FLOW_CTLR_CC4_HVC_CONTROL_0: ReadWrite<u32, FLOW_CTLR_CC4_HVC_CONTROL_0::Register>),
        (0x64 => pub FLOW_CTLR_CC4_RETENTION_CONTROL_0: ReadWrite<u32, FLOW_CTLR_CC4_RETENTION_CONTROL_0::Register>),
        (0x68 => pub FLOW_CTLR_CC4_FC_STATUS_0: ReadWrite<u32, FLOW_CTLR_CC4_FC_STATUS_0::Register>),
        (0x6C => pub FLOW_CTLR_CC4_CORE0_CTRL_0: ReadWrite<u32, FLOW_CTLR_CC4_CORE_CTRL_0::Register>),
        (0x70 => pub FLOW_CTLR_CC4_CORE1_CTRL_0: ReadWrite<u32, FLOW_CTLR_CC4_CORE_CTRL_0::Register>),
        (0x74 => pub FLOW_CTLR_CC4_CORE2_CTRL_0: ReadWrite<u32, FLOW_CTLR_CC4_CORE_CTRL_0::Register>),
        (0x78 => pub FLOW_CTLR_CC4_CORE3_CTRL_0: ReadWrite<u32, FLOW_CTLR_CC4_CORE_CTRL_0::Register>),
        (0x7C => pub FLOW_CTLR_CORE0_IDLE_COUNTER_0: ReadOnly<u32, FLOW_CTLR_CORE_IDLE_COUNTER_0::Register>),
        (0x80 => pub FLOW_CTLR_CORE1_IDLE_COUNTER_0: ReadOnly<u32, FLOW_CTLR_CORE_IDLE_COUNTER_0::Register>),
        (0x84 => pub FLOW_CTLR_CORE2_IDLE_COUNTER_0: ReadOnly<u32, FLOW_CTLR_CORE_IDLE_COUNTER_0::Register>),
        (0x88 => pub FLOW_CTLR_CORE3_IDLE_COUNTER_0: ReadOnly<u32, FLOW_CTLR_CORE_IDLE_COUNTER_0::Register>),
        (0x8C => pub FLOW_CTLR_CC4_HVC_RETRY_0: ReadWrite<u32, FLOW_CTLR_CC4_HVC_RETRY_0::Register>),
        (0x90 => pub FLOW_CTLR_L2FLUSH_TIMEOUT_CNTR_0: ReadWrite<u32, FLOW_CTLR_L2FLUSH_TIMEOUT_CNTR_0::Register>),
        (0x94 => pub FLOW_CTLR_L2FLUSH_CONTROL_0: ReadWrite<u32, FLOW_CTLR_L2FLUSH_CONTROL_0::Register>),
        (0x98 => pub FLOW_CTLR_BPMP_CLUSTER_CONTROL_0: ReadWrite<u32, FLOW_CTLR_BPMP_CLUSTER_CONTROL_0::Register>),
        (0x9C => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x9C]);
