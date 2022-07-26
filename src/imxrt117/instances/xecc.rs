#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! XECC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::xecc::Instance;
pub use crate::imxrt117::peripherals::xecc::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::xecc::{
    ECC_BASE_ADDR0, ECC_BASE_ADDR1, ECC_BASE_ADDR2, ECC_BASE_ADDR3, ECC_CTRL, ECC_END_ADDR0,
    ECC_END_ADDR1, ECC_END_ADDR2, ECC_END_ADDR3, ERR_DATA_INJ, ERR_ECC_INJ, ERR_SIG_EN, ERR_STATUS,
    ERR_STAT_EN, MULTI_ERR_ADDR, MULTI_ERR_BIT_FIELD, MULTI_ERR_DATA, MULTI_ERR_ECC,
    SINGLE_ERR_ADDR, SINGLE_ERR_BIT_FIELD, SINGLE_ERR_DATA, SINGLE_ERR_ECC, SINGLE_ERR_POS,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The XECC_FLEXSPI1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type XECC_FLEXSPI1 = Instance<1>;

/// The XECC_FLEXSPI1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type XECC_FLEXSPI1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct XECC_FLEXSPI1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for XECC_FLEXSPI1 {}
impl crate::Valid for XECC_FLEXSPI1 {
    fn take() -> Option<Self> {
        <XECC_FLEXSPI1>::take()
    }
    fn release(self) {
        <XECC_FLEXSPI1>::release(self);
    }
    unsafe fn steal() -> Self {
        <XECC_FLEXSPI1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static XECC_FLEXSPI1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the XECC_FLEXSPI1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl XECC_FLEXSPI1 {
    const INSTANCE: Self = Self {
        addr: 0x4001c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::XECC_FLEXSPI1_INT,
            crate::interrupt::XECC_FLEXSPI1_FATAL_INT,
        ],
    };

    /// Reset values for each field in XECC_FLEXSPI1
    pub const reset: ResetValues = ResetValues {
        ECC_CTRL: 0x00000000,
        ERR_STATUS: 0x00000000,
        ERR_STAT_EN: 0x00000000,
        ERR_SIG_EN: 0x00000000,
        ERR_DATA_INJ: 0x00000000,
        ERR_ECC_INJ: 0x00000000,
        SINGLE_ERR_ADDR: 0x00000000,
        SINGLE_ERR_DATA: 0x00000000,
        SINGLE_ERR_ECC: 0x00000000,
        SINGLE_ERR_POS: 0x00000000,
        SINGLE_ERR_BIT_FIELD: 0x00000000,
        MULTI_ERR_ADDR: 0x00000000,
        MULTI_ERR_DATA: 0x00000000,
        MULTI_ERR_ECC: 0x00000000,
        MULTI_ERR_BIT_FIELD: 0x00000000,
        ECC_BASE_ADDR0: 0x00000000,
        ECC_END_ADDR0: 0x00000000,
        ECC_BASE_ADDR1: 0x00000000,
        ECC_END_ADDR1: 0x00000000,
        ECC_BASE_ADDR2: 0x00000000,
        ECC_END_ADDR2: 0x00000000,
        ECC_BASE_ADDR3: 0x00000000,
        ECC_END_ADDR3: 0x00000000,
    };

    /// Safe access to XECC_FLEXSPI1
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
        let taken = XECC_FLEXSPI1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to XECC_FLEXSPI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = XECC_FLEXSPI1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal XECC_FLEXSPI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        XECC_FLEXSPI1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl XECC_FLEXSPI1 {
    /// The interrupts associated with XECC_FLEXSPI1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::XECC_FLEXSPI1_INT,
        crate::interrupt::XECC_FLEXSPI1_FATAL_INT,
    ];

    /// The interrupts associated with XECC_FLEXSPI1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to XECC_FLEXSPI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XECC_FLEXSPI1: *const RegisterBlock = 0x4001c000 as *const _;

/// The XECC_FLEXSPI2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type XECC_FLEXSPI2 = Instance<2>;

/// The XECC_FLEXSPI2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type XECC_FLEXSPI2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct XECC_FLEXSPI2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for XECC_FLEXSPI2 {}
impl crate::Valid for XECC_FLEXSPI2 {
    fn take() -> Option<Self> {
        <XECC_FLEXSPI2>::take()
    }
    fn release(self) {
        <XECC_FLEXSPI2>::release(self);
    }
    unsafe fn steal() -> Self {
        <XECC_FLEXSPI2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static XECC_FLEXSPI2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the XECC_FLEXSPI2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl XECC_FLEXSPI2 {
    const INSTANCE: Self = Self {
        addr: 0x40020000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::XECC_FLEXSPI2_INT,
            crate::interrupt::XECC_FLEXSPI2_FATAL_INT,
        ],
    };

    /// Reset values for each field in XECC_FLEXSPI2
    pub const reset: ResetValues = ResetValues {
        ECC_CTRL: 0x00000000,
        ERR_STATUS: 0x00000000,
        ERR_STAT_EN: 0x00000000,
        ERR_SIG_EN: 0x00000000,
        ERR_DATA_INJ: 0x00000000,
        ERR_ECC_INJ: 0x00000000,
        SINGLE_ERR_ADDR: 0x00000000,
        SINGLE_ERR_DATA: 0x00000000,
        SINGLE_ERR_ECC: 0x00000000,
        SINGLE_ERR_POS: 0x00000000,
        SINGLE_ERR_BIT_FIELD: 0x00000000,
        MULTI_ERR_ADDR: 0x00000000,
        MULTI_ERR_DATA: 0x00000000,
        MULTI_ERR_ECC: 0x00000000,
        MULTI_ERR_BIT_FIELD: 0x00000000,
        ECC_BASE_ADDR0: 0x00000000,
        ECC_END_ADDR0: 0x00000000,
        ECC_BASE_ADDR1: 0x00000000,
        ECC_END_ADDR1: 0x00000000,
        ECC_BASE_ADDR2: 0x00000000,
        ECC_END_ADDR2: 0x00000000,
        ECC_BASE_ADDR3: 0x00000000,
        ECC_END_ADDR3: 0x00000000,
    };

    /// Safe access to XECC_FLEXSPI2
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
        let taken = XECC_FLEXSPI2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to XECC_FLEXSPI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = XECC_FLEXSPI2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal XECC_FLEXSPI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        XECC_FLEXSPI2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl XECC_FLEXSPI2 {
    /// The interrupts associated with XECC_FLEXSPI2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::XECC_FLEXSPI2_INT,
        crate::interrupt::XECC_FLEXSPI2_FATAL_INT,
    ];

    /// The interrupts associated with XECC_FLEXSPI2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to XECC_FLEXSPI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XECC_FLEXSPI2: *const RegisterBlock = 0x40020000 as *const _;

/// The XECC_SEMC peripheral instance.
#[cfg(not(feature = "doc"))]
pub type XECC_SEMC = Instance<0>;

/// The XECC_SEMC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type XECC_SEMC = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct XECC_SEMC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for XECC_SEMC {}
impl crate::Valid for XECC_SEMC {
    fn take() -> Option<Self> {
        <XECC_SEMC>::take()
    }
    fn release(self) {
        <XECC_SEMC>::release(self);
    }
    unsafe fn steal() -> Self {
        <XECC_SEMC>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static XECC_SEMC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the XECC_SEMC peripheral instance
#[cfg(not(feature = "nosync"))]
impl XECC_SEMC {
    const INSTANCE: Self = Self {
        addr: 0x40024000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::XECC_SEMC_INT,
            crate::interrupt::XECC_SEMC_FATAL_INT,
        ],
    };

    /// Reset values for each field in XECC_SEMC
    pub const reset: ResetValues = ResetValues {
        ECC_CTRL: 0x00000000,
        ERR_STATUS: 0x00000000,
        ERR_STAT_EN: 0x00000000,
        ERR_SIG_EN: 0x00000000,
        ERR_DATA_INJ: 0x00000000,
        ERR_ECC_INJ: 0x00000000,
        SINGLE_ERR_ADDR: 0x00000000,
        SINGLE_ERR_DATA: 0x00000000,
        SINGLE_ERR_ECC: 0x00000000,
        SINGLE_ERR_POS: 0x00000000,
        SINGLE_ERR_BIT_FIELD: 0x00000000,
        MULTI_ERR_ADDR: 0x00000000,
        MULTI_ERR_DATA: 0x00000000,
        MULTI_ERR_ECC: 0x00000000,
        MULTI_ERR_BIT_FIELD: 0x00000000,
        ECC_BASE_ADDR0: 0x00000000,
        ECC_END_ADDR0: 0x00000000,
        ECC_BASE_ADDR1: 0x00000000,
        ECC_END_ADDR1: 0x00000000,
        ECC_BASE_ADDR2: 0x00000000,
        ECC_END_ADDR2: 0x00000000,
        ECC_BASE_ADDR3: 0x00000000,
        ECC_END_ADDR3: 0x00000000,
    };

    /// Safe access to XECC_SEMC
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
        let taken = XECC_SEMC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to XECC_SEMC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = XECC_SEMC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal XECC_SEMC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        XECC_SEMC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl XECC_SEMC {
    /// The interrupts associated with XECC_SEMC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::XECC_SEMC_INT,
        crate::interrupt::XECC_SEMC_FATAL_INT,
    ];

    /// The interrupts associated with XECC_SEMC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to XECC_SEMC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XECC_SEMC: *const RegisterBlock = 0x40024000 as *const _;
