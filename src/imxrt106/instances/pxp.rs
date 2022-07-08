#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PXP v2.0 Register Reference Index
//!
//! Used by: imxrt1062, imxrt1064

pub use crate::imxrt106::peripherals::pxp::Instance;
pub use crate::imxrt106::peripherals::pxp::{RegisterBlock, ResetValues};

pub use crate::imxrt106::peripherals::pxp::{
    AS_BUF, AS_CLRKEYHIGH, AS_CLRKEYLOW, AS_CTRL, AS_PITCH, CSC1_COEF0, CSC1_COEF1, CSC1_COEF2,
    CTRL, CTRL_CLR, CTRL_SET, CTRL_TOG, NEXT, OUT_AS_LRC, OUT_AS_ULC, OUT_BUF, OUT_BUF2, OUT_CTRL,
    OUT_CTRL_CLR, OUT_CTRL_SET, OUT_CTRL_TOG, OUT_LRC, OUT_PITCH, OUT_PS_LRC, OUT_PS_ULC,
    PORTER_DUFF_CTRL, POWER, PS_BACKGROUND, PS_BUF, PS_CLRKEYHIGH, PS_CLRKEYLOW, PS_CTRL,
    PS_CTRL_CLR, PS_CTRL_SET, PS_CTRL_TOG, PS_OFFSET, PS_PITCH, PS_SCALE, PS_UBUF, PS_VBUF, STAT,
    STAT_CLR, STAT_SET, STAT_TOG,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The PXP peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PXP = Instance<0>;

/// The PXP peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PXP = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct PXP {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PXP {}
impl crate::Valid for PXP {
    fn take() -> Option<Self> {
        <PXP>::take()
    }
    fn release(self) {
        <PXP>::release(self);
    }
    unsafe fn steal() -> Self {
        <PXP>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PXP_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PXP peripheral instance
#[cfg(not(feature = "nosync"))]
impl PXP {
    const INSTANCE: Self = Self {
        addr: 0x402b4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::PXP],
    };

    /// Reset values for each field in PXP
    pub const reset: ResetValues = ResetValues {
        CTRL: 0xC0000000,
        CTRL_SET: 0xC0000000,
        CTRL_CLR: 0xC0000000,
        CTRL_TOG: 0xC0000000,
        STAT: 0x00000000,
        STAT_SET: 0x00000000,
        STAT_CLR: 0x00000000,
        STAT_TOG: 0x00000000,
        OUT_CTRL: 0x00000000,
        OUT_CTRL_SET: 0x00000000,
        OUT_CTRL_CLR: 0x00000000,
        OUT_CTRL_TOG: 0x00000000,
        OUT_BUF: 0x00000000,
        OUT_BUF2: 0x00000000,
        OUT_PITCH: 0x00000000,
        OUT_LRC: 0x00000000,
        OUT_PS_ULC: 0x00000000,
        OUT_PS_LRC: 0x00000000,
        OUT_AS_ULC: 0x00000000,
        OUT_AS_LRC: 0x00000000,
        PS_CTRL: 0x00000000,
        PS_CTRL_SET: 0x00000000,
        PS_CTRL_CLR: 0x00000000,
        PS_CTRL_TOG: 0x00000000,
        PS_BUF: 0x00000000,
        PS_UBUF: 0x00000000,
        PS_VBUF: 0x00000000,
        PS_PITCH: 0x00000000,
        PS_BACKGROUND: 0x00000000,
        PS_SCALE: 0x10001000,
        PS_OFFSET: 0x00000000,
        PS_CLRKEYLOW: 0x00FFFFFF,
        PS_CLRKEYHIGH: 0x00000000,
        AS_CTRL: 0x00000000,
        AS_BUF: 0x00000000,
        AS_PITCH: 0x00000000,
        AS_CLRKEYLOW: 0x00FFFFFF,
        AS_CLRKEYHIGH: 0x00000000,
        CSC1_COEF0: 0x04000000,
        CSC1_COEF1: 0x01230208,
        CSC1_COEF2: 0x079B076C,
        POWER: 0x00000000,
        NEXT: 0x00000000,
        PORTER_DUFF_CTRL: 0x00000000,
    };

    /// Safe access to PXP
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
        let taken = PXP_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PXP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PXP_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PXP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PXP_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PXP {
    /// The interrupts associated with PXP
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::PXP];

    /// The interrupts associated with PXP
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PXP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PXP: *const RegisterBlock = 0x402b4000 as *const _;
