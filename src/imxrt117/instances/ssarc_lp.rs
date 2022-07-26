#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SSARC Registers
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::ssarc_lp::Instance;
pub use crate::imxrt117::peripherals::ssarc_lp::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::ssarc_lp::{
    CTRL, DESC_ADDR_DOWN_0, DESC_ADDR_DOWN_1, DESC_ADDR_DOWN_10, DESC_ADDR_DOWN_11,
    DESC_ADDR_DOWN_12, DESC_ADDR_DOWN_13, DESC_ADDR_DOWN_14, DESC_ADDR_DOWN_15, DESC_ADDR_DOWN_2,
    DESC_ADDR_DOWN_3, DESC_ADDR_DOWN_4, DESC_ADDR_DOWN_5, DESC_ADDR_DOWN_6, DESC_ADDR_DOWN_7,
    DESC_ADDR_DOWN_8, DESC_ADDR_DOWN_9, DESC_ADDR_UP_0, DESC_ADDR_UP_1, DESC_ADDR_UP_10,
    DESC_ADDR_UP_11, DESC_ADDR_UP_12, DESC_ADDR_UP_13, DESC_ADDR_UP_14, DESC_ADDR_UP_15,
    DESC_ADDR_UP_2, DESC_ADDR_UP_3, DESC_ADDR_UP_4, DESC_ADDR_UP_5, DESC_ADDR_UP_6, DESC_ADDR_UP_7,
    DESC_ADDR_UP_8, DESC_ADDR_UP_9, DESC_CTRL0_0, DESC_CTRL0_1, DESC_CTRL0_10, DESC_CTRL0_11,
    DESC_CTRL0_12, DESC_CTRL0_13, DESC_CTRL0_14, DESC_CTRL0_15, DESC_CTRL0_2, DESC_CTRL0_3,
    DESC_CTRL0_4, DESC_CTRL0_5, DESC_CTRL0_6, DESC_CTRL0_7, DESC_CTRL0_8, DESC_CTRL0_9,
    DESC_CTRL1_0, DESC_CTRL1_1, DESC_CTRL1_10, DESC_CTRL1_11, DESC_CTRL1_12, DESC_CTRL1_13,
    DESC_CTRL1_14, DESC_CTRL1_15, DESC_CTRL1_2, DESC_CTRL1_3, DESC_CTRL1_4, DESC_CTRL1_5,
    DESC_CTRL1_6, DESC_CTRL1_7, DESC_CTRL1_8, DESC_CTRL1_9, HP_TIMEOUT, HW_GROUP_PENDING,
    INT_STATUS, SW_GROUP_PENDING,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The SSARC_LP peripheral instance.
#[cfg(not(feature = "doc"))]
pub type SSARC_LP = Instance<0>;

/// The SSARC_LP peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type SSARC_LP = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct SSARC_LP {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for SSARC_LP {}
impl crate::Valid for SSARC_LP {
    fn take() -> Option<Self> {
        <SSARC_LP>::take()
    }
    fn release(self) {
        <SSARC_LP>::release(self);
    }
    unsafe fn steal() -> Self {
        <SSARC_LP>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static SSARC_LP_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the SSARC_LP peripheral instance
#[cfg(not(feature = "nosync"))]
impl SSARC_LP {
    const INSTANCE: Self = Self {
        addr: 0x40cb8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in SSARC_LP
    pub const reset: ResetValues = ResetValues {
        DESC_CTRL0_0: 0x00000000,
        DESC_CTRL1_0: 0x00000000,
        DESC_ADDR_UP_0: 0x00000000,
        DESC_ADDR_DOWN_0: 0x00000000,
        DESC_CTRL0_1: 0x00000000,
        DESC_CTRL1_1: 0x00000000,
        DESC_ADDR_UP_1: 0x00000000,
        DESC_ADDR_DOWN_1: 0x00000000,
        DESC_CTRL0_2: 0x00000000,
        DESC_CTRL1_2: 0x00000000,
        DESC_ADDR_UP_2: 0x00000000,
        DESC_ADDR_DOWN_2: 0x00000000,
        DESC_CTRL0_3: 0x00000000,
        DESC_CTRL1_3: 0x00000000,
        DESC_ADDR_UP_3: 0x00000000,
        DESC_ADDR_DOWN_3: 0x00000000,
        DESC_CTRL0_4: 0x00000000,
        DESC_CTRL1_4: 0x00000000,
        DESC_ADDR_UP_4: 0x00000000,
        DESC_ADDR_DOWN_4: 0x00000000,
        DESC_CTRL0_5: 0x00000000,
        DESC_CTRL1_5: 0x00000000,
        DESC_ADDR_UP_5: 0x00000000,
        DESC_ADDR_DOWN_5: 0x00000000,
        DESC_CTRL0_6: 0x00000000,
        DESC_CTRL1_6: 0x00000000,
        DESC_ADDR_UP_6: 0x00000000,
        DESC_ADDR_DOWN_6: 0x00000000,
        DESC_CTRL0_7: 0x00000000,
        DESC_CTRL1_7: 0x00000000,
        DESC_ADDR_UP_7: 0x00000000,
        DESC_ADDR_DOWN_7: 0x00000000,
        DESC_CTRL0_8: 0x00000000,
        DESC_CTRL1_8: 0x00000000,
        DESC_ADDR_UP_8: 0x00000000,
        DESC_ADDR_DOWN_8: 0x00000000,
        DESC_CTRL0_9: 0x00000000,
        DESC_CTRL1_9: 0x00000000,
        DESC_ADDR_UP_9: 0x00000000,
        DESC_ADDR_DOWN_9: 0x00000000,
        DESC_CTRL0_10: 0x00000000,
        DESC_CTRL1_10: 0x00000000,
        DESC_ADDR_UP_10: 0x00000000,
        DESC_ADDR_DOWN_10: 0x00000000,
        DESC_CTRL0_11: 0x00000000,
        DESC_CTRL1_11: 0x00000000,
        DESC_ADDR_UP_11: 0x00000000,
        DESC_ADDR_DOWN_11: 0x00000000,
        DESC_CTRL0_12: 0x00000000,
        DESC_CTRL1_12: 0x00000000,
        DESC_ADDR_UP_12: 0x00000000,
        DESC_ADDR_DOWN_12: 0x00000000,
        DESC_CTRL0_13: 0x00000000,
        DESC_CTRL1_13: 0x00000000,
        DESC_ADDR_UP_13: 0x00000000,
        DESC_ADDR_DOWN_13: 0x00000000,
        DESC_CTRL0_14: 0x00000000,
        DESC_CTRL1_14: 0x00000000,
        DESC_ADDR_UP_14: 0x00000000,
        DESC_ADDR_DOWN_14: 0x00000000,
        DESC_CTRL0_15: 0x00000000,
        DESC_CTRL1_15: 0x00000000,
        DESC_ADDR_UP_15: 0x00000000,
        DESC_ADDR_DOWN_15: 0x00000000,
        CTRL: 0x00000000,
        INT_STATUS: 0x00000000,
        HP_TIMEOUT: 0x00000000,
        HW_GROUP_PENDING: 0x00000000,
        SW_GROUP_PENDING: 0x00000000,
    };

    /// Safe access to SSARC_LP
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
        let taken = SSARC_LP_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to SSARC_LP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = SSARC_LP_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal SSARC_LP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        SSARC_LP_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl SSARC_LP {
    /// The interrupts associated with SSARC_LP
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with SSARC_LP
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to SSARC_LP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SSARC_LP: *const RegisterBlock = 0x40cb8000 as *const _;
