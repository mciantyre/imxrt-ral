#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC SNVS GPR
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::iomuxc_snvs_gpr::Instance;
pub use crate::imxrt117::peripherals::iomuxc_snvs_gpr::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::iomuxc_snvs_gpr::{
    GPR0, GPR1, GPR10, GPR11, GPR12, GPR13, GPR14, GPR15, GPR16, GPR17, GPR18, GPR19, GPR2, GPR20,
    GPR21, GPR22, GPR23, GPR24, GPR25, GPR26, GPR27, GPR28, GPR29, GPR3, GPR30, GPR31, GPR32,
    GPR33, GPR34, GPR35, GPR36, GPR37, GPR4, GPR5, GPR6, GPR7, GPR8, GPR9,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The IOMUXC_SNVS_GPR peripheral instance.
#[cfg(not(feature = "doc"))]
pub type IOMUXC_SNVS_GPR = Instance<0>;

/// The IOMUXC_SNVS_GPR peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type IOMUXC_SNVS_GPR = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct IOMUXC_SNVS_GPR {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for IOMUXC_SNVS_GPR {}
impl crate::Valid for IOMUXC_SNVS_GPR {
    fn take() -> Option<Self> {
        <IOMUXC_SNVS_GPR>::take()
    }
    fn release(self) {
        <IOMUXC_SNVS_GPR>::release(self);
    }
    unsafe fn steal() -> Self {
        <IOMUXC_SNVS_GPR>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static IOMUXC_SNVS_GPR_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the IOMUXC_SNVS_GPR peripheral instance
#[cfg(not(feature = "nosync"))]
impl IOMUXC_SNVS_GPR {
    const INSTANCE: Self = Self {
        addr: 0x40c98000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in IOMUXC_SNVS_GPR
    pub const reset: ResetValues = ResetValues {
        GPR0: 0x00000000,
        GPR1: 0x00000000,
        GPR2: 0x00000000,
        GPR3: 0x00000000,
        GPR4: 0x00000000,
        GPR5: 0x00000000,
        GPR6: 0x00000000,
        GPR7: 0x00000000,
        GPR8: 0x00000000,
        GPR9: 0x00000000,
        GPR10: 0x00000000,
        GPR11: 0x00000000,
        GPR12: 0x00000000,
        GPR13: 0x00000000,
        GPR14: 0x00000000,
        GPR15: 0x00000000,
        GPR16: 0x00000000,
        GPR17: 0x00000000,
        GPR18: 0x00000000,
        GPR19: 0x00000000,
        GPR20: 0x00000000,
        GPR21: 0x00000000,
        GPR22: 0x00000000,
        GPR23: 0x00000000,
        GPR24: 0x00000000,
        GPR25: 0x00000000,
        GPR26: 0x00000000,
        GPR27: 0x00000000,
        GPR28: 0x00000000,
        GPR29: 0x00000000,
        GPR30: 0x00000000,
        GPR31: 0x00000000,
        GPR32: 0x00000000,
        GPR33: 0x00000000,
        GPR34: 0x00000000,
        GPR35: 0x00000000,
        GPR36: 0x04800000,
        GPR37: 0x00000000,
    };

    /// Safe access to IOMUXC_SNVS_GPR
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
        let taken = IOMUXC_SNVS_GPR_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to IOMUXC_SNVS_GPR
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = IOMUXC_SNVS_GPR_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal IOMUXC_SNVS_GPR
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        IOMUXC_SNVS_GPR_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl IOMUXC_SNVS_GPR {
    /// The interrupts associated with IOMUXC_SNVS_GPR
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with IOMUXC_SNVS_GPR
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to IOMUXC_SNVS_GPR
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IOMUXC_SNVS_GPR: *const RegisterBlock = 0x40c98000 as *const _;
