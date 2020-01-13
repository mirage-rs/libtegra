//! Abstractions of the timer interfaces provided by the Tegra X1.
//!
//! See `CHAPTER 8: TIMERS` in the Tegra X1 Technical Reference Manual for details.
//!
//! # Overview
//!
//! | Name | Primary Use            | Related Interrupt | Secure |   Freq.  |
//! |------|------------------------|-------------------|--------|----------|
//! | RTC  | Wall Clock Timer       | RTC               | Pseudo | 32kHz    |
//! | TMR  | NVIDIA Generic Timers  | TMR9-0 TMR10-13   | Cfg    | 1MHz OSC |
//! | WDT  | Per CPU/COP            | WDT_<>            | Cfg    | 1Mhz     |
//! | TSC  | Reference for GT       | N/A               | Yes    | OSC      |
//! | GT   | ARM CPU Generic Timers | PPIs*             | Yes    | TSC      |

mod timerus;

/// Reads the current time in microseconds.
#[inline]
pub fn get_microseconds() -> u32 {
    unsafe { (*timerus::REGISTERS).TIMERUS_CNTR_1US_0.get() }
}

/// Sleeps for a given duration in microseconds.
#[inline]
pub fn usleep(duration: u32) {
    let start = get_microseconds();

    while (get_microseconds() - start) <= duration {}
}
