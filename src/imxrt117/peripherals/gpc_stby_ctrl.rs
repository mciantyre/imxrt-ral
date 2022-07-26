#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPC_STBY
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// Standby Authentication Control
pub mod STBY_AUTHEN_CTRL {

    /// Configuration lock
    pub mod LOCK_CFG {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// STBY Misc
pub mod STBY_MISC {

    /// Force CPU0 requesting standby mode
    pub mod FORCE_CPU0_STBY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Force CPU0 requesting standby mode
    pub mod FORCE_CPU1_STBY {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Force CPU2 requesting standby mode
    pub mod FORCE_CPU2_STBY {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Force CPU3 requesting standby mode
    pub mod FORCE_CPU3_STBY {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// STBY lpcg_in control
pub mod STBY_LPCG_IN_CTRL {

    /// Step count, useage is depending on CNT_MODE
    pub mod STEP_CNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Count mode
    pub mod CNT_MODE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Counter disable mode: not use step counter, step completes once receiving step_done
            pub const b0: u32 = 0b00;

            /// 0b01: Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT
            pub const b1: u32 = 0b01;

            /// 0b10: Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes
            pub const b2: u32 = 0b10;

            /// 0b11: Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value
            pub const b3: u32 = 0b11;
        }
    }

    /// Disable this step
    pub mod DISABLE {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// STBY pll_in control
pub mod STBY_PLL_IN_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY bias_in control
pub mod STBY_BIAS_IN_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY pldo_in control
pub mod STBY_PLDO_IN_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY bandgap_in control
pub mod STBY_BANDGAP_IN_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY ldo_in control
pub mod STBY_LDO_IN_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY dcdc_in control
pub mod STBY_DCDC_IN_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY PMIC in control
pub mod STBY_PMIC_IN_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY PMIC out control
pub mod STBY_PMIC_OUT_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY DCDC out control
pub mod STBY_DCDC_OUT_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY LDO out control
pub mod STBY_LDO_OUT_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY bandgap out control
pub mod STBY_BANDGAP_OUT_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY pldo out control
pub mod STBY_PLDO_OUT_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY bias out control
pub mod STBY_BIAS_OUT_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY PLL out control
pub mod STBY_PLL_OUT_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}

/// STBY LPCG out control
pub mod STBY_LPCG_OUT_CTRL {
    pub use super::STBY_LPCG_IN_CTRL::CNT_MODE;
    pub use super::STBY_LPCG_IN_CTRL::DISABLE;
    pub use super::STBY_LPCG_IN_CTRL::STEP_CNT;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 1],

    /// Standby Authentication Control
    pub STBY_AUTHEN_CTRL: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// STBY Misc
    pub STBY_MISC: RWRegister<u32>,

    _reserved3: [u32; 56],

    /// STBY lpcg_in control
    pub STBY_LPCG_IN_CTRL: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// STBY pll_in control
    pub STBY_PLL_IN_CTRL: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// STBY bias_in control
    pub STBY_BIAS_IN_CTRL: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// STBY pldo_in control
    pub STBY_PLDO_IN_CTRL: RWRegister<u32>,

    _reserved7: [u32; 1],

    /// STBY bandgap_in control
    pub STBY_BANDGAP_IN_CTRL: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// STBY ldo_in control
    pub STBY_LDO_IN_CTRL: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// STBY dcdc_in control
    pub STBY_DCDC_IN_CTRL: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// STBY PMIC in control
    pub STBY_PMIC_IN_CTRL: RWRegister<u32>,

    _reserved11: [u32; 43],

    /// STBY PMIC out control
    pub STBY_PMIC_OUT_CTRL: RWRegister<u32>,

    _reserved12: [u32; 3],

    /// STBY DCDC out control
    pub STBY_DCDC_OUT_CTRL: RWRegister<u32>,

    _reserved13: [u32; 3],

    /// STBY LDO out control
    pub STBY_LDO_OUT_CTRL: RWRegister<u32>,

    _reserved14: [u32; 3],

    /// STBY bandgap out control
    pub STBY_BANDGAP_OUT_CTRL: RWRegister<u32>,

    _reserved15: [u32; 1],

    /// STBY pldo out control
    pub STBY_PLDO_OUT_CTRL: RWRegister<u32>,

    _reserved16: [u32; 1],

    /// STBY bias out control
    pub STBY_BIAS_OUT_CTRL: RWRegister<u32>,

    _reserved17: [u32; 3],

    /// STBY PLL out control
    pub STBY_PLL_OUT_CTRL: RWRegister<u32>,

    _reserved18: [u32; 3],

    /// STBY LPCG out control
    pub STBY_LPCG_OUT_CTRL: RWRegister<u32>,
}
pub struct ResetValues {
    pub STBY_AUTHEN_CTRL: u32,
    pub STBY_MISC: u32,
    pub STBY_LPCG_IN_CTRL: u32,
    pub STBY_PLL_IN_CTRL: u32,
    pub STBY_BIAS_IN_CTRL: u32,
    pub STBY_PLDO_IN_CTRL: u32,
    pub STBY_BANDGAP_IN_CTRL: u32,
    pub STBY_LDO_IN_CTRL: u32,
    pub STBY_DCDC_IN_CTRL: u32,
    pub STBY_PMIC_IN_CTRL: u32,
    pub STBY_PMIC_OUT_CTRL: u32,
    pub STBY_DCDC_OUT_CTRL: u32,
    pub STBY_LDO_OUT_CTRL: u32,
    pub STBY_BANDGAP_OUT_CTRL: u32,
    pub STBY_PLDO_OUT_CTRL: u32,
    pub STBY_BIAS_OUT_CTRL: u32,
    pub STBY_PLL_OUT_CTRL: u32,
    pub STBY_LPCG_OUT_CTRL: u32,
}
pub struct Instance<const N: u8> {
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) addr: u32,
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) intrs: &'static [crate::Interrupt],
}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> ::core::ops::Deref for Instance<N> {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

#[cfg(not(feature = "nosync"))]
unsafe impl<const N: u8> Send for Instance<N> {}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> Instance<N> {
    /// Return the interrupt signals associated with this
    /// peripheral instance
    ///
    /// Collection may be empty if there is no interrupt signal
    /// associated with the peripheral. There's no guarantee for
    /// interrupt signal ordering in the collection.
    #[inline(always)]
    pub const fn interrupts<'a>(&'a self) -> &'a [crate::Interrupt] {
        self.intrs
    }
}
