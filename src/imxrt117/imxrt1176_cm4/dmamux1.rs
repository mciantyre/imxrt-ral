#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX

pub use crate::imxrt117::peripherals::dmamux1::Instance;
pub use crate::imxrt117::peripherals::dmamux1::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::dmamux1::{
    CHCFG0, CHCFG1, CHCFG10, CHCFG11, CHCFG12, CHCFG13, CHCFG14, CHCFG15, CHCFG16, CHCFG17,
    CHCFG18, CHCFG19, CHCFG2, CHCFG20, CHCFG21, CHCFG22, CHCFG23, CHCFG24, CHCFG25, CHCFG26,
    CHCFG27, CHCFG28, CHCFG29, CHCFG3, CHCFG30, CHCFG31, CHCFG4, CHCFG5, CHCFG6, CHCFG7, CHCFG8,
    CHCFG9,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The DMAMUX1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type DMAMUX1 = Instance<0>;

/// The DMAMUX1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type DMAMUX1 = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct DMAMUX1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for DMAMUX1 {}
impl crate::Valid for DMAMUX1 {
    fn take() -> Option<Self> {
        <DMAMUX1>::take()
    }
    fn release(self) {
        <DMAMUX1>::release(self);
    }
    unsafe fn steal() -> Self {
        <DMAMUX1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static DMAMUX1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the DMAMUX1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl DMAMUX1 {
    const INSTANCE: Self = Self {
        addr: 0x40c18000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in DMAMUX1
    pub const reset: ResetValues = ResetValues {
        CHCFG0: 0x00000000,
        CHCFG1: 0x00000000,
        CHCFG2: 0x00000000,
        CHCFG3: 0x00000000,
        CHCFG4: 0x00000000,
        CHCFG5: 0x00000000,
        CHCFG6: 0x00000000,
        CHCFG7: 0x00000000,
        CHCFG8: 0x00000000,
        CHCFG9: 0x00000000,
        CHCFG10: 0x00000000,
        CHCFG11: 0x00000000,
        CHCFG12: 0x00000000,
        CHCFG13: 0x00000000,
        CHCFG14: 0x00000000,
        CHCFG15: 0x00000000,
        CHCFG16: 0x00000000,
        CHCFG17: 0x00000000,
        CHCFG18: 0x00000000,
        CHCFG19: 0x00000000,
        CHCFG20: 0x00000000,
        CHCFG21: 0x00000000,
        CHCFG22: 0x00000000,
        CHCFG23: 0x00000000,
        CHCFG24: 0x00000000,
        CHCFG25: 0x00000000,
        CHCFG26: 0x00000000,
        CHCFG27: 0x00000000,
        CHCFG28: 0x00000000,
        CHCFG29: 0x00000000,
        CHCFG30: 0x00000000,
        CHCFG31: 0x00000000,
    };

    /// Safe access to DMAMUX1
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
        let taken = DMAMUX1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to DMAMUX1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = DMAMUX1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal DMAMUX1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        DMAMUX1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl DMAMUX1 {
    /// The interrupts associated with DMAMUX1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with DMAMUX1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to DMAMUX1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMAMUX1: *const RegisterBlock = 0x40c18000 as *const _;
