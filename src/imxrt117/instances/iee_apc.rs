#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IEE_APC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::iee_apc::Instance;
pub use crate::imxrt117::peripherals::iee_apc::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::iee_apc::{
    REGION0_BOT_ADDR, REGION0_RDC_D0, REGION0_RDC_D1, REGION0_TOP_ADDR, REGION1_BOT_ADDR,
    REGION1_RDC_D0, REGION1_RDC_D1, REGION1_TOP_ADDR, REGION2_BOT_ADDR, REGION2_RDC_D0,
    REGION2_RDC_D1, REGION2_TOP_ADDR, REGION3_BOT_ADDR, REGION3_RDC_D0, REGION3_RDC_D1,
    REGION3_TOP_ADDR, REGION4_BOT_ADDR, REGION4_RDC_D0, REGION4_RDC_D1, REGION4_TOP_ADDR,
    REGION5_BOT_ADDR, REGION5_RDC_D0, REGION5_RDC_D1, REGION5_TOP_ADDR, REGION6_BOT_ADDR,
    REGION6_RDC_D0, REGION6_RDC_D1, REGION6_TOP_ADDR, REGION7_BOT_ADDR, REGION7_RDC_D0,
    REGION7_RDC_D1, REGION7_TOP_ADDR,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The IEE_APC peripheral instance.
#[cfg(not(feature = "doc"))]
pub type IEE_APC = Instance<0>;

/// The IEE_APC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type IEE_APC = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct IEE_APC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for IEE_APC {}
impl crate::Valid for IEE_APC {
    fn take() -> Option<Self> {
        <IEE_APC>::take()
    }
    fn release(self) {
        <IEE_APC>::release(self);
    }
    unsafe fn steal() -> Self {
        <IEE_APC>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static IEE_APC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the IEE_APC peripheral instance
#[cfg(not(feature = "nosync"))]
impl IEE_APC {
    const INSTANCE: Self = Self {
        addr: 0x40068000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in IEE_APC
    pub const reset: ResetValues = ResetValues {
        REGION0_TOP_ADDR: 0x00000000,
        REGION0_BOT_ADDR: 0x00000000,
        REGION0_RDC_D0: 0x00000000,
        REGION0_RDC_D1: 0x00000000,
        REGION1_TOP_ADDR: 0x00000000,
        REGION1_BOT_ADDR: 0x00000000,
        REGION1_RDC_D0: 0x00000000,
        REGION1_RDC_D1: 0x00000000,
        REGION2_TOP_ADDR: 0x00000000,
        REGION2_BOT_ADDR: 0x00000000,
        REGION2_RDC_D0: 0x00000000,
        REGION2_RDC_D1: 0x00000000,
        REGION3_TOP_ADDR: 0x00000000,
        REGION3_BOT_ADDR: 0x00000000,
        REGION3_RDC_D0: 0x00000000,
        REGION3_RDC_D1: 0x00000000,
        REGION4_TOP_ADDR: 0x00000000,
        REGION4_BOT_ADDR: 0x00000000,
        REGION4_RDC_D0: 0x00000000,
        REGION4_RDC_D1: 0x00000000,
        REGION5_TOP_ADDR: 0x00000000,
        REGION5_BOT_ADDR: 0x00000000,
        REGION5_RDC_D0: 0x00000000,
        REGION5_RDC_D1: 0x00000000,
        REGION6_TOP_ADDR: 0x00000000,
        REGION6_BOT_ADDR: 0x00000000,
        REGION6_RDC_D0: 0x00000000,
        REGION6_RDC_D1: 0x00000000,
        REGION7_TOP_ADDR: 0x00000000,
        REGION7_BOT_ADDR: 0x00000000,
        REGION7_RDC_D0: 0x00000000,
        REGION7_RDC_D1: 0x00000000,
    };

    /// Safe access to IEE_APC
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
        let taken = IEE_APC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to IEE_APC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = IEE_APC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal IEE_APC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        IEE_APC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl IEE_APC {
    /// The interrupts associated with IEE_APC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with IEE_APC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to IEE_APC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IEE_APC: *const RegisterBlock = 0x40068000 as *const _;
