#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AIPSTZ Control Registers
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

pub use crate::imxrt106::peripherals::aipstz::Instance;
pub use crate::imxrt106::peripherals::aipstz::{RegisterBlock, ResetValues};

pub use crate::imxrt106::peripherals::aipstz::{MPR, OPACR, OPACR1, OPACR2, OPACR3, OPACR4};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The AIPSTZ1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type AIPSTZ1 = Instance<1>;

/// The AIPSTZ1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type AIPSTZ1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct AIPSTZ1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for AIPSTZ1 {}
impl crate::Valid for AIPSTZ1 {
    fn take() -> Option<Self> {
        <AIPSTZ1>::take()
    }
    fn release(self) {
        <AIPSTZ1>::release(self);
    }
    unsafe fn steal() -> Self {
        <AIPSTZ1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static AIPSTZ1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the AIPSTZ1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl AIPSTZ1 {
    const INSTANCE: Self = Self {
        addr: 0x4007c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in AIPSTZ1
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    /// Safe access to AIPSTZ1
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
        let taken = AIPSTZ1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to AIPSTZ1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = AIPSTZ1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal AIPSTZ1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        AIPSTZ1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl AIPSTZ1 {
    /// The interrupts associated with AIPSTZ1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with AIPSTZ1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to AIPSTZ1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ1: *const RegisterBlock = 0x4007c000 as *const _;

/// The AIPSTZ2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type AIPSTZ2 = Instance<2>;

/// The AIPSTZ2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type AIPSTZ2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct AIPSTZ2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for AIPSTZ2 {}
impl crate::Valid for AIPSTZ2 {
    fn take() -> Option<Self> {
        <AIPSTZ2>::take()
    }
    fn release(self) {
        <AIPSTZ2>::release(self);
    }
    unsafe fn steal() -> Self {
        <AIPSTZ2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static AIPSTZ2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the AIPSTZ2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl AIPSTZ2 {
    const INSTANCE: Self = Self {
        addr: 0x4017c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in AIPSTZ2
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    /// Safe access to AIPSTZ2
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
        let taken = AIPSTZ2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to AIPSTZ2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = AIPSTZ2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal AIPSTZ2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        AIPSTZ2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl AIPSTZ2 {
    /// The interrupts associated with AIPSTZ2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with AIPSTZ2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to AIPSTZ2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ2: *const RegisterBlock = 0x4017c000 as *const _;

/// The AIPSTZ3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type AIPSTZ3 = Instance<3>;

/// The AIPSTZ3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type AIPSTZ3 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct AIPSTZ3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for AIPSTZ3 {}
impl crate::Valid for AIPSTZ3 {
    fn take() -> Option<Self> {
        <AIPSTZ3>::take()
    }
    fn release(self) {
        <AIPSTZ3>::release(self);
    }
    unsafe fn steal() -> Self {
        <AIPSTZ3>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static AIPSTZ3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the AIPSTZ3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl AIPSTZ3 {
    const INSTANCE: Self = Self {
        addr: 0x4027c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in AIPSTZ3
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    /// Safe access to AIPSTZ3
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
        let taken = AIPSTZ3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to AIPSTZ3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = AIPSTZ3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal AIPSTZ3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        AIPSTZ3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl AIPSTZ3 {
    /// The interrupts associated with AIPSTZ3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with AIPSTZ3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to AIPSTZ3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ3: *const RegisterBlock = 0x4027c000 as *const _;

/// The AIPSTZ4 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type AIPSTZ4 = Instance<4>;

/// The AIPSTZ4 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type AIPSTZ4 = Instance<4>;
/// ```
#[cfg(feature = "doc")]
pub struct AIPSTZ4 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for AIPSTZ4 {}
impl crate::Valid for AIPSTZ4 {
    fn take() -> Option<Self> {
        <AIPSTZ4>::take()
    }
    fn release(self) {
        <AIPSTZ4>::release(self);
    }
    unsafe fn steal() -> Self {
        <AIPSTZ4>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static AIPSTZ4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the AIPSTZ4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl AIPSTZ4 {
    const INSTANCE: Self = Self {
        addr: 0x4037c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in AIPSTZ4
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    /// Safe access to AIPSTZ4
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
        let taken = AIPSTZ4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to AIPSTZ4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = AIPSTZ4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal AIPSTZ4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        AIPSTZ4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl AIPSTZ4 {
    /// The interrupts associated with AIPSTZ4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with AIPSTZ4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to AIPSTZ4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ4: *const RegisterBlock = 0x4037c000 as *const _;
