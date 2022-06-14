//! Miscellaneous system control registers.

use tock_registers::{register_structs, registers::ReadWrite};

use crate::memory_map::APB;

/// A pointer to the AMBA Peripheral Bus register block that can be accessed by dereferencing it.
pub const REGISTERS: *const AmbaPeripheralBus = APB as *const AmbaPeripheralBus;

pub mod misc_pp {
    use tock_registers::{register_bitfields, register_structs, registers::*};

    register_bitfields! {
        u32,

        /// Bitfields of the `APB_MISC_PP_STRAPPING_OPT_A_0` register.
        pub APB_MISC_PP_STRAPPING_OPT_A_0 [
            BOOT_SELECT OFFSET(26) NUMBITS(3) [],

            NVPROD_UART OFFSET(13) NUMBITS(1) [],

            RCM_STRAPS OFFSET(10) NUMBITS(3) [],

            BOOT_FAST_UART OFFSET(9) NUMBITS(1) [],

            MIO_WIDTH OFFSET(8) NUMBITS(1) [],

            RAM_CODE OFFSET(4) NUMBITS(2) [],

            NOR_WIDTH OFFSET(0) NUMBITS(1) []
        ],

        /// Bitfields of the `APB_MISC_PP_CONFIG_CTL_0` register.
        pub APB_MISC_PP_CONFIG_CTL_0 [
            /// Whether RTCK Daisy chaining should be enabled.
            TBE OFFSET(7) NUMBITS(1) [],

            /// NOTE: Do not set this bit!
            XBAR_SO_DEFAULT OFFSET(1) NUMBITS(1) [],

            /// NOTE: Do not set this bit!
            CPU_XBAR_SO_ENABLE OFFSET(0) NUMBITS(1) []
        ],

        /// Bitfields of the `APB_MISC_PP_PINMUX_GLOBAL_0_0` register.
        pub APB_MISC_PP_PINMUX_GLOBAL_0_0 [
            /// Whether inputs should be clamped when tristated.
            CLAMP_INPUTS_WHEN_TRISTATED OFFSET(0) NUMBITS(1) []
        ]
    }

    register_structs! {
        /// Representation of the APB Control Registers.
        #[allow(non_snake_case)]
        pub MiscPP {
            (0x000 => _reserved0: [ReadWrite<u8>; 0x8]),
            (0x008 => pub APB_MISC_PP_STRAPPING_OPT_A_0: ReadWrite<u32, APB_MISC_PP_STRAPPING_OPT_A_0::Register>),
            (0x00C => _reserved1: [ReadWrite<u8>; 0x18]),
            (0x024 => pub APB_MISC_PP_CONFIG_CTL_0: ReadWrite<u32, APB_MISC_PP_CONFIG_CTL_0::Register>),
            (0x028 => _reserved2: [ReadWrite<u8>; 0x18]),
            (0x040 => pub APB_MISC_PP_PINMUX_GLOBAL_0_0: ReadWrite<u32, APB_MISC_PP_PINMUX_GLOBAL_0_0::Register>),
            (0x044 => _reserved3: [ReadWrite<u8>; 0x3BC]),
            (0x400 => @END),
        }
    }

    assert_eq_size!(MiscPP, [u8; 0x400]);
}

pub mod misc_gp {
    use tock_registers::{register_structs, registers::*};

    // TODO: Bitfields.

    register_structs! {
        /// Representation of the APB Pad Control Registers.
        #[allow(non_snake_case)]
        pub MiscGP {
            (0x000 => _reserved0: [ReadWrite<u8>; 0x4]),
            (0x004 => pub APB_MISC_GP_HIDREV_0: ReadOnly<u32>),
            (0x008 => _reserved1: [ReadWrite<u8>; 0x8]),
            (0x010 => pub APB_MISC_GP_ASDBGREG_0: ReadWrite<u32>),
            (0x014 => _reserved2: [ReadWrite<u8>; 0xC0]),
            (0x0D4 => pub APB_MISC_GP_SDMMC1_CLK_LPBK_CONTROL_0: ReadWrite<u32>),
            (0x0D8 => pub APB_MISC_GP_SDMMC3_CLK_LPBK_CONTROL_0: ReadWrite<u32>),
            (0x0DC => pub APB_MISC_GP_EMMC2_PAD_CFG_CONTROL_0: ReadWrite<u32>),
            (0x0E0 => pub APB_MISC_GP_EMMC4_PAD_CFG_CONTROL_0: ReadWrite<u32>),
            (0x0E4 => pub APB_MISC_GP_ALS_PROX_INT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x0E8 => pub APB_MISC_GP_AP_READY_CFGPADCTRL_0: ReadWrite<u32>),
            (0x0EC => pub APB_MISC_GP_AP_WAKE_BT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x0F0 => pub APB_MISC_GP_AP_WAKE_NFC_CFGPADCTRL_0: ReadWrite<u32>),
            (0x0F4 => pub APB_MISC_GP_AUD_MCLK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x0F8 => pub APB_MISC_GP_BATT_BCL_CFGPADCTRL_0: ReadWrite<u32>),
            (0x0FC => pub APB_MISC_GP_BT_RST_CFGPADCTRL_0: ReadWrite<u32>),
            (0x100 => pub APB_MISC_GP_BT_WAKE_AP_CFGPADCTRL_0: ReadWrite<u32>),
            (0x104 => pub APB_MISC_GP_BUTTON_HOME_CFGPADCTRL_0: ReadWrite<u32>),
            (0x108 => pub APB_MISC_GP_BUTTON_POWER_ON_CFGPADCTRL_0: ReadWrite<u32>),
            (0x10C => pub APB_MISC_GP_BUTTON_SLIDE_SW_CFGPADCTRL_0: ReadWrite<u32>),
            (0x110 => pub APB_MISC_GP_BUTTON_VOL_DOWN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x114 => pub APB_MISC_GP_BUTTON_VOL_UP_CFGPADCTRL_0: ReadWrite<u32>),
            (0x118 => pub APB_MISC_GP_CAM1_MCLK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x11C => pub APB_MISC_GP_CAM1_PWDN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x120 => pub APB_MISC_GP_CAM1_STROBE_CFGPADCTRL_0: ReadWrite<u32>),
            (0x124 => pub APB_MISC_GP_CAM2_MCLK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x128 => pub APB_MISC_GP_CAM2_PWDN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x12C => pub APB_MISC_GP_CAM_AF_EN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x130 => pub APB_MISC_GP_CAM_FLASH_EN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x134 => pub APB_MISC_GP_CAM_I2C_SCL_CFGPADCTRL_0: ReadWrite<u32>),
            (0x138 => pub APB_MISC_GP_CAM_I2C_SDA_CFGPADCTRL_0: ReadWrite<u32>),
            (0x13C => pub APB_MISC_GP_CAM_RST_CFGPADCTRL_0: ReadWrite<u32>),
            (0x140 => pub APB_MISC_GP_CLK_32K_IN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x144 => pub APB_MISC_GP_CLK_32K_OUT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x148 => pub APB_MISC_GP_CLK_REQ_CFGPADCTRL_0: ReadWrite<u32>),
            (0x14C => pub APB_MISC_GP_CORE_PWR_REQ_CFGPADCTRL_0: ReadWrite<u32>),
            (0x150 => pub APB_MISC_GP_CPU_PWR_REQ_CFGPADCTRL_0: ReadWrite<u32>),
            (0x154 => pub APB_MISC_GP_DAP1_DIN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x158 => pub APB_MISC_GP_DAP1_DOUT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x15C => pub APB_MISC_GP_DAP1_FS_CFGPADCTRL_0: ReadWrite<u32>),
            (0x160 => pub APB_MISC_GP_DAP1_SCLK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x164 => pub APB_MISC_GP_DAP2_DIN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x168 => pub APB_MISC_GP_DAP2_DOUT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x16C => pub APB_MISC_GP_DAP2_FS_CFGPADCTRL_0: ReadWrite<u32>),
            (0x170 => pub APB_MISC_GP_DAP2_SCLK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x174 => pub APB_MISC_GP_DAP4_DIN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x178 => pub APB_MISC_GP_DAP4_DOUT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x17C => pub APB_MISC_GP_DAP4_FS_CFGPADCTRL_0: ReadWrite<u32>),
            (0x180 => pub APB_MISC_GP_DAP4_SCLK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x184 => pub APB_MISC_GP_DMIC1_CLK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x188 => pub APB_MISC_GP_DMIC1_DAT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x18C => pub APB_MISC_GP_DMIC2_CLK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x190 => pub APB_MISC_GP_DMIC2_DAT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x194 => pub APB_MISC_GP_DMIC3_CLK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x198 => pub APB_MISC_GP_DMIC3_DAT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x19C => pub APB_MISC_GP_DP_HPD_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1A0 => pub APB_MISC_GP_DVFS_CLK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1A4 => pub APB_MISC_GP_DVFS_PWM_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1A8 => pub APB_MISC_GP_GEN1_I2C_SCL_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1AC => pub APB_MISC_GP_GEN1_I2C_SDA_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1B0 => pub APB_MISC_GP_GEN2_I2C_SCL_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1B4 => pub APB_MISC_GP_GEN2_I2C_SDA_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1B8 => pub APB_MISC_GP_GEN3_I2C_SCL_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1BC => pub APB_MISC_GP_GEN3_I2C_SDA_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1C0 => pub APB_MISC_GP_GPIO_PA6_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1C4 => pub APB_MISC_GP_GPIO_PCC7_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1C8 => pub APB_MISC_GP_GPIO_PE6_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1CC => pub APB_MISC_GP_GPIO_PE7_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1D0 => pub APB_MISC_GP_GPIO_PH6_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1D4 => pub APB_MISC_GP_GPIO_PK0_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1D8 => pub APB_MISC_GP_GPIO_PK1_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1DC => pub APB_MISC_GP_GPIO_PK2_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1E0 => pub APB_MISC_GP_GPIO_PK3_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1E4 => pub APB_MISC_GP_GPIO_PK4_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1E8 => pub APB_MISC_GP_GPIO_PK5_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1EC => pub APB_MISC_GP_GPIO_PK6_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1F0 => pub APB_MISC_GP_GPIO_PK7_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1F4 => pub APB_MISC_GP_GPIO_PL0_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1F8 => pub APB_MISC_GP_GPIO_PL1_CFGPADCTRL_0: ReadWrite<u32>),
            (0x1FC => pub APB_MISC_GP_GPIO_PZ0_CFGPADCTRL_0: ReadWrite<u32>),
            (0x200 => pub APB_MISC_GP_GPIO_PZ1_CFGPADCTRL_0: ReadWrite<u32>),
            (0x204 => pub APB_MISC_GP_GPIO_PZ2_CFGPADCTRL_0: ReadWrite<u32>),
            (0x208 => pub APB_MISC_GP_GPIO_PZ3_CFGPADCTRL_0: ReadWrite<u32>),
            (0x20C => pub APB_MISC_GP_GPIO_PZ4_CFGPADCTRL_0: ReadWrite<u32>),
            (0x210 => pub APB_MISC_GP_GPIO_PZ5_CFGPADCTRL_0: ReadWrite<u32>),
            (0x214 => pub APB_MISC_GP_GPIO_X1_AUD_CFGPADCTRL_0: ReadWrite<u32>),
            (0x218 => pub APB_MISC_GP_GPIO_X3_AUD_CFGPADCTRL_0: ReadWrite<u32>),
            (0x21C => pub APB_MISC_GP_GPS_EN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x220 => pub APB_MISC_GP_GPS_RST_CFGPADCTRL_0: ReadWrite<u32>),
            (0x224 => pub APB_MISC_GP_HDMI_CEC_CFGPADCTRL_0: ReadWrite<u32>),
            (0x228 => pub APB_MISC_GP_HDMI_INT_DP_HPD_CFGPADCTRL_0: ReadWrite<u32>),
            (0x22C => pub APB_MISC_GP_JTAG_RTCK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x230 => pub APB_MISC_GP_LCD_BL_EN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x234 => pub APB_MISC_GP_LCD_BL_PWM_CFGPADCTRL_0: ReadWrite<u32>),
            (0x238 => pub APB_MISC_GP_LCD_GPIO1_CFGPADCTRL_0: ReadWrite<u32>),
            (0x23C => pub APB_MISC_GP_LCD_GPIO2_CFGPADCTRL_0: ReadWrite<u32>),
            (0x240 => pub APB_MISC_GP_LCD_RST_CFGPADCTRL_0: ReadWrite<u32>),
            (0x244 => pub APB_MISC_GP_LCD_TE_CFGPADCTRL_0: ReadWrite<u32>),
            (0x248 => pub APB_MISC_GP_MODEM_WAKE_AP_CFGPADCTRL_0: ReadWrite<u32>),
            (0x24C => pub APB_MISC_GP_MOTION_INT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x250 => pub APB_MISC_GP_NFC_EN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x254 => pub APB_MISC_GP_NFC_INT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x258 => pub APB_MISC_GP_PEX_L0_CLKREQ_N_CFGPADCTRL_0: ReadWrite<u32>),
            (0x25C => pub APB_MISC_GP_PEX_L0_RST_N_CFGPADCTRL_0: ReadWrite<u32>),
            (0x260 => pub APB_MISC_GP_PEX_L1_CLKREQ_N_CFGPADCTRL_0: ReadWrite<u32>),
            (0x264 => pub APB_MISC_GP_PEX_L1_RST_N_CFGPADCTRL_0: ReadWrite<u32>),
            (0x268 => pub APB_MISC_GP_PEX_WAKE_N_CFGPADCTRL_0: ReadWrite<u32>),
            (0x26C => pub APB_MISC_GP_PWR_I2C_SCL_CFGPADCTRL_0: ReadWrite<u32>),
            (0x270 => pub APB_MISC_GP_PWR_I2C_SDA_CFGPADCTRL_0: ReadWrite<u32>),
            (0x274 => pub APB_MISC_GP_PWR_INT_N_CFGPADCTRL_0: ReadWrite<u32>),
            (0x278 => pub APB_MISC_GP_QSPI_COMP_CFGPADCTRL_0: ReadWrite<u32>),
            (0x27C => _reserved3: [ReadWrite<u8>; 0x14]),
            (0x290 => pub APB_MISC_GP_QSPI_SCK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x294 => pub APB_MISC_GP_SATA_LED_ACTIVE_CFGPADCTRL_0: ReadWrite<u32>),
            (0x298 => pub APB_MISC_GP_SDMMC1_PAD_CFGPADCTRL_0: ReadWrite<u32>),
            (0x29C => pub APB_MISC_GP_EMMC2_PAD_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2A0 => pub APB_MISC_GP_EMMC2_PAD_DRV_TYPE_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2A4 => pub APB_MISC_GP_EMMC2_PAD_PUPD_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2A8 => _reserved4: [ReadWrite<u8>; 0x8]),
            (0x2B0 => pub APB_MISC_GP_SDMMC3_PAD_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2B4 => pub APB_MISC_GP_EMMC4_PAD_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2B8 => pub APB_MISC_GP_EMMC4_PAD_DRV_TYPE_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2BC => pub APB_MISC_GP_EMMC4_PAD_PUPD_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2C0 => _reserved5: [ReadWrite<u8>; 0x8]),
            (0x2C8 => pub APB_MISC_GP_SHUTDOWN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2CC => pub APB_MISC_GP_SPDIF_IN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2D0 => pub APB_MISC_GP_SPDIF_OUT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2D4 => pub APB_MISC_GP_SPI1_CS0_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2D8 => pub APB_MISC_GP_SPI1_CS1_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2DC => pub APB_MISC_GP_SPI1_MISO_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2E0 => pub APB_MISC_GP_SPI1_MOSI_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2E4 => pub APB_MISC_GP_SPI1_SCK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2E8 => pub APB_MISC_GP_SPI2_CS0_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2EC => pub APB_MISC_GP_SPI2_CS1_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2F0 => pub APB_MISC_GP_SPI2_MISO_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2F4 => pub APB_MISC_GP_SPI2_MOSI_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2F8 => pub APB_MISC_GP_SPI2_SCK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x2FC => pub APB_MISC_GP_SPI4_CS0_CFGPADCTRL_0: ReadWrite<u32>),
            (0x300 => pub APB_MISC_GP_SPI4_MISO_CFGPADCTRL_0: ReadWrite<u32>),
            (0x304 => pub APB_MISC_GP_SPI4_MOSI_CFGPADCTRL_0: ReadWrite<u32>),
            (0x308 => pub APB_MISC_GP_SPI4_SCK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x30C => pub APB_MISC_GP_TEMP_ALERT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x310 => pub APB_MISC_GP_TOUCH_CLK_CFGPADCTRL_0: ReadWrite<u32>),
            (0x314 => pub APB_MISC_GP_TOUCH_INT_CFGPADCTRL_0: ReadWrite<u32>),
            (0x318 => pub APB_MISC_GP_TOUCH_RST_CFGPADCTRL_0: ReadWrite<u32>),
            (0x31C => pub APB_MISC_GP_UART1_CTS_CFGPADCTRL_0: ReadWrite<u32>),
            (0x320 => pub APB_MISC_GP_UART1_RTS_CFGPADCTRL_0: ReadWrite<u32>),
            (0x324 => pub APB_MISC_GP_UART1_RX_CFGPADCTRL_0: ReadWrite<u32>),
            (0x328 => pub APB_MISC_GP_UART1_TX_CFGPADCTRL_0: ReadWrite<u32>),
            (0x32C => pub APB_MISC_GP_UART2_CTS_CFGPADCTRL_0: ReadWrite<u32>),
            (0x330 => pub APB_MISC_GP_UART2_RTS_CFGPADCTRL_0: ReadWrite<u32>),
            (0x334 => pub APB_MISC_GP_UART2_RX_CFGPADCTRL_0: ReadWrite<u32>),
            (0x338 => pub APB_MISC_GP_UART2_TX_CFGPADCTRL_0: ReadWrite<u32>),
            (0x33C => pub APB_MISC_GP_UART3_CTS_CFGPADCTRL_0: ReadWrite<u32>),
            (0x340 => pub APB_MISC_GP_UART3_RTS_CFGPADCTRL_0: ReadWrite<u32>),
            (0x344 => pub APB_MISC_GP_UART3_RX_CFGPADCTRL_0: ReadWrite<u32>),
            (0x348 => pub APB_MISC_GP_UART3_TX_CFGPADCTRL_0: ReadWrite<u32>),
            (0x34C => pub APB_MISC_GP_UART4_CTS_CFGPADCTRL_0: ReadWrite<u32>),
            (0x350 => pub APB_MISC_GP_UART4_RTS_CFGPADCTRL_0: ReadWrite<u32>),
            (0x354 => pub APB_MISC_GP_UART4_RX_CFGPADCTRL_0: ReadWrite<u32>),
            (0x358 => pub APB_MISC_GP_UART4_TX_CFGPADCTRL_0: ReadWrite<u32>),
            (0x35C => pub APB_MISC_GP_USB_VBUS_EN0_CFGPADCTRL_0: ReadWrite<u32>),
            (0x360 => pub APB_MISC_GP_USB_VBUS_EN1_CFGPADCTRL_0: ReadWrite<u32>),
            (0x364 => pub APB_MISC_GP_WIFI_EN_CFGPADCTRL_0: ReadWrite<u32>),
            (0x368 => pub APB_MISC_GP_WIFI_RST_CFGPADCTRL_0: ReadWrite<u32>),
            (0x36C => pub APB_MISC_GP_WIFI_WAKE_AP_CFGPADCTRL_0: ReadWrite<u32>),
            (0x370 => pub APB_MISC_GP_QSPI_COMP_CONTROL_0: ReadWrite<u32>),
            (0x374 => pub APB_MISC_GP_VGPIO_GPIO_MUX_SEL_0: ReadWrite<u32>),
            (0x378 => pub APB_MISC_GP_QSPI_SCK_LPBK_CONTROL_0: ReadWrite<u32>),
            (0x37C => @END),
        }
    }

    assert_eq_size!(MiscGP, [u8; 0x37C]);
}

register_structs! {
    /// Representation of the APB Misc Registers.
    #[allow(non_snake_case)]
    pub AmbaPeripheralBus {
        (0x000 => pub pp: misc_pp::MiscPP),
        (0x400 => _reserved0: [ReadWrite<u8>; 0x400]), // SC1X_PADS
        (0x800 => pub gp: misc_gp::MiscGP),
        (0xB7C => @END),
    }
}

assert_eq_size!(AmbaPeripheralBus, [u8; 0xB7C]);
