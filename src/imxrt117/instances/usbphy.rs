#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USBPHY
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

pub use crate::imxrt117::peripherals::usbphy::Instance;
pub use crate::imxrt117::peripherals::usbphy::{RegisterBlock, ResetValues};

pub use crate::imxrt117::peripherals::usbphy::{
    ANACTRL, ANACTRL_CLR, ANACTRL_SET, ANACTRL_TOG, CTRL, CTRL_CLR, CTRL_SET, CTRL_TOG, DEBUG,
    DEBUG0_STATUS, DEBUG1, DEBUG1_CLR, DEBUG1_SET, DEBUG1_TOG, DEBUG_CLR, DEBUG_SET, DEBUG_TOG,
    PLL_SIC, PLL_SIC_CLR, PLL_SIC_SET, PLL_SIC_TOG, PWD, PWD_CLR, PWD_SET, PWD_TOG, RX, RX_CLR,
    RX_SET, RX_TOG, STATUS, TRIM_OVERRIDE_EN, TRIM_OVERRIDE_EN_CLR, TRIM_OVERRIDE_EN_SET,
    TRIM_OVERRIDE_EN_TOG, TX, TX_CLR, TX_SET, TX_TOG, USB1_CHRG_DETECT, USB1_CHRG_DETECT_CLR,
    USB1_CHRG_DETECT_SET, USB1_CHRG_DETECT_TOG, USB1_CHRG_DET_STAT, USB1_LOOPBACK,
    USB1_LOOPBACK_CLR, USB1_LOOPBACK_HSFSCNT, USB1_LOOPBACK_HSFSCNT_CLR, USB1_LOOPBACK_HSFSCNT_SET,
    USB1_LOOPBACK_HSFSCNT_TOG, USB1_LOOPBACK_SET, USB1_LOOPBACK_TOG, USB1_VBUS_DETECT,
    USB1_VBUS_DETECT_CLR, USB1_VBUS_DETECT_SET, USB1_VBUS_DETECT_TOG, USB1_VBUS_DET_STAT, VERSION,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The USBPHY1 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type USBPHY1 = Instance<1>;

/// The USBPHY1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type USBPHY1 = Instance<1>;
/// ```
#[cfg(feature = "doc")]
pub struct USBPHY1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for USBPHY1 {}
impl crate::Valid for USBPHY1 {
    fn take() -> Option<Self> {
        <USBPHY1>::take()
    }
    fn release(self) {
        <USBPHY1>::release(self);
    }
    unsafe fn steal() -> Self {
        <USBPHY1>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USBPHY1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USBPHY1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl USBPHY1 {
    const INSTANCE: Self = Self {
        addr: 0x40434000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::USBPHY1],
    };

    /// Reset values for each field in USBPHY1
    pub const reset: ResetValues = ResetValues {
        PWD: 0x001E1C00,
        PWD_SET: 0x001E1C00,
        PWD_CLR: 0x001E1C00,
        PWD_TOG: 0x001E1C00,
        TX: 0x10060607,
        TX_SET: 0x10060607,
        TX_CLR: 0x10060607,
        TX_TOG: 0x10060607,
        RX: 0x00000000,
        RX_SET: 0x00000000,
        RX_CLR: 0x00000000,
        RX_TOG: 0x00000000,
        CTRL: 0x88000000,
        CTRL_SET: 0x88000000,
        CTRL_CLR: 0x88000000,
        CTRL_TOG: 0x88000000,
        STATUS: 0x00000000,
        DEBUG: 0x7F180000,
        DEBUG_SET: 0x7F180000,
        DEBUG_CLR: 0x7F180000,
        DEBUG_TOG: 0x7F180000,
        DEBUG0_STATUS: 0x00000000,
        DEBUG1: 0x00001000,
        DEBUG1_SET: 0x00001000,
        DEBUG1_CLR: 0x00001000,
        DEBUG1_TOG: 0x00001000,
        VERSION: 0x04030000,
        PLL_SIC: 0x00D12000,
        PLL_SIC_SET: 0x00D12000,
        PLL_SIC_CLR: 0x00D12000,
        PLL_SIC_TOG: 0x00D12000,
        USB1_VBUS_DETECT: 0x00700004,
        USB1_VBUS_DETECT_SET: 0x00700004,
        USB1_VBUS_DETECT_CLR: 0x00700004,
        USB1_VBUS_DETECT_TOG: 0x00700004,
        USB1_VBUS_DET_STAT: 0x00000000,
        USB1_CHRG_DETECT: 0x80180000,
        USB1_CHRG_DETECT_SET: 0x80180000,
        USB1_CHRG_DETECT_CLR: 0x80180000,
        USB1_CHRG_DETECT_TOG: 0x80180000,
        USB1_CHRG_DET_STAT: 0x00000000,
        ANACTRL: 0x00000402,
        ANACTRL_SET: 0x00000402,
        ANACTRL_CLR: 0x00000402,
        ANACTRL_TOG: 0x00000402,
        USB1_LOOPBACK: 0x00550000,
        USB1_LOOPBACK_SET: 0x00550000,
        USB1_LOOPBACK_CLR: 0x00550000,
        USB1_LOOPBACK_TOG: 0x00550000,
        USB1_LOOPBACK_HSFSCNT: 0x00040010,
        USB1_LOOPBACK_HSFSCNT_SET: 0x00040010,
        USB1_LOOPBACK_HSFSCNT_CLR: 0x00040010,
        USB1_LOOPBACK_HSFSCNT_TOG: 0x00040010,
        TRIM_OVERRIDE_EN: 0x0000007F,
        TRIM_OVERRIDE_EN_SET: 0x0000007F,
        TRIM_OVERRIDE_EN_CLR: 0x0000007F,
        TRIM_OVERRIDE_EN_TOG: 0x0000007F,
    };

    /// Safe access to USBPHY1
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
        let taken = USBPHY1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USBPHY1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USBPHY1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USBPHY1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USBPHY1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl USBPHY1 {
    /// The interrupts associated with USBPHY1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::USBPHY1];

    /// The interrupts associated with USBPHY1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USBPHY1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USBPHY1: *const RegisterBlock = 0x40434000 as *const _;

/// The USBPHY2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type USBPHY2 = Instance<2>;

/// The USBPHY2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type USBPHY2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct USBPHY2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for USBPHY2 {}
impl crate::Valid for USBPHY2 {
    fn take() -> Option<Self> {
        <USBPHY2>::take()
    }
    fn release(self) {
        <USBPHY2>::release(self);
    }
    unsafe fn steal() -> Self {
        <USBPHY2>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static USBPHY2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the USBPHY2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl USBPHY2 {
    const INSTANCE: Self = Self {
        addr: 0x40438000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::USBPHY2],
    };

    /// Reset values for each field in USBPHY2
    pub const reset: ResetValues = ResetValues {
        PWD: 0x001E1C00,
        PWD_SET: 0x001E1C00,
        PWD_CLR: 0x001E1C00,
        PWD_TOG: 0x001E1C00,
        TX: 0x10060607,
        TX_SET: 0x10060607,
        TX_CLR: 0x10060607,
        TX_TOG: 0x10060607,
        RX: 0x00000000,
        RX_SET: 0x00000000,
        RX_CLR: 0x00000000,
        RX_TOG: 0x00000000,
        CTRL: 0x88000000,
        CTRL_SET: 0x88000000,
        CTRL_CLR: 0x88000000,
        CTRL_TOG: 0x88000000,
        STATUS: 0x00000000,
        DEBUG: 0x7F180000,
        DEBUG_SET: 0x7F180000,
        DEBUG_CLR: 0x7F180000,
        DEBUG_TOG: 0x7F180000,
        DEBUG0_STATUS: 0x00000000,
        DEBUG1: 0x00001000,
        DEBUG1_SET: 0x00001000,
        DEBUG1_CLR: 0x00001000,
        DEBUG1_TOG: 0x00001000,
        VERSION: 0x04030000,
        PLL_SIC: 0x00D12000,
        PLL_SIC_SET: 0x00D12000,
        PLL_SIC_CLR: 0x00D12000,
        PLL_SIC_TOG: 0x00D12000,
        USB1_VBUS_DETECT: 0x00700004,
        USB1_VBUS_DETECT_SET: 0x00700004,
        USB1_VBUS_DETECT_CLR: 0x00700004,
        USB1_VBUS_DETECT_TOG: 0x00700004,
        USB1_VBUS_DET_STAT: 0x00000000,
        USB1_CHRG_DETECT: 0x80180000,
        USB1_CHRG_DETECT_SET: 0x80180000,
        USB1_CHRG_DETECT_CLR: 0x80180000,
        USB1_CHRG_DETECT_TOG: 0x80180000,
        USB1_CHRG_DET_STAT: 0x00000000,
        ANACTRL: 0x00000402,
        ANACTRL_SET: 0x00000402,
        ANACTRL_CLR: 0x00000402,
        ANACTRL_TOG: 0x00000402,
        USB1_LOOPBACK: 0x00550000,
        USB1_LOOPBACK_SET: 0x00550000,
        USB1_LOOPBACK_CLR: 0x00550000,
        USB1_LOOPBACK_TOG: 0x00550000,
        USB1_LOOPBACK_HSFSCNT: 0x00040010,
        USB1_LOOPBACK_HSFSCNT_SET: 0x00040010,
        USB1_LOOPBACK_HSFSCNT_CLR: 0x00040010,
        USB1_LOOPBACK_HSFSCNT_TOG: 0x00040010,
        TRIM_OVERRIDE_EN: 0x0000007F,
        TRIM_OVERRIDE_EN_SET: 0x0000007F,
        TRIM_OVERRIDE_EN_CLR: 0x0000007F,
        TRIM_OVERRIDE_EN_TOG: 0x0000007F,
    };

    /// Safe access to USBPHY2
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
        let taken = USBPHY2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to USBPHY2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = USBPHY2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal USBPHY2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        USBPHY2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl USBPHY2 {
    /// The interrupts associated with USBPHY2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::USBPHY2];

    /// The interrupts associated with USBPHY2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to USBPHY2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USBPHY2: *const RegisterBlock = 0x40438000 as *const _;
