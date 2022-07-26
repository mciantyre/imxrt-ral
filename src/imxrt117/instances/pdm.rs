#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PDM
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::pdm::Instance;
pub use crate::imxrt117::peripherals::pdm::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::pdm::{
    CTRL_1, CTRL_2, DATACH0, DATACH1, DATACH2, DATACH3, DATACH4, DATACH5, DATACH6, DATACH7,
    DC_CTRL, FIFO_CTRL, FIFO_STAT, RANGE_CTRL, RANGE_STAT, STAT, VAD0_CTRL_1, VAD0_CTRL_2,
    VAD0_NCONFIG, VAD0_NDATA, VAD0_SCONFIG, VAD0_STAT, VAD0_ZCD,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The PDM peripheral instance.
#[cfg(not(feature = "doc"))]
pub type PDM = Instance<0>;

/// The PDM peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type PDM = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct PDM {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for PDM {}
impl crate::Valid for PDM {
    fn take() -> Option<Self> {
        <PDM>::take()
    }
    fn release(self) {
        <PDM>::release(self);
    }
    unsafe fn steal() -> Self {
        <PDM>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static PDM_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the PDM peripheral instance
#[cfg(not(feature = "nosync"))]
impl PDM {
    const INSTANCE: Self = Self {
        addr: 0x40c20000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::PDM_HWVAD_EVENT,
            crate::interrupt::PDM_HWVAD_ERROR,
            crate::interrupt::PDM_EVENT,
            crate::interrupt::PDM_ERROR,
        ],
    };

    /// Reset values for each field in PDM
    pub const reset: ResetValues = ResetValues {
        CTRL_1: 0x00000000,
        CTRL_2: 0x00000000,
        STAT: 0x00000000,
        FIFO_CTRL: 0x00000007,
        FIFO_STAT: 0x00000000,
        DATACH0: 0x00000000,
        DATACH1: 0x00000000,
        DATACH2: 0x00000000,
        DATACH3: 0x00000000,
        DATACH4: 0x00000000,
        DATACH5: 0x00000000,
        DATACH6: 0x00000000,
        DATACH7: 0x00000000,
        DC_CTRL: 0x00000000,
        RANGE_CTRL: 0x00000000,
        RANGE_STAT: 0x00000000,
        VAD0_CTRL_1: 0x00000000,
        VAD0_CTRL_2: 0x000A0000,
        VAD0_STAT: 0x80000000,
        VAD0_SCONFIG: 0x00000000,
        VAD0_NCONFIG: 0x80000000,
        VAD0_NDATA: 0x00000000,
        VAD0_ZCD: 0x00000004,
    };

    /// Safe access to PDM
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
        let taken = PDM_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to PDM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = PDM_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal PDM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        PDM_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl PDM {
    /// The interrupts associated with PDM
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 4] = [
        crate::interrupt::PDM_HWVAD_EVENT,
        crate::interrupt::PDM_HWVAD_ERROR,
        crate::interrupt::PDM_EVENT,
        crate::interrupt::PDM_ERROR,
    ];

    /// The interrupts associated with PDM
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to PDM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PDM: *const RegisterBlock = 0x40c20000 as *const _;
