#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DAC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::dac::Instance;
pub use crate::imxrt117::peripherals::dac::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::dac::{CR, CR2, DATA, PARAM, PTR, VERID};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The DAC peripheral instance.
#[cfg(not(feature = "doc"))]
pub type DAC = Instance<0>;

/// The DAC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type DAC = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct DAC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for DAC {}
impl crate::Valid for DAC {
    fn take() -> Option<Self> {
        <DAC>::take()
    }
    fn release(self) {
        <DAC>::release(self);
    }
    unsafe fn steal() -> Self {
        <DAC>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static DAC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the DAC peripheral instance
#[cfg(not(feature = "nosync"))]
impl DAC {
    const INSTANCE: Self = Self {
        addr: 0x40064000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::DAC],
    };

    /// Reset values for each field in DAC
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01000000,
        PARAM: 0x00000003,
        DATA: 0x00000000,
        CR: 0x00000002,
        PTR: 0x00000000,
        CR2: 0x00000000,
    };

    /// Safe access to DAC
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
        let taken = DAC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to DAC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = DAC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal DAC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        DAC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl DAC {
    /// The interrupts associated with DAC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::DAC];

    /// The interrupts associated with DAC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to DAC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DAC: *const RegisterBlock = 0x40064000 as *const _;
