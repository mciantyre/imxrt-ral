#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPADC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::lpadc::Instance;
pub use crate::imxrt117::peripherals::lpadc::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::lpadc::{
    CFG, CMDH1, CMDH10, CMDH11, CMDH12, CMDH13, CMDH14, CMDH15, CMDH2, CMDH3, CMDH4, CMDH5, CMDH6,
    CMDH7, CMDH8, CMDH9, CMDL1, CMDL10, CMDL11, CMDL12, CMDL13, CMDL14, CMDL15, CMDL2, CMDL3,
    CMDL4, CMDL5, CMDL6, CMDL7, CMDL8, CMDL9, CTRL, CV1, CV2, CV3, CV4, DE, FCTRL, IE, PARAM,
    PAUSE, RESFIFO, STAT, SWTRIG, TCTRL0, TCTRL1, TCTRL2, TCTRL3, TCTRL4, TCTRL5, TCTRL6, TCTRL7,
    VERID,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The LPADC1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPADC1 = Instance<1>;

/// The LPADC1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPADC1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct LPADC1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPADC1 {}
impl crate::Valid for LPADC1 {
    fn take() -> Option<Self> {
        <LPADC1>::take()
    }
    fn release(self) {
        <LPADC1>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPADC1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPADC1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPADC1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPADC1 {
    const INSTANCE: Self = Self {
        addr: 0x40050000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ADC1],
    };

    /// Reset values for each field in LPADC1
    pub const reset: ResetValues = ResetValues {
        VERID: 0x0100001A,
        PARAM: 0x0F041008,
        CTRL: 0x00000000,
        STAT: 0x00000000,
        IE: 0x00000000,
        DE: 0x00000000,
        CFG: 0x00800000,
        PAUSE: 0x00000000,
        FCTRL: 0x00000000,
        SWTRIG: 0x00000000,
        TCTRL0: 0x00000000,
        TCTRL1: 0x00000000,
        TCTRL2: 0x00000000,
        TCTRL3: 0x00000000,
        TCTRL4: 0x00000000,
        TCTRL5: 0x00000000,
        TCTRL6: 0x00000000,
        TCTRL7: 0x00000000,
        CMDL1: 0x00002000,
        CMDH1: 0x00000000,
        CMDL2: 0x00002000,
        CMDH2: 0x00000000,
        CMDL3: 0x00002000,
        CMDH3: 0x00000000,
        CMDL4: 0x00002000,
        CMDH4: 0x00000000,
        CMDL5: 0x00002000,
        CMDH5: 0x00000000,
        CMDL6: 0x00002000,
        CMDH6: 0x00000000,
        CMDL7: 0x00002000,
        CMDH7: 0x00000000,
        CMDL8: 0x00002000,
        CMDH8: 0x00000000,
        CMDL9: 0x00002000,
        CMDH9: 0x00000000,
        CMDL10: 0x00002000,
        CMDH10: 0x00000000,
        CMDL11: 0x00002000,
        CMDH11: 0x00000000,
        CMDL12: 0x00002000,
        CMDH12: 0x00000000,
        CMDL13: 0x00002000,
        CMDH13: 0x00000000,
        CMDL14: 0x00002000,
        CMDH14: 0x00000000,
        CMDL15: 0x00002000,
        CMDH15: 0x00000000,
        CV1: 0x00000000,
        CV2: 0x00000000,
        CV3: 0x00000000,
        CV4: 0x00000000,
        RESFIFO: 0x00000000,
    };

    /// Safe access to LPADC1
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
        let taken = LPADC1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPADC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPADC1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPADC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPADC1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPADC1 {
    /// The interrupts associated with LPADC1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ADC1];

    /// The interrupts associated with LPADC1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPADC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPADC1: *const RegisterBlock = 0x40050000 as *const _;

/// The LPADC2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPADC2 = Instance<2>;

/// The LPADC2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPADC2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct LPADC2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPADC2 {}
impl crate::Valid for LPADC2 {
    fn take() -> Option<Self> {
        <LPADC2>::take()
    }
    fn release(self) {
        <LPADC2>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPADC2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPADC2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPADC2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPADC2 {
    const INSTANCE: Self = Self {
        addr: 0x40054000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::ADC2],
    };

    /// Reset values for each field in LPADC2
    pub const reset: ResetValues = ResetValues {
        VERID: 0x0100001A,
        PARAM: 0x0F041008,
        CTRL: 0x00000000,
        STAT: 0x00000000,
        IE: 0x00000000,
        DE: 0x00000000,
        CFG: 0x00800000,
        PAUSE: 0x00000000,
        FCTRL: 0x00000000,
        SWTRIG: 0x00000000,
        TCTRL0: 0x00000000,
        TCTRL1: 0x00000000,
        TCTRL2: 0x00000000,
        TCTRL3: 0x00000000,
        TCTRL4: 0x00000000,
        TCTRL5: 0x00000000,
        TCTRL6: 0x00000000,
        TCTRL7: 0x00000000,
        CMDL1: 0x00002000,
        CMDH1: 0x00000000,
        CMDL2: 0x00002000,
        CMDH2: 0x00000000,
        CMDL3: 0x00002000,
        CMDH3: 0x00000000,
        CMDL4: 0x00002000,
        CMDH4: 0x00000000,
        CMDL5: 0x00002000,
        CMDH5: 0x00000000,
        CMDL6: 0x00002000,
        CMDH6: 0x00000000,
        CMDL7: 0x00002000,
        CMDH7: 0x00000000,
        CMDL8: 0x00002000,
        CMDH8: 0x00000000,
        CMDL9: 0x00002000,
        CMDH9: 0x00000000,
        CMDL10: 0x00002000,
        CMDH10: 0x00000000,
        CMDL11: 0x00002000,
        CMDH11: 0x00000000,
        CMDL12: 0x00002000,
        CMDH12: 0x00000000,
        CMDL13: 0x00002000,
        CMDH13: 0x00000000,
        CMDL14: 0x00002000,
        CMDH14: 0x00000000,
        CMDL15: 0x00002000,
        CMDH15: 0x00000000,
        CV1: 0x00000000,
        CV2: 0x00000000,
        CV3: 0x00000000,
        CV4: 0x00000000,
        RESFIFO: 0x00000000,
    };

    /// Safe access to LPADC2
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
        let taken = LPADC2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPADC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPADC2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPADC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPADC2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPADC2 {
    /// The interrupts associated with LPADC2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::ADC2];

    /// The interrupts associated with LPADC2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPADC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPADC2: *const RegisterBlock = 0x40054000 as *const _;
