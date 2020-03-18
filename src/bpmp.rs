//! Abstractions over the Boot and Power Management Processor.
//!
//! See Chapter 41 in the Tegra X1 Technical Reference Manual
//! for details.
//!
//! # Description
//!
//! The Tegra X1 Boot and Power Management Processor  is referred
//! to as BPMP-Lite or BPMP, as it represents a first architectural
//! step towards a dedicated boot and power management processor.
//!
//! The BPMP is an ARM7-based microcontroller used for power
//! management and boot purposes. Outside of boot, the BPMP runs the
//! "BPMP Firmware" (BPMP-FW), which provides various services related
//! to runtime power management.
//!
//! In cold boot, the BPMP-Lite processor runs the boot ROM code as well
//! as a portion of the Boot Loader. As part of cold boot, the BPMP-FW
//! is loaded into DRAM and the CPU triggers the BPMP-Lite processor to
//! begin executing that firmware.
//!
//! ## Delaying Execution
//!
//! Through the [Flow Controller], code execution on the BPMP can be halted
//! for a given delay in milliseconds or microseconds. Though the RTC is
//! more accurate, the main advantage of [`msleep`] and [`usleep`] over the
//! [RTC] is resource-efficiency. These functions are meant to reduce power
//! consumption while sleeping.
//!
//! ```no_run
//! use libtegra::bpmp;
//!
//! // Halt the BPMP for 32 milliseconds.
//! bpmp::msleep(32);
//!
//! // Halt the BPMP for 32.000 microseconds, or 32 milliseconds.
//! bpmp::usleep(32_000);
//! ```
//!
//! ## Manual Halting
//!
//! For many hardware sequences, such as rebooting or powering off, there may be the
//! need for halting the BPMP manually. Said functionality is provided through the
//! [`halt`] function:
//!
//! ```no_run
//! use libtegra::{bpmp, pmc};
//!
//! /// Forcefully reboots the SoC into Recovery Mode.
//! unsafe fn reboot_to_rcm() -> ! {
//!     let pmc = &*pmc::REGISTERS;
//!
//!     // Additional resource cleanup here...
//!     // (BPMP MMU disable, SD unmount, display finalization, ...)
//!
//!     // Reboot into RCM.
//!     pmc.APBDEV_PMC_SCRATCH0_0.set(2);
//!     pmc.APBDEV_PMC_CNTRL_0.set(pmc.APBDEV_PMC_CNTRL_0.get() | (1 << 4));
//!
//!     // Halt the BPMP.
//!     loop {
//!         bpmp::halt();
//!     }
//! }
//!
//! unsafe {
//!     reboot_to_rcm();
//! }
//! ```
//!
//! [Flow Controller]: ../flow
//! [`msleep`]: fn.msleep.html
//! [`usleep`]: fn.usleep.html
//! [RTC]: ../timer
//! [`halt`]: fn.halt.html

use core::cmp::min;

use crate::flow;

/// Sleeps for the given amount of microseconds.
///
/// This function specifically halts the BPMP to reduce
/// power while sleeping.
///
/// NOTE: Though the RTC is more accurate for big values,
/// this function guarantees time + delay.
pub fn usleep(mut microseconds: u32) {
    let controller = unsafe { &*flow::REGISTERS };

    let mut delay;
    while microseconds > 0 {
        // Figure out the delay to use.
        delay = min(microseconds, 0xFF);
        microseconds -= delay;

        // Halt the CPU for the given duration.
        controller.FLOW_CTLR_HALT_COP_EVENTS_0.set(0x4200_0000 | delay);
    }
}

/// Sleeps for the given amount of milliseconds.
///
/// This function specifically halts the BPMP to reduce
/// power while sleeping.
///
/// NOTE: Though the RTC is more accurate for big values,
/// this function guarantees time + delay.
pub fn msleep(mut milliseconds: u32) {
    let controller = unsafe { &*flow::REGISTERS };

    let mut delay;
    while milliseconds > 0 {
        // Figure out the delay to use.
        delay = min(milliseconds, 0xFF);
        milliseconds -= delay;

        // Halt the CPU for the given duration.
        controller.FLOW_CTLR_HALT_COP_EVENTS_0.set(0x4100_0000 | delay);
    }
}

/// Halts the BPMP processor.
pub fn halt() {
    let controller = unsafe { &*flow::REGISTERS };

    controller.FLOW_CTLR_HALT_COP_EVENTS_0.modify(
        flow::FLOW_CTLR_HALT_COP_EVENTS_0::MODE::FlowModeWaitevent
        + flow::FLOW_CTLR_HALT_COP_EVENTS_0::JTAG::SET
    );
}
