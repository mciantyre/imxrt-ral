#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC GPR
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// GPR0 General Purpose Register
pub mod GPR0 {

    /// SAI1 MCLK1 source select
    pub mod SAI1_MCLK1_SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI1 MCLK2 source select
    pub mod SAI1_MCLK2_SEL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI1 MCLK3 source select
    pub mod SAI1_MCLK3_SEL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI1_MCLK signal direction control
    pub mod SAI1_MCLK_DIR {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR1 General Purpose Register
pub mod GPR1 {

    /// SAI2 MCLK3 source select
    pub mod SAI2_MCLK3_SEL {
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

    /// SAI2_MCLK signal direction control
    pub mod SAI2_MCLK_DIR {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR2 General Purpose Register
pub mod GPR2 {

    /// SAI3 MCLK3 source select
    pub mod SAI3_MCLK3_SEL {
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

    /// SAI3_MCLK signal direction control
    pub mod SAI3_MCLK_DIR {
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

    /// SAI4_MCLK signal direction control
    pub mod SAI4_MCLK_DIR {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR3 General Purpose Register
pub mod GPR3 {

    /// Divider ratio control for mclk from hmclk.
    pub mod MQS_CLK_DIV {
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

    /// MQS software reset
    pub mod MQS_SW_RST {
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

    /// MQS enable
    pub mod MQS_EN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Medium Quality Sound (MQS) Oversample
    pub mod MQS_OVERSAMPLE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR4 General Purpose Register
pub mod GPR4 {

    /// ENET TX_CLK select
    pub mod ENET_TX_CLK_SEL {
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

    /// ENET_REF_CLK direction control
    pub mod ENET_REF_CLK_DIR {
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

    /// ENET master timer source select
    pub mod ENET_TIME_SEL {
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

    /// ENET ENET_1588_EVENT0_IN source select
    pub mod ENET_EVENT0IN_SEL {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR5 General Purpose Register
pub mod GPR5 {

    /// ENET1G TX_CLK select
    pub mod ENET1G_TX_CLK_SEL {
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

    /// ENET1G_REF_CLK direction control
    pub mod ENET1G_REF_CLK_DIR {
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

    /// ENET1G RGMII TX clock output enable
    pub mod ENET1G_RGMII_EN {
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

    /// ENET1G master timer source select
    pub mod ENET1G_TIME_SEL {
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

    /// ENET1G ENET_1588_EVENT0_IN source select
    pub mod ENET1G_EVENT0IN_SEL {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR6 General Purpose Register
pub mod GPR6 {

    /// ENET_QOS_REF_CLK direction control
    pub mod ENET_QOS_REF_CLK_DIR {
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

    /// ENET_QOS RGMII TX clock output enable
    pub mod ENET_QOS_RGMII_EN {
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

    /// ENET_QOS master timer source select
    pub mod ENET_QOS_TIME_SEL {
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

    /// ENET_QOS PHY Interface Select
    pub mod ENET_QOS_INTF_SEL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENET_QOS clock generator enable
    pub mod ENET_QOS_CLKGEN_EN {
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

    /// ENET_QOS ENET_1588_EVENT0_IN source select
    pub mod ENET_QOS_EVENT0IN_SEL {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR7 General Purpose Register
pub mod GPR7 {

    /// Global interrupt
    pub mod GINT {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR8 General Purpose Register
pub mod GPR8 {

    /// WDOG1 timeout mask for WDOG_ANY
    pub mod WDOG1_MASK {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR9 General Purpose Register
pub mod GPR9 {

    /// WDOG2 timeout mask for WDOG_ANY
    pub mod WDOG2_MASK {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR10 General Purpose Register
pub mod GPR10 {

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR11 General Purpose Register
pub mod GPR11 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR12 General Purpose Register
pub mod GPR12 {

    /// QTIMER1 timer counter freeze
    pub mod QTIMER1_TMR_CNTS_FREEZE {
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

    /// QTIMER1 TMR0 input select
    pub mod QTIMER1_TRM0_INPUT_SEL {
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

    /// QTIMER1 TMR1 input select
    pub mod QTIMER1_TRM1_INPUT_SEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// QTIMER1 TMR2 input select
    pub mod QTIMER1_TRM2_INPUT_SEL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// QTIMER1 TMR3 input select
    pub mod QTIMER1_TRM3_INPUT_SEL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR13 General Purpose Register
pub mod GPR13 {

    /// QTIMER2 timer counter freeze
    pub mod QTIMER2_TMR_CNTS_FREEZE {
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

    /// QTIMER2 TMR0 input select
    pub mod QTIMER2_TRM0_INPUT_SEL {
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

    /// QTIMER2 TMR1 input select
    pub mod QTIMER2_TRM1_INPUT_SEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// QTIMER2 TMR2 input select
    pub mod QTIMER2_TRM2_INPUT_SEL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// QTIMER2 TMR3 input select
    pub mod QTIMER2_TRM3_INPUT_SEL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR14 General Purpose Register
pub mod GPR14 {

    /// QTIMER3 timer counter freeze
    pub mod QTIMER3_TMR_CNTS_FREEZE {
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

    /// QTIMER3 TMR0 input select
    pub mod QTIMER3_TRM0_INPUT_SEL {
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

    /// QTIMER3 TMR1 input select
    pub mod QTIMER3_TRM1_INPUT_SEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// QTIMER3 TMR2 input select
    pub mod QTIMER3_TRM2_INPUT_SEL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// QTIMER3 TMR3 input select
    pub mod QTIMER3_TRM3_INPUT_SEL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR15 General Purpose Register
pub mod GPR15 {

    /// QTIMER4 timer counter freeze
    pub mod QTIMER4_TMR_CNTS_FREEZE {
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

    /// QTIMER4 TMR0 input select
    pub mod QTIMER4_TRM0_INPUT_SEL {
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

    /// QTIMER4 TMR1 input select
    pub mod QTIMER4_TRM1_INPUT_SEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// QTIMER4 TMR2 input select
    pub mod QTIMER4_TRM2_INPUT_SEL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// QTIMER4 TMR3 input select
    pub mod QTIMER4_TRM3_INPUT_SEL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR16 General Purpose Register
pub mod GPR16 {

    /// FlexRAM bank config source select
    pub mod FLEXRAM_BANK_CFG_SEL {
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

    /// CM7 platform AHB clock enable
    pub mod CM7_FORCE_HCLK_EN {
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

    /// CM7 sleep request selection
    pub mod M7_GPC_SLEEP_SEL {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR17 General Purpose Register
pub mod GPR17 {

    /// FlexRAM bank config value
    pub mod FLEXRAM_BANK_CFG_LOW {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR18 General Purpose Register
pub mod GPR18 {

    /// FlexRAM bank config value
    pub mod FLEXRAM_BANK_CFG_HIGH {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR20 General Purpose Register
pub mod GPR20 {

    /// IOMUXC XBAR_INOUT4 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_4 {
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

    /// IOMUXC XBAR_INOUT5 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_5 {
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

    /// IOMUXC XBAR_INOUT6 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_6 {
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

    /// IOMUXC XBAR_INOUT7 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_7 {
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

    /// IOMUXC XBAR_INOUT8 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_8 {
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

    /// IOMUXC XBAR_INOUT9 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_9 {
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

    /// IOMUXC XBAR_INOUT10 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_10 {
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

    /// IOMUXC XBAR_INOUT11 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_11 {
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

    /// IOMUXC XBAR_INOUT12 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_12 {
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

    /// IOMUXC XBAR_INOUT13 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_13 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IOMUXC XBAR_INOUT14 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_14 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IOMUXC XBAR_INOUT15 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_15 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IOMUXC XBAR_INOUT16 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_16 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IOMUXC XBAR_INOUT17 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_17 {
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

    /// IOMUXC XBAR_INOUT18 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_18 {
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

    /// IOMUXC XBAR_INOUT19 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_19 {
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

    /// IOMUXC XBAR_INOUT20 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_20 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IOMUXC XBAR_INOUT21 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_21 {
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

    /// IOMUXC XBAR_INOUT22 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_22 {
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

    /// IOMUXC XBAR_INOUT23 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_23 {
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

    /// IOMUXC XBAR_INOUT24 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_24 {
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

    /// IOMUXC XBAR_INOUT25 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_25 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IOMUXC XBAR_INOUT26 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_26 {
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

    /// IOMUXC XBAR_INOUT27 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_27 {
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

    /// IOMUXC XBAR_INOUT28 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_28 {
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

    /// IOMUXC XBAR_INOUT29 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_29 {
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

    /// IOMUXC XBAR_INOUT30 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_30 {
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

    /// IOMUXC XBAR_INOUT31 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_31 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR21 General Purpose Register
pub mod GPR21 {

    /// IOMUXC XBAR_INOUT32 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_32 {
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

    /// IOMUXC XBAR_INOUT33 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_33 {
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

    /// IOMUXC XBAR_INOUT34 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_34 {
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

    /// IOMUXC XBAR_INOUT35 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_35 {
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

    /// IOMUXC XBAR_INOUT36 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_36 {
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

    /// IOMUXC XBAR_INOUT37 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_37 {
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

    /// IOMUXC XBAR_INOUT38 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_38 {
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

    /// IOMUXC XBAR_INOUT39 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_39 {
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

    /// IOMUXC XBAR_INOUT40 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_40 {
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

    /// IOMUXC XBAR_INOUT41 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_41 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IOMUXC XBAR_INOUT42 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_42 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR22 General Purpose Register
pub mod GPR22 {

    /// GPT1 1 MHz clock source select
    pub mod REF_1M_CLK_GPT1 {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR23 General Purpose Register
pub mod GPR23 {

    /// GPT2 1 MHz clock source select
    pub mod REF_1M_CLK_GPT2 {
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

    /// GPT2 input capture channel 1 source select
    pub mod GPT2_CAPIN1_SEL {
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

    /// GPT2 input capture channel 2 source select
    pub mod GPT2_CAPIN2_SEL {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR24 General Purpose Register
pub mod GPR24 {

    /// GPT3 1 MHz clock source select
    pub mod REF_1M_CLK_GPT3 {
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

    /// GPT3 input capture channel 1 source select
    pub mod GPT3_CAPIN1_SEL {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR25 General Purpose Register
pub mod GPR25 {

    /// GPT4 1 MHz clock source select
    pub mod REF_1M_CLK_GPT4 {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR26 General Purpose Register
pub mod GPR26 {

    /// GPT5 1 MHz clock source select
    pub mod REF_1M_CLK_GPT5 {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR27 General Purpose Register
pub mod GPR27 {

    /// GPT6 1 MHz clock source select
    pub mod REF_1M_CLK_GPT6 {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR28 General Purpose Register
pub mod GPR28 {

    /// uSDHC block cacheable attribute value of AXI read transactions
    pub mod ARCACHE_USDHC {
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

    /// uSDHC block cacheable attribute value of AXI write transactions
    pub mod AWCACHE_USDHC {
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

    /// no description available
    pub mod CACHE_ENET1G {
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

    /// ENET block cacheable attribute value of AXI transactions
    pub mod CACHE_ENET {
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

    /// USB block cacheable attribute value of AXI transactions
    pub mod CACHE_USB {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR29 General Purpose Register
pub mod GPR29 {

    /// USBPHY1 register access clock enable
    pub mod USBPHY1_IPG_CLK_ACTIVE {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR30 General Purpose Register
pub mod GPR30 {

    /// USBPHY2 register access clock enable
    pub mod USBPHY2_IPG_CLK_ACTIVE {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR31 General Purpose Register
pub mod GPR31 {

    /// OCRAM M7 RMW wait enable
    pub mod RMW2_WAIT_BVALID_CPL {
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

    /// OCRAM M7 clock gating enable
    pub mod OCRAM_M7_CLK_GATING {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR32 General Purpose Register
pub mod GPR32 {

    /// OCRAM1 RMW wait enable
    pub mod RMW1_WAIT_BVALID_CPL {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR33 General Purpose Register
pub mod GPR33 {

    /// OCRAM2 RMW wait enable
    pub mod RMW2_WAIT_BVALID_CPL {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR34 General Purpose Register
pub mod GPR34 {

    /// XECC_FLEXSPI1 RMW wait enable
    pub mod XECC_FLEXSPI1_WAIT_BVALID_CPL {
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

    /// FlexSPI1 OTFAD enable
    pub mod FLEXSPI1_OTFAD_EN {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR35 General Purpose Register
pub mod GPR35 {

    /// XECC_FLEXSPI2 RMW wait enable
    pub mod XECC_FLEXSPI2_WAIT_BVALID_CPL {
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

    /// FlexSPI2 OTFAD enable
    pub mod FLEXSPI2_OTFAD_EN {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR36 General Purpose Register
pub mod GPR36 {

    /// XECC_SEMC RMW wait enable
    pub mod XECC_SEMC_WAIT_BVALID_CPL {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR37 General Purpose Register
pub mod GPR37 {

    /// ARM non-secure (non-invasive) debug enable
    pub mod NIDEN {
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

    /// ARM invasive debug enable
    pub mod DBG_EN {
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

    /// Exclusive monitor response select of illegal command
    pub mod EXC_MON {
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

    /// CM7 debug halt mask
    pub mod M7_DBG_ACK_MASK {
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

    /// CM4 debug halt mask
    pub mod M4_DBG_ACK_MASK {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR38 General Purpose Register
pub mod GPR38 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR39 General Purpose Register
pub mod GPR39 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR40 General Purpose Register
pub mod GPR40 {

    /// GPIO2 and CM7_GPIO2 share same IO MUX function, GPIO_MUX2 selects one GPIO function.
    pub mod GPIO_MUX2_GPIO_SEL_LOW {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR41 General Purpose Register
pub mod GPR41 {

    /// GPIO2 and CM7_GPIO2 share same IO MUX function, GPIO_MUX2 selects one GPIO function.
    pub mod GPIO_MUX2_GPIO_SEL_HIGH {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR42 General Purpose Register
pub mod GPR42 {

    /// GPIO3 and CM7_GPIO3 share same IO MUX function, GPIO_MUX3 selects one GPIO function.
    pub mod GPIO_MUX3_GPIO_SEL_LOW {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR43 General Purpose Register
pub mod GPR43 {

    /// GPIO3 and CM7_GPIO3 share same IO MUX function, GPIO_MUX3 selects one GPIO function.
    pub mod GPIO_MUX3_GPIO_SEL_HIGH {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR44 General Purpose Register
pub mod GPR44 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR45 General Purpose Register
pub mod GPR45 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR46 General Purpose Register
pub mod GPR46 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR47 General Purpose Register
pub mod GPR47 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR48 General Purpose Register
pub mod GPR48 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR49 General Purpose Register
pub mod GPR49 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR50 General Purpose Register
pub mod GPR50 {

    /// CAAM manager processor identifier
    pub mod CAAM_IPS_MGR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR51 General Purpose Register
pub mod GPR51 {

    /// Clear CM7 NMI holding register
    pub mod M7_NMI_CLEAR {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR52 General Purpose Register
pub mod GPR52 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR53 General Purpose Register
pub mod GPR53 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR54 General Purpose Register
pub mod GPR54 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR55 General Purpose Register
pub mod GPR55 {
    pub use super::GPR10::DWP;
    pub use super::GPR10::DWP_LOCK;
}

/// GPR59 General Purpose Register
pub mod GPR59 {

    /// Powers down inactive lanes reported by CSI2X_CFG_NUM_LANES.
    pub mod MIPI_CSI_AUTO_PD_EN {
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

    /// MIPI CSI APB clock domain and User interface clock domain software reset bit
    pub mod MIPI_CSI_SOFT_RST_N {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Assert reset
            pub const ASSERT: u32 = 0b0;

            /// 0b1: De-assert reset
            pub const DEAST: u32 = 0b1;
        }
    }

    /// Enables the slave clock lane feature to maintain HS reception state during continuous clock mode operation, despite line glitches.
    pub mod MIPI_CSI_CONT_CLK_MODE {
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

    /// When high, enables received DDR clock on CLK_DRXHS
    pub mod MIPI_CSI_DDRCLK_EN {
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

    /// Power Down input for MIPI CSI PHY.
    pub mod MIPI_CSI_PD_RX {
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

    /// Assert to enable MIPI CSI Receive Enable
    pub mod MIPI_CSI_RX_ENABLE {
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

    /// MIPI CSI PHY on-chip termination control bits
    pub mod MIPI_CSI_RX_RCAL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Programming bits that adjust the threshold voltage of LP-CD, default setting 2'b01
    pub mod MIPI_CSI_RXCDRP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 344mV
            pub const VAL0: u32 = 0b00;

            /// 0b01: 325mV (Default)
            pub const VAL01: u32 = 0b01;

            /// 0b10: 307mV
            pub const VAL10: u32 = 0b10;

            /// 0b11: Invalid
            pub const VAL11: u32 = 0b11;
        }
    }

    /// Programming bits that adjust the threshold voltage of LP-RX, default setting 2'b01
    pub mod MIPI_CSI_RXLPRP {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bits used to program T_HS_SETTLE.
    pub mod MIPI_CSI_S_PRG_RXHS_SETTLE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (6 bits: 0x3f << 12)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR62 General Purpose Register
pub mod GPR62 {

    /// MIPI DSI Clock Lane triming bits
    pub mod MIPI_DSI_CLK_TM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MIPI DSI Data Lane 0 triming bits
    pub mod MIPI_DSI_D0_TM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MIPI DSI Data Lane 1 triming bits
    pub mod MIPI_DSI_D1_TM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MIPI DSI PHY on-chip termination control bits
    pub mod MIPI_DSI_TX_RCAL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DSI transmit ULPS mode enable
    pub mod MIPI_DSI_TX_ULPS_ENABLE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (3 bits: 0b111 << 11)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MIPI DSI APB clock domain software reset bit
    pub mod MIPI_DSI_PCLK_SOFT_RESET_N {
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

            /// 0b0: Assert reset
            pub const ASSERT: u32 = 0b0;

            /// 0b1: De-assert reset
            pub const DEASSERT: u32 = 0b1;
        }
    }

    /// MIPI DSI Byte clock domain software reset bit
    pub mod MIPI_DSI_BYTE_SOFT_RESET_N {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MIPI_DSI_PCLK_SOFT_RESET_N::RW;
    }

    /// MIPI DSI Pixel clock domain software reset bit
    pub mod MIPI_DSI_DPI_SOFT_RESET_N {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MIPI_DSI_PCLK_SOFT_RESET_N::RW;
    }

    /// MIPI DSI Escape clock domain software reset bit
    pub mod MIPI_DSI_ESC_SOFT_RESET_N {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MIPI_DSI_PCLK_SOFT_RESET_N::RW;
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR63 General Purpose Register
pub mod GPR63 {

    /// DSI transmit ULPS mode active flag
    pub mod MIPI_DSI_TX_ULPS_ACTIVE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR64 General Purpose Register
pub mod GPR64 {

    /// Compensation code freeze
    pub mod GPIO_DISP1_FREEZE {
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

    /// COMPEN and COMPTQ control the operating modes of the compensation cell
    pub mod GPIO_DISP1_COMPTQ {
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

    /// COMPEN and COMPTQ control the operating modes of the compensation cell
    pub mod GPIO_DISP1_COMPEN {
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

    /// Compensation code fast freeze
    pub mod GPIO_DISP1_FASTFRZ_EN {
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

    /// GPIO_DISP_B1 IO bank's 4-bit PMOS compensation codes from core
    pub mod GPIO_DISP1_RASRCP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_DISP_B1 IO bank's 4-bit NMOS compensation codes from core
    pub mod GPIO_DISP1_RASRCN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_DISP1_NASRC selection
    pub mod GPIO_DISP1_SELECT_NASRC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_DISP_B1 IO bank reference voltage generator cell sleep enable
    pub mod GPIO_DISP1_REFGEN_SLEEP {
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

    /// GPIO_DISP_B1 IO bank power supply mode latch enable
    pub mod GPIO_DISP1_SUPLYDET_LATCH {
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

    /// GPIO_DISP_B1 IO bank compensation OK flag
    pub mod GPIO_DISP1_COMPOK {
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

    /// GPIO_DISP_B1 IO bank compensation codes
    pub mod GPIO_DISP1_NASRC {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (4 bits: 0b1111 << 21)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR65 General Purpose Register
pub mod GPR65 {

    /// Compensation code freeze
    pub mod GPIO_EMC1_FREEZE {
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

    /// COMPEN and COMPTQ control the operating modes of the compensation cell
    pub mod GPIO_EMC1_COMPTQ {
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

    /// COMPEN and COMPTQ control the operating modes of the compensation cell
    pub mod GPIO_EMC1_COMPEN {
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

    /// Compensation code fast freeze
    pub mod GPIO_EMC1_FASTFRZ_EN {
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

    /// GPIO_EMC_B1 IO bank's 4-bit PMOS compensation codes from core
    pub mod GPIO_EMC1_RASRCP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_EMC_B1 IO bank's 4-bit NMOS compensation codes from core
    pub mod GPIO_EMC1_RASRCN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_EMC1_NASRC selection
    pub mod GPIO_EMC1_SELECT_NASRC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_EMC_B1 IO bank reference voltage generator cell sleep enable
    pub mod GPIO_EMC1_REFGEN_SLEEP {
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

    /// GPIO_EMC_B1 IO bank power supply mode latch enable
    pub mod GPIO_EMC1_SUPLYDET_LATCH {
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

    /// GPIO_EMC_B1 IO bank compensation OK flag
    pub mod GPIO_EMC1_COMPOK {
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

    /// GPIO_EMC_B1 IO bank compensation codes
    pub mod GPIO_EMC1_NASRC {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (4 bits: 0b1111 << 21)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR66 General Purpose Register
pub mod GPR66 {

    /// Compensation code freeze
    pub mod GPIO_EMC2_FREEZE {
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

    /// COMPEN and COMPTQ control the operating modes of the compensation cell
    pub mod GPIO_EMC2_COMPTQ {
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

    /// COMPEN and COMPTQ control the operating modes of the compensation cell
    pub mod GPIO_EMC2_COMPEN {
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

    /// Compensation code fast freeze
    pub mod GPIO_EMC2_FASTFRZ_EN {
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

    /// GPIO_EMC_B2 IO bank's 4-bit PMOS compensation codes from core
    pub mod GPIO_EMC2_RASRCP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_EMC_B2 IO bank's 4-bit NMOS compensation codes from core
    pub mod GPIO_EMC2_RASRCN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_EMC2_NASRC selection
    pub mod GPIO_EMC2_SELECT_NASRC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_EMC_B2 IO bank reference voltage generator cell sleep enable
    pub mod GPIO_EMC2_REFGEN_SLEEP {
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

    /// GPIO_EMC_B2 IO bank power supply mode latch enable
    pub mod GPIO_EMC2_SUPLYDET_LATCH {
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

    /// GPIO_EMC_B2 IO bank compensation OK flag
    pub mod GPIO_EMC2_COMPOK {
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

    /// GPIO_EMC_B2 IO bank compensation codes
    pub mod GPIO_EMC2_NASRC {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (4 bits: 0b1111 << 21)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR67 General Purpose Register
pub mod GPR67 {

    /// Compensation code freeze
    pub mod GPIO_SD1_FREEZE {
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

    /// COMPEN and COMPTQ control the operating modes of the compensation cell
    pub mod GPIO_SD1_COMPTQ {
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

    /// COMPEN and COMPTQ control the operating modes of the compensation cell
    pub mod GPIO_SD1_COMPEN {
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

    /// Compensation code fast freeze
    pub mod GPIO_SD1_FASTFRZ_EN {
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

    /// GPIO_SD_B1 IO bank's 4-bit PMOS compensation codes from core
    pub mod GPIO_SD1_RASRCP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_SD_B1 IO bank's 4-bit NMOS compensation codes from core
    pub mod GPIO_SD1_RASRCN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_SD1_NASRC selection
    pub mod GPIO_SD1_SELECT_NASRC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_SD_B1 IO bank reference voltage generator cell sleep enable
    pub mod GPIO_SD1_REFGEN_SLEEP {
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

    /// GPIO_SD_B1 IO bank power supply mode latch enable
    pub mod GPIO_SD1_SUPLYDET_LATCH {
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

    /// GPIO_SD_B1 IO bank compensation OK flag
    pub mod GPIO_SD1_COMPOK {
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

    /// GPIO_SD_B1 IO bank compensation codes
    pub mod GPIO_SD1_NASRC {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (4 bits: 0b1111 << 21)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR68 General Purpose Register
pub mod GPR68 {

    /// Compensation code freeze
    pub mod GPIO_SD2_FREEZE {
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

    /// COMPEN and COMPTQ control the operating modes of the compensation cell
    pub mod GPIO_SD2_COMPTQ {
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

    /// COMPEN and COMPTQ control the operating modes of the compensation cell
    pub mod GPIO_SD2_COMPEN {
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

    /// Compensation code fast freeze
    pub mod GPIO_SD2_FASTFRZ_EN {
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

    /// GPIO_SD_B2 IO bank's 4-bit PMOS compensation codes from core
    pub mod GPIO_SD2_RASRCP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_SD_B2 IO bank's 4-bit NMOS compensation codes from core
    pub mod GPIO_SD2_RASRCN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_SD2_NASRC selection
    pub mod GPIO_SD2_SELECT_NASRC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_SD_B2 IO bank reference voltage generator cell sleep enable
    pub mod GPIO_SD2_REFGEN_SLEEP {
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

    /// GPIO_SD_B2 IO bank power supply mode latch enable
    pub mod GPIO_SD2_SUPLYDET_LATCH {
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

    /// GPIO_SD_B2 IO bank compensation OK flag
    pub mod GPIO_SD2_COMPOK {
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

    /// GPIO_SD_B2 IO bank compensation codes
    pub mod GPIO_SD2_NASRC {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (4 bits: 0b1111 << 21)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR69 General Purpose Register
pub mod GPR69 {

    /// GPIO_DISP_B2 IO bank supply voltage range selection
    pub mod GPIO_DISP2_HIGH_RANGE {
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

    /// GPIO_DISP_B2 IO bank supply voltage range selection
    pub mod GPIO_DISP2_LOW_RANGE {
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

    /// GPIO_AD IO bank supply voltage range selection for GPIO_AD_00 to GPIO_AD_17
    pub mod GPIO_AD0_HIGH_RANGE {
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

    /// GPIO_AD IO bank supply voltage range selection for GPIO_AD_00 to GPIO_AD_17
    pub mod GPIO_AD0_LOW_RANGE {
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

    /// GPIO_LPSR IO bank supply voltage range selection for GPIO_AD_18 to GPIO_AD_35
    pub mod GPIO_AD1_HIGH_RANGE {
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

    /// GPIO_LPSR IO bank supply voltage range selection for GPIO_AD_18 to GPIO_AD_35
    pub mod GPIO_AD1_LOW_RANGE {
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

    /// GPIO_DISP_B1 IO bank supply voltage detector sleep mode enable
    pub mod SUPLYDET_DISP1_SLEEP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_EMC_B1 IO bank supply voltage detector sleep mode enable
    pub mod SUPLYDET_EMC1_SLEEP {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_EMC_B2 IO bank supply voltage detector sleep mode enable
    pub mod SUPLYDET_EMC2_SLEEP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_SD_B1 IO bank supply voltage detector sleep mode enable
    pub mod SUPLYDET_SD1_SLEEP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPIO_SD_B2 IO bank supply voltage detector sleep mode enable
    pub mod SUPLYDET_SD2_SLEEP {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR70 General Purpose Register
pub mod GPR70 {

    /// ADC1 doze mode
    pub mod ADC1_IPG_DOZE {
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

    /// ADC1 stop request
    pub mod ADC1_STOP_REQ {
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

    /// ADC1 stop mode selection, cannot change when ADC1_STOP_REQ is asserted.
    pub mod ADC1_IPG_STOP_MODE {
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

            /// 0b0: This module is functional in Stop Mode
            pub const FUNC: u32 = 0b0;

            /// 0b1: This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'.
            pub const NONFUNC: u32 = 0b1;
        }
    }

    /// ADC2 doze mode
    pub mod ADC2_IPG_DOZE {
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

    /// ADC2 stop request
    pub mod ADC2_STOP_REQ {
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

    /// ADC2 stop mode selection, cannot change when ADC2_STOP_REQ is asserted.
    pub mod ADC2_IPG_STOP_MODE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_STOP_MODE::RW;
    }

    /// CAN3 doze mode
    pub mod CAAM_IPG_DOZE {
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

    /// CAAM stop request
    pub mod CAAM_STOP_REQ {
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

    /// CAN1 doze mode
    pub mod CAN1_IPG_DOZE {
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

    /// CAN1 stop request
    pub mod CAN1_STOP_REQ {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CAN2 doze mode
    pub mod CAN2_IPG_DOZE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CAN2 stop request
    pub mod CAN2_STOP_REQ {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CAN3 doze mode
    pub mod CAN3_IPG_DOZE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CAN3 stop request
    pub mod CAN3_STOP_REQ {
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

    /// EDMA stop request
    pub mod EDMA_STOP_REQ {
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

    /// EDMA_LPSR stop request
    pub mod EDMA_LPSR_STOP_REQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENET doze mode
    pub mod ENET_IPG_DOZE {
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

    /// ENET stop request
    pub mod ENET_STOP_REQ {
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

    /// ENET1G doze mode
    pub mod ENET1G_IPG_DOZE {
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

    /// ENET1G stop request
    pub mod ENET1G_STOP_REQ {
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

    /// FLEXIO2 doze mode
    pub mod FLEXIO1_IPG_DOZE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FLEXIO2 doze mode
    pub mod FLEXIO2_IPG_DOZE {
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

    /// FLEXSPI1 doze mode
    pub mod FLEXSPI1_IPG_DOZE {
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

    /// FLEXSPI1 stop request
    pub mod FLEXSPI1_STOP_REQ {
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

    /// FLEXSPI2 doze mode
    pub mod FLEXSPI2_IPG_DOZE {
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

    /// FLEXSPI2 stop request
    pub mod FLEXSPI2_STOP_REQ {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR71 General Purpose Register
pub mod GPR71 {

    /// GPT1 doze mode
    pub mod GPT1_IPG_DOZE {
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

    /// GPT2 doze mode
    pub mod GPT2_IPG_DOZE {
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

    /// GPT3 doze mode
    pub mod GPT3_IPG_DOZE {
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

    /// GPT4 doze mode
    pub mod GPT4_IPG_DOZE {
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

    /// GPT5 doze mode
    pub mod GPT5_IPG_DOZE {
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

    /// GPT6 doze mode
    pub mod GPT6_IPG_DOZE {
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

    /// LPI2C1 doze mode
    pub mod LPI2C1_IPG_DOZE {
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

    /// LPI2C1 stop request
    pub mod LPI2C1_STOP_REQ {
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

    /// LPI2C1 stop mode selection, cannot change when LPI2C1_STOP_REQ is asserted.
    pub mod LPI2C1_IPG_STOP_MODE {
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

            /// 0b0: This module is functional in Stop Mode
            pub const FUNC: u32 = 0b0;

            /// 0b1: This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'.
            pub const NONFUNC: u32 = 0b1;
        }
    }

    /// LPI2C2 doze mode
    pub mod LPI2C2_IPG_DOZE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPI2C2 stop request
    pub mod LPI2C2_STOP_REQ {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPI2C2 stop mode selection, cannot change when LPI2C2_STOP_REQ is asserted.
    pub mod LPI2C2_IPG_STOP_MODE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_IPG_STOP_MODE::RW;
    }

    /// LPI2C3 doze mode
    pub mod LPI2C3_IPG_DOZE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPI2C3 stop request
    pub mod LPI2C3_STOP_REQ {
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

    /// LPI2C3 stop mode selection, cannot change when LPI2C3_STOP_REQ is asserted.
    pub mod LPI2C3_IPG_STOP_MODE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_IPG_STOP_MODE::RW;
    }

    /// LPI2C4 doze mode
    pub mod LPI2C4_IPG_DOZE {
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

    /// LPI2C4 stop request
    pub mod LPI2C4_STOP_REQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPI2C4 stop mode selection, cannot change when LPI2C4_STOP_REQ is asserted.
    pub mod LPI2C4_IPG_STOP_MODE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_IPG_STOP_MODE::RW;
    }

    /// LPI2C5 doze mode
    pub mod LPI2C5_IPG_DOZE {
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

    /// LPI2C5 stop request
    pub mod LPI2C5_STOP_REQ {
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

    /// LPI2C5 stop mode selection, cannot change when LPI2C5_STOP_REQ is asserted.
    pub mod LPI2C5_IPG_STOP_MODE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_IPG_STOP_MODE::RW;
    }

    /// LPI2C6 doze mode
    pub mod LPI2C6_IPG_DOZE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPI2C6 stop request
    pub mod LPI2C6_STOP_REQ {
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

    /// LPI2C6 stop mode selection, cannot change when LPI2C6_STOP_REQ is asserted.
    pub mod LPI2C6_IPG_STOP_MODE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_IPG_STOP_MODE::RW;
    }

    /// LPSPI1 doze mode
    pub mod LPSPI1_IPG_DOZE {
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

    /// LPSPI1 stop request
    pub mod LPSPI1_STOP_REQ {
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

    /// LPSPI1 stop mode selection, cannot change when LPSPI1_STOP_REQ is asserted.
    pub mod LPSPI1_IPG_STOP_MODE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_IPG_STOP_MODE::RW;
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR72 General Purpose Register
pub mod GPR72 {

    /// LPSPI2 doze mode
    pub mod LPSPI2_IPG_DOZE {
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

    /// LPSPI2 stop request
    pub mod LPSPI2_STOP_REQ {
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

    /// LPSPI2 stop mode selection, cannot change when LPSPI2_STOP_REQ is asserted.
    pub mod LPSPI2_IPG_STOP_MODE {
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

            /// 0b0: This module is functional in Stop Mode
            pub const FUNC: u32 = 0b0;

            /// 0b1: This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'.
            pub const NONFUNC: u32 = 0b1;
        }
    }

    /// LPSPI3 doze mode
    pub mod LPSPI3_IPG_DOZE {
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

    /// LPSPI3 stop request
    pub mod LPSPI3_STOP_REQ {
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

    /// LPSPI3 stop mode selection, cannot change when LPSPI3_STOP_REQ is asserted.
    pub mod LPSPI3_IPG_STOP_MODE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPSPI4 doze mode
    pub mod LPSPI4_IPG_DOZE {
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

    /// LPSPI4 stop request
    pub mod LPSPI4_STOP_REQ {
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

    /// LPSPI4 stop mode selection, cannot change when LPSPI4_STOP_REQ is asserted.
    pub mod LPSPI4_IPG_STOP_MODE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPSPI5 doze mode
    pub mod LPSPI5_IPG_DOZE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPSPI5 stop request
    pub mod LPSPI5_STOP_REQ {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPSPI5 stop mode selection, cannot change when LPSPI5_STOP_REQ is asserted.
    pub mod LPSPI5_IPG_STOP_MODE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPSPI6 doze mode
    pub mod LPSPI6_IPG_DOZE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPSPI6 stop request
    pub mod LPSPI6_STOP_REQ {
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

    /// LPSPI6 stop mode selection, cannot change when LPSPI6_STOP_REQ is asserted.
    pub mod LPSPI6_IPG_STOP_MODE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPUART1 doze mode
    pub mod LPUART1_IPG_DOZE {
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

    /// LPUART1 stop request
    pub mod LPUART1_STOP_REQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPUART1 stop mode selection, cannot change when LPUART1_STOP_REQ is asserted.
    pub mod LPUART1_IPG_STOP_MODE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPUART2 doze mode
    pub mod LPUART2_IPG_DOZE {
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

    /// LPUART2 stop request
    pub mod LPUART2_STOP_REQ {
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

    /// LPUART2 stop mode selection, cannot change when LPUART2_STOP_REQ is asserted.
    pub mod LPUART2_IPG_STOP_MODE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPUART3 doze mode
    pub mod LPUART3_IPG_DOZE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPUART3 stop request
    pub mod LPUART3_STOP_REQ {
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

    /// LPUART3 stop mode selection, cannot change when LPUART3_STOP_REQ is asserted.
    pub mod LPUART3_IPG_STOP_MODE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPUART4 doze mode
    pub mod LPUART4_IPG_DOZE {
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

    /// LPUART4 stop request
    pub mod LPUART4_STOP_REQ {
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

    /// LPUART4 stop mode selection, cannot change when LPUART4_STOP_REQ is asserted.
    pub mod LPUART4_IPG_STOP_MODE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR73 General Purpose Register
pub mod GPR73 {

    /// LPUART5 doze mode
    pub mod LPUART5_IPG_DOZE {
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

    /// LPUART5 stop request
    pub mod LPUART5_STOP_REQ {
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

    /// LPUART5 stop mode selection, cannot change when LPUART5_STOP_REQ is asserted.
    pub mod LPUART5_IPG_STOP_MODE {
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

            /// 0b0: This module is functional in Stop Mode
            pub const FUNC: u32 = 0b0;

            /// 0b1: This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'.
            pub const NONFUNC: u32 = 0b1;
        }
    }

    /// LPUART6 doze mode
    pub mod LPUART6_IPG_DOZE {
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

    /// LPUART6 stop request
    pub mod LPUART6_STOP_REQ {
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

    /// LPUART6 stop mode selection, cannot change when LPUART6_STOP_REQ is asserted.
    pub mod LPUART6_IPG_STOP_MODE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// LPUART7 doze mode
    pub mod LPUART7_IPG_DOZE {
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

    /// LPUART7 stop request
    pub mod LPUART7_STOP_REQ {
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

    /// LPUART7 stop mode selection, cannot change when LPUART7_STOP_REQ is asserted.
    pub mod LPUART7_IPG_STOP_MODE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// LPUART8 doze mode
    pub mod LPUART8_IPG_DOZE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPUART8 stop request
    pub mod LPUART8_STOP_REQ {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPUART8 stop mode selection, cannot change when LPUART8_STOP_REQ is asserted.
    pub mod LPUART8_IPG_STOP_MODE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// LPUART9 doze mode
    pub mod LPUART9_IPG_DOZE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPUART9 stop request
    pub mod LPUART9_STOP_REQ {
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

    /// LPUART9 stop mode selection, cannot change when LPUART9_STOP_REQ is asserted.
    pub mod LPUART9_IPG_STOP_MODE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// LPUART10 doze mode
    pub mod LPUART10_IPG_DOZE {
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

    /// LPUART10 stop request
    pub mod LPUART10_STOP_REQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPUART10 stop mode selection, cannot change when LPUART10_STOP_REQ is asserted.
    pub mod LPUART10_IPG_STOP_MODE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// LPUART11 doze mode
    pub mod LPUART11_IPG_DOZE {
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

    /// LPUART11 stop request
    pub mod LPUART11_STOP_REQ {
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

    /// LPUART11 stop mode selection, cannot change when LPUART11_STOP_REQ is asserted.
    pub mod LPUART11_IPG_STOP_MODE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// LPUART12 doze mode
    pub mod LPUART12_IPG_DOZE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPUART12 stop request
    pub mod LPUART12_STOP_REQ {
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

    /// LPUART12 stop mode selection, cannot change when LPUART12_STOP_REQ is asserted.
    pub mod LPUART12_IPG_STOP_MODE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// MIC doze mode
    pub mod MIC_IPG_DOZE {
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

    /// MIC stop request
    pub mod MIC_STOP_REQ {
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

    /// MIC stop mode selection, cannot change when MIC_STOP_REQ is asserted.
    pub mod MIC_IPG_STOP_MODE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR74 General Purpose Register
pub mod GPR74 {

    /// PIT1 stop request
    pub mod PIT1_STOP_REQ {
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

    /// PIT2 stop request
    pub mod PIT2_STOP_REQ {
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

    /// SEMC stop request
    pub mod SEMC_STOP_REQ {
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

    /// SIM1 doze mode
    pub mod SIM1_IPG_DOZE {
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

    /// SIM2 doze mode
    pub mod SIM2_IPG_DOZE {
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

    /// SNVS_HP doze mode
    pub mod SNVS_HP_IPG_DOZE {
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

    /// SNVS_HP stop request
    pub mod SNVS_HP_STOP_REQ {
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

    /// WDOG1 doze mode
    pub mod WDOG1_IPG_DOZE {
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

    /// WDOG2 doze mode
    pub mod WDOG2_IPG_DOZE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI1 stop request
    pub mod SAI1_STOP_REQ {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI2 stop request
    pub mod SAI2_STOP_REQ {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI3 stop request
    pub mod SAI3_STOP_REQ {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI4 stop request
    pub mod SAI4_STOP_REQ {
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

    /// FLEXIO1 bus clock domain stop request
    pub mod FLEXIO1_STOP_REQ_BUS {
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

    /// FLEXIO1 peripheral clock domain stop request
    pub mod FLEXIO1_STOP_REQ_PER {
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

    /// FLEXIO2 bus clock domain stop request
    pub mod FLEXIO2_STOP_REQ_BUS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FLEXIO2 peripheral clock domain stop request
    pub mod FLEXIO2_STOP_REQ_PER {
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

    /// Domain write protection
    pub mod DWP {
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

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR75 General Purpose Register
pub mod GPR75 {

    /// ADC1 stop acknowledge
    pub mod ADC1_STOP_ACK {
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

    /// ADC2 stop acknowledge
    pub mod ADC2_STOP_ACK {
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

    /// CAAM stop acknowledge
    pub mod CAAM_STOP_ACK {
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

    /// CAN1 stop acknowledge
    pub mod CAN1_STOP_ACK {
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

    /// CAN2 stop acknowledge
    pub mod CAN2_STOP_ACK {
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

    /// CAN3 stop acknowledge
    pub mod CAN3_STOP_ACK {
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

    /// EDMA stop acknowledge
    pub mod EDMA_STOP_ACK {
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

    /// EDMA_LPSR stop acknowledge
    pub mod EDMA_LPSR_STOP_ACK {
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

    /// ENET stop acknowledge
    pub mod ENET_STOP_ACK {
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

    /// ENET1G stop acknowledge
    pub mod ENET1G_STOP_ACK {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FLEXSPI1 stop acknowledge
    pub mod FLEXSPI1_STOP_ACK {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FLEXSPI2 stop acknowledge
    pub mod FLEXSPI2_STOP_ACK {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPI2C1 stop acknowledge
    pub mod LPI2C1_STOP_ACK {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPI2C2 stop acknowledge
    pub mod LPI2C2_STOP_ACK {
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

    /// LPI2C3 stop acknowledge
    pub mod LPI2C3_STOP_ACK {
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

    /// LPI2C4 stop acknowledge
    pub mod LPI2C4_STOP_ACK {
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

    /// LPI2C5 stop acknowledge
    pub mod LPI2C5_STOP_ACK {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPI2C6 stop acknowledge
    pub mod LPI2C6_STOP_ACK {
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

    /// LPSPI1 stop acknowledge
    pub mod LPSPI1_STOP_ACK {
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

    /// LPSPI2 stop acknowledge
    pub mod LPSPI2_STOP_ACK {
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

    /// LPSPI3 stop acknowledge
    pub mod LPSPI3_STOP_ACK {
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

    /// LPSPI4 stop acknowledge
    pub mod LPSPI4_STOP_ACK {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPSPI5 stop acknowledge
    pub mod LPSPI5_STOP_ACK {
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

    /// LPSPI6 stop acknowledge
    pub mod LPSPI6_STOP_ACK {
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

    /// LPUART1 stop acknowledge
    pub mod LPUART1_STOP_ACK {
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

    /// LPUART2 stop acknowledge
    pub mod LPUART2_STOP_ACK {
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

    /// LPUART3 stop acknowledge
    pub mod LPUART3_STOP_ACK {
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

    /// LPUART4 stop acknowledge
    pub mod LPUART4_STOP_ACK {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPUART5 stop acknowledge
    pub mod LPUART5_STOP_ACK {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPUART6 stop acknowledge
    pub mod LPUART6_STOP_ACK {
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

    /// LPUART7 stop acknowledge
    pub mod LPUART7_STOP_ACK {
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

    /// LPUART8 stop acknowledge
    pub mod LPUART8_STOP_ACK {
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

/// GPR76 General Purpose Register
pub mod GPR76 {

    /// LPUART9 stop acknowledge
    pub mod LPUART9_STOP_ACK {
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

    /// LPUART10 stop acknowledge
    pub mod LPUART10_STOP_ACK {
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

    /// LPUART11 stop acknowledge
    pub mod LPUART11_STOP_ACK {
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

    /// LPUART12 stop acknowledge
    pub mod LPUART12_STOP_ACK {
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

    /// MIC stop acknowledge
    pub mod MIC_STOP_ACK {
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

    /// PIT1 stop acknowledge
    pub mod PIT1_STOP_ACK {
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

    /// PIT2 stop acknowledge
    pub mod PIT2_STOP_ACK {
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

    /// SEMC stop acknowledge
    pub mod SEMC_STOP_ACK {
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

    /// SNVS_HP stop acknowledge
    pub mod SNVS_HP_STOP_ACK {
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

    /// SAI1 stop acknowledge
    pub mod SAI1_STOP_ACK {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI2 stop acknowledge
    pub mod SAI2_STOP_ACK {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI3 stop acknowledge
    pub mod SAI3_STOP_ACK {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SAI4 stop acknowledge
    pub mod SAI4_STOP_ACK {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FLEXIO1 stop acknowledge of bus clock domain
    pub mod FLEXIO1_STOP_ACK_BUS {
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

    /// FLEXIO1 stop acknowledge of peripheral clock domain
    pub mod FLEXIO1_STOP_ACK_PER {
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

    /// FLEXIO2 stop acknowledge of bus clock domain
    pub mod FLEXIO2_STOP_ACK_BUS {
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

    /// FLEXIO2 stop acknowledge of peripheral clock domain
    pub mod FLEXIO2_STOP_ACK_PER {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// GPR0 General Purpose Register
    pub GPR0: RWRegister<u32>,

    /// GPR1 General Purpose Register
    pub GPR1: RWRegister<u32>,

    /// GPR2 General Purpose Register
    pub GPR2: RWRegister<u32>,

    /// GPR3 General Purpose Register
    pub GPR3: RWRegister<u32>,

    /// GPR4 General Purpose Register
    pub GPR4: RWRegister<u32>,

    /// GPR5 General Purpose Register
    pub GPR5: RWRegister<u32>,

    /// GPR6 General Purpose Register
    pub GPR6: RWRegister<u32>,

    /// GPR7 General Purpose Register
    pub GPR7: RWRegister<u32>,

    /// GPR8 General Purpose Register
    pub GPR8: RWRegister<u32>,

    /// GPR9 General Purpose Register
    pub GPR9: RWRegister<u32>,

    /// GPR10 General Purpose Register
    pub GPR10: RWRegister<u32>,

    /// GPR11 General Purpose Register
    pub GPR11: RWRegister<u32>,

    /// GPR12 General Purpose Register
    pub GPR12: RWRegister<u32>,

    /// GPR13 General Purpose Register
    pub GPR13: RWRegister<u32>,

    /// GPR14 General Purpose Register
    pub GPR14: RWRegister<u32>,

    /// GPR15 General Purpose Register
    pub GPR15: RWRegister<u32>,

    /// GPR16 General Purpose Register
    pub GPR16: RWRegister<u32>,

    /// GPR17 General Purpose Register
    pub GPR17: RWRegister<u32>,

    /// GPR18 General Purpose Register
    pub GPR18: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// GPR20 General Purpose Register
    pub GPR20: RWRegister<u32>,

    /// GPR21 General Purpose Register
    pub GPR21: RWRegister<u32>,

    /// GPR22 General Purpose Register
    pub GPR22: RWRegister<u32>,

    /// GPR23 General Purpose Register
    pub GPR23: RWRegister<u32>,

    /// GPR24 General Purpose Register
    pub GPR24: RWRegister<u32>,

    /// GPR25 General Purpose Register
    pub GPR25: RWRegister<u32>,

    /// GPR26 General Purpose Register
    pub GPR26: RWRegister<u32>,

    /// GPR27 General Purpose Register
    pub GPR27: RWRegister<u32>,

    /// GPR28 General Purpose Register
    pub GPR28: RWRegister<u32>,

    /// GPR29 General Purpose Register
    pub GPR29: RWRegister<u32>,

    /// GPR30 General Purpose Register
    pub GPR30: RWRegister<u32>,

    /// GPR31 General Purpose Register
    pub GPR31: RWRegister<u32>,

    /// GPR32 General Purpose Register
    pub GPR32: RWRegister<u32>,

    /// GPR33 General Purpose Register
    pub GPR33: RWRegister<u32>,

    /// GPR34 General Purpose Register
    pub GPR34: RWRegister<u32>,

    /// GPR35 General Purpose Register
    pub GPR35: RWRegister<u32>,

    /// GPR36 General Purpose Register
    pub GPR36: RWRegister<u32>,

    /// GPR37 General Purpose Register
    pub GPR37: RWRegister<u32>,

    /// GPR38 General Purpose Register
    pub GPR38: RWRegister<u32>,

    /// GPR39 General Purpose Register
    pub GPR39: RWRegister<u32>,

    /// GPR40 General Purpose Register
    pub GPR40: RWRegister<u32>,

    /// GPR41 General Purpose Register
    pub GPR41: RWRegister<u32>,

    /// GPR42 General Purpose Register
    pub GPR42: RWRegister<u32>,

    /// GPR43 General Purpose Register
    pub GPR43: RWRegister<u32>,

    /// GPR44 General Purpose Register
    pub GPR44: RWRegister<u32>,

    /// GPR45 General Purpose Register
    pub GPR45: RWRegister<u32>,

    /// GPR46 General Purpose Register
    pub GPR46: RWRegister<u32>,

    /// GPR47 General Purpose Register
    pub GPR47: RWRegister<u32>,

    /// GPR48 General Purpose Register
    pub GPR48: RWRegister<u32>,

    /// GPR49 General Purpose Register
    pub GPR49: RWRegister<u32>,

    /// GPR50 General Purpose Register
    pub GPR50: RWRegister<u32>,

    /// GPR51 General Purpose Register
    pub GPR51: RWRegister<u32>,

    /// GPR52 General Purpose Register
    pub GPR52: RWRegister<u32>,

    /// GPR53 General Purpose Register
    pub GPR53: RWRegister<u32>,

    /// GPR54 General Purpose Register
    pub GPR54: RWRegister<u32>,

    /// GPR55 General Purpose Register
    pub GPR55: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// GPR59 General Purpose Register
    pub GPR59: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// GPR62 General Purpose Register
    pub GPR62: RWRegister<u32>,

    /// GPR63 General Purpose Register
    pub GPR63: RORegister<u32>,

    /// GPR64 General Purpose Register
    pub GPR64: RWRegister<u32>,

    /// GPR65 General Purpose Register
    pub GPR65: RWRegister<u32>,

    /// GPR66 General Purpose Register
    pub GPR66: RWRegister<u32>,

    /// GPR67 General Purpose Register
    pub GPR67: RWRegister<u32>,

    /// GPR68 General Purpose Register
    pub GPR68: RWRegister<u32>,

    /// GPR69 General Purpose Register
    pub GPR69: RWRegister<u32>,

    /// GPR70 General Purpose Register
    pub GPR70: RWRegister<u32>,

    /// GPR71 General Purpose Register
    pub GPR71: RWRegister<u32>,

    /// GPR72 General Purpose Register
    pub GPR72: RWRegister<u32>,

    /// GPR73 General Purpose Register
    pub GPR73: RWRegister<u32>,

    /// GPR74 General Purpose Register
    pub GPR74: RWRegister<u32>,

    /// GPR75 General Purpose Register
    pub GPR75: RORegister<u32>,

    /// GPR76 General Purpose Register
    pub GPR76: RORegister<u32>,
}
pub struct ResetValues {
    pub GPR0: u32,
    pub GPR1: u32,
    pub GPR2: u32,
    pub GPR3: u32,
    pub GPR4: u32,
    pub GPR5: u32,
    pub GPR6: u32,
    pub GPR7: u32,
    pub GPR8: u32,
    pub GPR9: u32,
    pub GPR10: u32,
    pub GPR11: u32,
    pub GPR12: u32,
    pub GPR13: u32,
    pub GPR14: u32,
    pub GPR15: u32,
    pub GPR16: u32,
    pub GPR17: u32,
    pub GPR18: u32,
    pub GPR20: u32,
    pub GPR21: u32,
    pub GPR22: u32,
    pub GPR23: u32,
    pub GPR24: u32,
    pub GPR25: u32,
    pub GPR26: u32,
    pub GPR27: u32,
    pub GPR28: u32,
    pub GPR29: u32,
    pub GPR30: u32,
    pub GPR31: u32,
    pub GPR32: u32,
    pub GPR33: u32,
    pub GPR34: u32,
    pub GPR35: u32,
    pub GPR36: u32,
    pub GPR37: u32,
    pub GPR38: u32,
    pub GPR39: u32,
    pub GPR40: u32,
    pub GPR41: u32,
    pub GPR42: u32,
    pub GPR43: u32,
    pub GPR44: u32,
    pub GPR45: u32,
    pub GPR46: u32,
    pub GPR47: u32,
    pub GPR48: u32,
    pub GPR49: u32,
    pub GPR50: u32,
    pub GPR51: u32,
    pub GPR52: u32,
    pub GPR53: u32,
    pub GPR54: u32,
    pub GPR55: u32,
    pub GPR59: u32,
    pub GPR62: u32,
    pub GPR63: u32,
    pub GPR64: u32,
    pub GPR65: u32,
    pub GPR66: u32,
    pub GPR67: u32,
    pub GPR68: u32,
    pub GPR69: u32,
    pub GPR70: u32,
    pub GPR71: u32,
    pub GPR72: u32,
    pub GPR73: u32,
    pub GPR74: u32,
    pub GPR75: u32,
    pub GPR76: u32,
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
