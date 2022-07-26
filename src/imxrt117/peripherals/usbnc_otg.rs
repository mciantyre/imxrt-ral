#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USBNC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// USB OTG Control 1 Register
pub mod CTRL1 {

    /// OVER_CUR_DIS
    pub mod OVER_CUR_DIS {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enables overcurrent detection
            pub const OVRCRNT_DETCT_EN: u32 = 0b0;

            /// 0b1: Disables overcurrent detection
            pub const OVRCRNT_DETCT_DIS: u32 = 0b1;
        }
    }

    /// OVER_CUR_POL
    pub mod OVER_CUR_POL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: High active (high on this signal represents an overcurrent condition)
            pub const ACTIVE_HI_OVRCRNT: u32 = 0b0;

            /// 0b1: Low active (low on this signal represents an overcurrent condition)
            pub const ACTIVE_LOW_OVRCRNT: u32 = 0b1;
        }
    }

    /// PWR_POL
    pub mod PWR_POL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PMIC Power Pin is Low active.
            pub const ACTIVE_LO_PMIC: u32 = 0b0;

            /// 0b1: PMIC Power Pin is High active.
            pub const ACTIVE_HI_PMIC: u32 = 0b1;
        }
    }

    /// WIE
    pub mod WIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt Disabled
            pub const INT_DIS: u32 = 0b0;

            /// 0b1: Interrupt Enabled
            pub const INT_EN: u32 = 0b1;
        }
    }

    /// WKUP_SW_EN
    pub mod WKUP_SW_EN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable
            pub const SW_WKUP_DIS: u32 = 0b0;

            /// 0b1: Enable
            pub const SW_WKUP_EN: u32 = 0b1;
        }
    }

    /// WKUP_SW
    pub mod WKUP_SW {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Inactive
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Force wake-up
            pub const FORCE_WKUP: u32 = 0b1;
        }
    }

    /// WKUP_ID_EN
    pub mod WKUP_ID_EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable
            pub const WKUP_ID_DIS: u32 = 0b0;

            /// 0b1: Enable
            pub const WKUP_ID_EN: u32 = 0b1;
        }
    }

    /// WKUP_VBUS_EN
    pub mod WKUP_VBUS_EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable
            pub const WKUP_VBUS_DIS: u32 = 0b0;

            /// 0b1: Enable
            pub const WKUP_VBUS_EN: u32 = 0b1;
        }
    }

    /// Wake-up on DPDM change enable
    pub mod WKUP_DPDM_EN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DPDM changes wake-up to be disabled only when VBUS is 0.
            pub const DPDM_WKUP_DIS: u32 = 0b0;

            /// 0b1: (Default) DPDM changes wake-up to be enabled, it is for device only.
            pub const DPDM_WKUP_EN: u32 = 0b1;
        }
    }

    /// WIR
    pub mod WIR {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No wake-up interrupt request received
            pub const NO_WKUP_REQ: u32 = 0b0;

            /// 0b1: Wake-up Interrupt Request received
            pub const WKUP_REQ: u32 = 0b1;
        }
    }
}

/// USB OTG Control 2 Register
pub mod CTRL2 {

    /// VBUS_SOURCE_SEL
    pub mod VBUS_SOURCE_SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: vbus_valid
            pub const VBUS_VALID: u32 = 0b00;

            /// 0b01: sess_valid
            pub const SESS_VALID_1: u32 = 0b01;

            /// 0b10: sess_valid
            pub const SESS_VALID_2: u32 = 0b10;

            /// 0b11: sess_valid
            pub const SESS_VALID_3: u32 = 0b11;
        }
    }

    /// Auto Resume Enable
    pub mod AUTURESUME_EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Default
            pub const DEFAULT: u32 = 0b0;
        }
    }

    /// LOWSPEED_EN
    pub mod LOWSPEED_EN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AUTURESUME_EN::RW;
    }

    /// UTMI_CLK_VLD
    pub mod UTMI_CLK_VLD {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AUTURESUME_EN::RW;
    }
}

/// USB Host HSIC Control Register
pub mod HSIC_CTRL {

    /// HSIC_CLK_ON
    pub mod HSIC_CLK_ON {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Inactive
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// HSIC_EN
    pub mod HSIC_EN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// CLK_VLD
    pub mod CLK_VLD {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Invalid
            pub const INVALID: u32 = 0b0;

            /// 0b1: Valid
            pub const VALID: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// USB OTG Control 1 Register
    pub CTRL1: RWRegister<u32>,

    /// USB OTG Control 2 Register
    pub CTRL2: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// USB Host HSIC Control Register
    pub HSIC_CTRL: RWRegister<u32>,
}
pub struct ResetValues {
    pub CTRL1: u32,
    pub CTRL2: u32,
    pub HSIC_CTRL: u32,
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
