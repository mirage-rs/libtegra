//! Various utilities to be used with or within libtegra.
//!
//! # Detecting SoC Revisions
//!
//! Although the crate provides hardware abstractions to Tegra X1 devices,
//! different hardware revisions carry potentially breaking changes that
//! should rather not be mixed up in applications aimed to support all
//! Tegra X1 hardware. For this kind of purpose, this module exposes helper
//! functions to dynamically detect the revision of the SoC at runtime:
//!
//! ```no_run
//! use libtegra::utils::*;
//!
//! if is_mariko() {
//!     // We're on a Tegra X1+ (or Tegra214).
//! } else {
//!     // We're on a Tegra X1 (or Tegra210).
//! }
//! ```

use crate::apb;

// Helpers for extracting chip ID details.
const MAJOR_VERSION_SHIFT: u32 = 0x4;
const MAJOR_VERSION_MASK: u32 = 0xF;
const MINOR_VERSION_SHIFT: u32 = 0x10;
const MINOR_VERSION_MASK: u32 = 0xF;
const CHIP_ID_SHIFT: u32 = 0x8;
const CHIP_ID_MASK: u32 = 0xF;

// Tegra chip ID values.
const TEGRA21_CHIP_ID: u32 = 0x21;

// Tegra chip revisions.
const TEGRA214_REVISION: u32 = 0x2;

/// Reads the chip ID's value from the SoC.
pub fn get_chip_id() -> u32 {
    let apb = unsafe { &*apb::misc::REGISTERS };

    apb.gp.APB_MISC_GP_HIDREV_0.get()
}

/// Extracts the chip's major version from the chip ID value.
#[inline(always)]
pub fn get_chip_id_major(id: u32) -> u32 {
    (id >> MAJOR_VERSION_SHIFT) & MAJOR_VERSION_MASK
}

/// Extracts the chip's minor version from the chip ID value.
#[inline(always)]
pub fn get_chip_id_minor(id: u32) -> u32 {
    (id >> MINOR_VERSION_SHIFT) & MINOR_VERSION_MASK
}

/// Checks whether the SoC is from the Tegra210 generation.
///
/// NOTE: If this function ever returns false, it is advised to make the
/// application panic immediately. Due to differences in the hardware and
/// how it is programmed in previous generations of the Tegra SoC, this
/// crate would be worthless anyway and utilizing it is likely to cause
/// more harm than good.
pub fn is_tegra210() -> bool {
    let chip_id = (get_chip_id() >> CHIP_ID_SHIFT) & CHIP_ID_MASK;

    chip_id == TEGRA21_CHIP_ID
}

/// Checks whether the SoC is from the Tegra210B01 revision through the chip ID.
///
/// NOTE: To check if the platform is a first-generation Tegra X1 SoC,
/// see [`is_tegra210`] for details.
///
/// [`is_tegra210`]: fn.is_tegra210.html
pub fn is_tegra210_b01() -> bool {
    let id = get_chip_id();

    is_tegra210() && get_chip_id_major(id) == TEGRA214_REVISION
}

/// Checks whether the SoC is from the Tegra210 generation.
///
/// NOTE: These SoCs carry the codename "Erista".
#[inline(always)]
pub fn is_erista() -> bool {
    !is_tegra210_b01()
}

/// Checks whether the SoC is from the Tegra214 generation.
///
/// NOTE: These SoCs carry the codename "Mariko".
#[inline(always)]
pub fn is_mariko() -> bool {
    is_tegra210_b01()
}
