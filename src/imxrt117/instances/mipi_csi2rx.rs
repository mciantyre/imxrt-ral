#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! no description available
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::mipi_csi2rx::Instance;
pub use crate::imxrt117::peripherals::mipi_csi2rx::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::mipi_csi2rx::{
    BIT_ERR, CFG_DISABLE_DATA_LANES, CFG_DISABLE_PAYLOAD_0, CFG_DISABLE_PAYLOAD_1, CFG_IGNORE_VC,
    CFG_NUM_LANES, CFG_VID_HSYNC, CFG_VID_HSYNC_BP, CFG_VID_HSYNC_FP, CFG_VID_P_FIFO_SEND_LEVEL,
    CFG_VID_VC, CFG_VID_VSYNC, IRQ_MASK, IRQ_STATUS, PPI_ERRCONTROL, PPI_ERRESC, PPI_ERRSOTSYNC_HS,
    PPI_ERRSOT_HS, PPI_ERRSYNCESC, ULPS_STATUS,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The MIPI_CSI2RX peripheral instance.
#[cfg(not(feature = "doc"))]
pub type MIPI_CSI2RX = Instance<0>;

/// The MIPI_CSI2RX peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type MIPI_CSI2RX = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct MIPI_CSI2RX {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for MIPI_CSI2RX {}
impl crate::Valid for MIPI_CSI2RX {
    fn take() -> Option<Self> {
        <MIPI_CSI2RX>::take()
    }
    fn release(self) {
        <MIPI_CSI2RX>::release(self);
    }
    unsafe fn steal() -> Self {
        <MIPI_CSI2RX>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static MIPI_CSI2RX_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the MIPI_CSI2RX peripheral instance
#[cfg(not(feature = "nosync"))]
impl MIPI_CSI2RX {
    const INSTANCE: Self = Self {
        addr: 0x40810000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::MIPI_CSI],
    };

    /// Reset values for each field in MIPI_CSI2RX
    pub const reset: ResetValues = ResetValues {
        CFG_NUM_LANES: 0x00000000,
        CFG_DISABLE_DATA_LANES: 0x0000000F,
        BIT_ERR: 0x00000000,
        IRQ_STATUS: 0x00000000,
        IRQ_MASK: 0x00000000,
        ULPS_STATUS: 0x00000000,
        PPI_ERRSOT_HS: 0x00000000,
        PPI_ERRSOTSYNC_HS: 0x00000000,
        PPI_ERRESC: 0x00000000,
        PPI_ERRSYNCESC: 0x00000000,
        PPI_ERRCONTROL: 0x00000000,
        CFG_DISABLE_PAYLOAD_0: 0x00000000,
        CFG_DISABLE_PAYLOAD_1: 0x00000000,
        CFG_IGNORE_VC: 0x00000000,
        CFG_VID_VC: 0x00000000,
        CFG_VID_P_FIFO_SEND_LEVEL: 0x00000000,
        CFG_VID_VSYNC: 0x00000000,
        CFG_VID_HSYNC_FP: 0x00000000,
        CFG_VID_HSYNC: 0x00000000,
        CFG_VID_HSYNC_BP: 0x00000000,
    };

    /// Safe access to MIPI_CSI2RX
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
        let taken = MIPI_CSI2RX_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to MIPI_CSI2RX
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = MIPI_CSI2RX_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal MIPI_CSI2RX
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        MIPI_CSI2RX_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl MIPI_CSI2RX {
    /// The interrupts associated with MIPI_CSI2RX
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::MIPI_CSI];

    /// The interrupts associated with MIPI_CSI2RX
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to MIPI_CSI2RX
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const MIPI_CSI2RX: *const RegisterBlock = 0x40810000 as *const _;
