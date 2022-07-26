#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FLEXRAM

pub use crate::imxrt117::peripherals::flexram::Instance;
pub use crate::imxrt117::peripherals::flexram::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::flexram::{
    D0TCM_ECC_MULTI_ERROR_ADDR, D0TCM_ECC_MULTI_ERROR_DATA, D0TCM_ECC_MULTI_ERROR_INFO,
    D0TCM_ECC_SINGLE_ERROR_ADDR, D0TCM_ECC_SINGLE_ERROR_DATA, D0TCM_ECC_SINGLE_ERROR_INFO,
    D1TCM_ECC_MULTI_ERROR_ADDR, D1TCM_ECC_MULTI_ERROR_DATA, D1TCM_ECC_MULTI_ERROR_INFO,
    D1TCM_ECC_SINGLE_ERROR_ADDR, D1TCM_ECC_SINGLE_ERROR_DATA, D1TCM_ECC_SINGLE_ERROR_INFO,
    DTCM_MAGIC_ADDR, FLEXRAM_CTRL, INT_SIG_EN, INT_STATUS, INT_STAT_EN, ITCM_ECC_MULTI_ERROR_ADDR,
    ITCM_ECC_MULTI_ERROR_DATA_LSB, ITCM_ECC_MULTI_ERROR_DATA_MSB, ITCM_ECC_MULTI_ERROR_INFO,
    ITCM_ECC_SINGLE_ERROR_ADDR, ITCM_ECC_SINGLE_ERROR_DATA_LSB, ITCM_ECC_SINGLE_ERROR_DATA_MSB,
    ITCM_ECC_SINGLE_ERROR_INFO, ITCM_MAGIC_ADDR, OCRAM_ECC_MULTI_ERROR_ADDR,
    OCRAM_ECC_MULTI_ERROR_DATA_LSB, OCRAM_ECC_MULTI_ERROR_DATA_MSB, OCRAM_ECC_MULTI_ERROR_INFO,
    OCRAM_ECC_SINGLE_ERROR_ADDR, OCRAM_ECC_SINGLE_ERROR_DATA_LSB, OCRAM_ECC_SINGLE_ERROR_DATA_MSB,
    OCRAM_ECC_SINGLE_ERROR_INFO, OCRAM_MAGIC_ADDR, OCRAM_PIPELINE_STATUS, TCM_CTRL,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The FLEXRAM peripheral instance.
#[cfg(not(feature = "doc"))]
pub type FLEXRAM = Instance<0>;

/// The FLEXRAM peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type FLEXRAM = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct FLEXRAM {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for FLEXRAM {}
impl crate::Valid for FLEXRAM {
    fn take() -> Option<Self> {
        <FLEXRAM>::take()
    }
    fn release(self) {
        <FLEXRAM>::release(self);
    }
    unsafe fn steal() -> Self {
        <FLEXRAM>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static FLEXRAM_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the FLEXRAM peripheral instance
#[cfg(not(feature = "nosync"))]
impl FLEXRAM {
    const INSTANCE: Self = Self {
        addr: 0x40028000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::FLEXRAM, crate::interrupt::FLEXRAM_ECC],
    };

    /// Reset values for each field in FLEXRAM
    pub const reset: ResetValues = ResetValues {
        TCM_CTRL: 0x00000000,
        OCRAM_MAGIC_ADDR: 0x00000000,
        DTCM_MAGIC_ADDR: 0x00000000,
        ITCM_MAGIC_ADDR: 0x00000000,
        INT_STATUS: 0x00000000,
        INT_STAT_EN: 0x00000000,
        INT_SIG_EN: 0x00000000,
        OCRAM_ECC_SINGLE_ERROR_INFO: 0x00000000,
        OCRAM_ECC_SINGLE_ERROR_ADDR: 0x00000000,
        OCRAM_ECC_SINGLE_ERROR_DATA_LSB: 0x00000000,
        OCRAM_ECC_SINGLE_ERROR_DATA_MSB: 0x00000000,
        OCRAM_ECC_MULTI_ERROR_INFO: 0x00000000,
        OCRAM_ECC_MULTI_ERROR_ADDR: 0x00000000,
        OCRAM_ECC_MULTI_ERROR_DATA_LSB: 0x00000000,
        OCRAM_ECC_MULTI_ERROR_DATA_MSB: 0x00000000,
        ITCM_ECC_SINGLE_ERROR_INFO: 0x00000000,
        ITCM_ECC_SINGLE_ERROR_ADDR: 0x00000000,
        ITCM_ECC_SINGLE_ERROR_DATA_LSB: 0x00000000,
        ITCM_ECC_SINGLE_ERROR_DATA_MSB: 0x00000000,
        ITCM_ECC_MULTI_ERROR_INFO: 0x00000000,
        ITCM_ECC_MULTI_ERROR_ADDR: 0x00000000,
        ITCM_ECC_MULTI_ERROR_DATA_LSB: 0x00000000,
        ITCM_ECC_MULTI_ERROR_DATA_MSB: 0x00000000,
        D0TCM_ECC_SINGLE_ERROR_INFO: 0x00000000,
        D0TCM_ECC_SINGLE_ERROR_ADDR: 0x00000000,
        D0TCM_ECC_SINGLE_ERROR_DATA: 0x00000000,
        D0TCM_ECC_MULTI_ERROR_INFO: 0x00000000,
        D0TCM_ECC_MULTI_ERROR_ADDR: 0x00000000,
        D0TCM_ECC_MULTI_ERROR_DATA: 0x00000000,
        D1TCM_ECC_SINGLE_ERROR_INFO: 0x00000000,
        D1TCM_ECC_SINGLE_ERROR_ADDR: 0x00000000,
        D1TCM_ECC_SINGLE_ERROR_DATA: 0x00000000,
        D1TCM_ECC_MULTI_ERROR_INFO: 0x00000000,
        D1TCM_ECC_MULTI_ERROR_ADDR: 0x00000000,
        D1TCM_ECC_MULTI_ERROR_DATA: 0x00000000,
        FLEXRAM_CTRL: 0x00000000,
        OCRAM_PIPELINE_STATUS: 0x00000000,
    };

    /// Safe access to FLEXRAM
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
        let taken = FLEXRAM_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to FLEXRAM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = FLEXRAM_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal FLEXRAM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        FLEXRAM_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl FLEXRAM {
    /// The interrupts associated with FLEXRAM
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] =
        [crate::interrupt::FLEXRAM, crate::interrupt::FLEXRAM_ECC];

    /// The interrupts associated with FLEXRAM
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to FLEXRAM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FLEXRAM: *const RegisterBlock = 0x40028000 as *const _;
