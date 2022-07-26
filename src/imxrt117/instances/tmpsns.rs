#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Temperature Sensor Memory Map
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::tmpsns::Instance;
pub use crate::imxrt117::peripherals::tmpsns::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::tmpsns::{
    CTRL0, CTRL0_CLR, CTRL0_SET, CTRL0_TOG, CTRL1, CTRL1_CLR, CTRL1_SET, CTRL1_TOG, RANGE0,
    RANGE0_CLR, RANGE0_SET, RANGE0_TOG, RANGE1, RANGE1_CLR, RANGE1_SET, RANGE1_TOG, STATUS0,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The TMPSNS peripheral instance.
#[cfg(not(feature = "doc"))]
pub type TMPSNS = Instance<0>;

/// The TMPSNS peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type TMPSNS = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct TMPSNS {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for TMPSNS {}
impl crate::Valid for TMPSNS {
    fn take() -> Option<Self> {
        <TMPSNS>::take()
    }
    fn release(self) {
        <TMPSNS>::release(self);
    }
    unsafe fn steal() -> Self {
        <TMPSNS>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static TMPSNS_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the TMPSNS peripheral instance
#[cfg(not(feature = "nosync"))]
impl TMPSNS {
    const INSTANCE: Self = Self {
        addr: 0x00000000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::TMPSNS_INT,
            crate::interrupt::TMPSNS_LOW_HIGH,
            crate::interrupt::TMPSNS_PANIC,
        ],
    };

    /// Reset values for each field in TMPSNS
    pub const reset: ResetValues = ResetValues {
        CTRL0: 0x00008020,
        CTRL0_SET: 0x00008020,
        CTRL0_CLR: 0x00008020,
        CTRL0_TOG: 0x00008020,
        CTRL1: 0x00800000,
        CTRL1_SET: 0x00800000,
        CTRL1_CLR: 0x00800000,
        CTRL1_TOG: 0x00800000,
        RANGE0: 0x00000000,
        RANGE0_SET: 0x00000000,
        RANGE0_CLR: 0x00000000,
        RANGE0_TOG: 0x00000000,
        RANGE1: 0x00000000,
        RANGE1_SET: 0x00000000,
        RANGE1_CLR: 0x00000000,
        RANGE1_TOG: 0x00000000,
        STATUS0: 0x00000000,
    };

    /// Safe access to TMPSNS
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
        let taken = TMPSNS_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to TMPSNS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = TMPSNS_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal TMPSNS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        TMPSNS_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl TMPSNS {
    /// The interrupts associated with TMPSNS
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 3] = [
        crate::interrupt::TMPSNS_INT,
        crate::interrupt::TMPSNS_LOW_HIGH,
        crate::interrupt::TMPSNS_PANIC,
    ];

    /// The interrupts associated with TMPSNS
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to TMPSNS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TMPSNS: *const RegisterBlock = 0x00000000 as *const _;
