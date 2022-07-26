#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CCM_OBS
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::ccm_obs::Instance;
pub use crate::imxrt117::peripherals::ccm_obs::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::ccm_obs::{
    OBSERVE_AUTHEN_0, OBSERVE_AUTHEN_1, OBSERVE_AUTHEN_2, OBSERVE_AUTHEN_3, OBSERVE_AUTHEN_4,
    OBSERVE_AUTHEN_5, OBSERVE_AUTHEN_CLR_0, OBSERVE_AUTHEN_CLR_1, OBSERVE_AUTHEN_CLR_2,
    OBSERVE_AUTHEN_CLR_3, OBSERVE_AUTHEN_CLR_4, OBSERVE_AUTHEN_CLR_5, OBSERVE_AUTHEN_SET_0,
    OBSERVE_AUTHEN_SET_1, OBSERVE_AUTHEN_SET_2, OBSERVE_AUTHEN_SET_3, OBSERVE_AUTHEN_SET_4,
    OBSERVE_AUTHEN_SET_5, OBSERVE_AUTHEN_TOG_0, OBSERVE_AUTHEN_TOG_1, OBSERVE_AUTHEN_TOG_2,
    OBSERVE_AUTHEN_TOG_3, OBSERVE_AUTHEN_TOG_4, OBSERVE_AUTHEN_TOG_5, OBSERVE_CONTROL_0,
    OBSERVE_CONTROL_1, OBSERVE_CONTROL_2, OBSERVE_CONTROL_3, OBSERVE_CONTROL_4, OBSERVE_CONTROL_5,
    OBSERVE_CONTROL_CLR_0, OBSERVE_CONTROL_CLR_1, OBSERVE_CONTROL_CLR_2, OBSERVE_CONTROL_CLR_3,
    OBSERVE_CONTROL_CLR_4, OBSERVE_CONTROL_CLR_5, OBSERVE_CONTROL_SET_0, OBSERVE_CONTROL_SET_1,
    OBSERVE_CONTROL_SET_2, OBSERVE_CONTROL_SET_3, OBSERVE_CONTROL_SET_4, OBSERVE_CONTROL_SET_5,
    OBSERVE_CONTROL_TOG_0, OBSERVE_CONTROL_TOG_1, OBSERVE_CONTROL_TOG_2, OBSERVE_CONTROL_TOG_3,
    OBSERVE_CONTROL_TOG_4, OBSERVE_CONTROL_TOG_5, OBSERVE_FREQUENCY_CURRENT_0,
    OBSERVE_FREQUENCY_CURRENT_1, OBSERVE_FREQUENCY_CURRENT_2, OBSERVE_FREQUENCY_CURRENT_3,
    OBSERVE_FREQUENCY_CURRENT_4, OBSERVE_FREQUENCY_CURRENT_5, OBSERVE_FREQUENCY_MAX_0,
    OBSERVE_FREQUENCY_MAX_1, OBSERVE_FREQUENCY_MAX_2, OBSERVE_FREQUENCY_MAX_3,
    OBSERVE_FREQUENCY_MAX_4, OBSERVE_FREQUENCY_MAX_5, OBSERVE_FREQUENCY_MIN_0,
    OBSERVE_FREQUENCY_MIN_1, OBSERVE_FREQUENCY_MIN_2, OBSERVE_FREQUENCY_MIN_3,
    OBSERVE_FREQUENCY_MIN_4, OBSERVE_FREQUENCY_MIN_5, OBSERVE_STATUS0_0, OBSERVE_STATUS0_1,
    OBSERVE_STATUS0_2, OBSERVE_STATUS0_3, OBSERVE_STATUS0_4, OBSERVE_STATUS0_5,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The CCM_OBS peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CCM_OBS = Instance<0>;

/// The CCM_OBS peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CCM_OBS = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct CCM_OBS {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for CCM_OBS {}
impl crate::Valid for CCM_OBS {
    fn take() -> Option<Self> {
        <CCM_OBS>::take()
    }
    fn release(self) {
        <CCM_OBS>::release(self);
    }
    unsafe fn steal() -> Self {
        <CCM_OBS>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CCM_OBS_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CCM_OBS peripheral instance
#[cfg(not(feature = "nosync"))]
impl CCM_OBS {
    const INSTANCE: Self = Self {
        addr: 0x40150000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in CCM_OBS
    pub const reset: ResetValues = ResetValues {
        OBSERVE_CONTROL_0: 0x00000000,
        OBSERVE_CONTROL_SET_0: 0x00000000,
        OBSERVE_CONTROL_CLR_0: 0x00000000,
        OBSERVE_CONTROL_TOG_0: 0x00000000,
        OBSERVE_STATUS0_0: 0x00000000,
        OBSERVE_AUTHEN_0: 0x00000000,
        OBSERVE_AUTHEN_SET_0: 0x00000000,
        OBSERVE_AUTHEN_CLR_0: 0x00000000,
        OBSERVE_AUTHEN_TOG_0: 0x00000000,
        OBSERVE_FREQUENCY_CURRENT_0: 0x00000000,
        OBSERVE_FREQUENCY_MIN_0: 0xFFFFFFC0,
        OBSERVE_FREQUENCY_MAX_0: 0x00000000,
        OBSERVE_CONTROL_1: 0x00000000,
        OBSERVE_CONTROL_SET_1: 0x00000000,
        OBSERVE_CONTROL_CLR_1: 0x00000000,
        OBSERVE_CONTROL_TOG_1: 0x00000000,
        OBSERVE_STATUS0_1: 0x00000000,
        OBSERVE_AUTHEN_1: 0x00000000,
        OBSERVE_AUTHEN_SET_1: 0x00000000,
        OBSERVE_AUTHEN_CLR_1: 0x00000000,
        OBSERVE_AUTHEN_TOG_1: 0x00000000,
        OBSERVE_FREQUENCY_CURRENT_1: 0x00000000,
        OBSERVE_FREQUENCY_MIN_1: 0xFFFFFFC0,
        OBSERVE_FREQUENCY_MAX_1: 0x00000000,
        OBSERVE_CONTROL_2: 0x00000000,
        OBSERVE_CONTROL_SET_2: 0x00000000,
        OBSERVE_CONTROL_CLR_2: 0x00000000,
        OBSERVE_CONTROL_TOG_2: 0x00000000,
        OBSERVE_STATUS0_2: 0x00000000,
        OBSERVE_AUTHEN_2: 0x00000000,
        OBSERVE_AUTHEN_SET_2: 0x00000000,
        OBSERVE_AUTHEN_CLR_2: 0x00000000,
        OBSERVE_AUTHEN_TOG_2: 0x00000000,
        OBSERVE_FREQUENCY_CURRENT_2: 0x00000000,
        OBSERVE_FREQUENCY_MIN_2: 0xFFFFFFC0,
        OBSERVE_FREQUENCY_MAX_2: 0x00000000,
        OBSERVE_CONTROL_3: 0x00000000,
        OBSERVE_CONTROL_SET_3: 0x00000000,
        OBSERVE_CONTROL_CLR_3: 0x00000000,
        OBSERVE_CONTROL_TOG_3: 0x00000000,
        OBSERVE_STATUS0_3: 0x00000000,
        OBSERVE_AUTHEN_3: 0x00000000,
        OBSERVE_AUTHEN_SET_3: 0x00000000,
        OBSERVE_AUTHEN_CLR_3: 0x00000000,
        OBSERVE_AUTHEN_TOG_3: 0x00000000,
        OBSERVE_FREQUENCY_CURRENT_3: 0x00000000,
        OBSERVE_FREQUENCY_MIN_3: 0xFFFFFFC0,
        OBSERVE_FREQUENCY_MAX_3: 0x00000000,
        OBSERVE_CONTROL_4: 0x00000000,
        OBSERVE_CONTROL_SET_4: 0x00000000,
        OBSERVE_CONTROL_CLR_4: 0x00000000,
        OBSERVE_CONTROL_TOG_4: 0x00000000,
        OBSERVE_STATUS0_4: 0x00000000,
        OBSERVE_AUTHEN_4: 0x00000000,
        OBSERVE_AUTHEN_SET_4: 0x00000000,
        OBSERVE_AUTHEN_CLR_4: 0x00000000,
        OBSERVE_AUTHEN_TOG_4: 0x00000000,
        OBSERVE_FREQUENCY_CURRENT_4: 0x00000000,
        OBSERVE_FREQUENCY_MIN_4: 0xFFFFFFC0,
        OBSERVE_FREQUENCY_MAX_4: 0x00000000,
        OBSERVE_CONTROL_5: 0x00000000,
        OBSERVE_CONTROL_SET_5: 0x00000000,
        OBSERVE_CONTROL_CLR_5: 0x00000000,
        OBSERVE_CONTROL_TOG_5: 0x00000000,
        OBSERVE_STATUS0_5: 0x00000000,
        OBSERVE_AUTHEN_5: 0x00000000,
        OBSERVE_AUTHEN_SET_5: 0x00000000,
        OBSERVE_AUTHEN_CLR_5: 0x00000000,
        OBSERVE_AUTHEN_TOG_5: 0x00000000,
        OBSERVE_FREQUENCY_CURRENT_5: 0x00000000,
        OBSERVE_FREQUENCY_MIN_5: 0xFFFFFFC0,
        OBSERVE_FREQUENCY_MAX_5: 0x00000000,
    };

    /// Safe access to CCM_OBS
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
        let taken = CCM_OBS_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CCM_OBS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CCM_OBS_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CCM_OBS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CCM_OBS_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CCM_OBS {
    /// The interrupts associated with CCM_OBS
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with CCM_OBS
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CCM_OBS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CCM_OBS: *const RegisterBlock = 0x40150000 as *const _;
