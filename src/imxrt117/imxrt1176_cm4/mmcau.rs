#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CAU

use crate::RWRegister;

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// Status Register
pub mod CASR {

    /// Illegal Command
    pub mod IC {
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

            /// 0b0: No illegal commands issued.
            pub const IC_0: u32 = 0b0;

            /// 0b1: Illegal command issued.
            pub const IC_1: u32 = 0b1;
        }
    }

    /// DES Parity Error
    pub mod DPE {
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

            /// 0b0: No error detected.
            pub const DPE_0: u32 = 0b0;

            /// 0b1: DES key parity error detected.
            pub const DPE_1: u32 = 0b1;
        }
    }

    /// CAU Version
    pub mod VER {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0001: Initial CAU version.
            pub const VER_1: u32 = 0b0001;

            /// 0b0010: Second version, added support for SHA-256 algorithm (This is the value on this device).
            pub const VER_2: u32 = 0b0010;
        }
    }
}

/// Accumulator
pub mod CAA {

    /// Accumulator
    pub mod ACC {
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

/// General Purpose Register
pub mod CA0 {

    /// General Purpose Registers
    pub mod CAn {
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

/// General Purpose Register
pub mod CA1 {
    pub use super::CA0::CAn;
}

/// General Purpose Register
pub mod CA2 {
    pub use super::CA0::CAn;
}

/// General Purpose Register
pub mod CA3 {
    pub use super::CA0::CAn;
}

/// General Purpose Register
pub mod CA4 {
    pub use super::CA0::CAn;
}

/// General Purpose Register
pub mod CA5 {
    pub use super::CA0::CAn;
}

/// General Purpose Register
pub mod CA6 {
    pub use super::CA0::CAn;
}

/// General Purpose Register
pub mod CA7 {
    pub use super::CA0::CAn;
}

/// General Purpose Register
pub mod CA8 {
    pub use super::CA0::CAn;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Status Register
    pub CASR: RWRegister<u32>,

    /// Accumulator
    pub CAA: RWRegister<u32>,

    /// General Purpose Register
    pub CA0: RWRegister<u32>,

    /// General Purpose Register
    pub CA1: RWRegister<u32>,

    /// General Purpose Register
    pub CA2: RWRegister<u32>,

    /// General Purpose Register
    pub CA3: RWRegister<u32>,

    /// General Purpose Register
    pub CA4: RWRegister<u32>,

    /// General Purpose Register
    pub CA5: RWRegister<u32>,

    /// General Purpose Register
    pub CA6: RWRegister<u32>,

    /// General Purpose Register
    pub CA7: RWRegister<u32>,

    /// General Purpose Register
    pub CA8: RWRegister<u32>,
}
pub struct ResetValues {
    pub CASR: u32,
    pub CAA: u32,
    pub CA0: u32,
    pub CA1: u32,
    pub CA2: u32,
    pub CA3: u32,
    pub CA4: u32,
    pub CA5: u32,
    pub CA6: u32,
    pub CA7: u32,
    pub CA8: u32,
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

/// The MMCAU peripheral instance.
#[cfg(not(feature = "doc"))]
pub type MMCAU = Instance<0>;

/// The MMCAU peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type MMCAU = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct MMCAU {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for MMCAU {}
impl crate::Valid for MMCAU {
    fn take() -> Option<Self> {
        <MMCAU>::take()
    }
    fn release(self) {
        <MMCAU>::release(self);
    }
    unsafe fn steal() -> Self {
        <MMCAU>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static MMCAU_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the MMCAU peripheral instance
#[cfg(not(feature = "nosync"))]
impl MMCAU {
    const INSTANCE: Self = Self {
        addr: 0xe0081000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in MMCAU
    pub const reset: ResetValues = ResetValues {
        CASR: 0x20000000,
        CAA: 0x00000000,
        CA0: 0x00000000,
        CA1: 0x00000000,
        CA2: 0x00000000,
        CA3: 0x00000000,
        CA4: 0x00000000,
        CA5: 0x00000000,
        CA6: 0x00000000,
        CA7: 0x00000000,
        CA8: 0x00000000,
    };

    /// Safe access to MMCAU
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
        let taken = MMCAU_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to MMCAU
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = MMCAU_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal MMCAU
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        MMCAU_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl MMCAU {
    /// The interrupts associated with MMCAU
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with MMCAU
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to MMCAU
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const MMCAU: *const RegisterBlock = 0xe0081000 as *const _;
