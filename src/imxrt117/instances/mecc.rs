#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MECC64
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::mecc::Instance;
pub use crate::imxrt117::peripherals::mecc::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::mecc::{
    ERR_DATA_INJ_HIGH0, ERR_DATA_INJ_HIGH1, ERR_DATA_INJ_HIGH2, ERR_DATA_INJ_HIGH3,
    ERR_DATA_INJ_LOW0, ERR_DATA_INJ_LOW1, ERR_DATA_INJ_LOW2, ERR_DATA_INJ_LOW3, ERR_ECC_INJ0,
    ERR_ECC_INJ1, ERR_ECC_INJ2, ERR_ECC_INJ3, ERR_SIG_EN, ERR_STATUS, ERR_STAT_EN,
    MULTI_ERR_ADDR_ECC0, MULTI_ERR_ADDR_ECC1, MULTI_ERR_ADDR_ECC2, MULTI_ERR_ADDR_ECC3,
    MULTI_ERR_DATA_HIGH0, MULTI_ERR_DATA_HIGH1, MULTI_ERR_DATA_HIGH2, MULTI_ERR_DATA_HIGH3,
    MULTI_ERR_DATA_LOW0, MULTI_ERR_DATA_LOW1, MULTI_ERR_DATA_LOW2, MULTI_ERR_DATA_LOW3,
    PENDING_STAT, PIPE_ECC_EN, SINGLE_ERR_ADDR_ECC0, SINGLE_ERR_ADDR_ECC1, SINGLE_ERR_ADDR_ECC2,
    SINGLE_ERR_ADDR_ECC3, SINGLE_ERR_DATA_HIGH0, SINGLE_ERR_DATA_HIGH1, SINGLE_ERR_DATA_HIGH2,
    SINGLE_ERR_DATA_HIGH3, SINGLE_ERR_DATA_LOW0, SINGLE_ERR_DATA_LOW1, SINGLE_ERR_DATA_LOW2,
    SINGLE_ERR_DATA_LOW3, SINGLE_ERR_POS_HIGH0, SINGLE_ERR_POS_HIGH1, SINGLE_ERR_POS_HIGH2,
    SINGLE_ERR_POS_HIGH3, SINGLE_ERR_POS_LOW0, SINGLE_ERR_POS_LOW1, SINGLE_ERR_POS_LOW2,
    SINGLE_ERR_POS_LOW3,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The MECC1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type MECC1 = Instance<1>;

/// The MECC1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type MECC1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct MECC1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for MECC1 {}
impl crate::Valid for MECC1 {
    fn take() -> Option<Self> {
        <MECC1>::take()
    }
    fn release(self) {
        <MECC1>::release(self);
    }
    unsafe fn steal() -> Self {
        <MECC1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static MECC1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the MECC1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl MECC1 {
    const INSTANCE: Self = Self {
        addr: 0x40014000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::MECC1_INT,
            crate::interrupt::MECC1_FATAL_INT,
            crate::interrupt::MECC2_FATAL_INT,
        ],
    };

    /// Reset values for each field in MECC1
    pub const reset: ResetValues = ResetValues {
        ERR_STATUS: 0x00000000,
        ERR_STAT_EN: 0x00000000,
        ERR_SIG_EN: 0x00000000,
        ERR_DATA_INJ_LOW0: 0x00000000,
        ERR_DATA_INJ_HIGH0: 0x00000000,
        ERR_ECC_INJ0: 0x00000000,
        ERR_DATA_INJ_LOW1: 0x00000000,
        ERR_DATA_INJ_HIGH1: 0x00000000,
        ERR_ECC_INJ1: 0x00000000,
        ERR_DATA_INJ_LOW2: 0x00000000,
        ERR_DATA_INJ_HIGH2: 0x00000000,
        ERR_ECC_INJ2: 0x00000000,
        ERR_DATA_INJ_LOW3: 0x00000000,
        ERR_DATA_INJ_HIGH3: 0x00000000,
        ERR_ECC_INJ3: 0x00000000,
        SINGLE_ERR_ADDR_ECC0: 0x00000000,
        SINGLE_ERR_DATA_LOW0: 0x00000000,
        SINGLE_ERR_DATA_HIGH0: 0x00000000,
        SINGLE_ERR_POS_LOW0: 0x00000000,
        SINGLE_ERR_POS_HIGH0: 0x00000000,
        SINGLE_ERR_ADDR_ECC1: 0x00000000,
        SINGLE_ERR_DATA_LOW1: 0x00000000,
        SINGLE_ERR_DATA_HIGH1: 0x00000000,
        SINGLE_ERR_POS_LOW1: 0x00000000,
        SINGLE_ERR_POS_HIGH1: 0x00000000,
        SINGLE_ERR_ADDR_ECC2: 0x00000000,
        SINGLE_ERR_DATA_LOW2: 0x00000000,
        SINGLE_ERR_DATA_HIGH2: 0x00000000,
        SINGLE_ERR_POS_LOW2: 0x00000000,
        SINGLE_ERR_POS_HIGH2: 0x00000000,
        SINGLE_ERR_ADDR_ECC3: 0x00000000,
        SINGLE_ERR_DATA_LOW3: 0x00000000,
        SINGLE_ERR_DATA_HIGH3: 0x00000000,
        SINGLE_ERR_POS_LOW3: 0x00000000,
        SINGLE_ERR_POS_HIGH3: 0x00000000,
        MULTI_ERR_ADDR_ECC0: 0x00000000,
        MULTI_ERR_DATA_LOW0: 0x00000000,
        MULTI_ERR_DATA_HIGH0: 0x00000000,
        MULTI_ERR_ADDR_ECC1: 0x00000000,
        MULTI_ERR_DATA_LOW1: 0x00000000,
        MULTI_ERR_DATA_HIGH1: 0x00000000,
        MULTI_ERR_ADDR_ECC2: 0x00000000,
        MULTI_ERR_DATA_LOW2: 0x00000000,
        MULTI_ERR_DATA_HIGH2: 0x00000000,
        MULTI_ERR_ADDR_ECC3: 0x00000000,
        MULTI_ERR_DATA_LOW3: 0x00000000,
        MULTI_ERR_DATA_HIGH3: 0x00000000,
        PIPE_ECC_EN: 0x00000000,
        PENDING_STAT: 0x00000000,
    };

    /// Safe access to MECC1
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
        let taken = MECC1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to MECC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = MECC1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal MECC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        MECC1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl MECC1 {
    /// The interrupts associated with MECC1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 3] = [
        crate::interrupt::MECC1_INT,
        crate::interrupt::MECC1_FATAL_INT,
        crate::interrupt::MECC2_FATAL_INT,
    ];

    /// The interrupts associated with MECC1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to MECC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const MECC1: *const RegisterBlock = 0x40014000 as *const _;

/// The MECC2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type MECC2 = Instance<2>;

/// The MECC2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type MECC2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct MECC2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for MECC2 {}
impl crate::Valid for MECC2 {
    fn take() -> Option<Self> {
        <MECC2>::take()
    }
    fn release(self) {
        <MECC2>::release(self);
    }
    unsafe fn steal() -> Self {
        <MECC2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static MECC2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the MECC2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl MECC2 {
    const INSTANCE: Self = Self {
        addr: 0x40018000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::MECC2_INT],
    };

    /// Reset values for each field in MECC2
    pub const reset: ResetValues = ResetValues {
        ERR_STATUS: 0x00000000,
        ERR_STAT_EN: 0x00000000,
        ERR_SIG_EN: 0x00000000,
        ERR_DATA_INJ_LOW0: 0x00000000,
        ERR_DATA_INJ_HIGH0: 0x00000000,
        ERR_ECC_INJ0: 0x00000000,
        ERR_DATA_INJ_LOW1: 0x00000000,
        ERR_DATA_INJ_HIGH1: 0x00000000,
        ERR_ECC_INJ1: 0x00000000,
        ERR_DATA_INJ_LOW2: 0x00000000,
        ERR_DATA_INJ_HIGH2: 0x00000000,
        ERR_ECC_INJ2: 0x00000000,
        ERR_DATA_INJ_LOW3: 0x00000000,
        ERR_DATA_INJ_HIGH3: 0x00000000,
        ERR_ECC_INJ3: 0x00000000,
        SINGLE_ERR_ADDR_ECC0: 0x00000000,
        SINGLE_ERR_DATA_LOW0: 0x00000000,
        SINGLE_ERR_DATA_HIGH0: 0x00000000,
        SINGLE_ERR_POS_LOW0: 0x00000000,
        SINGLE_ERR_POS_HIGH0: 0x00000000,
        SINGLE_ERR_ADDR_ECC1: 0x00000000,
        SINGLE_ERR_DATA_LOW1: 0x00000000,
        SINGLE_ERR_DATA_HIGH1: 0x00000000,
        SINGLE_ERR_POS_LOW1: 0x00000000,
        SINGLE_ERR_POS_HIGH1: 0x00000000,
        SINGLE_ERR_ADDR_ECC2: 0x00000000,
        SINGLE_ERR_DATA_LOW2: 0x00000000,
        SINGLE_ERR_DATA_HIGH2: 0x00000000,
        SINGLE_ERR_POS_LOW2: 0x00000000,
        SINGLE_ERR_POS_HIGH2: 0x00000000,
        SINGLE_ERR_ADDR_ECC3: 0x00000000,
        SINGLE_ERR_DATA_LOW3: 0x00000000,
        SINGLE_ERR_DATA_HIGH3: 0x00000000,
        SINGLE_ERR_POS_LOW3: 0x00000000,
        SINGLE_ERR_POS_HIGH3: 0x00000000,
        MULTI_ERR_ADDR_ECC0: 0x00000000,
        MULTI_ERR_DATA_LOW0: 0x00000000,
        MULTI_ERR_DATA_HIGH0: 0x00000000,
        MULTI_ERR_ADDR_ECC1: 0x00000000,
        MULTI_ERR_DATA_LOW1: 0x00000000,
        MULTI_ERR_DATA_HIGH1: 0x00000000,
        MULTI_ERR_ADDR_ECC2: 0x00000000,
        MULTI_ERR_DATA_LOW2: 0x00000000,
        MULTI_ERR_DATA_HIGH2: 0x00000000,
        MULTI_ERR_ADDR_ECC3: 0x00000000,
        MULTI_ERR_DATA_LOW3: 0x00000000,
        MULTI_ERR_DATA_HIGH3: 0x00000000,
        PIPE_ECC_EN: 0x00000000,
        PENDING_STAT: 0x00000000,
    };

    /// Safe access to MECC2
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
        let taken = MECC2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to MECC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = MECC2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal MECC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        MECC2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl MECC2 {
    /// The interrupts associated with MECC2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::MECC2_INT];

    /// The interrupts associated with MECC2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to MECC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const MECC2: *const RegisterBlock = 0x40018000 as *const _;
