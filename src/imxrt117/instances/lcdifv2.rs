#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LCDIF_V2
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::lcdifv2::Instance;
pub use crate::imxrt117::peripherals::lcdifv2::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::lcdifv2::{
    CLUT_LOAD, CSC0_COEF0, CSC0_COEF1, CSC0_COEF2, CSC1_COEF0, CSC1_COEF1, CSC1_COEF2, CTRL,
    CTRLDESCL0_1, CTRLDESCL0_2, CTRLDESCL0_3, CTRLDESCL0_4, CTRLDESCL0_5, CTRLDESCL0_6,
    CTRLDESCL1_1, CTRLDESCL1_2, CTRLDESCL1_3, CTRLDESCL1_4, CTRLDESCL1_5, CTRLDESCL1_6,
    CTRLDESCL2_1, CTRLDESCL2_2, CTRLDESCL2_3, CTRLDESCL2_4, CTRLDESCL2_5, CTRLDESCL2_6,
    CTRLDESCL3_1, CTRLDESCL3_2, CTRLDESCL3_3, CTRLDESCL3_4, CTRLDESCL3_5, CTRLDESCL3_6,
    CTRLDESCL4_1, CTRLDESCL4_2, CTRLDESCL4_3, CTRLDESCL4_4, CTRLDESCL4_5, CTRLDESCL4_6,
    CTRLDESCL5_1, CTRLDESCL5_2, CTRLDESCL5_3, CTRLDESCL5_4, CTRLDESCL5_5, CTRLDESCL5_6,
    CTRLDESCL6_1, CTRLDESCL6_2, CTRLDESCL6_3, CTRLDESCL6_4, CTRLDESCL6_5, CTRLDESCL6_6,
    CTRLDESCL7_1, CTRLDESCL7_2, CTRLDESCL7_3, CTRLDESCL7_4, CTRLDESCL7_5, CTRLDESCL7_6, CTRL_CLR,
    CTRL_SET, CTRL_TOG, DISP_PARA, DISP_SIZE, HSYN_PARA, INT_ENABLE_D0, INT_ENABLE_D1,
    INT_STATUS_D0, INT_STATUS_D1, PDI_PARA, VSYN_PARA,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The LCDIFV2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type LCDIFV2 = Instance<0>;

/// The LCDIFV2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type LCDIFV2 = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct LCDIFV2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for LCDIFV2 {}
impl crate::Valid for LCDIFV2 {
    fn take() -> Option<Self> {
        <LCDIFV2>::take()
    }
    fn release(self) {
        <LCDIFV2>::release(self);
    }
    unsafe fn steal() -> Self {
        <LCDIFV2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static LCDIFV2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the LCDIFV2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl LCDIFV2 {
    const INSTANCE: Self = Self {
        addr: 0x40808000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::LCDIFv2],
    };

    /// Reset values for each field in LCDIFV2
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x80000000,
        CTRL_SET: 0x80000000,
        CTRL_CLR: 0x80000000,
        CTRL_TOG: 0x80000000,
        DISP_PARA: 0x00000000,
        DISP_SIZE: 0x00000000,
        HSYN_PARA: 0x00C01803,
        VSYN_PARA: 0x00C01803,
        INT_STATUS_D0: 0x00000000,
        INT_ENABLE_D0: 0x00000000,
        INT_STATUS_D1: 0x00000000,
        INT_ENABLE_D1: 0x00000000,
        PDI_PARA: 0x00000000,
        CTRLDESCL0_1: 0x00000000,
        CTRLDESCL0_2: 0x00000000,
        CTRLDESCL0_3: 0x00000000,
        CTRLDESCL0_4: 0x00000000,
        CTRLDESCL0_5: 0x00000000,
        CTRLDESCL0_6: 0x00000000,
        CSC0_COEF0: 0x04000000,
        CSC0_COEF1: 0x01230208,
        CSC0_COEF2: 0x076B079C,
        CTRLDESCL1_1: 0x00000000,
        CTRLDESCL1_2: 0x00000000,
        CTRLDESCL1_3: 0x00000000,
        CTRLDESCL1_4: 0x00000000,
        CTRLDESCL1_5: 0x00000000,
        CTRLDESCL1_6: 0x00000000,
        CSC1_COEF0: 0x04000000,
        CSC1_COEF1: 0x01230208,
        CSC1_COEF2: 0x076B079C,
        CTRLDESCL2_1: 0x00000000,
        CTRLDESCL2_2: 0x00000000,
        CTRLDESCL2_3: 0x00000000,
        CTRLDESCL2_4: 0x00000000,
        CTRLDESCL2_5: 0x00000000,
        CTRLDESCL2_6: 0x00000000,
        CTRLDESCL3_1: 0x00000000,
        CTRLDESCL3_2: 0x00000000,
        CTRLDESCL3_3: 0x00000000,
        CTRLDESCL3_4: 0x00000000,
        CTRLDESCL3_5: 0x00000000,
        CTRLDESCL3_6: 0x00000000,
        CTRLDESCL4_1: 0x00000000,
        CTRLDESCL4_2: 0x00000000,
        CTRLDESCL4_3: 0x00000000,
        CTRLDESCL4_4: 0x00000000,
        CTRLDESCL4_5: 0x00000000,
        CTRLDESCL4_6: 0x00000000,
        CTRLDESCL5_1: 0x00000000,
        CTRLDESCL5_2: 0x00000000,
        CTRLDESCL5_3: 0x00000000,
        CTRLDESCL5_4: 0x00000000,
        CTRLDESCL5_5: 0x00000000,
        CTRLDESCL5_6: 0x00000000,
        CTRLDESCL6_1: 0x00000000,
        CTRLDESCL6_2: 0x00000000,
        CTRLDESCL6_3: 0x00000000,
        CTRLDESCL6_4: 0x00000000,
        CTRLDESCL6_5: 0x00000000,
        CTRLDESCL6_6: 0x00000000,
        CTRLDESCL7_1: 0x00000000,
        CTRLDESCL7_2: 0x00000000,
        CTRLDESCL7_3: 0x00000000,
        CTRLDESCL7_4: 0x00000000,
        CTRLDESCL7_5: 0x00000000,
        CTRLDESCL7_6: 0x00000000,
        CLUT_LOAD: 0x00000000,
    };

    /// Safe access to LCDIFV2
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
        let taken = LCDIFV2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to LCDIFV2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = LCDIFV2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LCDIFV2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        LCDIFV2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl LCDIFV2 {
    /// The interrupts associated with LCDIFV2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::LCDIFv2];

    /// The interrupts associated with LCDIFV2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to LCDIFV2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LCDIFV2: *const RegisterBlock = 0x40808000 as *const _;
