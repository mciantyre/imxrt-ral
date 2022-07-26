#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MX6RT_ANADIG_REGISTER
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::anadig_pmu::Instance;
pub use crate::imxrt117::peripherals::anadig_pmu::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::anadig_pmu::{
    BANDGAP_ENABLE_SP, BANDGAP_STBY_EN_SP, FBB_M7_CONFIGURE, FBB_M7_ENABLE_SP, FBB_M7_STBY_EN_SP,
    LDO_LPSR_ANA_BYPASS_EN_SP, LDO_LPSR_ANA_ENABLE_SP, LDO_LPSR_ANA_LP_MODE_SP,
    LDO_LPSR_ANA_STBY_EN_SP, LDO_LPSR_ANA_TRACKING_EN_SP, LDO_LPSR_DIG_BYPASS_EN_SP,
    LDO_LPSR_DIG_ENABLE_SP, LDO_LPSR_DIG_LP_MODE_SP, LDO_LPSR_DIG_STBY_EN_SP,
    LDO_LPSR_DIG_TRACKING_EN_SP, LDO_LPSR_DIG_TRG_SP0, LDO_LPSR_DIG_TRG_SP1, LDO_LPSR_DIG_TRG_SP2,
    LDO_LPSR_DIG_TRG_SP3, LDO_PLL_ENABLE_SP, LPSR_1P8_LDO_OTP_TRIM_VALUE, PLL_LDO_STBY_EN_SP,
    PMU_BIAS_CTRL, PMU_BIAS_CTRL2, PMU_LDO_PLL, PMU_POWER_DETECT_CTRL, PMU_REF_CTRL,
    RBB_LPSR_CONFIGURE, RBB_LPSR_ENABLE_SP, RBB_LPSR_STBY_EN_SP, RBB_SOC_CONFIGURE,
    RBB_SOC_ENABLE_SP, RBB_SOC_STBY_EN_SP, REFTOP_OTP_TRIM_VALUE,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The ANADIG_PMU peripheral instance.
#[cfg(not(feature = "doc"))]
pub type ANADIG_PMU = Instance<0>;

/// The ANADIG_PMU peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ANADIG_PMU = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct ANADIG_PMU {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for ANADIG_PMU {}
impl crate::Valid for ANADIG_PMU {
    fn take() -> Option<Self> {
        <ANADIG_PMU>::take()
    }
    fn release(self) {
        <ANADIG_PMU>::release(self);
    }
    unsafe fn steal() -> Self {
        <ANADIG_PMU>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ANADIG_PMU_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ANADIG_PMU peripheral instance
#[cfg(not(feature = "nosync"))]
impl ANADIG_PMU {
    const INSTANCE: Self = Self {
        addr: 0x40c84000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::LPSR_LP8_BROWNOUT,
            crate::interrupt::LPSR_LP0_BROWNOUT,
        ],
    };

    /// Reset values for each field in ANADIG_PMU
    pub const reset: ResetValues = ResetValues {
        PMU_LDO_PLL: 0x00000001,
        PMU_BIAS_CTRL: 0x00000000,
        PMU_BIAS_CTRL2: 0x00000000,
        PMU_REF_CTRL: 0x00000000,
        PMU_POWER_DETECT_CTRL: 0x00000000,
        LDO_PLL_ENABLE_SP: 0x00000000,
        LDO_LPSR_ANA_ENABLE_SP: 0x00000000,
        LDO_LPSR_ANA_LP_MODE_SP: 0x00000000,
        LDO_LPSR_ANA_TRACKING_EN_SP: 0x00000000,
        LDO_LPSR_ANA_BYPASS_EN_SP: 0x00000000,
        LDO_LPSR_ANA_STBY_EN_SP: 0x00000000,
        LDO_LPSR_DIG_ENABLE_SP: 0x00000000,
        LDO_LPSR_DIG_TRG_SP0: 0x00000000,
        LDO_LPSR_DIG_TRG_SP1: 0x00000000,
        LDO_LPSR_DIG_TRG_SP2: 0x00000000,
        LDO_LPSR_DIG_TRG_SP3: 0x00000000,
        LDO_LPSR_DIG_LP_MODE_SP: 0x00000000,
        LDO_LPSR_DIG_TRACKING_EN_SP: 0x00000000,
        LDO_LPSR_DIG_BYPASS_EN_SP: 0x00000000,
        LDO_LPSR_DIG_STBY_EN_SP: 0x00000000,
        BANDGAP_ENABLE_SP: 0x00000000,
        FBB_M7_ENABLE_SP: 0x00000000,
        RBB_SOC_ENABLE_SP: 0x00000000,
        RBB_LPSR_ENABLE_SP: 0x00000000,
        BANDGAP_STBY_EN_SP: 0x00000000,
        PLL_LDO_STBY_EN_SP: 0x00000000,
        FBB_M7_STBY_EN_SP: 0x00000000,
        RBB_SOC_STBY_EN_SP: 0x00000000,
        RBB_LPSR_STBY_EN_SP: 0x00000000,
        FBB_M7_CONFIGURE: 0x00002F11,
        RBB_LPSR_CONFIGURE: 0x00003022,
        RBB_SOC_CONFIGURE: 0x00000044,
        REFTOP_OTP_TRIM_VALUE: 0x00000000,
        LPSR_1P8_LDO_OTP_TRIM_VALUE: 0x00000000,
    };

    /// Safe access to ANADIG_PMU
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
        let taken = ANADIG_PMU_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ANADIG_PMU
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ANADIG_PMU_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ANADIG_PMU
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ANADIG_PMU_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl ANADIG_PMU {
    /// The interrupts associated with ANADIG_PMU
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::LPSR_LP8_BROWNOUT,
        crate::interrupt::LPSR_LP0_BROWNOUT,
    ];

    /// The interrupts associated with ANADIG_PMU
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ANADIG_PMU
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ANADIG_PMU: *const RegisterBlock = 0x40c84000 as *const _;
