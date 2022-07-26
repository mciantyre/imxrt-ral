#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MX6RT_ANADIG_REGISTER
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::anadig_pll::Instance;
pub use crate::imxrt117::peripherals::anadig_pll::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::anadig_pll::{
    ARM_PLL_CTRL, PLL_AUDIO_CTRL, PLL_AUDIO_DENOMINATOR, PLL_AUDIO_DIV_SELECT, PLL_AUDIO_NUMERATOR,
    PLL_AUDIO_SS, PLL_VIDEO_CTRL, PLL_VIDEO_DENOMINATOR, PLL_VIDEO_DIV_SELECT, PLL_VIDEO_NUMERATOR,
    PLL_VIDEO_SS, SYS_PLL1_CTRL, SYS_PLL1_DENOMINATOR, SYS_PLL1_DIV_SELECT, SYS_PLL1_NUMERATOR,
    SYS_PLL1_SS, SYS_PLL2_CTRL, SYS_PLL2_MFD, SYS_PLL2_PFD, SYS_PLL2_SS, SYS_PLL2_UPDATE,
    SYS_PLL3_CTRL, SYS_PLL3_PFD, SYS_PLL3_UPDATE,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The ANADIG_PLL peripheral instance.
#[cfg(not(feature = "doc"))]
pub type ANADIG_PLL = Instance<0>;

/// The ANADIG_PLL peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type ANADIG_PLL = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct ANADIG_PLL {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for ANADIG_PLL {}
impl crate::Valid for ANADIG_PLL {
    fn take() -> Option<Self> {
        <ANADIG_PLL>::take()
    }
    fn release(self) {
        <ANADIG_PLL>::release(self);
    }
    unsafe fn steal() -> Self {
        <ANADIG_PLL>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static ANADIG_PLL_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the ANADIG_PLL peripheral instance
#[cfg(not(feature = "nosync"))]
impl ANADIG_PLL {
    const INSTANCE: Self = Self {
        addr: 0x40c84000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in ANADIG_PLL
    pub const reset: ResetValues = ResetValues {
        ARM_PLL_CTRL: 0x400000A6,
        SYS_PLL3_CTRL: 0x40000003,
        SYS_PLL3_UPDATE: 0x00000000,
        SYS_PLL3_PFD: 0x8CA0918D,
        SYS_PLL2_CTRL: 0x40000000,
        SYS_PLL2_UPDATE: 0x00000000,
        SYS_PLL2_SS: 0x00000000,
        SYS_PLL2_PFD: 0xA098909B,
        SYS_PLL2_MFD: 0x0FFFFFFF,
        SYS_PLL1_SS: 0x00000000,
        SYS_PLL1_CTRL: 0x00004000,
        SYS_PLL1_DENOMINATOR: 0x2FFFFFFE,
        SYS_PLL1_NUMERATOR: 0x1FFFFFFF,
        SYS_PLL1_DIV_SELECT: 0x0000001D,
        PLL_AUDIO_CTRL: 0x00004000,
        PLL_AUDIO_SS: 0x00000000,
        PLL_AUDIO_DENOMINATOR: 0x2FFFFFFE,
        PLL_AUDIO_NUMERATOR: 0x1FFFFFFF,
        PLL_AUDIO_DIV_SELECT: 0x00000029,
        PLL_VIDEO_CTRL: 0x00004000,
        PLL_VIDEO_SS: 0x00000000,
        PLL_VIDEO_DENOMINATOR: 0x2FFFFFFE,
        PLL_VIDEO_NUMERATOR: 0x1FFFFFFF,
        PLL_VIDEO_DIV_SELECT: 0x00000029,
    };

    /// Safe access to ANADIG_PLL
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
        let taken = ANADIG_PLL_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to ANADIG_PLL
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = ANADIG_PLL_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal ANADIG_PLL
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        ANADIG_PLL_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl ANADIG_PLL {
    /// The interrupts associated with ANADIG_PLL
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with ANADIG_PLL
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to ANADIG_PLL
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ANADIG_PLL: *const RegisterBlock = 0x40c84000 as *const _;
