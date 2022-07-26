#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ASRC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::asrc::Instance;
pub use crate::imxrt117::peripherals::asrc::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::asrc::{
    ASR56K, ASR76K, ASRCCR, ASRCDR1, ASRCDR2, ASRCFG, ASRCNCR, ASRCSR, ASRCTR, ASRDIA, ASRDIB,
    ASRDIC, ASRDOA, ASRDOB, ASRDOC, ASRFSTA, ASRFSTB, ASRFSTC, ASRIDRHA, ASRIDRHB, ASRIDRHC,
    ASRIDRLA, ASRIDRLB, ASRIDRLC, ASRIER, ASRMCR1A, ASRMCR1B, ASRMCR1C, ASRMCRA, ASRMCRB, ASRMCRC,
    ASRPM1, ASRPM2, ASRPM3, ASRPM4, ASRPM5, ASRSTR, ASRTFR1,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The ASRC peripheral instance.
#[cfg(not(feature = "doc"))]
pub type ASRC = Instance<0>;

/// The ASRC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ASRC = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct ASRC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for ASRC {}
impl crate::Valid for ASRC {
    fn take() -> Option<Self> {
        <ASRC>::take()
    }
    fn release(self) {
        <ASRC>::release(self);
    }
    unsafe fn steal() -> Self {
        <ASRC>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ASRC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ASRC peripheral instance
#[cfg(not(feature = "nosync"))]
impl ASRC {
    const INSTANCE: Self = Self {
        addr: 0x40414000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ASRC],
    };

    /// Reset values for each field in ASRC
    pub const reset: ResetValues = ResetValues {
        ASRCTR: 0x00000000,
        ASRIER: 0x00000000,
        ASRCNCR: 0x00000000,
        ASRCFG: 0x00000000,
        ASRCSR: 0x00000000,
        ASRCDR1: 0x00000000,
        ASRCDR2: 0x00000000,
        ASRSTR: 0x00000000,
        ASRPM1: 0x00000000,
        ASRPM2: 0x00000000,
        ASRPM3: 0x00000000,
        ASRPM4: 0x00000000,
        ASRPM5: 0x00000000,
        ASRTFR1: 0x00000000,
        ASRCCR: 0x00000000,
        ASRDIA: 0x00000000,
        ASRDOA: 0x00000000,
        ASRDIB: 0x00000000,
        ASRDOB: 0x00000000,
        ASRDIC: 0x00000000,
        ASRDOC: 0x00000000,
        ASRIDRHA: 0x00000000,
        ASRIDRLA: 0x00000000,
        ASRIDRHB: 0x00000000,
        ASRIDRLB: 0x00000000,
        ASRIDRHC: 0x00000000,
        ASRIDRLC: 0x00000000,
        ASR76K: 0x00000A47,
        ASR56K: 0x00000DF3,
        ASRMCRA: 0x00000000,
        ASRFSTA: 0x00000000,
        ASRMCRB: 0x00000000,
        ASRFSTB: 0x00000000,
        ASRMCRC: 0x00000000,
        ASRFSTC: 0x00000000,
        ASRMCR1A: 0x00000000,
        ASRMCR1B: 0x00000000,
        ASRMCR1C: 0x00000000,
    };

    /// Safe access to ASRC
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
        let taken = ASRC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ASRC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ASRC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ASRC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ASRC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl ASRC {
    /// The interrupts associated with ASRC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ASRC];

    /// The interrupts associated with ASRC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ASRC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ASRC: *const RegisterBlock = 0x40414000 as *const _;
