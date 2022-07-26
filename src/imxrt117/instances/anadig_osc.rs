#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MX6RT_ANADIG_REGISTER
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::anadig_osc::Instance;
pub use crate::imxrt117::peripherals::anadig_osc::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::anadig_osc::{
    OSC_16M_CTRL, OSC_24M_CTRL, OSC_400M_CTRL0, OSC_400M_CTRL1, OSC_400M_CTRL2, OSC_48M_CTRL,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The ANADIG_OSC peripheral instance.
#[cfg(not(feature = "doc"))]
pub type ANADIG_OSC = Instance<0>;

/// The ANADIG_OSC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ANADIG_OSC = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct ANADIG_OSC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for ANADIG_OSC {}
impl crate::Valid for ANADIG_OSC {
    fn take() -> Option<Self> {
        <ANADIG_OSC>::take()
    }
    fn release(self) {
        <ANADIG_OSC>::release(self);
    }
    unsafe fn steal() -> Self {
        <ANADIG_OSC>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ANADIG_OSC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ANADIG_OSC peripheral instance
#[cfg(not(feature = "nosync"))]
impl ANADIG_OSC {
    const INSTANCE: Self = Self {
        addr: 0x40c84000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in ANADIG_OSC
    pub const reset: ResetValues = ResetValues {
        OSC_48M_CTRL: 0x017901F2,
        OSC_24M_CTRL: 0x00000080,
        OSC_400M_CTRL0: 0x00000000,
        OSC_400M_CTRL1: 0x00000001,
        OSC_400M_CTRL2: 0x00000000,
        OSC_16M_CTRL: 0x00000007,
    };

    /// Safe access to ANADIG_OSC
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
        let taken = ANADIG_OSC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ANADIG_OSC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ANADIG_OSC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ANADIG_OSC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ANADIG_OSC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl ANADIG_OSC {
    /// The interrupts associated with ANADIG_OSC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with ANADIG_OSC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ANADIG_OSC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ANADIG_OSC: *const RegisterBlock = 0x40c84000 as *const _;
