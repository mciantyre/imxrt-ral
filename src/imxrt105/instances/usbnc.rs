#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB
//!
//! Used by: imxrt1051, imxrt1052

pub use crate::imxrt105::peripherals::usbnc::Instance;
pub use crate::imxrt105::peripherals::usbnc::{RegisterBlock, ResetValues};

pub use crate::imxrt105::peripherals::usbnc::{USB_OTG1_CTRL, USB_OTG1_PHY_CTRL_0};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The USBNC1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type USBNC1 = Instance<1>;

/// The USBNC1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type USBNC1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct USBNC1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for USBNC1 {}
impl crate::Valid for USBNC1 {
    fn take() -> Option<Self> {
        <USBNC1>::take()
    }
    fn release(self) {
        <USBNC1>::release(self);
    }
    unsafe fn steal() -> Self {
        <USBNC1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USBNC1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USBNC1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl USBNC1 {
    const INSTANCE: Self = Self {
        addr: 0x402e0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in USBNC1
    pub const reset: ResetValues = ResetValues {
        USB_OTG1_CTRL: 0x30001000,
        USB_OTG1_PHY_CTRL_0: 0x00000000,
    };

    /// Safe access to USBNC1
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
        let taken = USBNC1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USBNC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USBNC1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USBNC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USBNC1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl USBNC1 {
    /// The interrupts associated with USBNC1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with USBNC1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USBNC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USBNC1: *const RegisterBlock = 0x402e0000 as *const _;

/// The USBNC2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type USBNC2 = Instance<2>;

/// The USBNC2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type USBNC2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct USBNC2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for USBNC2 {}
impl crate::Valid for USBNC2 {
    fn take() -> Option<Self> {
        <USBNC2>::take()
    }
    fn release(self) {
        <USBNC2>::release(self);
    }
    unsafe fn steal() -> Self {
        <USBNC2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USBNC2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USBNC2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl USBNC2 {
    const INSTANCE: Self = Self {
        addr: 0x402e0004,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in USBNC2
    pub const reset: ResetValues = ResetValues {
        USB_OTG1_CTRL: 0x30001000,
        USB_OTG1_PHY_CTRL_0: 0x00000000,
    };

    /// Safe access to USBNC2
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
        let taken = USBNC2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USBNC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USBNC2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USBNC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USBNC2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl USBNC2 {
    /// The interrupts associated with USBNC2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with USBNC2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USBNC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USBNC2: *const RegisterBlock = 0x402e0004 as *const _;
