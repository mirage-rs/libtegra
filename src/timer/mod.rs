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

pub mod rtc;
pub mod timerus;

/// Reads the current time in seconds.
#[inline]
pub fn get_seconds() -> u32 {
    unsafe { (*rtc::REGISTERS).APBDEV_RTC_SECONDS_0.get() }
}

/// Reads the current time in milliseconds.
#[inline]
pub fn get_milliseconds() -> u32 {
    let rtc = unsafe { &*rtc::REGISTERS };

    rtc.APBDEV_RTC_MILLI_SECONDS_0.get() + (rtc.APBDEV_RTC_SHADOW_SECONDS_0.get() * 1000)
}

/// Reads the current time in microseconds.
#[inline]
pub fn get_microseconds() -> u32 {
    unsafe { (*timerus::REGISTERS).TIMERUS_CNTR_1US_0.get() }
}

/// Sleeps for a given duration in seconds.
#[inline]
pub fn sleep(duration: u32) {
    let start = get_seconds();

    while (get_seconds() - start) <= duration {}
}

/// Sleeps for a given duration in milliseconds.
#[inline]
pub fn msleep(duration: u32) {
    let start = get_milliseconds();

    while (get_milliseconds() - start) <= duration {}
}

/// Sleeps for a given duration in microseconds.
#[inline]
pub fn usleep(duration: u32) {
    let start = get_microseconds();

    while (get_microseconds() - start) <= duration {}
}
