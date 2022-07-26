#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MECC64
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Error Interrupt Status Register
pub mod ERR_STATUS {

    /// Single Bit Error On OCRAM Bank0
    pub mod SINGLE_ERR0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Single bit error does not happen on OCRAM bank0.
            pub const SINGLE_ERR0_0: u32 = 0b0;

            /// 0b1: Single bit error happens on OCRAM bank0.
            pub const SINGLE_ERR0_1: u32 = 0b1;
        }
    }

    /// Single Bit Error On OCRAM Bank1
    pub mod SINGLE_ERR1 {
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

            /// 0b0: Single bit error does not happen on OCRAM bank1.
            pub const SINGLE_ERR1_0: u32 = 0b0;

            /// 0b1: Single bit error happens on OCRAM bank1.
            pub const SINGLE_ERR1_1: u32 = 0b1;
        }
    }

    /// Single Bit Error On OCRAM Bank2
    pub mod SINGLE_ERR2 {
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

            /// 0b0: Single bit error does not happen on OCRAM bank2.
            pub const SINGLE_ERR2_0: u32 = 0b0;

            /// 0b1: Single bit error happens on OCRAM bank2.
            pub const SINGLE_ERR2_1: u32 = 0b1;
        }
    }

    /// Single Bit Error On OCRAM Bank3
    pub mod SINGLE_ERR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Single bit error does not happen on OCRAM bank3.
            pub const SINGLE_ERR3_0: u32 = 0b0;

            /// 0b1: Single bit error happens on OCRAM bank3.
            pub const SINGLE_ERR3_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error On OCRAM Bank0
    pub mod MULTI_ERR0 {
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

            /// 0b0: Multiple bits error does not happen on OCRAM bank0.
            pub const MULTI_ERR0_0: u32 = 0b0;

            /// 0b1: Multiple bits error happens on OCRAM bank0.
            pub const MULTI_ERR0_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error On OCRAM Bank1
    pub mod MULTI_ERR1 {
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

            /// 0b0: Multiple bits error does not happen on OCRAM bank1.
            pub const MULTI_ERR1_0: u32 = 0b0;

            /// 0b1: Multiple bits error happens on OCRAM bank1.
            pub const MULTI_ERR1_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error On OCRAM Bank2
    pub mod MULTI_ERR2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Multiple bits error does not happen on OCRAM bank2.
            pub const MULTI_ERR2_0: u32 = 0b0;

            /// 0b1: Multiple bits error happens on OCRAM bank2.
            pub const MULTI_ERR2_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error On OCRAM Bank3
    pub mod MULTI_ERR3 {
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

            /// 0b0: Multiple bits error does not happen on OCRAM bank3.
            pub const MULTI_ERR3_0: u32 = 0b0;

            /// 0b1: Multiple bits error happens on OCRAM bank3.
            pub const MULTI_ERR3_1: u32 = 0b1;
        }
    }

    /// AXI Strobe Error On OCRAM Bank0
    pub mod STRB_ERR0 {
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

            /// 0b0: AXI strobe error does not happen on OCRAM bank0.
            pub const STRB_ERR0_0: u32 = 0b0;

            /// 0b1: AXI strobe error happens on OCRAM bank0.
            pub const STRB_ERR0_1: u32 = 0b1;
        }
    }

    /// AXI Strobe Error On OCRAM Bank1
    pub mod STRB_ERR1 {
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

            /// 0b0: AXI strobe error does not happen on OCRAM bank1.
            pub const STRB_ERR1_0: u32 = 0b0;

            /// 0b1: AXI strobe error happens on OCRAM bank1.
            pub const STRB_ERR1_1: u32 = 0b1;
        }
    }

    /// AXI Strobe Error On OCRAM Bank2
    pub mod STRB_ERR2 {
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

            /// 0b0: AXI strobe error does not happen on OCRAM bank2.
            pub const STRB_ERR2_0: u32 = 0b0;

            /// 0b1: AXI strobe error happens on OCRAM bank2.
            pub const STRB_ERR2_1: u32 = 0b1;
        }
    }

    /// AXI Strobe Error On OCRAM Bank3
    pub mod STRB_ERR3 {
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

            /// 0b0: AXI strobe error does not happen on OCRAM bank3.
            pub const STRB_ERR3_0: u32 = 0b0;

            /// 0b1: AXI strobe error happens on OCRAM bank3.
            pub const STRB_ERR3_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error On Bank0
    pub mod ADDR_ERR0 {
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

            /// 0b0: OCRAM access error does not happen on OCRAM bank0.
            pub const ADDR_ERR0_0: u32 = 0b0;

            /// 0b1: OCRAM access error happens on OCRAM bank0.
            pub const ADDR_ERR0_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error On Bank1
    pub mod ADDR_ERR1 {
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

            /// 0b0: OCRAM access error does not happen on OCRAM bank1.
            pub const ADDR_ERR1_0: u32 = 0b0;

            /// 0b1: OCRAM access error happens on OCRAM bank1.
            pub const ADDR_ERR1_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error On Bank2
    pub mod ADDR_ERR2 {
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

            /// 0b0: OCRAM access error does not happen on OCRAM bank2.
            pub const ADDR_ERR2_0: u32 = 0b0;

            /// 0b1: OCRAM access error happens on OCRAM bank2.
            pub const ADDR_ERR2_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error On Bank3
    pub mod ADDR_ERR3 {
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

            /// 0b0: OCRAM access error does not happen on OCRAM bank3.
            pub const ADDR_ERR3_0: u32 = 0b0;

            /// 0b1: OCRAM access error happens on OCRAM bank3.
            pub const ADDR_ERR3_1: u32 = 0b1;
        }
    }
}

/// Error Interrupt Status Enable Register
pub mod ERR_STAT_EN {

    /// Single Bit Error Status Enable On OCRAM Bank0
    pub mod SINGLE_ERR0_STAT_EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const SINGLE_ERR0_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SINGLE_ERR0_STAT_EN_1: u32 = 0b1;
        }
    }

    /// Single Bit Error Status Enable On OCRAM Bank1
    pub mod SINGLE_ERR1_STAT_EN {
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

            /// 0b0: Disabled
            pub const SINGLE_ERR1_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SINGLE_ERR1_STAT_EN_1: u32 = 0b1;
        }
    }

    /// Single Bit Error Status Enable On OCRAM Bank2
    pub mod SINGLE_ERR2_STAT_EN {
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

            /// 0b0: Disabled
            pub const SINGLE_ERR2_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SINGLE_ERR2_STAT_EN_1: u32 = 0b1;
        }
    }

    /// Single Bit Error Status Enable On OCRAM Bank3
    pub mod SINGLE_ERR3_STAT_EN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const SINGLE_ERR3_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SINGLE_ERR3_STAT_EN_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error Status Enable On OCRAM Bank0
    pub mod MULTI_ERR0_STAT_EN {
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

            /// 0b0: Disabled
            pub const MULTI_ERR0_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const MULTI_ERR0_STAT_EN_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error Status Enable On OCRAM Bank1
    pub mod MULTI_ERR1_STAT_EN {
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

            /// 0b0: Disabled
            pub const MULTI_ERR1_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const MULTI_ERR1_STAT_EN_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error Status Enable On OCRAM Bank2
    pub mod MULTI_ERR2_STAT_EN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const MULTI_ERR2_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const MULTI_ERR2_STAT_EN_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error Status Enable On OCRAM Bank3
    pub mod MULTI_ERR3_STAT_EN {
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

            /// 0b0: Disabled
            pub const MULTI_ERR3_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const MULTI_ERR3_STAT_EN_1: u32 = 0b1;
        }
    }

    /// AXI Strobe Error Status Enable On OCRAM Bank0
    pub mod STRB_ERR0_STAT_EN {
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

            /// 0b0: Disabled
            pub const STRB_ERR0_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const STRB_ERR0_STAT_EN_1: u32 = 0b1;
        }
    }

    /// AXI Strobe Error Status Enable On OCRAM Bank1
    pub mod STRB_ERR1_STAT_EN {
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

            /// 0b0: Disabled
            pub const STRB_ERR1_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const STRB_ERR1_STAT_EN_1: u32 = 0b1;
        }
    }

    /// AXI Strobe Error Status Enable On OCRAM Bank2
    pub mod STRB_ERR2_STAT_EN {
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

            /// 0b0: Disabled
            pub const STRB_ERR2_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const STRB_ERR2_STAT_EN_1: u32 = 0b1;
        }
    }

    /// AXI Strobe Error Status Enable On OCRAM Bank3
    pub mod STRB_ERR3_STAT_EN {
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

            /// 0b0: Disabled
            pub const STRB_ERR3_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const STRB_ERR3_STAT_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Status Enable On Bank0
    pub mod ADDR_ERR0_STAT_EN {
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
            pub const ADDR_ERR0_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ADDR_ERR0_STAT_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Status Enable On Bank1
    pub mod ADDR_ERR1_STAT_EN {
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

            /// 0b0: Disabled
            pub const ADDR_ERR1_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ADDR_ERR1_STAT_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Status Enable On Bank2
    pub mod ADDR_ERR2_STAT_EN {
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

            /// 0b0: Disabled
            pub const ADDR_ERR2_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ADDR_ERR2_STAT_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Status Enable On Bank3
    pub mod ADDR_ERR3_STAT_EN {
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

            /// 0b0: Disabled
            pub const ADDR_ERR3_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ADDR_ERR3_STAT_EN_1: u32 = 0b1;
        }
    }
}

/// Error Interrupt Enable Register
pub mod ERR_SIG_EN {

    /// Single Bit Error Interrupt Enable On OCRAM Bank0
    pub mod SINGLE_ERR0_SIG_EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const SINGLE_ERR0_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SINGLE_ERR0_SIG_EN_1: u32 = 0b1;
        }
    }

    /// Single Bit Error Interrupt Enable On OCRAM Bank1
    pub mod SINGLE_ERR1_SIG_EN {
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

            /// 0b0: Disabled
            pub const SINGLE_ERR1_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SINGLE_ERR1_SIG_EN_1: u32 = 0b1;
        }
    }

    /// Single Bit Error Interrupt Enable On OCRAM Bank2
    pub mod SINGLE_ERR2_SIG_EN {
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

            /// 0b0: Disabled
            pub const SINGLE_ERR2_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SINGLE_ERR2_SIG_EN_1: u32 = 0b1;
        }
    }

    /// Single Bit Error Interrupt Enable On OCRAM Bank3
    pub mod SINGLE_ERR3_SIG_EN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const SINGLE_ERR3_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SINGLE_ERR3_SIG_EN_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error Interrupt Enable On OCRAM Bank0
    pub mod MULTI_ERR0_SIG_EN {
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

            /// 0b0: Disabled
            pub const MULTI_ERR0_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const MULTI_ERR0_SIG_EN_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error Interrupt Enable On OCRAM Bank1
    pub mod MULTI_ERR1_SIG_EN {
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

            /// 0b0: Disabled
            pub const MULTI_ERR1_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const MULTI_ERR1_SIG_EN_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error Interrupt Enable On OCRAM Bank2
    pub mod MULTI_ERR2_SIG_EN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const MULTI_ERR2_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const MULTI_ERR2_SIG_EN_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error Interrupt Enable On OCRAM Bank3
    pub mod MULTI_ERR3_SIG_EN {
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

            /// 0b0: Disabled
            pub const MULTI_ERR3_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const MULTI_ERR3_SIG_EN_1: u32 = 0b1;
        }
    }

    /// AXI Strobe Error Interrupt Enable On OCRAM Bank0
    pub mod STRB_ERR0_SIG_EN {
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

            /// 0b0: Disabled
            pub const STRB_ERR0_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const STRB_ERR0_SIG_EN_1: u32 = 0b1;
        }
    }

    /// AXI Strobe Error Interrupt Enable On OCRAM Bank1
    pub mod STRB_ERR1_SIG_EN {
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

            /// 0b0: Disabled
            pub const STRB_ERR1_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const STRB_ERR1_SIG_EN_1: u32 = 0b1;
        }
    }

    /// AXI Strobe Error Interrupt Enable On OCRAM Bank2
    pub mod STRB_ERR2_SIG_EN {
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

            /// 0b0: Disabled
            pub const STRB_ERR2_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const STRB_ERR2_SIG_EN_1: u32 = 0b1;
        }
    }

    /// AXI Strobe Error Interrupt Enable On OCRAM Bank3
    pub mod STRB_ERR3_SIG_EN {
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

            /// 0b0: Disabled
            pub const STRB_ERR3_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const STRB_ERR3_SIG_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Interrupt Enable On Bank0
    pub mod ADDR_ERR0_SIG_EN {
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
            pub const ADDR_ERR0_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ADDR_ERR0_SIG_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Interrupt Enable On Bank1
    pub mod ADDR_ERR1_SIG_EN {
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

            /// 0b0: Disabled
            pub const ADDR_ERR1_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ADDR_ERR1_SIG_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Interrupt Enable On Bank2
    pub mod ADDR_ERR2_SIG_EN {
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

            /// 0b0: Disabled
            pub const ADDR_ERR2_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ADDR_ERR2_SIG_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Interrupt Enable On Bank3
    pub mod ADDR_ERR3_SIG_EN {
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

            /// 0b0: Disabled
            pub const ADDR_ERR3_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ADDR_ERR3_SIG_EN_1: u32 = 0b1;
        }
    }
}

/// Error Injection On LOW 32 bits Of OCRAM Bank0 Write Data
pub mod ERR_DATA_INJ_LOW0 {

    /// Error Injection On LOW 32 bits Of OCRAM Bank0 Write Data
    pub mod ERR_DATA_INJ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Error Injection On HIGH 32 bits Of OCRAM Bank0 Write Data
pub mod ERR_DATA_INJ_HIGH0 {
    pub use super::ERR_DATA_INJ_LOW0::ERR_DATA_INJ;
}

/// Error Injection On 8 bits ECC code Of OCRAM Bank0 Write Data
pub mod ERR_ECC_INJ0 {

    /// Error Injection On 8 bits ECC code Of OCRAM Bank0 Write Data
    pub mod ERR_ECC_INJ {
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
}

/// Error Injection On LOW 32 bits Of OCRAM Bank1 Write Data
pub mod ERR_DATA_INJ_LOW1 {
    pub use super::ERR_DATA_INJ_LOW0::ERR_DATA_INJ;
}

/// Error Injection On HIGH 32 bits Of OCRAM Bank1 Write Data
pub mod ERR_DATA_INJ_HIGH1 {
    pub use super::ERR_DATA_INJ_LOW0::ERR_DATA_INJ;
}

/// Error Injection On 8 bits ECC code Of OCRAM Bank1 Write Data
pub mod ERR_ECC_INJ1 {
    pub use super::ERR_ECC_INJ0::ERR_ECC_INJ;
}

/// Error Injection On LOW 32 bits Of OCRAM Bank2 Write Data
pub mod ERR_DATA_INJ_LOW2 {
    pub use super::ERR_DATA_INJ_LOW0::ERR_DATA_INJ;
}

/// Error Injection On HIGH 32 bits Of OCRAM Bank2 Write Data
pub mod ERR_DATA_INJ_HIGH2 {
    pub use super::ERR_DATA_INJ_LOW0::ERR_DATA_INJ;
}

/// Error Injection On 8 bits ECC code Of OCRAM Bank2 Write Data
pub mod ERR_ECC_INJ2 {
    pub use super::ERR_ECC_INJ0::ERR_ECC_INJ;
}

/// Error Injection On LOW 32 bits Of OCRAM Bank3 Write Data
pub mod ERR_DATA_INJ_LOW3 {
    pub use super::ERR_DATA_INJ_LOW0::ERR_DATA_INJ;
}

/// Error Injection On HIGH 32 bits Of OCRAM Bank3 Write Data
pub mod ERR_DATA_INJ_HIGH3 {
    pub use super::ERR_DATA_INJ_LOW0::ERR_DATA_INJ;
}

/// Error Injection On 8 bits ECC code Of OCRAM Bank3 Write Data
pub mod ERR_ECC_INJ3 {
    pub use super::ERR_ECC_INJ0::ERR_ECC_INJ;
}

/// Single Error Address And ECC code On OCRAM Bank0
pub mod SINGLE_ERR_ADDR_ECC0 {

    /// Single Error ECC code On OCRAM Bank0
    pub mod SINGLE_ERR_ECC {
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

    /// Single Error Address On OCRAM Bank0
    pub mod SINGLE_ERR_ADDR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (19 bits: 0x7ffff << 8)
        pub const mask: u32 = 0x7ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LOW 32 Bits Single Error Read Data On OCRAM Bank0
pub mod SINGLE_ERR_DATA_LOW0 {

    /// LOW 32 Bits Single Error Read Data On OCRAM Bank0
    pub mod SINGLE_ERR_DATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// HIGH 32 Bits Single Error Read Data On OCRAM Bank0
pub mod SINGLE_ERR_DATA_HIGH0 {
    pub use super::SINGLE_ERR_DATA_LOW0::SINGLE_ERR_DATA;
}

/// LOW Single Error Bit Position On OCRAM Bank0
pub mod SINGLE_ERR_POS_LOW0 {

    /// LOW Single Error Bit Position On OCRAM Bank0
    pub mod SINGLE_ERR_POS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// HIGH Single Error Bit Position On OCRAM Bank0
pub mod SINGLE_ERR_POS_HIGH0 {
    pub use super::SINGLE_ERR_POS_LOW0::SINGLE_ERR_POS;
}

/// Single Error Address And ECC code On OCRAM Bank1
pub mod SINGLE_ERR_ADDR_ECC1 {
    pub use super::SINGLE_ERR_ADDR_ECC0::SINGLE_ERR_ADDR;
    pub use super::SINGLE_ERR_ADDR_ECC0::SINGLE_ERR_ECC;
}

/// LOW 32 Bits Single Error Read Data On OCRAM Bank1
pub mod SINGLE_ERR_DATA_LOW1 {
    pub use super::SINGLE_ERR_DATA_LOW0::SINGLE_ERR_DATA;
}

/// HIGH 32 Bits Single Error Read Data On OCRAM Bank1
pub mod SINGLE_ERR_DATA_HIGH1 {
    pub use super::SINGLE_ERR_DATA_LOW0::SINGLE_ERR_DATA;
}

/// LOW Single Error Bit Position On OCRAM Bank1
pub mod SINGLE_ERR_POS_LOW1 {
    pub use super::SINGLE_ERR_POS_LOW0::SINGLE_ERR_POS;
}

/// HIGH Single Error Bit Position On OCRAM Bank1
pub mod SINGLE_ERR_POS_HIGH1 {
    pub use super::SINGLE_ERR_POS_LOW0::SINGLE_ERR_POS;
}

/// Single Error Address And ECC code On OCRAM Bank2
pub mod SINGLE_ERR_ADDR_ECC2 {
    pub use super::SINGLE_ERR_ADDR_ECC0::SINGLE_ERR_ADDR;
    pub use super::SINGLE_ERR_ADDR_ECC0::SINGLE_ERR_ECC;
}

/// LOW 32 Bits Single Error Read Data On OCRAM Bank2
pub mod SINGLE_ERR_DATA_LOW2 {
    pub use super::SINGLE_ERR_DATA_LOW0::SINGLE_ERR_DATA;
}

/// HIGH 32 Bits Single Error Read Data On OCRAM Bank2
pub mod SINGLE_ERR_DATA_HIGH2 {
    pub use super::SINGLE_ERR_DATA_LOW0::SINGLE_ERR_DATA;
}

/// LOW Single Error Bit Position On OCRAM Bank2
pub mod SINGLE_ERR_POS_LOW2 {
    pub use super::SINGLE_ERR_POS_LOW0::SINGLE_ERR_POS;
}

/// HIGH Single Error Bit Position On OCRAM Bank2
pub mod SINGLE_ERR_POS_HIGH2 {
    pub use super::SINGLE_ERR_POS_LOW0::SINGLE_ERR_POS;
}

/// Single Error Address And ECC code On OCRAM Bank3
pub mod SINGLE_ERR_ADDR_ECC3 {
    pub use super::SINGLE_ERR_ADDR_ECC0::SINGLE_ERR_ADDR;
    pub use super::SINGLE_ERR_ADDR_ECC0::SINGLE_ERR_ECC;
}

/// LOW 32 Bits Single Error Read Data On OCRAM Bank3
pub mod SINGLE_ERR_DATA_LOW3 {
    pub use super::SINGLE_ERR_DATA_LOW0::SINGLE_ERR_DATA;
}

/// HIGH 32 Bits Single Error Read Data On OCRAM Bank3
pub mod SINGLE_ERR_DATA_HIGH3 {
    pub use super::SINGLE_ERR_DATA_LOW0::SINGLE_ERR_DATA;
}

/// LOW Single Error Bit Position On OCRAM Bank3
pub mod SINGLE_ERR_POS_LOW3 {
    pub use super::SINGLE_ERR_POS_LOW0::SINGLE_ERR_POS;
}

/// HIGH Single Error Bit Position On OCRAM Bank3
pub mod SINGLE_ERR_POS_HIGH3 {
    pub use super::SINGLE_ERR_POS_LOW0::SINGLE_ERR_POS;
}

/// Multiple Error Address And ECC code On OCRAM Bank0
pub mod MULTI_ERR_ADDR_ECC0 {

    /// Multiple Error ECC code On OCRAM Bank0
    pub mod MULTI_ERR_ECC {
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

    /// Multiple Error Address On OCRAM Bank0
    pub mod MULTI_ERR_ADDR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (19 bits: 0x7ffff << 8)
        pub const mask: u32 = 0x7ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LOW 32 Bits Multiple Error Read Data On OCRAM Bank0
pub mod MULTI_ERR_DATA_LOW0 {

    /// LOW 32 Bits Multiple Error Read Data On OCRAM Bank0
    pub mod MULTI_ERR_DATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// HIGH 32 Bits Multiple Error Read Data On OCRAM Bank0
pub mod MULTI_ERR_DATA_HIGH0 {
    pub use super::MULTI_ERR_DATA_LOW0::MULTI_ERR_DATA;
}

/// Multiple Error Address And ECC code On OCRAM Bank1
pub mod MULTI_ERR_ADDR_ECC1 {
    pub use super::MULTI_ERR_ADDR_ECC0::MULTI_ERR_ADDR;
    pub use super::MULTI_ERR_ADDR_ECC0::MULTI_ERR_ECC;
}

/// LOW 32 Bits Multiple Error Read Data On OCRAM Bank1
pub mod MULTI_ERR_DATA_LOW1 {
    pub use super::MULTI_ERR_DATA_LOW0::MULTI_ERR_DATA;
}

/// HIGH 32 Bits Multiple Error Read Data On OCRAM Bank1
pub mod MULTI_ERR_DATA_HIGH1 {
    pub use super::MULTI_ERR_DATA_LOW0::MULTI_ERR_DATA;
}

/// Multiple Error Address And ECC code On OCRAM Bank2
pub mod MULTI_ERR_ADDR_ECC2 {
    pub use super::MULTI_ERR_ADDR_ECC0::MULTI_ERR_ADDR;
    pub use super::MULTI_ERR_ADDR_ECC0::MULTI_ERR_ECC;
}

/// LOW 32 Bits Multiple Error Read Data On OCRAM Bank2
pub mod MULTI_ERR_DATA_LOW2 {
    pub use super::MULTI_ERR_DATA_LOW0::MULTI_ERR_DATA;
}

/// HIGH 32 Bits Multiple Error Read Data On OCRAM Bank2
pub mod MULTI_ERR_DATA_HIGH2 {
    pub use super::MULTI_ERR_DATA_LOW0::MULTI_ERR_DATA;
}

/// Multiple Error Address And ECC code On OCRAM Bank3
pub mod MULTI_ERR_ADDR_ECC3 {
    pub use super::MULTI_ERR_ADDR_ECC0::MULTI_ERR_ADDR;
    pub use super::MULTI_ERR_ADDR_ECC0::MULTI_ERR_ECC;
}

/// LOW 32 Bits Multiple Error Read Data On OCRAM Bank3
pub mod MULTI_ERR_DATA_LOW3 {
    pub use super::MULTI_ERR_DATA_LOW0::MULTI_ERR_DATA;
}

/// HIGH 32 Bits Multiple Error Read Data On OCRAM Bank3
pub mod MULTI_ERR_DATA_HIGH3 {
    pub use super::MULTI_ERR_DATA_LOW0::MULTI_ERR_DATA;
}

/// OCRAM Pipeline And ECC Enable
pub mod PIPE_ECC_EN {

    /// Read Data Wait Enable
    pub mod READ_DATA_WAIT_EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable.
            pub const READ_DATA_WAIT_EN_0: u32 = 0b0;

            /// 0b1: Enable.
            pub const READ_DATA_WAIT_EN_1: u32 = 0b1;
        }
    }

    /// Read Address Pipeline Enable
    pub mod READ_ADDR_PIPE_EN {
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

            /// 0b0: Disable.
            pub const READ_ADDR_PIPE_EN_0: u32 = 0b0;

            /// 0b1: Enable.
            pub const READ_ADDR_PIPE_EN_1: u32 = 0b1;
        }
    }

    /// Write Data Pipeline Enable
    pub mod WRITE_DATA_PIPE_EN {
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

            /// 0b0: Disable.
            pub const WRITE_DATA_PIPE_EN_0: u32 = 0b0;

            /// 0b1: Enable.
            pub const WRITE_DATA_PIPE_EN_1: u32 = 0b1;
        }
    }

    /// Write Address Pipeline Enable
    pub mod WRITE_ADDR_PIPE_EN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable.
            pub const WRITE_ADDR_PIPE_EN_0: u32 = 0b0;

            /// 0b1: Enable.
            pub const WRITE_ADDR_PIPE_EN_1: u32 = 0b1;
        }
    }

    /// ECC Function Enable
    pub mod ECC_EN {
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

            /// 0b0: Disable.
            pub const ECC_EN_0: u32 = 0b0;

            /// 0b1: Enable.
            pub const ECC_EN_1: u32 = 0b1;
        }
    }
}

/// Pending Status
pub mod PENDING_STAT {

    /// Read Data Wait Pending
    pub mod READ_DATA_WAIT_PENDING {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No update pending status for READ_DATA_WAIT_EN.
            pub const READ_DATA_WAIT_PENDING_0: u32 = 0b0;

            /// 0b1: When READ_DATA_WAIT_EN register bit is changed, this register bit will be set until the new setup becomes valid in the controller.
            pub const READ_DATA_WAIT_PENDING_1: u32 = 0b1;
        }
    }

    /// Read Address Pipeline Pending
    pub mod READ_ADDR_PIPE_PENDING {
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

            /// 0b0: No update pending status for READ_ADDR_PIPE_EN.
            pub const READ_ADDR_PIPE_PENDING_0: u32 = 0b0;

            /// 0b1: When READ_ADDR_PIPE_EN register bit is changed, this register bit will be set until the new setup becomes valid in the controller.
            pub const READ_ADDR_PIPE_PENDING_1: u32 = 0b1;
        }
    }

    /// Write Data Pipeline Pending
    pub mod WRITE_DATA_PIPE_PENDING {
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

            /// 0b0: No update pending status for WRITE_DATA_PIPE_EN.
            pub const WRITE_DATA_PIPE_PENDING_0: u32 = 0b0;

            /// 0b1: When WRITE_DATA_PIPE_EN register bit is changed, this register bit will be set until the new setup becomes valid in the controller.
            pub const WRITE_DATA_PIPE_PENDING_1: u32 = 0b1;
        }
    }

    /// Write Address Pipeline Pending
    pub mod WRITE_ADDR_PIPE_PENDING {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No update pending status for WRITE_ADDR_PIPE_EN.
            pub const WRITE_ADDR_PIPE_PENDING_0: u32 = 0b0;

            /// 0b1: When WRITE_ADDR_PIPE_EN register bit is changed, this register bit will be set until the new setup becomes valid in the controller.
            pub const WRITE_ADDR_PIPE_PENDING_1: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Error Interrupt Status Register
    pub ERR_STATUS: RWRegister<u32>,

    /// Error Interrupt Status Enable Register
    pub ERR_STAT_EN: RWRegister<u32>,

    /// Error Interrupt Enable Register
    pub ERR_SIG_EN: RWRegister<u32>,

    /// Error Injection On LOW 32 bits Of OCRAM Bank0 Write Data
    pub ERR_DATA_INJ_LOW0: RWRegister<u32>,

    /// Error Injection On HIGH 32 bits Of OCRAM Bank0 Write Data
    pub ERR_DATA_INJ_HIGH0: RWRegister<u32>,

    /// Error Injection On 8 bits ECC code Of OCRAM Bank0 Write Data
    pub ERR_ECC_INJ0: RWRegister<u32>,

    /// Error Injection On LOW 32 bits Of OCRAM Bank1 Write Data
    pub ERR_DATA_INJ_LOW1: RWRegister<u32>,

    /// Error Injection On HIGH 32 bits Of OCRAM Bank1 Write Data
    pub ERR_DATA_INJ_HIGH1: RWRegister<u32>,

    /// Error Injection On 8 bits ECC code Of OCRAM Bank1 Write Data
    pub ERR_ECC_INJ1: RWRegister<u32>,

    /// Error Injection On LOW 32 bits Of OCRAM Bank2 Write Data
    pub ERR_DATA_INJ_LOW2: RWRegister<u32>,

    /// Error Injection On HIGH 32 bits Of OCRAM Bank2 Write Data
    pub ERR_DATA_INJ_HIGH2: RWRegister<u32>,

    /// Error Injection On 8 bits ECC code Of OCRAM Bank2 Write Data
    pub ERR_ECC_INJ2: RWRegister<u32>,

    /// Error Injection On LOW 32 bits Of OCRAM Bank3 Write Data
    pub ERR_DATA_INJ_LOW3: RWRegister<u32>,

    /// Error Injection On HIGH 32 bits Of OCRAM Bank3 Write Data
    pub ERR_DATA_INJ_HIGH3: RWRegister<u32>,

    /// Error Injection On 8 bits ECC code Of OCRAM Bank3 Write Data
    pub ERR_ECC_INJ3: RWRegister<u32>,

    /// Single Error Address And ECC code On OCRAM Bank0
    pub SINGLE_ERR_ADDR_ECC0: RORegister<u32>,

    /// LOW 32 Bits Single Error Read Data On OCRAM Bank0
    pub SINGLE_ERR_DATA_LOW0: RORegister<u32>,

    /// HIGH 32 Bits Single Error Read Data On OCRAM Bank0
    pub SINGLE_ERR_DATA_HIGH0: RORegister<u32>,

    /// LOW Single Error Bit Position On OCRAM Bank0
    pub SINGLE_ERR_POS_LOW0: RORegister<u32>,

    /// HIGH Single Error Bit Position On OCRAM Bank0
    pub SINGLE_ERR_POS_HIGH0: RORegister<u32>,

    /// Single Error Address And ECC code On OCRAM Bank1
    pub SINGLE_ERR_ADDR_ECC1: RORegister<u32>,

    /// LOW 32 Bits Single Error Read Data On OCRAM Bank1
    pub SINGLE_ERR_DATA_LOW1: RORegister<u32>,

    /// HIGH 32 Bits Single Error Read Data On OCRAM Bank1
    pub SINGLE_ERR_DATA_HIGH1: RORegister<u32>,

    /// LOW Single Error Bit Position On OCRAM Bank1
    pub SINGLE_ERR_POS_LOW1: RORegister<u32>,

    /// HIGH Single Error Bit Position On OCRAM Bank1
    pub SINGLE_ERR_POS_HIGH1: RORegister<u32>,

    /// Single Error Address And ECC code On OCRAM Bank2
    pub SINGLE_ERR_ADDR_ECC2: RORegister<u32>,

    /// LOW 32 Bits Single Error Read Data On OCRAM Bank2
    pub SINGLE_ERR_DATA_LOW2: RORegister<u32>,

    /// HIGH 32 Bits Single Error Read Data On OCRAM Bank2
    pub SINGLE_ERR_DATA_HIGH2: RORegister<u32>,

    /// LOW Single Error Bit Position On OCRAM Bank2
    pub SINGLE_ERR_POS_LOW2: RORegister<u32>,

    /// HIGH Single Error Bit Position On OCRAM Bank2
    pub SINGLE_ERR_POS_HIGH2: RORegister<u32>,

    /// Single Error Address And ECC code On OCRAM Bank3
    pub SINGLE_ERR_ADDR_ECC3: RORegister<u32>,

    /// LOW 32 Bits Single Error Read Data On OCRAM Bank3
    pub SINGLE_ERR_DATA_LOW3: RORegister<u32>,

    /// HIGH 32 Bits Single Error Read Data On OCRAM Bank3
    pub SINGLE_ERR_DATA_HIGH3: RORegister<u32>,

    /// LOW Single Error Bit Position On OCRAM Bank3
    pub SINGLE_ERR_POS_LOW3: RORegister<u32>,

    /// HIGH Single Error Bit Position On OCRAM Bank3
    pub SINGLE_ERR_POS_HIGH3: RORegister<u32>,

    /// Multiple Error Address And ECC code On OCRAM Bank0
    pub MULTI_ERR_ADDR_ECC0: RORegister<u32>,

    /// LOW 32 Bits Multiple Error Read Data On OCRAM Bank0
    pub MULTI_ERR_DATA_LOW0: RORegister<u32>,

    /// HIGH 32 Bits Multiple Error Read Data On OCRAM Bank0
    pub MULTI_ERR_DATA_HIGH0: RORegister<u32>,

    /// Multiple Error Address And ECC code On OCRAM Bank1
    pub MULTI_ERR_ADDR_ECC1: RORegister<u32>,

    /// LOW 32 Bits Multiple Error Read Data On OCRAM Bank1
    pub MULTI_ERR_DATA_LOW1: RORegister<u32>,

    /// HIGH 32 Bits Multiple Error Read Data On OCRAM Bank1
    pub MULTI_ERR_DATA_HIGH1: RORegister<u32>,

    /// Multiple Error Address And ECC code On OCRAM Bank2
    pub MULTI_ERR_ADDR_ECC2: RORegister<u32>,

    /// LOW 32 Bits Multiple Error Read Data On OCRAM Bank2
    pub MULTI_ERR_DATA_LOW2: RORegister<u32>,

    /// HIGH 32 Bits Multiple Error Read Data On OCRAM Bank2
    pub MULTI_ERR_DATA_HIGH2: RORegister<u32>,

    /// Multiple Error Address And ECC code On OCRAM Bank3
    pub MULTI_ERR_ADDR_ECC3: RORegister<u32>,

    /// LOW 32 Bits Multiple Error Read Data On OCRAM Bank3
    pub MULTI_ERR_DATA_LOW3: RORegister<u32>,

    /// HIGH 32 Bits Multiple Error Read Data On OCRAM Bank3
    pub MULTI_ERR_DATA_HIGH3: RORegister<u32>,

    _reserved1: [u32; 17],

    /// OCRAM Pipeline And ECC Enable
    pub PIPE_ECC_EN: RWRegister<u32>,

    /// Pending Status
    pub PENDING_STAT: RORegister<u32>,
}
pub struct ResetValues {
    pub ERR_STATUS: u32,
    pub ERR_STAT_EN: u32,
    pub ERR_SIG_EN: u32,
    pub ERR_DATA_INJ_LOW0: u32,
    pub ERR_DATA_INJ_HIGH0: u32,
    pub ERR_ECC_INJ0: u32,
    pub ERR_DATA_INJ_LOW1: u32,
    pub ERR_DATA_INJ_HIGH1: u32,
    pub ERR_ECC_INJ1: u32,
    pub ERR_DATA_INJ_LOW2: u32,
    pub ERR_DATA_INJ_HIGH2: u32,
    pub ERR_ECC_INJ2: u32,
    pub ERR_DATA_INJ_LOW3: u32,
    pub ERR_DATA_INJ_HIGH3: u32,
    pub ERR_ECC_INJ3: u32,
    pub SINGLE_ERR_ADDR_ECC0: u32,
    pub SINGLE_ERR_DATA_LOW0: u32,
    pub SINGLE_ERR_DATA_HIGH0: u32,
    pub SINGLE_ERR_POS_LOW0: u32,
    pub SINGLE_ERR_POS_HIGH0: u32,
    pub SINGLE_ERR_ADDR_ECC1: u32,
    pub SINGLE_ERR_DATA_LOW1: u32,
    pub SINGLE_ERR_DATA_HIGH1: u32,
    pub SINGLE_ERR_POS_LOW1: u32,
    pub SINGLE_ERR_POS_HIGH1: u32,
    pub SINGLE_ERR_ADDR_ECC2: u32,
    pub SINGLE_ERR_DATA_LOW2: u32,
    pub SINGLE_ERR_DATA_HIGH2: u32,
    pub SINGLE_ERR_POS_LOW2: u32,
    pub SINGLE_ERR_POS_HIGH2: u32,
    pub SINGLE_ERR_ADDR_ECC3: u32,
    pub SINGLE_ERR_DATA_LOW3: u32,
    pub SINGLE_ERR_DATA_HIGH3: u32,
    pub SINGLE_ERR_POS_LOW3: u32,
    pub SINGLE_ERR_POS_HIGH3: u32,
    pub MULTI_ERR_ADDR_ECC0: u32,
    pub MULTI_ERR_DATA_LOW0: u32,
    pub MULTI_ERR_DATA_HIGH0: u32,
    pub MULTI_ERR_ADDR_ECC1: u32,
    pub MULTI_ERR_DATA_LOW1: u32,
    pub MULTI_ERR_DATA_HIGH1: u32,
    pub MULTI_ERR_ADDR_ECC2: u32,
    pub MULTI_ERR_DATA_LOW2: u32,
    pub MULTI_ERR_DATA_HIGH2: u32,
    pub MULTI_ERR_ADDR_ECC3: u32,
    pub MULTI_ERR_DATA_LOW3: u32,
    pub MULTI_ERR_DATA_HIGH3: u32,
    pub PIPE_ECC_EN: u32,
    pub PENDING_STAT: u32,
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
