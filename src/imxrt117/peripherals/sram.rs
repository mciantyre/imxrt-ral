#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Secure RAM
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// Control Register
pub mod CTRL {

    /// RAM Read Enable (with lock)
    pub mod RAM_RD_EN {
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

            /// 0b0: Disable read access
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable read access
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// RAM Write Enable (with lock)
    pub mod RAM_WR_EN {
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

            /// 0b0: Disable write access
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable write access
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Power Enable (with lock)
    pub mod PWR_EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (4 bits: 0b1111 << 2)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Tamper Block Enable (with lock)
    pub mod TAMPER_BLOCK_EN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Allow R/W access to secure RAM when tamper is detected
            pub const ACCESS: u32 = 0b0;

            /// 0b1: Block R/W access to secure RAM when tamper is detected
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// Turn off power on tamper event (with lock)
    pub mod TAMPER_PWR_OFF_EN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable the turn off function when tamper is detected
            pub const OFF: u32 = 0b0;

            /// 0b1: Turn off power for all secure RAM banks when tamper is detected
            pub const ON: u32 = 0b1;
        }
    }

    /// Lock bits
    pub mod LOCK_BIT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 3072],

    /// Control Register
    pub CTRL: RWRegister<u32>,
}
pub struct ResetValues {
    pub CTRL: u32,
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
