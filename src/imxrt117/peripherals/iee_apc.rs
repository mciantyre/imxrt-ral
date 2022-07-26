#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IEE_APC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// End address of IEE region (n)
pub mod REGION0_TOP_ADDR {

    /// End address of IEE region
    pub mod TOP_ADDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (29 bits: 0x1fffffff << 0)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Start address of IEE region (n)
pub mod REGION0_BOT_ADDR {

    /// Start address of IEE region
    pub mod BOT_ADDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (29 bits: 0x1fffffff << 0)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Region control of core domain 0 for region (n)
pub mod REGION0_RDC_D0 {

    /// Write disable of core domain 1
    pub mod RDC_D0_WRITE_DIS {
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

            /// 0b0: Write to TOP_ADDR and BOT_ADDR of this region enabled
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Write to TOP_ADDR and BOT_ADDR of this region disabled
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Lock bit for bit 0
    pub mod RDC_D0_LOCK {
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

            /// 0b0: Bit 0 is unlocked
            pub const UNLOCK: u32 = 0b0;

            /// 0b1: Bit 0 is locked
            pub const LOCK: u32 = 0b1;
        }
    }
}

/// Region control of core domain 1 for region (n)
pub mod REGION0_RDC_D1 {

    /// Write disable of core domain 1
    pub mod RDC_D1_WRITE_DIS {
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

            /// 0b0: Write to TOP_ADDR and BOT_ADDR of this region enabled
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Write to TOP_ADDR and BOT_ADDR of this region disabled
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Lock bit for bit 0
    pub mod RDC_D1_LOCK {
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

            /// 0b0: Bit 0 is unlocked
            pub const UNLOCK: u32 = 0b0;

            /// 0b1: Bit 0 is locked
            pub const LOCK: u32 = 0b1;
        }
    }
}

/// End address of IEE region (n)
pub mod REGION1_TOP_ADDR {
    pub use super::REGION0_TOP_ADDR::TOP_ADDR;
}

/// Start address of IEE region (n)
pub mod REGION1_BOT_ADDR {
    pub use super::REGION0_BOT_ADDR::BOT_ADDR;
}

/// Region control of core domain 0 for region (n)
pub mod REGION1_RDC_D0 {
    pub use super::REGION0_RDC_D0::RDC_D0_LOCK;
    pub use super::REGION0_RDC_D0::RDC_D0_WRITE_DIS;
}

/// Region control of core domain 1 for region (n)
pub mod REGION1_RDC_D1 {
    pub use super::REGION0_RDC_D1::RDC_D1_LOCK;
    pub use super::REGION0_RDC_D1::RDC_D1_WRITE_DIS;
}

/// End address of IEE region (n)
pub mod REGION2_TOP_ADDR {
    pub use super::REGION0_TOP_ADDR::TOP_ADDR;
}

/// Start address of IEE region (n)
pub mod REGION2_BOT_ADDR {
    pub use super::REGION0_BOT_ADDR::BOT_ADDR;
}

/// Region control of core domain 0 for region (n)
pub mod REGION2_RDC_D0 {
    pub use super::REGION0_RDC_D0::RDC_D0_LOCK;
    pub use super::REGION0_RDC_D0::RDC_D0_WRITE_DIS;
}

/// Region control of core domain 1 for region (n)
pub mod REGION2_RDC_D1 {
    pub use super::REGION0_RDC_D1::RDC_D1_LOCK;
    pub use super::REGION0_RDC_D1::RDC_D1_WRITE_DIS;
}

/// End address of IEE region (n)
pub mod REGION3_TOP_ADDR {
    pub use super::REGION0_TOP_ADDR::TOP_ADDR;
}

/// Start address of IEE region (n)
pub mod REGION3_BOT_ADDR {
    pub use super::REGION0_BOT_ADDR::BOT_ADDR;
}

/// Region control of core domain 0 for region (n)
pub mod REGION3_RDC_D0 {
    pub use super::REGION0_RDC_D0::RDC_D0_LOCK;
    pub use super::REGION0_RDC_D0::RDC_D0_WRITE_DIS;
}

/// Region control of core domain 1 for region (n)
pub mod REGION3_RDC_D1 {
    pub use super::REGION0_RDC_D1::RDC_D1_LOCK;
    pub use super::REGION0_RDC_D1::RDC_D1_WRITE_DIS;
}

/// End address of IEE region (n)
pub mod REGION4_TOP_ADDR {
    pub use super::REGION0_TOP_ADDR::TOP_ADDR;
}

/// Start address of IEE region (n)
pub mod REGION4_BOT_ADDR {
    pub use super::REGION0_BOT_ADDR::BOT_ADDR;
}

/// Region control of core domain 0 for region (n)
pub mod REGION4_RDC_D0 {
    pub use super::REGION0_RDC_D0::RDC_D0_LOCK;
    pub use super::REGION0_RDC_D0::RDC_D0_WRITE_DIS;
}

/// Region control of core domain 1 for region (n)
pub mod REGION4_RDC_D1 {
    pub use super::REGION0_RDC_D1::RDC_D1_LOCK;
    pub use super::REGION0_RDC_D1::RDC_D1_WRITE_DIS;
}

/// End address of IEE region (n)
pub mod REGION5_TOP_ADDR {
    pub use super::REGION0_TOP_ADDR::TOP_ADDR;
}

/// Start address of IEE region (n)
pub mod REGION5_BOT_ADDR {
    pub use super::REGION0_BOT_ADDR::BOT_ADDR;
}

/// Region control of core domain 0 for region (n)
pub mod REGION5_RDC_D0 {
    pub use super::REGION0_RDC_D0::RDC_D0_LOCK;
    pub use super::REGION0_RDC_D0::RDC_D0_WRITE_DIS;
}

/// Region control of core domain 1 for region (n)
pub mod REGION5_RDC_D1 {
    pub use super::REGION0_RDC_D1::RDC_D1_LOCK;
    pub use super::REGION0_RDC_D1::RDC_D1_WRITE_DIS;
}

/// End address of IEE region (n)
pub mod REGION6_TOP_ADDR {
    pub use super::REGION0_TOP_ADDR::TOP_ADDR;
}

/// Start address of IEE region (n)
pub mod REGION6_BOT_ADDR {
    pub use super::REGION0_BOT_ADDR::BOT_ADDR;
}

/// Region control of core domain 0 for region (n)
pub mod REGION6_RDC_D0 {
    pub use super::REGION0_RDC_D0::RDC_D0_LOCK;
    pub use super::REGION0_RDC_D0::RDC_D0_WRITE_DIS;
}

/// Region control of core domain 1 for region (n)
pub mod REGION6_RDC_D1 {
    pub use super::REGION0_RDC_D1::RDC_D1_LOCK;
    pub use super::REGION0_RDC_D1::RDC_D1_WRITE_DIS;
}

/// End address of IEE region (n)
pub mod REGION7_TOP_ADDR {
    pub use super::REGION0_TOP_ADDR::TOP_ADDR;
}

/// Start address of IEE region (n)
pub mod REGION7_BOT_ADDR {
    pub use super::REGION0_BOT_ADDR::BOT_ADDR;
}

/// Region control of core domain 0 for region (n)
pub mod REGION7_RDC_D0 {
    pub use super::REGION0_RDC_D0::RDC_D0_LOCK;
    pub use super::REGION0_RDC_D0::RDC_D0_WRITE_DIS;
}

/// Region control of core domain 1 for region (n)
pub mod REGION7_RDC_D1 {
    pub use super::REGION0_RDC_D1::RDC_D1_LOCK;
    pub use super::REGION0_RDC_D1::RDC_D1_WRITE_DIS;
}
#[repr(C)]
pub struct RegisterBlock {
    /// End address of IEE region (n)
    pub REGION0_TOP_ADDR: RWRegister<u32>,

    /// Start address of IEE region (n)
    pub REGION0_BOT_ADDR: RWRegister<u32>,

    /// Region control of core domain 0 for region (n)
    pub REGION0_RDC_D0: RWRegister<u32>,

    /// Region control of core domain 1 for region (n)
    pub REGION0_RDC_D1: RWRegister<u32>,

    /// End address of IEE region (n)
    pub REGION1_TOP_ADDR: RWRegister<u32>,

    /// Start address of IEE region (n)
    pub REGION1_BOT_ADDR: RWRegister<u32>,

    /// Region control of core domain 0 for region (n)
    pub REGION1_RDC_D0: RWRegister<u32>,

    /// Region control of core domain 1 for region (n)
    pub REGION1_RDC_D1: RWRegister<u32>,

    /// End address of IEE region (n)
    pub REGION2_TOP_ADDR: RWRegister<u32>,

    /// Start address of IEE region (n)
    pub REGION2_BOT_ADDR: RWRegister<u32>,

    /// Region control of core domain 0 for region (n)
    pub REGION2_RDC_D0: RWRegister<u32>,

    /// Region control of core domain 1 for region (n)
    pub REGION2_RDC_D1: RWRegister<u32>,

    /// End address of IEE region (n)
    pub REGION3_TOP_ADDR: RWRegister<u32>,

    /// Start address of IEE region (n)
    pub REGION3_BOT_ADDR: RWRegister<u32>,

    /// Region control of core domain 0 for region (n)
    pub REGION3_RDC_D0: RWRegister<u32>,

    /// Region control of core domain 1 for region (n)
    pub REGION3_RDC_D1: RWRegister<u32>,

    /// End address of IEE region (n)
    pub REGION4_TOP_ADDR: RWRegister<u32>,

    /// Start address of IEE region (n)
    pub REGION4_BOT_ADDR: RWRegister<u32>,

    /// Region control of core domain 0 for region (n)
    pub REGION4_RDC_D0: RWRegister<u32>,

    /// Region control of core domain 1 for region (n)
    pub REGION4_RDC_D1: RWRegister<u32>,

    /// End address of IEE region (n)
    pub REGION5_TOP_ADDR: RWRegister<u32>,

    /// Start address of IEE region (n)
    pub REGION5_BOT_ADDR: RWRegister<u32>,

    /// Region control of core domain 0 for region (n)
    pub REGION5_RDC_D0: RWRegister<u32>,

    /// Region control of core domain 1 for region (n)
    pub REGION5_RDC_D1: RWRegister<u32>,

    /// End address of IEE region (n)
    pub REGION6_TOP_ADDR: RWRegister<u32>,

    /// Start address of IEE region (n)
    pub REGION6_BOT_ADDR: RWRegister<u32>,

    /// Region control of core domain 0 for region (n)
    pub REGION6_RDC_D0: RWRegister<u32>,

    /// Region control of core domain 1 for region (n)
    pub REGION6_RDC_D1: RWRegister<u32>,

    /// End address of IEE region (n)
    pub REGION7_TOP_ADDR: RWRegister<u32>,

    /// Start address of IEE region (n)
    pub REGION7_BOT_ADDR: RWRegister<u32>,

    /// Region control of core domain 0 for region (n)
    pub REGION7_RDC_D0: RWRegister<u32>,

    /// Region control of core domain 1 for region (n)
    pub REGION7_RDC_D1: RWRegister<u32>,
}
pub struct ResetValues {
    pub REGION0_TOP_ADDR: u32,
    pub REGION0_BOT_ADDR: u32,
    pub REGION0_RDC_D0: u32,
    pub REGION0_RDC_D1: u32,
    pub REGION1_TOP_ADDR: u32,
    pub REGION1_BOT_ADDR: u32,
    pub REGION1_RDC_D0: u32,
    pub REGION1_RDC_D1: u32,
    pub REGION2_TOP_ADDR: u32,
    pub REGION2_BOT_ADDR: u32,
    pub REGION2_RDC_D0: u32,
    pub REGION2_RDC_D1: u32,
    pub REGION3_TOP_ADDR: u32,
    pub REGION3_BOT_ADDR: u32,
    pub REGION3_RDC_D0: u32,
    pub REGION3_RDC_D1: u32,
    pub REGION4_TOP_ADDR: u32,
    pub REGION4_BOT_ADDR: u32,
    pub REGION4_RDC_D0: u32,
    pub REGION4_RDC_D1: u32,
    pub REGION5_TOP_ADDR: u32,
    pub REGION5_BOT_ADDR: u32,
    pub REGION5_RDC_D0: u32,
    pub REGION5_RDC_D1: u32,
    pub REGION6_TOP_ADDR: u32,
    pub REGION6_BOT_ADDR: u32,
    pub REGION6_RDC_D0: u32,
    pub REGION6_RDC_D1: u32,
    pub REGION7_TOP_ADDR: u32,
    pub REGION7_BOT_ADDR: u32,
    pub REGION7_RDC_D0: u32,
    pub REGION7_RDC_D1: u32,
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
