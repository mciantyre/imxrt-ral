#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DSI Host DPI Interface
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::dsi_host_dpi_intfc::Instance;
pub use crate::imxrt117::peripherals::dsi_host_dpi_intfc::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::dsi_host_dpi_intfc::{
    BLLP_MODE, ENABLE_MULT_PKTS, HBP, HFP, HSA, HSYNC_POLARITY, INTERFACE_COLOR_CODING,
    PIXEL_FIFO_SEND_LEVEL, PIXEL_FORMAT, PIXEL_PAYLOAD_SIZE, USE_NULL_PKT_BLLP, VACTIVE, VBP, VFP,
    VIDEO_MODE, VSYNC_POLARITY,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The DSI_HOST_DPI_INTFC peripheral instance.
#[cfg(not(feature = "doc"))]
pub type DSI_HOST_DPI_INTFC = Instance<0>;

/// The DSI_HOST_DPI_INTFC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type DSI_HOST_DPI_INTFC = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct DSI_HOST_DPI_INTFC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for DSI_HOST_DPI_INTFC {}
impl crate::Valid for DSI_HOST_DPI_INTFC {
    fn take() -> Option<Self> {
        <DSI_HOST_DPI_INTFC>::take()
    }
    fn release(self) {
        <DSI_HOST_DPI_INTFC>::release(self);
    }
    unsafe fn steal() -> Self {
        <DSI_HOST_DPI_INTFC>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static DSI_HOST_DPI_INTFC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the DSI_HOST_DPI_INTFC peripheral instance
#[cfg(not(feature = "nosync"))]
impl DSI_HOST_DPI_INTFC {
    const INSTANCE: Self = Self {
        addr: 0x4080c200,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in DSI_HOST_DPI_INTFC
    pub const reset: ResetValues = ResetValues {
        PIXEL_PAYLOAD_SIZE: 0x00000000,
        PIXEL_FIFO_SEND_LEVEL: 0x00000000,
        INTERFACE_COLOR_CODING: 0x00000000,
        PIXEL_FORMAT: 0x00000000,
        VSYNC_POLARITY: 0x00000000,
        HSYNC_POLARITY: 0x00000000,
        VIDEO_MODE: 0x00000000,
        HFP: 0x00000000,
        HBP: 0x00000000,
        HSA: 0x00000000,
        ENABLE_MULT_PKTS: 0x00000000,
        VBP: 0x00000000,
        VFP: 0x00000000,
        BLLP_MODE: 0x00000000,
        USE_NULL_PKT_BLLP: 0x00000000,
        VACTIVE: 0x00000000,
    };

    /// Safe access to DSI_HOST_DPI_INTFC
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
        let taken = DSI_HOST_DPI_INTFC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to DSI_HOST_DPI_INTFC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = DSI_HOST_DPI_INTFC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal DSI_HOST_DPI_INTFC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        DSI_HOST_DPI_INTFC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl DSI_HOST_DPI_INTFC {
    /// The interrupts associated with DSI_HOST_DPI_INTFC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with DSI_HOST_DPI_INTFC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to DSI_HOST_DPI_INTFC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DSI_HOST_DPI_INTFC: *const RegisterBlock = 0x4080c200 as *const _;
