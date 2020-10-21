//! Driver for Tegra X1 Multi-Purpose Pins and Pin Multiplexing.
//!
//! See Chapter 9 in the Tegra X1 Technical Reference Manual for
//! details.
//!
//! # Description
//!
//! Tegra X1 devices can be configured with different I/O functions
//! on particular pins to allow use in a variety of different
//! configurations.
//!
//! Many of the pins on Tegra X1 devices are connected to Multi-Purpose
//! I/O (MPIO) pads. An MPIO can operate in two modes: either acting
//! as a signal for a particular I/O controller, referred to as
//! Special-Function I/O (SFIO) or as a software-controlled
//! General-Purpose I/O function, referred to as [`Gpio`]. Each MPIO
//! has up to four SFIO [`PinFunction`]s as well as being a [`Gpio`].
//!
//! Though each MPIO has up to 5 functions (a [`Gpio`] function and up
//! to 4 SFIO functions), a given MPIO can only act as a single function
//! at a given point in time. The Pinmux controller in Tegra X1 devices
//! includes the logic and registers to select a particular function for
//! each MPIO.
//!
//! ## Configuration
//!
//! Every MPIO on the SoC belongs to a particular [`PinGrP`] which provides
//! the functionality for configuration of [`PinFunction`], [`PinPull`] state,
//! [`PinTristate`] mode, [`PinIo`] settings, [`PinLock`] controls, [`PinOd`]
//! state, and [`PinEIoHv`] mode. All of these options can either be configured
//! individually via a respective method or universally via the [`PinGrP::config`]
//! method.
//!
//! ```no_run
//! use libtegra::pinmux;
//! use libtegra::pinmux::PinGrP;
//!
//! // I2C5 Pinmux configuration for the Nintendo Switch.
//! PinGrP::PwrI2CSclPy3.set_io(pinmux::PinIo::Input);
//! PinGrP::PwrI2CSdaPy4.set_io(pinmux::PinIo::Input);
//! ```
//!
//! [`Gpio`]: ../gpio/struct.Gpio.html
//! [`PinFunction`]: enum.PinFunction.html
//! [`PinGrP`]: enum.PinGrP.html
//! [`PinFunction`]: enum.PinFunction.html
//! [`PinPull`]: enum.PinPull.html
//! [`PinTristate`]: enum.PinTristate.html
//! [`PinIo`]: enum.PinIo.html
//! [`PinLock`]: enum.PinLock.html
//! [`PinOd`]: enum.PinOd.html
//! [`PinEIoHv`]: enum.PinEIoHv.html
//! [`PinGrP::config`]: enum.PinGrP.html#method.config

// Inspired by https://github.com/NVIDIA/tegra-pinmux-scripts.

use register::mmio::ReadWrite;

pub use registers::*;

mod registers;

/// Enumeration over possible Pin Groups.
#[derive(Clone, Copy, Debug, PartialEq)]
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

/// Enumeration over possible Pin Functions.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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

/// Possible Pin Pull-up/down states.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PinPull {
    /// Nothing.
    None,
    /// Enables internal Pull-down resistors.
    Down,
    /// Enables internal Pull-up resistors.
    Up,
}

/// Tri-State (high-Z) option.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PinTristate {
    /// Disables the pad's output driver.
    Passthrough,
    /// Enables the pad's output driver.
    Tristate,
}

/// Parking configurations for LP0.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PinPark {
    /// Default state.
    Default,
    /// Normal state.
    Normal,
    /// Parked state.
    Parked,
}

/// Possible I/O configurations of a pin.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PinIo {
    /// No I/O configuration.
    None,
    /// Output configuration.
    Output,
    /// Input configuration.
    Input,
}

/// Lock control for writing to a pin.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PinLock {
    /// Default state.
    Default,
    /// No lock control.
    Disable,
    /// Lock control.
    Enable,
}

/// Base Driver control settings.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PinLpdr {
    /// Default state.
    Default,
    /// Disables LPDR.
    Disable,
    /// Enables LPDR.
    Enable,
}

/// Reserved.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PinOd {
    /// Default state.
    Default,
    /// Disables Od.
    Disable,
    /// Enables Od.
    ///
    /// NOTE: **NEVER EVER USE THIS!**
    Enable,
}

/// Enumeration over possible operation states.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PinEIoHv {
    /// Default state.
    Default,
    /// Enables regular-voltage operation.
    Normal,
    /// Enables high-voltage operation (3.3V).
    High,
}

/// Schmitt mode configuration options.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum PinSchmt {
    /// Default mode.
    Default,
    /// Disables Schmitt mode on a pin.
    Disable,
    /// Enables Schmitt mode on a pin.
    Enable,
}

/// Representation of a pin group of the SoC, including the respective functions.
#[derive(Debug)]
struct SocPinGrP(PinGrP, [PinFunction; 4]);

/// A representation of the pins of the SoC.
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
    /// Configures this Pin Group with a set of options.
    pub fn config(
        self,
        function: PinFunction,
        pull: PinPull,
        tristate: PinTristate,
        io: PinIo,
        lock: PinLock,
        od: PinOd,
        hv: PinEIoHv,
    ) {
        self.set_function(function);
        self.set_pull(pull);
        self.set_tristate(tristate);
        self.set_io(io);
        self.set_lock(lock);
        self.set_od(od);
        self.set_e_io_hv(hv);
    }

    /// Configures a given Pin Function for this Pin Group.
    pub fn set_function(self, function: PinFunction) {
        // Avoid setting of reserved pins.
        if function == PinFunction::Default
            || function == PinFunction::Reserved
            || self == PinGrP::Reserved
        {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as usize;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4) as u32) as *const ReadWrite<u32>) };

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

    /// Configures a given Pin Pull state for this Pin Group.
    pub fn set_pull(self, pull: PinPull) {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Set the bits accordingly.
        let mut value = register.get();
        value &= !(3 << 2);
        value |= (pull as u32) << 2;
        register.set(value);
    }

    /// Configures a given Tri-State for this Pin Group.
    pub fn set_tristate(self, tristate: PinTristate) {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match tristate {
            PinTristate::Passthrough => value &= !(1 << 4),
            PinTristate::Tristate => value |= 1 << 4,
        };
        register.set(value);
    }

    /// Sets the Parking state for this Pin Group.
    pub fn set_park(self, park: PinPark) {
        // If the state should be default, leave it as-is.
        if park == PinPark::Default {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        let mut value = register.get();
        match park {
            PinPark::Parked => value |= 1 << 5,
            PinPark::Normal => value &= !(1 << 5),
            _ => unreachable!(),
        };
        register.set(value);
    }

    /// Sets a given I/O configuration for this Pin Group.
    pub fn set_io(self, io: PinIo) {
        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match io {
            PinIo::Input => value |= 1 << 6,
            PinIo::Output | PinIo::None => value &= !(1 << 6),
        };
        register.set(value);
    }

    /// Configures a given Lock control state for this Pin Group.
    pub fn set_lock(self, lock: PinLock) {
        // If the state should be default, leave it as-is.
        if lock == PinLock::Default {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match lock {
            PinLock::Enable => value |= 1 << 7,
            PinLock::Disable => value &= !(1 << 7),
            _ => unreachable!(),
        };
        register.set(value);
    }

    /// Configures a given LPDR state for this Pin Group.
    pub fn set_lpdr(self, lpdr: PinLpdr) {
        // If the state should be default, leave it as-is.
        if lpdr == PinLpdr::Default {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match lpdr {
            PinLpdr::Enable => value |= 1 << 7,
            PinLpdr::Disable => value &= !(1 << 7),
            _ => unreachable!(),
        };
        register.set(value);
    }

    /// Configures the given operation voltage state for this Pin Group.
    pub fn set_e_io_hv(self, hv: PinEIoHv) {
        // If the state should be default, leave it as-is.
        if hv == PinEIoHv::Default {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match hv {
            PinEIoHv::High => value |= 1 << 10,
            PinEIoHv::Normal => value &= !(1 << 10),
            _ => unreachable!(),
        };
        register.set(value);
    }

    /// Configures a given OD state for this Pin Group.
    pub fn set_od(self, od: PinOd) {
        // If the state should be default, leave it as-is.
        if od == PinOd::Default {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match od {
            PinOd::Enable => panic!("NEVER EVER SET THIS!"),
            PinOd::Disable => value &= !(1 << 11),
            _ => unreachable!(),
        };
        register.set(value);
    }

    /// Configures Schmitt mode for this Pin Group.
    pub fn set_schmt(self, schmt: PinSchmt) {
        // If the state should be default, leave it as-is.
        if schmt == PinSchmt::Default {
            return;
        }

        // Compute the register offset that corresponds to this pin.
        let pin = self as u32;
        let register = unsafe { &*((PINMUX_BASE + (pin * 4)) as *const ReadWrite<u32>) };

        // Set or clear the bit accordingly.
        let mut value = register.get();
        match schmt {
            PinSchmt::Enable => value |= 1 << 12,
            PinSchmt::Disable => value &= !(1 << 12),
            _ => unreachable!(),
        };
        register.set(value);
    }
}
