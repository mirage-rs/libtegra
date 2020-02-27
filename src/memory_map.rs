//! Unified Tegra X1 Memory Address Map constants.
//!
//! See Chapter 2 in the Tegra X1 Technical Reference Manual for details.

/// Memory mappings of the PCIe Controller.
pub mod pcie {
    /// Base address of the PCIe registers.
    pub const BASE: u32 = 0x0100_0000;

    /// Start address of the PCIe A1 register block.
    pub const PCIE_A1: u32 = BASE + 0x000000000;
    /// Start address of the PCIe A2 register block.
    pub const PCIE_A2: u32 = BASE + 0x001000000;
    /// Start address of the PCIe A3 register block.
    pub const PCIE_A3: u32 = BASE + 0x1F00_0000;
}

// TODO: IRAM

/// Start address of the Host1x register block.
pub const HOST1X: u32 = 0x5000_0000;

/// Memory mappings of the ARM registers.
pub mod arm {
    /// Base address of the ARM registers.
    pub const BASE: u32 = 0x5004_0000;

    /// Start address of the ARM PERIPHBASE register block.
    pub const PERIPHBASE: u32 = BASE;
    /// Start address of the ARM Interrupt Distributor register block.
    pub const INTERRUPT_DISTRIBUTOR: u32 = BASE + 0x1000;
}

/// Start address of the BPMP-Lite CACHE register block.
pub const BPMP_CACHE: u32 = 0x5004_0000;

// TODO: Interrupt Controllers.

/// Start address of the MSelect register block.
pub const MSELECT: u32 = 0x5006_0000;

/// Start address of the Graphics Host register block.
pub const GRAPHICS_HOST: u32 = 0x5400_0000;

/// Start of the VI register block.
pub const VI: u32 = 0x5408_0000;

/// Start of the CSI register block.
pub const CSI: u32 = 0x5408_0000;

/// Start of the ISP register block.
pub const ISP: u32 = 0x5460_0000;

/// Start of the ISPB register block.
pub const ISPB: u32 = 0x5468_0000;

/// Start of the VII2C register block.
pub const VII2C: u32 = 0x546C_0000;

/// Start of the Display A register block.
pub const DISPLAY_A: u32 = 0x5420_0000;

/// Start of the Display B register block.
pub const DISPLAY_B: u32 = 0x5424_0000;

/// Start of the DSI register block.
pub const DSI: u32 = 0x5430_0000;

/// Start of the VIC register block.
pub const VIC: u32 = 0x5434_0000;

/// Start of the NVENC register block.
pub const NVENC: u32 = 0x544C_0000;

/// Start of the NVDEC register block.
pub const NVDEC: u32 = 0x5448_0000;

/// Start of the NVJPG register block.
pub const NVJPG: u32 = 0x5438_0000;

/// Start of the DSIB register block.
pub const DSIB: u32 = 0x5440_0000;

/// Start of the TSEC register block.
pub const TSEC: u32 = 0x5450_0000;

/// Start of the TSEC2 register block.
pub const TSEC2: u32 = 0x5410_0000;

/// Start of the SOR register block.
pub const SOR: u32 = 0x5454_0000;

/// Start of the SOR1 register block.
pub const SOR1: u32 = 0x5458_0000;

/// Start of the DPAUX register block.
pub const DPAUX: u32 = 0x545C_0000;

/// Start of the DPAUX1 register block.
pub const DPAUX1: u32 = 0x5404_0000;

/// Memory mappings of the GPU Controller.
pub mod gpu {
    /// Base address of the GPU registers.
    pub const BASE: u32 = 0x5700_0000;

    /// Start address of the GPU GART register block.
    pub const GART: u32 = BASE;
}

/// Start of the TMR register block.
pub const TMR: u32 = 0x6000_5000;

/// Start of the Clock and Reset Controller register block.
pub const CAR: u32 = 0x6000_6000;

/// Start of the Flow Controller register block.
pub const FLOW: u32 = 0x6000_7000;

/// Start of the System Registers block.
pub const SYSREG: u32 = 0x6000_C000;

/// Start of the Activity Monitor register block.
pub const ACTMON: u32 = 0x6000_C800;

/// Memory mappings of the GPIO Controller.
pub mod gpio {
    /// Base address of the GPIO registers.
    pub const BASE: u32 = 0x6000_D000;

    /// Start address of the GPIO 1 register block.
    pub const GPIO_1: u32 = BASE + 0x000;
    /// Start address of the GPIO 2 register block.
    pub const GPIO_2: u32 = BASE + 0x100;
    /// Start address of the GPIO 3 register block.
    pub const GPIO_3: u32 = BASE + 0x200;
    /// Start address of the GPIO 4 register block.
    pub const GPIO_4: u32 = BASE + 0x300;
    /// Start address of the GPIO 5 register block.
    pub const GPIO_5: u32 = BASE + 0x400;
    /// Start address of the GPIO 6 register block.
    pub const GPIO_6: u32 = BASE + 0x500;
    /// Start address of the GPIO 7 register block.
    pub const GPIO_7: u32 = BASE + 0x600;
    /// Start address of the GPIO 8 register block.
    pub const GPIO_8: u32 = BASE + 0x700;
}

/// Start of the Exception Vectors register block.
pub const EXCEPTION_VECTORS: u32 = 0x6000_F000;

/// Start of the IPATCH register block.
pub const IPATCH: u32 = 0x6001_DC00;

/// Start of the MISC register block.
pub const MISC: u32 = 0x7000_0000;

/// Memory mappings of the UART Controller.
pub mod uart {
    /// Base address of the UART registers.
    pub const BASE: u32 = 0x7000_6000;

    /// Start address of the UART A register block.
    pub const UART_A: u32 = BASE + 0x000;
    /// Start address of the UART B register block.
    pub const UART_B: u32 = BASE + 0x040;
    /// Start address of the UART C register block.
    pub const UART_C: u32 = BASE + 0x200;
    /// Start address of the UART D register block.
    pub const UART_D: u32 = BASE + 0x300;
    /// Start address of the UART E register block.
    pub const UART_E: u32 = BASE + 0x400;
}

/// Start of the PWM register block.
pub const PWM: u32 = 0x7000_A000;

/// Memory mappings of the I2C Controller.
pub mod i2c {
    /// Base address of the I2C registers.
    pub const BASE: u32 = 0x7000_C000;

    /// Start address of the I2C 1 register block.
    pub const I2C_1: u32 = BASE + 0x0000;
    /// Start address of the I2C 2 register block.
    pub const I2C_2: u32 = BASE + 0x0400;
    /// Start address of the I2C 3 register block.
    pub const I2C_3: u32 = BASE + 0x0500;
    /// Start address of the I2C 4 register block.
    pub const I2C_4: u32 = BASE + 0x0700;
    /// Start address of the I2C 5 register block.
    pub const I2C_5: u32 = BASE + 0x1000;
    /// Start address of the I2C 6 register block.
    pub const I2C_6: u32 = BASE + 0x1100;
}

/// Start of the RTC register block.
pub const RTC: u32 = 0x7000_E000;

/// Start of the PMC register block.
pub const PMC: u32 = 0x7000_E400;

/// Start of the FUSE register block.
pub const FUSE: u32 = 0x7000_F800;

/// Start of the KFUSE register block.
pub const KFUSE: u32 = 0x7000_FC00;

/// Start of the SE register block.
pub const SE: u32 = 0x7001_2000;

/// Start of the MC register block.
pub const MC: u32 = 0x7001_9000;

/// Start of the SATA register block.
pub const SATA: u32 = 0x7002_0000;

/// Start of the SYSCTR0 register block.
pub const SYSCTR0: u32 = 0x700F_0000;

/// Start of the SYSCTR1 register block.
pub const SYSCTR1: u32 = 0x7010_0000;

/// Start of the MIPI CAL register block.
pub const MIPI_CAL: u32 = 0x700E_3000;

/// Start of the DVFS register block.
pub const DVFS: u32 = 0x7011_0000;

/// Start of the CSITE register block.
pub const CSITE: u32 = 0x7200_0000;

/// Start of the TZRAM register block.
pub const TZRAM: u32 = 0x7C01_0000;

/// Start of the USB register block.
pub const USB: u32 = 0x7D00_0000;

/// Start of the USB2 register block.
pub const USB2: u32 = 0x7D00_4000;

/// Start of the boot code.
pub const IROM: u32 = 0x0010_0000;
