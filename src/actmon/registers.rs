use tock_registers::{register_bitfields, register_structs, registers::*};

use crate::memory_map::ACTMON;

/// A pointer to the Activity Monitor registers that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = ACTMON as *const Registers;

register_bitfields! {
    u32,

    /// Bitfields of the `ACTMON_GLB_STATUS_0` register.
    pub ACTMON_GLB_STATUS_0 [
        /// CPU Monitor Interrupt status.
        CPU_INTR OFFSET(31) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// COP Monitor Interrupt status.
        COP_INTR OFFSET(30) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// AHB Monitor Interrupt status.
        AHB_INTR OFFSET(29) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// APB Monitor Interrupt status.
        APB_INTR OFFSET(28) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// CPU Frequency Interrupt status.
        CPU_FREQ_INTR OFFSET(27) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// MC_ALL Interrupt status.
        MCALL_INTR OFFSET(26) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// MC_CPU Interrupt status.
        MCCPU_INTR OFFSET(25) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// CPU monitor active status.
        CPU_MON_ACT OFFSET(15) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// COP monitor active status.
        COP_MON_ACT OFFSET(14) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// AHB monitor active status.
        AHB_MON_ACT OFFSET(13) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// APB monitor active status.
        APB_MON_ACT OFFSET(12) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// CPU Frequency monitor active status.
        CPU_FREQ_MON_ACT OFFSET(10) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// MC_ALL monitor active status.
        MCALL_MON_ACT OFFSET(9) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// MC_CPU monitor active status.
        MCCPU_MON_ACT OFFSET(8) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ]
    ],

    /// Bitfields of the `ACTMON_GLB_PERIOD_CTRL_0` register.
    pub ACTMON_GLB_PERIOD_CTRL_0 [
        /// The sampling period time of the source provider.
        SOURCE OFFSET(8) NUMBITS(1) [
            /// Sampling period time base in milliseconds.
            MSec = 0,
            /// Sampling period time base in microseconds.
            USec = 1
        ],

        /// Sampling period in milliseconds/microseconds.
        SAMPLE_PERIOD OFFSET(0) NUMBITS(8) []
    ],

    /// Bitfields of the `ACTMON_CPU_CTRL_0` register.
    pub ACTMON_CTRL_0 [
        /// Enable Monitor. Set by software to enable sampling. Cleared in one of the
        /// following ways: (a) When software intends to stop the monitor, it can do
        /// so by clearing this field, (b) when the sampling period expires (and we
        /// are not in periodic mode).
        ENB OFFSET(31) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Enable interrupt when consecutive `CONSECUTIVE_UPPER_NUM` upper watermark
        /// breaches are detected.
        CONSECUTIVE_ABOVE_WMARK_EN OFFSET(30) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Enable interrupt when consecutive `CONSECUTIVE_LOWER_NUM` lower watermark
        /// braches are detected.
        CONSECUTIVE_BELOW_WMARK_EN OFFSET(29) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Number `(N+1)` of consecutive upper watermark breaches that need to occur
        /// to raise an interrupt.
        CONSECUTIVE_ABOVE_WMARK_NUM OFFSET(26) NUMBITS(3) [
            Disable = 0,
            Enable = 1
        ],

        /// Number `(N+1)` of consecutive lower watermark breaches that need to occur
        /// to raise an interrupt.
        CONSECUTIVE_BELOW_WMARK_NUM OFFSET(23) NUMBITS(3) [
            Disable = 0,
            Enable = 1
        ],

        /// Enable interrupt for number of consecutive lower watermark breaches that
        /// need to occur to raise an interrupt.
        WHEN_OVERFLOW_EN OFFSET(22) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Enable interrupt when AVG value is above its upper watermark value.
        AVG_ABOVE_WMARK_EN OFFSET(21) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Enable interrupt when AVG value is below its lower watermark value.
        AVG_BELOW_WMARK_EN OFFSET(20) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Enable interrupt at the end of sample period.
        AT_END_EN OFFSET(19) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Enable periodic mode. Sample for one sample period or periodic sampling.
        ENB_PERIODIC OFFSET(18) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],

        /// Variable for IIR filter. Default is 6, which translates to `2^(K+1) = 128`.
        K_VAL OFFSET(10) NUMBITS(3) [],

        /// Selection criteria on parent module for the activity signal.
        ///
        /// **Only ever set this in AHB control registers!**
        AHB_MONITOR_COND OFFSET(4) NUMBITS(6) [
            Cpu = 0,
            Cop = 1,
            Na1 = 2,
            CSite = 3,
            Arc = 4,
            AhbDma = 5,
            Usb = 6,
            ApbDma = 7,
            Na2 = 8,
            Na3 = 9,
            Na4 = 10,
            Na5 = 11,
            Na6 = 12,
            Na7 = 13,
            Se = 14,
            Dds = 15,
            BSea = 16,
            Na8 = 17,
            Usb2 = 18,
            Na9 = 19,
            All = 63
        ],

        /// Selection criteria on parent module for the activity signal.
        ///
        /// **Only ever set this in APB control registers!**
        APB_MONITOR_COND OFFSET(4) NUMBITS(6) [
            Dtv = 0,
            I2c1 = 1,
            I2c2 = 2,
            I2c3 = 3,
            I2c4 = 4,
            Dvc = 5,
            I2c6 = 6,
            Reserved1 = 7,
            Reserved2 = 8,
            Spi1 = 9,
            Spi2 = 10,
            Spi3 = 11,
            Spi4 = 12,
            Spi5 = 13,
            Spi6 = 14,
            QSpi = 15,
            UartA = 16,
            UartB = 17,
            UartC = 18,
            UartD = 19,
            All = 63
        ],

        /// Selection criteria on parent module for type of pulse signal.
        ///
        /// **Only ever set this in AHB control registers!**
        AHB_SAMPLE_COND OFFSET(0) NUMBITS(4) [
            AhbMasterActive = 0,
            AhbMasterSlaveActive = 1,
            AhbDataXfer = 2,
            AhbIdle = 3,
            MasterIdle = 4,
            AhbBusy = 5,
            Disable = 6
        ],

        /// Selection criteria on parent module for type of pulse signal.
        ///
        /// **Only ever set this in APB control registers!**
        APB_SAMPLE_COND OFFSET(0) NUMBITS(4) [
            PSelActive = 0,
            PReadyActive = 1,
            PEnablePSelActive = 2,
            ApbIdle = 3,
            Disable = 4
        ]
    ],

    /// Bitfields of the `ACTMON_CPU_INTR_STATUS_0` register.
    pub ACTMON_INTR_STATUS_0 [
        /// Assert at the end of sampling period, if count value crosses upper watermark
        /// value consecutively for the number of times specified in `CONSECUTIVE_UPPER_NUM`.
        ///
        /// Writing `1` clears this interrupt, `0` has no effect.
        CONSECUTIVE_UPPER OFFSET(31) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// Assert at the end of sampling period, if count value crosses lower watermark
        /// value consecutively for the number of times specified in `CONSECUTIVE_LOWER_NUM`.
        ///
        /// Writing `1` clears this interrupt, `0` has no effect.
        CONSECUTIVE_LOWER OFFSET(30) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// Assert at the end of sampling period, if interrupt at the end of sample
        /// period is enabled.
        ///
        /// Writing `1` clears this interrupt, `0` has no effect.
        AT_END OFFSET(29) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// Assert at the end of sampling period, if there is an overflow.
        ///
        /// Writing `1` clears this interrupt, `0` has no effect.
        WHEN_OVERFLOW OFFSET(26) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// Assert at the end of sampling period, if AVG count value crosses lower AVG
        /// watermark value.
        ///
        /// Writing `1` clears this interrupt, `0` has no effect.
        AVG_BELOW_WMARK OFFSET(25) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ],

        /// Assert at the end of sampling period, if AVG count value crosses upper AVG
        /// watermark value.
        ///
        /// Writing `1` clears this interrupt, `0` has no effect.
        AVG_ABOVE_WMARK OFFSET(24) NUMBITS(1) [
            NoIntr = 0,
            Intr = 1
        ]
    ],

    /// Bitfields of the `ACTMON_APB_CTRL_SAPB_0` register.
    pub ACTMON_APB_CTRL_SAPB_0 [
        /// Selection criteria on parent module for the activity signal.
        ///
        /// Only used for APB/AHB monitor.
        MONITOR_COND_SAPB OFFSET(4) NUMBITS(6) [
            Apb2Jtag = 0,
            Ape = 1,
            Atomics = 2,
            Cec = 3,
            CSite = 4,
            Dds = 5,
            Dp2 = 6,
            Dvfs = 7,
            Emc0 = 8,
            Emc1 = 9,
            EmcB = 10,
            Fuse = 11,
            Hda = 12,
            KFuse = 13,
            La = 14,
            Mc0 = 15,
            Mc1 = 16,
            McB = 17,
            MipiCal = 18,
            Misc = 19,
            PinmuxAux = 20,
            Pmc = 21,
            Pwm = 22,
            Rtc = 23,
            Sata = 24,
            SataAux = 25,
            Sdmmc1 = 26,
            Sdmmc2 = 27,
            Sdmmc3 = 28,
            Sdmmc4 = 29,
            Se = 30,
            SocTherm = 31,
            SecureRegs = 32,
            Stm = 33,
            SysCtr0 = 34,
            SysCtr1 = 35,
            XUsbDev = 36,
            XUsbHost = 37,
            SUxbPadctl = 38,
            All = 63
        ],

        /// Selection criteria on parent module for type of pulse signal.
        ///
        /// Only used for APB/AHB monitor.
        SAMPLE_COND_SAPB OFFSET(0) NUMBITS(4) [
            PSelActive = 0,
            PReadyActive = 1,
            PEnablePSelActive = 2,
            ApbIdle = 3,
            Disable = 4
        ]
    ],

    /// Bitfields of the `ACTMON_HISTOGRAM_CONFIG_0` register.
    pub ACTMON_HISTOGRAM_CONFIG_0 [
        SOURCE OFFSET(12) NUMBITS(4) [
            None = 0,
            Ahb = 1,
            Apb = 2,
            Cop = 3,
            Cpu = 4,
            McAll = 5,
            McCpu = 6,
            CpuFreq = 7,
            Na = 8,
            ApbMmio = 9
        ],

        /// Scaling factor for the idle counter before the value is used to update
        /// histogram bucket.
        SHIFT OFFSET(4) NUMBITS(5) [],

        /// Whether buckets should be incremented when other buckets have saturated.
        STALL_ON_SINGLE_SATURATE OFFSET(3) NUMBITS(1) [
            /// Continue incrementing buckets even when another bucket has saturated.
            False = 0,
            /// Stop incrementing buckets when at least one other bucket has saturated.
            True = 1
        ],

        /// Whether the underflow bucket should be incremented.
        NO_UNDERFLOW_BUCKET OFFSET(2) NUMBITS(1) [
            /// Increase bucket 0 when idle time is less than the minimum value.
            False = 0,
            /// Ignore idle times that are less than the minimum value.
            True = 1
        ],

        /// Whether linear mode should be used to expand buckets.
        LINEAR_MODE OFFSET(1) NUMBITS(1) [
            /// Bucket width increases exponentially with a power of two.
            Disable = 0,
            /// Bucket width is the same for all buckets.
            Enable = 1
        ],

        /// Enable histogram recording.
        ACTIVE OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ]
    ]
}

register_structs! {
    /// Representation of the Activity Montior registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x000 => pub ACTMON_GLB_STATUS_0: ReadOnly<u32, ACTMON_GLB_STATUS_0::Register>),
        (0x004 => pub ACTMON_GLB_PERIOD_CTRL_0: ReadWrite<u32, ACTMON_GLB_PERIOD_CTRL_0::Register>),
        (0x008 => _reserved0),
        (0x080 => pub ACTMON_CPU_CTRL_0: ReadWrite<u32, ACTMON_CTRL_0::Register>),
        (0x084 => pub ACTMON_CPU_UPPER_WMARK_0: ReadWrite<u32>),
        (0x088 => pub ACTMON_CPU_LOWER_WMARK_0: ReadWrite<u32>),
        (0x08C => pub ACTMON_CPU_INIT_AVG_0: ReadWrite<u32>),
        (0x090 => pub ACTMON_CPU_AVG_UPPER_WMARK_0: ReadWrite<u32>),
        (0x094 => pub ACTMON_CPU_AVG_LOWER_WMARK_0: ReadWrite<u32>),
        (0x098 => pub ACTMON_CPU_COUNT_WEIGHT_0: ReadWrite<u32>),
        (0x09C => pub ACTMON_CPU_COUNT_0: ReadOnly<u32>),
        (0x0A0 => pub ACTMON_CPU_AVG_COUNT_0: ReadOnly<u32>),
        (0x0A4 => pub ACTMON_CPU_INTR_STATUS_0: ReadWrite<u32, ACTMON_INTR_STATUS_0::Register>),
        (0x0A8 => _reserved1),
        (0x0C0 => pub ACTMON_COP_CTRL_0: ReadWrite<u32, ACTMON_CTRL_0::Register>),
        (0x0C4 => pub ACTMON_COP_UPPER_WMARK_0: ReadWrite<u32>),
        (0x0C8 => pub ACTMON_COP_LOWER_WMARK_0: ReadWrite<u32>),
        (0x0CC => pub ACTMON_COP_INIT_AVG_0: ReadWrite<u32>),
        (0x0D0 => pub ACTMON_COP_AVG_UPPER_WMARK_0: ReadWrite<u32>),
        (0x0D4 => pub ACTMON_COP_AVG_LOWER_WMARK_0: ReadWrite<u32>),
        (0x0D8 => pub ACTMON_COP_COUNT_WEIGHT_0: ReadWrite<u32>),
        (0x0DC => pub ACTMON_COP_COUNT_0: ReadOnly<u32>),
        (0x0E0 => pub ACTMON_COP_AVG_COUNT_0: ReadOnly<u32>),
        (0x0E4 => pub ACTMON_COP_INTR_STATUS_0: ReadWrite<u32, ACTMON_INTR_STATUS_0::Register>),
        (0x0E8 => _reserved2),
        (0x100 => pub ACTMON_AHB_CTRL_0: ReadWrite<u32, ACTMON_CTRL_0::Register>),
        (0x104 => pub ACTMON_AHB_UPPER_WMARK_0: ReadWrite<u32>),
        (0x108 => pub ACTMON_AHB_LOWER_WMARK_0: ReadWrite<u32>),
        (0x10C => pub ACTMON_AHB_INIT_AVG_0: ReadWrite<u32>),
        (0x110 => pub ACTMON_AHB_AVG_UPPER_WMARK_0: ReadWrite<u32>),
        (0x114 => pub ACTMON_AHB_AVG_LOWER_WMARK_0: ReadWrite<u32>),
        (0x118 => pub ACTMON_AHB_COUNT_WEIGHT_0: ReadWrite<u32>),
        (0x11C => pub ACTMON_AHB_COUNT_0: ReadOnly<u32>),
        (0x120 => pub ACTMON_AHB_AVG_COUNT_0: ReadOnly<u32>),
        (0x124 => pub ACTMON_AHB_INTR_STATUS_0: ReadWrite<u32, ACTMON_INTR_STATUS_0::Register>),
        (0x128 => _reserved3),
        (0x140 => pub ACTMON_APB_CTRL_0: ReadWrite<u32, ACTMON_CTRL_0::Register>),
        (0x144 => pub ACTMON_APB_UPPER_WMARK_0: ReadWrite<u32>),
        (0x148 => pub ACTMON_APB_LOWER_WMARK_0: ReadWrite<u32>),
        (0x14C => pub ACTMON_APB_INIT_AVG_0: ReadWrite<u32>),
        (0x150 => pub ACTMON_APB_AVG_UPPER_WMARK_0: ReadWrite<u32>),
        (0x154 => pub ACTMON_APB_AVG_LOWER_WMARK_0: ReadWrite<u32>),
        (0x158 => pub ACTMON_APB_COUNT_WEIGHT_0: ReadWrite<u32>),
        (0x15C => pub ACTMON_APB_COUNT_0: ReadOnly<u32>),
        (0x160 => pub ACTMON_APB_AVG_COUNT_0: ReadOnly<u32>),
        (0x164 => pub ACTMON_APB_INTR_STATUS_0: ReadWrite<u32, ACTMON_INTR_STATUS_0::Register>),
        (0x168 => pub ACTMON_APB_CTRL_SAPB_0: ReadWrite<u32, ACTMON_APB_CTRL_SAPB_0::Register>),
        (0x16C => _reserved4),
        (0x180 => pub ACTMON_CPU_FREQ_CTRL_0: ReadWrite<u32, ACTMON_CTRL_0::Register>),
        (0x184 => pub ACTMON_CPU_FREQ_UPPER_WMARK_0: ReadWrite<u32>),
        (0x188 => pub ACTMON_CPU_FREQ_LOWER_WMARK_0: ReadWrite<u32>),
        (0x18C => pub ACTMON_CPU_FREQ_INIT_AVG_0: ReadWrite<u32>),
        (0x190 => pub ACTMON_CPU_FREQ_AVG_UPPER_WMARK_0: ReadWrite<u32>),
        (0x194 => pub ACTMON_CPU_FREQ_AVG_LOWER_WMARK_0: ReadWrite<u32>),
        (0x198 => pub ACTMON_CPU_FREQ_COUNT_WEIGHT_0: ReadWrite<u32>),
        (0x19C => pub ACTMON_CPU_FREQ_COUNT_0: ReadOnly<u32>),
        (0x1A0 => pub ACTMON_CPU_FREQ_AVG_COUNT_0: ReadOnly<u32>),
        (0x1A4 => pub ACTMON_CPU_FREQ_INTR_STATUS_0: ReadWrite<u32, ACTMON_INTR_STATUS_0::Register>),
        (0x1A8 => _reserved5),
        (0x1C0 => pub ACTMON_MCALL_CTRL_0: ReadWrite<u32, ACTMON_CTRL_0::Register>),
        (0x1C4 => pub ACTMON_MCALL_UPPER_WMARK_0: ReadWrite<u32>),
        (0x1C8 => pub ACTMON_MCALL_LOWER_WMARK_0: ReadWrite<u32>),
        (0x1CC => pub ACTMON_MCALL_INIT_AVG_0: ReadWrite<u32>),
        (0x1D0 => pub ACTMON_MCALL_AVG_UPPER_WMARK_0: ReadWrite<u32>),
        (0x1D4 => pub ACTMON_MCALL_AVG_LOWER_WMARK_0: ReadWrite<u32>),
        (0x1D8 => pub ACTMON_MCALL_COUNT_WEIGHT_0: ReadWrite<u32>),
        (0x1DC => pub ACTMON_MCALL_COUNT_0: ReadOnly<u32>),
        (0x1E0 => pub ACTMON_MCALL_AVG_COUNT_0: ReadOnly<u32>),
        (0x1E4 => pub ACTMON_MCALL_INTR_STATUS_0: ReadWrite<u32, ACTMON_INTR_STATUS_0::Register>),
        (0x1E8 => _reserved6),
        (0x200 => pub ACTMON_MCCPU_CTRL_0: ReadWrite<u32, ACTMON_CTRL_0::Register>),
        (0x204 => pub ACTMON_MCCPU_UPPER_WMARK_0: ReadWrite<u32>),
        (0x208 => pub ACTMON_MCCPU_LOWER_WMARK_0: ReadWrite<u32>),
        (0x20C => pub ACTMON_MCCPU_INIT_AVG_0: ReadWrite<u32>),
        (0x210 => pub ACTMON_MCCPU_AVG_UPPER_WMARK_0: ReadWrite<u32>),
        (0x214 => pub ACTMON_MCCPU_AVG_LOWER_WMARK_0: ReadWrite<u32>),
        (0x218 => pub ACTMON_MCCPU_COUNT_WEIGHT_0: ReadWrite<u32>),
        (0x21C => pub ACTMON_MCCPU_COUNT_0: ReadOnly<u32>),
        (0x220 => pub ACTMON_MCCPU_AVG_COUNT_0: ReadOnly<u32>),
        (0x224 => pub ACTMON_MCCPU_INTR_STATUS_0: ReadWrite<u32, ACTMON_INTR_STATUS_0::Register>),
        (0x228 => _reserved7),
        (0x300 => pub ACTMON_HISTOGRAM_CONFIG_0: ReadWrite<u32, ACTMON_HISTOGRAM_CONFIG_0::Register>),
        (0x304 => pub ACTMON_HISTOGRAM_CTRL_0: ReadWrite<u32>),
        (0x308 => _reserved8),
        (0x380 => pub ACTMON_HISTOGRAM_DATA_0: [ReadOnly<u32>; 0x20]),
        (0x400 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x400]);
