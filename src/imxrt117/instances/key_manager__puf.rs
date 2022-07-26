#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PUF
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::key_manager__puf::Instance;
pub use crate::imxrt117::peripherals::key_manager__puf::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::key_manager__puf::{
    ALLOW, CFG, CODEINPUT, CODEOUTPUT, CTRL, IDXBLK, IDXBLK_DP, IDXBLK_SHIFT, IDXBLK_STATUS,
    IFSTAT, INTEN, INTSTAT, KEYENABLE, KEYINDEX, KEYINPUT, KEYLOCK, KEYMASK0, KEYMASK1,
    KEYOUTINDEX, KEYOUTPUT, KEYRESET, KEYSIZE, PWRCTRL, STAT, VERSION,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The KEY_MANAGER__PUF peripheral instance.
#[cfg(not(feature = "doc"))]
pub type KEY_MANAGER__PUF = Instance<0>;

/// The KEY_MANAGER__PUF peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type KEY_MANAGER__PUF = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct KEY_MANAGER__PUF {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for KEY_MANAGER__PUF {}
impl crate::Valid for KEY_MANAGER__PUF {
    fn take() -> Option<Self> {
        <KEY_MANAGER__PUF>::take()
    }
    fn release(self) {
        <KEY_MANAGER__PUF>::release(self);
    }
    unsafe fn steal() -> Self {
        <KEY_MANAGER__PUF>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static KEY_MANAGER__PUF_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the KEY_MANAGER__PUF peripheral instance
#[cfg(not(feature = "nosync"))]
impl KEY_MANAGER__PUF {
    const INSTANCE: Self = Self {
        addr: 0x40c82000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in KEY_MANAGER__PUF
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        KEYINDEX: 0x00000000,
        KEYSIZE: 0x00000000,
        STAT: 0x00000001,
        ALLOW: 0x00000000,
        KEYINPUT: 0x00000000,
        CODEINPUT: 0x00000000,
        CODEOUTPUT: 0x00000000,
        KEYOUTINDEX: 0x00000000,
        KEYOUTPUT: 0x00000000,
        IFSTAT: 0x00000000,
        VERSION: 0x00000000,
        INTEN: 0x00000000,
        INTSTAT: 0x00000000,
        PWRCTRL: 0x00000001,
        CFG: 0x00000000,
        KEYLOCK: 0x0000000A,
        KEYENABLE: 0x00000005,
        KEYRESET: 0x00000000,
        IDXBLK: 0xAAAAAAAA,
        IDXBLK_DP: 0xAAAAAAAA,
        KEYMASK0: 0x00000000,
        KEYMASK1: 0x00000000,
        IDXBLK_STATUS: 0xAAAAAAAA,
        IDXBLK_SHIFT: 0x00000000,
    };

    /// Safe access to KEY_MANAGER__PUF
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
        let taken = KEY_MANAGER__PUF_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to KEY_MANAGER__PUF
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = KEY_MANAGER__PUF_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal KEY_MANAGER__PUF
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        KEY_MANAGER__PUF_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl KEY_MANAGER__PUF {
    /// The interrupts associated with KEY_MANAGER__PUF
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with KEY_MANAGER__PUF
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to KEY_MANAGER__PUF
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const KEY_MANAGER__PUF: *const RegisterBlock = 0x40c82000 as *const _;
