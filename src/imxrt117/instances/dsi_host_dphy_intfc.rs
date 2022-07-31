#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DSI HOST DPHY INTFC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::dsi_host_dphy_intfc::Instance;
pub use crate::imxrt117::peripherals::dsi_host_dphy_intfc::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::dsi_host_dphy_intfc::{
    AUTO_PD_EN, CM, CN, CO, LOCK, LOCK_BYP, MC_PRG_HS_PREPARE, MC_PRG_HS_TRAIL, MC_PRG_HS_ZERO,
    M_PRG_HS_PREPARE, M_PRG_HS_TRAIL, M_PRG_HS_ZERO, PD_PLL, PD_TX, RXCDRP, RXLPRP, TST, TX_RCAL,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The DSI_HOST_DPHY_INTFC peripheral instance.
#[cfg(not(feature = "doc"))]
pub type DSI_HOST_DPHY_INTFC = Instance<0>;

/// The DSI_HOST_DPHY_INTFC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type DSI_HOST_DPHY_INTFC = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct DSI_HOST_DPHY_INTFC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for DSI_HOST_DPHY_INTFC {}
impl crate::Valid for DSI_HOST_DPHY_INTFC {
    fn take() -> Option<Self> {
        <DSI_HOST_DPHY_INTFC>::take()
    }
    fn release(self) {
        <DSI_HOST_DPHY_INTFC>::release(self);
    }
    unsafe fn steal() -> Self {
        <DSI_HOST_DPHY_INTFC>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static DSI_HOST_DPHY_INTFC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the DSI_HOST_DPHY_INTFC peripheral instance
#[cfg(not(feature = "nosync"))]
impl DSI_HOST_DPHY_INTFC {
    const INSTANCE: Self = Self {
        addr: 0x4080c300,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in DSI_HOST_DPHY_INTFC
    pub const reset: ResetValues = ResetValues {
        PD_TX: 0x00000001,
        M_PRG_HS_PREPARE: 0x00000000,
        MC_PRG_HS_PREPARE: 0x00000000,
        M_PRG_HS_ZERO: 0x00000000,
        MC_PRG_HS_ZERO: 0x00000000,
        M_PRG_HS_TRAIL: 0x00000000,
        MC_PRG_HS_TRAIL: 0x00000000,
        PD_PLL: 0x00000001,
        TST: 0x00000025,
        CN: 0x00000000,
        CM: 0x00000000,
        CO: 0x00000000,
        LOCK: 0x00000000,
        LOCK_BYP: 0x00000000,
        TX_RCAL: 0x00000000,
        AUTO_PD_EN: 0x00000000,
        RXLPRP: 0x00000000,
        RXCDRP: 0x00000000,
    };

    /// Safe access to DSI_HOST_DPHY_INTFC
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
        let taken = DSI_HOST_DPHY_INTFC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to DSI_HOST_DPHY_INTFC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = DSI_HOST_DPHY_INTFC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal DSI_HOST_DPHY_INTFC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        DSI_HOST_DPHY_INTFC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl DSI_HOST_DPHY_INTFC {
    /// The interrupts associated with DSI_HOST_DPHY_INTFC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with DSI_HOST_DPHY_INTFC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to DSI_HOST_DPHY_INTFC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DSI_HOST_DPHY_INTFC: *const RegisterBlock = 0x4080c300 as *const _;
