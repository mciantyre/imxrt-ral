#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PGMC_MIF
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::pgmc_cpc0_mif::Instance;
pub use crate::imxrt117::peripherals::pgmc_cpc0_mif::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::pgmc_cpc0_mif::{
    MIF_AUTHEN_CTRL, MIF_MLPL_ARR_PDN, MIF_MLPL_HS, MIF_MLPL_IG, MIF_MLPL_INITN, MIF_MLPL_ISO,
    MIF_MLPL_LS, MIF_MLPL_PER_PDN, MIF_MLPL_SLEEP, MIF_MLPL_STDBY,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The PGMC_CPC0_MIF0 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_CPC0_MIF0 = Instance<1>;

/// The PGMC_CPC0_MIF0 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_CPC0_MIF0 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_CPC0_MIF0 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_CPC0_MIF0 {}
impl crate::Valid for PGMC_CPC0_MIF0 {
    fn take() -> Option<Self> {
        <PGMC_CPC0_MIF0>::take()
    }
    fn release(self) {
        <PGMC_CPC0_MIF0>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_CPC0_MIF0>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_CPC0_MIF0_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_CPC0_MIF0 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_CPC0_MIF0 {
    const INSTANCE: Self = Self {
        addr: 0x40c89100,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_CPC0_MIF0
    pub const reset: ResetValues = ResetValues {
        MIF_AUTHEN_CTRL: 0x00000000,
        MIF_MLPL_SLEEP: 0x0000FF00,
        MIF_MLPL_IG: 0x00000060,
        MIF_MLPL_LS: 0x00000010,
        MIF_MLPL_HS: 0x00000002,
        MIF_MLPL_STDBY: 0x00000040,
        MIF_MLPL_ARR_PDN: 0x00000C00,
        MIF_MLPL_PER_PDN: 0x00004A00,
        MIF_MLPL_INITN: 0x000035FF,
        MIF_MLPL_ISO: 0x0000CA00,
    };

    /// Safe access to PGMC_CPC0_MIF0
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
        let taken = PGMC_CPC0_MIF0_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_CPC0_MIF0
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_CPC0_MIF0_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_CPC0_MIF0
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_CPC0_MIF0_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_CPC0_MIF0 {
    /// The interrupts associated with PGMC_CPC0_MIF0
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_CPC0_MIF0
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_CPC0_MIF0
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_CPC0_MIF0: *const RegisterBlock = 0x40c89100 as *const _;

/// The PGMC_CPC0_MIF1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_CPC0_MIF1 = Instance<2>;

/// The PGMC_CPC0_MIF1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_CPC0_MIF1 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_CPC0_MIF1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_CPC0_MIF1 {}
impl crate::Valid for PGMC_CPC0_MIF1 {
    fn take() -> Option<Self> {
        <PGMC_CPC0_MIF1>::take()
    }
    fn release(self) {
        <PGMC_CPC0_MIF1>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_CPC0_MIF1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_CPC0_MIF1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_CPC0_MIF1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_CPC0_MIF1 {
    const INSTANCE: Self = Self {
        addr: 0x40c89200,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_CPC0_MIF1
    pub const reset: ResetValues = ResetValues {
        MIF_AUTHEN_CTRL: 0x00000000,
        MIF_MLPL_SLEEP: 0x0000FF00,
        MIF_MLPL_IG: 0x00000060,
        MIF_MLPL_LS: 0x00000010,
        MIF_MLPL_HS: 0x00000002,
        MIF_MLPL_STDBY: 0x00000040,
        MIF_MLPL_ARR_PDN: 0x00000C00,
        MIF_MLPL_PER_PDN: 0x00004A00,
        MIF_MLPL_INITN: 0x000035FF,
        MIF_MLPL_ISO: 0x0000CA00,
    };

    /// Safe access to PGMC_CPC0_MIF1
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
        let taken = PGMC_CPC0_MIF1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_CPC0_MIF1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_CPC0_MIF1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_CPC0_MIF1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_CPC0_MIF1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_CPC0_MIF1 {
    /// The interrupts associated with PGMC_CPC0_MIF1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_CPC0_MIF1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_CPC0_MIF1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_CPC0_MIF1: *const RegisterBlock = 0x40c89200 as *const _;

/// The PGMC_CPC1_MIF0 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_CPC1_MIF0 = Instance<3>;

/// The PGMC_CPC1_MIF0 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_CPC1_MIF0 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_CPC1_MIF0 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_CPC1_MIF0 {}
impl crate::Valid for PGMC_CPC1_MIF0 {
    fn take() -> Option<Self> {
        <PGMC_CPC1_MIF0>::take()
    }
    fn release(self) {
        <PGMC_CPC1_MIF0>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_CPC1_MIF0>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_CPC1_MIF0_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_CPC1_MIF0 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_CPC1_MIF0 {
    const INSTANCE: Self = Self {
        addr: 0x40c89500,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_CPC1_MIF0
    pub const reset: ResetValues = ResetValues {
        MIF_AUTHEN_CTRL: 0x00000000,
        MIF_MLPL_SLEEP: 0x0000FF00,
        MIF_MLPL_IG: 0x00000060,
        MIF_MLPL_LS: 0x00000010,
        MIF_MLPL_HS: 0x00000002,
        MIF_MLPL_STDBY: 0x00000040,
        MIF_MLPL_ARR_PDN: 0x00000C00,
        MIF_MLPL_PER_PDN: 0x00004A00,
        MIF_MLPL_INITN: 0x000035FF,
        MIF_MLPL_ISO: 0x0000CA00,
    };

    /// Safe access to PGMC_CPC1_MIF0
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
        let taken = PGMC_CPC1_MIF0_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_CPC1_MIF0
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_CPC1_MIF0_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_CPC1_MIF0
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_CPC1_MIF0_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_CPC1_MIF0 {
    /// The interrupts associated with PGMC_CPC1_MIF0
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_CPC1_MIF0
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_CPC1_MIF0
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_CPC1_MIF0: *const RegisterBlock = 0x40c89500 as *const _;

/// The PGMC_CPC1_MIF1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_CPC1_MIF1 = Instance<4>;

/// The PGMC_CPC1_MIF1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_CPC1_MIF1 = Instance<4>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_CPC1_MIF1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_CPC1_MIF1 {}
impl crate::Valid for PGMC_CPC1_MIF1 {
    fn take() -> Option<Self> {
        <PGMC_CPC1_MIF1>::take()
    }
    fn release(self) {
        <PGMC_CPC1_MIF1>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_CPC1_MIF1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_CPC1_MIF1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_CPC1_MIF1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_CPC1_MIF1 {
    const INSTANCE: Self = Self {
        addr: 0x40c89600,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_CPC1_MIF1
    pub const reset: ResetValues = ResetValues {
        MIF_AUTHEN_CTRL: 0x00000000,
        MIF_MLPL_SLEEP: 0x0000FF00,
        MIF_MLPL_IG: 0x00000060,
        MIF_MLPL_LS: 0x00000010,
        MIF_MLPL_HS: 0x00000002,
        MIF_MLPL_STDBY: 0x00000040,
        MIF_MLPL_ARR_PDN: 0x00000C00,
        MIF_MLPL_PER_PDN: 0x00004A00,
        MIF_MLPL_INITN: 0x000035FF,
        MIF_MLPL_ISO: 0x0000CA00,
    };

    /// Safe access to PGMC_CPC1_MIF1
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
        let taken = PGMC_CPC1_MIF1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_CPC1_MIF1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_CPC1_MIF1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_CPC1_MIF1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_CPC1_MIF1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_CPC1_MIF1 {
    /// The interrupts associated with PGMC_CPC1_MIF1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_CPC1_MIF1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_CPC1_MIF1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_CPC1_MIF1: *const RegisterBlock = 0x40c89600 as *const _;
