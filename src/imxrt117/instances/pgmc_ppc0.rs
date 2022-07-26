#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PGMC_PPC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::pgmc_ppc0::Instance;
pub use crate::imxrt117::peripherals::pgmc_ppc0::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::pgmc_ppc0::{
    PPC_AUTHEN_CTRL, PPC_MODE, PPC_STBY_CM_CTRL, PPC_STBY_SP_CTRL,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The PGMC_PPC0 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_PPC0 = Instance<0>;

/// The PGMC_PPC0 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_PPC0 = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_PPC0 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_PPC0 {}
impl crate::Valid for PGMC_PPC0 {
    fn take() -> Option<Self> {
        <PGMC_PPC0>::take()
    }
    fn release(self) {
        <PGMC_PPC0>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_PPC0>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_PPC0_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_PPC0 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_PPC0 {
    const INSTANCE: Self = Self {
        addr: 0x40c8b000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_PPC0
    pub const reset: ResetValues = ResetValues {
        PPC_AUTHEN_CTRL: 0x00000F00,
        PPC_MODE: 0x00000000,
        PPC_STBY_CM_CTRL: 0x00000000,
        PPC_STBY_SP_CTRL: 0x00000000,
    };

    /// Safe access to PGMC_PPC0
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
        let taken = PGMC_PPC0_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_PPC0
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_PPC0_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_PPC0
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_PPC0_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_PPC0 {
    /// The interrupts associated with PGMC_PPC0
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_PPC0
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_PPC0
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_PPC0: *const RegisterBlock = 0x40c8b000 as *const _;
