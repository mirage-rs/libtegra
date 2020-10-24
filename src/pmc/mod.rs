//! Driver for the Tegra X1 Power Management Controller.
//!
//! See Chapter 12 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! The Power Management Controller (PMC) block interacts with an external Power Management
//! Integrated Circuit (PMIC) through sideband signals. The PMC controls the entry and exit
//! of the system from different sleep modes. It provides power-gating controllers for SoC
//! and CPU power partitions, except for the Maxwell GPU power partition. The PMC also
//! provides deep power down (DPD) mode control for pads, and scratch storage to save some
//! of the context during sleep modes (when the CPU and/or SoC power rails are off).
//!
//! Sleep (LP1) and Deep Sleep (LP0) require specific logic to maintain some states and control
//! the power domains, including signaling to the external PMIC to provide power to the main
//! logic in Tegra X1 devices. All this logic is centralized in the PMC block.
//!
//! # Power Gating/Ungating
//!
//! This module exposes an implementation of the power-gating logic for
//! certain [`Partition`]s through the [`powergate_partition`] function:
//!
//! ```no_run
//! use libtegra::{car::Clock, pmc};
//!
//! /// Attempts to bring up all the Serial Output Resources.
//! fn bring_up_sors() -> Result<(), ()> {
//!     // Power-ungate SOR.
//!     pmc::powergate_partition(pmc::Partition::SOR, false)?;
//!
//!     // Bring up clocks.
//!     Clock::SOR_SAFE.enable();
//!     Clock::SOR0.enable();
//!     Clock::SOR1.enable();
//!     Clock::DPAUX.enable();
//!     Clock::DPAUX1.enable();
//!     Clock::MIPI_CAL.enable();
//!     Clock::CSI.enable();
//!     Clock::DSI.enable();
//!     Clock::DSIB.enable();
//!
//!     // Power-gate SOR.
//!     pmc::powergate_partition(pmc::Partition::SOR, true)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! [`Partition`]: enum.Partition.html
//! [`powergate_partition`]: fn.powergate_partition.html

use crate::timer::usleep;

pub use registers::*;

mod registers;

enum_from_primitive! {
    /// Enumeration over power-gated PMC partitions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum Partition {
        CRAIL = 0,
        VideoEncode = 2,
        PCX = 3,
        MPEGEncode = 6,
        SAX = 8,
        CE1 = 9,
        CE2 = 10,
        CE3 = 11,
        CE0 = 14,
        C0NC = 15,
        SOR = 17,
        DIS = 18,
        DISB = 19,
        XUSBA = 20,
        XUSBB = 21,
        XUSBC = 22,
        VIC = 23,
        IRAM = 24,
        NVDEC = 25,
        NVJPG = 26,
        AUD = 27,
        DFD = 28,
        VE2 = 29,
    }
}

/// Toggles power gating for a given partition.
pub fn powergate_partition(partition: Partition, enable: bool) -> Result<(), ()> {
    let pmc = unsafe { &*REGISTERS };

    let partition_id = partition as u32;
    let partition_mask = 1 << partition_id;
    let desired_state = (enable as u32) << partition_id;

    // Check if the partition already has the desired state.
    if (pmc.APBDEV_PMC_PWRGATE_STATUS_0.get() & partition_mask) == desired_state {
        return Ok(());
    }

    // Wait for the power gating controller to enter idle state.
    let mut i = 5001;
    while (pmc.APBDEV_PMC_PWRGATE_TOGGLE_0.get() & 0x100) != 0 {
        usleep(1);
        i -= 1;

        if i < 1 {
            return Err(());
        }
    }

    // Toggle power gating.
    pmc.APBDEV_PMC_PWRGATE_TOGGLE_0.set(partition_id | 0x100);

    // Wait for the changes to take effect.
    i = 5001;
    while i > 0 {
        if (pmc.APBDEV_PMC_PWRGATE_STATUS_0.get() & partition_mask) == desired_state {
            return Ok(());
        }

        usleep(1);
        i -= 1;
    }

    Err(())
}
