//! Driver for the Tegra X1 FUSE Controller.

use crate::{car, pmc, timer::msleep};

pub use registers::*;

mod registers;

/// Initializes the FUSE driver.
pub fn init() {
    let car = unsafe { &*car::REGISTERS };

    // Make FUSE registers visible.
    car.CLK_RST_CONTROLLER_MISC_CLK_ENB_0.set(
        (car.CLK_RST_CONTROLLER_MISC_CLK_ENB_0.get() & 0xEFFF_FFFF) | 0x1000_0000
    );

    // Disable the private key.
    disable_private_key();

    // Disable programming.
    disable_programming();
}

/// Waits for the FUSE driver to enter idle state.
pub fn wait_idle() {
    let controller = unsafe { &*REGISTERS };

    while (controller.FUSE_CTRL.get() & 0xF0000) != 0x40000 {
        // Wait until idle state was entered.
    }
}

/// Disables all FUSE programming.
pub fn disable_programming() {
    let controller = unsafe { &*REGISTERS };

    controller.FUSE_DISABLEREGPROGRAM.set(1);
}

/// Disables access to the FUSE private key.
pub fn disable_private_key() {
    let controller = unsafe { &*REGISTERS };

    controller.FUSE_PRIVATEKEYDISABLE.set(0x10);
}

/// Enables power to the FUSE hardware array.
pub fn enable_power() {
    let pmc = unsafe { &*pmc::REGISTERS };

    // Clear PMC_FUSE_CTRL_PS18_LATCH_CLEAR.
    pmc.APBDEV_PMC_FUSE_CONTROL_0.set(
        pmc.APBDEV_PMC_FUSE_CONTROL_0.get() & !0x200
    );
    msleep(1);
    // Set PMC_FUSE_CTRL_PS18_LATCH_SET.
    pmc.APBDEV_PMC_FUSE_CONTROL_0.set(
        pmc.APBDEV_PMC_FUSE_CONTROL_0.get() | 0x100
    );
    msleep(1);
}

/// Disables power to the FUSE hardware array.
pub fn disable_power() {
    let pmc = unsafe { &*pmc::REGISTERS };

    // Clear PMC_FUSE_CTRL_PS18_LATCH_SET.
    pmc.APBDEV_PMC_FUSE_CONTROL_0.set(
        pmc.APBDEV_PMC_FUSE_CONTROL_0.get() & !0x100
    );
    msleep(1);
    // Set PMC_FUSE_CTRL_PS18_LATCH_CLEAR.
    pmc.APBDEV_PMC_FUSE_CONTROL_0.set(
        pmc.APBDEV_PMC_FUSE_CONTROL_0.get() | 0x200
    );
    msleep(1);
}

/// Reads a FUSE from the hardware array.
pub fn read(address: u32) -> Result<u32, ()> {
    let controller = unsafe { &*REGISTERS };

    // Check if address is in a valid range.
    if address >= 192 {
        return Err(());
    }

    // Wait for idle state.
    wait_idle();

    // Program the target address.
    controller.FUSE_ADDR.set(address);

    // Enable read operation in control register.
    let mut control_value = controller.FUSE_CTRL.get();
    control_value &= !0x3; // Mask the value.
    control_value |= 0x1; // Set READ command.
    controller.FUSE_CTRL.set(control_value);

    // Wait for idle state.
    wait_idle();

    Ok(controller.FUSE_RDATA.get())
}

/// Writes a FUSE in the hardware array.
pub fn write(address: u32, value: u32) -> Result<(), ()> {
    let controller = unsafe { &*REGISTERS };

    // Check if address is in a valid range.
    if address >= 192 {
        return Err(());
    }

    // Wait for idle state.
    wait_idle();

    // Program the target address and value.
    controller.FUSE_ADDR.set(address);
    controller.FUSE_WDATA.set(value);

    // Enable write operation in control register.
    let mut control_value = controller.FUSE_CTRL.get();
    control_value &= !0x3; // Mask the value.
    control_value |= 0x2; // Set WRITE command.
    controller.FUSE_CTRL.set(control_value);

    // Wait for idle state.
    wait_idle();

    Ok(())
}

/// Senses the FUSE hardware array into shadow cache.
pub fn sense() {
    let controller = unsafe { &*REGISTERS };

    // Wait for idle state.
    wait_idle();

    // Enable sense operation in control register.
    let mut control_value = controller.FUSE_CTRL.get();
    control_value &= !0x3; // Mask the value.
    control_value |= 0x3; // Set SENSE_CTRL command.
    controller.FUSE_CTRL.set(control_value);

    // Wait for idle state.
    wait_idle();
}
