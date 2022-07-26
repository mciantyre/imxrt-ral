#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SAI
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::sai::Instance;
pub use crate::imxrt117::peripherals::sai::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::sai::{
    PARAM, RCR1, RCR2, RCR3, RCR4, RCR5, RCSR, RDR0, RFR0, RMR, TCR1, TCR2, TCR3, TCR4, TCR5, TCSR,
    TDR0, TFR0, TMR, VERID,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The SAI2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type SAI2 = Instance<2>;

/// The SAI2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type SAI2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct SAI2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for SAI2 {}
impl crate::Valid for SAI2 {
    fn take() -> Option<Self> {
        <SAI2>::take()
    }
    fn release(self) {
        <SAI2>::release(self);
    }
    unsafe fn steal() -> Self {
        <SAI2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static SAI2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the SAI2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl SAI2 {
    const INSTANCE: Self = Self {
        addr: 0x40408000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::SAI2],
    };

    /// Reset values for each field in SAI2
    pub const reset: ResetValues = ResetValues {
        VERID: 0x03010000,
        PARAM: 0x00050501,
        TCSR: 0x00000000,
        TCR1: 0x00000000,
        TCR2: 0x00000000,
        TCR3: 0x00000000,
        TCR4: 0x00000000,
        TCR5: 0x00000000,
        TDR0: 0x00000000,
        TFR0: 0x00000000,
        TMR: 0x00000000,
        RCSR: 0x00000000,
        RCR1: 0x00000000,
        RCR2: 0x00000000,
        RCR3: 0x00000000,
        RCR4: 0x00000000,
        RCR5: 0x00000000,
        RDR0: 0x00000000,
        RFR0: 0x00000000,
        RMR: 0x00000000,
    };

    /// Safe access to SAI2
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
        let taken = SAI2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to SAI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = SAI2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal SAI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        SAI2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl SAI2 {
    /// The interrupts associated with SAI2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::SAI2];

    /// The interrupts associated with SAI2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to SAI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI2: *const RegisterBlock = 0x40408000 as *const _;

/// The SAI3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type SAI3 = Instance<3>;

/// The SAI3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type SAI3 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct SAI3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for SAI3 {}
impl crate::Valid for SAI3 {
    fn take() -> Option<Self> {
        <SAI3>::take()
    }
    fn release(self) {
        <SAI3>::release(self);
    }
    unsafe fn steal() -> Self {
        <SAI3>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static SAI3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the SAI3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl SAI3 {
    const INSTANCE: Self = Self {
        addr: 0x4040c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::SAI3_RX, crate::interrupt::SAI3_TX],
    };

    /// Reset values for each field in SAI3
    pub const reset: ResetValues = ResetValues {
        VERID: 0x03010000,
        PARAM: 0x00050501,
        TCSR: 0x00000000,
        TCR1: 0x00000000,
        TCR2: 0x00000000,
        TCR3: 0x00000000,
        TCR4: 0x00000000,
        TCR5: 0x00000000,
        TDR0: 0x00000000,
        TFR0: 0x00000000,
        TMR: 0x00000000,
        RCSR: 0x00000000,
        RCR1: 0x00000000,
        RCR2: 0x00000000,
        RCR3: 0x00000000,
        RCR4: 0x00000000,
        RCR5: 0x00000000,
        RDR0: 0x00000000,
        RFR0: 0x00000000,
        RMR: 0x00000000,
    };

    /// Safe access to SAI3
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
        let taken = SAI3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to SAI3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = SAI3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal SAI3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        SAI3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl SAI3 {
    /// The interrupts associated with SAI3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] =
        [crate::interrupt::SAI3_RX, crate::interrupt::SAI3_TX];

    /// The interrupts associated with SAI3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to SAI3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI3: *const RegisterBlock = 0x4040c000 as *const _;

/// The SAI4 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type SAI4 = Instance<4>;

/// The SAI4 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type SAI4 = Instance<4>;
/// ```
#[cfg(feature = "doc")]
pub struct SAI4 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for SAI4 {}
impl crate::Valid for SAI4 {
    fn take() -> Option<Self> {
        <SAI4>::take()
    }
    fn release(self) {
        <SAI4>::release(self);
    }
    unsafe fn steal() -> Self {
        <SAI4>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static SAI4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the SAI4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl SAI4 {
    const INSTANCE: Self = Self {
        addr: 0x40c40000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::SAI4_RX, crate::interrupt::SAI4_TX],
    };

    /// Reset values for each field in SAI4
    pub const reset: ResetValues = ResetValues {
        VERID: 0x03010000,
        PARAM: 0x00050501,
        TCSR: 0x00000000,
        TCR1: 0x00000000,
        TCR2: 0x00000000,
        TCR3: 0x00000000,
        TCR4: 0x00000000,
        TCR5: 0x00000000,
        TDR0: 0x00000000,
        TFR0: 0x00000000,
        TMR: 0x00000000,
        RCSR: 0x00000000,
        RCR1: 0x00000000,
        RCR2: 0x00000000,
        RCR3: 0x00000000,
        RCR4: 0x00000000,
        RCR5: 0x00000000,
        RDR0: 0x00000000,
        RFR0: 0x00000000,
        RMR: 0x00000000,
    };

    /// Safe access to SAI4
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
        let taken = SAI4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to SAI4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = SAI4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal SAI4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        SAI4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl SAI4 {
    /// The interrupts associated with SAI4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] =
        [crate::interrupt::SAI4_RX, crate::interrupt::SAI4_TX];

    /// The interrupts associated with SAI4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to SAI4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI4: *const RegisterBlock = 0x40c40000 as *const _;
