#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MX6RT_ANADIG_REGISTER
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// PMU_LDO_LPSR_ANA_REGISTER
pub mod PMU_LDO_LPSR_ANA {

    /// reg_lp_en
    pub mod REG_LP_EN {
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

    /// reg_disable
    pub mod REG_DISABLE {
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

    /// pull_down_2ma_en
    pub mod PULL_DOWN_2MA_EN {
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

    /// LPSR_ANA_CONTROL_MODE
    pub mod LPSR_ANA_CONTROL_MODE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SW Control
            pub const sw: u32 = 0b0;

            /// 0b1: HW Control
            pub const hw: u32 = 0b1;
        }
    }

    /// bypass_mode_en
    pub mod BYPASS_MODE_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// standby_en
    pub mod STANDBY_EN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// always_4ma_pulldown_en
    pub mod ALWAYS_4MA_PULLDOWN_EN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Track Mode Enable
    pub mod TRACK_MODE_EN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal use
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Switch preparation
            pub const SWITCH: u32 = 0b1;
        }
    }

    /// pull_down_20ua_en
    pub mod PULL_DOWN_20UA_EN {
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

/// PMU_LDO_LPSR_DIG_2_REGISTER
pub mod PMU_LDO_LPSR_DIG_2 {

    /// voltage_step_inc
    pub mod VOLTAGE_STEP_INC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PMU_LDO_LPSR_DIG_REGISTER
pub mod PMU_LDO_LPSR_DIG {

    /// ENABLE_ILIMIT
    pub mod REG_EN {
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

    /// LPSR_DIG_CONTROL_MODE
    pub mod LPSR_DIG_CONTROL_MODE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SW Control
            pub const sw: u32 = 0b0;

            /// 0b1: HW Control
            pub const hw: u32 = 0b1;
        }
    }

    /// standby_en
    pub mod STANDBY_EN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// tracking_mode
    pub mod TRACKING_MODE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// bypass_mode
    pub mod BYPASS_MODE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VOLTAGE_SELECT
    pub mod VOLTAGE_SELECT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (5 bits: 0b11111 << 20)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Stable Voltage (range)
            pub const bitval0: u32 = 0b00000;

            /// 0b00001: Stable Voltage (range)
            pub const bitval1: u32 = 0b00001;

            /// 0b00010: Stable Voltage (range)
            pub const bitval2: u32 = 0b00010;

            /// 0b00011: Stable Voltage (range)
            pub const bitval3: u32 = 0b00011;

            /// 0b00100: Stable Voltage (range)
            pub const bitval4: u32 = 0b00100;

            /// 0b00101: Stable Voltage (range)
            pub const bitval5: u32 = 0b00101;

            /// 0b00110: Stable Voltage (range)
            pub const bitval6: u32 = 0b00110;

            /// 0b00111: Stable Voltage (range)
            pub const bitval7: u32 = 0b00111;

            /// 0b01000: Stable Voltage (range)
            pub const bitval8: u32 = 0b01000;

            /// 0b01001: Stable Voltage (range)
            pub const bitval9: u32 = 0b01001;

            /// 0b01010: Stable Voltage (range)
            pub const bitval10: u32 = 0b01010;

            /// 0b01011: Stable Voltage (range)
            pub const bitval11: u32 = 0b01011;

            /// 0b01100: Stable Voltage (range)
            pub const bitval12: u32 = 0b01100;

            /// 0b01101: Stable Voltage (range)
            pub const bitval13: u32 = 0b01101;

            /// 0b01110: Stable Voltage (range)
            pub const bitval14: u32 = 0b01110;

            /// 0b01111: Stable Voltage (range)
            pub const bitval15: u32 = 0b01111;

            /// 0b10000: Stable Voltage (range)
            pub const bitval16: u32 = 0b10000;

            /// 0b10001: Stable Voltage (range)
            pub const bitval17: u32 = 0b10001;

            /// 0b10010: Stable Voltage (range)
            pub const bitval18: u32 = 0b10010;

            /// 0b10011: Stable Voltage (range)
            pub const bitval19: u32 = 0b10011;

            /// 0b10100: Stable Voltage (range)
            pub const bitval20: u32 = 0b10100;

            /// 0b10101: Stable Voltage (range)
            pub const bitval21: u32 = 0b10101;

            /// 0b10110: Stable Voltage (range)
            pub const bitval22: u32 = 0b10110;

            /// 0b10111: Stable Voltage (range)
            pub const bitval23: u32 = 0b10111;

            /// 0b11000: Stable Voltage (range)
            pub const bitval24: u32 = 0b11000;

            /// 0b11001: Stable Voltage (range)
            pub const bitval25: u32 = 0b11001;

            /// 0b11010: Stable Voltage (range)
            pub const bitval26: u32 = 0b11010;

            /// 0b11011: Stable Voltage (range)
            pub const bitval27: u32 = 0b11011;

            /// 0b11100: Stable Voltage (range)
            pub const bitval28: u32 = 0b11100;

            /// 0b11101: Stable Voltage (range)
            pub const bitval29: u32 = 0b11101;

            /// 0b11110: Stable Voltage (range)
            pub const bitval30: u32 = 0b11110;

            /// 0b11111: Stable Voltage (range)
            pub const bitval31: u32 = 0b11111;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 324],

    /// PMU_LDO_LPSR_ANA_REGISTER
    pub PMU_LDO_LPSR_ANA: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// PMU_LDO_LPSR_DIG_2_REGISTER
    pub PMU_LDO_LPSR_DIG_2: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// PMU_LDO_LPSR_DIG_REGISTER
    pub PMU_LDO_LPSR_DIG: RWRegister<u32>,
}
pub struct ResetValues {
    pub PMU_LDO_LPSR_ANA: u32,
    pub PMU_LDO_LPSR_DIG_2: u32,
    pub PMU_LDO_LPSR_DIG: u32,
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
