#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SRC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::src::Instance;
pub use crate::imxrt117::peripherals::src::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::src::{
    AUTHEN_DISPLAY, AUTHEN_M4CORE, AUTHEN_M4DEBUG, AUTHEN_M7CORE, AUTHEN_M7DEBUG, AUTHEN_MEGA,
    AUTHEN_USBPHY1, AUTHEN_USBPHY2, AUTHEN_WAKEUP, CTRL_DISPLAY, CTRL_M4CORE, CTRL_M4DEBUG,
    CTRL_M7CORE, CTRL_M7DEBUG, CTRL_MEGA, CTRL_USBPHY1, CTRL_USBPHY2, CTRL_WAKEUP, DOMAIN_DISPLAY,
    DOMAIN_M4CORE, DOMAIN_M4DEBUG, DOMAIN_M7CORE, DOMAIN_M7DEBUG, DOMAIN_MEGA, DOMAIN_USBPHY1,
    DOMAIN_USBPHY2, DOMAIN_WAKEUP, GPR1, GPR10, GPR11, GPR12, GPR13, GPR14, GPR15, GPR16, GPR17,
    GPR18, GPR19, GPR2, GPR20, GPR3, GPR4, GPR5, GPR6, GPR7, GPR8, GPR9, SBMR1, SBMR2, SCR,
    SETPOINT_DISPLAY, SETPOINT_M4CORE, SETPOINT_M4DEBUG, SETPOINT_M7CORE, SETPOINT_M7DEBUG,
    SETPOINT_MEGA, SETPOINT_USBPHY1, SETPOINT_USBPHY2, SETPOINT_WAKEUP, SRMR, SRSR, STAT_DISPLAY,
    STAT_M4CORE, STAT_M4DEBUG, STAT_M7CORE, STAT_M7DEBUG, STAT_MEGA, STAT_USBPHY1, STAT_USBPHY2,
    STAT_WAKEUP,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The SRC peripheral instance.
#[cfg(not(feature = "doc"))]
pub type SRC = Instance<0>;

/// The SRC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type SRC = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct SRC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for SRC {}
impl crate::Valid for SRC {
    fn take() -> Option<Self> {
        <SRC>::take()
    }
    fn release(self) {
        <SRC>::release(self);
    }
    unsafe fn steal() -> Self {
        <SRC>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static SRC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the SRC peripheral instance
#[cfg(not(feature = "nosync"))]
impl SRC {
    const INSTANCE: Self = Self {
        addr: 0x40c04000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in SRC
    pub const reset: ResetValues = ResetValues {
        SCR: 0x00000000,
        SRMR: 0x00000000,
        SBMR1: 0x00000000,
        SBMR2: 0x00000000,
        SRSR: 0x00000000,
        GPR1: 0x00000000,
        GPR2: 0x00000000,
        GPR3: 0x00000000,
        GPR4: 0x00000000,
        GPR5: 0x00000000,
        GPR6: 0x00000000,
        GPR7: 0x00000000,
        GPR8: 0x00000000,
        GPR9: 0x00000000,
        GPR10: 0x00000000,
        GPR11: 0x00000000,
        GPR12: 0x00000000,
        GPR13: 0x00000000,
        GPR14: 0x00000000,
        GPR15: 0x00000000,
        GPR16: 0x00000000,
        GPR17: 0x00000000,
        GPR18: 0x00000000,
        GPR19: 0x00000000,
        GPR20: 0x00000000,
        AUTHEN_MEGA: 0x00000000,
        CTRL_MEGA: 0x00000000,
        SETPOINT_MEGA: 0x00000000,
        DOMAIN_MEGA: 0x00000000,
        STAT_MEGA: 0x00000000,
        AUTHEN_DISPLAY: 0x00000000,
        CTRL_DISPLAY: 0x00000000,
        SETPOINT_DISPLAY: 0x00000000,
        DOMAIN_DISPLAY: 0x00000000,
        STAT_DISPLAY: 0x00000000,
        AUTHEN_WAKEUP: 0x00000000,
        CTRL_WAKEUP: 0x00000000,
        SETPOINT_WAKEUP: 0x00000000,
        DOMAIN_WAKEUP: 0x00000000,
        STAT_WAKEUP: 0x00000000,
        AUTHEN_M4CORE: 0x00000000,
        CTRL_M4CORE: 0x00000000,
        SETPOINT_M4CORE: 0x00000000,
        DOMAIN_M4CORE: 0x00000000,
        STAT_M4CORE: 0x00000000,
        AUTHEN_M7CORE: 0x00000000,
        CTRL_M7CORE: 0x00000000,
        SETPOINT_M7CORE: 0x00000000,
        DOMAIN_M7CORE: 0x00000000,
        STAT_M7CORE: 0x00000000,
        AUTHEN_M4DEBUG: 0x00000000,
        CTRL_M4DEBUG: 0x00000000,
        SETPOINT_M4DEBUG: 0x00000000,
        DOMAIN_M4DEBUG: 0x00000000,
        STAT_M4DEBUG: 0x00000000,
        AUTHEN_M7DEBUG: 0x00000000,
        CTRL_M7DEBUG: 0x00000000,
        SETPOINT_M7DEBUG: 0x00000000,
        DOMAIN_M7DEBUG: 0x00000000,
        STAT_M7DEBUG: 0x00000000,
        AUTHEN_USBPHY1: 0x00000000,
        CTRL_USBPHY1: 0x00000000,
        SETPOINT_USBPHY1: 0x00000000,
        DOMAIN_USBPHY1: 0x00000000,
        STAT_USBPHY1: 0x00000000,
        AUTHEN_USBPHY2: 0x00000000,
        CTRL_USBPHY2: 0x00000000,
        SETPOINT_USBPHY2: 0x00000000,
        DOMAIN_USBPHY2: 0x00000000,
        STAT_USBPHY2: 0x00000000,
    };

    /// Safe access to SRC
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
        let taken = SRC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to SRC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = SRC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal SRC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        SRC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl SRC {
    /// The interrupts associated with SRC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with SRC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to SRC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SRC: *const RegisterBlock = 0x40c04000 as *const _;
