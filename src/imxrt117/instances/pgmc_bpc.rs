#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PGMC_BPC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::pgmc_bpc::Instance;
pub use crate::imxrt117::peripherals::pgmc_bpc::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::pgmc_bpc::{
    BPC_AUTHEN_CTRL, BPC_FLAG, BPC_MODE, BPC_POWER_CTRL, BPC_SSAR_RESTORE_CTRL, BPC_SSAR_SAVE_CTRL,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The PGMC_BPC0 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_BPC0 = Instance<1>;

/// The PGMC_BPC0 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_BPC0 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_BPC0 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_BPC0 {}
impl crate::Valid for PGMC_BPC0 {
    fn take() -> Option<Self> {
        <PGMC_BPC0>::take()
    }
    fn release(self) {
        <PGMC_BPC0>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_BPC0>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_BPC0_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_BPC0 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_BPC0 {
    const INSTANCE: Self = Self {
        addr: 0x40c88000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_BPC0
    pub const reset: ResetValues = ResetValues {
        BPC_AUTHEN_CTRL: 0x00000F00,
        BPC_MODE: 0x00000000,
        BPC_POWER_CTRL: 0x00000000,
        BPC_FLAG: 0x00000000,
        BPC_SSAR_SAVE_CTRL: 0x00000000,
        BPC_SSAR_RESTORE_CTRL: 0x00000000,
    };

    /// Safe access to PGMC_BPC0
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
        let taken = PGMC_BPC0_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_BPC0
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_BPC0_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_BPC0
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_BPC0_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_BPC0 {
    /// The interrupts associated with PGMC_BPC0
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_BPC0
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_BPC0
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_BPC0: *const RegisterBlock = 0x40c88000 as *const _;

/// The PGMC_BPC1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_BPC1 = Instance<2>;

/// The PGMC_BPC1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_BPC1 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_BPC1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_BPC1 {}
impl crate::Valid for PGMC_BPC1 {
    fn take() -> Option<Self> {
        <PGMC_BPC1>::take()
    }
    fn release(self) {
        <PGMC_BPC1>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_BPC1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_BPC1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_BPC1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_BPC1 {
    const INSTANCE: Self = Self {
        addr: 0x40c88200,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_BPC1
    pub const reset: ResetValues = ResetValues {
        BPC_AUTHEN_CTRL: 0x00000F00,
        BPC_MODE: 0x00000000,
        BPC_POWER_CTRL: 0x00000000,
        BPC_FLAG: 0x00000000,
        BPC_SSAR_SAVE_CTRL: 0x00000000,
        BPC_SSAR_RESTORE_CTRL: 0x00000000,
    };

    /// Safe access to PGMC_BPC1
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
        let taken = PGMC_BPC1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_BPC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_BPC1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_BPC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_BPC1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_BPC1 {
    /// The interrupts associated with PGMC_BPC1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_BPC1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_BPC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_BPC1: *const RegisterBlock = 0x40c88200 as *const _;

/// The PGMC_BPC2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_BPC2 = Instance<3>;

/// The PGMC_BPC2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_BPC2 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_BPC2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_BPC2 {}
impl crate::Valid for PGMC_BPC2 {
    fn take() -> Option<Self> {
        <PGMC_BPC2>::take()
    }
    fn release(self) {
        <PGMC_BPC2>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_BPC2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_BPC2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_BPC2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_BPC2 {
    const INSTANCE: Self = Self {
        addr: 0x40c88400,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_BPC2
    pub const reset: ResetValues = ResetValues {
        BPC_AUTHEN_CTRL: 0x00000F00,
        BPC_MODE: 0x00000000,
        BPC_POWER_CTRL: 0x00000000,
        BPC_FLAG: 0x00000000,
        BPC_SSAR_SAVE_CTRL: 0x00000000,
        BPC_SSAR_RESTORE_CTRL: 0x00000000,
    };

    /// Safe access to PGMC_BPC2
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
        let taken = PGMC_BPC2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_BPC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_BPC2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_BPC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_BPC2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_BPC2 {
    /// The interrupts associated with PGMC_BPC2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_BPC2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_BPC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_BPC2: *const RegisterBlock = 0x40c88400 as *const _;

/// The PGMC_BPC3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_BPC3 = Instance<4>;

/// The PGMC_BPC3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_BPC3 = Instance<4>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_BPC3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_BPC3 {}
impl crate::Valid for PGMC_BPC3 {
    fn take() -> Option<Self> {
        <PGMC_BPC3>::take()
    }
    fn release(self) {
        <PGMC_BPC3>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_BPC3>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_BPC3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_BPC3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_BPC3 {
    const INSTANCE: Self = Self {
        addr: 0x40c88600,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_BPC3
    pub const reset: ResetValues = ResetValues {
        BPC_AUTHEN_CTRL: 0x00000F00,
        BPC_MODE: 0x00000000,
        BPC_POWER_CTRL: 0x00000000,
        BPC_FLAG: 0x00000000,
        BPC_SSAR_SAVE_CTRL: 0x00000000,
        BPC_SSAR_RESTORE_CTRL: 0x00000000,
    };

    /// Safe access to PGMC_BPC3
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
        let taken = PGMC_BPC3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_BPC3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_BPC3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_BPC3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_BPC3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_BPC3 {
    /// The interrupts associated with PGMC_BPC3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_BPC3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_BPC3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_BPC3: *const RegisterBlock = 0x40c88600 as *const _;

/// The PGMC_BPC4 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_BPC4 = Instance<5>;

/// The PGMC_BPC4 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_BPC4 = Instance<5>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_BPC4 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_BPC4 {}
impl crate::Valid for PGMC_BPC4 {
    fn take() -> Option<Self> {
        <PGMC_BPC4>::take()
    }
    fn release(self) {
        <PGMC_BPC4>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_BPC4>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_BPC4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_BPC4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_BPC4 {
    const INSTANCE: Self = Self {
        addr: 0x40c88800,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_BPC4
    pub const reset: ResetValues = ResetValues {
        BPC_AUTHEN_CTRL: 0x00000F00,
        BPC_MODE: 0x00000000,
        BPC_POWER_CTRL: 0x00000000,
        BPC_FLAG: 0x00000000,
        BPC_SSAR_SAVE_CTRL: 0x00000000,
        BPC_SSAR_RESTORE_CTRL: 0x00000000,
    };

    /// Safe access to PGMC_BPC4
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
        let taken = PGMC_BPC4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_BPC4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_BPC4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_BPC4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_BPC4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_BPC4 {
    /// The interrupts associated with PGMC_BPC4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_BPC4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_BPC4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_BPC4: *const RegisterBlock = 0x40c88800 as *const _;

/// The PGMC_BPC5 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_BPC5 = Instance<6>;

/// The PGMC_BPC5 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_BPC5 = Instance<6>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_BPC5 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_BPC5 {}
impl crate::Valid for PGMC_BPC5 {
    fn take() -> Option<Self> {
        <PGMC_BPC5>::take()
    }
    fn release(self) {
        <PGMC_BPC5>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_BPC5>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_BPC5_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_BPC5 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_BPC5 {
    const INSTANCE: Self = Self {
        addr: 0x40c88a00,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_BPC5
    pub const reset: ResetValues = ResetValues {
        BPC_AUTHEN_CTRL: 0x00000F00,
        BPC_MODE: 0x00000000,
        BPC_POWER_CTRL: 0x00000000,
        BPC_FLAG: 0x00000000,
        BPC_SSAR_SAVE_CTRL: 0x00000000,
        BPC_SSAR_RESTORE_CTRL: 0x00000000,
    };

    /// Safe access to PGMC_BPC5
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
        let taken = PGMC_BPC5_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_BPC5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_BPC5_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_BPC5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_BPC5_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_BPC5 {
    /// The interrupts associated with PGMC_BPC5
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_BPC5
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_BPC5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_BPC5: *const RegisterBlock = 0x40c88a00 as *const _;

/// The PGMC_BPC6 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_BPC6 = Instance<7>;

/// The PGMC_BPC6 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_BPC6 = Instance<7>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_BPC6 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_BPC6 {}
impl crate::Valid for PGMC_BPC6 {
    fn take() -> Option<Self> {
        <PGMC_BPC6>::take()
    }
    fn release(self) {
        <PGMC_BPC6>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_BPC6>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_BPC6_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_BPC6 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_BPC6 {
    const INSTANCE: Self = Self {
        addr: 0x40c88c00,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_BPC6
    pub const reset: ResetValues = ResetValues {
        BPC_AUTHEN_CTRL: 0x00000F00,
        BPC_MODE: 0x00000000,
        BPC_POWER_CTRL: 0x00000000,
        BPC_FLAG: 0x00000000,
        BPC_SSAR_SAVE_CTRL: 0x00000000,
        BPC_SSAR_RESTORE_CTRL: 0x00000000,
    };

    /// Safe access to PGMC_BPC6
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
        let taken = PGMC_BPC6_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_BPC6
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_BPC6_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_BPC6
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_BPC6_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_BPC6 {
    /// The interrupts associated with PGMC_BPC6
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_BPC6
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_BPC6
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_BPC6: *const RegisterBlock = 0x40c88c00 as *const _;

/// The PGMC_BPC7 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PGMC_BPC7 = Instance<8>;

/// The PGMC_BPC7 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PGMC_BPC7 = Instance<8>;
/// ```
#[cfg(feature = "doc")]
pub struct PGMC_BPC7 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PGMC_BPC7 {}
impl crate::Valid for PGMC_BPC7 {
    fn take() -> Option<Self> {
        <PGMC_BPC7>::take()
    }
    fn release(self) {
        <PGMC_BPC7>::release(self);
    }
    unsafe fn steal() -> Self {
        <PGMC_BPC7>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PGMC_BPC7_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PGMC_BPC7 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PGMC_BPC7 {
    const INSTANCE: Self = Self {
        addr: 0x40c88e00,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in PGMC_BPC7
    pub const reset: ResetValues = ResetValues {
        BPC_AUTHEN_CTRL: 0x00000F00,
        BPC_MODE: 0x00000000,
        BPC_POWER_CTRL: 0x00000000,
        BPC_FLAG: 0x00000000,
        BPC_SSAR_SAVE_CTRL: 0x00000000,
        BPC_SSAR_RESTORE_CTRL: 0x00000000,
    };

    /// Safe access to PGMC_BPC7
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
        let taken = PGMC_BPC7_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PGMC_BPC7
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PGMC_BPC7_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PGMC_BPC7
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PGMC_BPC7_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PGMC_BPC7 {
    /// The interrupts associated with PGMC_BPC7
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with PGMC_BPC7
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PGMC_BPC7
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGMC_BPC7: *const RegisterBlock = 0x40c88e00 as *const _;
