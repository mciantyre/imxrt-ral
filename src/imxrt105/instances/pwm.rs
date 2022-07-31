#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PWM
//!
//! Used by: imxrt1051, imxrt1052

pub use crate::imxrt105::peripherals::pwm::Instance;
pub use crate::imxrt105::peripherals::pwm::{RegisterBlock, ResetValues};

pub use crate::imxrt105::peripherals::pwm::{
    DTSRCSEL, FCTRL0, FCTRL20, FFILT0, FSTS0, FTST0, MASK, MCTRL, MCTRL2, OUTEN, SMCAPTCOMPA_0,
    SMCAPTCOMPA_1, SMCAPTCOMPA_2, SMCAPTCOMPA_3, SMCAPTCOMPB_0, SMCAPTCOMPB_1, SMCAPTCOMPB_2,
    SMCAPTCOMPB_3, SMCAPTCOMPX_0, SMCAPTCOMPX_1, SMCAPTCOMPX_2, SMCAPTCOMPX_3, SMCAPTCTRLA_0,
    SMCAPTCTRLA_1, SMCAPTCTRLA_2, SMCAPTCTRLA_3, SMCAPTCTRLB_0, SMCAPTCTRLB_1, SMCAPTCTRLB_2,
    SMCAPTCTRLB_3, SMCAPTCTRLX_0, SMCAPTCTRLX_1, SMCAPTCTRLX_2, SMCAPTCTRLX_3, SMCNT_0, SMCNT_1,
    SMCNT_2, SMCNT_3, SMCTRL2_0, SMCTRL2_1, SMCTRL2_2, SMCTRL2_3, SMCTRL_0, SMCTRL_1, SMCTRL_2,
    SMCTRL_3, SMCVAL0CYC_0, SMCVAL0CYC_1, SMCVAL0CYC_2, SMCVAL0CYC_3, SMCVAL0_0, SMCVAL0_1,
    SMCVAL0_2, SMCVAL0_3, SMCVAL1CYC_0, SMCVAL1CYC_1, SMCVAL1CYC_2, SMCVAL1CYC_3, SMCVAL1_0,
    SMCVAL1_1, SMCVAL1_2, SMCVAL1_3, SMCVAL2CYC_0, SMCVAL2CYC_1, SMCVAL2CYC_2, SMCVAL2CYC_3,
    SMCVAL2_0, SMCVAL2_1, SMCVAL2_2, SMCVAL2_3, SMCVAL3CYC_0, SMCVAL3CYC_1, SMCVAL3CYC_2,
    SMCVAL3CYC_3, SMCVAL3_0, SMCVAL3_1, SMCVAL3_2, SMCVAL3_3, SMCVAL4CYC_0, SMCVAL4CYC_1,
    SMCVAL4CYC_2, SMCVAL4CYC_3, SMCVAL4_0, SMCVAL4_1, SMCVAL4_2, SMCVAL4_3, SMCVAL5CYC_0,
    SMCVAL5CYC_1, SMCVAL5CYC_2, SMCVAL5CYC_3, SMCVAL5_0, SMCVAL5_1, SMCVAL5_2, SMCVAL5_3,
    SMDISMAP0_0, SMDISMAP0_1, SMDISMAP0_2, SMDISMAP0_3, SMDISMAP1_0, SMDISMAP1_1, SMDISMAP1_2,
    SMDISMAP1_3, SMDMAEN_0, SMDMAEN_1, SMDMAEN_2, SMDMAEN_3, SMDTCNT0_0, SMDTCNT0_1, SMDTCNT0_2,
    SMDTCNT0_3, SMDTCNT1_0, SMDTCNT1_1, SMDTCNT1_2, SMDTCNT1_3, SMFRACVAL1_0, SMFRACVAL1_1,
    SMFRACVAL1_2, SMFRACVAL1_3, SMFRACVAL2_0, SMFRACVAL2_1, SMFRACVAL2_2, SMFRACVAL2_3,
    SMFRACVAL3_0, SMFRACVAL3_1, SMFRACVAL3_2, SMFRACVAL3_3, SMFRACVAL4_0, SMFRACVAL4_1,
    SMFRACVAL4_2, SMFRACVAL4_3, SMFRACVAL5_0, SMFRACVAL5_1, SMFRACVAL5_2, SMFRACVAL5_3, SMFRCTRL_0,
    SMFRCTRL_1, SMFRCTRL_2, SMFRCTRL_3, SMINIT_0, SMINIT_1, SMINIT_2, SMINIT_3, SMINTEN_0,
    SMINTEN_1, SMINTEN_2, SMINTEN_3, SMOCTRL_0, SMOCTRL_1, SMOCTRL_2, SMOCTRL_3, SMSTS_0, SMSTS_1,
    SMSTS_2, SMSTS_3, SMTCTRL_0, SMTCTRL_1, SMTCTRL_2, SMTCTRL_3, SMVAL0_0, SMVAL0_1, SMVAL0_2,
    SMVAL0_3, SMVAL1_0, SMVAL1_1, SMVAL1_2, SMVAL1_3, SMVAL2_0, SMVAL2_1, SMVAL2_2, SMVAL2_3,
    SMVAL3_0, SMVAL3_1, SMVAL3_2, SMVAL3_3, SMVAL4_0, SMVAL4_1, SMVAL4_2, SMVAL4_3, SMVAL5_0,
    SMVAL5_1, SMVAL5_2, SMVAL5_3, SWCOUT,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The PWM1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PWM1 = Instance<1>;

/// The PWM1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PWM1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct PWM1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PWM1 {}
impl crate::Valid for PWM1 {
    fn take() -> Option<Self> {
        <PWM1>::take()
    }
    fn release(self) {
        <PWM1>::release(self);
    }
    unsafe fn steal() -> Self {
        <PWM1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PWM1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PWM1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PWM1 {
    const INSTANCE: Self = Self {
        addr: 0x403dc000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::PWM1_0,
            crate::interrupt::PWM1_1,
            crate::interrupt::PWM1_2,
            crate::interrupt::PWM1_3,
            crate::interrupt::PWM1_FAULT,
        ],
    };

    /// Reset values for each field in PWM1
    pub const reset: ResetValues = ResetValues {
        OUTEN: 0x00000000,
        MASK: 0x00000000,
        SWCOUT: 0x00000000,
        DTSRCSEL: 0x00000000,
        MCTRL: 0x00000000,
        MCTRL2: 0x00000000,
        FCTRL0: 0x00000000,
        FSTS0: 0x00000000,
        FFILT0: 0x00000000,
        FTST0: 0x00000000,
        FCTRL20: 0x00000000,
        SMCNT_0: 0x00000000,
        SMINIT_0: 0x00000000,
        SMCTRL2_0: 0x00000000,
        SMCTRL_0: 0x00000400,
        SMVAL0_0: 0x00000000,
        SMFRACVAL1_0: 0x00000000,
        SMVAL1_0: 0x00000000,
        SMFRACVAL2_0: 0x00000000,
        SMVAL2_0: 0x00000000,
        SMFRACVAL3_0: 0x00000000,
        SMVAL3_0: 0x00000000,
        SMFRACVAL4_0: 0x00000000,
        SMVAL4_0: 0x00000000,
        SMFRACVAL5_0: 0x00000000,
        SMVAL5_0: 0x00000000,
        SMFRCTRL_0: 0x00000000,
        SMOCTRL_0: 0x00000000,
        SMSTS_0: 0x00000000,
        SMINTEN_0: 0x00000000,
        SMDMAEN_0: 0x00000000,
        SMTCTRL_0: 0x00000000,
        SMDISMAP0_0: 0x0000FFFF,
        SMDISMAP1_0: 0x0000FFFF,
        SMDTCNT0_0: 0x000007FF,
        SMDTCNT1_0: 0x000007FF,
        SMCAPTCTRLA_0: 0x00000000,
        SMCAPTCOMPA_0: 0x00000000,
        SMCAPTCTRLB_0: 0x00000000,
        SMCAPTCOMPB_0: 0x00000000,
        SMCAPTCTRLX_0: 0x00000000,
        SMCAPTCOMPX_0: 0x00000000,
        SMCVAL0_0: 0x00000000,
        SMCVAL0CYC_0: 0x00000000,
        SMCVAL1_0: 0x00000000,
        SMCVAL1CYC_0: 0x00000000,
        SMCVAL2_0: 0x00000000,
        SMCVAL2CYC_0: 0x00000000,
        SMCVAL3_0: 0x00000000,
        SMCVAL3CYC_0: 0x00000000,
        SMCVAL4_0: 0x00000000,
        SMCVAL4CYC_0: 0x00000000,
        SMCVAL5_0: 0x00000000,
        SMCVAL5CYC_0: 0x00000000,
        SMCNT_1: 0x00000000,
        SMINIT_1: 0x00000000,
        SMCTRL2_1: 0x00000000,
        SMCTRL_1: 0x00000400,
        SMVAL0_1: 0x00000000,
        SMFRACVAL1_1: 0x00000000,
        SMVAL1_1: 0x00000000,
        SMFRACVAL2_1: 0x00000000,
        SMVAL2_1: 0x00000000,
        SMFRACVAL3_1: 0x00000000,
        SMVAL3_1: 0x00000000,
        SMFRACVAL4_1: 0x00000000,
        SMVAL4_1: 0x00000000,
        SMFRACVAL5_1: 0x00000000,
        SMVAL5_1: 0x00000000,
        SMFRCTRL_1: 0x00000000,
        SMOCTRL_1: 0x00000000,
        SMSTS_1: 0x00000000,
        SMINTEN_1: 0x00000000,
        SMDMAEN_1: 0x00000000,
        SMTCTRL_1: 0x00000000,
        SMDISMAP0_1: 0x0000FFFF,
        SMDISMAP1_1: 0x0000FFFF,
        SMDTCNT0_1: 0x000007FF,
        SMDTCNT1_1: 0x000007FF,
        SMCAPTCTRLA_1: 0x00000000,
        SMCAPTCOMPA_1: 0x00000000,
        SMCAPTCTRLB_1: 0x00000000,
        SMCAPTCOMPB_1: 0x00000000,
        SMCAPTCTRLX_1: 0x00000000,
        SMCAPTCOMPX_1: 0x00000000,
        SMCVAL0_1: 0x00000000,
        SMCVAL0CYC_1: 0x00000000,
        SMCVAL1_1: 0x00000000,
        SMCVAL1CYC_1: 0x00000000,
        SMCVAL2_1: 0x00000000,
        SMCVAL2CYC_1: 0x00000000,
        SMCVAL3_1: 0x00000000,
        SMCVAL3CYC_1: 0x00000000,
        SMCVAL4_1: 0x00000000,
        SMCVAL4CYC_1: 0x00000000,
        SMCVAL5_1: 0x00000000,
        SMCVAL5CYC_1: 0x00000000,
        SMCNT_2: 0x00000000,
        SMINIT_2: 0x00000000,
        SMCTRL2_2: 0x00000000,
        SMCTRL_2: 0x00000400,
        SMVAL0_2: 0x00000000,
        SMFRACVAL1_2: 0x00000000,
        SMVAL1_2: 0x00000000,
        SMFRACVAL2_2: 0x00000000,
        SMVAL2_2: 0x00000000,
        SMFRACVAL3_2: 0x00000000,
        SMVAL3_2: 0x00000000,
        SMFRACVAL4_2: 0x00000000,
        SMVAL4_2: 0x00000000,
        SMFRACVAL5_2: 0x00000000,
        SMVAL5_2: 0x00000000,
        SMFRCTRL_2: 0x00000000,
        SMOCTRL_2: 0x00000000,
        SMSTS_2: 0x00000000,
        SMINTEN_2: 0x00000000,
        SMDMAEN_2: 0x00000000,
        SMTCTRL_2: 0x00000000,
        SMDISMAP0_2: 0x0000FFFF,
        SMDISMAP1_2: 0x0000FFFF,
        SMDTCNT0_2: 0x000007FF,
        SMDTCNT1_2: 0x000007FF,
        SMCAPTCTRLA_2: 0x00000000,
        SMCAPTCOMPA_2: 0x00000000,
        SMCAPTCTRLB_2: 0x00000000,
        SMCAPTCOMPB_2: 0x00000000,
        SMCAPTCTRLX_2: 0x00000000,
        SMCAPTCOMPX_2: 0x00000000,
        SMCVAL0_2: 0x00000000,
        SMCVAL0CYC_2: 0x00000000,
        SMCVAL1_2: 0x00000000,
        SMCVAL1CYC_2: 0x00000000,
        SMCVAL2_2: 0x00000000,
        SMCVAL2CYC_2: 0x00000000,
        SMCVAL3_2: 0x00000000,
        SMCVAL3CYC_2: 0x00000000,
        SMCVAL4_2: 0x00000000,
        SMCVAL4CYC_2: 0x00000000,
        SMCVAL5_2: 0x00000000,
        SMCVAL5CYC_2: 0x00000000,
        SMCNT_3: 0x00000000,
        SMINIT_3: 0x00000000,
        SMCTRL2_3: 0x00000000,
        SMCTRL_3: 0x00000400,
        SMVAL0_3: 0x00000000,
        SMFRACVAL1_3: 0x00000000,
        SMVAL1_3: 0x00000000,
        SMFRACVAL2_3: 0x00000000,
        SMVAL2_3: 0x00000000,
        SMFRACVAL3_3: 0x00000000,
        SMVAL3_3: 0x00000000,
        SMFRACVAL4_3: 0x00000000,
        SMVAL4_3: 0x00000000,
        SMFRACVAL5_3: 0x00000000,
        SMVAL5_3: 0x00000000,
        SMFRCTRL_3: 0x00000000,
        SMOCTRL_3: 0x00000000,
        SMSTS_3: 0x00000000,
        SMINTEN_3: 0x00000000,
        SMDMAEN_3: 0x00000000,
        SMTCTRL_3: 0x00000000,
        SMDISMAP0_3: 0x0000FFFF,
        SMDISMAP1_3: 0x0000FFFF,
        SMDTCNT0_3: 0x000007FF,
        SMDTCNT1_3: 0x000007FF,
        SMCAPTCTRLA_3: 0x00000000,
        SMCAPTCOMPA_3: 0x00000000,
        SMCAPTCTRLB_3: 0x00000000,
        SMCAPTCOMPB_3: 0x00000000,
        SMCAPTCTRLX_3: 0x00000000,
        SMCAPTCOMPX_3: 0x00000000,
        SMCVAL0_3: 0x00000000,
        SMCVAL0CYC_3: 0x00000000,
        SMCVAL1_3: 0x00000000,
        SMCVAL1CYC_3: 0x00000000,
        SMCVAL2_3: 0x00000000,
        SMCVAL2CYC_3: 0x00000000,
        SMCVAL3_3: 0x00000000,
        SMCVAL3CYC_3: 0x00000000,
        SMCVAL4_3: 0x00000000,
        SMCVAL4CYC_3: 0x00000000,
        SMCVAL5_3: 0x00000000,
        SMCVAL5CYC_3: 0x00000000,
    };

    /// Safe access to PWM1
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
        let taken = PWM1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PWM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PWM1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PWM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PWM1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PWM1 {
    /// The interrupts associated with PWM1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 5] = [
        crate::interrupt::PWM1_0,
        crate::interrupt::PWM1_1,
        crate::interrupt::PWM1_2,
        crate::interrupt::PWM1_3,
        crate::interrupt::PWM1_FAULT,
    ];

    /// The interrupts associated with PWM1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PWM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWM1: *const RegisterBlock = 0x403dc000 as *const _;

/// The PWM2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PWM2 = Instance<2>;

/// The PWM2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PWM2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct PWM2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PWM2 {}
impl crate::Valid for PWM2 {
    fn take() -> Option<Self> {
        <PWM2>::take()
    }
    fn release(self) {
        <PWM2>::release(self);
    }
    unsafe fn steal() -> Self {
        <PWM2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PWM2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PWM2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PWM2 {
    const INSTANCE: Self = Self {
        addr: 0x403e0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::PWM2_0,
            crate::interrupt::PWM2_1,
            crate::interrupt::PWM2_2,
            crate::interrupt::PWM2_3,
            crate::interrupt::PWM2_FAULT,
        ],
    };

    /// Reset values for each field in PWM2
    pub const reset: ResetValues = ResetValues {
        OUTEN: 0x00000000,
        MASK: 0x00000000,
        SWCOUT: 0x00000000,
        DTSRCSEL: 0x00000000,
        MCTRL: 0x00000000,
        MCTRL2: 0x00000000,
        FCTRL0: 0x00000000,
        FSTS0: 0x00000000,
        FFILT0: 0x00000000,
        FTST0: 0x00000000,
        FCTRL20: 0x00000000,
        SMCNT_0: 0x00000000,
        SMINIT_0: 0x00000000,
        SMCTRL2_0: 0x00000000,
        SMCTRL_0: 0x00000400,
        SMVAL0_0: 0x00000000,
        SMFRACVAL1_0: 0x00000000,
        SMVAL1_0: 0x00000000,
        SMFRACVAL2_0: 0x00000000,
        SMVAL2_0: 0x00000000,
        SMFRACVAL3_0: 0x00000000,
        SMVAL3_0: 0x00000000,
        SMFRACVAL4_0: 0x00000000,
        SMVAL4_0: 0x00000000,
        SMFRACVAL5_0: 0x00000000,
        SMVAL5_0: 0x00000000,
        SMFRCTRL_0: 0x00000000,
        SMOCTRL_0: 0x00000000,
        SMSTS_0: 0x00000000,
        SMINTEN_0: 0x00000000,
        SMDMAEN_0: 0x00000000,
        SMTCTRL_0: 0x00000000,
        SMDISMAP0_0: 0x0000FFFF,
        SMDISMAP1_0: 0x0000FFFF,
        SMDTCNT0_0: 0x000007FF,
        SMDTCNT1_0: 0x000007FF,
        SMCAPTCTRLA_0: 0x00000000,
        SMCAPTCOMPA_0: 0x00000000,
        SMCAPTCTRLB_0: 0x00000000,
        SMCAPTCOMPB_0: 0x00000000,
        SMCAPTCTRLX_0: 0x00000000,
        SMCAPTCOMPX_0: 0x00000000,
        SMCVAL0_0: 0x00000000,
        SMCVAL0CYC_0: 0x00000000,
        SMCVAL1_0: 0x00000000,
        SMCVAL1CYC_0: 0x00000000,
        SMCVAL2_0: 0x00000000,
        SMCVAL2CYC_0: 0x00000000,
        SMCVAL3_0: 0x00000000,
        SMCVAL3CYC_0: 0x00000000,
        SMCVAL4_0: 0x00000000,
        SMCVAL4CYC_0: 0x00000000,
        SMCVAL5_0: 0x00000000,
        SMCVAL5CYC_0: 0x00000000,
        SMCNT_1: 0x00000000,
        SMINIT_1: 0x00000000,
        SMCTRL2_1: 0x00000000,
        SMCTRL_1: 0x00000400,
        SMVAL0_1: 0x00000000,
        SMFRACVAL1_1: 0x00000000,
        SMVAL1_1: 0x00000000,
        SMFRACVAL2_1: 0x00000000,
        SMVAL2_1: 0x00000000,
        SMFRACVAL3_1: 0x00000000,
        SMVAL3_1: 0x00000000,
        SMFRACVAL4_1: 0x00000000,
        SMVAL4_1: 0x00000000,
        SMFRACVAL5_1: 0x00000000,
        SMVAL5_1: 0x00000000,
        SMFRCTRL_1: 0x00000000,
        SMOCTRL_1: 0x00000000,
        SMSTS_1: 0x00000000,
        SMINTEN_1: 0x00000000,
        SMDMAEN_1: 0x00000000,
        SMTCTRL_1: 0x00000000,
        SMDISMAP0_1: 0x0000FFFF,
        SMDISMAP1_1: 0x0000FFFF,
        SMDTCNT0_1: 0x000007FF,
        SMDTCNT1_1: 0x000007FF,
        SMCAPTCTRLA_1: 0x00000000,
        SMCAPTCOMPA_1: 0x00000000,
        SMCAPTCTRLB_1: 0x00000000,
        SMCAPTCOMPB_1: 0x00000000,
        SMCAPTCTRLX_1: 0x00000000,
        SMCAPTCOMPX_1: 0x00000000,
        SMCVAL0_1: 0x00000000,
        SMCVAL0CYC_1: 0x00000000,
        SMCVAL1_1: 0x00000000,
        SMCVAL1CYC_1: 0x00000000,
        SMCVAL2_1: 0x00000000,
        SMCVAL2CYC_1: 0x00000000,
        SMCVAL3_1: 0x00000000,
        SMCVAL3CYC_1: 0x00000000,
        SMCVAL4_1: 0x00000000,
        SMCVAL4CYC_1: 0x00000000,
        SMCVAL5_1: 0x00000000,
        SMCVAL5CYC_1: 0x00000000,
        SMCNT_2: 0x00000000,
        SMINIT_2: 0x00000000,
        SMCTRL2_2: 0x00000000,
        SMCTRL_2: 0x00000400,
        SMVAL0_2: 0x00000000,
        SMFRACVAL1_2: 0x00000000,
        SMVAL1_2: 0x00000000,
        SMFRACVAL2_2: 0x00000000,
        SMVAL2_2: 0x00000000,
        SMFRACVAL3_2: 0x00000000,
        SMVAL3_2: 0x00000000,
        SMFRACVAL4_2: 0x00000000,
        SMVAL4_2: 0x00000000,
        SMFRACVAL5_2: 0x00000000,
        SMVAL5_2: 0x00000000,
        SMFRCTRL_2: 0x00000000,
        SMOCTRL_2: 0x00000000,
        SMSTS_2: 0x00000000,
        SMINTEN_2: 0x00000000,
        SMDMAEN_2: 0x00000000,
        SMTCTRL_2: 0x00000000,
        SMDISMAP0_2: 0x0000FFFF,
        SMDISMAP1_2: 0x0000FFFF,
        SMDTCNT0_2: 0x000007FF,
        SMDTCNT1_2: 0x000007FF,
        SMCAPTCTRLA_2: 0x00000000,
        SMCAPTCOMPA_2: 0x00000000,
        SMCAPTCTRLB_2: 0x00000000,
        SMCAPTCOMPB_2: 0x00000000,
        SMCAPTCTRLX_2: 0x00000000,
        SMCAPTCOMPX_2: 0x00000000,
        SMCVAL0_2: 0x00000000,
        SMCVAL0CYC_2: 0x00000000,
        SMCVAL1_2: 0x00000000,
        SMCVAL1CYC_2: 0x00000000,
        SMCVAL2_2: 0x00000000,
        SMCVAL2CYC_2: 0x00000000,
        SMCVAL3_2: 0x00000000,
        SMCVAL3CYC_2: 0x00000000,
        SMCVAL4_2: 0x00000000,
        SMCVAL4CYC_2: 0x00000000,
        SMCVAL5_2: 0x00000000,
        SMCVAL5CYC_2: 0x00000000,
        SMCNT_3: 0x00000000,
        SMINIT_3: 0x00000000,
        SMCTRL2_3: 0x00000000,
        SMCTRL_3: 0x00000400,
        SMVAL0_3: 0x00000000,
        SMFRACVAL1_3: 0x00000000,
        SMVAL1_3: 0x00000000,
        SMFRACVAL2_3: 0x00000000,
        SMVAL2_3: 0x00000000,
        SMFRACVAL3_3: 0x00000000,
        SMVAL3_3: 0x00000000,
        SMFRACVAL4_3: 0x00000000,
        SMVAL4_3: 0x00000000,
        SMFRACVAL5_3: 0x00000000,
        SMVAL5_3: 0x00000000,
        SMFRCTRL_3: 0x00000000,
        SMOCTRL_3: 0x00000000,
        SMSTS_3: 0x00000000,
        SMINTEN_3: 0x00000000,
        SMDMAEN_3: 0x00000000,
        SMTCTRL_3: 0x00000000,
        SMDISMAP0_3: 0x0000FFFF,
        SMDISMAP1_3: 0x0000FFFF,
        SMDTCNT0_3: 0x000007FF,
        SMDTCNT1_3: 0x000007FF,
        SMCAPTCTRLA_3: 0x00000000,
        SMCAPTCOMPA_3: 0x00000000,
        SMCAPTCTRLB_3: 0x00000000,
        SMCAPTCOMPB_3: 0x00000000,
        SMCAPTCTRLX_3: 0x00000000,
        SMCAPTCOMPX_3: 0x00000000,
        SMCVAL0_3: 0x00000000,
        SMCVAL0CYC_3: 0x00000000,
        SMCVAL1_3: 0x00000000,
        SMCVAL1CYC_3: 0x00000000,
        SMCVAL2_3: 0x00000000,
        SMCVAL2CYC_3: 0x00000000,
        SMCVAL3_3: 0x00000000,
        SMCVAL3CYC_3: 0x00000000,
        SMCVAL4_3: 0x00000000,
        SMCVAL4CYC_3: 0x00000000,
        SMCVAL5_3: 0x00000000,
        SMCVAL5CYC_3: 0x00000000,
    };

    /// Safe access to PWM2
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
        let taken = PWM2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PWM2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PWM2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PWM2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PWM2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PWM2 {
    /// The interrupts associated with PWM2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 5] = [
        crate::interrupt::PWM2_0,
        crate::interrupt::PWM2_1,
        crate::interrupt::PWM2_2,
        crate::interrupt::PWM2_3,
        crate::interrupt::PWM2_FAULT,
    ];

    /// The interrupts associated with PWM2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PWM2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWM2: *const RegisterBlock = 0x403e0000 as *const _;

/// The PWM3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PWM3 = Instance<3>;

/// The PWM3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PWM3 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct PWM3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PWM3 {}
impl crate::Valid for PWM3 {
    fn take() -> Option<Self> {
        <PWM3>::take()
    }
    fn release(self) {
        <PWM3>::release(self);
    }
    unsafe fn steal() -> Self {
        <PWM3>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PWM3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PWM3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PWM3 {
    const INSTANCE: Self = Self {
        addr: 0x403e4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::PWM3_0,
            crate::interrupt::PWM3_1,
            crate::interrupt::PWM3_2,
            crate::interrupt::PWM3_3,
            crate::interrupt::PWM3_FAULT,
        ],
    };

    /// Reset values for each field in PWM3
    pub const reset: ResetValues = ResetValues {
        OUTEN: 0x00000000,
        MASK: 0x00000000,
        SWCOUT: 0x00000000,
        DTSRCSEL: 0x00000000,
        MCTRL: 0x00000000,
        MCTRL2: 0x00000000,
        FCTRL0: 0x00000000,
        FSTS0: 0x00000000,
        FFILT0: 0x00000000,
        FTST0: 0x00000000,
        FCTRL20: 0x00000000,
        SMCNT_0: 0x00000000,
        SMINIT_0: 0x00000000,
        SMCTRL2_0: 0x00000000,
        SMCTRL_0: 0x00000400,
        SMVAL0_0: 0x00000000,
        SMFRACVAL1_0: 0x00000000,
        SMVAL1_0: 0x00000000,
        SMFRACVAL2_0: 0x00000000,
        SMVAL2_0: 0x00000000,
        SMFRACVAL3_0: 0x00000000,
        SMVAL3_0: 0x00000000,
        SMFRACVAL4_0: 0x00000000,
        SMVAL4_0: 0x00000000,
        SMFRACVAL5_0: 0x00000000,
        SMVAL5_0: 0x00000000,
        SMFRCTRL_0: 0x00000000,
        SMOCTRL_0: 0x00000000,
        SMSTS_0: 0x00000000,
        SMINTEN_0: 0x00000000,
        SMDMAEN_0: 0x00000000,
        SMTCTRL_0: 0x00000000,
        SMDISMAP0_0: 0x0000FFFF,
        SMDISMAP1_0: 0x0000FFFF,
        SMDTCNT0_0: 0x000007FF,
        SMDTCNT1_0: 0x000007FF,
        SMCAPTCTRLA_0: 0x00000000,
        SMCAPTCOMPA_0: 0x00000000,
        SMCAPTCTRLB_0: 0x00000000,
        SMCAPTCOMPB_0: 0x00000000,
        SMCAPTCTRLX_0: 0x00000000,
        SMCAPTCOMPX_0: 0x00000000,
        SMCVAL0_0: 0x00000000,
        SMCVAL0CYC_0: 0x00000000,
        SMCVAL1_0: 0x00000000,
        SMCVAL1CYC_0: 0x00000000,
        SMCVAL2_0: 0x00000000,
        SMCVAL2CYC_0: 0x00000000,
        SMCVAL3_0: 0x00000000,
        SMCVAL3CYC_0: 0x00000000,
        SMCVAL4_0: 0x00000000,
        SMCVAL4CYC_0: 0x00000000,
        SMCVAL5_0: 0x00000000,
        SMCVAL5CYC_0: 0x00000000,
        SMCNT_1: 0x00000000,
        SMINIT_1: 0x00000000,
        SMCTRL2_1: 0x00000000,
        SMCTRL_1: 0x00000400,
        SMVAL0_1: 0x00000000,
        SMFRACVAL1_1: 0x00000000,
        SMVAL1_1: 0x00000000,
        SMFRACVAL2_1: 0x00000000,
        SMVAL2_1: 0x00000000,
        SMFRACVAL3_1: 0x00000000,
        SMVAL3_1: 0x00000000,
        SMFRACVAL4_1: 0x00000000,
        SMVAL4_1: 0x00000000,
        SMFRACVAL5_1: 0x00000000,
        SMVAL5_1: 0x00000000,
        SMFRCTRL_1: 0x00000000,
        SMOCTRL_1: 0x00000000,
        SMSTS_1: 0x00000000,
        SMINTEN_1: 0x00000000,
        SMDMAEN_1: 0x00000000,
        SMTCTRL_1: 0x00000000,
        SMDISMAP0_1: 0x0000FFFF,
        SMDISMAP1_1: 0x0000FFFF,
        SMDTCNT0_1: 0x000007FF,
        SMDTCNT1_1: 0x000007FF,
        SMCAPTCTRLA_1: 0x00000000,
        SMCAPTCOMPA_1: 0x00000000,
        SMCAPTCTRLB_1: 0x00000000,
        SMCAPTCOMPB_1: 0x00000000,
        SMCAPTCTRLX_1: 0x00000000,
        SMCAPTCOMPX_1: 0x00000000,
        SMCVAL0_1: 0x00000000,
        SMCVAL0CYC_1: 0x00000000,
        SMCVAL1_1: 0x00000000,
        SMCVAL1CYC_1: 0x00000000,
        SMCVAL2_1: 0x00000000,
        SMCVAL2CYC_1: 0x00000000,
        SMCVAL3_1: 0x00000000,
        SMCVAL3CYC_1: 0x00000000,
        SMCVAL4_1: 0x00000000,
        SMCVAL4CYC_1: 0x00000000,
        SMCVAL5_1: 0x00000000,
        SMCVAL5CYC_1: 0x00000000,
        SMCNT_2: 0x00000000,
        SMINIT_2: 0x00000000,
        SMCTRL2_2: 0x00000000,
        SMCTRL_2: 0x00000400,
        SMVAL0_2: 0x00000000,
        SMFRACVAL1_2: 0x00000000,
        SMVAL1_2: 0x00000000,
        SMFRACVAL2_2: 0x00000000,
        SMVAL2_2: 0x00000000,
        SMFRACVAL3_2: 0x00000000,
        SMVAL3_2: 0x00000000,
        SMFRACVAL4_2: 0x00000000,
        SMVAL4_2: 0x00000000,
        SMFRACVAL5_2: 0x00000000,
        SMVAL5_2: 0x00000000,
        SMFRCTRL_2: 0x00000000,
        SMOCTRL_2: 0x00000000,
        SMSTS_2: 0x00000000,
        SMINTEN_2: 0x00000000,
        SMDMAEN_2: 0x00000000,
        SMTCTRL_2: 0x00000000,
        SMDISMAP0_2: 0x0000FFFF,
        SMDISMAP1_2: 0x0000FFFF,
        SMDTCNT0_2: 0x000007FF,
        SMDTCNT1_2: 0x000007FF,
        SMCAPTCTRLA_2: 0x00000000,
        SMCAPTCOMPA_2: 0x00000000,
        SMCAPTCTRLB_2: 0x00000000,
        SMCAPTCOMPB_2: 0x00000000,
        SMCAPTCTRLX_2: 0x00000000,
        SMCAPTCOMPX_2: 0x00000000,
        SMCVAL0_2: 0x00000000,
        SMCVAL0CYC_2: 0x00000000,
        SMCVAL1_2: 0x00000000,
        SMCVAL1CYC_2: 0x00000000,
        SMCVAL2_2: 0x00000000,
        SMCVAL2CYC_2: 0x00000000,
        SMCVAL3_2: 0x00000000,
        SMCVAL3CYC_2: 0x00000000,
        SMCVAL4_2: 0x00000000,
        SMCVAL4CYC_2: 0x00000000,
        SMCVAL5_2: 0x00000000,
        SMCVAL5CYC_2: 0x00000000,
        SMCNT_3: 0x00000000,
        SMINIT_3: 0x00000000,
        SMCTRL2_3: 0x00000000,
        SMCTRL_3: 0x00000400,
        SMVAL0_3: 0x00000000,
        SMFRACVAL1_3: 0x00000000,
        SMVAL1_3: 0x00000000,
        SMFRACVAL2_3: 0x00000000,
        SMVAL2_3: 0x00000000,
        SMFRACVAL3_3: 0x00000000,
        SMVAL3_3: 0x00000000,
        SMFRACVAL4_3: 0x00000000,
        SMVAL4_3: 0x00000000,
        SMFRACVAL5_3: 0x00000000,
        SMVAL5_3: 0x00000000,
        SMFRCTRL_3: 0x00000000,
        SMOCTRL_3: 0x00000000,
        SMSTS_3: 0x00000000,
        SMINTEN_3: 0x00000000,
        SMDMAEN_3: 0x00000000,
        SMTCTRL_3: 0x00000000,
        SMDISMAP0_3: 0x0000FFFF,
        SMDISMAP1_3: 0x0000FFFF,
        SMDTCNT0_3: 0x000007FF,
        SMDTCNT1_3: 0x000007FF,
        SMCAPTCTRLA_3: 0x00000000,
        SMCAPTCOMPA_3: 0x00000000,
        SMCAPTCTRLB_3: 0x00000000,
        SMCAPTCOMPB_3: 0x00000000,
        SMCAPTCTRLX_3: 0x00000000,
        SMCAPTCOMPX_3: 0x00000000,
        SMCVAL0_3: 0x00000000,
        SMCVAL0CYC_3: 0x00000000,
        SMCVAL1_3: 0x00000000,
        SMCVAL1CYC_3: 0x00000000,
        SMCVAL2_3: 0x00000000,
        SMCVAL2CYC_3: 0x00000000,
        SMCVAL3_3: 0x00000000,
        SMCVAL3CYC_3: 0x00000000,
        SMCVAL4_3: 0x00000000,
        SMCVAL4CYC_3: 0x00000000,
        SMCVAL5_3: 0x00000000,
        SMCVAL5CYC_3: 0x00000000,
    };

    /// Safe access to PWM3
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
        let taken = PWM3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PWM3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PWM3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PWM3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PWM3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PWM3 {
    /// The interrupts associated with PWM3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 5] = [
        crate::interrupt::PWM3_0,
        crate::interrupt::PWM3_1,
        crate::interrupt::PWM3_2,
        crate::interrupt::PWM3_3,
        crate::interrupt::PWM3_FAULT,
    ];

    /// The interrupts associated with PWM3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PWM3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWM3: *const RegisterBlock = 0x403e4000 as *const _;

/// The PWM4 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PWM4 = Instance<4>;

/// The PWM4 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PWM4 = Instance<4>;
/// ```
#[cfg(feature = "doc")]
pub struct PWM4 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PWM4 {}
impl crate::Valid for PWM4 {
    fn take() -> Option<Self> {
        <PWM4>::take()
    }
    fn release(self) {
        <PWM4>::release(self);
    }
    unsafe fn steal() -> Self {
        <PWM4>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PWM4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PWM4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl PWM4 {
    const INSTANCE: Self = Self {
        addr: 0x403e8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::PWM4_0,
            crate::interrupt::PWM4_1,
            crate::interrupt::PWM4_2,
            crate::interrupt::PWM4_3,
            crate::interrupt::PWM4_FAULT,
        ],
    };

    /// Reset values for each field in PWM4
    pub const reset: ResetValues = ResetValues {
        OUTEN: 0x00000000,
        MASK: 0x00000000,
        SWCOUT: 0x00000000,
        DTSRCSEL: 0x00000000,
        MCTRL: 0x00000000,
        MCTRL2: 0x00000000,
        FCTRL0: 0x00000000,
        FSTS0: 0x00000000,
        FFILT0: 0x00000000,
        FTST0: 0x00000000,
        FCTRL20: 0x00000000,
        SMCNT_0: 0x00000000,
        SMINIT_0: 0x00000000,
        SMCTRL2_0: 0x00000000,
        SMCTRL_0: 0x00000400,
        SMVAL0_0: 0x00000000,
        SMFRACVAL1_0: 0x00000000,
        SMVAL1_0: 0x00000000,
        SMFRACVAL2_0: 0x00000000,
        SMVAL2_0: 0x00000000,
        SMFRACVAL3_0: 0x00000000,
        SMVAL3_0: 0x00000000,
        SMFRACVAL4_0: 0x00000000,
        SMVAL4_0: 0x00000000,
        SMFRACVAL5_0: 0x00000000,
        SMVAL5_0: 0x00000000,
        SMFRCTRL_0: 0x00000000,
        SMOCTRL_0: 0x00000000,
        SMSTS_0: 0x00000000,
        SMINTEN_0: 0x00000000,
        SMDMAEN_0: 0x00000000,
        SMTCTRL_0: 0x00000000,
        SMDISMAP0_0: 0x0000FFFF,
        SMDISMAP1_0: 0x0000FFFF,
        SMDTCNT0_0: 0x000007FF,
        SMDTCNT1_0: 0x000007FF,
        SMCAPTCTRLA_0: 0x00000000,
        SMCAPTCOMPA_0: 0x00000000,
        SMCAPTCTRLB_0: 0x00000000,
        SMCAPTCOMPB_0: 0x00000000,
        SMCAPTCTRLX_0: 0x00000000,
        SMCAPTCOMPX_0: 0x00000000,
        SMCVAL0_0: 0x00000000,
        SMCVAL0CYC_0: 0x00000000,
        SMCVAL1_0: 0x00000000,
        SMCVAL1CYC_0: 0x00000000,
        SMCVAL2_0: 0x00000000,
        SMCVAL2CYC_0: 0x00000000,
        SMCVAL3_0: 0x00000000,
        SMCVAL3CYC_0: 0x00000000,
        SMCVAL4_0: 0x00000000,
        SMCVAL4CYC_0: 0x00000000,
        SMCVAL5_0: 0x00000000,
        SMCVAL5CYC_0: 0x00000000,
        SMCNT_1: 0x00000000,
        SMINIT_1: 0x00000000,
        SMCTRL2_1: 0x00000000,
        SMCTRL_1: 0x00000400,
        SMVAL0_1: 0x00000000,
        SMFRACVAL1_1: 0x00000000,
        SMVAL1_1: 0x00000000,
        SMFRACVAL2_1: 0x00000000,
        SMVAL2_1: 0x00000000,
        SMFRACVAL3_1: 0x00000000,
        SMVAL3_1: 0x00000000,
        SMFRACVAL4_1: 0x00000000,
        SMVAL4_1: 0x00000000,
        SMFRACVAL5_1: 0x00000000,
        SMVAL5_1: 0x00000000,
        SMFRCTRL_1: 0x00000000,
        SMOCTRL_1: 0x00000000,
        SMSTS_1: 0x00000000,
        SMINTEN_1: 0x00000000,
        SMDMAEN_1: 0x00000000,
        SMTCTRL_1: 0x00000000,
        SMDISMAP0_1: 0x0000FFFF,
        SMDISMAP1_1: 0x0000FFFF,
        SMDTCNT0_1: 0x000007FF,
        SMDTCNT1_1: 0x000007FF,
        SMCAPTCTRLA_1: 0x00000000,
        SMCAPTCOMPA_1: 0x00000000,
        SMCAPTCTRLB_1: 0x00000000,
        SMCAPTCOMPB_1: 0x00000000,
        SMCAPTCTRLX_1: 0x00000000,
        SMCAPTCOMPX_1: 0x00000000,
        SMCVAL0_1: 0x00000000,
        SMCVAL0CYC_1: 0x00000000,
        SMCVAL1_1: 0x00000000,
        SMCVAL1CYC_1: 0x00000000,
        SMCVAL2_1: 0x00000000,
        SMCVAL2CYC_1: 0x00000000,
        SMCVAL3_1: 0x00000000,
        SMCVAL3CYC_1: 0x00000000,
        SMCVAL4_1: 0x00000000,
        SMCVAL4CYC_1: 0x00000000,
        SMCVAL5_1: 0x00000000,
        SMCVAL5CYC_1: 0x00000000,
        SMCNT_2: 0x00000000,
        SMINIT_2: 0x00000000,
        SMCTRL2_2: 0x00000000,
        SMCTRL_2: 0x00000400,
        SMVAL0_2: 0x00000000,
        SMFRACVAL1_2: 0x00000000,
        SMVAL1_2: 0x00000000,
        SMFRACVAL2_2: 0x00000000,
        SMVAL2_2: 0x00000000,
        SMFRACVAL3_2: 0x00000000,
        SMVAL3_2: 0x00000000,
        SMFRACVAL4_2: 0x00000000,
        SMVAL4_2: 0x00000000,
        SMFRACVAL5_2: 0x00000000,
        SMVAL5_2: 0x00000000,
        SMFRCTRL_2: 0x00000000,
        SMOCTRL_2: 0x00000000,
        SMSTS_2: 0x00000000,
        SMINTEN_2: 0x00000000,
        SMDMAEN_2: 0x00000000,
        SMTCTRL_2: 0x00000000,
        SMDISMAP0_2: 0x0000FFFF,
        SMDISMAP1_2: 0x0000FFFF,
        SMDTCNT0_2: 0x000007FF,
        SMDTCNT1_2: 0x000007FF,
        SMCAPTCTRLA_2: 0x00000000,
        SMCAPTCOMPA_2: 0x00000000,
        SMCAPTCTRLB_2: 0x00000000,
        SMCAPTCOMPB_2: 0x00000000,
        SMCAPTCTRLX_2: 0x00000000,
        SMCAPTCOMPX_2: 0x00000000,
        SMCVAL0_2: 0x00000000,
        SMCVAL0CYC_2: 0x00000000,
        SMCVAL1_2: 0x00000000,
        SMCVAL1CYC_2: 0x00000000,
        SMCVAL2_2: 0x00000000,
        SMCVAL2CYC_2: 0x00000000,
        SMCVAL3_2: 0x00000000,
        SMCVAL3CYC_2: 0x00000000,
        SMCVAL4_2: 0x00000000,
        SMCVAL4CYC_2: 0x00000000,
        SMCVAL5_2: 0x00000000,
        SMCVAL5CYC_2: 0x00000000,
        SMCNT_3: 0x00000000,
        SMINIT_3: 0x00000000,
        SMCTRL2_3: 0x00000000,
        SMCTRL_3: 0x00000400,
        SMVAL0_3: 0x00000000,
        SMFRACVAL1_3: 0x00000000,
        SMVAL1_3: 0x00000000,
        SMFRACVAL2_3: 0x00000000,
        SMVAL2_3: 0x00000000,
        SMFRACVAL3_3: 0x00000000,
        SMVAL3_3: 0x00000000,
        SMFRACVAL4_3: 0x00000000,
        SMVAL4_3: 0x00000000,
        SMFRACVAL5_3: 0x00000000,
        SMVAL5_3: 0x00000000,
        SMFRCTRL_3: 0x00000000,
        SMOCTRL_3: 0x00000000,
        SMSTS_3: 0x00000000,
        SMINTEN_3: 0x00000000,
        SMDMAEN_3: 0x00000000,
        SMTCTRL_3: 0x00000000,
        SMDISMAP0_3: 0x0000FFFF,
        SMDISMAP1_3: 0x0000FFFF,
        SMDTCNT0_3: 0x000007FF,
        SMDTCNT1_3: 0x000007FF,
        SMCAPTCTRLA_3: 0x00000000,
        SMCAPTCOMPA_3: 0x00000000,
        SMCAPTCTRLB_3: 0x00000000,
        SMCAPTCOMPB_3: 0x00000000,
        SMCAPTCTRLX_3: 0x00000000,
        SMCAPTCOMPX_3: 0x00000000,
        SMCVAL0_3: 0x00000000,
        SMCVAL0CYC_3: 0x00000000,
        SMCVAL1_3: 0x00000000,
        SMCVAL1CYC_3: 0x00000000,
        SMCVAL2_3: 0x00000000,
        SMCVAL2CYC_3: 0x00000000,
        SMCVAL3_3: 0x00000000,
        SMCVAL3CYC_3: 0x00000000,
        SMCVAL4_3: 0x00000000,
        SMCVAL4CYC_3: 0x00000000,
        SMCVAL5_3: 0x00000000,
        SMCVAL5CYC_3: 0x00000000,
    };

    /// Safe access to PWM4
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
        let taken = PWM4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PWM4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PWM4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PWM4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PWM4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PWM4 {
    /// The interrupts associated with PWM4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 5] = [
        crate::interrupt::PWM4_0,
        crate::interrupt::PWM4_1,
        crate::interrupt::PWM4_2,
        crate::interrupt::PWM4_3,
        crate::interrupt::PWM4_FAULT,
    ];

    /// The interrupts associated with PWM4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PWM4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWM4: *const RegisterBlock = 0x403e8000 as *const _;
