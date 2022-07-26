#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CM7_MCM

use crate::RWRegister;

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// Interrupt Status and Control Register
pub mod ISCR {

    /// Write Abort on Slave
    pub mod WABS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No abort
            pub const noabort: u32 = 0b0;

            /// 0b1: Abort
            pub const abort: u32 = 0b1;
        }
    }

    /// Write Abort on Slave Overrun
    pub mod WABSO {
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

            /// 0b0: No write abort overrun
            pub const no: u32 = 0b0;

            /// 0b1: Write abort overrun occurred
            pub const yes: u32 = 0b1;
        }
    }

    /// FPU Invalid Operation interrupt Status
    pub mod FIOC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No interrupt
            pub const No: u32 = 0b0;

            /// 0b1: Interrupt occured
            pub const Yes: u32 = 0b1;
        }
    }

    /// FPU Divide-by-Zero Interrupt Status
    pub mod FDZC {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIOC::RW;
    }

    /// FPU Overflow interrupt status
    pub mod FOFC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIOC::RW;
    }

    /// FPU Underflow Interrupt Status
    pub mod FUFC {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIOC::RW;
    }

    /// FPU Inexact Interrupt Status
    pub mod FIXC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIOC::RW;
    }

    /// FPU Input Denormal Interrupt Status
    pub mod FIDC {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIOC::RW;
    }

    /// TCM Write Abort Interrupt enable
    pub mod WABE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable interrupt
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable interrupt
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// FPU Invalid Operation Interrupt Enable
    pub mod FIOCE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WABE::RW;
    }

    /// FPU Divide-by-Zero Interrupt Enable
    pub mod FDZCE {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WABE::RW;
    }

    /// FPU Overflow Interrupt Enable
    pub mod FOFCE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WABE::RW;
    }

    /// FPU Underflow Interrupt Enable
    pub mod FUFCE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WABE::RW;
    }

    /// FPU Inexact Interrupt Enable
    pub mod FIXCE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WABE::RW;
    }

    /// FPU Input Denormal Interrupt Enable
    pub mod FIDCE {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WABE::RW;
    }
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 4],

    /// Interrupt Status and Control Register
    pub ISCR: RWRegister<u32>,
}
pub struct ResetValues {
    pub ISCR: u32,
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

/// The CM7_MCM peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CM7_MCM = Instance<0>;

/// The CM7_MCM peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CM7_MCM = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct CM7_MCM {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for CM7_MCM {}
impl crate::Valid for CM7_MCM {
    fn take() -> Option<Self> {
        <CM7_MCM>::take()
    }
    fn release(self) {
        <CM7_MCM>::release(self);
    }
    unsafe fn steal() -> Self {
        <CM7_MCM>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CM7_MCM_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CM7_MCM peripheral instance
#[cfg(not(feature = "nosync"))]
impl CM7_MCM {
    const INSTANCE: Self = Self {
        addr: 0xe0080000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in CM7_MCM
    pub const reset: ResetValues = ResetValues { ISCR: 0x00000000 };

    /// Safe access to CM7_MCM
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
        let taken = CM7_MCM_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CM7_MCM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CM7_MCM_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CM7_MCM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CM7_MCM_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CM7_MCM {
    /// The interrupts associated with CM7_MCM
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with CM7_MCM
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CM7_MCM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CM7_MCM: *const RegisterBlock = 0xe0080000 as *const _;
