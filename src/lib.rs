// Copyright 2020 Tom Burdick
// See LICENSE-APACHE and LICENSE-MIT for license details.

//! This project provides a register access layer (RAL) for all
//! NXP i.mx rt microcontrollers.
//!
//! When built, you must specify a device feature, such as `imxrt1062`.
//! This will cause all modules in that device's module to be re-exported
//! from the top level, so that for example `imxrt_ral::gpio` will resolve to
//! `imxrt_ral::imxrt1062::gpio`.
//!
//! In the generated documentation, all devices are visible inside their family
//! modules, but when built for a specific device, only that devices' constants
//! will be available.

#![cfg_attr(target_arch = "arm", no_std)]
#![allow(clippy::all)]

mod register;
pub mod usage;

pub use crate::register::{RORegister, UnsafeRORegister};
pub use crate::register::{RWRegister, UnsafeRWRegister};
pub use crate::register::{UnsafeWORegister, WORegister};

#[cfg(feature = "doc")]
/// Interrupt sources
///
/// This enum is empty when generating documentation.
/// To see the specific interrupts for your chip, see
/// the `Interrupt` type in your chip-specific module, like
///
/// - [`imxrt101::imxrt1011::Interrupt`](imxrt101::imxrt1011::Interrupt)
/// - [`imxrt106::imxrt1062::Interrupt`](imxrt106::imxrt1062::Interrupt)
/// - etc
///
/// `Interrupt` resolves to those values when building the RAL for
/// your chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interrupt {}

/// The constant for a peripheral with just one instance.
///
/// The CCM peripheral is an example of a "sole instance." On the other
/// hand, no LPUART peripheral will have this constant, since there are
/// multiple instances.
pub const SOLE_INSTANCE: u8 = 0;

/// All supported i.MX RT chips.
///
/// You may query the selected chip ID
/// - at compile time, using the `CHIP_ID` constant.
/// - at run time, through the `IMXRT_RAL_CHIP_ID` static.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ChipId {
    Imxrt1011 = 1011,
    Imxrt1015 = 1015,
    Imxrt1021 = 1021,
    Imxrt1051 = 1051,
    Imxrt1052 = 1052,
    Imxrt1061 = 1061,
    Imxrt1062 = 1062,
    Imxrt1064 = 1064,
}

impl ChipId {
    /// Equate two chip IDs in a constant context.
    pub const fn eq(self, other: ChipId) -> bool {
        self as u32 == other as u32
    }
}

#[cfg(feature = "imxrt1011")]
pub const CHIP_ID: ChipId = ChipId::Imxrt1011;
#[cfg(feature = "imxrt1015")]
pub const CHIP_ID: ChipId = ChipId::Imxrt1015;
#[cfg(feature = "imxrt1021")]
pub const CHIP_ID: ChipId = ChipId::Imxrt1021;
#[cfg(feature = "imxrt1051")]
pub const CHIP_ID: ChipId = ChipId::Imxrt1051;
#[cfg(feature = "imxrt1052")]
pub const CHIP_ID: ChipId = ChipId::Imxrt1052;
#[cfg(feature = "imxrt1061")]
pub const CHIP_ID: ChipId = ChipId::Imxrt1061;
#[cfg(feature = "imxrt1062")]
pub const CHIP_ID: ChipId = ChipId::Imxrt1062;
#[cfg(feature = "imxrt1064")]
pub const CHIP_ID: ChipId = ChipId::Imxrt1064;
#[cfg(any(
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
#[no_mangle]
pub static IMXRT_RAL_CHIP_ID: ChipId = CHIP_ID;

#[cfg(any(feature = "doc", feature = "imxrt1011", feature = "imxrt1015"))]
pub mod imxrt101;

#[cfg(feature = "imxrt1011")]
pub use imxrt101::imxrt1011::*;

#[cfg(feature = "imxrt1015")]
pub use imxrt101::imxrt1015::*;

#[cfg(any(feature = "doc", feature = "imxrt1021"))]
pub mod imxrt102;

#[cfg(feature = "imxrt1021")]
pub use imxrt102::imxrt1021::*;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod imxrt105;

#[cfg(feature = "imxrt1051")]
pub use imxrt105::imxrt1051::*;

#[cfg(feature = "imxrt1052")]
pub use imxrt105::imxrt1052::*;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod imxrt106;

#[cfg(feature = "imxrt1061")]
pub use imxrt106::imxrt1061::*;

#[cfg(feature = "imxrt1062")]
pub use imxrt106::imxrt1062::*;

#[cfg(feature = "imxrt1064")]
pub use imxrt106::imxrt1064::*;
