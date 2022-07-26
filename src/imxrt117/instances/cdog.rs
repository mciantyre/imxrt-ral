#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CDOG
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::cdog::Instance;
pub use crate::imxrt117::peripherals::cdog::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::cdog::{
    ADD, ADD1, ADD16, ADD256, CONTROL, FLAGS, INSTRUCTION_TIMER, PERSISTENT, RELOAD, RESTART,
    SECURE_COUNTER, START, STATUS, STATUS2, STOP, SUB, SUB1, SUB16, SUB256,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The CDOG peripheral instance.
#[cfg(not(feature = "doc"))]
pub type CDOG = Instance<0>;

/// The CDOG peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type CDOG = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct CDOG {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for CDOG {}
impl crate::Valid for CDOG {
    fn take() -> Option<Self> {
        <CDOG>::take()
    }
    fn release(self) {
        <CDOG>::release(self);
    }
    unsafe fn steal() -> Self {
        <CDOG>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static CDOG_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the CDOG peripheral instance
#[cfg(not(feature = "nosync"))]
impl CDOG {
    const INSTANCE: Self = Self {
        addr: 0x41900000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::CDOG],
    };

    /// Reset values for each field in CDOG
    pub const reset: ResetValues = ResetValues {
        CONTROL: 0x50092492,
        RELOAD: 0xFFFFFFFF,
        INSTRUCTION_TIMER: 0xFFFFFFFF,
        SECURE_COUNTER: 0x00000000,
        STATUS: 0x50000000,
        STATUS2: 0x00000000,
        FLAGS: 0x00000000,
        PERSISTENT: 0x00000000,
        START: 0x00000000,
        STOP: 0x00000000,
        RESTART: 0x00000000,
        ADD: 0x00000000,
        ADD1: 0x00000000,
        ADD16: 0x00000000,
        ADD256: 0x00000000,
        SUB: 0x00000000,
        SUB1: 0x00000000,
        SUB16: 0x00000000,
        SUB256: 0x00000000,
    };

    /// Safe access to CDOG
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
        let taken = CDOG_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to CDOG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = CDOG_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CDOG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        CDOG_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl CDOG {
    /// The interrupts associated with CDOG
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::CDOG];

    /// The interrupts associated with CDOG
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to CDOG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CDOG: *const RegisterBlock = 0x41900000 as *const _;
