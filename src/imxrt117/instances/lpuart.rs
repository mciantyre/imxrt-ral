#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPUART
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::lpuart::Instance;
pub use crate::imxrt117::peripherals::lpuart::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::lpuart::{
    BAUD, CTRL, DATA, FIFO, GLOBAL, MATCH, MODIR, PARAM, PINCFG, STAT, VERID, WATER,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The LPUART1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPUART1 = Instance<1>;

/// The LPUART1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPUART1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct LPUART1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPUART1 {}
impl crate::Valid for LPUART1 {
    fn take() -> Option<Self> {
        <LPUART1>::take()
    }
    fn release(self) {
        <LPUART1>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPUART1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART1 {
    const INSTANCE: Self = Self {
        addr: 0x4007c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART1],
    };

    /// Reset values for each field in LPUART1
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART1
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
        let taken = LPUART1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPUART1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPUART1 {
    /// The interrupts associated with LPUART1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART1];

    /// The interrupts associated with LPUART1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART1: *const RegisterBlock = 0x4007c000 as *const _;

/// The LPUART10 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPUART10 = Instance<10>;

/// The LPUART10 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPUART10 = Instance<10>;
/// ```
#[cfg(feature = "doc")]
pub struct LPUART10 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPUART10 {}
impl crate::Valid for LPUART10 {
    fn take() -> Option<Self> {
        <LPUART10>::take()
    }
    fn release(self) {
        <LPUART10>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPUART10>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART10_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART10 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART10 {
    const INSTANCE: Self = Self {
        addr: 0x400a0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART10],
    };

    /// Reset values for each field in LPUART10
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART10
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
        let taken = LPUART10_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART10
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPUART10_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART10
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART10_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPUART10 {
    /// The interrupts associated with LPUART10
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART10];

    /// The interrupts associated with LPUART10
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART10
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART10: *const RegisterBlock = 0x400a0000 as *const _;

/// The LPUART11 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPUART11 = Instance<11>;

/// The LPUART11 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPUART11 = Instance<11>;
/// ```
#[cfg(feature = "doc")]
pub struct LPUART11 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPUART11 {}
impl crate::Valid for LPUART11 {
    fn take() -> Option<Self> {
        <LPUART11>::take()
    }
    fn release(self) {
        <LPUART11>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPUART11>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART11_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART11 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART11 {
    const INSTANCE: Self = Self {
        addr: 0x40c24000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART11],
    };

    /// Reset values for each field in LPUART11
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART11
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
        let taken = LPUART11_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART11
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPUART11_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART11
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART11_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPUART11 {
    /// The interrupts associated with LPUART11
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART11];

    /// The interrupts associated with LPUART11
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART11
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART11: *const RegisterBlock = 0x40c24000 as *const _;

/// The LPUART12 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPUART12 = Instance<12>;

/// The LPUART12 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPUART12 = Instance<12>;
/// ```
#[cfg(feature = "doc")]
pub struct LPUART12 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPUART12 {}
impl crate::Valid for LPUART12 {
    fn take() -> Option<Self> {
        <LPUART12>::take()
    }
    fn release(self) {
        <LPUART12>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPUART12>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART12_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART12 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART12 {
    const INSTANCE: Self = Self {
        addr: 0x40c28000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART12],
    };

    /// Reset values for each field in LPUART12
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART12
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
        let taken = LPUART12_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART12
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPUART12_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART12
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART12_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPUART12 {
    /// The interrupts associated with LPUART12
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART12];

    /// The interrupts associated with LPUART12
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART12
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART12: *const RegisterBlock = 0x40c28000 as *const _;

/// The LPUART2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPUART2 = Instance<2>;

/// The LPUART2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPUART2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct LPUART2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPUART2 {}
impl crate::Valid for LPUART2 {
    fn take() -> Option<Self> {
        <LPUART2>::take()
    }
    fn release(self) {
        <LPUART2>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPUART2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART2 {
    const INSTANCE: Self = Self {
        addr: 0x40080000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART2],
    };

    /// Reset values for each field in LPUART2
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART2
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
        let taken = LPUART2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPUART2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPUART2 {
    /// The interrupts associated with LPUART2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART2];

    /// The interrupts associated with LPUART2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART2: *const RegisterBlock = 0x40080000 as *const _;

/// The LPUART3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPUART3 = Instance<3>;

/// The LPUART3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPUART3 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct LPUART3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPUART3 {}
impl crate::Valid for LPUART3 {
    fn take() -> Option<Self> {
        <LPUART3>::take()
    }
    fn release(self) {
        <LPUART3>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPUART3>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART3 {
    const INSTANCE: Self = Self {
        addr: 0x40084000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART3],
    };

    /// Reset values for each field in LPUART3
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART3
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
        let taken = LPUART3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPUART3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPUART3 {
    /// The interrupts associated with LPUART3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART3];

    /// The interrupts associated with LPUART3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART3: *const RegisterBlock = 0x40084000 as *const _;

/// The LPUART4 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPUART4 = Instance<4>;

/// The LPUART4 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPUART4 = Instance<4>;
/// ```
#[cfg(feature = "doc")]
pub struct LPUART4 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPUART4 {}
impl crate::Valid for LPUART4 {
    fn take() -> Option<Self> {
        <LPUART4>::take()
    }
    fn release(self) {
        <LPUART4>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPUART4>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART4 {
    const INSTANCE: Self = Self {
        addr: 0x40088000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART4],
    };

    /// Reset values for each field in LPUART4
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART4
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
        let taken = LPUART4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPUART4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPUART4 {
    /// The interrupts associated with LPUART4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART4];

    /// The interrupts associated with LPUART4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART4: *const RegisterBlock = 0x40088000 as *const _;

/// The LPUART5 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPUART5 = Instance<5>;

/// The LPUART5 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPUART5 = Instance<5>;
/// ```
#[cfg(feature = "doc")]
pub struct LPUART5 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPUART5 {}
impl crate::Valid for LPUART5 {
    fn take() -> Option<Self> {
        <LPUART5>::take()
    }
    fn release(self) {
        <LPUART5>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPUART5>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART5_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART5 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART5 {
    const INSTANCE: Self = Self {
        addr: 0x4008c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART5],
    };

    /// Reset values for each field in LPUART5
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART5
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
        let taken = LPUART5_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPUART5_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART5_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPUART5 {
    /// The interrupts associated with LPUART5
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART5];

    /// The interrupts associated with LPUART5
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART5: *const RegisterBlock = 0x4008c000 as *const _;

/// The LPUART6 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPUART6 = Instance<6>;

/// The LPUART6 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPUART6 = Instance<6>;
/// ```
#[cfg(feature = "doc")]
pub struct LPUART6 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPUART6 {}
impl crate::Valid for LPUART6 {
    fn take() -> Option<Self> {
        <LPUART6>::take()
    }
    fn release(self) {
        <LPUART6>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPUART6>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART6_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART6 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART6 {
    const INSTANCE: Self = Self {
        addr: 0x40090000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART6],
    };

    /// Reset values for each field in LPUART6
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART6
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
        let taken = LPUART6_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART6
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPUART6_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART6
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART6_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPUART6 {
    /// The interrupts associated with LPUART6
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART6];

    /// The interrupts associated with LPUART6
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART6
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART6: *const RegisterBlock = 0x40090000 as *const _;

/// The LPUART7 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPUART7 = Instance<7>;

/// The LPUART7 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPUART7 = Instance<7>;
/// ```
#[cfg(feature = "doc")]
pub struct LPUART7 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPUART7 {}
impl crate::Valid for LPUART7 {
    fn take() -> Option<Self> {
        <LPUART7>::take()
    }
    fn release(self) {
        <LPUART7>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPUART7>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART7_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART7 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART7 {
    const INSTANCE: Self = Self {
        addr: 0x40094000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART7],
    };

    /// Reset values for each field in LPUART7
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART7
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
        let taken = LPUART7_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART7
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPUART7_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART7
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART7_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPUART7 {
    /// The interrupts associated with LPUART7
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART7];

    /// The interrupts associated with LPUART7
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART7
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART7: *const RegisterBlock = 0x40094000 as *const _;

/// The LPUART8 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPUART8 = Instance<8>;

/// The LPUART8 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPUART8 = Instance<8>;
/// ```
#[cfg(feature = "doc")]
pub struct LPUART8 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPUART8 {}
impl crate::Valid for LPUART8 {
    fn take() -> Option<Self> {
        <LPUART8>::take()
    }
    fn release(self) {
        <LPUART8>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPUART8>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART8_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART8 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART8 {
    const INSTANCE: Self = Self {
        addr: 0x40098000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART8],
    };

    /// Reset values for each field in LPUART8
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART8
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
        let taken = LPUART8_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART8
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPUART8_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART8
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART8_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPUART8 {
    /// The interrupts associated with LPUART8
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART8];

    /// The interrupts associated with LPUART8
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART8
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART8: *const RegisterBlock = 0x40098000 as *const _;

/// The LPUART9 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LPUART9 = Instance<9>;

/// The LPUART9 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LPUART9 = Instance<9>;
/// ```
#[cfg(feature = "doc")]
pub struct LPUART9 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LPUART9 {}
impl crate::Valid for LPUART9 {
    fn take() -> Option<Self> {
        <LPUART9>::take()
    }
    fn release(self) {
        <LPUART9>::release(self);
    }
    unsafe fn steal() -> Self {
        <LPUART9>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LPUART9_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LPUART9 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LPUART9 {
    const INSTANCE: Self = Self {
        addr: 0x4009c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LPUART9],
    };

    /// Reset values for each field in LPUART9
    pub const reset: ResetValues = ResetValues {
        VERID: 0x04010003,
        PARAM: 0x00000202,
        GLOBAL: 0x00000000,
        PINCFG: 0x00000000,
        BAUD: 0x0F000004,
        STAT: 0x00C00000,
        CTRL: 0x00000000,
        DATA: 0x00001000,
        MATCH: 0x00000000,
        MODIR: 0x00000000,
        FIFO: 0x00C00011,
        WATER: 0x00000000,
    };

    /// Safe access to LPUART9
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
        let taken = LPUART9_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LPUART9
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LPUART9_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPUART9
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LPUART9_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LPUART9 {
    /// The interrupts associated with LPUART9
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LPUART9];

    /// The interrupts associated with LPUART9
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LPUART9
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART9: *const RegisterBlock = 0x4009c000 as *const _;
