#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IPS_Semaphores
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::sema4::Instance;
pub use crate::imxrt117::peripherals::sema4::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::sema4::{
    Gate0, Gate1, Gate10, Gate11, Gate12, Gate13, Gate14, Gate15, Gate2, Gate3, Gate4, Gate5,
    Gate6, Gate7, Gate8, Gate9, CP0INE, CP0NTF, CP1INE, CP1NTF, RSTGT, RSTNTF,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The SEMA4 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type SEMA4 = Instance<0>;

/// The SEMA4 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type SEMA4 = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct SEMA4 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for SEMA4 {}
impl crate::Valid for SEMA4 {
    fn take() -> Option<Self> {
        <SEMA4>::take()
    }
    fn release(self) {
        <SEMA4>::release(self);
    }
    unsafe fn steal() -> Self {
        <SEMA4>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static SEMA4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the SEMA4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl SEMA4 {
    const INSTANCE: Self = Self {
        addr: 0x40cc8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::SEMA4_CP0, crate::interrupt::SEMA4_CP1],
    };

    /// Reset values for each field in SEMA4
    pub const reset: ResetValues = ResetValues {
        Gate0: 0x00000000,
        Gate1: 0x00000000,
        Gate2: 0x00000000,
        Gate3: 0x00000000,
        Gate4: 0x00000000,
        Gate5: 0x00000000,
        Gate6: 0x00000000,
        Gate7: 0x00000000,
        Gate8: 0x00000000,
        Gate9: 0x00000000,
        Gate10: 0x00000000,
        Gate11: 0x00000000,
        Gate12: 0x00000000,
        Gate13: 0x00000000,
        Gate14: 0x00000000,
        Gate15: 0x00000000,
        CP0INE: 0x00000000,
        CP1INE: 0x00000000,
        CP0NTF: 0x00000000,
        CP1NTF: 0x00000000,
        RSTGT: 0x00000000,
        RSTNTF: 0x00000000,
    };

    /// Safe access to SEMA4
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
        let taken = SEMA4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to SEMA4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = SEMA4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal SEMA4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        SEMA4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl SEMA4 {
    /// The interrupts associated with SEMA4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] =
        [crate::interrupt::SEMA4_CP0, crate::interrupt::SEMA4_CP1];

    /// The interrupts associated with SEMA4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to SEMA4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEMA4: *const RegisterBlock = 0x40cc8000 as *const _;
