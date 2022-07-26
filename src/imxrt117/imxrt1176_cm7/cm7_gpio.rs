#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIO

pub use crate::imxrt117::peripherals::cm7_gpio::Instance;
pub use crate::imxrt117::peripherals::cm7_gpio::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::cm7_gpio::{
    DR, DR_CLEAR, DR_SET, DR_TOGGLE, EDGE_SEL, GDIR, ICR1, ICR2, IMR, ISR, PSR,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The CM7_GPIO2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CM7_GPIO2 = Instance<2>;

/// The CM7_GPIO2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CM7_GPIO2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct CM7_GPIO2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for CM7_GPIO2 {}
impl crate::Valid for CM7_GPIO2 {
    fn take() -> Option<Self> {
        <CM7_GPIO2>::take()
    }
    fn release(self) {
        <CM7_GPIO2>::release(self);
    }
    unsafe fn steal() -> Self {
        <CM7_GPIO2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CM7_GPIO2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CM7_GPIO2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl CM7_GPIO2 {
    const INSTANCE: Self = Self {
        addr: 0x42008000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::CM7_GPIO2_3],
    };

    /// Reset values for each field in CM7_GPIO2
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to CM7_GPIO2
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
        let taken = CM7_GPIO2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CM7_GPIO2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CM7_GPIO2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CM7_GPIO2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CM7_GPIO2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CM7_GPIO2 {
    /// The interrupts associated with CM7_GPIO2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::CM7_GPIO2_3];

    /// The interrupts associated with CM7_GPIO2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CM7_GPIO2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CM7_GPIO2: *const RegisterBlock = 0x42008000 as *const _;

/// The CM7_GPIO3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CM7_GPIO3 = Instance<3>;

/// The CM7_GPIO3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CM7_GPIO3 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct CM7_GPIO3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for CM7_GPIO3 {}
impl crate::Valid for CM7_GPIO3 {
    fn take() -> Option<Self> {
        <CM7_GPIO3>::take()
    }
    fn release(self) {
        <CM7_GPIO3>::release(self);
    }
    unsafe fn steal() -> Self {
        <CM7_GPIO3>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CM7_GPIO3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CM7_GPIO3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl CM7_GPIO3 {
    const INSTANCE: Self = Self {
        addr: 0x4200c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::CM7_GPIO2_3],
    };

    /// Reset values for each field in CM7_GPIO3
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to CM7_GPIO3
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
        let taken = CM7_GPIO3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CM7_GPIO3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CM7_GPIO3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CM7_GPIO3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CM7_GPIO3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CM7_GPIO3 {
    /// The interrupts associated with CM7_GPIO3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::CM7_GPIO2_3];

    /// The interrupts associated with CM7_GPIO3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CM7_GPIO3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CM7_GPIO3: *const RegisterBlock = 0x4200c000 as *const _;
