#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! VIDEO_MUX
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::video_mux::Instance;
pub use crate::imxrt117::peripherals::video_mux::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::video_mux::{
    CFG_DT_DISABLE, CFG_DT_DISABLE_CLR, CFG_DT_DISABLE_SET, CFG_DT_DISABLE_TOG, MIPI_DSI_CTRL,
    MIPI_DSI_CTRL_CLR, MIPI_DSI_CTRL_SET, MIPI_DSI_CTRL_TOG, PLM_CTRL, PLM_CTRL_CLR, PLM_CTRL_SET,
    PLM_CTRL_TOG, VID_MUX_CTRL, VID_MUX_CTRL_CLR, VID_MUX_CTRL_SET, VID_MUX_CTRL_TOG, YUV420_CTRL,
    YUV420_CTRL_CLR, YUV420_CTRL_SET, YUV420_CTRL_TOG,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The VIDEO_MUX peripheral instance.
#[cfg(not(feature = "doc"))]
pub type VIDEO_MUX = Instance<0>;

/// The VIDEO_MUX peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type VIDEO_MUX = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct VIDEO_MUX {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for VIDEO_MUX {}
impl crate::Valid for VIDEO_MUX {
    fn take() -> Option<Self> {
        <VIDEO_MUX>::take()
    }
    fn release(self) {
        <VIDEO_MUX>::release(self);
    }
    unsafe fn steal() -> Self {
        <VIDEO_MUX>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static VIDEO_MUX_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the VIDEO_MUX peripheral instance
#[cfg(not(feature = "nosync"))]
impl VIDEO_MUX {
    const INSTANCE: Self = Self {
        addr: 0x40818000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in VIDEO_MUX
    pub const reset: ResetValues = ResetValues {
        VID_MUX_CTRL: 0x00000000,
        VID_MUX_CTRL_SET: 0x00000000,
        VID_MUX_CTRL_CLR: 0x00000000,
        VID_MUX_CTRL_TOG: 0x00000000,
        PLM_CTRL: 0x00000000,
        PLM_CTRL_SET: 0x00000000,
        PLM_CTRL_CLR: 0x00000000,
        PLM_CTRL_TOG: 0x00000000,
        YUV420_CTRL: 0x00000000,
        YUV420_CTRL_SET: 0x00000000,
        YUV420_CTRL_CLR: 0x00000000,
        YUV420_CTRL_TOG: 0x00000000,
        CFG_DT_DISABLE: 0x00000000,
        CFG_DT_DISABLE_SET: 0x00000000,
        CFG_DT_DISABLE_CLR: 0x00000000,
        CFG_DT_DISABLE_TOG: 0x00000000,
        MIPI_DSI_CTRL: 0x00000000,
        MIPI_DSI_CTRL_SET: 0x00000000,
        MIPI_DSI_CTRL_CLR: 0x00000000,
        MIPI_DSI_CTRL_TOG: 0x00000000,
    };

    /// Safe access to VIDEO_MUX
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
        let taken = VIDEO_MUX_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to VIDEO_MUX
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = VIDEO_MUX_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal VIDEO_MUX
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        VIDEO_MUX_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl VIDEO_MUX {
    /// The interrupts associated with VIDEO_MUX
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with VIDEO_MUX
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to VIDEO_MUX
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const VIDEO_MUX: *const RegisterBlock = 0x40818000 as *const _;
