#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FlexCAN wrapper
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::can_wrapper::Instance;
pub use crate::imxrt117::peripherals::can_wrapper::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::can_wrapper::GFWR;
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The CAN1_WRAPPER peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CAN1_WRAPPER = Instance<1>;

/// The CAN1_WRAPPER peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CAN1_WRAPPER = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct CAN1_WRAPPER {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for CAN1_WRAPPER {}
impl crate::Valid for CAN1_WRAPPER {
    fn take() -> Option<Self> {
        <CAN1_WRAPPER>::take()
    }
    fn release(self) {
        <CAN1_WRAPPER>::release(self);
    }
    unsafe fn steal() -> Self {
        <CAN1_WRAPPER>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CAN1_WRAPPER_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CAN1_WRAPPER peripheral instance
#[cfg(not(feature = "nosync"))]
impl CAN1_WRAPPER {
    const INSTANCE: Self = Self {
        addr: 0x400c4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in CAN1_WRAPPER
    pub const reset: ResetValues = ResetValues { GFWR: 0x0000007F };

    /// Safe access to CAN1_WRAPPER
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
        let taken = CAN1_WRAPPER_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CAN1_WRAPPER
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CAN1_WRAPPER_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CAN1_WRAPPER
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CAN1_WRAPPER_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CAN1_WRAPPER {
    /// The interrupts associated with CAN1_WRAPPER
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with CAN1_WRAPPER
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CAN1_WRAPPER
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN1_WRAPPER: *const RegisterBlock = 0x400c4000 as *const _;

/// The CAN2_WRAPPER peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CAN2_WRAPPER = Instance<2>;

/// The CAN2_WRAPPER peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CAN2_WRAPPER = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct CAN2_WRAPPER {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for CAN2_WRAPPER {}
impl crate::Valid for CAN2_WRAPPER {
    fn take() -> Option<Self> {
        <CAN2_WRAPPER>::take()
    }
    fn release(self) {
        <CAN2_WRAPPER>::release(self);
    }
    unsafe fn steal() -> Self {
        <CAN2_WRAPPER>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CAN2_WRAPPER_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CAN2_WRAPPER peripheral instance
#[cfg(not(feature = "nosync"))]
impl CAN2_WRAPPER {
    const INSTANCE: Self = Self {
        addr: 0x400c8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in CAN2_WRAPPER
    pub const reset: ResetValues = ResetValues { GFWR: 0x0000007F };

    /// Safe access to CAN2_WRAPPER
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
        let taken = CAN2_WRAPPER_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CAN2_WRAPPER
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CAN2_WRAPPER_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CAN2_WRAPPER
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CAN2_WRAPPER_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CAN2_WRAPPER {
    /// The interrupts associated with CAN2_WRAPPER
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with CAN2_WRAPPER
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CAN2_WRAPPER
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN2_WRAPPER: *const RegisterBlock = 0x400c8000 as *const _;

/// The CAN3_WRAPPER peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CAN3_WRAPPER = Instance<3>;

/// The CAN3_WRAPPER peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CAN3_WRAPPER = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct CAN3_WRAPPER {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for CAN3_WRAPPER {}
impl crate::Valid for CAN3_WRAPPER {
    fn take() -> Option<Self> {
        <CAN3_WRAPPER>::take()
    }
    fn release(self) {
        <CAN3_WRAPPER>::release(self);
    }
    unsafe fn steal() -> Self {
        <CAN3_WRAPPER>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CAN3_WRAPPER_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CAN3_WRAPPER peripheral instance
#[cfg(not(feature = "nosync"))]
impl CAN3_WRAPPER {
    const INSTANCE: Self = Self {
        addr: 0x40c3c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in CAN3_WRAPPER
    pub const reset: ResetValues = ResetValues { GFWR: 0x0000007F };

    /// Safe access to CAN3_WRAPPER
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
        let taken = CAN3_WRAPPER_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CAN3_WRAPPER
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CAN3_WRAPPER_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CAN3_WRAPPER
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CAN3_WRAPPER_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CAN3_WRAPPER {
    /// The interrupts associated with CAN3_WRAPPER
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with CAN3_WRAPPER
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CAN3_WRAPPER
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN3_WRAPPER: *const RegisterBlock = 0x40c3c000 as *const _;
