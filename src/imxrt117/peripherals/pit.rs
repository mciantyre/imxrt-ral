#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PIT
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// PIT Module Control Register
pub mod MCR {

    /// Freeze
    pub mod FRZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timers continue to run in Debug mode.
            pub const t000001: u32 = 0b0;

            /// 0b1: Timers are stopped in Debug mode.
            pub const t0000011: u32 = 0b1;
        }
    }

    /// Module Disable for PIT
    pub mod MDIS {
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

            /// 0b0: Clock for standard PIT timers is enabled.
            pub const t0301: u32 = 0b0;

            /// 0b1: Clock for standard PIT timers is disabled.
            pub const t00000111: u32 = 0b1;
        }
    }
}

/// PIT Upper Lifetime Timer Register
pub mod LTMR64H {

    /// Life Timer value
    pub mod LTH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PIT Lower Lifetime Timer Register
pub mod LTMR64L {

    /// Life Timer value
    pub mod LTL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Load Value Register
pub mod LDVAL_0 {

    /// Timer Start Value
    pub mod TSV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Current Timer Value Register
pub mod CVAL_0 {

    /// Current Timer Value
    pub mod TVL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Control Register
pub mod TCTRL_0 {

    /// Timer Enable
    pub mod TEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timer n is disabled.
            pub const t02981: u32 = 0b0;

            /// 0b1: Timer n is enabled.
            pub const t008981: u32 = 0b1;
        }
    }

    /// Timer Interrupt Enable
    pub mod TIE {
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

            /// 0b0: Interrupt requests from Timer n are disabled.
            pub const t0331: u32 = 0b0;

            /// 0b1: Interrupt is requested whenever TIF is set.
            pub const t077711: u32 = 0b1;
        }
    }

    /// Chain Mode
    pub mod CHN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timer is not chained.
            pub const timer0001: u32 = 0b0;

            /// 0b1: Timer is chained to a previous timer. For example, for channel 2, if this field is set, Timer 2 is chained to Timer 1.
            pub const timer0081: u32 = 0b1;
        }
    }
}

/// Timer Flag Register
pub mod TFLG_0 {

    /// Timer Interrupt Flag
    pub mod TIF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timeout has not yet occurred.
            pub const t0022331: u32 = 0b0;

            /// 0b1: Timeout has occurred.
            pub const t0022332221: u32 = 0b1;
        }
    }
}

/// Timer Load Value Register
pub mod LDVAL_1 {
    pub use super::LDVAL_0::TSV;
}

/// Current Timer Value Register
pub mod CVAL_1 {
    pub use super::CVAL_0::TVL;
}

/// Timer Control Register
pub mod TCTRL_1 {
    pub use super::TCTRL_0::CHN;
    pub use super::TCTRL_0::TEN;
    pub use super::TCTRL_0::TIE;
}

/// Timer Flag Register
pub mod TFLG_1 {
    pub use super::TFLG_0::TIF;
}

/// Timer Load Value Register
pub mod LDVAL_2 {
    pub use super::LDVAL_0::TSV;
}

/// Current Timer Value Register
pub mod CVAL_2 {
    pub use super::CVAL_0::TVL;
}

/// Timer Control Register
pub mod TCTRL_2 {
    pub use super::TCTRL_0::CHN;
    pub use super::TCTRL_0::TEN;
    pub use super::TCTRL_0::TIE;
}

/// Timer Flag Register
pub mod TFLG_2 {
    pub use super::TFLG_0::TIF;
}

/// Timer Load Value Register
pub mod LDVAL_3 {
    pub use super::LDVAL_0::TSV;
}

/// Current Timer Value Register
pub mod CVAL_3 {
    pub use super::CVAL_0::TVL;
}

/// Timer Control Register
pub mod TCTRL_3 {
    pub use super::TCTRL_0::CHN;
    pub use super::TCTRL_0::TEN;
    pub use super::TCTRL_0::TIE;
}

/// Timer Flag Register
pub mod TFLG_3 {
    pub use super::TFLG_0::TIF;
}
#[repr(C)]
pub struct RegisterBlock {
    /// PIT Module Control Register
    pub MCR: RWRegister<u32>,

    _reserved1: [u32; 55],

    /// PIT Upper Lifetime Timer Register
    pub LTMR64H: RORegister<u32>,

    /// PIT Lower Lifetime Timer Register
    pub LTMR64L: RORegister<u32>,

    _reserved2: [u32; 6],

    /// Timer Load Value Register
    pub LDVAL_0: RWRegister<u32>,

    /// Current Timer Value Register
    pub CVAL_0: RORegister<u32>,

    /// Timer Control Register
    pub TCTRL_0: RWRegister<u32>,

    /// Timer Flag Register
    pub TFLG_0: RWRegister<u32>,

    /// Timer Load Value Register
    pub LDVAL_1: RWRegister<u32>,

    /// Current Timer Value Register
    pub CVAL_1: RORegister<u32>,

    /// Timer Control Register
    pub TCTRL_1: RWRegister<u32>,

    /// Timer Flag Register
    pub TFLG_1: RWRegister<u32>,

    /// Timer Load Value Register
    pub LDVAL_2: RWRegister<u32>,

    /// Current Timer Value Register
    pub CVAL_2: RORegister<u32>,

    /// Timer Control Register
    pub TCTRL_2: RWRegister<u32>,

    /// Timer Flag Register
    pub TFLG_2: RWRegister<u32>,

    /// Timer Load Value Register
    pub LDVAL_3: RWRegister<u32>,

    /// Current Timer Value Register
    pub CVAL_3: RORegister<u32>,

    /// Timer Control Register
    pub TCTRL_3: RWRegister<u32>,

    /// Timer Flag Register
    pub TFLG_3: RWRegister<u32>,
}
pub struct ResetValues {
    pub MCR: u32,
    pub LTMR64H: u32,
    pub LTMR64L: u32,
    pub LDVAL_0: u32,
    pub CVAL_0: u32,
    pub TCTRL_0: u32,
    pub TFLG_0: u32,
    pub LDVAL_1: u32,
    pub CVAL_1: u32,
    pub TCTRL_1: u32,
    pub TFLG_1: u32,
    pub LDVAL_2: u32,
    pub CVAL_2: u32,
    pub TCTRL_2: u32,
    pub TFLG_2: u32,
    pub LDVAL_3: u32,
    pub CVAL_3: u32,
    pub TCTRL_3: u32,
    pub TFLG_3: u32,
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
