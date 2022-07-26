#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DCIC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::dcic::Instance;
pub use crate::imxrt117::peripherals::dcic::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::dcic::{
    DCICC, DCICIC, DCICRC1, DCICRC10, DCICRC11, DCICRC12, DCICRC13, DCICRC14, DCICRC15, DCICRC16,
    DCICRC2, DCICRC3, DCICRC4, DCICRC5, DCICRC6, DCICRC7, DCICRC8, DCICRC9, DCICRCS1, DCICRCS10,
    DCICRCS11, DCICRCS12, DCICRCS13, DCICRCS14, DCICRCS15, DCICRCS16, DCICRCS2, DCICRCS3, DCICRCS4,
    DCICRCS5, DCICRCS6, DCICRCS7, DCICRCS8, DCICRCS9, DCICRRS1, DCICRRS10, DCICRRS11, DCICRRS12,
    DCICRRS13, DCICRRS14, DCICRRS15, DCICRRS16, DCICRRS2, DCICRRS3, DCICRRS4, DCICRRS5, DCICRRS6,
    DCICRRS7, DCICRRS8, DCICRRS9, DCICRS1, DCICRS10, DCICRS11, DCICRS12, DCICRS13, DCICRS14,
    DCICRS15, DCICRS16, DCICRS2, DCICRS3, DCICRS4, DCICRS5, DCICRS6, DCICRS7, DCICRS8, DCICRS9,
    DCICS,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The DCIC1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type DCIC1 = Instance<1>;

/// The DCIC1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type DCIC1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct DCIC1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for DCIC1 {}
impl crate::Valid for DCIC1 {
    fn take() -> Option<Self> {
        <DCIC1>::take()
    }
    fn release(self) {
        <DCIC1>::release(self);
    }
    unsafe fn steal() -> Self {
        <DCIC1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static DCIC1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the DCIC1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl DCIC1 {
    const INSTANCE: Self = Self {
        addr: 0x40819000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::DCIC1],
    };

    /// Reset values for each field in DCIC1
    pub const reset: ResetValues = ResetValues {
        DCICC: 0x00000070,
        DCICIC: 0x00000003,
        DCICS: 0x00000000,
        DCICRC1: 0x00000000,
        DCICRC2: 0x00000000,
        DCICRC3: 0x00000000,
        DCICRC4: 0x00000000,
        DCICRC5: 0x00000000,
        DCICRC6: 0x00000000,
        DCICRC7: 0x00000000,
        DCICRC8: 0x00000000,
        DCICRC9: 0x00000000,
        DCICRC10: 0x00000000,
        DCICRC11: 0x00000000,
        DCICRC12: 0x00000000,
        DCICRC13: 0x00000000,
        DCICRC14: 0x00000000,
        DCICRC15: 0x00000000,
        DCICRC16: 0x00000000,
        DCICRS1: 0x00000000,
        DCICRS2: 0x00000000,
        DCICRS3: 0x00000000,
        DCICRS4: 0x00000000,
        DCICRS5: 0x00000000,
        DCICRS6: 0x00000000,
        DCICRS7: 0x00000000,
        DCICRS8: 0x00000000,
        DCICRS9: 0x00000000,
        DCICRS10: 0x00000000,
        DCICRS11: 0x00000000,
        DCICRS12: 0x00000000,
        DCICRS13: 0x00000000,
        DCICRS14: 0x00000000,
        DCICRS15: 0x00000000,
        DCICRS16: 0x00000000,
        DCICRRS1: 0x00000000,
        DCICRRS2: 0x00000000,
        DCICRRS3: 0x00000000,
        DCICRRS4: 0x00000000,
        DCICRRS5: 0x00000000,
        DCICRRS6: 0x00000000,
        DCICRRS7: 0x00000000,
        DCICRRS8: 0x00000000,
        DCICRRS9: 0x00000000,
        DCICRRS10: 0x00000000,
        DCICRRS11: 0x00000000,
        DCICRRS12: 0x00000000,
        DCICRRS13: 0x00000000,
        DCICRRS14: 0x00000000,
        DCICRRS15: 0x00000000,
        DCICRRS16: 0x00000000,
        DCICRCS1: 0x00000000,
        DCICRCS2: 0x00000000,
        DCICRCS3: 0x00000000,
        DCICRCS4: 0x00000000,
        DCICRCS5: 0x00000000,
        DCICRCS6: 0x00000000,
        DCICRCS7: 0x00000000,
        DCICRCS8: 0x00000000,
        DCICRCS9: 0x00000000,
        DCICRCS10: 0x00000000,
        DCICRCS11: 0x00000000,
        DCICRCS12: 0x00000000,
        DCICRCS13: 0x00000000,
        DCICRCS14: 0x00000000,
        DCICRCS15: 0x00000000,
        DCICRCS16: 0x00000000,
    };

    /// Safe access to DCIC1
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
        let taken = DCIC1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to DCIC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = DCIC1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal DCIC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        DCIC1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl DCIC1 {
    /// The interrupts associated with DCIC1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::DCIC1];

    /// The interrupts associated with DCIC1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to DCIC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DCIC1: *const RegisterBlock = 0x40819000 as *const _;

/// The DCIC2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type DCIC2 = Instance<2>;

/// The DCIC2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type DCIC2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct DCIC2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for DCIC2 {}
impl crate::Valid for DCIC2 {
    fn take() -> Option<Self> {
        <DCIC2>::take()
    }
    fn release(self) {
        <DCIC2>::release(self);
    }
    unsafe fn steal() -> Self {
        <DCIC2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static DCIC2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the DCIC2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl DCIC2 {
    const INSTANCE: Self = Self {
        addr: 0x4081a000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::DCIC2],
    };

    /// Reset values for each field in DCIC2
    pub const reset: ResetValues = ResetValues {
        DCICC: 0x00000070,
        DCICIC: 0x00000003,
        DCICS: 0x00000000,
        DCICRC1: 0x00000000,
        DCICRC2: 0x00000000,
        DCICRC3: 0x00000000,
        DCICRC4: 0x00000000,
        DCICRC5: 0x00000000,
        DCICRC6: 0x00000000,
        DCICRC7: 0x00000000,
        DCICRC8: 0x00000000,
        DCICRC9: 0x00000000,
        DCICRC10: 0x00000000,
        DCICRC11: 0x00000000,
        DCICRC12: 0x00000000,
        DCICRC13: 0x00000000,
        DCICRC14: 0x00000000,
        DCICRC15: 0x00000000,
        DCICRC16: 0x00000000,
        DCICRS1: 0x00000000,
        DCICRS2: 0x00000000,
        DCICRS3: 0x00000000,
        DCICRS4: 0x00000000,
        DCICRS5: 0x00000000,
        DCICRS6: 0x00000000,
        DCICRS7: 0x00000000,
        DCICRS8: 0x00000000,
        DCICRS9: 0x00000000,
        DCICRS10: 0x00000000,
        DCICRS11: 0x00000000,
        DCICRS12: 0x00000000,
        DCICRS13: 0x00000000,
        DCICRS14: 0x00000000,
        DCICRS15: 0x00000000,
        DCICRS16: 0x00000000,
        DCICRRS1: 0x00000000,
        DCICRRS2: 0x00000000,
        DCICRRS3: 0x00000000,
        DCICRRS4: 0x00000000,
        DCICRRS5: 0x00000000,
        DCICRRS6: 0x00000000,
        DCICRRS7: 0x00000000,
        DCICRRS8: 0x00000000,
        DCICRRS9: 0x00000000,
        DCICRRS10: 0x00000000,
        DCICRRS11: 0x00000000,
        DCICRRS12: 0x00000000,
        DCICRRS13: 0x00000000,
        DCICRRS14: 0x00000000,
        DCICRRS15: 0x00000000,
        DCICRRS16: 0x00000000,
        DCICRCS1: 0x00000000,
        DCICRCS2: 0x00000000,
        DCICRCS3: 0x00000000,
        DCICRCS4: 0x00000000,
        DCICRCS5: 0x00000000,
        DCICRCS6: 0x00000000,
        DCICRCS7: 0x00000000,
        DCICRCS8: 0x00000000,
        DCICRCS9: 0x00000000,
        DCICRCS10: 0x00000000,
        DCICRCS11: 0x00000000,
        DCICRCS12: 0x00000000,
        DCICRCS13: 0x00000000,
        DCICRCS14: 0x00000000,
        DCICRCS15: 0x00000000,
        DCICRCS16: 0x00000000,
    };

    /// Safe access to DCIC2
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
        let taken = DCIC2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to DCIC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = DCIC2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal DCIC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        DCIC2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl DCIC2 {
    /// The interrupts associated with DCIC2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::DCIC2];

    /// The interrupts associated with DCIC2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to DCIC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DCIC2: *const RegisterBlock = 0x4081a000 as *const _;
