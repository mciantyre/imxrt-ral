#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USBNC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::usbnc_otg::Instance;
pub use crate::imxrt117::peripherals::usbnc_otg::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::usbnc_otg::{CTRL1, CTRL2, HSIC_CTRL};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The USBNC_OTG1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type USBNC_OTG1 = Instance<1>;

/// The USBNC_OTG1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type USBNC_OTG1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct USBNC_OTG1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for USBNC_OTG1 {}
impl crate::Valid for USBNC_OTG1 {
    fn take() -> Option<Self> {
        <USBNC_OTG1>::take()
    }
    fn release(self) {
        <USBNC_OTG1>::release(self);
    }
    unsafe fn steal() -> Self {
        <USBNC_OTG1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USBNC_OTG1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USBNC_OTG1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl USBNC_OTG1 {
    const INSTANCE: Self = Self {
        addr: 0x40430200,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in USBNC_OTG1
    pub const reset: ResetValues = ResetValues {
        CTRL1: 0x30001000,
        CTRL2: 0x5F000000,
        HSIC_CTRL: 0x10004084,
    };

    /// Safe access to USBNC_OTG1
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
        let taken = USBNC_OTG1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USBNC_OTG1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USBNC_OTG1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USBNC_OTG1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USBNC_OTG1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl USBNC_OTG1 {
    /// The interrupts associated with USBNC_OTG1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with USBNC_OTG1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USBNC_OTG1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USBNC_OTG1: *const RegisterBlock = 0x40430200 as *const _;

/// The USBNC_OTG2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type USBNC_OTG2 = Instance<2>;

/// The USBNC_OTG2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type USBNC_OTG2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct USBNC_OTG2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for USBNC_OTG2 {}
impl crate::Valid for USBNC_OTG2 {
    fn take() -> Option<Self> {
        <USBNC_OTG2>::take()
    }
    fn release(self) {
        <USBNC_OTG2>::release(self);
    }
    unsafe fn steal() -> Self {
        <USBNC_OTG2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USBNC_OTG2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USBNC_OTG2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl USBNC_OTG2 {
    const INSTANCE: Self = Self {
        addr: 0x4042c200,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in USBNC_OTG2
    pub const reset: ResetValues = ResetValues {
        CTRL1: 0x30001000,
        CTRL2: 0x5F000000,
        HSIC_CTRL: 0x10004084,
    };

    /// Safe access to USBNC_OTG2
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
        let taken = USBNC_OTG2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USBNC_OTG2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USBNC_OTG2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USBNC_OTG2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USBNC_OTG2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl USBNC_OTG2 {
    /// The interrupts associated with USBNC_OTG2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with USBNC_OTG2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USBNC_OTG2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USBNC_OTG2: *const RegisterBlock = 0x4042c200 as *const _;
