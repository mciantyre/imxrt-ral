#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! WDOG

pub use crate::imxrt117::peripherals::rtwdog::Instance;
pub use crate::imxrt117::peripherals::rtwdog::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::rtwdog::{CNT, CS, TOVAL, WIN};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The RTWDOG3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type RTWDOG3 = Instance<3>;

/// The RTWDOG3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type RTWDOG3 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct RTWDOG3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for RTWDOG3 {}
impl crate::Valid for RTWDOG3 {
    fn take() -> Option<Self> {
        <RTWDOG3>::take()
    }
    fn release(self) {
        <RTWDOG3>::release(self);
    }
    unsafe fn steal() -> Self {
        <RTWDOG3>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static RTWDOG3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the RTWDOG3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl RTWDOG3 {
    const INSTANCE: Self = Self {
        addr: 0x40038000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in RTWDOG3
    pub const reset: ResetValues = ResetValues {
        CS: 0x00002180,
        CNT: 0x00000000,
        TOVAL: 0x00007D00,
        WIN: 0x00000000,
    };

    /// Safe access to RTWDOG3
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
        let taken = RTWDOG3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to RTWDOG3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = RTWDOG3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal RTWDOG3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        RTWDOG3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl RTWDOG3 {
    /// The interrupts associated with RTWDOG3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with RTWDOG3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to RTWDOG3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RTWDOG3: *const RegisterBlock = 0x40038000 as *const _;

/// The RTWDOG4 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type RTWDOG4 = Instance<4>;

/// The RTWDOG4 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type RTWDOG4 = Instance<4>;
/// ```
#[cfg(feature = "doc")]
pub struct RTWDOG4 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for RTWDOG4 {}
impl crate::Valid for RTWDOG4 {
    fn take() -> Option<Self> {
        <RTWDOG4>::take()
    }
    fn release(self) {
        <RTWDOG4>::release(self);
    }
    unsafe fn steal() -> Self {
        <RTWDOG4>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static RTWDOG4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the RTWDOG4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl RTWDOG4 {
    const INSTANCE: Self = Self {
        addr: 0x40c10000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::RTWDOG4],
    };

    /// Reset values for each field in RTWDOG4
    pub const reset: ResetValues = ResetValues {
        CS: 0x00002180,
        CNT: 0x00000000,
        TOVAL: 0x00007D00,
        WIN: 0x00000000,
    };

    /// Safe access to RTWDOG4
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
        let taken = RTWDOG4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to RTWDOG4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = RTWDOG4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal RTWDOG4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        RTWDOG4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl RTWDOG4 {
    /// The interrupts associated with RTWDOG4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::RTWDOG4];

    /// The interrupts associated with RTWDOG4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to RTWDOG4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RTWDOG4: *const RegisterBlock = 0x40c10000 as *const _;
