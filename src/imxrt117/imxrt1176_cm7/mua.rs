#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MUA

use crate::{RORegister, RWRegister};

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// Processor A Transmit Register 0
pub mod TR0 {

    /// TR0
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

/// Processor A Transmit Register 1
pub mod TR1 {
    pub use super::TR0::DATA;
}

/// Processor A Transmit Register 2
pub mod TR2 {
    pub use super::TR0::DATA;
}

/// Processor A Transmit Register 3
pub mod TR3 {
    pub use super::TR0::DATA;
}

/// Processor A Receive Register 0
pub mod RR0 {

    /// RR0
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

/// Processor A Receive Register 1
pub mod RR1 {
    pub use super::RR0::DATA;
}

/// Processor A Receive Register 2
pub mod RR2 {
    pub use super::RR0::DATA;
}

/// Processor A Receive Register 3
pub mod RR3 {
    pub use super::RR0::DATA;
}

/// Processor A Status Register
pub mod SR {

    /// Fn
    pub mod Fn {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: BAFn bit in MUB.CR register is written 0 (default).
            pub const zero: u32 = 0b000;

            /// 0b001: BAFn bit in MUB.CR register is written 1.
            pub const one: u32 = 0b001;
        }
    }

    /// EP
    pub mod EP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The Processor A-side event is not pending (default).
            pub const not_pending: u32 = 0b0;

            /// 0b1: The Processor A-side event is pending.
            pub const pending: u32 = 0b1;
        }
    }

    /// RS
    pub mod RS {
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

            /// 0b0: The Processor B-side of the MU is not in reset.
            pub const not_reset: u32 = 0b0;

            /// 0b1: The Processor B-side of the MU is in reset.
            pub const reset: u32 = 0b1;
        }
    }

    /// FUP
    pub mod FUP {
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

            /// 0b0: No flags updated, initiated by the Processor A, in progress (default)
            pub const no_update: u32 = 0b0;

            /// 0b1: Processor A initiated flags update, processing
            pub const update: u32 = 0b1;
        }
    }

    /// TEn
    pub mod TEn {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: MUA.TRn register is not empty.
            pub const not_empty: u32 = 0b0000;

            /// 0b0001: MUA.TRn register is empty (default).
            pub const empty: u32 = 0b0001;
        }
    }

    /// RFn
    pub mod RFn {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: MUA.RRn register is not full (default).
            pub const not_full: u32 = 0b0000;

            /// 0b0001: MUA.RRn register has received data from MUB.TRn register and is ready to be read by the Processor A.
            pub const full: u32 = 0b0001;
        }
    }

    /// GIPn
    pub mod GIPn {
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

            /// 0b0000: Processor A general purpose interrupt n is not pending. (default)
            pub const not_pending: u32 = 0b0000;

            /// 0b0001: Processor A general purpose interrupt n is pending.
            pub const pending: u32 = 0b0001;
        }
    }
}

/// Processor A Control Register
pub mod CR {

    /// Fn
    pub mod Fn {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: N/A. Self clearing bit (default).
            pub const not_appl: u32 = 0b000;

            /// 0b001: Asserts the Processor A MU reset.
            pub const assert_reset: u32 = 0b001;
        }
    }

    /// MUR
    pub mod MUR {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::Fn::RW;
    }

    /// GIRn
    pub mod GIRn {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Processor A General Interrupt n is not requested to the Processor B (default).
            pub const not_requested: u32 = 0b0000;

            /// 0b0001: Processor A General Interrupt n is requested to the Processor B.
            pub const requested: u32 = 0b0001;
        }
    }

    /// TIEn
    pub mod TIEn {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Disables Processor A Transmit Interrupt n. (default)
            pub const disable: u32 = 0b0000;

            /// 0b0001: Enables Processor A Transmit Interrupt n.
            pub const enable: u32 = 0b0001;
        }
    }

    /// RIEn
    pub mod RIEn {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Disables Processor A Receive Interrupt n. (default)
            pub const disable: u32 = 0b0000;

            /// 0b0001: Enables Processor A Receive Interrupt n.
            pub const enable: u32 = 0b0001;
        }
    }

    /// GIEn
    pub mod GIEn {
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

            /// 0b0000: Disables Processor A General Interrupt n. (default)
            pub const disable: u32 = 0b0000;

            /// 0b0001: Enables Processor A General Interrupt n.
            pub const enable: u32 = 0b0001;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Processor A Transmit Register 0
    pub TR0: RWRegister<u32>,

    /// Processor A Transmit Register 1
    pub TR1: RWRegister<u32>,

    /// Processor A Transmit Register 2
    pub TR2: RWRegister<u32>,

    /// Processor A Transmit Register 3
    pub TR3: RWRegister<u32>,

    /// Processor A Receive Register 0
    pub RR0: RORegister<u32>,

    /// Processor A Receive Register 1
    pub RR1: RORegister<u32>,

    /// Processor A Receive Register 2
    pub RR2: RORegister<u32>,

    /// Processor A Receive Register 3
    pub RR3: RORegister<u32>,

    /// Processor A Status Register
    pub SR: RWRegister<u32>,

    /// Processor A Control Register
    pub CR: RWRegister<u32>,
}
pub struct ResetValues {
    pub TR0: u32,
    pub TR1: u32,
    pub TR2: u32,
    pub TR3: u32,
    pub RR0: u32,
    pub RR1: u32,
    pub RR2: u32,
    pub RR3: u32,
    pub SR: u32,
    pub CR: u32,
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

/// The MUA peripheral instance.
#[cfg(not(feature = "doc"))]
pub type MUA = Instance<0>;

/// The MUA peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type MUA = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct MUA {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for MUA {}
impl crate::Valid for MUA {
    fn take() -> Option<Self> {
        <MUA>::take()
    }
    fn release(self) {
        <MUA>::release(self);
    }
    unsafe fn steal() -> Self {
        <MUA>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static MUA_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the MUA peripheral instance
#[cfg(not(feature = "nosync"))]
impl MUA {
    const INSTANCE: Self = Self {
        addr: 0x40c48000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::MUA],
    };

    /// Reset values for each field in MUA
    pub const reset: ResetValues = ResetValues {
        TR0: 0x00000000,
        TR1: 0x00000000,
        TR2: 0x00000000,
        TR3: 0x00000000,
        RR0: 0x00000000,
        RR1: 0x00000000,
        RR2: 0x00000000,
        RR3: 0x00000000,
        SR: 0x00F00080,
        CR: 0x00000000,
    };

    /// Safe access to MUA
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
        let taken = MUA_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to MUA
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = MUA_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal MUA
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        MUA_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl MUA {
    /// The interrupts associated with MUA
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::MUA];

    /// The interrupts associated with MUA
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to MUA
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const MUA: *const RegisterBlock = 0x40c48000 as *const _;
