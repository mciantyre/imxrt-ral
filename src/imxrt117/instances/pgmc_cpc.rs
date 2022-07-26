#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PGMC_CPC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::pgmc_cpc::Instance;
pub use crate::imxrt117::peripherals::pgmc_cpc::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::pgmc_cpc::{
    CPC_AUTHEN_CTRL, CPC_CACHE_CM_CTRL, CPC_CACHE_MODE, CPC_CACHE_SP_CTRL_0, CPC_CACHE_SP_CTRL_1,
    CPC_CORE_MODE, CPC_CORE_POWER_CTRL, CPC_FLAG, CPC_LMEM_CM_CTRL, CPC_LMEM_MODE,
    CPC_LMEM_SP_CTRL_0, CPC_LMEM_SP_CTRL_1,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The PGMC_CPC0 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_CPC0 = Instance<1>;

/// The PGMC_CPC0 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_CPC0 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_CPC0 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_CPC0 {}
impl crate::Valid for PGMC_CPC0 {
    fn take() -> Option<Self> {
        <PGMC_CPC0>::take()
    }
    fn release(self) {
        <PGMC_CPC0>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_CPC0>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_CPC0_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_CPC0 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_CPC0 {
    const INSTANCE: Self = Self {
        addr: 0x40c89000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_CPC0
    pub const reset: ResetValues = ResetValues {
        CPC_AUTHEN_CTRL: 0x00000F00,
        CPC_CORE_MODE: 0x00000000,
        CPC_CORE_POWER_CTRL: 0x00000000,
        CPC_FLAG: 0x00000000,
        CPC_CACHE_MODE: 0x00000000,
        CPC_CACHE_CM_CTRL: 0x00003330,
        CPC_CACHE_SP_CTRL_0: 0x33333333,
        CPC_CACHE_SP_CTRL_1: 0x33333333,
        CPC_LMEM_MODE: 0x00000000,
        CPC_LMEM_CM_CTRL: 0x00000000,
        CPC_LMEM_SP_CTRL_0: 0x00000000,
        CPC_LMEM_SP_CTRL_1: 0x00000000,
    };

    /// Safe access to PGMC_CPC0
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
        let taken = PGMC_CPC0_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_CPC0
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_CPC0_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_CPC0
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_CPC0_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_CPC0 {
    /// The interrupts associated with PGMC_CPC0
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_CPC0
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_CPC0
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_CPC0: *const RegisterBlock = 0x40c89000 as *const _;

/// The PGMC_CPC1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_CPC1 = Instance<2>;

/// The PGMC_CPC1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_CPC1 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_CPC1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_CPC1 {}
impl crate::Valid for PGMC_CPC1 {
    fn take() -> Option<Self> {
        <PGMC_CPC1>::take()
    }
    fn release(self) {
        <PGMC_CPC1>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_CPC1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_CPC1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_CPC1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_CPC1 {
    const INSTANCE: Self = Self {
        addr: 0x40c89400,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_CPC1
    pub const reset: ResetValues = ResetValues {
        CPC_AUTHEN_CTRL: 0x00000F00,
        CPC_CORE_MODE: 0x00000000,
        CPC_CORE_POWER_CTRL: 0x00000000,
        CPC_FLAG: 0x00000000,
        CPC_CACHE_MODE: 0x00000000,
        CPC_CACHE_CM_CTRL: 0x00003330,
        CPC_CACHE_SP_CTRL_0: 0x33333333,
        CPC_CACHE_SP_CTRL_1: 0x33333333,
        CPC_LMEM_MODE: 0x00000000,
        CPC_LMEM_CM_CTRL: 0x00000000,
        CPC_LMEM_SP_CTRL_0: 0x00000000,
        CPC_LMEM_SP_CTRL_1: 0x00000000,
    };

    /// Safe access to PGMC_CPC1
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
        let taken = PGMC_CPC1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_CPC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_CPC1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_CPC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_CPC1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_CPC1 {
    /// The interrupts associated with PGMC_CPC1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_CPC1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_CPC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_CPC1: *const RegisterBlock = 0x40c89400 as *const _;
