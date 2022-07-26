#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! KEYMGR
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// CSR Master Key Control Register
pub mod MASTER_KEY_CTRL {

    /// Key select for SNVS OTPMK. Default value comes from FUSE_MASTER_KEY_SEL.
    pub mod SELECT {
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

            /// 0b0: select key from UDF
            pub const SELECT_FROM_UDF: u32 = 0b0;

            /// 0b1: If LOCK = 1, select key from PUF, otherwise select key from fuse (bypass the fuse OTPMK to SNVS)
            pub const SELECT_FROM_PUF: u32 = 0b1;
        }
    }

    /// lock this register, prevent from writing. Default value comes from FUSE_MASTER_KEY_SEL_LOCK.
    pub mod LOCK {
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

            /// 0b0: not locked
            pub const UNLOCK: u32 = 0b0;

            /// 0b1: locked
            pub const LOCK: u32 = 0b1;
        }
    }
}

/// CSR OTFAD-1 Key Control
pub mod OTFAD1_KEY_CTRL {

    /// key select for OTFAD-1. Default value comes from FUSE_OTFAD1_KEY_SEL.
    pub mod SELECT {
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

            /// 0b0: Select key from OCOTP USER_KEY5
            pub const SELECT_FROM_USER_KEY5: u32 = 0b0;

            /// 0b1: If PUF_KEY_CTRL\[LOCK\] is 1, select key from PUF, otherwise select key from OCOTP USER_KEY5
            pub const SELECT_FROM_PUF: u32 = 0b1;
        }
    }

    /// lock this register, prevent from writing. Default value comes from FUSE_OTFAD1_KEY_SEL_LOCK.
    pub mod LOCK {
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

            /// 0b0: not locked
            pub const UNLOCK: u32 = 0b0;

            /// 0b1: locked
            pub const LOCK: u32 = 0b1;
        }
    }
}

/// CSR OTFAD-2 Key Control
pub mod OTFAD2_KEY_CTRL {

    /// key select for OTFAD-2. Default value comes from FUSE_OTFAD1_KEY_SEL.
    pub mod SELECT {
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

            /// 0b0: select key from OCOTP USER_KEY5
            pub const SELECT_FROM_USER_KEY5: u32 = 0b0;

            /// 0b1: If PUF_KEY_CTRL\[LOCK\] is 1, select key from PUF, otherwise select key from OCOTP USER_KEY5
            pub const SELECT_FROM_PUF: u32 = 0b1;
        }
    }

    /// lock this register, prevent from writing. Default value comes from FUSE_OTFAD2_KEY_SEL_LOCK.
    pub mod LOCK {
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

            /// 0b0: not locked
            pub const UNLOCK: u32 = 0b0;

            /// 0b1: locked
            pub const LOCK: u32 = 0b1;
        }
    }
}

/// CSR IEE Key Control
pub mod IEE_KEY_CTRL {

    /// Restart load key signal for IEE
    pub mod RELOAD {
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

            /// 0b0: Do nothing
            pub const IDLE: u32 = 0b0;

            /// 0b1: Restart IEE key load flow
            pub const RESTART: u32 = 0b1;
        }
    }
}

/// CSR PUF Key Control
pub mod PUF_KEY_CTRL {

    /// Lock signal for key select
    pub mod LOCK {
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

            /// 0b0: Do not lock the key select
            pub const UNLOCK: u32 = 0b0;

            /// 0b1: Lock the key select to select key from PUF, otherwise bypass key from OCOPT and do not lock. Once it has been set to 1, it cannot be reset manually. It will be set to 0 when the IEE key reload operation is done.
            pub const LOCK: u32 = 0b1;
        }
    }
}

/// Slot 0 Control
pub mod SLOT0_CTRL {

    /// Whitelist
    pub mod WHITE_LIST {
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

    /// Lock whitelist
    pub mod LOCK_LIST {
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

            /// 0b0: Whitelist is not locked
            pub const UNLOCK: u32 = 0b0;

            /// 0b1: Whitelist is locked
            pub const LOCK: u32 = 0b1;
        }
    }

    /// Allow non-secure write access to this register and the slot it controls
    pub mod TZ_NS {
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

    /// Allow user write access to this register and the slot it controls
    pub mod TZ_USER {
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

/// Slot1 Control
pub mod SLOT1_CTRL {
    pub use super::SLOT0_CTRL::LOCK_CONTROL;
    pub use super::SLOT0_CTRL::LOCK_LIST;
    pub use super::SLOT0_CTRL::TZ_NS;
    pub use super::SLOT0_CTRL::TZ_USER;
    pub use super::SLOT0_CTRL::WHITE_LIST;
}

/// Slot2 Control
pub mod SLOT2_CTRL {
    pub use super::SLOT0_CTRL::LOCK_CONTROL;
    pub use super::SLOT0_CTRL::LOCK_LIST;
    pub use super::SLOT0_CTRL::TZ_NS;
    pub use super::SLOT0_CTRL::TZ_USER;
    pub use super::SLOT0_CTRL::WHITE_LIST;
}

/// Slot3 Control
pub mod SLOT3_CTRL {
    pub use super::SLOT0_CTRL::LOCK_CONTROL;
    pub use super::SLOT0_CTRL::LOCK_LIST;
    pub use super::SLOT0_CTRL::TZ_NS;
    pub use super::SLOT0_CTRL::TZ_USER;
    pub use super::SLOT0_CTRL::WHITE_LIST;
}

/// Slot 4 Control
pub mod SLOT4_CTRL {
    pub use super::SLOT0_CTRL::LOCK_CONTROL;
    pub use super::SLOT0_CTRL::LOCK_LIST;
    pub use super::SLOT0_CTRL::TZ_NS;
    pub use super::SLOT0_CTRL::TZ_USER;
    pub use super::SLOT0_CTRL::WHITE_LIST;
}
#[repr(C)]
pub struct RegisterBlock {
    /// CSR Master Key Control Register
    pub MASTER_KEY_CTRL: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// CSR OTFAD-1 Key Control
    pub OTFAD1_KEY_CTRL: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// CSR OTFAD-2 Key Control
    pub OTFAD2_KEY_CTRL: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// CSR IEE Key Control
    pub IEE_KEY_CTRL: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// CSR PUF Key Control
    pub PUF_KEY_CTRL: RWRegister<u32>,

    _reserved5: [u32; 243],

    /// Slot 0 Control
    pub SLOT0_CTRL: RWRegister<u32>,

    /// Slot1 Control
    pub SLOT1_CTRL: RWRegister<u32>,

    /// Slot2 Control
    pub SLOT2_CTRL: RWRegister<u32>,

    /// Slot3 Control
    pub SLOT3_CTRL: RWRegister<u32>,

    /// Slot 4 Control
    pub SLOT4_CTRL: RWRegister<u32>,
}
pub struct ResetValues {
    pub MASTER_KEY_CTRL: u32,
    pub OTFAD1_KEY_CTRL: u32,
    pub OTFAD2_KEY_CTRL: u32,
    pub IEE_KEY_CTRL: u32,
    pub PUF_KEY_CTRL: u32,
    pub SLOT0_CTRL: u32,
    pub SLOT1_CTRL: u32,
    pub SLOT2_CTRL: u32,
    pub SLOT3_CTRL: u32,
    pub SLOT4_CTRL: u32,
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
