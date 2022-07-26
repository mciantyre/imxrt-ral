#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! no description available
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Analog Control Register CTRL0
pub mod CTRL0 {

    /// Master power-down for bandgap module
    pub mod REFTOP_PWD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power-down for bandgap voltage-reference buffer
    pub mod REFTOP_LINREGREF_PWD {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power-down VBGUP detector in bandgap
    pub mod REFTOP_PWDVBGUP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low-power control bit
    pub mod REFTOP_LOWPOWER {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// bandgap self-bias control bit
    pub mod REFTOP_SELFBIASOFF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Analog Control Register CTRL0
pub mod CTRL0_SET {
    pub use super::CTRL0::REFTOP_LINREGREF_PWD;
    pub use super::CTRL0::REFTOP_LOWPOWER;
    pub use super::CTRL0::REFTOP_PWD;
    pub use super::CTRL0::REFTOP_PWDVBGUP;
    pub use super::CTRL0::REFTOP_SELFBIASOFF;
}

/// Analog Control Register CTRL0
pub mod CTRL0_CLR {
    pub use super::CTRL0::REFTOP_LINREGREF_PWD;
    pub use super::CTRL0::REFTOP_LOWPOWER;
    pub use super::CTRL0::REFTOP_PWD;
    pub use super::CTRL0::REFTOP_PWDVBGUP;
    pub use super::CTRL0::REFTOP_SELFBIASOFF;
}

/// Analog Control Register CTRL0
pub mod CTRL0_TOG {
    pub use super::CTRL0::REFTOP_LINREGREF_PWD;
    pub use super::CTRL0::REFTOP_LOWPOWER;
    pub use super::CTRL0::REFTOP_PWD;
    pub use super::CTRL0::REFTOP_PWDVBGUP;
    pub use super::CTRL0::REFTOP_SELFBIASOFF;
}

/// Analog Status Register STAT0
pub mod STAT0 {

    /// Brief description here
    pub mod REFTOP_VBGUP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Brief description here
    pub mod VDD1_PORB {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Brief description here
    pub mod VDD2_PORB {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Brief description here
    pub mod VDD3_PORB {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Analog Status Register STAT0
pub mod STAT0_SET {
    pub use super::STAT0::REFTOP_VBGUP;
    pub use super::STAT0::VDD1_PORB;
    pub use super::STAT0::VDD2_PORB;
    pub use super::STAT0::VDD3_PORB;
}

/// Analog Status Register STAT0
pub mod STAT0_CLR {
    pub use super::STAT0::REFTOP_VBGUP;
    pub use super::STAT0::VDD1_PORB;
    pub use super::STAT0::VDD2_PORB;
    pub use super::STAT0::VDD3_PORB;
}

/// Analog Status Register STAT0
pub mod STAT0_TOG {
    pub use super::STAT0::REFTOP_VBGUP;
    pub use super::STAT0::VDD1_PORB;
    pub use super::STAT0::VDD2_PORB;
    pub use super::STAT0::VDD3_PORB;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Analog Control Register CTRL0
    pub CTRL0: RWRegister<u32>,

    /// Analog Control Register CTRL0
    pub CTRL0_SET: RWRegister<u32>,

    /// Analog Control Register CTRL0
    pub CTRL0_CLR: RWRegister<u32>,

    /// Analog Control Register CTRL0
    pub CTRL0_TOG: RWRegister<u32>,

    _reserved1: [u32; 16],

    /// Analog Status Register STAT0
    pub STAT0: RORegister<u32>,

    /// Analog Status Register STAT0
    pub STAT0_SET: RORegister<u32>,

    /// Analog Status Register STAT0
    pub STAT0_CLR: RORegister<u32>,

    /// Analog Status Register STAT0
    pub STAT0_TOG: RORegister<u32>,
}
pub struct ResetValues {
    pub CTRL0: u32,
    pub CTRL0_SET: u32,
    pub CTRL0_CLR: u32,
    pub CTRL0_TOG: u32,
    pub STAT0: u32,
    pub STAT0_SET: u32,
    pub STAT0_CLR: u32,
    pub STAT0_TOG: u32,
}
pub struct Instance<const N: u8> {
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) addr: u32,
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) intrs: &'static [crate::Interrupt],
}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> ::core::ops::Deref for Instance<N> {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

#[cfg(not(feature = "nosync"))]
unsafe impl<const N: u8> Send for Instance<N> {}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> Instance<N> {
    /// Return the interrupt signals associated with this
    /// peripheral instance
    ///
    /// Collection may be empty if there is no interrupt signal
    /// associated with the peripheral. There's no guarantee for
    /// interrupt signal ordering in the collection.
    #[inline(always)]
    pub const fn interrupts<'a>(&'a self) -> &'a [crate::Interrupt] {
        self.intrs
    }
}
