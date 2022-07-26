#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPC_SP
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::gpc_set_point_ctrl::Instance;
pub use crate::imxrt117::peripherals::gpc_set_point_ctrl::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::gpc_set_point_ctrl::{
    SP_AUTHEN_CTRL, SP_BG_PLDO_OFF_CTRL, SP_BG_PLDO_ON_CTRL, SP_BIAS_OFF_CTRL, SP_BIAS_ON_CTRL,
    SP_CPU_REQ, SP_DCDC_DOWN_CTRL, SP_DCDC_UP_CTRL, SP_GROUP_DOWN_CTRL, SP_GROUP_UP_CTRL,
    SP_INT_CTRL, SP_ISO_OFF_CTRL, SP_ISO_ON_CTRL, SP_LDO_POST_CTRL, SP_LDO_PRE_CTRL,
    SP_LPCG_OFF_CTRL, SP_LPCG_ON_CTRL, SP_PLL_OFF_CTRL, SP_PLL_ON_CTRL, SP_POWER_OFF_CTRL,
    SP_POWER_ON_CTRL, SP_PRIORITY_0_7, SP_PRIORITY_8_15, SP_RESET_EARLY_CTRL, SP_RESET_LATE_CTRL,
    SP_ROOT_DOWN_CTRL, SP_ROOT_UP_CTRL, SP_ROSC_CTRL, SP_SSAR_RESTORE_CTRL, SP_SSAR_SAVE_CTRL,
    SP_SYS_STAT,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The GPC_SET_POINT_CTRL peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPC_SET_POINT_CTRL = Instance<0>;

/// The GPC_SET_POINT_CTRL peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPC_SET_POINT_CTRL = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct GPC_SET_POINT_CTRL {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPC_SET_POINT_CTRL {}
impl crate::Valid for GPC_SET_POINT_CTRL {
    fn take() -> Option<Self> {
        <GPC_SET_POINT_CTRL>::take()
    }
    fn release(self) {
        <GPC_SET_POINT_CTRL>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPC_SET_POINT_CTRL>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPC_SET_POINT_CTRL_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPC_SET_POINT_CTRL peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPC_SET_POINT_CTRL {
    const INSTANCE: Self = Self {
        addr: 0x40c02000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in GPC_SET_POINT_CTRL
    pub const reset: ResetValues = ResetValues {
        SP_AUTHEN_CTRL: 0x00000F00,
        SP_INT_CTRL: 0x00000001,
        SP_CPU_REQ: 0x00000000,
        SP_SYS_STAT: 0x0000FFFF,
        SP_ROSC_CTRL: 0x00000000,
        SP_PRIORITY_0_7: 0x76543210,
        SP_PRIORITY_8_15: 0xFEDCBA98,
        SP_SSAR_SAVE_CTRL: 0x00000004,
        SP_LPCG_OFF_CTRL: 0x00000004,
        SP_GROUP_DOWN_CTRL: 0x00000004,
        SP_ROOT_DOWN_CTRL: 0x00000004,
        SP_PLL_OFF_CTRL: 0x00000004,
        SP_ISO_ON_CTRL: 0x00000004,
        SP_RESET_EARLY_CTRL: 0x00000004,
        SP_POWER_OFF_CTRL: 0x00000004,
        SP_BIAS_OFF_CTRL: 0x00000004,
        SP_BG_PLDO_OFF_CTRL: 0x00000004,
        SP_LDO_PRE_CTRL: 0x00000004,
        SP_DCDC_DOWN_CTRL: 0x00000004,
        SP_DCDC_UP_CTRL: 0x00000004,
        SP_LDO_POST_CTRL: 0x00000004,
        SP_BG_PLDO_ON_CTRL: 0x00000004,
        SP_BIAS_ON_CTRL: 0x00000004,
        SP_POWER_ON_CTRL: 0x00000004,
        SP_RESET_LATE_CTRL: 0x00000004,
        SP_ISO_OFF_CTRL: 0x00000004,
        SP_PLL_ON_CTRL: 0x00000004,
        SP_ROOT_UP_CTRL: 0x00000004,
        SP_GROUP_UP_CTRL: 0x00000004,
        SP_LPCG_ON_CTRL: 0x00000004,
        SP_SSAR_RESTORE_CTRL: 0x00000004,
    };

    /// Safe access to GPC_SET_POINT_CTRL
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
        let taken = GPC_SET_POINT_CTRL_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPC_SET_POINT_CTRL
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPC_SET_POINT_CTRL_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPC_SET_POINT_CTRL
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPC_SET_POINT_CTRL_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPC_SET_POINT_CTRL {
    /// The interrupts associated with GPC_SET_POINT_CTRL
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with GPC_SET_POINT_CTRL
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPC_SET_POINT_CTRL
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPC_SET_POINT_CTRL: *const RegisterBlock = 0x40c02000 as *const _;
