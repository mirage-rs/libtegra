use register::mmio::ReadWrite;

use crate::{memory_map::CAR, timer::usleep};

pub const CLK_RST_CONTROLLER_RST_DEVICES_L: u32 = 0x4;
pub const CLK_RST_CONTROLLER_RST_DEVICES_H: u32 = 0x8;
pub const CLK_RST_CONTROLLER_RST_DEVICES_U: u32 = 0xC;
pub const CLK_RST_CONTROLLER_RST_DEVICES_X: u32 = 0x28C;
pub const CLK_RST_CONTROLLER_RST_DEVICES_Y: u32 = 0x2A4;
pub const CLK_RST_CONTROLLER_RST_DEVICES_V: u32 = 0x358;
pub const CLK_RST_CONTROLLER_RST_DEVICES_W: u32 = 0x35C;

pub const CLK_RST_CONTROLLER_CLK_OUT_ENB_L: u32 = 0x10;
pub const CLK_RST_CONTROLLER_CLK_OUT_ENB_H: u32 = 0x14;
pub const CLK_RST_CONTROLLER_CLK_OUT_ENB_U: u32 = 0x18;
pub const CLK_RST_CONTROLLER_CLK_OUT_ENB_X: u32 = 0x280;
pub const CLK_RST_CONTROLLER_CLK_OUT_ENB_Y: u32 = 0x298;
pub const CLK_RST_CONTROLLER_CLK_OUT_ENB_V: u32 = 0x360;
pub const CLK_RST_CONTROLLER_CLK_OUT_ENB_W: u32 = 0x364;

pub const CLK_NO_SOURCE: u32 = 0;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_UART_A: u32 = 0x178;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_UART_B: u32 = 0x17C;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_UART_C: u32 = 0x1A0;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_UART_D: u32 = 0x1C0;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_UART_APE: u32 = 0x710;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_1: u32 = 0x124;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_2: u32 = 0x198;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_3: u32 = 0x1B8;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_4: u32 = 0x3C4;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_5: u32 = 0x128;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_I2C_6: u32 = 0x65C;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_SE: u32 = 0x42C;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X: u32 = 0x180;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_TSEC: u32 = 0x1F4;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_TSECB: u32 = 0x6D8;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_SOR1: u32 = 0x410;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_CSITE: u32 = 0x1D4;
pub const CLK_RST_CONTROLLER_CLK_SOURCE_PWM: u32 = 0x110;

pub const CLK_L_CPU: u8 = 0;
pub const CLK_L_BPMP: u8 = 1;
pub const CLK_L_SYS: u8 = 2;
pub const CLK_L_ISPB: u8 = 3;
pub const CLK_L_RTC: u8 = 4;
pub const CLK_L_TMR: u8 = 5;
pub const CLK_L_UARTA: u8 = 6;
pub const CLK_L_UARTB: u8 = 7;
pub const CLK_L_GPIO: u8 = 8;
pub const CLK_L_SDMMC2: u8 = 9;
pub const CLK_L_SPDIF: u8 = 10;
pub const CLK_L_I2S2: u8 = 11;
pub const CLK_L_I2C1: u8 = 12;
pub const CLK_L_NDFLASH: u8 = 13;
pub const CLK_L_SDMMC1: u8 = 14;
pub const CLK_L_SDMMC4: u8 = 15;
pub const CLK_L_TWC: u8 = 16;
pub const CLK_L_PWM: u8 = 17;
pub const CLK_L_I2S3: u8 = 18;
pub const CLK_L_EPP: u8 = 19;
pub const CLK_L_VI: u8 = 20;
pub const CLK_L_2D: u8 = 21;
pub const CLK_L_USBD: u8 = 22;
pub const CLK_L_ISP: u8 = 23;
pub const CLK_L_3D: u8 = 24;
pub const CLK_L_DISP2: u8 = 26;
pub const CLK_L_DISP1: u8 = 27;
pub const CLK_L_HOST1X: u8 = 28;
pub const CLK_L_VCP: u8 = 29;
pub const CLK_L_I2S1: u8 = 30;
pub const CLK_L_BPMP_CACHE_CTRL: u8 = 31;

pub const CLK_H_MEM: u8 = 0;
pub const CLK_H_AHBDMA: u8 = 1;
pub const CLK_H_APBDMA: u8 = 2;
pub const CLK_H_KBC: u8 = 4;
pub const CLK_H_STAT_MON: u8 = 5;
pub const CLK_H_PMC: u8 = 6;
pub const CLK_H_FUSE: u8 = 7;
pub const CLK_H_KFUSE: u8 = 8;
pub const CLK_H_SPI1: u8 = 9;
pub const CLK_H_SNOR: u8 = 10;
pub const CLK_H_JTAG2TBC: u8 = 11;
pub const CLK_H_SPI2: u8 = 12;
pub const CLK_H_XIO: u8 = 13;
pub const CLK_H_SPI3: u8 = 14;
pub const CLK_H_I2C5: u8 = 15;
pub const CLK_H_DSI: u8 = 16;
pub const CLK_H_HSI: u8 = 18;
pub const CLK_H_HDMI: u8 = 19;
pub const CLK_H_CSI: u8 = 20;
pub const CLK_H_I2C2: u8 = 22;
pub const CLK_H_UARTC: u8 = 23;
pub const CLK_H_MIPI_CAL: u8 = 24;
pub const CLK_H_EMC: u8 = 25;
pub const CLK_H_USB2: u8 = 26;
pub const CLK_H_USB3: u8 = 27;
pub const CLK_H_MPE: u8 = 28;
pub const CLK_H_VDE: u8 = 29;
pub const CLK_H_BSEA: u8 = 30;
pub const CLK_H_BSEV: u8 = 31;

pub const CLK_U_UARTD: u8 = 1;
pub const CLK_U_UARTE: u8 = 2;
pub const CLK_U_I2C3: u8 = 3;
pub const CLK_U_SPI4: u8 = 4;
pub const CLK_U_SDMMC3: u8 = 5;
pub const CLK_U_PCIE: u8 = 6;
pub const CLK_U_UNUSED: u8 = 7;
pub const CLK_U_AFI: u8 = 8;
pub const CLK_U_CSITE: u8 = 9;
pub const CLK_U_PCIEXCLK: u8 = 10;
pub const CLK_U_BPMPUCQ: u8 = 11;
pub const CLK_U_LA: u8 = 12;
pub const CLK_U_TRACECLKIN: u8 = 13;
pub const CLK_U_SOC_THERM: u8 = 14;
pub const CLK_U_DTV: u8 = 15;
pub const CLK_U_NAND_SPEED: u8 = 16;
pub const CLK_U_I2C_SLOW: u8 = 17;
pub const CLK_U_DSIB: u8 = 18;
pub const CLK_U_TSEC: u8 = 19;
pub const CLK_U_IRAMA: u8 = 20;
pub const CLK_U_IRAMB: u8 = 21;
pub const CLK_U_IRAMC: u8 = 22;
pub const CLK_U_IRAMD: u8 = 23;
pub const CLK_U_BPMP_CACHE_RAM: u8 = 24;
pub const CLK_U_XUSB_HOST: u8 = 25;
pub const CLK_U_CLK_M_DOUBLER: u8 = 26;
pub const CLK_U_MSENC: u8 = 27;
pub const CLK_U_SUS_OUT: u8 = 28;
pub const CLK_U_DEV2_OUT: u8 = 29;
pub const CLK_U_DEV1_OUT: u8 = 30;
pub const CLK_U_XUSB_DEV: u8 = 31;

pub const CLK_V_CPUG: u8 = 0;
pub const CLK_V_CPULP: u8 = 1;
pub const CLK_V_3D2: u8 = 2;
pub const CLK_V_MSELECT: u8 = 3;
pub const CLK_V_TSENSOR: u8 = 4;
pub const CLK_V_I2S4: u8 = 5;
pub const CLK_V_I2S5: u8 = 6;
pub const CLK_V_I2C4: u8 = 7;
pub const CLK_V_SPI5: u8 = 8;
pub const CLK_V_SPI6: u8 = 9;
pub const CLK_V_AHUB: u8 = 10;
pub const CLK_V_APB2APE: u8 = 11;
pub const CLK_V_DAM0: u8 = 12;
pub const CLK_V_DAM1: u8 = 13;
pub const CLK_V_DAM2: u8 = 14;
pub const CLK_V_HDA2CODEC_2X: u8 = 15;
pub const CLK_V_ATOMICS: u8 = 16;
pub const CLK_V_SPDIF_DOUBLER: u8 = 22;
pub const CLK_V_ACTMON: u8 = 23;
pub const CLK_V_EXTPERIPH1: u8 = 24;
pub const CLK_V_EXTPERIPH2: u8 = 25;
pub const CLK_V_EXTPERIPH3: u8 = 26;
pub const CLK_V_SATA_OOB: u8 = 27;
pub const CLK_V_SATA: u8 = 28;
pub const CLK_V_HDA: u8 = 29;
pub const CLK_V_TZRAM: u8 = 30;
pub const CLK_V_SE: u8 = 31;

pub const CLK_W_HDA2HDMICODEC: u8 = 0;
pub const CLK_W_RESERVED0: u8 = 1;
pub const CLK_W_PCIERX0: u8 = 2;
pub const CLK_W_PCIERX1: u8 = 3;
pub const CLK_W_PCIERX2: u8 = 4;
pub const CLK_W_PCIERX3: u8 = 5;
pub const CLK_W_PCIERX4: u8 = 6;
pub const CLK_W_PCIERX5: u8 = 7;
pub const CLK_W_CEC: u8 = 8;
pub const CLK_W_PCIE2_IOBIST: u8 = 9;
pub const CLK_W_EMC_IOBIST: u8 = 10;
pub const CLK_W_HDMI_IOBIST: u8 = 11;
pub const CLK_W_SATA_IOBIST: u8 = 12;
pub const CLK_W_MIPI_IOBIST: u8 = 13;
pub const CLK_W_XUSB_PADCTL: u8 = 14;
pub const CLK_W_XUSB: u8 = 15;
pub const CLK_W_CILAB: u8 = 16;
pub const CLK_W_CILCD: u8 = 17;
pub const CLK_W_CILEF: u8 = 18;
pub const CLK_W_DSIA_LP: u8 = 19;
pub const CLK_W_DSIB_LP: u8 = 20;
pub const CLK_W_ENTROPY: u8 = 21;
pub const CLK_W_DDS: u8 = 22;
pub const CLK_W_DP2: u8 = 24;
pub const CLK_W_AMX0: u8 = 25;
pub const CLK_W_ADX0: u8 = 26;
pub const CLK_W_DVFS: u8 = 27;
pub const CLK_W_XUSB_SS: u8 = 28;
pub const CLK_W_EMC_LATENCY: u8 = 29;
pub const CLK_W_MC1: u8 = 30;

pub const CLK_X_SPARE: u8 = 0;
pub const CLK_X_DMIC1: u8 = 1;
pub const CLK_X_DMIC2: u8 = 2;
pub const CLK_X_ETR: u8 = 3;
pub const CLK_X_CAM_MCLK: u8 = 4;
pub const CLK_X_CAM_MCLK2: u8 = 5;
pub const CLK_X_I2C6: u8 = 6;
pub const CLK_X_MC_CAPA: u8 = 7;
pub const CLK_X_MC_CBPA: u8 = 8;
pub const CLK_X_MC_CPU: u8 = 9;
pub const CLK_X_MC_BBC: u8 = 10;
pub const CLK_X_VIM2_CLK: u8 = 11;
pub const CLK_X_MIPIBIF: u8 = 13;
pub const CLK_X_EMC_DLL: u8 = 14;
pub const CLK_X_HDMI_AUDIO: u8 = 16;
pub const CLK_X_UART_FST_MIPI_CAL: u8 = 17;
pub const CLK_X_VIC: u8 = 18;
pub const CLK_X_ADX1: u8 = 20;
pub const CLK_X_DPAUX: u8 = 21;
pub const CLK_X_SOR0: u8 = 22;
pub const CLK_X_SOR1: u8 = 23;
pub const CLK_X_GPU: u8 = 24;
pub const CLK_X_DBGAPB: u8 = 25;
pub const CLK_X_HPLL_ADSP: u8 = 26;
pub const CLK_X_PLLP_ADSP: u8 = 27;
pub const CLK_X_PLLA_ADSP: u8 = 28;
pub const CLK_X_PLLG_REF: u8 = 29;

pub const CLK_Y_SPARE1: u8 = 0;
pub const CLK_Y_SDMMC_LEGACY_TM: u8 = 1;
pub const CLK_Y_NVDEC: u8 = 2;
pub const CLK_Y_NVJPG: u8 = 3;
pub const CLK_Y_AXIAP: u8 = 4;
pub const CLK_Y_DMIC3: u8 = 5;
pub const CLK_Y_APE: u8 = 6;
pub const CLK_Y_ADSP: u8 = 7;
pub const CLK_Y_MC_CDPA: u8 = 8;
pub const CLK_Y_MC_CCPA: u8 = 9;
pub const CLK_Y_MAUD: u8 = 10;
pub const CLK_Y_SATA_USB_UPHY: u8 = 12;
pub const CLK_Y_PEX_USB_UPHY: u8 = 13;
pub const CLK_Y_TSECB: u8 = 14;
pub const CLK_Y_DPAUX1: u8 = 15;
pub const CLK_Y_VI_I2C: u8 = 16;
pub const CLK_Y_HSIC_TRK: u8 = 17;
pub const CLK_Y_USB2_TRK: u8 = 18;
pub const CLK_Y_QSPI: u8 = 19;
pub const CLK_Y_UARTAPE: u8 = 20;
pub const CLK_Y_ADSPINTF: u8 = 21;
pub const CLK_Y_ADSPPERIPH: u8 = 22;
pub const CLK_Y_ADSPDBG: u8 = 23;
pub const CLK_Y_ADSPWDT: u8 = 24;
pub const CLK_Y_ADSPSCU: u8 = 25;
pub const CLK_Y_ADSPNEON: u8 = 26;
pub const CLK_Y_NVENC: u8 = 27;
pub const CLK_Y_IQC2: u8 = 28;
pub const CLK_Y_IQC1: u8 = 29;
pub const CLK_Y_SOR_SAFE: u8 = 30;
pub const CLK_Y_PLLP_OUT_CPU: u8 = 31;

/// Representation of a device clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clock {
    /// The reset register offset for the clock device.
    reset: u32,
    /// The enable register offset for the clock device.
    enable: u32,
    /// The source register offset for the clock device.
    source: u32,
    /// The clock device index.
    index: u8,
    /// The clock source value.
    clock_source: u32,
    /// The clock divisor value.
    clock_divisor: u32,
}

impl Clock {
    /// Representation of the UART A clock.
    pub const UART_A: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_A,
        index: CLK_L_UARTA,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the UART B clock.
    pub const UART_B: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_B,
        index: CLK_L_UARTB,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the UART C clock.
    pub const UART_C: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_C,
        index: CLK_H_UARTC,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the UART D clock.
    pub const UART_D: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_D,
        index: CLK_U_UARTD,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the UART APE clock.
    pub const UART_APE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_UART_APE,
        index: CLK_Y_UARTAPE,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the I²C 1 clock.
    pub const I2C_1: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_1,
        index: CLK_L_I2C1,
        clock_source: 0,
        clock_divisor: 19,
    };

    /// Representation of the I²C 2 clock.
    pub const I2C_2: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_2,
        index: CLK_H_I2C2,
        clock_source: 0,
        clock_divisor: 4,
    };

    /// Representation of the I²C 3 clock.
    pub const I2C_3: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_3,
        index: CLK_U_I2C3,
        clock_source: 0,
        clock_divisor: 4,
    };

    /// Representation of the I²C 4 clock.
    pub const I2C_4: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_4,
        index: CLK_V_I2C4,
        clock_source: 0,
        clock_divisor: 19,
    };

    /// Representation of the I²C 5 clock.
    pub const I2C_5: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_5,
        index: CLK_H_I2C5,
        clock_source: 0,
        clock_divisor: 4,
    };

    /// Representation of the I²C 6 clock.
    pub const I2C_6: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_I2C_6,
        index: CLK_X_I2C6,
        clock_source: 0,
        clock_divisor: 19,
    };

    /// Representation of the Security Engine clock.
    pub const SE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_SE,
        index: CLK_V_SE,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the TZRAM clock.
    pub const TZRAM: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_V,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_V,
        source: CLK_NO_SOURCE,
        index: CLK_V_TZRAM,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the HOST1X clock.
    pub const HOST1X: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X,
        index: CLK_L_HOST1X,
        clock_source: 4,
        clock_divisor: 3,
    };

    /// Representation of the TSEC-A clock.
    pub const TSEC: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_TSEC,
        index: CLK_U_TSEC,
        clock_source: 0,
        clock_divisor: 2,
    };

    /// Representation of the TSEC-B clock.
    pub const TSECB: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_TSECB,
        index: CLK_Y_TSECB,
        clock_source: 0,
        clock_divisor: 2,
    };

    /// Representation of the SOR_SAFE clock.
    pub const SOR_SAFE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_NO_SOURCE,
        index: CLK_Y_SOR_SAFE,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the SOR0 clock.
    pub const SOR0: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_NO_SOURCE,
        index: CLK_X_SOR0,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the SOR1 clock.
    pub const SOR1: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_SOR1,
        index: CLK_X_SOR1,
        clock_source: 0,
        clock_divisor: 2,
    };

    /// Representation of the DPAUX clock.
    pub const DPAUX: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_X,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_X,
        source: CLK_NO_SOURCE,
        index: CLK_X_DPAUX,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the DPAUX1 clock.
    pub const DPAUX1: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_Y,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_Y,
        source: CLK_NO_SOURCE,
        index: CLK_Y_DPAUX1,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the MIPI CAL clock.
    pub const MIPI_CAL: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_NO_SOURCE,
        index: CLK_H_MIPI_CAL,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the CSI clock.
    pub const CSI: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_NO_SOURCE,
        index: CLK_H_CSI,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the DSI clock.
    pub const DSI: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_NO_SOURCE,
        index: CLK_H_DSI,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the DSIB clock.
    pub const DSIB: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_NO_SOURCE,
        index: CLK_U_DSIB,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the KFUSE clock.
    pub const KFUSE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_H,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_H,
        source: CLK_NO_SOURCE,
        index: CLK_H_KFUSE,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the CL-DVFS clock.
    pub const CL_DVFS: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_W,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_W,
        source: CLK_NO_SOURCE,
        index: CLK_W_DVFS,
        clock_source: 0,
        clock_divisor: 0,
    };

    /// Representation of the CSITE clock.
    pub const CSITE: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_U,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_U,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_CSITE,
        index: CLK_U_CSITE,
        clock_source: 0,
        clock_divisor: 4,
    };

    /// Representation of the PWM clock.
    pub const PWM: Self = Clock {
        reset: CLK_RST_CONTROLLER_RST_DEVICES_L,
        enable: CLK_RST_CONTROLLER_CLK_OUT_ENB_L,
        source: CLK_RST_CONTROLLER_CLK_SOURCE_PWM,
        index: CLK_L_PWM,
        clock_source: 6,
        clock_divisor: 4,
    };
}

impl Clock {
    #[inline(always)]
    fn get_mask(&self) -> u32 {
        (1 << (self.index & 0x1F)) as u32
    }

    fn set_reset(&self, reset: bool) {
        // Figure out the register to write to.
        let reset_reg = unsafe { &*((CAR + self.reset) as *const ReadWrite<u32>) };

        // Read the value to be modified and the mask to be used.
        let mut value = reset_reg.get();
        let mask = self.get_mask();

        // Set or clear the bit, as appropriate.
        if reset {
            value |= mask;
        } else {
            value &= !mask;
        }

        // Write the new value to the register.
        reset_reg.set(value);
    }

    fn set_enable(&self, enable: bool) {
        // Figure out the register to write to.
        let enable_reg = unsafe { &*((CAR + self.enable) as *const ReadWrite<u32>) };

        // Read the value to be modified and the mask to be used.
        let mut value = enable_reg.get();
        let mask = self.get_mask();

        // Set or clear the bit, as appropriate.
        if enable {
            value |= mask;
        } else {
            value &= !mask;
        }

        // Write the new value to the register.
        enable_reg.set(value);
    }

    /// Boots up the device.
    pub fn enable(&self) {
        // Disable the clock.
        self.disable();

        // Configure the clock source, if needed.
        if self.source != CLK_NO_SOURCE {
            unsafe {
                (*((CAR + self.source) as *const ReadWrite<u32>))
                    .set((self.clock_source << 29) | self.clock_divisor);
            }
        }

        // KFUSE needs more time for the changes to take effect.
        if self == &Self::KFUSE {
            // Enable the clock.
            self.set_enable(true);
            usleep(100);

            // Take clock off reset.
            self.set_reset(false);
            usleep(200);
        } else {
            // Enable the clock.
            self.set_enable(true);
            usleep(2);

            // Take clock off reset.
            self.set_reset(false);
        }

        assert!(self.is_enabled());
    }

    /// Disables the device.
    pub fn disable(&self) {
        // Put clock into reset.
        self.set_reset(true);
        // Disable.
        self.set_enable(false);

        assert!(!self.is_enabled());
    }

    /// Indicates whether the device is enabled or not.
    pub fn is_enabled(&self) -> bool {
        // Figure out the register to read from.
        let enable_reg = unsafe { &*((CAR + self.enable) as *const ReadWrite<u32>) };

        // Check if the mask bit is set.
        let mask = self.get_mask();
        (enable_reg.get() & mask) == mask
    }
}
