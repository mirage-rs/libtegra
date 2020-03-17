//! Driver for the Flow Controller of the Tegra X1.
//!
//! See Chapter 17 in the Tegra X1 Technical Reference Manual
//! for details.
//!
//! # Description
//!
//! The Flow Controller provides the sequencing of hardware-controlled
//! CPU power states for the main CPU complex and the BPMP.

use register::mmio::ReadWrite;

use crate::{
    memory_map::{CAR, EXCEPTION_VECTORS},
    timer::usleep
};

pub use registers::*;

mod registers;

/// Powers up the given CPU.
pub fn power_cpu(cpu: u32) {
    let controller = unsafe { &*REGISTERS };

    match cpu {
        0 => {
            // Enable CSR for CPU0.
            controller.FLOW_CTLR_CPU0_CSR_0.modify(
                FLOW_CTLR_CPU_CSR_0::ENABLE::SET
            );

            // Dummy read.
            controller.FLOW_CTLR_CPU0_CSR_0.get();

            // Put CPU0 in WaitEvent state and resume on SYSCLK cycle ticks.
            controller.FLOW_CTLR_HALT_CPU0_EVENTS_0.modify(
                FLOW_CTLR_HALT_CPU_EVENTS_0::MODE::FlowModeWaitevent
                + FLOW_CTLR_HALT_CPU_EVENTS_0::SCLK::SET
            );

            // Dummy read.
            controller.FLOW_CTLR_HALT_CPU0_EVENTS_0.get();
        }
        1 => {
            // Enable CSR for CPU1.
            controller.FLOW_CTLR_CPU1_CSR_0.modify(
                FLOW_CTLR_CPU_CSR_0::ENABLE::SET
            );

            // Dummy read.
            controller.FLOW_CTLR_CPU1_CSR_0.get();

            // Put CPU1 in WaitEvent state and resume on SYSCLK cycle ticks.
            controller.FLOW_CTLR_HALT_CPU1_EVENTS_0.modify(
                FLOW_CTLR_HALT_CPU_EVENTS_0::MODE::FlowModeWaitevent
                + FLOW_CTLR_HALT_CPU_EVENTS_0::SCLK::SET
            );

            // Dummy read.
            controller.FLOW_CTLR_HALT_CPU1_EVENTS_0.get();
        }
        2 => {
            // Enable CSR for CPU2.
            controller.FLOW_CTLR_CPU2_CSR_0.modify(
                FLOW_CTLR_CPU_CSR_0::ENABLE::SET
            );

            // Dummy read.
            controller.FLOW_CTLR_CPU2_CSR_0.get();

            // Put CPU2 in WaitEvent state and resume on SYSCLK cycle ticks.
            controller.FLOW_CTLR_HALT_CPU2_EVENTS_0.modify(
                FLOW_CTLR_HALT_CPU_EVENTS_0::MODE::FlowModeWaitevent
                + FLOW_CTLR_HALT_CPU_EVENTS_0::SCLK::SET
            );

            // Dummy read.
            controller.FLOW_CTLR_HALT_CPU2_EVENTS_0.get();
        }
        3 => {
            // Enable CSR for CPU3.
            controller.FLOW_CTLR_CPU3_CSR_0.modify(
                FLOW_CTLR_CPU_CSR_0::ENABLE::SET
            );

            // Dummy read.
            controller.FLOW_CTLR_CPU3_CSR_0.get();

            // Put CPU3 in WaitEvent state and resume on SYSCLK cycle ticks.
            controller.FLOW_CTLR_HALT_CPU3_EVENTS_0.modify(
                FLOW_CTLR_HALT_CPU_EVENTS_0::MODE::FlowModeWaitevent
                + FLOW_CTLR_HALT_CPU_EVENTS_0::SCLK::SET
            );

            // Dummy read.
            controller.FLOW_CTLR_HALT_CPU3_EVENTS_0.get();
        }
        _ => panic!("Invalid CPU given!"),
    }
}

/// Powers down the given CPU.
pub fn deplete_cpu(cpu: u32) {
    let controller = unsafe { &*REGISTERS };

    match cpu {
        0 => {
            // Make CPU0 wait for interrupts before being powered on again.
            controller.FLOW_CTLR_CPU0_CSR_0.modify(
                FLOW_CTLR_CPU_CSR_0::INTR_FLAG::SET
                + FLOW_CTLR_CPU_CSR_0::EVENT_FLAG::SET
                + FLOW_CTLR_CPU_CSR_0::ENABLE::SET
                + FLOW_CTLR_CPU_CSR_0::WAIT_WFI_BITMAP.val(cpu)
            );

            // Dummy read.
            controller.FLOW_CTLR_CPU0_CSR_0.get();

            // Put CPU0 in WaitEvent state.
            controller.FLOW_CTLR_HALT_CPU0_EVENTS_0.modify(
                FLOW_CTLR_HALT_CPU_EVENTS_0::MODE::FlowModeWaitevent
            );

            // Dummy read.
            controller.FLOW_CTLR_HALT_CPU0_EVENTS_0.get();

            // Reset the CPU0 countdown timers.
            controller.FLOW_CTLR_CC4_CORE0_CTRL_0.set(0);

            // Dummy read.
            controller.FLOW_CTLR_CC4_CORE0_CTRL_0.get();
        }
        1 => {
            // Make CPU1 wait for interrupts before being powered on again.
            controller.FLOW_CTLR_CPU1_CSR_0.modify(
                FLOW_CTLR_CPU_CSR_0::INTR_FLAG::SET
                + FLOW_CTLR_CPU_CSR_0::EVENT_FLAG::SET
                + FLOW_CTLR_CPU_CSR_0::ENABLE::SET
                + FLOW_CTLR_CPU_CSR_0::WAIT_WFI_BITMAP.val(cpu)
            );

            // Dummy read.
            controller.FLOW_CTLR_CPU1_CSR_0.get();

            // Put CPU1 in WaitEvent state.
            controller.FLOW_CTLR_HALT_CPU1_EVENTS_0.modify(
                FLOW_CTLR_HALT_CPU_EVENTS_0::MODE::FlowModeWaitevent
            );

            // Dummy read.
            controller.FLOW_CTLR_HALT_CPU1_EVENTS_0.get();

            // Reset the CPU1 countdown timers.
            controller.FLOW_CTLR_CC4_CORE1_CTRL_0.set(0);

            // Dummy read.
            controller.FLOW_CTLR_CC4_CORE1_CTRL_0.get();
        }
        2 => {
            // Make CPU2 wait for interrupts before being powered on again.
            controller.FLOW_CTLR_CPU2_CSR_0.modify(
                FLOW_CTLR_CPU_CSR_0::INTR_FLAG::SET
                + FLOW_CTLR_CPU_CSR_0::EVENT_FLAG::SET
                + FLOW_CTLR_CPU_CSR_0::ENABLE::SET
                + FLOW_CTLR_CPU_CSR_0::WAIT_WFI_BITMAP.val(cpu)
            );

            // Dummy read.
            controller.FLOW_CTLR_CPU2_CSR_0.get();

            // Put CPU2 in WaitEvent state.
            controller.FLOW_CTLR_HALT_CPU2_EVENTS_0.modify(
                FLOW_CTLR_HALT_CPU_EVENTS_0::MODE::FlowModeWaitevent
            );

            // Dummy read.
            controller.FLOW_CTLR_HALT_CPU2_EVENTS_0.get();

            // Reset the CPU2 countdown timers.
            controller.FLOW_CTLR_CC4_CORE2_CTRL_0.set(0);

            // Dummy read.
            controller.FLOW_CTLR_CC4_CORE2_CTRL_0.get();
        }
        3 => {
            // Make CPU3 wait for interrupts before being powered on again.
            controller.FLOW_CTLR_CPU3_CSR_0.modify(
                FLOW_CTLR_CPU_CSR_0::INTR_FLAG::SET
                + FLOW_CTLR_CPU_CSR_0::EVENT_FLAG::SET
                + FLOW_CTLR_CPU_CSR_0::ENABLE::SET
                + FLOW_CTLR_CPU_CSR_0::WAIT_WFI_BITMAP.val(cpu)
            );

            // Dummy read.
            controller.FLOW_CTLR_CPU3_CSR_0.get();

            // Put CPU3 in WaitEvent state.
            controller.FLOW_CTLR_HALT_CPU3_EVENTS_0.modify(
                FLOW_CTLR_HALT_CPU_EVENTS_0::MODE::FlowModeWaitevent
            );

            // Dummy read.
            controller.FLOW_CTLR_HALT_CPU3_EVENTS_0.get();

            // Reset the CPU0 countdown timers.
            controller.FLOW_CTLR_CC4_CORE3_CTRL_0.set(0);

            // Dummy read.
            controller.FLOW_CTLR_CC4_CORE3_CTRL_0.get();
        }
        _ => panic!("Invalid CPU given!"),
    }
}

/// Informs the BPMP that the cluster power-up sequence has completed.
pub fn lock_active_cluster() {
    let controller = unsafe { &*REGISTERS };

    // Lock the active cluster.
    controller.FLOW_CTLR_BPMP_CLUSTER_CONTROL_0.modify(
        FLOW_CTLR_BPMP_CLUSTER_CONTROL_0::ACTIVE_CLUSTER_LOCK::SET
    );

    // Dummy read.
    controller.FLOW_CTLR_BPMP_CLUSTER_CONTROL_0.get();
}

/// Powers on the BPMP processor.
pub fn power_bpmp(entrypoint: u32) {
    let controller = unsafe { &*REGISTERS };

    // Halt the BPMP.
    controller.FLOW_CTLR_HALT_COP_EVENTS_0.modify(
        FLOW_CTLR_HALT_COP_EVENTS_0::MODE::FlowModeWaitevent
    );

    // Assert BPMP reset.
    // TODO: Use the CAR register block, when implemented.
    unsafe {
        (*((CAR + 0x300) as *const ReadWrite<u32>)).set(1 << 1);
    }

    // Set reset address (stored in PMC_SCRATCH39).
    let bpmp_exception_reset_vector = unsafe {
        &*((EXCEPTION_VECTORS + 0x200) as *const ReadWrite<u32>)
    };
    bpmp_exception_reset_vector.set(entrypoint);

    while bpmp_exception_reset_vector.get() != entrypoint {
        // Wait until the value change is confirmed.
    }

    // Wait for a short time before de-asserting the reset signal.
    usleep(2);

    // De-assert BPMP reset.
    // TODO: Use the CAR register block, when implemented.
    unsafe {
        (*((CAR + 0x300) as *const ReadWrite<u32>)).set(1 << 1);
    }

    // Un-halt the BPMP.
    controller.FLOW_CTLR_HALT_COP_EVENTS_0.set(0);
}

/// Powers off the BPMP processor.
pub fn deplete_bpmp() {
    let controller = unsafe { &*REGISTERS };

    // Halt the BPMP.
    controller.FLOW_CTLR_HALT_COP_EVENTS_0.modify(
        FLOW_CTLR_HALT_COP_EVENTS_0::MODE::FlowModeWaitevent
    );

    // Assert BPMP reset.
    // TODO: Use the CAR register block, when implemented.
    unsafe {
        (*((CAR + 0x300) as *const ReadWrite<u32>)).set(1 << 1);
    }

    // Clear reset address.
    let bpmp_exception_reset_vector = unsafe {
        &*((EXCEPTION_VECTORS + 0x200) as *const ReadWrite<u32>)
    };
    bpmp_exception_reset_vector.set(0);

    while bpmp_exception_reset_vector.get() != 0 {
        // Wait until the value change is confirmed.
    }
}

/// Enable routing legacy FIQ to the GICD.
pub fn enable_fiq_to_ccplex_routing() {
    let controller = unsafe { &*REGISTERS };

    // Enable passing FIQs to the GICD.
    controller.FLOW_CTLR_FLOW_DBG_QUAL_0.modify(
        FLOW_CTLR_FLOW_DBG_QUAL_0::FIQ2CCPLEX_ENABLE::SET
    );
}

/// Disable routing legacy FIQ to the GICD.
pub fn disable_fiq_to_ccplex_routing() {
    let controller = unsafe { &*REGISTERS };

    // Disable passing FIQs to the GICD.
    controller.FLOW_CTLR_FLOW_DBG_QUAL_0.modify(
        FLOW_CTLR_FLOW_DBG_QUAL_0::FIQ2CCPLEX_ENABLE::CLEAR
    );
}
