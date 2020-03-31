//! Driver for the Tegra X1 Memory Controller.
//!
//! See Chapter 18 of the Tegra X1 Technical Reference Manual
//! for details.

use crate::{car, timer::usleep};

pub use registers::*;

mod registers;

/// Configures the Memory Controller TSEC carveout.
pub fn config_tsec_carveout(bom: u32, size_mb: u32, lock: bool) {
    let controller = unsafe { &*REGISTERS };

    controller.MC_SEC_CARVEOUT_BOM_0.set(bom);
    controller.MC_SEC_CARVEOUT_SIZE_MB_0.set(size_mb);

    if lock {
        controller.MC_SEC_CARVEOUT_REG_CTRL_0.set(1);
    }
}

/// Configures the Memory Controller carveout.
pub fn config_carveout() {
    let controller = unsafe { &*REGISTERS };

    controller.MC_VIDEO_PROTECT_GPU_OVERRIDE_0_0.set(1);
    controller.MC_VIDEO_PROTECT_GPU_OVERRIDE_1_0.set(0);
    controller.MC_VIDEO_PROTECT_BOM_0.set(0);
    controller.MC_VIDEO_PROTECT_SIZE_MB_0.set(0);
    controller.MC_VIDEO_PROTECT_REG_CTRL_0.set(1);

    config_tsec_carveout(0, 0, true);

    controller.MC_MTS_CARVEOUT_BOM_0.set(0);
    controller.MC_MTS_CARVEOUT_SIZE_MB_0.set(0);
    controller.MC_MTS_CARVEOUT_ADR_HI_0.set(0);
    controller.MC_MTS_CARVEOUT_REG_CTRL_0.set(1);

    controller.MC_SECURITY_CARVEOUT1_BOM_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_BOM_HI_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_SIZE_128KB_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_CA0_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_CA1_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_CA2_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_CA3_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_CA4_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_CFIA0_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_CFIA1_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_CFIA2_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_CFIA3_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_CFIA4_0.set(0);
    controller.MC_SECURITY_CARVEOUT1_CFG0_0.set(0x4000006);

    controller.MC_SECURITY_CARVEOUT3_BOM_0.set(0);
    controller.MC_SECURITY_CARVEOUT3_BOM_HI_0.set(0);
    controller.MC_SECURITY_CARVEOUT3_SIZE_128KB_0.set(0);
    controller.MC_SECURITY_CARVEOUT3_CA0_0.set(0);
    controller.MC_SECURITY_CARVEOUT3_CA1_0.set(0);
    controller.MC_SECURITY_CARVEOUT3_CA2_0.set(0x3000000);
    controller.MC_SECURITY_CARVEOUT3_CA3_0.set(0);
    controller.MC_SECURITY_CARVEOUT3_CA4_0.set(0x300);
    controller.MC_SECURITY_CARVEOUT3_CFIA0_0.set(0);
    controller.MC_SECURITY_CARVEOUT3_CFIA1_0.set(0);
    controller.MC_SECURITY_CARVEOUT3_CFIA2_0.set(0);
    controller.MC_SECURITY_CARVEOUT3_CFIA3_0.set(0);
    controller.MC_SECURITY_CARVEOUT3_CFIA4_0.set(0);
    controller.MC_SECURITY_CARVEOUT3_CFG0_0.set(0x4401E7E);

    controller.MC_SECURITY_CARVEOUT4_BOM_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_BOM_HI_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_SIZE_128KB_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_CA0_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_CA1_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_CA2_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_CA3_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_CA4_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_CFIA0_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_CFIA1_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_CFIA2_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_CFIA3_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_CFIA4_0.set(0);
    controller.MC_SECURITY_CARVEOUT4_CFG0_0.set(0x8F);

    controller.MC_SECURITY_CARVEOUT5_BOM_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_BOM_HI_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_SIZE_128KB_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_CA0_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_CA1_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_CA2_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_CA3_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_CA4_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_CFIA0_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_CFIA1_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_CFIA2_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_CFIA3_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_CFIA4_0.set(0);
    controller.MC_SECURITY_CARVEOUT5_CFG0_0.set(0x8F);
}

/// Finalizes the Memory Controller carveout configuration.
pub fn finalize_carveout() {
    let controller = unsafe { &*REGISTERS };

    controller.MC_SECURITY_CARVEOUT2_BOM_0.set(0x8002_0000);
    controller.MC_SECURITY_CARVEOUT2_BOM_HI_0.set(0);
    controller.MC_SECURITY_CARVEOUT2_SIZE_128KB_0.set(2);
    controller.MC_SECURITY_CARVEOUT2_CA0_0.set(0);
    controller.MC_SECURITY_CARVEOUT2_CA1_0.set(0);
    controller.MC_SECURITY_CARVEOUT2_CA2_0.set(0x3000000);
    controller.MC_SECURITY_CARVEOUT2_CA3_0.set(0);
    controller.MC_SECURITY_CARVEOUT2_CA4_0.set(0x300);
    controller.MC_SECURITY_CARVEOUT2_CFIA0_0.set(0);
    controller.MC_SECURITY_CARVEOUT2_CFIA1_0.set(0);
    controller.MC_SECURITY_CARVEOUT2_CFIA2_0.set(0);
    controller.MC_SECURITY_CARVEOUT2_CFIA3_0.set(0);
    controller.MC_SECURITY_CARVEOUT2_CFIA4_0.set(0);
    controller.MC_SECURITY_CARVEOUT2_CFG0_0.set(0x440167E);
}

/// Enables AHB redirecting.
pub fn enable_ahb_redirect() {
    let controller = unsafe { &*REGISTERS };
    let car = unsafe { &*car::REGISTERS };

    car.CLK_RST_CONTROLLER_LVL2_CLK_GATE_OVRD_0.set(
        (car.CLK_RST_CONTROLLER_LVL2_CLK_GATE_OVRD_0.get() & 0xFFF7_FFFF) | 0x80000
    );

    controller.MC_IRAM_BOM_0.set(0x4000_0000);
    controller.MC_IRAM_TOM_0.set(0x4003_F000);
}

/// Disables AHB redirecting.
pub fn disable_ahb_redirect() {
    let controller = unsafe { &*REGISTERS };
    let car = unsafe { &*car::REGISTERS };

    controller.MC_IRAM_BOM_0.set(0xFFFF_F000);
    controller.MC_IRAM_TOM_0.set(0);

    car.CLK_RST_CONTROLLER_LVL2_CLK_GATE_OVRD_0.set(
        car.CLK_RST_CONTROLLER_LVL2_CLK_GATE_OVRD_0.get() & 0xFFF7_FFFF
    );
}

/// Enables the Memory Controller.
pub fn enable_mc() {
    let car = unsafe { &*car::REGISTERS };

    // Set EMC clock source.
    car.CLK_RST_CONTROLLER_CLK_SOURCE_EMC_0.set(
        (car.CLK_RST_CONTROLLER_CLK_SOURCE_EMC_0.get() & 0x1FFF_FFFF) | 0x4000_0000
    );

    // Enable MIPI CAL clock.
    car.CLK_RST_CONTROLLER_CLK_ENB_H_SET_0.set(
        (car.CLK_RST_CONTROLLER_CLK_ENB_H_SET_0.get() & 0xFDFF_FFFF) | 0x2000000
    );

    // Enable MC clock.
    car.CLK_RST_CONTROLLER_CLK_ENB_H_SET_0.set(
        (car.CLK_RST_CONTROLLER_CLK_ENB_H_SET_0.get() & 0xFFFF_FFFE) | 1
    );

    // Enable EMC DLL clock.
    car.CLK_RST_CONTROLLER_CLK_ENB_X_SET_0.set(
        (car.CLK_RST_CONTROLLER_CLK_ENB_X_SET_0.get() & 0xFFFF_BFFF) | 0x4000
    );

    // Clear EMC and MC reset.
    car.CLK_RST_CONTROLLER_RST_DEV_H_CLR_0.set(0x2000001);
    usleep(5);

    disable_ahb_redirect();
}
