#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MX6RT_ANADIG_REGISTER
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::anadig_ldo_snvs::Instance;
pub use crate::imxrt117::peripherals::anadig_ldo_snvs::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::anadig_ldo_snvs::{
    PMU_LDO_LPSR_ANA, PMU_LDO_LPSR_DIG, PMU_LDO_LPSR_DIG_2,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The ANADIG_LDO_SNVS peripheral instance.
#[cfg(not(feature = "doc"))]
pub type ANADIG_LDO_SNVS = Instance<0>;

/// The ANADIG_LDO_SNVS peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ANADIG_LDO_SNVS = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct ANADIG_LDO_SNVS {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for ANADIG_LDO_SNVS {}
impl crate::Valid for ANADIG_LDO_SNVS {
    fn take() -> Option<Self> {
        <ANADIG_LDO_SNVS>::take()
    }
    fn release(self) {
        <ANADIG_LDO_SNVS>::release(self);
    }
    unsafe fn steal() -> Self {
        <ANADIG_LDO_SNVS>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ANADIG_LDO_SNVS_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ANADIG_LDO_SNVS peripheral instance
#[cfg(not(feature = "nosync"))]
impl ANADIG_LDO_SNVS {
    const INSTANCE: Self = Self {
        addr: 0x40c84000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in ANADIG_LDO_SNVS
    pub const reset: ResetValues = ResetValues {
        PMU_LDO_LPSR_ANA: 0x00000108,
        PMU_LDO_LPSR_DIG_2: 0x00000002,
        PMU_LDO_LPSR_DIG: 0x01301C05,
    };

    /// Safe access to ANADIG_LDO_SNVS
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
        let taken = ANADIG_LDO_SNVS_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ANADIG_LDO_SNVS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ANADIG_LDO_SNVS_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ANADIG_LDO_SNVS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ANADIG_LDO_SNVS_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl ANADIG_LDO_SNVS {
    /// The interrupts associated with ANADIG_LDO_SNVS
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with ANADIG_LDO_SNVS
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ANADIG_LDO_SNVS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ANADIG_LDO_SNVS: *const RegisterBlock = 0x40c84000 as *const _;
