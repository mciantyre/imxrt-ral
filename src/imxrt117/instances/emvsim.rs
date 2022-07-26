#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! EMVSIM
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::emvsim::Instance;
pub use crate::imxrt117::peripherals::emvsim::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::emvsim::{
    BGT_VAL, BWT_VAL, CLKCFG, CTRL, CWT_VAL, DIVISOR, GPCNT0_VAL, GPCNT1_VAL, INT_MASK, PARAM,
    PCSR, RX_BUF, RX_STATUS, RX_THD, TX_BUF, TX_GETU, TX_STATUS, TX_THD, VER_ID,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The EMVSIM1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type EMVSIM1 = Instance<1>;

/// The EMVSIM1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type EMVSIM1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct EMVSIM1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for EMVSIM1 {}
impl crate::Valid for EMVSIM1 {
    fn take() -> Option<Self> {
        <EMVSIM1>::take()
    }
    fn release(self) {
        <EMVSIM1>::release(self);
    }
    unsafe fn steal() -> Self {
        <EMVSIM1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static EMVSIM1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the EMVSIM1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl EMVSIM1 {
    const INSTANCE: Self = Self {
        addr: 0x40154000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::EMVSIM1],
    };

    /// Reset values for each field in EMVSIM1
    pub const reset: ResetValues = ResetValues {
        VER_ID: 0x00000000,
        PARAM: 0x00001010,
        CLKCFG: 0x00000000,
        DIVISOR: 0x00000174,
        CTRL: 0x01000006,
        INT_MASK: 0x0000FFFF,
        RX_THD: 0x00000001,
        TX_THD: 0x0000000F,
        RX_STATUS: 0x00000000,
        TX_STATUS: 0x000000B8,
        PCSR: 0x01000000,
        RX_BUF: 0x00000000,
        TX_BUF: 0x00000000,
        TX_GETU: 0x00000000,
        CWT_VAL: 0x0000FFFF,
        BWT_VAL: 0xFFFFFFFF,
        BGT_VAL: 0x00000000,
        GPCNT0_VAL: 0x0000FFFF,
        GPCNT1_VAL: 0x0000FFFF,
    };

    /// Safe access to EMVSIM1
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
        let taken = EMVSIM1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to EMVSIM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = EMVSIM1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal EMVSIM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        EMVSIM1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl EMVSIM1 {
    /// The interrupts associated with EMVSIM1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::EMVSIM1];

    /// The interrupts associated with EMVSIM1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to EMVSIM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const EMVSIM1: *const RegisterBlock = 0x40154000 as *const _;

/// The EMVSIM2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type EMVSIM2 = Instance<2>;

/// The EMVSIM2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type EMVSIM2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct EMVSIM2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for EMVSIM2 {}
impl crate::Valid for EMVSIM2 {
    fn take() -> Option<Self> {
        <EMVSIM2>::take()
    }
    fn release(self) {
        <EMVSIM2>::release(self);
    }
    unsafe fn steal() -> Self {
        <EMVSIM2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static EMVSIM2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the EMVSIM2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl EMVSIM2 {
    const INSTANCE: Self = Self {
        addr: 0x40158000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::EMVSIM2],
    };

    /// Reset values for each field in EMVSIM2
    pub const reset: ResetValues = ResetValues {
        VER_ID: 0x00000000,
        PARAM: 0x00001010,
        CLKCFG: 0x00000000,
        DIVISOR: 0x00000174,
        CTRL: 0x01000006,
        INT_MASK: 0x0000FFFF,
        RX_THD: 0x00000001,
        TX_THD: 0x0000000F,
        RX_STATUS: 0x00000000,
        TX_STATUS: 0x000000B8,
        PCSR: 0x01000000,
        RX_BUF: 0x00000000,
        TX_BUF: 0x00000000,
        TX_GETU: 0x00000000,
        CWT_VAL: 0x0000FFFF,
        BWT_VAL: 0xFFFFFFFF,
        BGT_VAL: 0x00000000,
        GPCNT0_VAL: 0x0000FFFF,
        GPCNT1_VAL: 0x0000FFFF,
    };

    /// Safe access to EMVSIM2
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
        let taken = EMVSIM2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to EMVSIM2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = EMVSIM2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal EMVSIM2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        EMVSIM2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl EMVSIM2 {
    /// The interrupts associated with EMVSIM2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::EMVSIM2];

    /// The interrupts associated with EMVSIM2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to EMVSIM2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const EMVSIM2: *const RegisterBlock = 0x40158000 as *const _;
