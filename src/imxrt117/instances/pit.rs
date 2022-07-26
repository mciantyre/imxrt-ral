#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PIT
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::pit::Instance;
pub use crate::imxrt117::peripherals::pit::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::pit::{
    CVAL_0, CVAL_1, CVAL_2, CVAL_3, LDVAL_0, LDVAL_1, LDVAL_2, LDVAL_3, LTMR64H, LTMR64L, MCR,
    TCTRL_0, TCTRL_1, TCTRL_2, TCTRL_3, TFLG_0, TFLG_1, TFLG_2, TFLG_3,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The PIT1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PIT1 = Instance<1>;

/// The PIT1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PIT1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct PIT1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PIT1 {}
impl crate::Valid for PIT1 {
    fn take() -> Option<Self> {
        <PIT1>::take()
    }
    fn release(self) {
        <PIT1>::release(self);
    }
    unsafe fn steal() -> Self {
        <PIT1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PIT1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PIT1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PIT1 {
    const INSTANCE: Self = Self {
        addr: 0x400d8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::PIT1],
    };

    /// Reset values for each field in PIT1
    pub const reset: ResetValues = ResetValues {
        MCR: 0x00000002,
        LTMR64H: 0x00000000,
        LTMR64L: 0x00000000,
        LDVAL_0: 0x00000000,
        CVAL_0: 0x00000000,
        TCTRL_0: 0x00000000,
        TFLG_0: 0x00000000,
        LDVAL_1: 0x00000000,
        CVAL_1: 0x00000000,
        TCTRL_1: 0x00000000,
        TFLG_1: 0x00000000,
        LDVAL_2: 0x00000000,
        CVAL_2: 0x00000000,
        TCTRL_2: 0x00000000,
        TFLG_2: 0x00000000,
        LDVAL_3: 0x00000000,
        CVAL_3: 0x00000000,
        TCTRL_3: 0x00000000,
        TFLG_3: 0x00000000,
    };

    /// Safe access to PIT1
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
        let taken = PIT1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PIT1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PIT1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PIT1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PIT1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PIT1 {
    /// The interrupts associated with PIT1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::PIT1];

    /// The interrupts associated with PIT1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PIT1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PIT1: *const RegisterBlock = 0x400d8000 as *const _;

/// The PIT2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PIT2 = Instance<2>;

/// The PIT2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PIT2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct PIT2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PIT2 {}
impl crate::Valid for PIT2 {
    fn take() -> Option<Self> {
        <PIT2>::take()
    }
    fn release(self) {
        <PIT2>::release(self);
    }
    unsafe fn steal() -> Self {
        <PIT2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PIT2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PIT2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PIT2 {
    const INSTANCE: Self = Self {
        addr: 0x40cb0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::PIT2],
    };

    /// Reset values for each field in PIT2
    pub const reset: ResetValues = ResetValues {
        MCR: 0x00000002,
        LTMR64H: 0x00000000,
        LTMR64L: 0x00000000,
        LDVAL_0: 0x00000000,
        CVAL_0: 0x00000000,
        TCTRL_0: 0x00000000,
        TFLG_0: 0x00000000,
        LDVAL_1: 0x00000000,
        CVAL_1: 0x00000000,
        TCTRL_1: 0x00000000,
        TFLG_1: 0x00000000,
        LDVAL_2: 0x00000000,
        CVAL_2: 0x00000000,
        TCTRL_2: 0x00000000,
        TFLG_2: 0x00000000,
        LDVAL_3: 0x00000000,
        CVAL_3: 0x00000000,
        TCTRL_3: 0x00000000,
        TFLG_3: 0x00000000,
    };

    /// Safe access to PIT2
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
        let taken = PIT2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PIT2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PIT2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PIT2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PIT2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PIT2 {
    /// The interrupts associated with PIT2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::PIT2];

    /// The interrupts associated with PIT2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PIT2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PIT2: *const RegisterBlock = 0x40cb0000 as *const _;
