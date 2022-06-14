use tock_registers::{register_bitfields, register_structs, registers::*};

use crate::memory_map::MISC;

/// Base address for Pinmux registers.
pub const PINMUX_BASE: u32 = MISC + 0x3000;

/// A pointer to the Pinmux register block that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = PINMUX_BASE as *const Registers;

register_bitfields! {
    u32,

    /// Pad Control Options for Pinmux registers.
    pub PAD_OPTIONS [
        /// Unknown.
        ///
        /// NOTE: Applicable to CZ pads.
        E_PREEMP OFFSET(15) NUMBITS(1) [],

        /// Enables different options of impedance code mapping for the pads.
        ///
        /// NOTE: Applicable to CZ/LV_CZ pads.
        DRV_TYPE OFFSET(13) NUMBITS(2) [
            Drive1X = 0,
            Drive2X = 1,
            Drive3X = 2,
            Drive4X = 3
        ],

        /// Schmitt Trigger.
        ///
        /// Enabling Schmitt provides better noise margin characteristics for the input.
        ///
        /// NOTE: Applicable to all pads.
        E_SCHMT OFFSET(12) NUMBITS(1) [],

        /// Reserved.
        E_OD OFFSET(11) NUMBITS(1) [],

        /// Enables High Voltage Operation (3.3V).
        ///
        /// NOTE: Applicable to DD pads only.
        E_IO_HV OFFSET(10) NUMBITS(1) [],

        /// High Speed Mode.
        ///
        /// Enables or disables high speed operation for Receiver and Transmitter.
        ///
        /// NOTE: Applicable to CZ pads.
        E_HSM OFFSET(9) NUMBITS(1) [],

        /// Enables Base Drivers when set High.
        ///
        /// NOTE: Applicable to ST and DD pads.
        E_LPDR OFFSET(8) NUMBITS(1) [],

        /// Lock control for writing to a register.
        ///
        /// NOTE: Applicable to all pads.
        LOCK OFFSET(7) NUMBITS(1) [],

        /// Input Receiver.
        ///
        /// Enables or disables input receiver.
        ///
        /// NOTE: Applicable to all pads.
        E_INPUT OFFSET(6) NUMBITS(1) [],

        /// Holds control during DPD/deep sleep (LP0).
        ///
        /// NOTE: Applicable to all pads.
        PARK OFFSET(5) NUMBITS(1) [],

        /// Tristate (high-Z) option.
        ///
        /// Disables or enables the pad's output driver.
        ///
        /// NOTE: Applicable to all pads.
        TRISTATE OFFSET(4) NUMBITS(1) [
            Passthrough = 0,
            Tristate = 1
        ],

        /// Internal Pull-up/down option.
        ///
        /// NOTE: Applicable to all pads.
        PUPD OFFSET(2) NUMBITS(2) [
            /// Enables nothing.
            None = 0,
            /// Enables internal Pull-down resistors.
            PullDown = 1,
            /// Enables internal Pull-up resistors.
            PullUp = 2,
            Rsvd = 3
        ],

        /// Unknown.
        PM OFFSET(0) NUMBITS(2) [
            Rsvd1 = 1,
            Rsvd2 = 2,
            Rsvd3 = 3
        ]
    ]
}

register_structs! {
    /// Representation of the Pinmux registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x000 => pub PINMUX_AUX_SDMMC1_CLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x004 => pub PINMUX_AUX_SDMMC1_CMD_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x008 => pub PINMUX_AUX_SDMMC1_DAT3_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x00C => pub PINMUX_AUX_SDMMC1_DAT2_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x010 => pub PINMUX_AUX_SDMMC1_DAT1_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x014 => pub PINMUX_AUX_SDMMC1_DAT0_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x018 => _reserved0: [ReadWrite<u8>; 0x4]),
        (0x01C => pub PINMUX_AUX_SDMMC3_CLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x020 => pub PINMUX_AUX_SDMMC3_CMD_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x024 => pub PINMUX_AUX_SDMMC3_DAT0_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x028 => pub PINMUX_AUX_SDMMC3_DAT1_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x02C => pub PINMUX_AUX_SDMMC3_DAT2_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x030 => pub PINMUX_AUX_SDMMC3_DAT3_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x034 => _reserved1: [ReadWrite<u8>; 0x4]),
        (0x038 => pub PINMUX_AUX_PEX_L0_RST_N_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x03C => pub PINMUX_AUX_PEX_L0_CLKREQ_N_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x040 => pub PINMUX_AUX_PEX_WAKE_N_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x044 => pub PINMUX_AUX_PEX_L1_RST_N_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x048 => pub PINMUX_AUX_PEX_L1_CLKREQ_N_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x04C => pub PINMUX_AUX_SATA_LED_ACTIVE_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x050 => pub PINMUX_AUX_SPI1_MOSI_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x054 => pub PINMUX_AUX_SPI1_MISO_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x058 => pub PINMUX_AUX_SPI1_SCK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x05C => pub PINMUX_AUX_SPI1_CS0_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x060 => pub PINMUX_AUX_SPI1_CS1_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x064 => pub PINMUX_AUX_SPI2_MOSI_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x068 => pub PINMUX_AUX_SPI2_MISO_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x06C => pub PINMUX_AUX_SPI2_SCK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x070 => pub PINMUX_AUX_SPI2_CS0_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x074 => pub PINMUX_AUX_SPI2_CS1_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x078 => pub PINMUX_AUX_SPI4_MOSI_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x07C => pub PINMUX_AUX_SPI4_MISO_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x080 => pub PINMUX_AUX_SPI4_SCK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x084 => pub PINMUX_AUX_SPI4_CS0_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x088 => pub PINMUX_AUX_QSPI_SCK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x08C => pub PINMUX_AUX_QSPI_CS_N_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x090 => pub PINMUX_AUX_QSPI_IO0_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x094 => pub PINMUX_AUX_QSPI_IO1_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x098 => pub PINMUX_AUX_QSPI_IO2_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x09C => pub PINMUX_AUX_QSPI_IO3_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0A0 => _reserved3: [ReadWrite<u8>; 0x4]),
        (0x0A4 => pub PINMUX_AUX_DMIC1_CLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0A8 => pub PINMUX_AUX_DMIC1_DAT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0AC => pub PINMUX_AUX_DMIC2_CLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0B0 => pub PINMUX_AUX_DMIC2_DAT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0B4 => pub PINMUX_AUX_DMIC3_CLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0B8 => pub PINMUX_AUX_DMIC3_DAT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0BC => pub PINMUX_AUX_GEN1_I2C_SCL_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0C0 => pub PINMUX_AUX_GEN1_I2C_SDA_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0C4 => pub PINMUX_AUX_GEN2_I2C_SCL_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0C8 => pub PINMUX_AUX_GEN2_I2C_SDA_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0CC => pub PINMUX_AUX_GEN3_I2C_SCL_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0D0 => pub PINMUX_AUX_GEN3_I2C_SDA_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0D4 => pub PINMUX_AUX_CAM_I2C_SCL_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0D8 => pub PINMUX_AUX_CAM_I2C_SDA_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0DC => pub PINMUX_AUX_PWR_I2C_SCL_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0E0 => pub PINMUX_AUX_PWR_I2C_SDA_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0E4 => pub PINMUX_AUX_UART1_TX_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0E8 => pub PINMUX_AUX_UART1_RX_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0EC => pub PINMUX_AUX_UART1_RTS_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0F0 => pub PINMUX_AUX_UART1_CTS_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0F4 => pub PINMUX_AUX_UART2_TX_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0F8 => pub PINMUX_AUX_UART2_RX_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x0FC => pub PINMUX_AUX_UART2_RTS_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x100 => pub PINMUX_AUX_UART2_CTS_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x104 => pub PINMUX_AUX_UART3_TX_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x108 => pub PINMUX_AUX_UART3_RX_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x10C => pub PINMUX_AUX_UART3_RTS_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x110 => pub PINMUX_AUX_UART3_CTS_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x114 => pub PINMUX_AUX_UART4_TX_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x118 => pub PINMUX_AUX_UART4_RX_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x11C => pub PINMUX_AUX_UART4_RTS_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x120 => pub PINMUX_AUX_UART4_CTS_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x124 => pub PINMUX_AUX_DAP1_FS_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x128 => pub PINMUX_AUX_DAP1_DIN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x12C => pub PINMUX_AUX_DAP1_DOUT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x130 => pub PINMUX_AUX_DAP1_SCLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x134 => pub PINMUX_AUX_DAP2_FS_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x138 => pub PINMUX_AUX_DAP2_DIN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x13C => pub PINMUX_AUX_DAP2_DOUT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x140 => pub PINMUX_AUX_DAP2_SCLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x144 => pub PINMUX_AUX_DAP4_FS_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x148 => pub PINMUX_AUX_DAP4_DIN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x14C => pub PINMUX_AUX_DAP4_DOUT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x150 => pub PINMUX_AUX_DAP4_SCLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x154 => pub PINMUX_AUX_CAM1_MCLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x158 => pub PINMUX_AUX_CAM2_MCLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x15C => pub PINMUX_AUX_JTAG_RTCK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x160 => pub PINMUX_AUX_CLK_32K_IN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x164 => pub PINMUX_AUX_CLK_32K_OUT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x168 => pub PINMUX_AUX_BATT_BCL_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x16C => pub PINMUX_AUX_CLK_REQ_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x170 => pub PINMUX_AUX_CPU_PWR_REQ_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x174 => pub PINMUX_AUX_PWR_INT_N_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x178 => pub PINMUX_AUX_SHUTDOWN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x17C => pub PINMUX_AUX_CORE_PWR_REQ_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x180 => pub PINMUX_AUX_AUD_MCLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x184 => pub PINMUX_AUX_DVFS_PWM_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x188 => pub PINMUX_AUX_DVFS_CLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x18C => pub PINMUX_AUX_GPIO_X1_AUD_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x190 => pub PINMUX_AUX_GPIO_X3_AUD_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x194 => pub PINMUX_AUX_GPIO_PCC7_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x198 => pub PINMUX_AUX_HDMI_CEC_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x19C => pub PINMUX_AUX_HDMI_INT_DP_HPD_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1A0 => pub PINMUX_AUX_SPDIF_OUT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1A4 => pub PINMUX_AUX_SPDIF_IN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1A8 => pub PINMUX_AUX_USB_VBUS_EN0_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1AC => pub PINMUX_AUX_USB_VBUS_EN1_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1B0 => pub PINMUX_AUX_DP_HPD0_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1B4 => pub PINMUX_AUX_WIFI_EN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1B8 => pub PINMUX_AUX_WIFI_RST_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1BC => pub PINMUX_AUX_WIFI_WAKE_AP_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1C0 => pub PINMUX_AUX_AP_WAKE_BT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1C4 => pub PINMUX_AUX_BT_RST_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1C8 => pub PINMUX_AUX_BT_WAKE_AP_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1CC => pub PINMUX_AUX_AP_WAKE_NFC_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1D0 => pub PINMUX_AUX_NFC_EN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1D4 => pub PINMUX_AUX_NFC_INT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1D8 => pub PINMUX_AUX_GPS_EN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1DC => pub PINMUX_AUX_GPS_RST_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1E0 => pub PINMUX_AUX_CAM_RST_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1E4 => pub PINMUX_AUX_CAM_AF_EN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1E8 => pub PINMUX_AUX_CAM_FLASH_EN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1EC => pub PINMUX_AUX_CAM1_PWDN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1F0 => pub PINMUX_AUX_CAM2_PWDN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1F4 => pub PINMUX_AUX_CAM1_STROBE_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1F8 => pub PINMUX_AUX_LCD_TE_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x1FC => pub PINMUX_AUX_LCD_BL_PWM_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x200 => pub PINMUX_AUX_LCD_BL_EN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x204 => pub PINMUX_AUX_LCD_RST_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x208 => pub PINMUX_AUX_LCD_GPIO1_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x20C => pub PINMUX_AUX_LCD_GPIO2_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x210 => pub PINMUX_AUX_AP_READY_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x214 => pub PINMUX_AUX_TOUCH_RST_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x218 => pub PINMUX_AUX_TOUCH_CLK_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x21C => pub PINMUX_AUX_MODEM_WAKE_AP_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x220 => pub PINMUX_AUX_TOUCH_INT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x224 => pub PINMUX_AUX_MOTION_INT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x228 => pub PINMUX_AUX_ALS_PROX_INT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x22C => pub PINMUX_AUX_TEMP_ALERT_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x230 => pub PINMUX_AUX_BUTTON_POWER_ON_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x234 => pub PINMUX_AUX_BUTTON_VOL_UP_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x238 => pub PINMUX_AUX_BUTTON_VOL_DOWN_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x23C => pub PINMUX_AUX_BUTTON_SLIDE_SW_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x240 => pub PINMUX_AUX_BUTTON_HOME_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x244 => pub PINMUX_AUX_GPIO_PA6_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x248 => pub PINMUX_AUX_GPIO_PE6_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x24C => pub PINMUX_AUX_GPIO_PE7_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x250 => pub PINMUX_AUX_GPIO_PH6_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x254 => pub PINMUX_AUX_GPIO_PK0_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x258 => pub PINMUX_AUX_GPIO_PK1_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x25C => pub PINMUX_AUX_GPIO_PK2_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x260 => pub PINMUX_AUX_GPIO_PK3_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x264 => pub PINMUX_AUX_GPIO_PK4_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x268 => pub PINMUX_AUX_GPIO_PK5_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x26C => pub PINMUX_AUX_GPIO_PK6_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x270 => pub PINMUX_AUX_GPIO_PK7_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x274 => pub PINMUX_AUX_GPIO_PL0_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x278 => pub PINMUX_AUX_GPIO_PL1_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x27C => pub PINMUX_AUX_GPIO_PZ0_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x280 => pub PINMUX_AUX_GPIO_PZ1_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x284 => pub PINMUX_AUX_GPIO_PZ2_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x288 => pub PINMUX_AUX_GPIO_PZ3_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x28C => pub PINMUX_AUX_GPIO_PZ4_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x290 => pub PINMUX_AUX_GPIO_PZ5_0: ReadWrite<u32, PAD_OPTIONS::Register>),
        (0x294 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x294]);
