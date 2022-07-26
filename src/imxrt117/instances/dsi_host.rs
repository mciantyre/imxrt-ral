#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DSI HOST
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::dsi_host::Instance;
pub use crate::imxrt117::peripherals::dsi_host::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::dsi_host::{
    CFG_AUTOINSERT_EOTP, CFG_BTA_H_TO_COUNT, CFG_EXTRA_CMDS_AFTER_EOTP, CFG_HTX_TO_COUNT,
    CFG_LRX_H_TO_COUNT, CFG_NONCONTINUOUS_CLK, CFG_NUM_LANES, CFG_STATUS_OUT, CFG_TWAKEUP,
    CFG_TX_GAP, CFG_T_POST, CFG_T_PRE, RX_ERROR_STATUS,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The DSI_HOST peripheral instance.
#[cfg(not(feature = "doc"))]
pub type DSI_HOST = Instance<0>;

/// The DSI_HOST peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type DSI_HOST = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct DSI_HOST {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for DSI_HOST {}
impl crate::Valid for DSI_HOST {
    fn take() -> Option<Self> {
        <DSI_HOST>::take()
    }
    fn release(self) {
        <DSI_HOST>::release(self);
    }
    unsafe fn steal() -> Self {
        <DSI_HOST>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static DSI_HOST_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the DSI_HOST peripheral instance
#[cfg(not(feature = "nosync"))]
impl DSI_HOST {
    const INSTANCE: Self = Self {
        addr: 0x4080c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::MIPI_DSI],
    };

    /// Reset values for each field in DSI_HOST
    pub const reset: ResetValues = ResetValues {
        CFG_NUM_LANES: 0x00000000,
        CFG_NONCONTINUOUS_CLK: 0x00000000,
        CFG_T_PRE: 0x00000000,
        CFG_T_POST: 0x00000000,
        CFG_TX_GAP: 0x00000000,
        CFG_AUTOINSERT_EOTP: 0x00000000,
        CFG_EXTRA_CMDS_AFTER_EOTP: 0x00000000,
        CFG_HTX_TO_COUNT: 0x00000000,
        CFG_LRX_H_TO_COUNT: 0x00000000,
        CFG_BTA_H_TO_COUNT: 0x00000000,
        CFG_TWAKEUP: 0x00000000,
        CFG_STATUS_OUT: 0x00000000,
        RX_ERROR_STATUS: 0x00000000,
    };

    /// Safe access to DSI_HOST
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
        let taken = DSI_HOST_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to DSI_HOST
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = DSI_HOST_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal DSI_HOST
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        DSI_HOST_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl DSI_HOST {
    /// The interrupts associated with DSI_HOST
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::MIPI_DSI];

    /// The interrupts associated with DSI_HOST
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to DSI_HOST
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DSI_HOST: *const RegisterBlock = 0x4080c000 as *const _;
