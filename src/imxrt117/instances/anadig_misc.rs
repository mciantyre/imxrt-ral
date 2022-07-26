#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MX6RT_ANADIG_REGISTER
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::anadig_misc::Instance;
pub use crate::imxrt117::peripherals::anadig_misc::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::anadig_misc::{
    MISC_DIFPROG, VDDLPSR_AI400M_CTRL, VDDLPSR_AI400M_RDATA, VDDLPSR_AI400M_WDATA, VDDLPSR_AI_CTRL,
    VDDLPSR_AI_RDATA_REFTOP, VDDLPSR_AI_RDATA_TMPSNS, VDDLPSR_AI_WDATA, VDDSOC2PLL_AI_CTRL_1G,
    VDDSOC2PLL_AI_CTRL_AUDIO, VDDSOC2PLL_AI_CTRL_VIDEO, VDDSOC2PLL_AI_RDATA_1G,
    VDDSOC2PLL_AI_RDATA_AUDIO, VDDSOC2PLL_AI_RDATA_VIDEO, VDDSOC2PLL_AI_WDATA_1G,
    VDDSOC2PLL_AI_WDATA_AUDIO, VDDSOC2PLL_AI_WDATA_VIDEO, VDDSOC_AI_CTRL, VDDSOC_AI_RDATA,
    VDDSOC_AI_WDATA,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The ANADIG_MISC peripheral instance.
#[cfg(not(feature = "doc"))]
pub type ANADIG_MISC = Instance<0>;

/// The ANADIG_MISC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ANADIG_MISC = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct ANADIG_MISC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for ANADIG_MISC {}
impl crate::Valid for ANADIG_MISC {
    fn take() -> Option<Self> {
        <ANADIG_MISC>::take()
    }
    fn release(self) {
        <ANADIG_MISC>::release(self);
    }
    unsafe fn steal() -> Self {
        <ANADIG_MISC>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ANADIG_MISC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ANADIG_MISC peripheral instance
#[cfg(not(feature = "nosync"))]
impl ANADIG_MISC {
    const INSTANCE: Self = Self {
        addr: 0x40c84000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in ANADIG_MISC
    pub const reset: ResetValues = ResetValues {
        MISC_DIFPROG: 0x001170B0,
        VDDSOC_AI_CTRL: 0x00000000,
        VDDSOC_AI_WDATA: 0x00000000,
        VDDSOC_AI_RDATA: 0x00000000,
        VDDSOC2PLL_AI_CTRL_1G: 0x00000000,
        VDDSOC2PLL_AI_WDATA_1G: 0x00000000,
        VDDSOC2PLL_AI_RDATA_1G: 0x00000000,
        VDDSOC2PLL_AI_CTRL_AUDIO: 0x00000000,
        VDDSOC2PLL_AI_WDATA_AUDIO: 0x00000000,
        VDDSOC2PLL_AI_RDATA_AUDIO: 0x00000000,
        VDDSOC2PLL_AI_CTRL_VIDEO: 0x00000000,
        VDDSOC2PLL_AI_WDATA_VIDEO: 0x00000000,
        VDDSOC2PLL_AI_RDATA_VIDEO: 0x00000000,
        VDDLPSR_AI_CTRL: 0x00000000,
        VDDLPSR_AI_WDATA: 0x00000000,
        VDDLPSR_AI_RDATA_REFTOP: 0x00000000,
        VDDLPSR_AI_RDATA_TMPSNS: 0x00000000,
        VDDLPSR_AI400M_CTRL: 0x00000000,
        VDDLPSR_AI400M_WDATA: 0x00000000,
        VDDLPSR_AI400M_RDATA: 0x00000000,
    };

    /// Safe access to ANADIG_MISC
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
        let taken = ANADIG_MISC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ANADIG_MISC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ANADIG_MISC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ANADIG_MISC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ANADIG_MISC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl ANADIG_MISC {
    /// The interrupts associated with ANADIG_MISC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with ANADIG_MISC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ANADIG_MISC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ANADIG_MISC: *const RegisterBlock = 0x40c84000 as *const _;
