//! Interface to the Tegra X1 Real-Time Clock.
//!
//! See Chapter 12 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! The Real-Time Clock (RTC) module maintains seconds and milliseconds
//! counters, and five alarm registers. The RTC is in the 'always on'
//! power domain, allowing for the counters to run and alarms to trigger
//! when the system is in low-power state. If configured, interrupts
//! triggered by the RTC can cause the system to wake up from a low-power
//! state.

use register::{mmio::*, register_bitfields, register_structs};

use crate::memory_map::RTC;

/// A pointer to the RTC register block that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = RTC as *const Registers;

register_bitfields! {
    u32,

    /// Bitfields of the `APBDEV_RTC_CONTROL_0` register.
    pub APBDEV_RTC_CONTROL_0 [
        /// When set, writes to the SECONDS counter are disabled.
        /// Can only be cleared by resetting the RTC module.
        WR_SEC_CNT OFFSET(0) NUMBITS(1) [
            /// Disables writes.
            Disable = 0,
            /// Enables writes.
            Enable = 1
        ]
    ],

    /// Bitfields of the `APBDEV_RTC_BUSY_0` register.
    pub APBDEV_RTC_BUSY_0 [
        /// This bit is set when a write is initiated on the APB side.
        ///
        /// It is cleared once the write completes in RTC 32 kHz clock domain,
        /// which could be several thousands of APB clocks.
        /// This must be IDLE before a write is initiated.
        ///
        /// NOTE: This bit is only for writes.
        STATUS OFFSET(0) NUMBITS(1) [
            /// IDLE state.
            Idle = 0,
            /// BUSY state.
            Busy = 1
        ]
    ],

    /// Bitfields of the `APBDEV_RTC_SECONDS_0` register.
    pub APBDEV_RTC_SECONDS_0 [
        /// The seconds counter which is incremented every 1000 milliseconds.
        SECONDS OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `APBDEV_RTC_SHADOW_SECONDS_0` register.
    pub APBDEV_RTC_SHADOW_SECONDS_0 [
        /// A snapshot of the SECONDS counter is taken,
        /// whenever there is a read to MILLI_SECONDS register.
        SHADOW_SECONDS OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `APBDEV_RTC_MILLI_SECONDS_0` register.
    pub APBDEV_RTC_MILLI_SECONDS_0 [
        /// Milliseconds counter which is incremented using the Bresenham algorithm.
        MILLI_SECONDS OFFSET(0) NUMBITS(10) []
    ],

    /// Bitfields of the `APBDEV_RTC_SECONDS_ALARM0_0` register.
    pub APBDEV_RTC_SECONDS_ALARM0_0 [
        /// Match value to trigger the alarm.
        SECS_MATCH_VALUE OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `APBDEV_RTC_SECONDS_ALARM1_0` register.
    pub APBDEV_RTC_SECONDS_ALARM1_0 [
        /// Match value to trigger the alarm.
        SECS_MATCH_VALUE OFFSET(0) NUMBITS(32) []
    ],

    /// Bitfields of the `APBDEV_RTC_MILLI_SECONDS_ALARM_0` register.
    pub APBDEV_RTC_MILLI_SECONDS_ALARM_0 [
        /// Milliseconds match value.
        MSEC_MATCH_VALUE OFFSET(0) NUMBITS(10) []
    ],

    /// Bitfields of the `APBDEV_RTC_SECONDS_COUNTDOWN_ALARM_0` register.
    pub APBDEV_RTC_SECONDS_COUNTDOWN_ALARM_0 [
        /// Enable bit for the countdown operation.
        ///
        /// If repeat is not set, this bit is cleared once the internal
        /// counters counts down to specified value.
        ENABLE OFFSET(31) NUMBITS(1) [
            /// Disable the countdown.
            Disable = 0,
            /// Enable the countdown.
            Enable = 1
        ],

        /// Repeat bit for the countdown operation.
        REPEAT OFFSET(30) NUMBITS(1) [
            /// Disable repetition of countdown operations.
            Disable = 0,
            /// Enable repetition of countdown operations.
            Enable = 1
        ],

        /// Number of seconds to countdown.
        VALUE OFFSET(0) NUMBITS(30) []
    ],

    /// Bitfields of the `APBDEV_RTC_MILLI_SECONDS_COUNTDOWN_ALARM_0` register.
    pub APBDEV_RTC_MILLI_SECONDS_COUNTDOWN_ALARM_0 [
        /// Enable bit for the countdown operation.
        ///
        /// If repeat is not set, this bit is cleared once the internal
        /// counters counts down to specified value.
        ENABLE OFFSET(31) NUMBITS(1) [
            /// Disable the countdown.
            Disable = 0,
            /// Enable the countdown.
            Enable = 1
        ],

        /// Repeat bit for the countdown operation.
        REPEAT OFFSET(30) NUMBITS(1) [
            /// Disable repetition of countdown operations.
            Disable = 0,
            /// Enable repetition of countdown operations.
            Enable = 1
        ],

        /// Number of seconds to countdown.
        VALUE OFFSET(0) NUMBITS(30) []
    ],

    /// Bitfields of the `APBDEV_RTC_INTR_MASK_0` register.
    pub APBDEV_RTC_INTR_MASK_0 [
        MSEC_CDN_ALARM OFFSET(4) NUMBITS(1) [],

        SEC_CDN_ALARM OFFSET(3) NUMBITS(1) [],

        MSEC_ALARM OFFSET(2) NUMBITS(1) [],

        SEC_ALARM1 OFFSET(1) NUMBITS(1) [],

        SEC_ALARM0 OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDEV_RTC_INTR_STATUS_0` register.
    pub APBDEV_RTC_INTR_STATUS_0 [
        MSEC_CDN_ALARM OFFSET(4) NUMBITS(1) [],

        SEC_CDN_ALARM OFFSET(3) NUMBITS(1) [],

        MSEC_ALARM OFFSET(2) NUMBITS(1) [],

        SEC_ALARM1 OFFSET(1) NUMBITS(1) [],

        SEC_ALARM0 OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDEV_RTC_INTR_SOURCE_0` register.
    pub APBDEV_RTC_INTR_SOURCE_0 [
        MSEC_CDN_ALARM OFFSET(4) NUMBITS(1) [],

        SEC_CDN_ALARM OFFSET(3) NUMBITS(1) [],

        MSEC_ALARM OFFSET(2) NUMBITS(1) [],

        SEC_ALARM1 OFFSET(1) NUMBITS(1) [],

        SEC_ALARM0 OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDEV_RTC_INTR_SET_0` register.
    pub APBDEV_RTC_INTR_SET_0 [
        MSEC_CDN_ALARM OFFSET(4) NUMBITS(1) [],

        SEC_CDN_ALARM OFFSET(3) NUMBITS(1) [],

        MSEC_ALARM OFFSET(2) NUMBITS(1) [],

        SEC_ALARM1 OFFSET(1) NUMBITS(1) [],

        SEC_ALARM0 OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDEV_RTC_CORRECTION_FACTOR_0` register.
    pub APBDEV_RTC_CORRECTION_FACTOR_0 [
        /// Decrement must be used when the 32 kHz clock is above its nominal frequency.
        DIRECTION OFFSET(9) NUMBITS(1) [
            Decrement = 0,
            Increment = 1
        ],

        PPM OFFSET(0) NUMBITS(9) []
    ]
}

register_structs! {
    /// Representation of the Real-Time Clock registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => pub APBDEV_RTC_CONTROL_0: ReadWrite<u32, APBDEV_RTC_CONTROL_0::Register>),
        (0x04 => pub APBDEV_RTC_BUSY_0: ReadOnly<u32, APBDEV_RTC_BUSY_0::Register>),
        (0x08 => pub APBDEV_RTC_SECONDS_0: ReadWrite<u32, APBDEV_RTC_SECONDS_0::Register>),
        (0x0C => pub APBDEV_RTC_SHADOW_SECONDS_0: ReadOnly<u32, APBDEV_RTC_SHADOW_SECONDS_0::Register>),
        (0x10 => pub APBDEV_RTC_MILLI_SECONDS_0: ReadOnly<u32, APBDEV_RTC_MILLI_SECONDS_0::Register>),
        (0x14 => pub APBDEV_RTC_SECONDS_ALARM0_0: ReadWrite<u32, APBDEV_RTC_SECONDS_ALARM0_0::Register>),
        (0x18 => pub APBDEV_RTC_SECONDS_ALARM1_0: ReadWrite<u32, APBDEV_RTC_SECONDS_ALARM1_0::Register>),
        (0x1C => pub APBDEV_RTC_MILLI_SECONDS_ALARM_0: ReadWrite<u32, APBDEV_RTC_MILLI_SECONDS_ALARM_0::Register>),
        (0x20 => pub APBDEV_RTC_SECONDS_COUNTDOWN_ALARM_0: ReadWrite<u32, APBDEV_RTC_SECONDS_COUNTDOWN_ALARM_0::Register>),
        (0x24 => pub APBDEV_RTC_MILLI_SECONDS_COUNTDOWN_ALARM_0: ReadWrite<u32, APBDEV_RTC_MILLI_SECONDS_COUNTDOWN_ALARM_0::Register>),
        (0x28 => pub APBDEV_RTC_INTR_MASK_0: ReadWrite<u32, APBDEV_RTC_INTR_MASK_0::Register>),
        (0x2C => pub APBDEV_RTC_INTR_STATUS_0: ReadWrite<u32, APBDEV_RTC_INTR_STATUS_0::Register>),
        (0x30 => pub APBDEV_RTC_INTR_SOURCE_0: ReadOnly<u32, APBDEV_RTC_INTR_SOURCE_0::Register>),
        (0x34 => pub APBDEV_RTC_INTR_SET_0: WriteOnly<u32, APBDEV_RTC_INTR_SET_0::Register>),
        (0x38 => pub APBDEV_RTC_CORRECTION_FACTOR_0: ReadWrite<u32, APBDEV_RTC_CORRECTION_FACTOR_0::Register>),
        (0x3C => @END),
    }
}
