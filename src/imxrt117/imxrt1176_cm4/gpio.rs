#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIO

pub use crate::imxrt117::peripherals::gpio::Instance;
pub use crate::imxrt117::peripherals::gpio::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::gpio::{
    DR, DR_CLEAR, DR_SET, DR_TOGGLE, EDGE_SEL, GDIR, ICR1, ICR2, IMR, ISR, PSR,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The GPIO1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO1 = Instance<1>;

/// The GPIO1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO1 {}
impl crate::Valid for GPIO1 {
    fn take() -> Option<Self> {
        <GPIO1>::take()
    }
    fn release(self) {
        <GPIO1>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO1 {
    const INSTANCE: Self = Self {
        addr: 0x4012c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO1_Combined_0_15,
            crate::interrupt::GPIO1_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO1
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO1
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
        let taken = GPIO1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO1 {
    /// The interrupts associated with GPIO1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO1_Combined_0_15,
        crate::interrupt::GPIO1_Combined_16_31,
    ];

    /// The interrupts associated with GPIO1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO1: *const RegisterBlock = 0x4012c000 as *const _;

/// The GPIO10 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO10 = Instance<10>;

/// The GPIO10 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO10 = Instance<10>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO10 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO10 {}
impl crate::Valid for GPIO10 {
    fn take() -> Option<Self> {
        <GPIO10>::take()
    }
    fn release(self) {
        <GPIO10>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO10>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO10_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO10 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO10 {
    const INSTANCE: Self = Self {
        addr: 0x40c68000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPIO7_8_9_10_11],
    };

    /// Reset values for each field in GPIO10
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO10
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
        let taken = GPIO10_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO10
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO10_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO10
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO10_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO10 {
    /// The interrupts associated with GPIO10
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPIO7_8_9_10_11];

    /// The interrupts associated with GPIO10
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO10
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO10: *const RegisterBlock = 0x40c68000 as *const _;

/// The GPIO11 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO11 = Instance<11>;

/// The GPIO11 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO11 = Instance<11>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO11 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO11 {}
impl crate::Valid for GPIO11 {
    fn take() -> Option<Self> {
        <GPIO11>::take()
    }
    fn release(self) {
        <GPIO11>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO11>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO11_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO11 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO11 {
    const INSTANCE: Self = Self {
        addr: 0x40c6c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPIO7_8_9_10_11],
    };

    /// Reset values for each field in GPIO11
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO11
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
        let taken = GPIO11_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO11
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO11_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO11
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO11_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO11 {
    /// The interrupts associated with GPIO11
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPIO7_8_9_10_11];

    /// The interrupts associated with GPIO11
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO11
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO11: *const RegisterBlock = 0x40c6c000 as *const _;

/// The GPIO12 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO12 = Instance<12>;

/// The GPIO12 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO12 = Instance<12>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO12 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO12 {}
impl crate::Valid for GPIO12 {
    fn take() -> Option<Self> {
        <GPIO12>::take()
    }
    fn release(self) {
        <GPIO12>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO12>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO12_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO12 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO12 {
    const INSTANCE: Self = Self {
        addr: 0x40c70000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO12_Combined_0_15,
            crate::interrupt::GPIO12_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO12
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO12
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
        let taken = GPIO12_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO12
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO12_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO12
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO12_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO12 {
    /// The interrupts associated with GPIO12
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO12_Combined_0_15,
        crate::interrupt::GPIO12_Combined_16_31,
    ];

    /// The interrupts associated with GPIO12
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO12
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO12: *const RegisterBlock = 0x40c70000 as *const _;

/// The GPIO13 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO13 = Instance<13>;

/// The GPIO13 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO13 = Instance<13>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO13 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO13 {}
impl crate::Valid for GPIO13 {
    fn take() -> Option<Self> {
        <GPIO13>::take()
    }
    fn release(self) {
        <GPIO13>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO13>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO13_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO13 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO13 {
    const INSTANCE: Self = Self {
        addr: 0x40ca0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPIO13_Combined_0_31],
    };

    /// Reset values for each field in GPIO13
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO13
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
        let taken = GPIO13_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO13
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO13_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO13
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO13_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO13 {
    /// The interrupts associated with GPIO13
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPIO13_Combined_0_31];

    /// The interrupts associated with GPIO13
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO13
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO13: *const RegisterBlock = 0x40ca0000 as *const _;

/// The GPIO2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO2 = Instance<2>;

/// The GPIO2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO2 {}
impl crate::Valid for GPIO2 {
    fn take() -> Option<Self> {
        <GPIO2>::take()
    }
    fn release(self) {
        <GPIO2>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO2 {
    const INSTANCE: Self = Self {
        addr: 0x40130000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO2_Combined_0_15,
            crate::interrupt::GPIO2_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO2
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO2
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
        let taken = GPIO2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO2 {
    /// The interrupts associated with GPIO2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO2_Combined_0_15,
        crate::interrupt::GPIO2_Combined_16_31,
    ];

    /// The interrupts associated with GPIO2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO2: *const RegisterBlock = 0x40130000 as *const _;

/// The GPIO3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO3 = Instance<3>;

/// The GPIO3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO3 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO3 {}
impl crate::Valid for GPIO3 {
    fn take() -> Option<Self> {
        <GPIO3>::take()
    }
    fn release(self) {
        <GPIO3>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO3>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO3 {
    const INSTANCE: Self = Self {
        addr: 0x40134000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO3_Combined_0_15,
            crate::interrupt::GPIO3_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO3
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO3
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
        let taken = GPIO3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO3 {
    /// The interrupts associated with GPIO3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO3_Combined_0_15,
        crate::interrupt::GPIO3_Combined_16_31,
    ];

    /// The interrupts associated with GPIO3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO3: *const RegisterBlock = 0x40134000 as *const _;

/// The GPIO4 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO4 = Instance<4>;

/// The GPIO4 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO4 = Instance<4>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO4 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO4 {}
impl crate::Valid for GPIO4 {
    fn take() -> Option<Self> {
        <GPIO4>::take()
    }
    fn release(self) {
        <GPIO4>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO4>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO4 {
    const INSTANCE: Self = Self {
        addr: 0x40138000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO4_Combined_0_15,
            crate::interrupt::GPIO4_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO4
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO4
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
        let taken = GPIO4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO4 {
    /// The interrupts associated with GPIO4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO4_Combined_0_15,
        crate::interrupt::GPIO4_Combined_16_31,
    ];

    /// The interrupts associated with GPIO4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO4: *const RegisterBlock = 0x40138000 as *const _;

/// The GPIO5 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO5 = Instance<5>;

/// The GPIO5 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO5 = Instance<5>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO5 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO5 {}
impl crate::Valid for GPIO5 {
    fn take() -> Option<Self> {
        <GPIO5>::take()
    }
    fn release(self) {
        <GPIO5>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO5>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO5_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO5 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO5 {
    const INSTANCE: Self = Self {
        addr: 0x4013c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO5_Combined_0_15,
            crate::interrupt::GPIO5_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO5
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO5
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
        let taken = GPIO5_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO5_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO5_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO5 {
    /// The interrupts associated with GPIO5
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO5_Combined_0_15,
        crate::interrupt::GPIO5_Combined_16_31,
    ];

    /// The interrupts associated with GPIO5
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO5: *const RegisterBlock = 0x4013c000 as *const _;

/// The GPIO6 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO6 = Instance<6>;

/// The GPIO6 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO6 = Instance<6>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO6 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO6 {}
impl crate::Valid for GPIO6 {
    fn take() -> Option<Self> {
        <GPIO6>::take()
    }
    fn release(self) {
        <GPIO6>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO6>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO6_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO6 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO6 {
    const INSTANCE: Self = Self {
        addr: 0x40140000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in GPIO6
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO6
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
        let taken = GPIO6_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO6
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO6_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO6
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO6_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO6 {
    /// The interrupts associated with GPIO6
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with GPIO6
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO6
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO6: *const RegisterBlock = 0x40140000 as *const _;

/// The GPIO7 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO7 = Instance<7>;

/// The GPIO7 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO7 = Instance<7>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO7 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO7 {}
impl crate::Valid for GPIO7 {
    fn take() -> Option<Self> {
        <GPIO7>::take()
    }
    fn release(self) {
        <GPIO7>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO7>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO7_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO7 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO7 {
    const INSTANCE: Self = Self {
        addr: 0x40c5c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPIO7_8_9_10_11],
    };

    /// Reset values for each field in GPIO7
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO7
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
        let taken = GPIO7_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO7
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO7_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO7
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO7_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO7 {
    /// The interrupts associated with GPIO7
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPIO7_8_9_10_11];

    /// The interrupts associated with GPIO7
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO7
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO7: *const RegisterBlock = 0x40c5c000 as *const _;

/// The GPIO8 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO8 = Instance<8>;

/// The GPIO8 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO8 = Instance<8>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO8 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO8 {}
impl crate::Valid for GPIO8 {
    fn take() -> Option<Self> {
        <GPIO8>::take()
    }
    fn release(self) {
        <GPIO8>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO8>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO8_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO8 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO8 {
    const INSTANCE: Self = Self {
        addr: 0x40c60000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPIO7_8_9_10_11],
    };

    /// Reset values for each field in GPIO8
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO8
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
        let taken = GPIO8_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO8
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO8_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO8
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO8_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO8 {
    /// The interrupts associated with GPIO8
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPIO7_8_9_10_11];

    /// The interrupts associated with GPIO8
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO8
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO8: *const RegisterBlock = 0x40c60000 as *const _;

/// The GPIO9 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPIO9 = Instance<9>;

/// The GPIO9 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO9 = Instance<9>;
/// ```
#[cfg(feature = "doc")]
pub struct GPIO9 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPIO9 {}
impl crate::Valid for GPIO9 {
    fn take() -> Option<Self> {
        <GPIO9>::take()
    }
    fn release(self) {
        <GPIO9>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPIO9>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO9_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO9 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO9 {
    const INSTANCE: Self = Self {
        addr: 0x40c64000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPIO7_8_9_10_11],
    };

    /// Reset values for each field in GPIO9
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO9
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
        let taken = GPIO9_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO9
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO9_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO9
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO9_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPIO9 {
    /// The interrupts associated with GPIO9
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPIO7_8_9_10_11];

    /// The interrupts associated with GPIO9
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO9
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO9: *const RegisterBlock = 0x40c64000 as *const _;
