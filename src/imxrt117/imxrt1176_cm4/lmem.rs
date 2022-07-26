#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LMEM

use crate::RWRegister;

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// PC bus Cache control register
pub mod PCCCR {

    /// Cache enable
    pub mod ENCACHE {
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

            /// 0b0: Cache disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: Cache enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Enable Write Buffer
    pub mod ENWRBUF {
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

            /// 0b0: Write buffer disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: Write buffer enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Forces all cacheable spaces to write through
    pub mod PCCR2 {
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

            /// 0b0: Does NOT force all cacheable spaces to write through
            pub const PCCR2_0: u32 = 0b0;

            /// 0b1: Forces all cacheable spaces to write through
            pub const PCCR2_1: u32 = 0b1;
        }
    }

    /// Forces no allocation on cache misses
    pub mod PCCR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Allocation on cache misses
            pub const PCCR3_0: u32 = 0b0;

            /// 0b1: Forces no allocation on cache misses (must also have PCCR2 asserted)
            pub const PCCR3_1: u32 = 0b1;
        }
    }

    /// Invalidate Way 0
    pub mod INVW0 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No operation
            pub const no_operation: u32 = 0b0;

            /// 0b1: When setting the GO bit, invalidate all lines in way 0.
            pub const invw0: u32 = 0b1;
        }
    }

    /// Push Way 0
    pub mod PUSHW0 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No operation
            pub const no_operation: u32 = 0b0;

            /// 0b1: When setting the GO bit, push all modified lines in way 0
            pub const pushw0: u32 = 0b1;
        }
    }

    /// Invalidate Way 1
    pub mod INVW1 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No operation
            pub const no_operation: u32 = 0b0;

            /// 0b1: When setting the GO bit, invalidate all lines in way 1
            pub const invw1: u32 = 0b1;
        }
    }

    /// Push Way 1
    pub mod PUSHW1 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No operation
            pub const no_operation: u32 = 0b0;

            /// 0b1: When setting the GO bit, push all modified lines in way 1
            pub const pushw1: u32 = 0b1;
        }
    }

    /// Initiate Cache Command
    pub mod GO {
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

            /// 0b0: Write: no effect. Read: no cache command active.
            pub const no_effect: u32 = 0b0;

            /// 0b1: Write: initiate command indicated by bits 27-24. Read: cache command active.
            pub const init_cmd: u32 = 0b1;
        }
    }
}

/// PC bus Cache line control register
pub mod PCCLCR {

    /// Initiate Cache Line Command
    pub mod LGO {
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

            /// 0b0: Write: no effect. Read: no line command active.
            pub const no_effect: u32 = 0b0;

            /// 0b1: Write: initiate line command indicated by bits 27-24. Read: line command active.
            pub const init_cmd: u32 = 0b1;
        }
    }

    /// Cache address
    pub mod CACHEADDR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (12 bits: 0xfff << 2)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Way select
    pub mod WSEL {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Way 0
            pub const way0: u32 = 0b0;

            /// 0b1: Way 1
            pub const way1: u32 = 0b1;
        }
    }

    /// Tag/Data Select
    pub mod TDSEL {
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

            /// 0b0: Data
            pub const data: u32 = 0b0;

            /// 0b1: Tag
            pub const tag: u32 = 0b1;
        }
    }

    /// Line Command Initial Valid Bit
    pub mod LCIVB {
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

    /// Line Command Initial Modified Bit
    pub mod LCIMB {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Line Command Way
    pub mod LCWAY {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Line Command
    pub mod LCMD {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Search and read or write
            pub const search_rw: u32 = 0b00;

            /// 0b01: Invalidate
            pub const invalidate: u32 = 0b01;

            /// 0b10: Push
            pub const push: u32 = 0b10;

            /// 0b11: Clear
            pub const clear: u32 = 0b11;
        }
    }

    /// Line Address Select
    pub mod LADSEL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Cache address
            pub const cache_addr: u32 = 0b0;

            /// 0b1: Physical address
            pub const phys_addr: u32 = 0b1;
        }
    }

    /// Line access type
    pub mod LACC {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Read
            pub const read: u32 = 0b0;

            /// 0b1: Write
            pub const write: u32 = 0b1;
        }
    }
}

/// PC bus Cache search address register
pub mod PCCSAR {

    /// Initiate Cache Line Command
    pub mod LGO {
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

            /// 0b0: Write: no effect. Read: no line command active.
            pub const no_effect: u32 = 0b0;

            /// 0b1: Write: initiate line command indicated by bits CLCR\[27:24\]. Read: line command active.
            pub const init_cmd: u32 = 0b1;
        }
    }

    /// Physical Address
    pub mod PHYADDR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (31 bits: 0x7fffffff << 1)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PC bus Cache read/write value register
pub mod PCCCVR {

    /// Cache read/write Data
    pub mod DATA {
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

/// PS bus Cache control register
pub mod PSCCR {

    /// Cache enable
    pub mod ENCACHE {
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

            /// 0b0: Cache disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: Cache enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Enable Write Buffer
    pub mod ENWRBUF {
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

            /// 0b0: Write buffer disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: Write buffer enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Forces all cacheable spaces to write through
    pub mod PSCR2 {
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

            /// 0b0: Does NOT force all cacheable spaces to write through
            pub const PSCR2_0: u32 = 0b0;

            /// 0b1: Forces all cacheable spaces to write through
            pub const PSCR2_1: u32 = 0b1;
        }
    }

    /// Forces no allocation on cache misses
    pub mod PSCR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Allocation on cache misses
            pub const PSCR3_0: u32 = 0b0;

            /// 0b1: Forces no allocation on cache misses (must also have PSCR2 asserted)
            pub const PSCR3_1: u32 = 0b1;
        }
    }

    /// Invalidate Way 0
    pub mod INVW0 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No operation
            pub const no_operation: u32 = 0b0;

            /// 0b1: When setting the GO bit, invalidate all lines in way 0.
            pub const invw0: u32 = 0b1;
        }
    }

    /// Push Way 0
    pub mod PUSHW0 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No operation
            pub const no_operation: u32 = 0b0;

            /// 0b1: When setting the GO bit, push all modified lines in way 0
            pub const pushw0: u32 = 0b1;
        }
    }

    /// Invalidate Way 1
    pub mod INVW1 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No operation
            pub const no_operation: u32 = 0b0;

            /// 0b1: When setting the GO bit, invalidate all lines in way 1
            pub const invw1: u32 = 0b1;
        }
    }

    /// Push Way 1
    pub mod PUSHW1 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No operation
            pub const no_operation: u32 = 0b0;

            /// 0b1: When setting the GO bit, push all modified lines in way 1
            pub const pushw1: u32 = 0b1;
        }
    }

    /// Initiate Cache Command
    pub mod GO {
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

            /// 0b0: Write: no effect. Read: no cache command active.
            pub const no_effect: u32 = 0b0;

            /// 0b1: Write: initiate command indicated by bits 27-24. Read: cache command active.
            pub const init_cmd: u32 = 0b1;
        }
    }
}

/// PS bus Cache line control register
pub mod PSCLCR {
    pub use super::PCCLCR::CACHEADDR;
    pub use super::PCCLCR::LACC;
    pub use super::PCCLCR::LADSEL;
    pub use super::PCCLCR::LCIMB;
    pub use super::PCCLCR::LCIVB;
    pub use super::PCCLCR::LCMD;
    pub use super::PCCLCR::LCWAY;
    pub use super::PCCLCR::LGO;
    pub use super::PCCLCR::TDSEL;
    pub use super::PCCLCR::WSEL;
}

/// PS bus Cache search address register
pub mod PSCSAR {
    pub use super::PCCSAR::LGO;
    pub use super::PCCSAR::PHYADDR;
}

/// PS bus Cache read/write value register
pub mod PSCCVR {
    pub use super::PCCCVR::DATA;
}
#[repr(C)]
pub struct RegisterBlock {
    /// PC bus Cache control register
    pub PCCCR: RWRegister<u32>,

    /// PC bus Cache line control register
    pub PCCLCR: RWRegister<u32>,

    /// PC bus Cache search address register
    pub PCCSAR: RWRegister<u32>,

    /// PC bus Cache read/write value register
    pub PCCCVR: RWRegister<u32>,

    _reserved1: [u32; 508],

    /// PS bus Cache control register
    pub PSCCR: RWRegister<u32>,

    /// PS bus Cache line control register
    pub PSCLCR: RWRegister<u32>,

    /// PS bus Cache search address register
    pub PSCSAR: RWRegister<u32>,

    /// PS bus Cache read/write value register
    pub PSCCVR: RWRegister<u32>,
}
pub struct ResetValues {
    pub PCCCR: u32,
    pub PCCLCR: u32,
    pub PCCSAR: u32,
    pub PCCCVR: u32,
    pub PSCCR: u32,
    pub PSCLCR: u32,
    pub PSCSAR: u32,
    pub PSCCVR: u32,
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

/// The LMEM peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LMEM = Instance<0>;

/// The LMEM peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LMEM = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct LMEM {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LMEM {}
impl crate::Valid for LMEM {
    fn take() -> Option<Self> {
        <LMEM>::take()
    }
    fn release(self) {
        <LMEM>::release(self);
    }
    unsafe fn steal() -> Self {
        <LMEM>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LMEM_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LMEM peripheral instance
#[cfg(not(feature = "nosync"))]
impl LMEM {
    const INSTANCE: Self = Self {
        addr: 0xe0082000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in LMEM
    pub const reset: ResetValues = ResetValues {
        PCCCR: 0x00000000,
        PCCLCR: 0x00000000,
        PCCSAR: 0x00000000,
        PCCCVR: 0x00000000,
        PSCCR: 0x00000000,
        PSCLCR: 0x00000000,
        PSCSAR: 0x00000000,
        PSCCVR: 0x00000000,
    };

    /// Safe access to LMEM
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Self> {
        let taken = LMEM_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LMEM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LMEM_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LMEM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LMEM_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LMEM {
    /// The interrupts associated with LMEM
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with LMEM
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LMEM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LMEM: *const RegisterBlock = 0xe0082000 as *const _;
