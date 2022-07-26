#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! no description available
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::osc_rc_400m::Instance;
pub use crate::imxrt117::peripherals::osc_rc_400m::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::osc_rc_400m::{
    CTRL0, CTRL0_CLR, CTRL0_SET, CTRL0_TOG, CTRL1, CTRL1_CLR, CTRL1_SET, CTRL1_TOG, CTRL2,
    CTRL2_CLR, CTRL2_SET, CTRL2_TOG, CTRL3, CTRL3_CLR, CTRL3_SET, CTRL3_TOG, STAT0, STAT0_CLR,
    STAT0_SET, STAT0_TOG, STAT1, STAT1_CLR, STAT1_SET, STAT1_TOG, STAT2, STAT2_CLR, STAT2_SET,
    STAT2_TOG,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The OSC_RC_400M peripheral instance.
#[cfg(not(feature = "doc"))]
pub type OSC_RC_400M = Instance<0>;

/// The OSC_RC_400M peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type OSC_RC_400M = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct OSC_RC_400M {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for OSC_RC_400M {}
impl crate::Valid for OSC_RC_400M {
    fn take() -> Option<Self> {
        <OSC_RC_400M>::take()
    }
    fn release(self) {
        <OSC_RC_400M>::release(self);
    }
    unsafe fn steal() -> Self {
        <OSC_RC_400M>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static OSC_RC_400M_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the OSC_RC_400M peripheral instance
#[cfg(not(feature = "nosync"))]
impl OSC_RC_400M {
    const INSTANCE: Self = Self {
        addr: 0x00000000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in OSC_RC_400M
    pub const reset: ResetValues = ResetValues {
        CTRL0: 0x00000000,
        CTRL0_SET: 0x00000000,
        CTRL0_CLR: 0x00000000,
        CTRL0_TOG: 0x00000000,
        CTRL1: 0x00000000,
        CTRL1_SET: 0x00000000,
        CTRL1_CLR: 0x00000000,
        CTRL1_TOG: 0x00000000,
        CTRL2: 0x00000000,
        CTRL2_SET: 0x00000000,
        CTRL2_CLR: 0x00000000,
        CTRL2_TOG: 0x00000000,
        CTRL3: 0x00000000,
        CTRL3_SET: 0x00000000,
        CTRL3_CLR: 0x00000000,
        CTRL3_TOG: 0x00000000,
        STAT0: 0x00000000,
        STAT0_SET: 0x00000000,
        STAT0_CLR: 0x00000000,
        STAT0_TOG: 0x00000000,
        STAT1: 0x00000000,
        STAT1_SET: 0x00000000,
        STAT1_CLR: 0x00000000,
        STAT1_TOG: 0x00000000,
        STAT2: 0x00000000,
        STAT2_SET: 0x00000000,
        STAT2_CLR: 0x00000000,
        STAT2_TOG: 0x00000000,
    };

    /// Safe access to OSC_RC_400M
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
        let taken = OSC_RC_400M_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to OSC_RC_400M
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = OSC_RC_400M_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal OSC_RC_400M
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        OSC_RC_400M_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl OSC_RC_400M {
    /// The interrupts associated with OSC_RC_400M
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with OSC_RC_400M
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to OSC_RC_400M
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OSC_RC_400M: *const RegisterBlock = 0x00000000 as *const _;
