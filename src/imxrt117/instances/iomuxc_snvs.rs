#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC SNVS
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::iomuxc_snvs::Instance;
pub use crate::imxrt117::peripherals::iomuxc_snvs::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::iomuxc_snvs::{
    SW_MUX_CTL_PAD_GPIO_SNVS_00_DIG, SW_MUX_CTL_PAD_GPIO_SNVS_01_DIG,
    SW_MUX_CTL_PAD_GPIO_SNVS_02_DIG, SW_MUX_CTL_PAD_GPIO_SNVS_03_DIG,
    SW_MUX_CTL_PAD_GPIO_SNVS_04_DIG, SW_MUX_CTL_PAD_GPIO_SNVS_05_DIG,
    SW_MUX_CTL_PAD_GPIO_SNVS_06_DIG, SW_MUX_CTL_PAD_GPIO_SNVS_07_DIG,
    SW_MUX_CTL_PAD_GPIO_SNVS_08_DIG, SW_MUX_CTL_PAD_GPIO_SNVS_09_DIG,
    SW_MUX_CTL_PAD_PMIC_ON_REQ_DIG, SW_MUX_CTL_PAD_PMIC_STBY_REQ_DIG, SW_MUX_CTL_PAD_WAKEUP_DIG,
    SW_PAD_CTL_PAD_GPIO_SNVS_00_DIG, SW_PAD_CTL_PAD_GPIO_SNVS_01_DIG,
    SW_PAD_CTL_PAD_GPIO_SNVS_02_DIG, SW_PAD_CTL_PAD_GPIO_SNVS_03_DIG,
    SW_PAD_CTL_PAD_GPIO_SNVS_04_DIG, SW_PAD_CTL_PAD_GPIO_SNVS_05_DIG,
    SW_PAD_CTL_PAD_GPIO_SNVS_06_DIG, SW_PAD_CTL_PAD_GPIO_SNVS_07_DIG,
    SW_PAD_CTL_PAD_GPIO_SNVS_08_DIG, SW_PAD_CTL_PAD_GPIO_SNVS_09_DIG, SW_PAD_CTL_PAD_ONOFF_DIG,
    SW_PAD_CTL_PAD_PMIC_ON_REQ_DIG, SW_PAD_CTL_PAD_PMIC_STBY_REQ_DIG, SW_PAD_CTL_PAD_POR_B_DIG,
    SW_PAD_CTL_PAD_TEST_MODE_DIG, SW_PAD_CTL_PAD_WAKEUP_DIG,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The IOMUXC_SNVS peripheral instance.
#[cfg(not(feature = "doc"))]
pub type IOMUXC_SNVS = Instance<0>;

/// The IOMUXC_SNVS peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type IOMUXC_SNVS = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct IOMUXC_SNVS {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for IOMUXC_SNVS {}
impl crate::Valid for IOMUXC_SNVS {
    fn take() -> Option<Self> {
        <IOMUXC_SNVS>::take()
    }
    fn release(self) {
        <IOMUXC_SNVS>::release(self);
    }
    unsafe fn steal() -> Self {
        <IOMUXC_SNVS>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static IOMUXC_SNVS_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the IOMUXC_SNVS peripheral instance
#[cfg(not(feature = "nosync"))]
impl IOMUXC_SNVS {
    const INSTANCE: Self = Self {
        addr: 0x40c94000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in IOMUXC_SNVS
    pub const reset: ResetValues = ResetValues {
        SW_MUX_CTL_PAD_WAKEUP_DIG: 0x00000005,
        SW_MUX_CTL_PAD_PMIC_ON_REQ_DIG: 0x00000000,
        SW_MUX_CTL_PAD_PMIC_STBY_REQ_DIG: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_SNVS_00_DIG: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SNVS_01_DIG: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SNVS_02_DIG: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SNVS_03_DIG: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SNVS_04_DIG: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SNVS_05_DIG: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SNVS_06_DIG: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SNVS_07_DIG: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SNVS_08_DIG: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SNVS_09_DIG: 0x00000005,
        SW_PAD_CTL_PAD_TEST_MODE_DIG: 0x00000006,
        SW_PAD_CTL_PAD_POR_B_DIG: 0x0000000E,
        SW_PAD_CTL_PAD_ONOFF_DIG: 0x0000000E,
        SW_PAD_CTL_PAD_WAKEUP_DIG: 0x0000000E,
        SW_PAD_CTL_PAD_PMIC_ON_REQ_DIG: 0x0000000A,
        SW_PAD_CTL_PAD_PMIC_STBY_REQ_DIG: 0x0000000A,
        SW_PAD_CTL_PAD_GPIO_SNVS_00_DIG: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_SNVS_01_DIG: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_SNVS_02_DIG: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_SNVS_03_DIG: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_SNVS_04_DIG: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_SNVS_05_DIG: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_SNVS_06_DIG: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_SNVS_07_DIG: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_SNVS_08_DIG: 0x00000002,
        SW_PAD_CTL_PAD_GPIO_SNVS_09_DIG: 0x00000002,
    };

    /// Safe access to IOMUXC_SNVS
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
        let taken = IOMUXC_SNVS_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to IOMUXC_SNVS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = IOMUXC_SNVS_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal IOMUXC_SNVS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        IOMUXC_SNVS_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl IOMUXC_SNVS {
    /// The interrupts associated with IOMUXC_SNVS
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with IOMUXC_SNVS
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to IOMUXC_SNVS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IOMUXC_SNVS: *const RegisterBlock = 0x40c94000 as *const _;
