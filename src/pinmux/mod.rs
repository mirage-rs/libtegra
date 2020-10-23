//! Driver for Tegra X1 Multi-Purpose Pins and Pin Multiplexing.
//!
//! See Chapter 9 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Description
//!
//! Tegra X1 devices can be configured with different I/O functions on particular pins
//! to enable their operation in a variety of different configurations.
//!
//! Many of the pins on Tegra X1 devices are connected to Multi-Purpose I/O (MPIO) pads.
//! An MPIO can operate in two modes: either acting as a signal for a particular I/O controller,
//! referred to as Special-Function I/O (SFIO) or as a software-controlled General-Purpose
//! I/O function, referred to as [`Gpio`].
//!
//! Though each MPIO has up to 5 functions (a GPIO function and up to 4 SFIO functions),
//! a given MPIO can only act as a single function at a given point in time. The Pinmux
//! controller in Tegra X1 devices includes the logic and registers to select a particular
//! function for each MPIO.
//!
//! # Configuration
//!
//! Various `get_` and `set_` methods are provided to configure [`PinGrP`]s (Pin Group Pads)
//! or to query their state. Many devices depend on proper Pin Multiplexing, so this module
//! provides the required functionality to drive the desired pins.
//!
//! # Safety
//!
//! Many of the configuration methods on a [`PinGrP`] are actually considered `unsafe` because
//! wrong usage of them can cause permanent damage to the hardware which is at the user's risk.
//!
//! [`Gpio`]: ../gpio/struct.Gpio.html
//! [`PinGrP`]: enum.PinGrP.html

// Inspired by https://github.com/NVIDIA/tegra-pinmux-scripts.

use enum_primitive::FromPrimitive;
use register::mmio::ReadWrite;

pub use registers::*;

mod registers;

/// Pin Groups on the Tegra X1 SoC that can be customized and variably configured.
///
/// Many drivers of the `libtegra` crate depend on proper Pin Multiplexing settings
/// for the specific board before they can be used.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum PinGrP {
    Sdmmc1ClkPm0,
    Sdmmc1CmdPm1,
    Sdmmc1Dat3Pm2,
    Sdmmc1Dat2Pm3,
    Sdmmc1Dat1Pm4,
    Sdmmc1Dat0Pm5,
    Sdmmc3ClkPp0 = 0x1C / 4,
    Sdmmc3CmdPp1,
    Sdmmc3Dat0Pp5,
    Sdmmc3Dat1Pp4,
    Sdmmc3Dat2Pp3,
    Sdmmc3Dat3Pp2,
    PexL0RstNPa0 = 0x38 / 4,
    PexL0ClkreqNPa1,
    PexWakeNPa2,
    PexL1RstNPa3,
    PexL1ClkreqNPa4,
    SataLedActivePa5,
    Spi1MosiPc0,
    Spi1MisoPc1,
    Spi1SckPc2,
    Spi1Cs0Pc3,
    Spi1Cs1Pc4,
    Spi2MosiPb4,
    Spi2MisoPb5,
    Spi2SckPb6,
    Spi2Cs0Pb7,
    Spi2Cs1Pdd0,
    Spi4MosiPc7,
    Spi4MisoPd0,
    Spi4SckPc5,
    Spi4Cs0Pc6,
    QspiSckPee0,
    QspiCsNPee1,
    QspiIo0Pee2,
    QspiIo1Pee3,
    QspiIo2Pee4,
    QspiIo3Pee5,
    Dmic1ClkPe0 = 0xA4 / 4,
    Dmic1DatPe1,
    Dmic2ClkPe2,
    Dmic2DatPe3,
    Dmic3ClkPe4,
    Dmic3DatPe5,
    Gen1I2CSclPj1,
    Gen1I2CSdaPj0,
    Gen2I2CSclPj2,
    Gen2I2CSdaPj3,
    Gen3I2CSclPf0,
    Gen3I2CSdaPf1,
    CamI2CSclPs2,
    CamI2CSdaPs3,
    PwrI2CSclPy3,
    PwrI2CSdaPy4,
    Uart1TxPu0,
    Uart1RxPu1,
    Uart1RtsPu2,
    Uart1CtsPu3,
    Uart2TxPg0,
    Uart2RxPg1,
    Uart2RtsPg2,
    Uart2CtsPg3,
    Uart3TxPd1,
    Uart3RxPd2,
    Uart3RtsPd3,
    Uart3CtsPd4,
    Uart4TxPi4,
    Uart4RxPi5,
    Uart4RtsPi6,
    Uart4CtsPi7,
    Dap1FsPb0,
    Dap1DinPb1,
    Dap1DoutPb2,
    Dap1SclkPb3,
    Dap2FsPaa0,
    Dap2DinPaa2,
    Dap2DoutPaa3,
    Dap2SclkPaa1,
    Dap4FsPj4,
    Dap4DinPj5,
    Dap4DoutPj6,
    Dap4SclkPj7,
    Cam1MclkPs0,
    Cam2MclkPs1,
    JtagRtck,
    Clk32KIn,
    Clk32KOutPy5,
    BattBcl,
    ClkReq,
    CpuPwrReq,
    PwrIntN,
    Shutdown,
    CorePwrReq,
    AudMclkPbb0,
    DvfsPwmPbb1,
    DvfsClkPbb2,
    GpioX1AudPbb3,
    GpioX3AudPbb4,
    Pcc7,
    HdmiCecPcc0,
    HdmiIntDpHpdPcc1,
    SpdifOutPcc2,
    SpdifInPcc3,
    UsbVbusEn0Pcc4,
    UsbVbusEn1Pcc5,
    DpHpd0Pcc6,
    WifiEnPh0,
    WifiRstPh1,
    WifiWakeApPh2,
    ApWakeBtPh3,
    BtRstPh4,
    BtWakeApPh5,
    ApWakeNfcPh7,
    NfcEnPi0,
    NfcIntPi1,
    GpsEnPi2,
    GpsRstPi3,
    CamRstPs4,
    CamAfEnPs5,
    CamFlashEnPs6,
    Cam1PwdnPs7,
    Cam2PwdnPt0,
    Cam1StrobePt1,
    LcdTePy2,
    LcdBlPwmPv0,
    LcdBlEnPv1,
    LcdRstPv2,
    LcdGpio1Pv3,
    LcdGpio2Pv4,
    ApReadyPv5,
    TouchRstPv6,
    TouchClkPv7,
    ModemWakeApPx0,
    TouchIntPx1,
    MotionIntPx2,
    AlsProxIntPx3,
    TempAlertPx4,
    ButtonPowerOnPx5,
    ButtonVolUpPx6,
    ButtonVolDownPx7,
    ButtonSlideSwPy0,
    ButtonHomePy1,
    Pa6,
    Pe6,
    Pe7,
    Ph6,
    Pk0,
    Pk1,
    Pk2,
    Pk3,
    Pk4,
    Pk5,
    Pk6,
    Pk7,
    Pl0,
    Pl1,
    Pz0,
    Pz1,
    Pz2,
    Pz3,
    Pz4,
    Pz5,

    // not expected
    Reserved,
}

enum_from_primitive! {
    /// Pin Functions that can be loaded onto a pin to control its behavior.
    ///
    /// Note that most pins are actually predestined for a specific pin functions among
    /// reserved ones. The driver will make sure that no unsupported pin function can be
    /// set on the wrong pin.
    #[derive(Debug, PartialEq, PartialOrd)]
    pub enum PinFunction {
        Default,
        Aud,
        Bcl,
        Blink,
        Ccla,
        Cec,
        Cldvfs,
        Clk,
        Core,
        Cpu,
        Displaya,
        Displayb,
        Dmic1,
        Dmic2,
        Dmic3,
        Dp,
        Dtv,
        Extperiph3,
        I2C1,
        I2C2,
        I2C3,
        I2Cpmu,
        I2Cvi,
        I2S1,
        I2S2,
        I2S3,
        I2S4A,
        I2S4B,
        I2S5A,
        I2S5B,
        Iqc0,
        Iqc1,
        Jtag,
        Pe,
        Pe0,
        Pe1,
        Pmi,
        Pwm0,
        Pwm1,
        Pwm2,
        Pwm3,
        Qspi,
        Sata,
        Sdmmc1,
        Sdmmc3,
        Shutdown,
        Soc,
        Sor0,
        Sor1,
        Spdif,
        Spi1,
        Spi2,
        Spi3,
        Spi4,
        Sys,
        Touch,
        Uart,
        Uarta,
        Uartb,
        Uartc,
        Uartd,
        Usb,
        Vgp1,
        Vgp2,
        Vgp3,
        Vgp4,
        Vgp5,
        Vgp6,
        Vimclk,
        Vimclk2,
        Rsvd0,
        Rsvd1,
        Rsvd2,
        Rsvd3,

        // not expected
        Reserved,
    }
}

enum_from_primitive! {
    /// State control for the Pull bit as a part of Pin configuration.
    ///
    /// The Pull Resistor bit can be configured to Pull-up, Pull-down or nothing per pad.
    #[derive(Debug, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum PinPull {
        /// Nothing.
        None,
        /// Enables internal Pull-down resistors.
        Down,
        /// Enables internal Pull-up resistors.
        Up,
        /// Reserved.
        Reserved,
    }
}

enum_from_primitive! {
    /// State control for the Tristate bit as a part of Pin configuration.
    ///
    /// Enables or disables the pad's output driver and thus overrides whether the pad is
    /// used as an SFIO or GPIO. For normal operations, a pad should be set to Passthrough.
    #[derive(Debug, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum PinTristate {
        /// Disables the pad's Tristate.
        Passthrough,
        /// Enables the pad's Tristate.
        Tristate,
    }
}

enum_from_primitive! {
    /// State control for the Parking bit as a part of Pin configuration.
    ///
    /// This state holds control during LP0 (Deep Sleep) and on LP0 entry, most pads will
    /// be put into PARKING state. Until pinmux recovery code on LP0 exit clears this bit,
    /// the pin will remain in LP0 state in the same value as that of LP0 entry. A Default
    /// state is provided to preserve the setting of this bit without touching it.
    #[derive(Debug, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum PinPark {
        /// Normal state.
        Normal,
        /// Parked state.
        Parked,
        /// Default state.
        Default,
    }
}

enum_from_primitive! {
    /// State control for the I/O direction bit as a part of Pin configuration.
    ///
    /// Depending on whether or not the Input Receiver of a pad is enabled through this setting,
    /// the pad will have either input or output direction for I/O.
    #[derive(Debug, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum PinIo {
        /// Output direction configuration.
        Output,
        /// Input direction configuration.
        Input,
    }
}

enum_from_primitive! {
    /// State control for the Lock bit as a part of Pin configuration.
    ///
    /// The Lock bit, as the name may suggest, locks down or grants write access
    /// to the pad that is configured with it, respectively. A Default state is
    /// provided to preserve the setting of the bit without touching it.
    #[derive(Debug, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum PinLock {
        /// No lock control.
        Disable,
        /// Lock control.
        Enable,
        /// Default state.
        Default,
    }
}

enum_from_primitive! {
    /// State control for the Base Driver control bit as a part of pin configuration.
    ///
    /// When interfacing chips require minimal rise and fall times, this can be set to
    /// enable Base Drivers to fine-tune their behavior. A Default state is provided
    /// to preserve the setting of this bit without touching it.
    #[derive(Debug, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum PinLpdr {
        /// Disables LPDR.
        Disable,
        /// Enables LPDR.
        Enable,
        /// Default state.
        Default,
    }
}

enum_from_primitive! {
    /// State control for the OD control bit as a part of pin configuration.
    ///
    /// This is marked as reserved by the Tegra Reference Manual and should never be set
    /// to another value than its default. For that reason, a Default state is provided
    /// to enforce this rule and skip direct modification of the bit's value.
    #[derive(Debug, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum PinOd {
        /// Disables OD.
        Disable,
        /// Enables OD.
        ///
        /// NOTE: **NEVER EVER USE THIS!**
        Enable,
        /// Default state.
        Default,
    }
}

enum_from_primitive! {
    /// State control for the I/O High Voltage control bit as a part of pin configuration.
    ///
    /// If pins are in need of 3.3V operation, open-drain pull-up capability can be enabled
    /// on them using this setting. A Default state is provided to preserve the setting of
    /// this bit without touching it.
    #[derive(Debug, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum PinIoHv {
        /// Enables regular-voltage operation.
        Normal,
        /// Enables high-voltage operation (3.3V).
        High,
        /// Default state.
        Default,
    }
}

enum_from_primitive! {
    /// State control for the Schmitt Trigger control bit as a part of pin configuration.
    ///
    /// Enables or disables the Schmitt trigger on a given pad. A Default state is provided
    /// to preserve the setting of this bit without touching it.
    #[derive(Debug, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum PinSchmt {
        /// Disables Schmitt mode on a pin.
        Disable,
        /// Enables Schmitt mode on a pin.
        Enable,
        /// Default mode.
        Default,
    }
}

enum_from_primitive! {
    /// State control for the Drive Type control bit as a part of pin configuration.
    ///
    /// Enables different combinations of impedance code mapping on a given pad. This
    /// can be configured for every supported pad individually and allows for fine-tuning
    /// the impedance of an individual pad.
    #[derive(Debug, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum PinDrive {
        /// 1X drive.
        Drive1X,
        /// 2X drive.
        Drive2X,
        /// 3X drive.
        Drive3X,
        /// 4X drive.
        Drive4X,
    }
}

#[derive(Debug)]
struct SocPinGrP(PinGrP, [PinFunction; 4]);

const SOC_PINS: [SocPinGrP; 165] = [
    SocPinGrP(
        PinGrP::Sdmmc1ClkPm0,
        [
            PinFunction::Sdmmc1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Sdmmc1CmdPm1,
        [
            PinFunction::Sdmmc1,
            PinFunction::Spi3,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Sdmmc1Dat3Pm2,
        [
            PinFunction::Sdmmc1,
            PinFunction::Spi3,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Sdmmc1Dat2Pm3,
        [
            PinFunction::Sdmmc1,
            PinFunction::Spi3,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Sdmmc1Dat1Pm4,
        [
            PinFunction::Sdmmc1,
            PinFunction::Spi3,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Sdmmc1Dat0Pm5,
        [
            PinFunction::Sdmmc1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Reserved,
        [
            PinFunction::Reserved,
            PinFunction::Reserved,
            PinFunction::Reserved,
            PinFunction::Reserved,
        ],
    ),
    SocPinGrP(
        PinGrP::Sdmmc3ClkPp0,
        [
            PinFunction::Sdmmc3,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Sdmmc3CmdPp1,
        [
            PinFunction::Sdmmc3,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Sdmmc3Dat0Pp5,
        [
            PinFunction::Sdmmc3,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Sdmmc3Dat1Pp4,
        [
            PinFunction::Sdmmc3,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Sdmmc3Dat2Pp3,
        [
            PinFunction::Sdmmc3,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Sdmmc3Dat3Pp2,
        [
            PinFunction::Sdmmc3,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Reserved,
        [
            PinFunction::Reserved,
            PinFunction::Reserved,
            PinFunction::Reserved,
            PinFunction::Reserved,
        ],
    ),
    SocPinGrP(
        PinGrP::PexL0RstNPa0,
        [
            PinFunction::Pe0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::PexL0ClkreqNPa1,
        [
            PinFunction::Pe0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::PexWakeNPa2,
        [
            PinFunction::Pe,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::PexL1RstNPa3,
        [
            PinFunction::Pe1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::PexL1ClkreqNPa4,
        [
            PinFunction::Pe1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::SataLedActivePa5,
        [
            PinFunction::Sata,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi1MosiPc0,
        [
            PinFunction::Spi1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi1MisoPc1,
        [
            PinFunction::Spi1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi1SckPc2,
        [
            PinFunction::Spi1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi1Cs0Pc3,
        [
            PinFunction::Spi1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi1Cs1Pc4,
        [
            PinFunction::Spi1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi2MosiPb4,
        [
            PinFunction::Spi2,
            PinFunction::Dtv,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi2MisoPb5,
        [
            PinFunction::Spi2,
            PinFunction::Dtv,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi2SckPb6,
        [
            PinFunction::Spi2,
            PinFunction::Dtv,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi2Cs0Pb7,
        [
            PinFunction::Spi2,
            PinFunction::Dtv,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi2Cs1Pdd0,
        [
            PinFunction::Spi2,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi4MosiPc7,
        [
            PinFunction::Spi4,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi4MisoPd0,
        [
            PinFunction::Spi4,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi4SckPc5,
        [
            PinFunction::Spi4,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Spi4Cs0Pc6,
        [
            PinFunction::Spi4,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::QspiSckPee0,
        [
            PinFunction::Qspi,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::QspiCsNPee1,
        [
            PinFunction::Qspi,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::QspiIo0Pee2,
        [
            PinFunction::Qspi,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::QspiIo1Pee3,
        [
            PinFunction::Qspi,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::QspiIo2Pee4,
        [
            PinFunction::Qspi,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::QspiIo3Pee5,
        [
            PinFunction::Qspi,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Reserved,
        [
            PinFunction::Reserved,
            PinFunction::Reserved,
            PinFunction::Reserved,
            PinFunction::Reserved,
        ],
    ),
    SocPinGrP(
        PinGrP::Dmic1ClkPe0,
        [
            PinFunction::Dmic1,
            PinFunction::I2S3,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dmic1DatPe1,
        [
            PinFunction::Dmic1,
            PinFunction::I2S3,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dmic2ClkPe2,
        [
            PinFunction::Dmic2,
            PinFunction::I2S3,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dmic2DatPe3,
        [
            PinFunction::Dmic2,
            PinFunction::I2S3,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dmic3ClkPe4,
        [
            PinFunction::Dmic3,
            PinFunction::I2S5A,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dmic3DatPe5,
        [
            PinFunction::Dmic3,
            PinFunction::I2S5A,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Gen1I2CSclPj1,
        [
            PinFunction::I2C1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Gen1I2CSdaPj0,
        [
            PinFunction::I2C1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Gen2I2CSclPj2,
        [
            PinFunction::I2C2,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Gen2I2CSdaPj3,
        [
            PinFunction::I2C2,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Gen3I2CSclPf0,
        [
            PinFunction::I2C3,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Gen3I2CSdaPf1,
        [
            PinFunction::I2C3,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::CamI2CSclPs2,
        [
            PinFunction::I2C3,
            PinFunction::I2Cvi,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::CamI2CSdaPs3,
        [
            PinFunction::I2C3,
            PinFunction::I2Cvi,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::PwrI2CSclPy3,
        [
            PinFunction::I2Cpmu,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::PwrI2CSdaPy4,
        [
            PinFunction::I2Cpmu,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart1TxPu0,
        [
            PinFunction::Uarta,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart1RxPu1,
        [
            PinFunction::Uarta,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart1RtsPu2,
        [
            PinFunction::Uarta,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart1CtsPu3,
        [
            PinFunction::Uarta,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart2TxPg0,
        [
            PinFunction::Uartb,
            PinFunction::I2S4A,
            PinFunction::Spdif,
            PinFunction::Uart,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart2RxPg1,
        [
            PinFunction::Uartb,
            PinFunction::I2S4A,
            PinFunction::Spdif,
            PinFunction::Uart,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart2RtsPg2,
        [
            PinFunction::Uartb,
            PinFunction::I2S4A,
            PinFunction::Rsvd2,
            PinFunction::Uart,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart2CtsPg3,
        [
            PinFunction::Uartb,
            PinFunction::I2S4A,
            PinFunction::Rsvd2,
            PinFunction::Uart,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart3TxPd1,
        [
            PinFunction::Uartc,
            PinFunction::Spi4,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart3RxPd2,
        [
            PinFunction::Uartc,
            PinFunction::Spi4,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart3RtsPd3,
        [
            PinFunction::Uartc,
            PinFunction::Spi4,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart3CtsPd4,
        [
            PinFunction::Uartc,
            PinFunction::Spi4,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart4TxPi4,
        [
            PinFunction::Uartd,
            PinFunction::Uart,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart4RxPi5,
        [
            PinFunction::Uartd,
            PinFunction::Uart,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart4RtsPi6,
        [
            PinFunction::Uartd,
            PinFunction::Uart,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Uart4CtsPi7,
        [
            PinFunction::Uartd,
            PinFunction::Uart,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dap1FsPb0,
        [
            PinFunction::I2S1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dap1DinPb1,
        [
            PinFunction::I2S1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dap1DoutPb2,
        [
            PinFunction::I2S1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dap1SclkPb3,
        [
            PinFunction::I2S1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dap2FsPaa0,
        [
            PinFunction::I2S2,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dap2DinPaa2,
        [
            PinFunction::I2S2,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dap2DoutPaa3,
        [
            PinFunction::I2S2,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dap2SclkPaa1,
        [
            PinFunction::I2S2,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dap4FsPj4,
        [
            PinFunction::I2S4B,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dap4DinPj5,
        [
            PinFunction::I2S4B,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dap4DoutPj6,
        [
            PinFunction::I2S4B,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Dap4SclkPj7,
        [
            PinFunction::I2S4B,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Cam1MclkPs0,
        [
            PinFunction::Extperiph3,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Cam2MclkPs1,
        [
            PinFunction::Extperiph3,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::JtagRtck,
        [
            PinFunction::Jtag,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Clk32KIn,
        [
            PinFunction::Clk,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Clk32KOutPy5,
        [
            PinFunction::Soc,
            PinFunction::Blink,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::BattBcl,
        [
            PinFunction::Bcl,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::ClkReq,
        [
            PinFunction::Sys,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::CpuPwrReq,
        [
            PinFunction::Cpu,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::PwrIntN,
        [
            PinFunction::Pmi,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Shutdown,
        [
            PinFunction::Shutdown,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::CorePwrReq,
        [
            PinFunction::Core,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::AudMclkPbb0,
        [
            PinFunction::Aud,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::DvfsPwmPbb1,
        [
            PinFunction::Rsvd0,
            PinFunction::Cldvfs,
            PinFunction::Spi3,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::DvfsClkPbb2,
        [
            PinFunction::Rsvd0,
            PinFunction::Cldvfs,
            PinFunction::Spi3,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::GpioX1AudPbb3,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Spi3,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::GpioX3AudPbb4,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Spi3,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pcc7,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::HdmiCecPcc0,
        [
            PinFunction::Cec,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::HdmiIntDpHpdPcc1,
        [
            PinFunction::Dp,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::SpdifOutPcc2,
        [
            PinFunction::Spdif,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::SpdifInPcc3,
        [
            PinFunction::Spdif,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::UsbVbusEn0Pcc4,
        [
            PinFunction::Usb,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::UsbVbusEn1Pcc5,
        [
            PinFunction::Usb,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::DpHpd0Pcc6,
        [
            PinFunction::Dp,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::WifiEnPh0,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::WifiRstPh1,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::WifiWakeApPh2,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::ApWakeBtPh3,
        [
            PinFunction::Rsvd0,
            PinFunction::Uartb,
            PinFunction::Spdif,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::BtRstPh4,
        [
            PinFunction::Rsvd0,
            PinFunction::Uartb,
            PinFunction::Spdif,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::BtWakeApPh5,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::ApWakeNfcPh7,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::NfcEnPi0,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::NfcIntPi1,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::GpsEnPi2,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::GpsRstPi3,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::CamRstPs4,
        [
            PinFunction::Vgp1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::CamAfEnPs5,
        [
            PinFunction::Vimclk,
            PinFunction::Vgp2,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::CamFlashEnPs6,
        [
            PinFunction::Vimclk,
            PinFunction::Vgp3,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Cam1PwdnPs7,
        [
            PinFunction::Vgp4,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Cam2PwdnPt0,
        [
            PinFunction::Vgp5,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Cam1StrobePt1,
        [
            PinFunction::Vgp6,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::LcdTePy2,
        [
            PinFunction::Displaya,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::LcdBlPwmPv0,
        [
            PinFunction::Displaya,
            PinFunction::Pwm0,
            PinFunction::Sor0,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::LcdBlEnPv1,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::LcdRstPv2,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::LcdGpio1Pv3,
        [
            PinFunction::Displayb,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::LcdGpio2Pv4,
        [
            PinFunction::Displayb,
            PinFunction::Pwm1,
            PinFunction::Rsvd2,
            PinFunction::Sor1,
        ],
    ),
    SocPinGrP(
        PinGrP::ApReadyPv5,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::TouchRstPv6,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::TouchClkPv7,
        [
            PinFunction::Touch,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::ModemWakeApPx0,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::TouchIntPx1,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::MotionIntPx2,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::AlsProxIntPx3,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::TempAlertPx4,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::ButtonPowerOnPx5,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::ButtonVolUpPx6,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::ButtonVolDownPx7,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::ButtonSlideSwPy0,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::ButtonHomePy1,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pa6,
        [
            PinFunction::Sata,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pe6,
        [
            PinFunction::Rsvd0,
            PinFunction::I2S5A,
            PinFunction::Pwm2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pe7,
        [
            PinFunction::Rsvd0,
            PinFunction::I2S5A,
            PinFunction::Pwm3,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Ph6,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pk0,
        [
            PinFunction::Iqc0,
            PinFunction::I2S5B,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pk1,
        [
            PinFunction::Iqc0,
            PinFunction::I2S5B,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pk2,
        [
            PinFunction::Iqc0,
            PinFunction::I2S5B,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pk3,
        [
            PinFunction::Iqc0,
            PinFunction::I2S5B,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pk4,
        [
            PinFunction::Iqc1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pk5,
        [
            PinFunction::Iqc1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pk6,
        [
            PinFunction::Iqc1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pk7,
        [
            PinFunction::Iqc1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pl0,
        [
            PinFunction::Rsvd0,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pl1,
        [
            PinFunction::Soc,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pz0,
        [
            PinFunction::Vimclk2,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pz1,
        [
            PinFunction::Vimclk2,
            PinFunction::Sdmmc1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pz2,
        [
            PinFunction::Sdmmc3,
            PinFunction::Ccla,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pz3,
        [
            PinFunction::Sdmmc3,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pz4,
        [
            PinFunction::Sdmmc1,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
    SocPinGrP(
        PinGrP::Pz5,
        [
            PinFunction::Soc,
            PinFunction::Rsvd1,
            PinFunction::Rsvd2,
            PinFunction::Rsvd3,
        ],
    ),
];

impl PinGrP {
    /// Configures a given Pin Function for this Pin Group.
    ///
    /// Applicable to all pads, but only certain functions may be supported per pad.
    ///
    /// # Panics
    ///
    /// Panics if the user attempts to set an unsupported pin function on this pad.
    ///
    /// # Safety
    ///
    /// Playing around with Pin Multiplexing settings can irreparably damage your hardware,
    /// please make sure that you know exactly what you are doing before calling this
    /// function.
    pub unsafe fn set_function(self, function: PinFunction) {
        // Avoid setting of reserved pins.
        if function == PinFunction::Default
            || function == PinFunction::Reserved
            || self == PinGrP::Reserved
        {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as usize;
        let register = &*((PINMUX_BASE + (pin * 4) as u32) as *const ReadWrite<u32>);

        // Compute the corresponding mux value.
        let mux = if function >= PinFunction::Rsvd0 {
            (function as u32 - PinFunction::Rsvd0 as u32) & 3
        } else {
            let mut i = 0;
            for func in &SOC_PINS[pin].1 {
                if *func == function {
                    break;
                }
                i += 1;
            }

            if i > 3 {
                panic!("Invalid mux value!");
            }

            i
        };

        // Set the bits accordingly.
        let mut value = register.get();
        value &= !(3 << 0);
        value |= mux << 0;
        register.set(value);
    }

    /// Extracts the currently configured Pull resistor state from this Pin Group.
    pub fn get_pull(self) -> PinPull {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Extract the desired bit and wrap it into the enum.
        let value = (register.get() & (3 << 2)) >> 2;
        PinPull::from_u32(value).unwrap()
    }

    /// Configures a given Pull resistor state for this Pin Group.
    ///
    /// Applicable to all pads.
    ///
    /// # Safety
    ///
    /// Playing around with Pin Multiplexing settings can irreparably damage your hardware,
    /// please make sure that you know exactly what you are doing before calling this
    /// function.
    pub unsafe fn set_pull(self, pull: PinPull) {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>);

        // Set the bits accordingly.
        let mut value = register.get();
        value &= !(3 << 2);
        value |= (pull as u32) << 2;
        register.set(value);
    }

    /// Extracts the currently configured Tri-State from this Pin Group.
    pub fn get_tristate(self) -> PinTristate {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Extract the desired bit and wrap it into the enum.
        let value = (register.get() & (1 << 4)) >> 4;
        PinTristate::from_u32(value).unwrap()
    }

    /// Configures a given Tri-State for this Pin Group.
    ///
    /// Applicable to all pads.
    ///
    /// # Safety
    ///
    /// Playing around with Pin Multiplexing settings can irreparably damage your hardware,
    /// please make sure that you know exactly what you are doing before calling this
    /// function.
    pub unsafe fn set_tristate(self, tristate: PinTristate) {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>);

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match tristate {
            PinTristate::Passthrough => value &= !(1 << 4),
            PinTristate::Tristate => value |= 1 << 4,
        };
        register.set(value);
    }

    /// Extracts the currently configured Parking state from this Pin Group.
    pub fn get_park(self) -> PinPark {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Extract the desired bit and wrap it into the enum.
        let value = (register.get() & (1 << 5)) >> 5;
        PinPark::from_u32(value).unwrap()
    }

    /// Sets the Parking state for this Pin Group.
    ///
    /// Applicable to all pads except for a few ones in the AO region. This method does not
    /// validate the settings it is being called with, the user must ensure that they are
    /// calling this method correctly. See the advice below.
    ///
    /// # Safety
    ///
    /// Playing around with Pin Multiplexing settings can irreparably damage your hardware,
    /// please make sure that you know exactly what you are doing before calling this
    /// function.
    pub unsafe fn set_park(self, park: PinPark) {
        // If the state should be default, leave it as-is.
        if park == PinPark::Default {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>);

        let mut value = register.get();
        match park {
            PinPark::Parked => value |= 1 << 5,
            PinPark::Normal => value &= !(1 << 5),
            _ => unreachable!(),
        };
        register.set(value);
    }

    /// Extracts the currently configured I/O state from this Pin Group.
    pub fn get_io(self) -> PinIo {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Extract the desired bit and wrap it into the enum.
        let value = (register.get() & (1 << 6)) >> 6;
        PinIo::from_u32(value).unwrap()
    }

    /// Sets a given I/O configuration for this Pin Group.
    ///
    /// Applicable to all pads.
    ///
    /// # Safety
    ///
    /// Playing around with Pin Multiplexing settings can irreparably damage your hardware,
    /// please make sure that you know exactly what you are doing before calling this
    /// function.
    pub unsafe fn set_io(self, io: PinIo) {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>);

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match io {
            PinIo::Input => value |= 1 << 6,
            PinIo::Output => value &= !(1 << 6),
        };
        register.set(value);
    }

    /// Extracts the currently configured Lock control state from this Pin Group.
    pub fn get_lock(self) -> PinLock {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Extract the desired bit and wrap it into the enum.
        let value = (register.get() & (1 << 7)) >> 7;
        PinLock::from_u32(value).unwrap()
    }

    /// Configures a given Lock control state for this Pin Group.
    ///
    /// Applicable to all pads.
    ///
    /// # Safety
    ///
    /// Playing around with Pin Multiplexing settings can irreparably damage your hardware,
    /// please make sure that you know exactly what you are doing before calling this
    /// function.
    pub unsafe fn set_lock(self, lock: PinLock) {
        // If the state should be default, leave it as-is.
        if lock == PinLock::Default {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>);

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match lock {
            PinLock::Enable => value |= 1 << 7,
            PinLock::Disable => value &= !(1 << 7),
            _ => unreachable!(),
        };
        register.set(value);
    }

    /// Extracts the currently configured LPDR state from this Pin Group.
    pub fn get_lpdr(self) -> PinLpdr {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Extract the desired bit and wrap it into the enum.
        let value = (register.get() & (1 << 8)) >> 8;
        PinLpdr::from_u32(value).unwrap()
    }

    /// Configures a given LPDR state for this Pin Group.
    ///
    /// Applicable to ST and DD pads. This method does not validate the settings it is
    /// being called with, the user must ensure that they are calling this method
    /// correctly. See the advice below.
    ///
    /// # Safety
    ///
    /// Playing around with Pin Multiplexing settings can irreparably damage your hardware,
    /// please make sure that you know exactly what you are doing before calling this
    /// function.
    pub unsafe fn set_lpdr(self, lpdr: PinLpdr) {
        // If the state should be default, leave it as-is.
        if lpdr == PinLpdr::Default {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>);

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match lpdr {
            PinLpdr::Enable => value |= 1 << 8,
            PinLpdr::Disable => value &= !(1 << 8),
            _ => unreachable!(),
        };
        register.set(value);
    }

    /// Extracts the currently configured operation voltage state from this Pin Group.
    pub fn get_io_hv(self) -> PinIoHv {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Extract the desired bit and wrap it into the enum.
        let value = (register.get() & (1 << 10)) >> 10;
        PinIoHv::from_u32(value).unwrap()
    }

    /// Configures the given operation voltage state for this Pin Group.
    ///
    /// Applicable to DD pads. This method does not validate the settings it is being called
    /// with, the user must ensure that they are calling this method correctly. See the
    /// advice below.
    ///
    /// # Safety
    ///
    /// Playing around with Pin Multiplexing settings can irreparably damage your hardware,
    /// please make sure that you know exactly what you are doing before calling this
    /// function.
    pub unsafe fn set_io_hv(self, hv: PinIoHv) {
        // If the state should be default, leave it as-is.
        if hv == PinIoHv::Default {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>);

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match hv {
            PinIoHv::High => value |= 1 << 10,
            PinIoHv::Normal => value &= !(1 << 10),
            _ => unreachable!(),
        };
        register.set(value);
    }

    /// Extracts the currently configured OD state from this Pin Group.
    pub fn get_od(self) -> PinOd {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Extract the desired bit and wrap it into the enum.
        let value = (register.get() & (1 << 11)) >> 11;
        PinOd::from_u32(value).unwrap()
    }

    /// Configures a given OD state for this Pin Group.
    ///
    /// This field is marked and should never be set by user code for any pin.
    ///
    /// # Safety
    ///
    /// Playing around with Pin Multiplexing settings can irreparably damage your hardware,
    /// please make sure that you know exactly what you are doing before calling this
    /// function.
    pub unsafe fn set_od(self, od: PinOd) {
        // If the state should be default, leave it as-is.
        if od == PinOd::Default {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>);

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match od {
            PinOd::Enable => panic!("NEVER EVER SET THIS!"),
            PinOd::Disable => value &= !(1 << 11),
            _ => unreachable!(),
        };
        register.set(value);
    }

    /// Extracts the currently configured Schmitt state from this Pin Group.
    pub fn get_schmt(self) -> PinSchmt {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Extract the desired bit and wrap it into the enum.
        let value = (register.get() & (1 << 12)) >> 12;
        PinSchmt::from_u32(value).unwrap()
    }

    /// Configures Schmitt mode for this Pin Group.
    ///
    /// Applicable to all pads.
    ///
    /// # Safety
    ///
    /// Playing around with Pin Multiplexing settings can irreparably damage your hardware,
    /// please make sure that you know exactly what you are doing before calling this
    /// function.
    pub unsafe fn set_schmt(self, schmt: PinSchmt) {
        // If the state should be default, leave it as-is.
        if schmt == PinSchmt::Default {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>);

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match schmt {
            PinSchmt::Enable => value |= 1 << 12,
            PinSchmt::Disable => value &= !(1 << 12),
            _ => unreachable!(),
        };
        register.set(value);
    }

    /// Extracts the currently configured impedance state from this Pin Group.
    pub fn get_drive(self) -> PinDrive {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Extract the desired bit and wrap it into the enum.
        let value = (register.get() & (3 << 13)) >> 13;
        PinDrive::from_u32(value).unwrap()
    }

    /// Configures a given impedance drive for this Pin Group.
    ///
    /// Applicable to CZ/LV_CZ pads. This method does not validate whether the drive
    /// setting is valid on the pin, the user must ensure that they are calling this
    /// method correctly. See the advice below.
    ///
    /// # Safety
    ///
    /// Playing around with Pin Multiplexing settings can irreparably damage your hardware,
    /// please make sure that you know exactly what you are doing before calling this
    /// function.
    pub unsafe fn set_drive(self, drive: PinDrive) {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>);

        // Set the bits accordingly.
        let mut value = register.get();
        value &= !(3 << 13);
        value |= (drive as u32) << 13;
        register.set(value);
    }
}
