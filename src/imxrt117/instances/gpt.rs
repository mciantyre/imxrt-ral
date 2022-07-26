#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPT
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::gpt::Instance;
pub use crate::imxrt117::peripherals::gpt::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::gpt::{CNT, CR, ICR1, ICR2, IR, OCR1, OCR2, OCR3, PR, SR};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The GPT1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPT1 = Instance<1>;

/// The GPT1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPT1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct GPT1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPT1 {}
impl crate::Valid for GPT1 {
    fn take() -> Option<Self> {
        <GPT1>::take()
    }
    fn release(self) {
        <GPT1>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPT1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPT1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPT1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPT1 {
    const INSTANCE: Self = Self {
        addr: 0x400ec000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPT1],
    };

    /// Reset values for each field in GPT1
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        PR: 0x00000000,
        SR: 0x00000000,
        IR: 0x00000000,
        OCR1: 0xFFFFFFFF,
        OCR2: 0xFFFFFFFF,
        OCR3: 0xFFFFFFFF,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        CNT: 0x00000000,
    };

    /// Safe access to GPT1
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
        let taken = GPT1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPT1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPT1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPT1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPT1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPT1 {
    /// The interrupts associated with GPT1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPT1];

    /// The interrupts associated with GPT1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPT1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPT1: *const RegisterBlock = 0x400ec000 as *const _;

/// The GPT2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPT2 = Instance<2>;

/// The GPT2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPT2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct GPT2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPT2 {}
impl crate::Valid for GPT2 {
    fn take() -> Option<Self> {
        <GPT2>::take()
    }
    fn release(self) {
        <GPT2>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPT2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPT2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPT2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPT2 {
    const INSTANCE: Self = Self {
        addr: 0x400f0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPT2],
    };

    /// Reset values for each field in GPT2
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        PR: 0x00000000,
        SR: 0x00000000,
        IR: 0x00000000,
        OCR1: 0xFFFFFFFF,
        OCR2: 0xFFFFFFFF,
        OCR3: 0xFFFFFFFF,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        CNT: 0x00000000,
    };

    /// Safe access to GPT2
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
        let taken = GPT2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPT2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPT2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPT2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPT2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPT2 {
    /// The interrupts associated with GPT2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPT2];

    /// The interrupts associated with GPT2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPT2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPT2: *const RegisterBlock = 0x400f0000 as *const _;

/// The GPT3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPT3 = Instance<3>;

/// The GPT3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPT3 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct GPT3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPT3 {}
impl crate::Valid for GPT3 {
    fn take() -> Option<Self> {
        <GPT3>::take()
    }
    fn release(self) {
        <GPT3>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPT3>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPT3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPT3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPT3 {
    const INSTANCE: Self = Self {
        addr: 0x400f4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPT3],
    };

    /// Reset values for each field in GPT3
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        PR: 0x00000000,
        SR: 0x00000000,
        IR: 0x00000000,
        OCR1: 0xFFFFFFFF,
        OCR2: 0xFFFFFFFF,
        OCR3: 0xFFFFFFFF,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        CNT: 0x00000000,
    };

    /// Safe access to GPT3
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
        let taken = GPT3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPT3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPT3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPT3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPT3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPT3 {
    /// The interrupts associated with GPT3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPT3];

    /// The interrupts associated with GPT3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPT3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPT3: *const RegisterBlock = 0x400f4000 as *const _;

/// The GPT4 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPT4 = Instance<4>;

/// The GPT4 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPT4 = Instance<4>;
/// ```
#[cfg(feature = "doc")]
pub struct GPT4 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPT4 {}
impl crate::Valid for GPT4 {
    fn take() -> Option<Self> {
        <GPT4>::take()
    }
    fn release(self) {
        <GPT4>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPT4>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPT4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPT4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPT4 {
    const INSTANCE: Self = Self {
        addr: 0x400f8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPT4],
    };

    /// Reset values for each field in GPT4
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        PR: 0x00000000,
        SR: 0x00000000,
        IR: 0x00000000,
        OCR1: 0xFFFFFFFF,
        OCR2: 0xFFFFFFFF,
        OCR3: 0xFFFFFFFF,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        CNT: 0x00000000,
    };

    /// Safe access to GPT4
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
        let taken = GPT4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPT4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPT4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPT4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPT4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPT4 {
    /// The interrupts associated with GPT4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPT4];

    /// The interrupts associated with GPT4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPT4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPT4: *const RegisterBlock = 0x400f8000 as *const _;

/// The GPT5 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPT5 = Instance<5>;

/// The GPT5 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPT5 = Instance<5>;
/// ```
#[cfg(feature = "doc")]
pub struct GPT5 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPT5 {}
impl crate::Valid for GPT5 {
    fn take() -> Option<Self> {
        <GPT5>::take()
    }
    fn release(self) {
        <GPT5>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPT5>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPT5_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPT5 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPT5 {
    const INSTANCE: Self = Self {
        addr: 0x400fc000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPT5],
    };

    /// Reset values for each field in GPT5
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        PR: 0x00000000,
        SR: 0x00000000,
        IR: 0x00000000,
        OCR1: 0xFFFFFFFF,
        OCR2: 0xFFFFFFFF,
        OCR3: 0xFFFFFFFF,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        CNT: 0x00000000,
    };

    /// Safe access to GPT5
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
        let taken = GPT5_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPT5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPT5_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPT5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPT5_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPT5 {
    /// The interrupts associated with GPT5
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPT5];

    /// The interrupts associated with GPT5
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPT5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPT5: *const RegisterBlock = 0x400fc000 as *const _;

/// The GPT6 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPT6 = Instance<6>;

/// The GPT6 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPT6 = Instance<6>;
/// ```
#[cfg(feature = "doc")]
pub struct GPT6 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPT6 {}
impl crate::Valid for GPT6 {
    fn take() -> Option<Self> {
        <GPT6>::take()
    }
    fn release(self) {
        <GPT6>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPT6>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPT6_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPT6 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPT6 {
    const INSTANCE: Self = Self {
        addr: 0x40100000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPT6],
    };

    /// Reset values for each field in GPT6
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        PR: 0x00000000,
        SR: 0x00000000,
        IR: 0x00000000,
        OCR1: 0xFFFFFFFF,
        OCR2: 0xFFFFFFFF,
        OCR3: 0xFFFFFFFF,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        CNT: 0x00000000,
    };

    /// Safe access to GPT6
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
        let taken = GPT6_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPT6
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPT6_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPT6
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPT6_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPT6 {
    /// The interrupts associated with GPT6
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPT6];

    /// The interrupts associated with GPT6
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPT6
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPT6: *const RegisterBlock = 0x40100000 as *const _;
