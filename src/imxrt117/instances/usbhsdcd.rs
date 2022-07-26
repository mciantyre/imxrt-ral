#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USBDCD
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::usbhsdcd::Instance;
pub use crate::imxrt117::peripherals::usbhsdcd::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::usbhsdcd::{
    CLOCK, CONTROL, SIGNAL_OVERRIDE, STATUS, TIMER0, TIMER1, TIMER2_BC1,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The USBHSDCD1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type USBHSDCD1 = Instance<1>;

/// The USBHSDCD1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type USBHSDCD1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct USBHSDCD1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for USBHSDCD1 {}
impl crate::Valid for USBHSDCD1 {
    fn take() -> Option<Self> {
        <USBHSDCD1>::take()
    }
    fn release(self) {
        <USBHSDCD1>::release(self);
    }
    unsafe fn steal() -> Self {
        <USBHSDCD1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USBHSDCD1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USBHSDCD1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl USBHSDCD1 {
    const INSTANCE: Self = Self {
        addr: 0x40434800,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in USBHSDCD1
    pub const reset: ResetValues = ResetValues {
        CONTROL: 0x00010000,
        CLOCK: 0x000000C1,
        STATUS: 0x00000000,
        SIGNAL_OVERRIDE: 0x00000000,
        TIMER0: 0x00100000,
        TIMER1: 0x000A0028,
        TIMER2_BC1: 0x00010028,
    };

    /// Safe access to USBHSDCD1
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
        let taken = USBHSDCD1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USBHSDCD1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USBHSDCD1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USBHSDCD1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USBHSDCD1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl USBHSDCD1 {
    /// The interrupts associated with USBHSDCD1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with USBHSDCD1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USBHSDCD1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USBHSDCD1: *const RegisterBlock = 0x40434800 as *const _;

/// The USBHSDCD2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type USBHSDCD2 = Instance<2>;

/// The USBHSDCD2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type USBHSDCD2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct USBHSDCD2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for USBHSDCD2 {}
impl crate::Valid for USBHSDCD2 {
    fn take() -> Option<Self> {
        <USBHSDCD2>::take()
    }
    fn release(self) {
        <USBHSDCD2>::release(self);
    }
    unsafe fn steal() -> Self {
        <USBHSDCD2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USBHSDCD2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USBHSDCD2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl USBHSDCD2 {
    const INSTANCE: Self = Self {
        addr: 0x40438800,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in USBHSDCD2
    pub const reset: ResetValues = ResetValues {
        CONTROL: 0x00010000,
        CLOCK: 0x000000C1,
        STATUS: 0x00000000,
        SIGNAL_OVERRIDE: 0x00000000,
        TIMER0: 0x00100000,
        TIMER1: 0x000A0028,
        TIMER2_BC1: 0x00010028,
    };

    /// Safe access to USBHSDCD2
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
        let taken = USBHSDCD2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USBHSDCD2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USBHSDCD2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USBHSDCD2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USBHSDCD2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl USBHSDCD2 {
    /// The interrupts associated with USBHSDCD2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with USBHSDCD2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USBHSDCD2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USBHSDCD2: *const RegisterBlock = 0x40438800 as *const _;
