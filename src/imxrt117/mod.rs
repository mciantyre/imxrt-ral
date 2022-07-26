//! Parent module for all IMXRT117 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature = "imxrt1176_cm4", feature = "doc"))]
pub mod imxrt1176_cm4;

#[cfg(any(feature = "imxrt1176_cm7", feature = "doc"))]
pub mod imxrt1176_cm7;
