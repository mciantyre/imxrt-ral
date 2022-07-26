#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! OTFAD
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::otfad::Instance;
pub use crate::imxrt117::peripherals::otfad::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::otfad::{
    CR, CTX_CTR0_0, CTX_CTR0_1, CTX_CTR0_2, CTX_CTR0_3, CTX_CTR1_0, CTX_CTR1_1, CTX_CTR1_2,
    CTX_CTR1_3, CTX_KEY0_0, CTX_KEY0_1, CTX_KEY0_2, CTX_KEY0_3, CTX_KEY1_0, CTX_KEY1_1, CTX_KEY1_2,
    CTX_KEY1_3, CTX_KEY2_0, CTX_KEY2_1, CTX_KEY2_2, CTX_KEY2_3, CTX_KEY3_0, CTX_KEY3_1, CTX_KEY3_2,
    CTX_KEY3_3, CTX_RGD_W0_0, CTX_RGD_W0_1, CTX_RGD_W0_2, CTX_RGD_W0_3, CTX_RGD_W1_0, CTX_RGD_W1_1,
    CTX_RGD_W1_2, CTX_RGD_W1_3, SR,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The OTFAD1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type OTFAD1 = Instance<1>;

/// The OTFAD1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type OTFAD1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct OTFAD1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for OTFAD1 {}
impl crate::Valid for OTFAD1 {
    fn take() -> Option<Self> {
        <OTFAD1>::take()
    }
    fn release(self) {
        <OTFAD1>::release(self);
    }
    unsafe fn steal() -> Self {
        <OTFAD1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static OTFAD1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the OTFAD1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl OTFAD1 {
    const INSTANCE: Self = Self {
        addr: 0x400cc000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in OTFAD1
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000040,
        CTX_KEY0_0: 0x00000000,
        CTX_KEY1_0: 0x00000000,
        CTX_KEY2_0: 0x00000000,
        CTX_KEY3_0: 0x00000000,
        CTX_CTR0_0: 0x00000000,
        CTX_CTR1_0: 0x00000000,
        CTX_RGD_W0_0: 0x00000000,
        CTX_RGD_W1_0: 0x000003F8,
        CTX_KEY0_1: 0x00000000,
        CTX_KEY1_1: 0x00000000,
        CTX_KEY2_1: 0x00000000,
        CTX_KEY3_1: 0x00000000,
        CTX_CTR0_1: 0x00000000,
        CTX_CTR1_1: 0x00000000,
        CTX_RGD_W0_1: 0x00000000,
        CTX_RGD_W1_1: 0x000003F8,
        CTX_KEY0_2: 0x00000000,
        CTX_KEY1_2: 0x00000000,
        CTX_KEY2_2: 0x00000000,
        CTX_KEY3_2: 0x00000000,
        CTX_CTR0_2: 0x00000000,
        CTX_CTR1_2: 0x00000000,
        CTX_RGD_W0_2: 0x00000000,
        CTX_RGD_W1_2: 0x000003F8,
        CTX_KEY0_3: 0x00000000,
        CTX_KEY1_3: 0x00000000,
        CTX_KEY2_3: 0x00000000,
        CTX_KEY3_3: 0x00000000,
        CTX_CTR0_3: 0x00000000,
        CTX_CTR1_3: 0x00000000,
        CTX_RGD_W0_3: 0x00000000,
        CTX_RGD_W1_3: 0x000003F8,
    };

    /// Safe access to OTFAD1
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
        let taken = OTFAD1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to OTFAD1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = OTFAD1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal OTFAD1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        OTFAD1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl OTFAD1 {
    /// The interrupts associated with OTFAD1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with OTFAD1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to OTFAD1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTFAD1: *const RegisterBlock = 0x400cc000 as *const _;

/// The OTFAD2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type OTFAD2 = Instance<2>;

/// The OTFAD2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type OTFAD2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct OTFAD2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for OTFAD2 {}
impl crate::Valid for OTFAD2 {
    fn take() -> Option<Self> {
        <OTFAD2>::take()
    }
    fn release(self) {
        <OTFAD2>::release(self);
    }
    unsafe fn steal() -> Self {
        <OTFAD2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static OTFAD2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the OTFAD2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl OTFAD2 {
    const INSTANCE: Self = Self {
        addr: 0x400d0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in OTFAD2
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000040,
        CTX_KEY0_0: 0x00000000,
        CTX_KEY1_0: 0x00000000,
        CTX_KEY2_0: 0x00000000,
        CTX_KEY3_0: 0x00000000,
        CTX_CTR0_0: 0x00000000,
        CTX_CTR1_0: 0x00000000,
        CTX_RGD_W0_0: 0x00000000,
        CTX_RGD_W1_0: 0x000003F8,
        CTX_KEY0_1: 0x00000000,
        CTX_KEY1_1: 0x00000000,
        CTX_KEY2_1: 0x00000000,
        CTX_KEY3_1: 0x00000000,
        CTX_CTR0_1: 0x00000000,
        CTX_CTR1_1: 0x00000000,
        CTX_RGD_W0_1: 0x00000000,
        CTX_RGD_W1_1: 0x000003F8,
        CTX_KEY0_2: 0x00000000,
        CTX_KEY1_2: 0x00000000,
        CTX_KEY2_2: 0x00000000,
        CTX_KEY3_2: 0x00000000,
        CTX_CTR0_2: 0x00000000,
        CTX_CTR1_2: 0x00000000,
        CTX_RGD_W0_2: 0x00000000,
        CTX_RGD_W1_2: 0x000003F8,
        CTX_KEY0_3: 0x00000000,
        CTX_KEY1_3: 0x00000000,
        CTX_KEY2_3: 0x00000000,
        CTX_KEY3_3: 0x00000000,
        CTX_CTR0_3: 0x00000000,
        CTX_CTR1_3: 0x00000000,
        CTX_RGD_W0_3: 0x00000000,
        CTX_RGD_W1_3: 0x000003F8,
    };

    /// Safe access to OTFAD2
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
        let taken = OTFAD2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to OTFAD2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = OTFAD2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal OTFAD2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        OTFAD2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl OTFAD2 {
    /// The interrupts associated with OTFAD2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with OTFAD2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to OTFAD2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTFAD2: *const RegisterBlock = 0x400d0000 as *const _;
