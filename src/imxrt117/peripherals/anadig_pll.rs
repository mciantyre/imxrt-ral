#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MX6RT_ANADIG_REGISTER
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// ARM_PLL_CTRL_REGISTER
pub mod ARM_PLL_CTRL {

    /// DIV_SELECT
    pub mod DIV_SELECT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL Start up initialization
    pub mod HOLD_RING_OFF {
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

            /// 0b0: Normal operation
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Initialize PLL start up
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Powers up the PLL.
    pub mod POWERUP {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Power down the PLL
            pub const PDOWN: u32 = 0b0;

            /// 0b1: Power Up the PLL
            pub const PUP: u32 = 0b1;
        }
    }

    /// Enable the clock output.
    pub mod ENABLE_CLK {
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

            /// 0b0: Disable the clock
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable the clock
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// POST_DIV_SEL
    pub mod POST_DIV_SEL {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (2 bits: 0b11 << 15)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Divide by 2
            pub const DIV2: u32 = 0b00;

            /// 0b01: Divide by 4
            pub const DIV4: u32 = 0b01;

            /// 0b10: Divide by 8
            pub const DIV8: u32 = 0b10;

            /// 0b11: Divide by 1
            pub const DIV1: u32 = 0b11;
        }
    }

    /// Bypass the pll.
    pub mod BYPASS {
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

            /// 0b0: Function mode
            pub const FUNC: u32 = 0b0;

            /// 0b1: Bypass Mode
            pub const BYPASS: u32 = 0b1;
        }
    }

    /// ARM_PLL_STABLE
    pub mod ARM_PLL_STABLE {
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

            /// 0b0: ARM PLL is not stable
            pub const DISABLE: u32 = 0b0;

            /// 0b1: ARM PLL is stable
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// ARM_PLL_GATE
    pub mod ARM_PLL_GATE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clock is not gated
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Clock is gated
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// pll_arm_control_mode
    pub mod ARM_PLL_CONTROL_MODE {
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

            /// 0b0: Software Mode (Default)
            pub const SW: u32 = 0b0;

            /// 0b1: GPC Mode
            pub const GPC: u32 = 0b1;
        }
    }
}

/// SYS_PLL3_CTRL_REGISTER
pub mod SYS_PLL3_CTRL {

    /// SYS PLL3 DIV2 gate
    pub mod SYS_PLL3_DIV2 {
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

    /// Enable Internal PLL Regulator
    pub mod PLL_REG_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL Start up initialization
    pub mod HOLD_RING_OFF {
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

            /// 0b0: Normal operation
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Initialize PLL start up
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable the clock output.
    pub mod ENABLE_CLK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable the clock
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable the clock
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// BYPASS
    pub mod BYPASS {
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

            /// 0b0: Function mode
            pub const FUNC: u32 = 0b0;

            /// 0b1: Bypass Mode
            pub const BYPASS: u32 = 0b1;
        }
    }

    /// Powers up the PLL.
    pub mod POWERUP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Power down the PLL
            pub const PDOWN: u32 = 0b0;

            /// 0b1: Power Up the PLL
            pub const PUP: u32 = 0b1;
        }
    }

    /// SYS_PLL3_DIV2_CONTROL_MODE
    pub mod SYS_PLL3_DIV2_CONTROL_MODE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Software Mode (Default)
            pub const SW: u32 = 0b0;

            /// 0b1: GPC Mode
            pub const GPC: u32 = 0b1;
        }
    }

    /// SYS_PLL3_STABLE
    pub mod SYS_PLL3_STABLE {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SYS_PLL3_GATE
    pub mod SYS_PLL3_GATE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clock is not gated
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Clock is gated
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// SYS_PLL3_control_mode
    pub mod SYS_PLL3_CONTROL_MODE {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYS_PLL3_DIV2_CONTROL_MODE::RW;
    }
}

/// SYS_PLL3_UPDATE_REGISTER
pub mod SYS_PLL3_UPDATE {

    /// PFD0_OVERRIDE
    pub mod PFD0_UPDATE {
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

    /// PFD1_OVERRIDE
    pub mod PFD1_UPDATE {
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

    /// PFD2_OVERRIDE
    pub mod PFD2_UPDATE {
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

    /// PFD3_UPDATE
    pub mod PFD3_UPDATE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// pfd0_control_mode
    pub mod PFD0_CONTROL_MODE {
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

            /// 0b0: Software Mode (Default)
            pub const SW: u32 = 0b0;

            /// 0b1: GPC Mode
            pub const GPC: u32 = 0b1;
        }
    }

    /// pfd1_control_mode
    pub mod PFD1_CONTROL_MODE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PFD0_CONTROL_MODE::RW;
    }

    /// pdf2_control_mode
    pub mod PDF2_CONTROL_MODE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PFD0_CONTROL_MODE::RW;
    }

    /// pfd3_control_mode
    pub mod PFD3_CONTROL_MODE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PFD0_CONTROL_MODE::RW;
    }
}

/// SYS_PLL3_PFD_REGISTER
pub mod SYS_PLL3_PFD {

    /// PFD0_FRAC
    pub mod PFD0_FRAC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD0_STABLE
    pub mod PFD0_STABLE {
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

    /// PFD0_DIV1_CLKGATE
    pub mod PFD0_DIV1_CLKGATE {
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

            /// 0b0: ref_pfd0 fractional divider clock is enabled
            pub const ON: u32 = 0b0;

            /// 0b1: Fractional divider clock (reference ref_pfd0) is off (power savings
            pub const OFF: u32 = 0b1;
        }
    }

    /// PFD1_FRAC
    pub mod PFD1_FRAC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD1_STABLE
    pub mod PFD1_STABLE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD1_DIV1_CLKGATE
    pub mod PFD1_DIV1_CLKGATE {
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

            /// 0b0: ref_pfd1 fractional divider clock is enabled
            pub const ON: u32 = 0b0;

            /// 0b1: Fractional divider clock (reference ref_pfd1) is off (power savings)
            pub const OFF: u32 = 0b1;
        }
    }

    /// PFD2_FRAC
    pub mod PFD2_FRAC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD2_STABLE
    pub mod PFD2_STABLE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD2_DIV1_CLKGATE
    pub mod PFD2_DIV1_CLKGATE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ref_pfd2 fractional divider clock is enabled
            pub const ON: u32 = 0b0;

            /// 0b1: Fractional divider clock (reference ref_pfd2) is off (power savings)
            pub const OFF: u32 = 0b1;
        }
    }

    /// PFD3_FRAC
    pub mod PFD3_FRAC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (6 bits: 0x3f << 24)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD3_STABLE
    pub mod PFD3_STABLE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD3_DIV1_CLKGATE
    pub mod PFD3_DIV1_CLKGATE {
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

            /// 0b0: ref_pfd3 fractional divider clock is enabled
            pub const ON: u32 = 0b0;

            /// 0b1: Fractional divider clock (reference ref_pfd3) is off (power savings)
            pub const OFF: u32 = 0b1;
        }
    }
}

/// SYS_PLL2_CTRL_REGISTER
pub mod SYS_PLL2_CTRL {

    /// Enable Internal PLL Regulator
    pub mod PLL_REG_EN {
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

    /// PLL Start up initialization
    pub mod HOLD_RING_OFF {
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

            /// 0b0: Normal operation
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Initialize PLL start up
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable the clock output.
    pub mod ENABLE_CLK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable the clock
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable the clock
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Bypass the pll.
    pub mod BYPASS {
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

            /// 0b0: Function mode
            pub const FUNC: u32 = 0b0;

            /// 0b1: Bypass Mode
            pub const BYPASS: u32 = 0b1;
        }
    }

    /// DITHER_ENABLE
    pub mod DITHER_ENABLE {
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

            /// 0b0: Disable Dither
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable Dither
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// PFD_OFFSET_EN
    pub mod PFD_OFFSET_EN {
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

    /// PLL_DDR_OVERRIDE
    pub mod PLL_DDR_OVERRIDE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Powers up the PLL.
    pub mod POWERUP {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Power down the PLL
            pub const PDOWN: u32 = 0b0;

            /// 0b1: Power Up the PLL
            pub const PUP: u32 = 0b1;
        }
    }

    /// SYS_PLL2_STABLE
    pub mod SYS_PLL2_STABLE {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SYS_PLL2_GATE
    pub mod SYS_PLL2_GATE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clock is not gated
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Clock is gated
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// SYS_PLL2_control_mode
    pub mod SYS_PLL2_CONTROL_MODE {
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

            /// 0b0: Software Mode (Default)
            pub const SW: u32 = 0b0;

            /// 0b1: GPC Mode
            pub const GPC: u32 = 0b1;
        }
    }
}

/// SYS_PLL2_UPDATE_REGISTER
pub mod SYS_PLL2_UPDATE {

    /// PFD0_UPDATE
    pub mod PFD0_UPDATE {
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

    /// PFD1_UPDATE
    pub mod PFD1_UPDATE {
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

    /// PFD2_UPDATE
    pub mod PFD2_UPDATE {
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

    /// PFD3_UPDATE
    pub mod PFD3_UPDATE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// pfd0_control_mode
    pub mod PFD0_CONTROL_MODE {
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

            /// 0b0: Software Mode (Default)
            pub const SW: u32 = 0b0;

            /// 0b1: GPC Mode
            pub const GPC: u32 = 0b1;
        }
    }

    /// pfd1_control_mode
    pub mod PFD1_CONTROL_MODE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PFD0_CONTROL_MODE::RW;
    }

    /// pfd2_control_mode
    pub mod PFD2_CONTROL_MODE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PFD0_CONTROL_MODE::RW;
    }

    /// pfd3_control_mode
    pub mod PFD3_CONTROL_MODE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PFD0_CONTROL_MODE::RW;
    }
}

/// SYS_PLL2_SS_REGISTER
pub mod SYS_PLL2_SS {

    /// STEP
    pub mod STEP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (15 bits: 0x7fff << 0)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENABLE
    pub mod ENABLE {
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

            /// 0b0: Disable Spread Spectrum
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable Spread Spectrum
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// STOP
    pub mod STOP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SYS_PLL2_PFD_REGISTER
pub mod SYS_PLL2_PFD {

    /// PFD0_FRAC
    pub mod PFD0_FRAC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD0_STABLE
    pub mod PFD0_STABLE {
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

    /// PFD0_DIV1_CLKGATE
    pub mod PFD0_DIV1_CLKGATE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD1_FRAC
    pub mod PFD1_FRAC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD1_STABLE
    pub mod PFD1_STABLE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD1_DIV1_CLKGATE
    pub mod PFD1_DIV1_CLKGATE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD2_FRAC
    pub mod PFD2_FRAC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD2_STABLE
    pub mod PFD2_STABLE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD2_DIV1_CLKGATE
    pub mod PFD2_DIV1_CLKGATE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD3_FRAC
    pub mod PFD3_FRAC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (6 bits: 0x3f << 24)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD3_STABLE
    pub mod PFD3_STABLE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFD3_DIV1_CLKGATE
    pub mod PFD3_DIV1_CLKGATE {
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

/// SYS_PLL2_MFD_REGISTER
pub mod SYS_PLL2_MFD {

    /// Denominator
    pub mod MFD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (30 bits: 0x3fffffff << 0)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SYS_PLL1_SS_REGISTER
pub mod SYS_PLL1_SS {
    pub use super::SYS_PLL2_SS::ENABLE;
    pub use super::SYS_PLL2_SS::STEP;
    pub use super::SYS_PLL2_SS::STOP;
}

/// SYS_PLL1_CTRL_REGISTER
pub mod SYS_PLL1_CTRL {

    /// ENABLE_CLK
    pub mod ENABLE_CLK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SYS_PLL1_GATE
    pub mod SYS_PLL1_GATE {
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

            /// 0b0: No gate
            pub const NOGATE: u32 = 0b0;

            /// 0b1: Gate the output
            pub const GATED: u32 = 0b1;
        }
    }

    /// SYS_PLL1_DIV2
    pub mod SYS_PLL1_DIV2 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SYS_PLL1_DIV5
    pub mod SYS_PLL1_DIV5 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SYS_PLL1_DIV5_CONTROL_MODE
    pub mod SYS_PLL1_DIV5_CONTROL_MODE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Software Mode (Default)
            pub const SW: u32 = 0b0;

            /// 0b1: GPC Mode
            pub const GPC: u32 = 0b1;
        }
    }

    /// SYS_PLL1_DIV2_CONTROL_MODE
    pub mod SYS_PLL1_DIV2_CONTROL_MODE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYS_PLL1_DIV5_CONTROL_MODE::RW;
    }

    /// SYS_PLL1_STABLE
    pub mod SYS_PLL1_STABLE {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SYS_PLL1_AI_BUSY
    pub mod SYS_PLL1_AI_BUSY {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SYS_PLL1_CONTROL_MODE
    pub mod SYS_PLL1_CONTROL_MODE {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SYS_PLL1_DIV5_CONTROL_MODE::RW;
    }
}

/// SYS_PLL1_DENOMINATOR_REGISTER
pub mod SYS_PLL1_DENOMINATOR {

    /// DENOM
    pub mod DENOM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (30 bits: 0x3fffffff << 0)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SYS_PLL1_NUMERATOR_REGISTER
pub mod SYS_PLL1_NUMERATOR {

    /// NUM
    pub mod NUM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (30 bits: 0x3fffffff << 0)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SYS_PLL1_DIV_SELECT_REGISTER
pub mod SYS_PLL1_DIV_SELECT {

    /// DIV_SELECT
    pub mod DIV_SELECT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PLL_AUDIO_CTRL_REGISTER
pub mod PLL_AUDIO_CTRL {

    /// ENABLE_CLK
    pub mod ENABLE_CLK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL_AUDIO_GATE
    pub mod PLL_AUDIO_GATE {
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

            /// 0b0: No gate
            pub const NOGATE: u32 = 0b0;

            /// 0b1: Gate the output
            pub const GATED: u32 = 0b1;
        }
    }

    /// PLL_AUDIO_STABLE
    pub mod PLL_AUDIO_STABLE {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// pll_audio_ai_busy
    pub mod PLL_AUDIO_AI_BUSY {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// pll_audio_control_mode
    pub mod PLL_AUDIO_CONTROL_MODE {
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

            /// 0b0: Software Mode (Default)
            pub const SW: u32 = 0b0;

            /// 0b1: GPC Mode
            pub const GPC: u32 = 0b1;
        }
    }
}

/// PLL_AUDIO_SS_REGISTER
pub mod PLL_AUDIO_SS {
    pub use super::SYS_PLL2_SS::ENABLE;
    pub use super::SYS_PLL2_SS::STEP;
    pub use super::SYS_PLL2_SS::STOP;
}

/// PLL_AUDIO_DENOMINATOR_REGISTER
pub mod PLL_AUDIO_DENOMINATOR {
    pub use super::SYS_PLL1_DENOMINATOR::DENOM;
}

/// PLL_AUDIO_NUMERATOR_REGISTER
pub mod PLL_AUDIO_NUMERATOR {
    pub use super::SYS_PLL1_NUMERATOR::NUM;
}

/// PLL_AUDIO_DIV_SELECT_REGISTER
pub mod PLL_AUDIO_DIV_SELECT {

    /// PLL_AUDIO_DIV_SELECT
    pub mod PLL_AUDIO_DIV_SELECT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PLL_VIDEO_CTRL_REGISTER
pub mod PLL_VIDEO_CTRL {

    /// ENABLE_CLK
    pub mod ENABLE_CLK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL_VIDEO_GATE
    pub mod PLL_VIDEO_GATE {
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

            /// 0b0: No gate
            pub const NOGATE: u32 = 0b0;

            /// 0b1: Gate the output
            pub const GATED: u32 = 0b1;
        }
    }

    /// pll_video_counter_clr
    pub mod PLL_VIDEO_COUNTER_CLR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLL_VIDEO_STABLE
    pub mod PLL_VIDEO_STABLE {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// pll_video_ai_busy
    pub mod PLL_VIDEO_AI_BUSY {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// pll_video_control_mode
    pub mod PLL_VIDEO_CONTROL_MODE {
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

            /// 0b0: Software Mode (Default)
            pub const SW: u32 = 0b0;

            /// 0b1: GPC Mode
            pub const GPC: u32 = 0b1;
        }
    }
}

/// PLL_VIDEO_SS_REGISTER
pub mod PLL_VIDEO_SS {
    pub use super::SYS_PLL2_SS::ENABLE;
    pub use super::SYS_PLL2_SS::STEP;
    pub use super::SYS_PLL2_SS::STOP;
}

/// PLL_VIDEO_DENOMINATOR_REGISTER
pub mod PLL_VIDEO_DENOMINATOR {
    pub use super::SYS_PLL1_DENOMINATOR::DENOM;
}

/// PLL_VIDEO_NUMERATOR_REGISTER
pub mod PLL_VIDEO_NUMERATOR {
    pub use super::SYS_PLL1_NUMERATOR::NUM;
}

/// PLL_VIDEO_DIV_SELECT_REGISTER
pub mod PLL_VIDEO_DIV_SELECT {
    pub use super::SYS_PLL1_DIV_SELECT::DIV_SELECT;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 128],

    /// ARM_PLL_CTRL_REGISTER
    pub ARM_PLL_CTRL: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// SYS_PLL3_CTRL_REGISTER
    pub SYS_PLL3_CTRL: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// SYS_PLL3_UPDATE_REGISTER
    pub SYS_PLL3_UPDATE: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// SYS_PLL3_PFD_REGISTER
    pub SYS_PLL3_PFD: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// SYS_PLL2_CTRL_REGISTER
    pub SYS_PLL2_CTRL: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// SYS_PLL2_UPDATE_REGISTER
    pub SYS_PLL2_UPDATE: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// SYS_PLL2_SS_REGISTER
    pub SYS_PLL2_SS: RWRegister<u32>,

    _reserved8: [u32; 3],

    /// SYS_PLL2_PFD_REGISTER
    pub SYS_PLL2_PFD: RWRegister<u32>,

    _reserved9: [u32; 11],

    /// SYS_PLL2_MFD_REGISTER
    pub SYS_PLL2_MFD: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// SYS_PLL1_SS_REGISTER
    pub SYS_PLL1_SS: RWRegister<u32>,

    _reserved11: [u32; 3],

    /// SYS_PLL1_CTRL_REGISTER
    pub SYS_PLL1_CTRL: RWRegister<u32>,

    _reserved12: [u32; 3],

    /// SYS_PLL1_DENOMINATOR_REGISTER
    pub SYS_PLL1_DENOMINATOR: RWRegister<u32>,

    _reserved13: [u32; 3],

    /// SYS_PLL1_NUMERATOR_REGISTER
    pub SYS_PLL1_NUMERATOR: RWRegister<u32>,

    _reserved14: [u32; 3],

    /// SYS_PLL1_DIV_SELECT_REGISTER
    pub SYS_PLL1_DIV_SELECT: RWRegister<u32>,

    _reserved15: [u32; 3],

    /// PLL_AUDIO_CTRL_REGISTER
    pub PLL_AUDIO_CTRL: RWRegister<u32>,

    _reserved16: [u32; 3],

    /// PLL_AUDIO_SS_REGISTER
    pub PLL_AUDIO_SS: RWRegister<u32>,

    _reserved17: [u32; 3],

    /// PLL_AUDIO_DENOMINATOR_REGISTER
    pub PLL_AUDIO_DENOMINATOR: RWRegister<u32>,

    _reserved18: [u32; 3],

    /// PLL_AUDIO_NUMERATOR_REGISTER
    pub PLL_AUDIO_NUMERATOR: RWRegister<u32>,

    _reserved19: [u32; 3],

    /// PLL_AUDIO_DIV_SELECT_REGISTER
    pub PLL_AUDIO_DIV_SELECT: RWRegister<u32>,

    _reserved20: [u32; 3],

    /// PLL_VIDEO_CTRL_REGISTER
    pub PLL_VIDEO_CTRL: RWRegister<u32>,

    _reserved21: [u32; 3],

    /// PLL_VIDEO_SS_REGISTER
    pub PLL_VIDEO_SS: RWRegister<u32>,

    _reserved22: [u32; 3],

    /// PLL_VIDEO_DENOMINATOR_REGISTER
    pub PLL_VIDEO_DENOMINATOR: RWRegister<u32>,

    _reserved23: [u32; 3],

    /// PLL_VIDEO_NUMERATOR_REGISTER
    pub PLL_VIDEO_NUMERATOR: RWRegister<u32>,

    _reserved24: [u32; 3],

    /// PLL_VIDEO_DIV_SELECT_REGISTER
    pub PLL_VIDEO_DIV_SELECT: RWRegister<u32>,
}
pub struct ResetValues {
    pub ARM_PLL_CTRL: u32,
    pub SYS_PLL3_CTRL: u32,
    pub SYS_PLL3_UPDATE: u32,
    pub SYS_PLL3_PFD: u32,
    pub SYS_PLL2_CTRL: u32,
    pub SYS_PLL2_UPDATE: u32,
    pub SYS_PLL2_SS: u32,
    pub SYS_PLL2_PFD: u32,
    pub SYS_PLL2_MFD: u32,
    pub SYS_PLL1_SS: u32,
    pub SYS_PLL1_CTRL: u32,
    pub SYS_PLL1_DENOMINATOR: u32,
    pub SYS_PLL1_NUMERATOR: u32,
    pub SYS_PLL1_DIV_SELECT: u32,
    pub PLL_AUDIO_CTRL: u32,
    pub PLL_AUDIO_SS: u32,
    pub PLL_AUDIO_DENOMINATOR: u32,
    pub PLL_AUDIO_NUMERATOR: u32,
    pub PLL_AUDIO_DIV_SELECT: u32,
    pub PLL_VIDEO_CTRL: u32,
    pub PLL_VIDEO_SS: u32,
    pub PLL_VIDEO_DENOMINATOR: u32,
    pub PLL_VIDEO_NUMERATOR: u32,
    pub PLL_VIDEO_DIV_SELECT: u32,
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
