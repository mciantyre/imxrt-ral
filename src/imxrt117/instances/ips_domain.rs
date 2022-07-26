#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IPS Domain
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::ips_domain::Instance;
pub use crate::imxrt117::peripherals::ips_domain::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::ips_domain::{
    SLOT_CTRL_0, SLOT_CTRL_1, SLOT_CTRL_10, SLOT_CTRL_11, SLOT_CTRL_12, SLOT_CTRL_13, SLOT_CTRL_14,
    SLOT_CTRL_15, SLOT_CTRL_16, SLOT_CTRL_17, SLOT_CTRL_18, SLOT_CTRL_19, SLOT_CTRL_2,
    SLOT_CTRL_20, SLOT_CTRL_21, SLOT_CTRL_22, SLOT_CTRL_23, SLOT_CTRL_24, SLOT_CTRL_25,
    SLOT_CTRL_26, SLOT_CTRL_27, SLOT_CTRL_28, SLOT_CTRL_29, SLOT_CTRL_3, SLOT_CTRL_30,
    SLOT_CTRL_31, SLOT_CTRL_32, SLOT_CTRL_33, SLOT_CTRL_34, SLOT_CTRL_35, SLOT_CTRL_36,
    SLOT_CTRL_37, SLOT_CTRL_4, SLOT_CTRL_5, SLOT_CTRL_6, SLOT_CTRL_7, SLOT_CTRL_8, SLOT_CTRL_9,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The IPS_DOMAIN peripheral instance.
#[cfg(not(feature = "doc"))]
pub type IPS_DOMAIN = Instance<0>;

/// The IPS_DOMAIN peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type IPS_DOMAIN = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct IPS_DOMAIN {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for IPS_DOMAIN {}
impl crate::Valid for IPS_DOMAIN {
    fn take() -> Option<Self> {
        <IPS_DOMAIN>::take()
    }
    fn release(self) {
        <IPS_DOMAIN>::release(self);
    }
    unsafe fn steal() -> Self {
        <IPS_DOMAIN>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static IPS_DOMAIN_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the IPS_DOMAIN peripheral instance
#[cfg(not(feature = "nosync"))]
impl IPS_DOMAIN {
    const INSTANCE: Self = Self {
        addr: 0x40c87c00,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in IPS_DOMAIN
    pub const reset: ResetValues = ResetValues {
        SLOT_CTRL_0: 0x0000000F,
        SLOT_CTRL_1: 0x0000000F,
        SLOT_CTRL_2: 0x0000000F,
        SLOT_CTRL_3: 0x0000000F,
        SLOT_CTRL_4: 0x0000000F,
        SLOT_CTRL_5: 0x0000000F,
        SLOT_CTRL_6: 0x0000000F,
        SLOT_CTRL_7: 0x0000000F,
        SLOT_CTRL_8: 0x0000000F,
        SLOT_CTRL_9: 0x0000000F,
        SLOT_CTRL_10: 0x0000000F,
        SLOT_CTRL_11: 0x0000000F,
        SLOT_CTRL_12: 0x0000000F,
        SLOT_CTRL_13: 0x0000000F,
        SLOT_CTRL_14: 0x0000000F,
        SLOT_CTRL_15: 0x0000000F,
        SLOT_CTRL_16: 0x0000000F,
        SLOT_CTRL_17: 0x0000000F,
        SLOT_CTRL_18: 0x0000000F,
        SLOT_CTRL_19: 0x0000000F,
        SLOT_CTRL_20: 0x0000000F,
        SLOT_CTRL_21: 0x0000000F,
        SLOT_CTRL_22: 0x0000000F,
        SLOT_CTRL_23: 0x0000000F,
        SLOT_CTRL_24: 0x0000000F,
        SLOT_CTRL_25: 0x0000000F,
        SLOT_CTRL_26: 0x0000000F,
        SLOT_CTRL_27: 0x0000000F,
        SLOT_CTRL_28: 0x0000000F,
        SLOT_CTRL_29: 0x0000000F,
        SLOT_CTRL_30: 0x0000000F,
        SLOT_CTRL_31: 0x0000000F,
        SLOT_CTRL_32: 0x0000000F,
        SLOT_CTRL_33: 0x0000000F,
        SLOT_CTRL_34: 0x0000000F,
        SLOT_CTRL_35: 0x0000000F,
        SLOT_CTRL_36: 0x0000000F,
        SLOT_CTRL_37: 0x0000000F,
    };

    /// Safe access to IPS_DOMAIN
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
        let taken = IPS_DOMAIN_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to IPS_DOMAIN
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = IPS_DOMAIN_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal IPS_DOMAIN
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        IPS_DOMAIN_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl IPS_DOMAIN {
    /// The interrupts associated with IPS_DOMAIN
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with IPS_DOMAIN
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to IPS_DOMAIN
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IPS_DOMAIN: *const RegisterBlock = 0x40c87c00 as *const _;
