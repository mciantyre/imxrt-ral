#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Secure RAM
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::sram::Instance;
pub use crate::imxrt117::peripherals::sram::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::sram::CTRL;
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The SRAM peripheral instance.
#[cfg(not(feature = "doc"))]
pub type SRAM = Instance<0>;

/// The SRAM peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type SRAM = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct SRAM {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for SRAM {}
impl crate::Valid for SRAM {
    fn take() -> Option<Self> {
        <SRAM>::take()
    }
    fn release(self) {
        <SRAM>::release(self);
    }
    unsafe fn steal() -> Self {
        <SRAM>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static SRAM_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the SRAM peripheral instance
#[cfg(not(feature = "nosync"))]
impl SRAM {
    const INSTANCE: Self = Self {
        addr: 0x40c9c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in SRAM
    pub const reset: ResetValues = ResetValues { CTRL: 0x00000000 };

    /// Safe access to SRAM
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
        let taken = SRAM_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to SRAM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = SRAM_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal SRAM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        SRAM_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl SRAM {
    /// The interrupts associated with SRAM
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with SRAM
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to SRAM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SRAM: *const RegisterBlock = 0x40c9c000 as *const _;
