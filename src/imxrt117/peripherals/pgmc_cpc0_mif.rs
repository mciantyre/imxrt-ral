#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PGMC_MIF
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// MIF Authentication Control
pub mod MIF_AUTHEN_CTRL {

    /// Configuration lock
    pub mod LOCK_CFG {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MIF MLPL control of SLEEP
pub mod MIF_MLPL_SLEEP {

    /// Signal behavior at each MLPL
    pub mod MLPL_CTRL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MIF MLPL control of IG
pub mod MIF_MLPL_IG {
    pub use super::MIF_MLPL_SLEEP::MLPL_CTRL;
}

/// MIF MLPL control of LS
pub mod MIF_MLPL_LS {
    pub use super::MIF_MLPL_SLEEP::MLPL_CTRL;
}

/// MIF MLPL control of HS
pub mod MIF_MLPL_HS {
    pub use super::MIF_MLPL_SLEEP::MLPL_CTRL;
}

/// MIF MLPL control of STDBY
pub mod MIF_MLPL_STDBY {
    pub use super::MIF_MLPL_SLEEP::MLPL_CTRL;
}

/// MIF MLPL control of array power down
pub mod MIF_MLPL_ARR_PDN {
    pub use super::MIF_MLPL_SLEEP::MLPL_CTRL;
}

/// MIF MLPL control of peripheral power down
pub mod MIF_MLPL_PER_PDN {
    pub use super::MIF_MLPL_SLEEP::MLPL_CTRL;
}

/// MIF MLPL control of INITN
pub mod MIF_MLPL_INITN {

    /// Signal behavior at each MLPL
    pub mod MLPL_CTRL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bypass vdd_ok. This field is locked by AUTHEN_CTRL\[LOCK_CFG\] field.
    pub mod BYPASS_VDD_OK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MIF MLPL control of isolation enable
pub mod MIF_MLPL_ISO {
    pub use super::MIF_MLPL_SLEEP::MLPL_CTRL;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 1],

    /// MIF Authentication Control
    pub MIF_AUTHEN_CTRL: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// MIF MLPL control of SLEEP
    pub MIF_MLPL_SLEEP: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// MIF MLPL control of IG
    pub MIF_MLPL_IG: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// MIF MLPL control of LS
    pub MIF_MLPL_LS: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// MIF MLPL control of HS
    pub MIF_MLPL_HS: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// MIF MLPL control of STDBY
    pub MIF_MLPL_STDBY: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// MIF MLPL control of array power down
    pub MIF_MLPL_ARR_PDN: RWRegister<u32>,

    _reserved8: [u32; 3],

    /// MIF MLPL control of peripheral power down
    pub MIF_MLPL_PER_PDN: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// MIF MLPL control of INITN
    pub MIF_MLPL_INITN: RWRegister<u32>,

    _reserved10: [u32; 11],

    /// MIF MLPL control of isolation enable
    pub MIF_MLPL_ISO: RWRegister<u32>,
}
pub struct ResetValues {
    pub MIF_AUTHEN_CTRL: u32,
    pub MIF_MLPL_SLEEP: u32,
    pub MIF_MLPL_IG: u32,
    pub MIF_MLPL_LS: u32,
    pub MIF_MLPL_HS: u32,
    pub MIF_MLPL_STDBY: u32,
    pub MIF_MLPL_ARR_PDN: u32,
    pub MIF_MLPL_PER_PDN: u32,
    pub MIF_MLPL_INITN: u32,
    pub MIF_MLPL_ISO: u32,
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
