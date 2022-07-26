#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IPS Domain
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// Slot Control Register
pub mod SLOT_CTRL_0 {

    /// Domain ID of the slot to be locked
    pub mod LOCKED_DOMAIN_ID {
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

    /// Lock domain ID of this slot
    pub mod DOMAIN_LOCK {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Do not lock the domain ID
            pub const UNLOCK: u32 = 0b0;

            /// 0b1: Lock the domain ID
            pub const LOCK: u32 = 0b1;
        }
    }

    /// Allow non-secure write access to this domain control register or domain register
    pub mod ALLOW_NONSECURE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Do not allow non-secure write access
            pub const PREVENT: u32 = 0b0;

            /// 0b1: Allow non-secure write access
            pub const ALLOW: u32 = 0b1;
        }
    }

    /// Allow user write access to this domain control register or domain register
    pub mod ALLOW_USER {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Do not allow user write access
            pub const PREVENT: u32 = 0b0;

            /// 0b1: Allow user write access
            pub const ALLOW: u32 = 0b1;
        }
    }

    /// Lock control of this slot
    pub mod LOCK_CONTROL {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Do not lock the control register of this slot
            pub const UNLOCK: u32 = 0b0;

            /// 0b1: Lock the control register of this slot
            pub const LOCK: u32 = 0b1;
        }
    }
}

/// Slot Control Register
pub mod SLOT_CTRL_1 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_2 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_3 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_4 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_5 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_6 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_7 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_8 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_9 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_10 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_11 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_12 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_13 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_14 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_15 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_16 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_17 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_18 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_19 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_20 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_21 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_22 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_23 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_24 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_25 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_26 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_27 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_28 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_29 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_30 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_31 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_32 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_33 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_34 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_35 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_36 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}

/// Slot Control Register
pub mod SLOT_CTRL_37 {
    pub use super::SLOT_CTRL_0::ALLOW_NONSECURE;
    pub use super::SLOT_CTRL_0::ALLOW_USER;
    pub use super::SLOT_CTRL_0::DOMAIN_LOCK;
    pub use super::SLOT_CTRL_0::LOCKED_DOMAIN_ID;
    pub use super::SLOT_CTRL_0::LOCK_CONTROL;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Slot Control Register
    pub SLOT_CTRL_0: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_1: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_2: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_3: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_4: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_5: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_6: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_7: RWRegister<u32>,

    _reserved8: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_8: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_9: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_10: RWRegister<u32>,

    _reserved11: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_11: RWRegister<u32>,

    _reserved12: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_12: RWRegister<u32>,

    _reserved13: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_13: RWRegister<u32>,

    _reserved14: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_14: RWRegister<u32>,

    _reserved15: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_15: RWRegister<u32>,

    _reserved16: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_16: RWRegister<u32>,

    _reserved17: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_17: RWRegister<u32>,

    _reserved18: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_18: RWRegister<u32>,

    _reserved19: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_19: RWRegister<u32>,

    _reserved20: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_20: RWRegister<u32>,

    _reserved21: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_21: RWRegister<u32>,

    _reserved22: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_22: RWRegister<u32>,

    _reserved23: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_23: RWRegister<u32>,

    _reserved24: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_24: RWRegister<u32>,

    _reserved25: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_25: RWRegister<u32>,

    _reserved26: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_26: RWRegister<u32>,

    _reserved27: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_27: RWRegister<u32>,

    _reserved28: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_28: RWRegister<u32>,

    _reserved29: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_29: RWRegister<u32>,

    _reserved30: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_30: RWRegister<u32>,

    _reserved31: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_31: RWRegister<u32>,

    _reserved32: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_32: RWRegister<u32>,

    _reserved33: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_33: RWRegister<u32>,

    _reserved34: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_34: RWRegister<u32>,

    _reserved35: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_35: RWRegister<u32>,

    _reserved36: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_36: RWRegister<u32>,

    _reserved37: [u32; 3],

    /// Slot Control Register
    pub SLOT_CTRL_37: RWRegister<u32>,
}
pub struct ResetValues {
    pub SLOT_CTRL_0: u32,
    pub SLOT_CTRL_1: u32,
    pub SLOT_CTRL_2: u32,
    pub SLOT_CTRL_3: u32,
    pub SLOT_CTRL_4: u32,
    pub SLOT_CTRL_5: u32,
    pub SLOT_CTRL_6: u32,
    pub SLOT_CTRL_7: u32,
    pub SLOT_CTRL_8: u32,
    pub SLOT_CTRL_9: u32,
    pub SLOT_CTRL_10: u32,
    pub SLOT_CTRL_11: u32,
    pub SLOT_CTRL_12: u32,
    pub SLOT_CTRL_13: u32,
    pub SLOT_CTRL_14: u32,
    pub SLOT_CTRL_15: u32,
    pub SLOT_CTRL_16: u32,
    pub SLOT_CTRL_17: u32,
    pub SLOT_CTRL_18: u32,
    pub SLOT_CTRL_19: u32,
    pub SLOT_CTRL_20: u32,
    pub SLOT_CTRL_21: u32,
    pub SLOT_CTRL_22: u32,
    pub SLOT_CTRL_23: u32,
    pub SLOT_CTRL_24: u32,
    pub SLOT_CTRL_25: u32,
    pub SLOT_CTRL_26: u32,
    pub SLOT_CTRL_27: u32,
    pub SLOT_CTRL_28: u32,
    pub SLOT_CTRL_29: u32,
    pub SLOT_CTRL_30: u32,
    pub SLOT_CTRL_31: u32,
    pub SLOT_CTRL_32: u32,
    pub SLOT_CTRL_33: u32,
    pub SLOT_CTRL_34: u32,
    pub SLOT_CTRL_35: u32,
    pub SLOT_CTRL_36: u32,
    pub SLOT_CTRL_37: u32,
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
