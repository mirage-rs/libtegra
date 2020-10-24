//! System-related APIs for identifying Tegra hardware.
//!
//! Use [`SoC`] to identify the Tegra model, platform and SoC.
//!
//! [`SoC`]: struct.SoC.html

// https://github.com/ARM-software/arm-trusted-firmware/blob/master/plat/nvidia/tegra/common/tegra_platform.c

use crate::apb;

const JEDEC_NVIDIA_MFID: u32 = 0x6B;
const JEDEC_NVIDIA_BKID: u32 = 0x03;

unsafe fn get_chip_id() -> (u32, u32, u32, u32, u32) {
    let hidrev = (&*apb::misc::REGISTERS).gp.APB_MISC_GP_HIDREV_0.get();

    (
        hidrev >> 20 & 0xF, // pre_si_platform revision.
        hidrev >> 16 & 0xF, // Chip ID minor revision.
        hidrev >> 8 & 0xFF, // Chip ID.
        hidrev >> 4 & 0xF,  // Chip ID major revision.
        hidrev & 0xF,       // Chip ID family register.
    )
}

/// Errors that may occur when trying to identify the Tegra hardware.
#[derive(Debug)]
pub enum Error {
    /// [`SoC`] was unable to identify the Tegra model through hardware configuration.
    ///
    /// If you ever encounter this, consider opening an issue on GitHub
    /// (https://github.com/mirage-rs/libtegra) to make it easier to add support for new
    /// models.
    UnknownHardware,
}

/// Tegra model revisions that can be derived from the Chip ID value.
///
/// `Model` may be extended with additional Tegra SoC variants in a minor or patch revision
/// in the future and must not be exhaustively matched against that to preserve compatibility
/// of existing code. Instead, add a `_` catch arm to match future variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Model {
    /// A Tegra132 SoC.
    T132,
    /// A Tegra186 SoC.
    T186,
    /// A Tegra194 SoC.
    T194,
    /// A Tegra210 SoC.
    T210,
    /// `Model` might be extended in the future and must not be exhaustively matched against.
    #[doc(hidden)]
    __NonExhaustive,
}

impl Model {
    pub(crate) fn derive(chip_id: u32) -> Result<Self, Error> {
        match chip_id {
            0x13 => Ok(Model::T132),
            0x18 => Ok(Model::T186),
            0x19 => Ok(Model::T194),
            0x21 => Ok(Model::T210),
            _ => Err(Error::UnknownHardware),
        }
    }
}

/// Platforms that are capable of running a Tegra system.
///
/// `Platform` may be extended with additional Tegra SoC variants in a minor or patch revision
/// in the future and must not be exhaustively matched against that to preserve compatibility
/// of existing code. Instead, add a `_` catch arm to match future variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    /// A silicon Tegra SoC.
    Silicon,
    /// Cadence's QuickTurn emulation system is a Solaris-based chip emulation system.
    Qt,
    /// FPGAs are used during early software/hardware development.
    Fpga,
    /// Chip emulation system.
    Emulation,
    /// Linsim is a reconfigurable, clock-driven, mixed RTL/cmodel simulation framework.
    Linsim,
    /// Unit FPGAs run the actual hardware block IP on the FPGA with the other parts of
    /// the system using Linsim.
    UnitFpga,
    /// The Virtualizer Development Kit (VDK) is the standard chip development from
    /// Synopsis.
    Vdk,
    /// `Platform` might be extended in the future and must not be exhaustively matched against.
    #[doc(hidden)]
    __NonExhaustive,
}

impl Platform {
    pub(crate) fn derive(major: u32, minor: u32, pre_si: u32) -> Result<Self, Error> {
        if major == 0 {
            match minor {
                0 | 2 => Ok(Platform::Qt),
                1 => Ok(Platform::Fpga),
                3 | 4 => Ok(Platform::Linsim),
                5 => Ok(Platform::UnitFpga),
                6 => Ok(Platform::Vdk),
                _ => Err(Error::UnknownHardware),
            }
        } else if pre_si > 0 {
            match pre_si {
                1 | 4 => Ok(Platform::Qt),
                2 => Ok(Platform::Fpga),
                5 | 6 => Ok(Platform::Linsim),
                3 => Ok(Platform::UnitFpga),
                8 => Ok(Platform::Vdk),
                _ => Err(Error::UnknownHardware),
            }
        } else {
            // Actual silicon platforms have a non-zero major version.
            Ok(Platform::Silicon)
        }
    }
}

/// Tegra Chip ID family variants that can be derived from the `HIDFAM` value.
///
/// `Family` may be extended with additional Tegra SoC variants in a minor or patch revision
/// in the future and must not be exhaustively matched against that to preserve compatibility
/// of existing code. Instead, add a `_` catch arm to match future variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    /// GPU family.
    Gpu,
    /// Handheld family.
    Handheld,
    /// BR Chips family.
    BrChips,
    /// CRUSH family.
    Crush,
    /// MCP family.
    Mcp,
    /// CK family.
    Ck,
    /// VAIO family.
    Vaio,
    /// Handheld SoC family.
    HandheldSoC,
    /// `Family` might be extended in the future and must not be exhaustively matched against.
    #[doc(hidden)]
    __NonExhaustive,
}

impl Family {
    pub(crate) fn derive(family: u32) -> Result<Self, Error> {
        match family {
            0 => Ok(Family::Gpu),
            1 => Ok(Family::Handheld),
            2 => Ok(Family::BrChips),
            3 => Ok(Family::Crush),
            4 => Ok(Family::Mcp),
            5 => Ok(Family::Ck),
            6 => Ok(Family::Vaio),
            7 => Ok(Family::HandheldSoC),
            _ => Err(Error::UnknownHardware),
        }
    }
}

/// SoC information of the underlying Tegra platform.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SoC {
    model: Model,
    major: u32,
    minor: u32,
    family: Family,
    platform: Platform,
}

impl SoC {
    /// Constructs a new `SoC` which gives access to the hardware information of the
    /// underlying device.
    pub fn get() -> Result<Self, Error> {
        let (pre_si, minor, chip_id, major, family) = unsafe { get_chip_id() };

        Ok(SoC {
            model: Model::derive(chip_id)?,
            major,
            minor,
            family: Family::derive(family)?,
            platform: Platform::derive(major, minor, pre_si)?,
        })
    }

    /// The underlying Tegra SoC model.
    pub fn model(&self) -> Model {
        self.model
    }

    /// The major Chip ID revision of the underlying Tegra SoC.
    pub fn major(&self) -> u32 {
        self.major
    }

    /// The minor Chip ID revision of the underlying Tegra SoC.
    pub fn minor(&self) -> u32 {
        self.minor
    }

    /// The underlying Tegra Chip ID family.
    pub fn family(&self) -> Family {
        self.family
    }

    /// The underlying Tegra platform kind.
    pub fn platform(&self) -> Platform {
        self.platform
    }

    /// Gets the SoC version which consists of the following bitfield.
    ///
    /// Bit 30:24 - JEP-106 continuation code for the SiP.
    /// Bit 23:16 - JEP-106 identification code with parity bit for the SiP.
    /// Bit 15:0  - Chip identification.
    pub fn soc_version(&self) -> i32 {
        let manfid = (JEDEC_NVIDIA_BKID << 24) | (JEDEC_NVIDIA_MFID << 16);

        (manfid | (self.model as u32 & 0xFFFF)) as i32
    }

    /// Gets the SoC revision which consists of the following bitfield.
    ///
    /// Bit 15:8 - Major version number.
    /// Bit 7:0  - Minor version number.
    pub fn soc_revision(&self) -> i32 {
        (self.major << 8 | self.minor) as i32
    }
}
