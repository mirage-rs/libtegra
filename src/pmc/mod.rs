use crate::timer::usleep;

pub use registers::*;

mod registers;

// TODO: Docs

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
///
/// # Example
///
/// ```no_run
/// use libtegra::pmc;
///
/// pmc::powergate_partition(pmc::Partition::SOR, true)
///     .expect("Failed to power gate SOR!");
/// ```
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
