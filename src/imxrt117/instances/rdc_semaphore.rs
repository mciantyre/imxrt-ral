#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SEMA42
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::rdc_semaphore::Instance;
pub use crate::imxrt117::peripherals::rdc_semaphore::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::rdc_semaphore::{
    GATE0, GATE1, GATE10, GATE11, GATE12, GATE13, GATE14, GATE15, GATE16, GATE17, GATE18, GATE19,
    GATE2, GATE20, GATE21, GATE22, GATE23, GATE24, GATE25, GATE26, GATE27, GATE28, GATE29, GATE3,
    GATE30, GATE31, GATE32, GATE33, GATE34, GATE35, GATE36, GATE37, GATE38, GATE39, GATE4, GATE40,
    GATE41, GATE42, GATE43, GATE44, GATE45, GATE46, GATE47, GATE48, GATE49, GATE5, GATE50, GATE51,
    GATE52, GATE53, GATE54, GATE55, GATE56, GATE57, GATE58, GATE59, GATE6, GATE60, GATE61, GATE62,
    GATE63, GATE7, GATE8, GATE9, RSTGT_,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The RDC_SEMAPHORE1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type RDC_SEMAPHORE1 = Instance<1>;

/// The RDC_SEMAPHORE1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type RDC_SEMAPHORE1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct RDC_SEMAPHORE1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for RDC_SEMAPHORE1 {}
impl crate::Valid for RDC_SEMAPHORE1 {
    fn take() -> Option<Self> {
        <RDC_SEMAPHORE1>::take()
    }
    fn release(self) {
        <RDC_SEMAPHORE1>::release(self);
    }
    unsafe fn steal() -> Self {
        <RDC_SEMAPHORE1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static RDC_SEMAPHORE1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the RDC_SEMAPHORE1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl RDC_SEMAPHORE1 {
    const INSTANCE: Self = Self {
        addr: 0x40c44000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in RDC_SEMAPHORE1
    pub const reset: ResetValues = ResetValues {
        GATE0: 0x00000000,
        GATE1: 0x00000000,
        GATE2: 0x00000000,
        GATE3: 0x00000000,
        GATE4: 0x00000000,
        GATE5: 0x00000000,
        GATE6: 0x00000000,
        GATE7: 0x00000000,
        GATE8: 0x00000000,
        GATE9: 0x00000000,
        GATE10: 0x00000000,
        GATE11: 0x00000000,
        GATE12: 0x00000000,
        GATE13: 0x00000000,
        GATE14: 0x00000000,
        GATE15: 0x00000000,
        GATE16: 0x00000000,
        GATE17: 0x00000000,
        GATE18: 0x00000000,
        GATE19: 0x00000000,
        GATE20: 0x00000000,
        GATE21: 0x00000000,
        GATE22: 0x00000000,
        GATE23: 0x00000000,
        GATE24: 0x00000000,
        GATE25: 0x00000000,
        GATE26: 0x00000000,
        GATE27: 0x00000000,
        GATE28: 0x00000000,
        GATE29: 0x00000000,
        GATE30: 0x00000000,
        GATE31: 0x00000000,
        GATE32: 0x00000000,
        GATE33: 0x00000000,
        GATE34: 0x00000000,
        GATE35: 0x00000000,
        GATE36: 0x00000000,
        GATE37: 0x00000000,
        GATE38: 0x00000000,
        GATE39: 0x00000000,
        GATE40: 0x00000000,
        GATE41: 0x00000000,
        GATE42: 0x00000000,
        GATE43: 0x00000000,
        GATE44: 0x00000000,
        GATE45: 0x00000000,
        GATE46: 0x00000000,
        GATE47: 0x00000000,
        GATE48: 0x00000000,
        GATE49: 0x00000000,
        GATE50: 0x00000000,
        GATE51: 0x00000000,
        GATE52: 0x00000000,
        GATE53: 0x00000000,
        GATE54: 0x00000000,
        GATE55: 0x00000000,
        GATE56: 0x00000000,
        GATE57: 0x00000000,
        GATE58: 0x00000000,
        GATE59: 0x00000000,
        GATE60: 0x00000000,
        GATE61: 0x00000000,
        GATE62: 0x00000000,
        GATE63: 0x00000000,
        RSTGT_: 0x00000000,
    };

    /// Safe access to RDC_SEMAPHORE1
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
        let taken = RDC_SEMAPHORE1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to RDC_SEMAPHORE1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = RDC_SEMAPHORE1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal RDC_SEMAPHORE1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        RDC_SEMAPHORE1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl RDC_SEMAPHORE1 {
    /// The interrupts associated with RDC_SEMAPHORE1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with RDC_SEMAPHORE1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to RDC_SEMAPHORE1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RDC_SEMAPHORE1: *const RegisterBlock = 0x40c44000 as *const _;

/// The RDC_SEMAPHORE2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type RDC_SEMAPHORE2 = Instance<2>;

/// The RDC_SEMAPHORE2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type RDC_SEMAPHORE2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct RDC_SEMAPHORE2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for RDC_SEMAPHORE2 {}
impl crate::Valid for RDC_SEMAPHORE2 {
    fn take() -> Option<Self> {
        <RDC_SEMAPHORE2>::take()
    }
    fn release(self) {
        <RDC_SEMAPHORE2>::release(self);
    }
    unsafe fn steal() -> Self {
        <RDC_SEMAPHORE2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static RDC_SEMAPHORE2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the RDC_SEMAPHORE2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl RDC_SEMAPHORE2 {
    const INSTANCE: Self = Self {
        addr: 0x40ccc000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in RDC_SEMAPHORE2
    pub const reset: ResetValues = ResetValues {
        GATE0: 0x00000000,
        GATE1: 0x00000000,
        GATE2: 0x00000000,
        GATE3: 0x00000000,
        GATE4: 0x00000000,
        GATE5: 0x00000000,
        GATE6: 0x00000000,
        GATE7: 0x00000000,
        GATE8: 0x00000000,
        GATE9: 0x00000000,
        GATE10: 0x00000000,
        GATE11: 0x00000000,
        GATE12: 0x00000000,
        GATE13: 0x00000000,
        GATE14: 0x00000000,
        GATE15: 0x00000000,
        GATE16: 0x00000000,
        GATE17: 0x00000000,
        GATE18: 0x00000000,
        GATE19: 0x00000000,
        GATE20: 0x00000000,
        GATE21: 0x00000000,
        GATE22: 0x00000000,
        GATE23: 0x00000000,
        GATE24: 0x00000000,
        GATE25: 0x00000000,
        GATE26: 0x00000000,
        GATE27: 0x00000000,
        GATE28: 0x00000000,
        GATE29: 0x00000000,
        GATE30: 0x00000000,
        GATE31: 0x00000000,
        GATE32: 0x00000000,
        GATE33: 0x00000000,
        GATE34: 0x00000000,
        GATE35: 0x00000000,
        GATE36: 0x00000000,
        GATE37: 0x00000000,
        GATE38: 0x00000000,
        GATE39: 0x00000000,
        GATE40: 0x00000000,
        GATE41: 0x00000000,
        GATE42: 0x00000000,
        GATE43: 0x00000000,
        GATE44: 0x00000000,
        GATE45: 0x00000000,
        GATE46: 0x00000000,
        GATE47: 0x00000000,
        GATE48: 0x00000000,
        GATE49: 0x00000000,
        GATE50: 0x00000000,
        GATE51: 0x00000000,
        GATE52: 0x00000000,
        GATE53: 0x00000000,
        GATE54: 0x00000000,
        GATE55: 0x00000000,
        GATE56: 0x00000000,
        GATE57: 0x00000000,
        GATE58: 0x00000000,
        GATE59: 0x00000000,
        GATE60: 0x00000000,
        GATE61: 0x00000000,
        GATE62: 0x00000000,
        GATE63: 0x00000000,
        RSTGT_: 0x00000000,
    };

    /// Safe access to RDC_SEMAPHORE2
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
        let taken = RDC_SEMAPHORE2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to RDC_SEMAPHORE2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = RDC_SEMAPHORE2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal RDC_SEMAPHORE2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        RDC_SEMAPHORE2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl RDC_SEMAPHORE2 {
    /// The interrupts associated with RDC_SEMAPHORE2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with RDC_SEMAPHORE2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to RDC_SEMAPHORE2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RDC_SEMAPHORE2: *const RegisterBlock = 0x40ccc000 as *const _;
