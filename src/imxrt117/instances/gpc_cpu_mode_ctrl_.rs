#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPC_CPU
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::gpc_cpu_mode_ctrl_::Instance;
pub use crate::imxrt117::peripherals::gpc_cpu_mode_ctrl_::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::gpc_cpu_mode_ctrl_::{
    CM_AUTHEN_CTRL, CM_INT_CTRL, CM_IRQ_WAKEUP_MASK_0, CM_IRQ_WAKEUP_MASK_1, CM_IRQ_WAKEUP_MASK_2,
    CM_IRQ_WAKEUP_MASK_3, CM_IRQ_WAKEUP_MASK_4, CM_IRQ_WAKEUP_MASK_5, CM_IRQ_WAKEUP_MASK_6,
    CM_IRQ_WAKEUP_MASK_7, CM_IRQ_WAKEUP_STAT_0, CM_IRQ_WAKEUP_STAT_1, CM_IRQ_WAKEUP_STAT_2,
    CM_IRQ_WAKEUP_STAT_3, CM_IRQ_WAKEUP_STAT_4, CM_IRQ_WAKEUP_STAT_5, CM_IRQ_WAKEUP_STAT_6,
    CM_IRQ_WAKEUP_STAT_7, CM_MISC, CM_MODE_CTRL, CM_MODE_STAT, CM_NON_IRQ_WAKEUP_MASK,
    CM_NON_IRQ_WAKEUP_STAT, CM_RUN_MODE_MAPPING, CM_SLEEP_ISO_CTRL, CM_SLEEP_LPCG_CTRL,
    CM_SLEEP_PLL_CTRL, CM_SLEEP_POWER_CTRL, CM_SLEEP_RESET_CTRL, CM_SLEEP_SSAR_CTRL,
    CM_SP0_MAPPING, CM_SP10_MAPPING, CM_SP11_MAPPING, CM_SP12_MAPPING, CM_SP13_MAPPING,
    CM_SP14_MAPPING, CM_SP15_MAPPING, CM_SP1_MAPPING, CM_SP2_MAPPING, CM_SP3_MAPPING,
    CM_SP4_MAPPING, CM_SP5_MAPPING, CM_SP6_MAPPING, CM_SP7_MAPPING, CM_SP8_MAPPING, CM_SP9_MAPPING,
    CM_SP_CTRL, CM_SP_STAT, CM_STBY_CTRL, CM_STOP_MODE_MAPPING, CM_SUSPEND_MODE_MAPPING,
    CM_WAIT_MODE_MAPPING, CM_WAKEUP_ISO_CTRL, CM_WAKEUP_LPCG_CTRL, CM_WAKEUP_PLL_CTRL,
    CM_WAKEUP_POWER_CTRL, CM_WAKEUP_RESET_CTRL, CM_WAKEUP_SSAR_CTRL,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The GPC_CPU_MODE_CTRL_0 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPC_CPU_MODE_CTRL_0 = Instance<0>;

/// The GPC_CPU_MODE_CTRL_0 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPC_CPU_MODE_CTRL_0 = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct GPC_CPU_MODE_CTRL_0 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPC_CPU_MODE_CTRL_0 {}
impl crate::Valid for GPC_CPU_MODE_CTRL_0 {
    fn take() -> Option<Self> {
        <GPC_CPU_MODE_CTRL_0>::take()
    }
    fn release(self) {
        <GPC_CPU_MODE_CTRL_0>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPC_CPU_MODE_CTRL_0>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPC_CPU_MODE_CTRL_0_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPC_CPU_MODE_CTRL_0 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPC_CPU_MODE_CTRL_0 {
    const INSTANCE: Self = Self {
        addr: 0x40c00000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in GPC_CPU_MODE_CTRL_0
    pub const reset: ResetValues = ResetValues {
        CM_AUTHEN_CTRL: 0x00000F00,
        CM_INT_CTRL: 0x00000007,
        CM_MISC: 0x00000006,
        CM_MODE_CTRL: 0x00000000,
        CM_MODE_STAT: 0x00000000,
        CM_IRQ_WAKEUP_MASK_0: 0x00000000,
        CM_IRQ_WAKEUP_MASK_1: 0x00000000,
        CM_IRQ_WAKEUP_MASK_2: 0x00000000,
        CM_IRQ_WAKEUP_MASK_3: 0x00000000,
        CM_IRQ_WAKEUP_MASK_4: 0x00000000,
        CM_IRQ_WAKEUP_MASK_5: 0x00000000,
        CM_IRQ_WAKEUP_MASK_6: 0x00000000,
        CM_IRQ_WAKEUP_MASK_7: 0x00000000,
        CM_NON_IRQ_WAKEUP_MASK: 0x00000001,
        CM_IRQ_WAKEUP_STAT_0: 0x00000000,
        CM_IRQ_WAKEUP_STAT_1: 0x00000000,
        CM_IRQ_WAKEUP_STAT_2: 0x00000000,
        CM_IRQ_WAKEUP_STAT_3: 0x00000000,
        CM_IRQ_WAKEUP_STAT_4: 0x00000000,
        CM_IRQ_WAKEUP_STAT_5: 0x00000000,
        CM_IRQ_WAKEUP_STAT_6: 0x00000000,
        CM_IRQ_WAKEUP_STAT_7: 0x00000000,
        CM_NON_IRQ_WAKEUP_STAT: 0x00000000,
        CM_SLEEP_SSAR_CTRL: 0x00000004,
        CM_SLEEP_LPCG_CTRL: 0x00000004,
        CM_SLEEP_PLL_CTRL: 0x00000004,
        CM_SLEEP_ISO_CTRL: 0x00000004,
        CM_SLEEP_RESET_CTRL: 0x00000004,
        CM_SLEEP_POWER_CTRL: 0x00000004,
        CM_WAKEUP_POWER_CTRL: 0x00000004,
        CM_WAKEUP_RESET_CTRL: 0x00000004,
        CM_WAKEUP_ISO_CTRL: 0x00000004,
        CM_WAKEUP_PLL_CTRL: 0x00000004,
        CM_WAKEUP_LPCG_CTRL: 0x00000004,
        CM_WAKEUP_SSAR_CTRL: 0x00000004,
        CM_SP_CTRL: 0x00000000,
        CM_SP_STAT: 0x00000000,
        CM_RUN_MODE_MAPPING: 0x0000FFFF,
        CM_WAIT_MODE_MAPPING: 0x0000FFFF,
        CM_STOP_MODE_MAPPING: 0x0000FFFF,
        CM_SUSPEND_MODE_MAPPING: 0x0000FFFF,
        CM_SP0_MAPPING: 0x0000FFFF,
        CM_SP1_MAPPING: 0x0000FFFF,
        CM_SP2_MAPPING: 0x0000FFFF,
        CM_SP3_MAPPING: 0x0000FFFF,
        CM_SP4_MAPPING: 0x0000FFFF,
        CM_SP5_MAPPING: 0x0000FFFF,
        CM_SP6_MAPPING: 0x0000FFFF,
        CM_SP7_MAPPING: 0x0000FFFF,
        CM_SP8_MAPPING: 0x0000FFFF,
        CM_SP9_MAPPING: 0x0000FFFF,
        CM_SP10_MAPPING: 0x0000FFFF,
        CM_SP11_MAPPING: 0x0000FFFF,
        CM_SP12_MAPPING: 0x0000FFFF,
        CM_SP13_MAPPING: 0x0000FFFF,
        CM_SP14_MAPPING: 0x0000FFFF,
        CM_SP15_MAPPING: 0x0000FFFF,
        CM_STBY_CTRL: 0x00000000,
    };

    /// Safe access to GPC_CPU_MODE_CTRL_0
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
        let taken = GPC_CPU_MODE_CTRL_0_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPC_CPU_MODE_CTRL_0
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPC_CPU_MODE_CTRL_0_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPC_CPU_MODE_CTRL_0
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPC_CPU_MODE_CTRL_0_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPC_CPU_MODE_CTRL_0 {
    /// The interrupts associated with GPC_CPU_MODE_CTRL_0
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with GPC_CPU_MODE_CTRL_0
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPC_CPU_MODE_CTRL_0
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPC_CPU_MODE_CTRL_0: *const RegisterBlock = 0x40c00000 as *const _;

/// The GPC_CPU_MODE_CTRL_1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type GPC_CPU_MODE_CTRL_1 = Instance<1>;

/// The GPC_CPU_MODE_CTRL_1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPC_CPU_MODE_CTRL_1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct GPC_CPU_MODE_CTRL_1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for GPC_CPU_MODE_CTRL_1 {}
impl crate::Valid for GPC_CPU_MODE_CTRL_1 {
    fn take() -> Option<Self> {
        <GPC_CPU_MODE_CTRL_1>::take()
    }
    fn release(self) {
        <GPC_CPU_MODE_CTRL_1>::release(self);
    }
    unsafe fn steal() -> Self {
        <GPC_CPU_MODE_CTRL_1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPC_CPU_MODE_CTRL_1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPC_CPU_MODE_CTRL_1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPC_CPU_MODE_CTRL_1 {
    const INSTANCE: Self = Self {
        addr: 0x40c00800,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in GPC_CPU_MODE_CTRL_1
    pub const reset: ResetValues = ResetValues {
        CM_AUTHEN_CTRL: 0x00000F00,
        CM_INT_CTRL: 0x00000007,
        CM_MISC: 0x00000006,
        CM_MODE_CTRL: 0x00000000,
        CM_MODE_STAT: 0x00000000,
        CM_IRQ_WAKEUP_MASK_0: 0x00000000,
        CM_IRQ_WAKEUP_MASK_1: 0x00000000,
        CM_IRQ_WAKEUP_MASK_2: 0x00000000,
        CM_IRQ_WAKEUP_MASK_3: 0x00000000,
        CM_IRQ_WAKEUP_MASK_4: 0x00000000,
        CM_IRQ_WAKEUP_MASK_5: 0x00000000,
        CM_IRQ_WAKEUP_MASK_6: 0x00000000,
        CM_IRQ_WAKEUP_MASK_7: 0x00000000,
        CM_NON_IRQ_WAKEUP_MASK: 0x00000001,
        CM_IRQ_WAKEUP_STAT_0: 0x00000000,
        CM_IRQ_WAKEUP_STAT_1: 0x00000000,
        CM_IRQ_WAKEUP_STAT_2: 0x00000000,
        CM_IRQ_WAKEUP_STAT_3: 0x00000000,
        CM_IRQ_WAKEUP_STAT_4: 0x00000000,
        CM_IRQ_WAKEUP_STAT_5: 0x00000000,
        CM_IRQ_WAKEUP_STAT_6: 0x00000000,
        CM_IRQ_WAKEUP_STAT_7: 0x00000000,
        CM_NON_IRQ_WAKEUP_STAT: 0x00000000,
        CM_SLEEP_SSAR_CTRL: 0x00000004,
        CM_SLEEP_LPCG_CTRL: 0x00000004,
        CM_SLEEP_PLL_CTRL: 0x00000004,
        CM_SLEEP_ISO_CTRL: 0x00000004,
        CM_SLEEP_RESET_CTRL: 0x00000004,
        CM_SLEEP_POWER_CTRL: 0x00000004,
        CM_WAKEUP_POWER_CTRL: 0x00000004,
        CM_WAKEUP_RESET_CTRL: 0x00000004,
        CM_WAKEUP_ISO_CTRL: 0x00000004,
        CM_WAKEUP_PLL_CTRL: 0x00000004,
        CM_WAKEUP_LPCG_CTRL: 0x00000004,
        CM_WAKEUP_SSAR_CTRL: 0x00000004,
        CM_SP_CTRL: 0x00000000,
        CM_SP_STAT: 0x00000000,
        CM_RUN_MODE_MAPPING: 0x0000FFFF,
        CM_WAIT_MODE_MAPPING: 0x0000FFFF,
        CM_STOP_MODE_MAPPING: 0x0000FFFF,
        CM_SUSPEND_MODE_MAPPING: 0x0000FFFF,
        CM_SP0_MAPPING: 0x0000FFFF,
        CM_SP1_MAPPING: 0x0000FFFF,
        CM_SP2_MAPPING: 0x0000FFFF,
        CM_SP3_MAPPING: 0x0000FFFF,
        CM_SP4_MAPPING: 0x0000FFFF,
        CM_SP5_MAPPING: 0x0000FFFF,
        CM_SP6_MAPPING: 0x0000FFFF,
        CM_SP7_MAPPING: 0x0000FFFF,
        CM_SP8_MAPPING: 0x0000FFFF,
        CM_SP9_MAPPING: 0x0000FFFF,
        CM_SP10_MAPPING: 0x0000FFFF,
        CM_SP11_MAPPING: 0x0000FFFF,
        CM_SP12_MAPPING: 0x0000FFFF,
        CM_SP13_MAPPING: 0x0000FFFF,
        CM_SP14_MAPPING: 0x0000FFFF,
        CM_SP15_MAPPING: 0x0000FFFF,
        CM_STBY_CTRL: 0x00000000,
    };

    /// Safe access to GPC_CPU_MODE_CTRL_1
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
        let taken = GPC_CPU_MODE_CTRL_1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPC_CPU_MODE_CTRL_1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPC_CPU_MODE_CTRL_1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPC_CPU_MODE_CTRL_1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPC_CPU_MODE_CTRL_1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl GPC_CPU_MODE_CTRL_1 {
    /// The interrupts associated with GPC_CPU_MODE_CTRL_1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with GPC_CPU_MODE_CTRL_1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPC_CPU_MODE_CTRL_1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPC_CPU_MODE_CTRL_1: *const RegisterBlock = 0x40c00800 as *const _;
