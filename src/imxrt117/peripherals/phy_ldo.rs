#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! no description available
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Analog Control Register CTRL0
pub mod CTRL0 {

    /// LinrReg master enable
    pub mod LINREG_EN {
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

    /// LinReg power-up load disable
    pub mod LINREG_PWRUPLOAD_DIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Internal pull-down enabled
            pub const LINREG_PWRUPLOAD_DIS_0: u32 = 0b0;

            /// 0b1: Internal pull-down disabled
            pub const LINREG_PWRUPLOAD_DIS_1: u32 = 0b1;
        }
    }

    /// LinReg current-limit enable
    pub mod LINREG_ILIMIT_EN {
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

    /// LinReg output voltage target setting
    pub mod LINREG_OUTPUT_TRG {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (5 bits: 0b11111 << 4)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Set output voltage to x.xV
            pub const LINREG_OUTPUT_TRG_0: u32 = 0b00000;

            /// 0b10000: Sets output voltage to 1.0V
            pub const LINREG_OUTPUT_TRG_16: u32 = 0b10000;

            /// 0b11111: Set output voltage to x.xV
            pub const LINREG_OUTPUT_TRG_31: u32 = 0b11111;
        }
    }

    /// Isolation control for attached PHY load
    pub mod LINREG_PHY_ISO_B {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
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

    /// LinrReg master enable
    pub mod LINREG_EN {
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

    /// LinReg power-up load disable
    pub mod LINREG_PWRUPLOAD_DIS {
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

    /// LinReg current-limit enable
    pub mod LINREG_ILIMIT_EN {
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

    /// LinReg output voltage target setting
    pub mod LINREG_OUTPUT_TRG {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (5 bits: 0b11111 << 4)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Isolation control for attached PHY load
    pub mod LINREG_PHY_ISO_B {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
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
pub mod CTRL0_CLR {
    pub use super::CTRL0_SET::LINREG_EN;
    pub use super::CTRL0_SET::LINREG_ILIMIT_EN;
    pub use super::CTRL0_SET::LINREG_OUTPUT_TRG;
    pub use super::CTRL0_SET::LINREG_PHY_ISO_B;
    pub use super::CTRL0_SET::LINREG_PWRUPLOAD_DIS;
}

/// Analog Control Register CTRL0
pub mod CTRL0_TOG {
    pub use super::CTRL0_SET::LINREG_EN;
    pub use super::CTRL0_SET::LINREG_ILIMIT_EN;
    pub use super::CTRL0_SET::LINREG_OUTPUT_TRG;
    pub use super::CTRL0_SET::LINREG_PHY_ISO_B;
    pub use super::CTRL0_SET::LINREG_PWRUPLOAD_DIS;
}

/// Analog Status Register STAT0
pub mod STAT0 {

    /// LinReg Status Bits
    pub mod LINREG_STAT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
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
    pub use super::STAT0::LINREG_STAT;
}

/// Analog Status Register STAT0
pub mod STAT0_CLR {
    pub use super::STAT0::LINREG_STAT;
}

/// Analog Status Register STAT0
pub mod STAT0_TOG {
    pub use super::STAT0::LINREG_STAT;
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
