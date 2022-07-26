#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ADC_ETC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// ADC_ETC Global Control Register
pub mod CTRL {

    /// TRIG enable register.
    pub mod TRIG_ENABLE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: disable all 8 external XBAR triggers.
            pub const TRIG_ENABLE_0: u32 = 0b00000000;

            /// 0b00000001: enable external XBAR trigger0.
            pub const TRIG_ENABLE_1: u32 = 0b00000001;

            /// 0b00000010: enable external XBAR trigger1.
            pub const TRIG_ENABLE_2: u32 = 0b00000010;

            /// 0b00000011: enable external XBAR trigger0 and trigger1.
            pub const TRIG_ENABLE_3: u32 = 0b00000011;

            /// 0b11111111: enable all 8 external XBAR triggers.
            pub const TRIG_ENABLE_255: u32 = 0b11111111;
        }
    }

    /// Pre-divider for trig delay and interval
    pub mod PRE_DIVIDER {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Select the trigger type of the DMA_REQ.
    pub mod DMA_MODE_SEL {
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

            /// 0b0: Trig DMA_REQ with latched signal, REQ will be cleared when ACK and source request cleared.
            pub const DMA_MODE_SEL_0: u32 = 0b0;

            /// 0b1: Trig DMA_REQ with pulsed signal, REQ will be cleared by ACK only.
            pub const DMA_MODE_SEL_1: u32 = 0b1;
        }
    }

    /// Software synchronous reset, active high.
    pub mod SOFTRST {
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

            /// 0b0: ADC_ETC works normally.
            pub const SOFTRST_0: u32 = 0b0;

            /// 0b1: All registers inside ADC_ETC will be reset to the default value.
            pub const SOFTRST_1: u32 = 0b1;
        }
    }
}

/// ETC DONE0 and DONE1 IRQ State Register
pub mod DONE0_1_IRQ {

    /// TRIG0 done0 interrupt detection.
    pub mod TRIG0_DONE0 {
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

            /// 0b0: No TRIG0_DONE0 interrupt detected
            pub const TRIG0_DONE0_0: u32 = 0b0;

            /// 0b1: TRIG0_DONE0 interrupt detected
            pub const TRIG0_DONE0_1: u32 = 0b1;
        }
    }

    /// TRIG1 done0 interrupt detection.
    pub mod TRIG1_DONE0 {
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

            /// 0b0: No TRIG1_DONE0 interrupt detected
            pub const TRIG1_DONE0_0: u32 = 0b0;

            /// 0b1: TRIG1_DONE0 interrupt detected
            pub const TRIG1_DONE0_1: u32 = 0b1;
        }
    }

    /// TRIG2 done0 interrupt detection.
    pub mod TRIG2_DONE0 {
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

            /// 0b0: No TRIG2_DONE0 interrupt detected
            pub const TRIG2_DONE0_0: u32 = 0b0;

            /// 0b1: TRIG2_DONE0 interrupt detected
            pub const TRIG2_DONE0_1: u32 = 0b1;
        }
    }

    /// TRIG3 done0 interrupt detection.
    pub mod TRIG3_DONE0 {
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

            /// 0b0: No TRIG3_DONE0 interrupt detected
            pub const TRIG3_DONE0_0: u32 = 0b0;

            /// 0b1: TRIG3_DONE0 interrupt detected
            pub const TRIG3_DONE0_1: u32 = 0b1;
        }
    }

    /// TRIG4 done0 interrupt detection.
    pub mod TRIG4_DONE0 {
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

            /// 0b0: No TRIG4_DONE0 interrupt detected
            pub const TRIG4_DONE0_0: u32 = 0b0;

            /// 0b1: TRIG4_DONE0 interrupt detected
            pub const TRIG4_DONE0_1: u32 = 0b1;
        }
    }

    /// TRIG5 done0 interrupt detection.
    pub mod TRIG5_DONE0 {
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

            /// 0b0: No TRIG5_DONE0 interrupt detected
            pub const TRIG5_DONE0_0: u32 = 0b0;

            /// 0b1: TRIG5_DONE0 interrupt detected
            pub const TRIG5_DONE0_1: u32 = 0b1;
        }
    }

    /// TRIG6 done0 interrupt detection.
    pub mod TRIG6_DONE0 {
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

            /// 0b0: No TRIG6_DONE0 interrupt detected
            pub const TRIG6_DONE0_0: u32 = 0b0;

            /// 0b1: TRIG6_DONE0 interrupt detected
            pub const TRIG6_DONE0_1: u32 = 0b1;
        }
    }

    /// TRIG7 done0 interrupt detection.
    pub mod TRIG7_DONE0 {
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

            /// 0b0: No TRIG7_DONE0 interrupt detected
            pub const TRIG7_DONE0_0: u32 = 0b0;

            /// 0b1: TRIG7_DONE0 interrupt detected
            pub const TRIG7_DONE0_1: u32 = 0b1;
        }
    }

    /// TRIG0 done1 interrupt detection.
    pub mod TRIG0_DONE1 {
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

            /// 0b0: No TRIG0_DONE1 interrupt detected
            pub const TRIG0_DONE1_0: u32 = 0b0;

            /// 0b1: TRIG0_DONE1 interrupt detected
            pub const TRIG0_DONE1_1: u32 = 0b1;
        }
    }

    /// TRIG1 done1 interrupt detection.
    pub mod TRIG1_DONE1 {
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

            /// 0b0: No TRIG1_DONE1 interrupt detected
            pub const TRIG1_DONE1_0: u32 = 0b0;

            /// 0b1: TRIG1_DONE1 interrupt detected
            pub const TRIG1_DONE1_1: u32 = 0b1;
        }
    }

    /// TRIG2 done1 interrupt detection.
    pub mod TRIG2_DONE1 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No TRIG2_DONE1 interrupt detected
            pub const TRIG2_DONE1_0: u32 = 0b0;

            /// 0b1: TRIG2_DONE1 interrupt detected
            pub const TRIG2_DONE1_1: u32 = 0b1;
        }
    }

    /// TRIG3 done1 interrupt detection.
    pub mod TRIG3_DONE1 {
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

            /// 0b0: No TRIG3_DONE1 interrupt detected
            pub const TRIG3_DONE1_0: u32 = 0b0;

            /// 0b1: TRIG3_DONE1 interrupt detected
            pub const TRIG3_DONE1_1: u32 = 0b1;
        }
    }

    /// TRIG4 done1 interrupt detection.
    pub mod TRIG4_DONE1 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No TRIG4_DONE1 interrupt detected
            pub const TRIG4_DONE1_0: u32 = 0b0;

            /// 0b1: TRIG4_DONE1 interrupt detected
            pub const TRIG4_DONE1_1: u32 = 0b1;
        }
    }

    /// TRIG5 done1 interrupt detection.
    pub mod TRIG5_DONE1 {
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

            /// 0b0: No TRIG5_DONE1 interrupt detected
            pub const TRIG5_DONE1_0: u32 = 0b0;

            /// 0b1: TRIG5_DONE1 interrupt detected
            pub const TRIG5_DONE1_1: u32 = 0b1;
        }
    }

    /// TRIG6 done1 interrupt detection.
    pub mod TRIG6_DONE1 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No TRIG6_DONE1 interrupt detected
            pub const TRIG6_DONE1_0: u32 = 0b0;

            /// 0b1: TRIG6_DONE1 interrupt detected
            pub const TRIG6_DONE1_1: u32 = 0b1;
        }
    }

    /// TRIG7 done1 interrupt detection.
    pub mod TRIG7_DONE1 {
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

            /// 0b0: No TRIG7_DONE1 interrupt detected
            pub const TRIG7_DONE1_0: u32 = 0b0;

            /// 0b1: TRIG7_DONE1 interrupt detected
            pub const TRIG7_DONE1_1: u32 = 0b1;
        }
    }
}

/// ETC DONE_2, DONE_3 and DONE_ERR IRQ State Register
pub mod DONE2_3_ERR_IRQ {

    /// TRIG0 done2 interrupt detection.
    pub mod TRIG0_DONE2 {
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

            /// 0b0: No TRIG0_DONE2 interrupt detected
            pub const TRIG0_DONE2_0: u32 = 0b0;

            /// 0b1: TRIG0_DONE2 interrupt detected
            pub const TRIG0_DONE2_1: u32 = 0b1;
        }
    }

    /// TRIG1 done2 interrupt detection.
    pub mod TRIG1_DONE2 {
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

            /// 0b0: No TRIG1_DONE2 interrupt detected
            pub const TRIG1_DONE2_0: u32 = 0b0;

            /// 0b1: TRIG1_DONE2 interrupt detected
            pub const TRIG1_DONE2_1: u32 = 0b1;
        }
    }

    /// TRIG2 done2 interrupt detection.
    pub mod TRIG2_DONE2 {
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

            /// 0b0: No TRIG2_DONE2 interrupt detected
            pub const TRIG2_DONE2_0: u32 = 0b0;

            /// 0b1: TRIG2_DONE2 interrupt detected
            pub const TRIG2_DONE2_1: u32 = 0b1;
        }
    }

    /// TRIG3 done2 interrupt detection.
    pub mod TRIG3_DONE2 {
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

            /// 0b0: No TRIG3_DONE2 interrupt detected
            pub const TRIG3_DONE2_0: u32 = 0b0;

            /// 0b1: TRIG3_DONE2 interrupt detected
            pub const TRIG3_DONE2_1: u32 = 0b1;
        }
    }

    /// TRIG4 done2 interrupt detection.
    pub mod TRIG4_DONE2 {
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

            /// 0b0: No TRIG4_DONE2 interrupt detected
            pub const TRIG4_DONE2_0: u32 = 0b0;

            /// 0b1: TRIG4_DONE2 interrupt detected
            pub const TRIG4_DONE2_1: u32 = 0b1;
        }
    }

    /// TRIG5 done2 interrupt detection.
    pub mod TRIG5_DONE2 {
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

            /// 0b0: No TRIG5_DONE2 interrupt detected
            pub const TRIG5_DONE2_0: u32 = 0b0;

            /// 0b1: TRIG5_DONE2 interrupt detected
            pub const TRIG5_DONE2_1: u32 = 0b1;
        }
    }

    /// TRIG6 done2 interrupt detection.
    pub mod TRIG6_DONE2 {
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

            /// 0b0: No TRIG6_DONE2 interrupt detected
            pub const TRIG6_DONE2_0: u32 = 0b0;

            /// 0b1: TRIG6_DONE2 interrupt detected
            pub const TRIG6_DONE2_1: u32 = 0b1;
        }
    }

    /// TRIG7 done2 interrupt detection.
    pub mod TRIG7_DONE2 {
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

            /// 0b0: No TRIG7_DONE2 interrupt detected
            pub const TRIG7_DONE2_0: u32 = 0b0;

            /// 0b1: TRIG7_DONE2 interrupt detected
            pub const TRIG7_DONE2_1: u32 = 0b1;
        }
    }

    /// TRIG0 done3 interrupt detection.
    pub mod TRIG0_DONE3 {
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

            /// 0b0: No TRIG0_DONE3 interrupt detected
            pub const TRIG0_DONE3_0: u32 = 0b0;

            /// 0b1: TRIG0_DONE3 interrupt detected
            pub const TRIG0_DONE3_1: u32 = 0b1;
        }
    }

    /// TRIG1 done3 interrupt detection.
    pub mod TRIG1_DONE3 {
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

            /// 0b0: No TRIG1_DONE3 interrupt detected
            pub const TRIG1_DONE3_0: u32 = 0b0;

            /// 0b1: TRIG1_DONE3 interrupt detected
            pub const TRIG1_DONE3_1: u32 = 0b1;
        }
    }

    /// TRIG2 done3 interrupt detection.
    pub mod TRIG2_DONE3 {
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

            /// 0b0: No TRIG2_DONE3 interrupt detected
            pub const TRIG2_DONE3_0: u32 = 0b0;

            /// 0b1: TRIG2_DONE3 interrupt detected
            pub const TRIG2_DONE3_1: u32 = 0b1;
        }
    }

    /// TRIG3 done3 interrupt detection.
    pub mod TRIG3_DONE3 {
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

            /// 0b0: No TRIG3_DONE3 interrupt detected
            pub const TRIG3_DONE3_0: u32 = 0b0;

            /// 0b1: TRIG3_DONE3 interrupt detected
            pub const TRIG3_DONE3_1: u32 = 0b1;
        }
    }

    /// TRIG4 done3 interrupt detection.
    pub mod TRIG4_DONE3 {
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

            /// 0b0: No TRIG4_DONE3 interrupt detected
            pub const TRIG4_DONE3_0: u32 = 0b0;

            /// 0b1: TRIG4_DONE3 interrupt detected
            pub const TRIG4_DONE3_1: u32 = 0b1;
        }
    }

    /// TRIG5 done3 interrupt detection.
    pub mod TRIG5_DONE3 {
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

            /// 0b0: No TRIG5_DONE3 interrupt detected
            pub const TRIG5_DONE3_0: u32 = 0b0;

            /// 0b1: TRIG5_DONE3 interrupt detected
            pub const TRIG5_DONE3_1: u32 = 0b1;
        }
    }

    /// TRIG6 done3 interrupt detection.
    pub mod TRIG6_DONE3 {
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

            /// 0b0: No TRIG6_DONE3 interrupt detected
            pub const TRIG6_DONE3_0: u32 = 0b0;

            /// 0b1: TRIG6_DONE3 interrupt detected
            pub const TRIG6_DONE3_1: u32 = 0b1;
        }
    }

    /// TRIG7 done3 interrupt detection.
    pub mod TRIG7_DONE3 {
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

            /// 0b0: No TRIG7_DONE3 interrupt detected
            pub const TRIG7_DONE3_0: u32 = 0b0;

            /// 0b1: TRIG7_DONE3 interrupt detected
            pub const TRIG7_DONE3_1: u32 = 0b1;
        }
    }

    /// TRIG0 error interrupt detection.
    pub mod TRIG0_ERR {
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

            /// 0b0: No TRIG0_ERR interrupt detected
            pub const TRIG0_ERR_0: u32 = 0b0;

            /// 0b1: TRIG0_ERR interrupt detected
            pub const TRIG0_ERR_1: u32 = 0b1;
        }
    }

    /// TRIG1 error interrupt detection.
    pub mod TRIG1_ERR {
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

            /// 0b0: No TRIG1_ERR interrupt detected
            pub const TRIG1_ERR_0: u32 = 0b0;

            /// 0b1: TRIG1_ERR interrupt detected
            pub const TRIG1_ERR_1: u32 = 0b1;
        }
    }

    /// TRIG2 error interrupt detection.
    pub mod TRIG2_ERR {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No TRIG2_ERR interrupt detected
            pub const TRIG2_ERR_0: u32 = 0b0;

            /// 0b1: TRIG2_ERR interrupt detected
            pub const TRIG2_ERR_1: u32 = 0b1;
        }
    }

    /// TRIG3 error interrupt detection.
    pub mod TRIG3_ERR {
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

            /// 0b0: No TRIG3_ERR interrupt detected
            pub const TRIG3_ERR_0: u32 = 0b0;

            /// 0b1: TRIG3_ERR interrupt detected
            pub const TRIG3_ERR_1: u32 = 0b1;
        }
    }

    /// TRIG4 error interrupt detection.
    pub mod TRIG4_ERR {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No TRIG4_ERR interrupt detected
            pub const TRIG4_ERR_0: u32 = 0b0;

            /// 0b1: TRIG4_ERR interrupt detected
            pub const TRIG4_ERR_1: u32 = 0b1;
        }
    }

    /// TRIG5 error interrupt detection.
    pub mod TRIG5_ERR {
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

            /// 0b0: No TRIG5_ERR interrupt detected
            pub const TRIG5_ERR_0: u32 = 0b0;

            /// 0b1: TRIG5_ERR interrupt detected
            pub const TRIG5_ERR_1: u32 = 0b1;
        }
    }

    /// TRIG6 error interrupt detection.
    pub mod TRIG6_ERR {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No TRIG6_ERR interrupt detected
            pub const TRIG6_ERR_0: u32 = 0b0;

            /// 0b1: TRIG6_ERR interrupt detected
            pub const TRIG6_ERR_1: u32 = 0b1;
        }
    }

    /// TRIG7 error interrupt detection.
    pub mod TRIG7_ERR {
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

            /// 0b0: No TRIG7_ERR interrupt detected
            pub const TRIG7_ERR_0: u32 = 0b0;

            /// 0b1: TRIG7_ERR interrupt detected
            pub const TRIG7_ERR_1: u32 = 0b1;
        }
    }
}

/// ETC DMA control Register
pub mod DMA_CTRL {

    /// Enable DMA request when TRIG0 done.
    pub mod TRIG0_ENABLE {
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

            /// 0b0: TRIG0 DMA request disabled.
            pub const TRIG0_ENABLE_0: u32 = 0b0;

            /// 0b1: TRIG0 DMA request enabled.
            pub const TRIG0_ENABLE_1: u32 = 0b1;
        }
    }

    /// Enable DMA request when TRIG1 done.
    pub mod TRIG1_ENABLE {
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

            /// 0b0: TRIG1 DMA request disabled.
            pub const TRIG1_ENABLE_0: u32 = 0b0;

            /// 0b1: TRIG1 DMA request enabled.
            pub const TRIG1_ENABLE_1: u32 = 0b1;
        }
    }

    /// Enable DMA request when TRIG2 done.
    pub mod TRIG2_ENABLE {
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

            /// 0b0: TRIG2 DMA request disabled.
            pub const TRIG2_ENABLE_0: u32 = 0b0;

            /// 0b1: TRIG2 DMA request enabled.
            pub const TRIG2_ENABLE_1: u32 = 0b1;
        }
    }

    /// Enable DMA request when TRIG3 done.
    pub mod TRIG3_ENABLE {
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

            /// 0b0: TRIG3 DMA request disabled.
            pub const TRIG3_ENABLE_0: u32 = 0b0;

            /// 0b1: TRIG3 DMA request enabled.
            pub const TRIG3_ENABLE_1: u32 = 0b1;
        }
    }

    /// Enable DMA request when TRIG4 done.
    pub mod TRIG4_ENABLE {
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

            /// 0b0: TRIG4 DMA request disabled.
            pub const TRIG4_ENABLE_0: u32 = 0b0;

            /// 0b1: TRIG4 DMA request enabled.
            pub const TRIG4_ENABLE_1: u32 = 0b1;
        }
    }

    /// Enable DMA request when TRIG5 done.
    pub mod TRIG5_ENABLE {
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

            /// 0b0: TRIG5 DMA request disabled.
            pub const TRIG5_ENABLE_0: u32 = 0b0;

            /// 0b1: TRIG5 DMA request enabled.
            pub const TRIG5_ENABLE_1: u32 = 0b1;
        }
    }

    /// Enable DMA request when TRIG6 done.
    pub mod TRIG6_ENABLE {
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

            /// 0b0: TRIG6 DMA request disabled.
            pub const TRIG6_ENABLE_0: u32 = 0b0;

            /// 0b1: TRIG6 DMA request enabled.
            pub const TRIG6_ENABLE_1: u32 = 0b1;
        }
    }

    /// Enable DMA request when TRIG7 done.
    pub mod TRIG7_ENABLE {
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

            /// 0b0: TRIG7 DMA request disabled.
            pub const TRIG7_ENABLE_0: u32 = 0b0;

            /// 0b1: TRIG7 DMA request enabled.
            pub const TRIG7_ENABLE_1: u32 = 0b1;
        }
    }

    /// Flag bit for DMA request
    pub mod TRIG0_REQ {
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

            /// 0b0: TRIG0_REQ not detected.
            pub const TRIG0_REQ_0: u32 = 0b0;

            /// 0b1: TRIG0_REQ detected.
            pub const TRIG0_REQ_1: u32 = 0b1;
        }
    }

    /// Flag bit for DMA request
    pub mod TRIG1_REQ {
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

            /// 0b0: TRIG1_REQ not detected.
            pub const TRIG1_REQ_0: u32 = 0b0;

            /// 0b1: TRIG1_REQ detected.
            pub const TRIG1_REQ_1: u32 = 0b1;
        }
    }

    /// Flag bit for DMA request
    pub mod TRIG2_REQ {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TRIG2_REQ not detected.
            pub const TRIG2_REQ_0: u32 = 0b0;

            /// 0b1: TRIG2_REQ detected.
            pub const TRIG2_REQ_1: u32 = 0b1;
        }
    }

    /// Flag bit for DMA request
    pub mod TRIG3_REQ {
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

            /// 0b0: TRIG3_REQ not detected.
            pub const TRIG3_REQ_0: u32 = 0b0;

            /// 0b1: TRIG3_REQ detected.
            pub const TRIG3_REQ_1: u32 = 0b1;
        }
    }

    /// Flag bit for DMA request
    pub mod TRIG4_REQ {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TRIG4_REQ not detected.
            pub const TRIG4_REQ_0: u32 = 0b0;

            /// 0b1: TRIG4_REQ detected.
            pub const TRIG4_REQ_1: u32 = 0b1;
        }
    }

    /// Flag bit for DMA request
    pub mod TRIG5_REQ {
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

            /// 0b0: TRIG5_REQ not detected.
            pub const TRIG5_REQ_0: u32 = 0b0;

            /// 0b1: TRIG5_REQ detected.
            pub const TRIG5_REQ_1: u32 = 0b1;
        }
    }

    /// Flag bit for DMA request
    pub mod TRIG6_REQ {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TRIG6_REQ not detected.
            pub const TRIG6_REQ_0: u32 = 0b0;

            /// 0b1: TRIG6_REQ detected.
            pub const TRIG6_REQ_1: u32 = 0b1;
        }
    }

    /// Flag bit for DMA request
    pub mod TRIG7_REQ {
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

            /// 0b0: TRIG7_REQ not detected.
            pub const TRIG7_REQ_0: u32 = 0b0;

            /// 0b1: TRIG7_REQ detected.
            pub const TRIG7_REQ_1: u32 = 0b1;
        }
    }
}

/// ETC_TRIG Control Register
pub mod TRIG0_CTRL {

    /// Software trigger. This field is self-clearing.
    pub mod SW_TRIG {
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

            /// 0b0: No software trigger event generated.
            pub const SW_TRIG_0: u32 = 0b0;

            /// 0b1: Software trigger event generated.
            pub const SW_TRIG_1: u32 = 0b1;
        }
    }

    /// Trigger mode selection.
    pub mod TRIG_MODE {
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

            /// 0b0: Hardware trigger. The softerware trigger will be ignored.
            pub const TRIG_MODE_0: u32 = 0b0;

            /// 0b1: Software trigger. The hardware trigger will be ignored.
            pub const TRIG_MODE_1: u32 = 0b1;
        }
    }

    /// The number of segments inside the trigger chain of TRIGa.
    pub mod TRIG_CHAIN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Trigger chain length is 1
            pub const TRIG_CHAIN_0: u32 = 0b000;

            /// 0b001: Trigger chain length is 2
            pub const TRIG_CHAIN_1: u32 = 0b001;

            /// 0b010: Trigger chain length is 3
            pub const TRIG_CHAIN_2: u32 = 0b010;

            /// 0b011: Trigger chain length is 4
            pub const TRIG_CHAIN_3: u32 = 0b011;

            /// 0b100: Trigger chain length is 5
            pub const TRIG_CHAIN_4: u32 = 0b100;

            /// 0b101: Trigger chain length is 6
            pub const TRIG_CHAIN_5: u32 = 0b101;

            /// 0b110: Trigger chain length is 7
            pub const TRIG_CHAIN_6: u32 = 0b110;

            /// 0b111: Trigger chain length is 8
            pub const TRIG_CHAIN_7: u32 = 0b111;
        }
    }

    /// External trigger priority, 7 is highest priority, while 0 is lowest
    pub mod TRIG_PRIORITY {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger synchronization mode selection
    pub mod SYNC_MODE {
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

            /// 0b0: Synchronization mode disabled, TRIGa and TRIG(a+4) are triggered independently.
            pub const SYNC_MODE_0: u32 = 0b0;

            /// 0b1: Synchronization mode enabled, TRIGa and TRIG(a+4) are triggered by TRIGa source synchronously.
            pub const SYNC_MODE_1: u32 = 0b1;
        }
    }

    /// Segment x done detection
    pub mod CHAINx_DONE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: segment x done not detected.
            pub const CHAINx_DONE_0: u32 = 0b00000000;

            /// 0b00000001: segment x done detected.
            pub const CHAINx_DONE_1: u32 = 0b00000001;
        }
    }
}

/// ETC_TRIG Counter Register
pub mod TRIG0_COUNTER {

    /// TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk
    pub mod INIT_DELAY {
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

    /// TRIGGER sampling interval counter
    pub mod SAMPLE_INTERVAL {
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

/// ETC_TRIG Chain 0/1 Register
pub mod TRIG0_CHAIN_1_0 {

    /// ADC hardware trigger command selection
    pub mod CSEL0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not a valid selection from the command buffer. Trigger event is ignored.
            pub const CSEL0_0: u32 = 0b0000;

            /// 0b0001: ADC CMD1 selected.
            pub const CSEL0_1: u32 = 0b0001;

            /// 0b0010: ADC CMD2 selected.
            pub const CSEL0_2: u32 = 0b0010;

            /// 0b0011: ADC CMD3 selected.
            pub const CSEL0_3: u32 = 0b0011;

            /// 0b0100: ADC CMD4 selected.
            pub const CSEL0_4: u32 = 0b0100;

            /// 0b0101: ADC CMD5 selected.
            pub const CSEL0_5: u32 = 0b0101;

            /// 0b0110: ADC CMD6 selected.
            pub const CSEL0_6: u32 = 0b0110;

            /// 0b0111: ADC CMD7 selected.
            pub const CSEL0_7: u32 = 0b0111;

            /// 0b1000: ADC CMD8 selected.
            pub const CSEL0_8: u32 = 0b1000;

            /// 0b1001: ADC CMD9 selected.
            pub const CSEL0_9: u32 = 0b1001;

            /// 0b1010: ADC CMD10 selected.
            pub const CSEL0_10: u32 = 0b1010;

            /// 0b1011: ADC CMD11 selected.
            pub const CSEL0_11: u32 = 0b1011;

            /// 0b1100: ADC CMD12 selected.
            pub const CSEL0_12: u32 = 0b1100;

            /// 0b1101: ADC CMD13 selected.
            pub const CSEL0_13: u32 = 0b1101;

            /// 0b1110: ADC CMD14 selected.
            pub const CSEL0_14: u32 = 0b1110;

            /// 0b1111: ADC CMD15 selected.
            pub const CSEL0_15: u32 = 0b1111;
        }
    }

    /// Segment 0 HWTS ADC hardware trigger selection
    pub mod HWTS0 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (8 bits: 0xff << 4)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: no trigger selected
            pub const HWTS0_0: u32 = 0b00000000;

            /// 0b00000001: ADC TRIG0 selected
            pub const HWTS0_1: u32 = 0b00000001;

            /// 0b00000010: ADC TRIG1 selected
            pub const HWTS0_2: u32 = 0b00000010;

            /// 0b00000100: ADC TRIG2 selected
            pub const HWTS0_4: u32 = 0b00000100;

            /// 0b00001000: ADC TRIG3 selected
            pub const HWTS0_8: u32 = 0b00001000;

            /// 0b00010000: ADC TRIG4 selected
            pub const HWTS0_16: u32 = 0b00010000;

            /// 0b00100000: ADC TRIG5 selected
            pub const HWTS0_32: u32 = 0b00100000;

            /// 0b01000000: ADC TRIG6 selected
            pub const HWTS0_64: u32 = 0b01000000;

            /// 0b10000000: ADC TRIG7 selected
            pub const HWTS0_128: u32 = 0b10000000;
        }
    }

    /// Segment 0 B2B
    pub mod B2B0 {
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

            /// 0b0: Disable B2B. Wait until delay value defined by TRIG0_COUNTER\[SAMPLE_INTERVAL\] is reached
            pub const B2B0_0: u32 = 0b0;

            /// 0b1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached.
            pub const B2B0_1: u32 = 0b1;
        }
    }

    /// Segment 0 interrupt enable. (This bit field is meaningful only when IE0_EN is set)
    pub mod IE0 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generate interrupt on Done0 when segment 0 finish.
            pub const IE0_0: u32 = 0b00;

            /// 0b01: Generate interrupt on Done1 when segment 0 finish.
            pub const IE0_1: u32 = 0b01;

            /// 0b10: Generate interrupt on Done2 when segment 0 finish.
            pub const IE0_2: u32 = 0b10;

            /// 0b11: Generate interrupt on Done3 when segment 0 finish.
            pub const IE0_3: u32 = 0b11;
        }
    }

    /// IRQ enable of segment 0.
    pub mod IE0_EN {
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

            /// 0b0: Interrupt DONE disabled.
            pub const IE0_EN_0: u32 = 0b0;

            /// 0b1: Interrupt DONE enabled. When segment 0 finish, an interrupt will be generated on the specific port configured by the IE0.
            pub const IE0_EN_1: u32 = 0b1;
        }
    }

    /// ADC hardware trigger command selection
    pub mod CSEL1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not a valid selection from the command buffer. Trigger event is ignored.
            pub const CSEL1_0: u32 = 0b0000;

            /// 0b0001: ADC CMD1 selected.
            pub const CSEL1_1: u32 = 0b0001;

            /// 0b0010: ADC CMD2 selected.
            pub const CSEL1_2: u32 = 0b0010;

            /// 0b0011: ADC CMD3 selected.
            pub const CSEL1_3: u32 = 0b0011;

            /// 0b0100: ADC CMD4 selected.
            pub const CSEL1_4: u32 = 0b0100;

            /// 0b0101: ADC CMD5 selected.
            pub const CSEL1_5: u32 = 0b0101;

            /// 0b0110: ADC CMD6 selected.
            pub const CSEL1_6: u32 = 0b0110;

            /// 0b0111: ADC CMD7 selected.
            pub const CSEL1_7: u32 = 0b0111;

            /// 0b1000: ADC CMD8 selected.
            pub const CSEL1_8: u32 = 0b1000;

            /// 0b1001: ADC CMD9 selected.
            pub const CSEL1_9: u32 = 0b1001;

            /// 0b1010: ADC CMD10 selected.
            pub const CSEL1_10: u32 = 0b1010;

            /// 0b1011: ADC CMD11 selected.
            pub const CSEL1_11: u32 = 0b1011;

            /// 0b1100: ADC CMD12 selected.
            pub const CSEL1_12: u32 = 0b1100;

            /// 0b1101: ADC CMD13 selected.
            pub const CSEL1_13: u32 = 0b1101;

            /// 0b1110: ADC CMD14 selected.
            pub const CSEL1_14: u32 = 0b1110;

            /// 0b1111: ADC CMD15 selected.
            pub const CSEL1_15: u32 = 0b1111;
        }
    }

    /// Segment 1 HWTS ADC hardware trigger selection
    pub mod HWTS1 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (8 bits: 0xff << 20)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: no trigger selected
            pub const HWTS1_0: u32 = 0b00000000;

            /// 0b00000001: ADC TRIG0 selected
            pub const HWTS1_1: u32 = 0b00000001;

            /// 0b00000010: ADC TRIG1 selected
            pub const HWTS1_2: u32 = 0b00000010;

            /// 0b00000100: ADC TRIG2 selected
            pub const HWTS1_4: u32 = 0b00000100;

            /// 0b00001000: ADC TRIG3 selected
            pub const HWTS1_8: u32 = 0b00001000;

            /// 0b00010000: ADC TRIG4 selected
            pub const HWTS1_16: u32 = 0b00010000;

            /// 0b00100000: ADC TRIG5 selected
            pub const HWTS1_32: u32 = 0b00100000;

            /// 0b01000000: ADC TRIG6 selected
            pub const HWTS1_64: u32 = 0b01000000;

            /// 0b10000000: ADC TRIG7 selected
            pub const HWTS1_128: u32 = 0b10000000;
        }
    }

    /// Segment 1 B2B
    pub mod B2B1 {
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

            /// 0b0: Disable B2B. Wait until delay value defined by TRIG1_COUNTER\[SAMPLE_INTERVAL\] is reached
            pub const B2B1_0: u32 = 0b0;

            /// 0b1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached.
            pub const B2B1_1: u32 = 0b1;
        }
    }

    /// Segment 1 interrupt enable. (This bit field is meaningful only when IE1_EN is set)
    pub mod IE1 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generate interrupt on Done0 when Segment 1 finish.
            pub const IE1_0: u32 = 0b00;

            /// 0b01: Generate interrupt on Done1 when Segment 1 finish.
            pub const IE1_1: u32 = 0b01;

            /// 0b10: Generate interrupt on Done2 when Segment 1 finish.
            pub const IE1_2: u32 = 0b10;

            /// 0b11: Generate interrupt on Done3 when Segment 1 finish.
            pub const IE1_3: u32 = 0b11;
        }
    }

    /// IRQ enable of segment 1.
    pub mod IE1_EN {
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

            /// 0b0: Interrupt DONE disabled.
            pub const IE1_EN_0: u32 = 0b0;

            /// 0b1: Interrupt DONE enabled. When segment 1 finish, an interrupt will be generated on the specific port configured by the IE1.
            pub const IE1_EN_1: u32 = 0b1;
        }
    }
}

/// ETC_TRIG Chain 2/3 Register
pub mod TRIG0_CHAIN_3_2 {

    /// ADC hardware trigger command selection
    pub mod CSEL2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not a valid selection from the command buffer. Trigger event is ignored.
            pub const CSEL2_0: u32 = 0b0000;

            /// 0b0001: ADC CMD1 selected.
            pub const CSEL2_1: u32 = 0b0001;

            /// 0b0010: ADC CMD2 selected.
            pub const CSEL2_2: u32 = 0b0010;

            /// 0b0011: ADC CMD3 selected.
            pub const CSEL2_3: u32 = 0b0011;

            /// 0b0100: ADC CMD4 selected.
            pub const CSEL2_4: u32 = 0b0100;

            /// 0b0101: ADC CMD5 selected.
            pub const CSEL2_5: u32 = 0b0101;

            /// 0b0110: ADC CMD6 selected.
            pub const CSEL2_6: u32 = 0b0110;

            /// 0b0111: ADC CMD7 selected.
            pub const CSEL2_7: u32 = 0b0111;

            /// 0b1000: ADC CMD8 selected.
            pub const CSEL2_8: u32 = 0b1000;

            /// 0b1001: ADC CMD9 selected.
            pub const CSEL2_9: u32 = 0b1001;

            /// 0b1010: ADC CMD10 selected.
            pub const CSEL2_10: u32 = 0b1010;

            /// 0b1011: ADC CMD11 selected.
            pub const CSEL2_11: u32 = 0b1011;

            /// 0b1100: ADC CMD12 selected.
            pub const CSEL2_12: u32 = 0b1100;

            /// 0b1101: ADC CMD13 selected.
            pub const CSEL2_13: u32 = 0b1101;

            /// 0b1110: ADC CMD14 selected.
            pub const CSEL2_14: u32 = 0b1110;

            /// 0b1111: ADC CMD15 selected.
            pub const CSEL2_15: u32 = 0b1111;
        }
    }

    /// Segment 2 HWTS ADC hardware trigger selection
    pub mod HWTS2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (8 bits: 0xff << 4)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: no trigger selected
            pub const HWTS2_0: u32 = 0b00000000;

            /// 0b00000001: ADC TRIG0 selected
            pub const HWTS2_1: u32 = 0b00000001;

            /// 0b00000010: ADC TRIG1 selected
            pub const HWTS2_2: u32 = 0b00000010;

            /// 0b00000100: ADC TRIG2 selected
            pub const HWTS2_4: u32 = 0b00000100;

            /// 0b00001000: ADC TRIG3 selected
            pub const HWTS2_8: u32 = 0b00001000;

            /// 0b00010000: ADC TRIG4 selected
            pub const HWTS2_16: u32 = 0b00010000;

            /// 0b00100000: ADC TRIG5 selected
            pub const HWTS2_32: u32 = 0b00100000;

            /// 0b01000000: ADC TRIG6 selected
            pub const HWTS2_64: u32 = 0b01000000;

            /// 0b10000000: ADC TRIG7 selected
            pub const HWTS2_128: u32 = 0b10000000;
        }
    }

    /// Segment 2 B2B
    pub mod B2B2 {
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

            /// 0b0: Disable B2B. Wait until delay value defined by TRIG2_COUNTER\[SAMPLE_INTERVAL\] is reached
            pub const B2B2_0: u32 = 0b0;

            /// 0b1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached.
            pub const B2B2_1: u32 = 0b1;
        }
    }

    /// Segment 2 interrupt enable. (This bit field is meaningful only when IE2_EN is set)
    pub mod IE2 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generate interrupt on Done0 when segment 2 finish.
            pub const IE2_0: u32 = 0b00;

            /// 0b01: Generate interrupt on Done1 when segment 2 finish.
            pub const IE2_1: u32 = 0b01;

            /// 0b10: Generate interrupt on Done2 when segment 2 finish.
            pub const IE2_2: u32 = 0b10;

            /// 0b11: Generate interrupt on Done3 when segment 2 finish.
            pub const IE2_3: u32 = 0b11;
        }
    }

    /// IRQ enable of segment 2.
    pub mod IE2_EN {
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

            /// 0b0: Interrupt DONE disabled.
            pub const IE2_EN_0: u32 = 0b0;

            /// 0b1: Interrupt DONE enabled. When segment 2 finish, an interrupt will be generated on the specific port configured by the IE2.
            pub const IE2_EN_1: u32 = 0b1;
        }
    }

    /// ADC hardware trigger command selection
    pub mod CSEL3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not a valid selection from the command buffer. Trigger event is ignored.
            pub const CSEL3_0: u32 = 0b0000;

            /// 0b0001: ADC CMD1 selected.
            pub const CSEL3_1: u32 = 0b0001;

            /// 0b0010: ADC CMD2 selected.
            pub const CSEL3_2: u32 = 0b0010;

            /// 0b0011: ADC CMD3 selected.
            pub const CSEL3_3: u32 = 0b0011;

            /// 0b0100: ADC CMD4 selected.
            pub const CSEL3_4: u32 = 0b0100;

            /// 0b0101: ADC CMD5 selected.
            pub const CSEL3_5: u32 = 0b0101;

            /// 0b0110: ADC CMD6 selected.
            pub const CSEL3_6: u32 = 0b0110;

            /// 0b0111: ADC CMD7 selected.
            pub const CSEL3_7: u32 = 0b0111;

            /// 0b1000: ADC CMD8 selected.
            pub const CSEL3_8: u32 = 0b1000;

            /// 0b1001: ADC CMD9 selected.
            pub const CSEL3_9: u32 = 0b1001;

            /// 0b1010: ADC CMD10 selected.
            pub const CSEL3_10: u32 = 0b1010;

            /// 0b1011: ADC CMD11 selected.
            pub const CSEL3_11: u32 = 0b1011;

            /// 0b1100: ADC CMD12 selected.
            pub const CSEL3_12: u32 = 0b1100;

            /// 0b1101: ADC CMD13 selected.
            pub const CSEL3_13: u32 = 0b1101;

            /// 0b1110: ADC CMD14 selected.
            pub const CSEL3_14: u32 = 0b1110;

            /// 0b1111: ADC CMD15 selected.
            pub const CSEL3_15: u32 = 0b1111;
        }
    }

    /// Segment 3 HWTS ADC hardware trigger selection
    pub mod HWTS3 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (8 bits: 0xff << 20)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: no trigger selected
            pub const HWTS3_0: u32 = 0b00000000;

            /// 0b00000001: ADC TRIG0 selected
            pub const HWTS3_1: u32 = 0b00000001;

            /// 0b00000010: ADC TRIG1 selected
            pub const HWTS3_2: u32 = 0b00000010;

            /// 0b00000100: ADC TRIG2 selected
            pub const HWTS3_4: u32 = 0b00000100;

            /// 0b00001000: ADC TRIG3 selected
            pub const HWTS3_8: u32 = 0b00001000;

            /// 0b00010000: ADC TRIG4 selected
            pub const HWTS3_16: u32 = 0b00010000;

            /// 0b00100000: ADC TRIG5 selected
            pub const HWTS3_32: u32 = 0b00100000;

            /// 0b01000000: ADC TRIG6 selected
            pub const HWTS3_64: u32 = 0b01000000;

            /// 0b10000000: ADC TRIG7 selected
            pub const HWTS3_128: u32 = 0b10000000;
        }
    }

    /// Segment 3 B2B
    pub mod B2B3 {
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

            /// 0b0: Disable B2B. Wait until delay value defined by TRIG3_COUNTER\[SAMPLE_INTERVAL\] is reached
            pub const B2B3_0: u32 = 0b0;

            /// 0b1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached.
            pub const B2B3_1: u32 = 0b1;
        }
    }

    /// Segment 3 interrupt enable. (This bit field is meaningful only when IE3_EN is set)
    pub mod IE3 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generate interrupt on Done0 when segment 3 finish.
            pub const IE3_0: u32 = 0b00;

            /// 0b01: Generate interrupt on Done1 when segment 3 finish.
            pub const IE3_1: u32 = 0b01;

            /// 0b10: Generate interrupt on Done2 when segment 3 finish.
            pub const IE3_2: u32 = 0b10;

            /// 0b11: Generate interrupt on Done3 when segment 3 finish.
            pub const IE3_3: u32 = 0b11;
        }
    }

    /// IRQ enable of segment 3.
    pub mod IE3_EN {
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

            /// 0b0: Interrupt DONE disabled.
            pub const IE3_EN_0: u32 = 0b0;

            /// 0b1: Interrupt DONE enabled. When segment 3 finish, an interrupt will be generated on the specific port configured by the IE3.
            pub const IE3_EN_1: u32 = 0b1;
        }
    }
}

/// ETC_TRIG Chain 4/5 Register
pub mod TRIG0_CHAIN_5_4 {

    /// ADC hardware trigger command selection
    pub mod CSEL4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not a valid selection from the command buffer. Trigger event is ignored.
            pub const CSEL4_0: u32 = 0b0000;

            /// 0b0001: ADC CMD1 selected.
            pub const CSEL4_1: u32 = 0b0001;

            /// 0b0010: ADC CMD2 selected.
            pub const CSEL4_2: u32 = 0b0010;

            /// 0b0011: ADC CMD3 selected.
            pub const CSEL4_3: u32 = 0b0011;

            /// 0b0100: ADC CMD4 selected.
            pub const CSEL4_4: u32 = 0b0100;

            /// 0b0101: ADC CMD5 selected.
            pub const CSEL4_5: u32 = 0b0101;

            /// 0b0110: ADC CMD6 selected.
            pub const CSEL4_6: u32 = 0b0110;

            /// 0b0111: ADC CMD7 selected.
            pub const CSEL4_7: u32 = 0b0111;

            /// 0b1000: ADC CMD8 selected.
            pub const CSEL4_8: u32 = 0b1000;

            /// 0b1001: ADC CMD9 selected.
            pub const CSEL4_9: u32 = 0b1001;

            /// 0b1010: ADC CMD10 selected.
            pub const CSEL4_10: u32 = 0b1010;

            /// 0b1011: ADC CMD11 selected.
            pub const CSEL4_11: u32 = 0b1011;

            /// 0b1100: ADC CMD12 selected.
            pub const CSEL4_12: u32 = 0b1100;

            /// 0b1101: ADC CMD13 selected.
            pub const CSEL4_13: u32 = 0b1101;

            /// 0b1110: ADC CMD14 selected.
            pub const CSEL4_14: u32 = 0b1110;

            /// 0b1111: ADC CMD15 selected.
            pub const CSEL4_15: u32 = 0b1111;
        }
    }

    /// Segment 4 HWTS ADC hardware trigger selection
    pub mod HWTS4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (8 bits: 0xff << 4)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: no trigger selected
            pub const HWTS4_0: u32 = 0b00000000;

            /// 0b00000001: ADC TRIG0 selected
            pub const HWTS4_1: u32 = 0b00000001;

            /// 0b00000010: ADC TRIG1 selected
            pub const HWTS4_2: u32 = 0b00000010;

            /// 0b00000100: ADC TRIG2 selected
            pub const HWTS4_4: u32 = 0b00000100;

            /// 0b00001000: ADC TRIG3 selected
            pub const HWTS4_8: u32 = 0b00001000;

            /// 0b00010000: ADC TRIG4 selected
            pub const HWTS4_16: u32 = 0b00010000;

            /// 0b00100000: ADC TRIG5 selected
            pub const HWTS4_32: u32 = 0b00100000;

            /// 0b01000000: ADC TRIG6 selected
            pub const HWTS4_64: u32 = 0b01000000;

            /// 0b10000000: ADC TRIG7 selected
            pub const HWTS4_128: u32 = 0b10000000;
        }
    }

    /// Segment 4 B2B
    pub mod B2B4 {
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

            /// 0b0: Disable B2B. Wait until delay value defined by TRIG4_COUNTER\[SAMPLE_INTERVAL\] is reached
            pub const B2B4_0: u32 = 0b0;

            /// 0b1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached.
            pub const B2B4_1: u32 = 0b1;
        }
    }

    /// Segment 4 interrupt enable. (This bit field is meaningful only when IE4_EN is set)
    pub mod IE4 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generate interrupt on Done0 when segment 4 finish.
            pub const IE4_0: u32 = 0b00;

            /// 0b01: Generate interrupt on Done1 when segment 4 finish.
            pub const IE4_1: u32 = 0b01;

            /// 0b10: Generate interrupt on Done2 when segment 4 finish.
            pub const IE4_2: u32 = 0b10;

            /// 0b11: Generate interrupt on Done3 when segment 4 finish.
            pub const IE4_3: u32 = 0b11;
        }
    }

    /// IRQ enable of segment 4.
    pub mod IE4_EN {
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

            /// 0b0: Interrupt DONE disabled.
            pub const IE4_EN_0: u32 = 0b0;

            /// 0b1: Interrupt DONE enabled. When segment 4 finish, an interrupt will be generated on the specific port configured by the IE4.
            pub const IE4_EN_1: u32 = 0b1;
        }
    }

    /// ADC hardware trigger command selection
    pub mod CSEL5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not a valid selection from the command buffer. Trigger event is ignored.
            pub const CSEL5_0: u32 = 0b0000;

            /// 0b0001: ADC CMD1 selected.
            pub const CSEL5_1: u32 = 0b0001;

            /// 0b0010: ADC CMD2 selected.
            pub const CSEL5_2: u32 = 0b0010;

            /// 0b0011: ADC CMD3 selected.
            pub const CSEL5_3: u32 = 0b0011;

            /// 0b0100: ADC CMD4 selected.
            pub const CSEL5_4: u32 = 0b0100;

            /// 0b0101: ADC CMD5 selected.
            pub const CSEL5_5: u32 = 0b0101;

            /// 0b0110: ADC CMD6 selected.
            pub const CSEL5_6: u32 = 0b0110;

            /// 0b0111: ADC CMD7 selected.
            pub const CSEL5_7: u32 = 0b0111;

            /// 0b1000: ADC CMD8 selected.
            pub const CSEL5_8: u32 = 0b1000;

            /// 0b1001: ADC CMD9 selected.
            pub const CSEL5_9: u32 = 0b1001;

            /// 0b1010: ADC CMD10 selected.
            pub const CSEL5_10: u32 = 0b1010;

            /// 0b1011: ADC CMD11 selected.
            pub const CSEL5_11: u32 = 0b1011;

            /// 0b1100: ADC CMD12 selected.
            pub const CSEL5_12: u32 = 0b1100;

            /// 0b1101: ADC CMD13 selected.
            pub const CSEL5_13: u32 = 0b1101;

            /// 0b1110: ADC CMD14 selected.
            pub const CSEL5_14: u32 = 0b1110;

            /// 0b1111: ADC CMD15 selected.
            pub const CSEL5_15: u32 = 0b1111;
        }
    }

    /// Segment 5 HWTS ADC hardware trigger selection
    pub mod HWTS5 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (8 bits: 0xff << 20)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: no trigger selected
            pub const HWTS5_0: u32 = 0b00000000;

            /// 0b00000001: ADC TRIG0 selected
            pub const HWTS5_1: u32 = 0b00000001;

            /// 0b00000010: ADC TRIG1 selected
            pub const HWTS5_2: u32 = 0b00000010;

            /// 0b00000100: ADC TRIG2 selected
            pub const HWTS5_4: u32 = 0b00000100;

            /// 0b00001000: ADC TRIG3 selected
            pub const HWTS5_8: u32 = 0b00001000;

            /// 0b00010000: ADC TRIG4 selected
            pub const HWTS5_16: u32 = 0b00010000;

            /// 0b00100000: ADC TRIG5 selected
            pub const HWTS5_32: u32 = 0b00100000;

            /// 0b01000000: ADC TRIG6 selected
            pub const HWTS5_64: u32 = 0b01000000;

            /// 0b10000000: ADC TRIG7 selected
            pub const HWTS5_128: u32 = 0b10000000;
        }
    }

    /// Segment 5 B2B
    pub mod B2B5 {
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

            /// 0b0: Disable B2B. Wait until delay value defined by TRIG5_COUNTER\[SAMPLE_INTERVAL\] is reached
            pub const B2B5_0: u32 = 0b0;

            /// 0b1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached.
            pub const B2B5_1: u32 = 0b1;
        }
    }

    /// Segment 5 interrupt enable. (This bit field is meaningful only when IE5_EN is set)
    pub mod IE5 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generate interrupt on Done0 when segment 5 finish.
            pub const IE5_0: u32 = 0b00;

            /// 0b01: Generate interrupt on Done1 when segment 5 finish.
            pub const IE5_1: u32 = 0b01;

            /// 0b10: Generate interrupt on Done2 when segment 5 finish.
            pub const IE5_2: u32 = 0b10;

            /// 0b11: Generate interrupt on Done3 when segment 5 finish.
            pub const IE5_3: u32 = 0b11;
        }
    }

    /// IRQ enable of segment 5.
    pub mod IE5_EN {
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

            /// 0b0: Interrupt DONE disabled.
            pub const IE5_EN_0: u32 = 0b0;

            /// 0b1: Interrupt DONE enabled. When segment 5 finish, an interrupt will be generated on the specific port configured by the IE5.
            pub const IE5_EN_1: u32 = 0b1;
        }
    }
}

/// ETC_TRIG Chain 6/7 Register
pub mod TRIG0_CHAIN_7_6 {

    /// ADC hardware trigger command selection
    pub mod CSEL6 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not a valid selection from the command buffer. Trigger event is ignored.
            pub const CSEL6_0: u32 = 0b0000;

            /// 0b0001: ADC CMD1 selected.
            pub const CSEL6_1: u32 = 0b0001;

            /// 0b0010: ADC CMD2 selected.
            pub const CSEL6_2: u32 = 0b0010;

            /// 0b0011: ADC CMD3 selected.
            pub const CSEL6_3: u32 = 0b0011;

            /// 0b0100: ADC CMD4 selected.
            pub const CSEL6_4: u32 = 0b0100;

            /// 0b0101: ADC CMD5 selected.
            pub const CSEL6_5: u32 = 0b0101;

            /// 0b0110: ADC CMD6 selected.
            pub const CSEL6_6: u32 = 0b0110;

            /// 0b0111: ADC CMD7 selected.
            pub const CSEL6_7: u32 = 0b0111;

            /// 0b1000: ADC CMD8 selected.
            pub const CSEL6_8: u32 = 0b1000;

            /// 0b1001: ADC CMD9 selected.
            pub const CSEL6_9: u32 = 0b1001;

            /// 0b1010: ADC CMD10 selected.
            pub const CSEL6_10: u32 = 0b1010;

            /// 0b1011: ADC CMD11 selected.
            pub const CSEL6_11: u32 = 0b1011;

            /// 0b1100: ADC CMD12 selected.
            pub const CSEL6_12: u32 = 0b1100;

            /// 0b1101: ADC CMD13 selected.
            pub const CSEL6_13: u32 = 0b1101;

            /// 0b1110: ADC CMD14 selected.
            pub const CSEL6_14: u32 = 0b1110;

            /// 0b1111: ADC CMD15 selected.
            pub const CSEL6_15: u32 = 0b1111;
        }
    }

    /// Segment 6 HWTS ADC hardware trigger selection
    pub mod HWTS6 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (8 bits: 0xff << 4)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: no trigger selected
            pub const HWTS6_0: u32 = 0b00000000;

            /// 0b00000001: ADC TRIG0 selected
            pub const HWTS6_1: u32 = 0b00000001;

            /// 0b00000010: ADC TRIG1 selected
            pub const HWTS6_2: u32 = 0b00000010;

            /// 0b00000100: ADC TRIG2 selected
            pub const HWTS6_4: u32 = 0b00000100;

            /// 0b00001000: ADC TRIG3 selected
            pub const HWTS6_8: u32 = 0b00001000;

            /// 0b00010000: ADC TRIG4 selected
            pub const HWTS6_16: u32 = 0b00010000;

            /// 0b00100000: ADC TRIG5 selected
            pub const HWTS6_32: u32 = 0b00100000;

            /// 0b01000000: ADC TRIG6 selected
            pub const HWTS6_64: u32 = 0b01000000;

            /// 0b10000000: ADC TRIG7 selected
            pub const HWTS6_128: u32 = 0b10000000;
        }
    }

    /// Segment 6 B2B
    pub mod B2B6 {
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

            /// 0b0: Disable B2B. Wait until delay value defined by TRIG6_COUNTER\[SAMPLE_INTERVAL\] is reached
            pub const B2B6_0: u32 = 0b0;

            /// 0b1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached.
            pub const B2B6_1: u32 = 0b1;
        }
    }

    /// Segment 6 interrupt enable. (This bit field is meaningful only when IE6_EN is set)
    pub mod IE6 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generate interrupt on Done0 when segment 6 finish.
            pub const IE6_0: u32 = 0b00;

            /// 0b01: Generate interrupt on Done1 when segment 6 finish.
            pub const IE6_1: u32 = 0b01;

            /// 0b10: Generate interrupt on Done2 when segment 6 finish.
            pub const IE6_2: u32 = 0b10;

            /// 0b11: Generate interrupt on Done3 when segment 6 finish.
            pub const IE6_3: u32 = 0b11;
        }
    }

    /// IRQ enable of segment 6.
    pub mod IE6_EN {
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

            /// 0b0: Interrupt DONE disabled.
            pub const IE6_EN_0: u32 = 0b0;

            /// 0b1: Interrupt DONE enabled. When segment 6 finish, an interrupt will be generated on the specific port configured by the IE6.
            pub const IE6_EN_1: u32 = 0b1;
        }
    }

    /// ADC hardware trigger command selection
    pub mod CSEL7 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not a valid selection from the command buffer. Trigger event is ignored.
            pub const CSEL7_0: u32 = 0b0000;

            /// 0b0001: ADC CMD1 selected.
            pub const CSEL7_1: u32 = 0b0001;

            /// 0b0010: ADC CMD2 selected.
            pub const CSEL7_2: u32 = 0b0010;

            /// 0b0011: ADC CMD3 selected.
            pub const CSEL7_3: u32 = 0b0011;

            /// 0b0100: ADC CMD4 selected.
            pub const CSEL7_4: u32 = 0b0100;

            /// 0b0101: ADC CMD5 selected.
            pub const CSEL7_5: u32 = 0b0101;

            /// 0b0110: ADC CMD6 selected.
            pub const CSEL7_6: u32 = 0b0110;

            /// 0b0111: ADC CMD7 selected.
            pub const CSEL7_7: u32 = 0b0111;

            /// 0b1000: ADC CMD8 selected.
            pub const CSEL7_8: u32 = 0b1000;

            /// 0b1001: ADC CMD9 selected.
            pub const CSEL7_9: u32 = 0b1001;

            /// 0b1010: ADC CMD10 selected.
            pub const CSEL7_10: u32 = 0b1010;

            /// 0b1011: ADC CMD11 selected.
            pub const CSEL7_11: u32 = 0b1011;

            /// 0b1100: ADC CMD12 selected.
            pub const CSEL7_12: u32 = 0b1100;

            /// 0b1101: ADC CMD13 selected.
            pub const CSEL7_13: u32 = 0b1101;

            /// 0b1110: ADC CMD14 selected.
            pub const CSEL7_14: u32 = 0b1110;

            /// 0b1111: ADC CMD15 selected.
            pub const CSEL7_15: u32 = 0b1111;
        }
    }

    /// Segment 7 HWTS ADC hardware trigger selection
    pub mod HWTS7 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (8 bits: 0xff << 20)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: no trigger selected
            pub const HWTS7_0: u32 = 0b00000000;

            /// 0b00000001: ADC TRIG0 selected
            pub const HWTS7_1: u32 = 0b00000001;

            /// 0b00000010: ADC TRIG1 selected
            pub const HWTS7_2: u32 = 0b00000010;

            /// 0b00000100: ADC TRIG2 selected
            pub const HWTS7_4: u32 = 0b00000100;

            /// 0b00001000: ADC TRIG3 selected
            pub const HWTS7_8: u32 = 0b00001000;

            /// 0b00010000: ADC TRIG4 selected
            pub const HWTS7_16: u32 = 0b00010000;

            /// 0b00100000: ADC TRIG5 selected
            pub const HWTS7_32: u32 = 0b00100000;

            /// 0b01000000: ADC TRIG6 selected
            pub const HWTS7_64: u32 = 0b01000000;

            /// 0b10000000: ADC TRIG7 selected
            pub const HWTS7_128: u32 = 0b10000000;
        }
    }

    /// Segment 7 B2B
    pub mod B2B7 {
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

            /// 0b0: Disable B2B. Wait until delay value defined by TRIG7_COUNTER\[SAMPLE_INTERVAL\] is reached
            pub const B2B7_0: u32 = 0b0;

            /// 0b1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached.
            pub const B2B7_1: u32 = 0b1;
        }
    }

    /// Segment 7 interrupt enable. (This bit field is meaningful only when IE7_EN is set)
    pub mod IE7 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generate interrupt on Done0 when segment 7 finish.
            pub const IE7_0: u32 = 0b00;

            /// 0b01: Generate interrupt on Done1 when segment 7 finish.
            pub const IE7_1: u32 = 0b01;

            /// 0b10: Generate interrupt on Done2 when segment 7 finish.
            pub const IE7_2: u32 = 0b10;

            /// 0b11: Generate interrupt on Done3 when segment 7 finish.
            pub const IE7_3: u32 = 0b11;
        }
    }

    /// IRQ enable of segment 7.
    pub mod IE7_EN {
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

            /// 0b0: Interrupt DONE disabled.
            pub const IE7_EN_0: u32 = 0b0;

            /// 0b1: Interrupt DONE enabled. When segment 7 finish, an interrupt will be generated on the specific port configured by the IE7.
            pub const IE7_EN_1: u32 = 0b1;
        }
    }
}

/// ETC_TRIG Result Data 1/0 Register
pub mod TRIG0_RESULT_1_0 {

    /// Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module
    pub mod DATA0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module
    pub mod DATA1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ETC_TRIG Result Data 3/2 Register
pub mod TRIG0_RESULT_3_2 {

    /// Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module
    pub mod DATA2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module
    pub mod DATA3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ETC_TRIG Result Data 5/4 Register
pub mod TRIG0_RESULT_5_4 {

    /// Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module
    pub mod DATA4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module
    pub mod DATA5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ETC_TRIG Result Data 7/6 Register
pub mod TRIG0_RESULT_7_6 {

    /// Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module
    pub mod DATA6 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module
    pub mod DATA7 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ETC_TRIG Control Register
pub mod TRIG1_CTRL {
    pub use super::TRIG0_CTRL::CHAINx_DONE;
    pub use super::TRIG0_CTRL::SW_TRIG;
    pub use super::TRIG0_CTRL::SYNC_MODE;
    pub use super::TRIG0_CTRL::TRIG_CHAIN;
    pub use super::TRIG0_CTRL::TRIG_MODE;
    pub use super::TRIG0_CTRL::TRIG_PRIORITY;
}

/// ETC_TRIG Counter Register
pub mod TRIG1_COUNTER {
    pub use super::TRIG0_COUNTER::INIT_DELAY;
    pub use super::TRIG0_COUNTER::SAMPLE_INTERVAL;
}

/// ETC_TRIG Chain 0/1 Register
pub mod TRIG1_CHAIN_1_0 {
    pub use super::TRIG0_CHAIN_1_0::B2B0;
    pub use super::TRIG0_CHAIN_1_0::B2B1;
    pub use super::TRIG0_CHAIN_1_0::CSEL0;
    pub use super::TRIG0_CHAIN_1_0::CSEL1;
    pub use super::TRIG0_CHAIN_1_0::HWTS0;
    pub use super::TRIG0_CHAIN_1_0::HWTS1;
    pub use super::TRIG0_CHAIN_1_0::IE0;
    pub use super::TRIG0_CHAIN_1_0::IE0_EN;
    pub use super::TRIG0_CHAIN_1_0::IE1;
    pub use super::TRIG0_CHAIN_1_0::IE1_EN;
}

/// ETC_TRIG Chain 2/3 Register
pub mod TRIG1_CHAIN_3_2 {
    pub use super::TRIG0_CHAIN_3_2::B2B2;
    pub use super::TRIG0_CHAIN_3_2::B2B3;
    pub use super::TRIG0_CHAIN_3_2::CSEL2;
    pub use super::TRIG0_CHAIN_3_2::CSEL3;
    pub use super::TRIG0_CHAIN_3_2::HWTS2;
    pub use super::TRIG0_CHAIN_3_2::HWTS3;
    pub use super::TRIG0_CHAIN_3_2::IE2;
    pub use super::TRIG0_CHAIN_3_2::IE2_EN;
    pub use super::TRIG0_CHAIN_3_2::IE3;
    pub use super::TRIG0_CHAIN_3_2::IE3_EN;
}

/// ETC_TRIG Chain 4/5 Register
pub mod TRIG1_CHAIN_5_4 {
    pub use super::TRIG0_CHAIN_5_4::B2B4;
    pub use super::TRIG0_CHAIN_5_4::B2B5;
    pub use super::TRIG0_CHAIN_5_4::CSEL4;
    pub use super::TRIG0_CHAIN_5_4::CSEL5;
    pub use super::TRIG0_CHAIN_5_4::HWTS4;
    pub use super::TRIG0_CHAIN_5_4::HWTS5;
    pub use super::TRIG0_CHAIN_5_4::IE4;
    pub use super::TRIG0_CHAIN_5_4::IE4_EN;
    pub use super::TRIG0_CHAIN_5_4::IE5;
    pub use super::TRIG0_CHAIN_5_4::IE5_EN;
}

/// ETC_TRIG Chain 6/7 Register
pub mod TRIG1_CHAIN_7_6 {
    pub use super::TRIG0_CHAIN_7_6::B2B6;
    pub use super::TRIG0_CHAIN_7_6::B2B7;
    pub use super::TRIG0_CHAIN_7_6::CSEL6;
    pub use super::TRIG0_CHAIN_7_6::CSEL7;
    pub use super::TRIG0_CHAIN_7_6::HWTS6;
    pub use super::TRIG0_CHAIN_7_6::HWTS7;
    pub use super::TRIG0_CHAIN_7_6::IE6;
    pub use super::TRIG0_CHAIN_7_6::IE6_EN;
    pub use super::TRIG0_CHAIN_7_6::IE7;
    pub use super::TRIG0_CHAIN_7_6::IE7_EN;
}

/// ETC_TRIG Result Data 1/0 Register
pub mod TRIG1_RESULT_1_0 {
    pub use super::TRIG0_RESULT_1_0::DATA0;
    pub use super::TRIG0_RESULT_1_0::DATA1;
}

/// ETC_TRIG Result Data 3/2 Register
pub mod TRIG1_RESULT_3_2 {
    pub use super::TRIG0_RESULT_3_2::DATA2;
    pub use super::TRIG0_RESULT_3_2::DATA3;
}

/// ETC_TRIG Result Data 5/4 Register
pub mod TRIG1_RESULT_5_4 {
    pub use super::TRIG0_RESULT_5_4::DATA4;
    pub use super::TRIG0_RESULT_5_4::DATA5;
}

/// ETC_TRIG Result Data 7/6 Register
pub mod TRIG1_RESULT_7_6 {
    pub use super::TRIG0_RESULT_7_6::DATA6;
    pub use super::TRIG0_RESULT_7_6::DATA7;
}

/// ETC_TRIG Control Register
pub mod TRIG2_CTRL {
    pub use super::TRIG0_CTRL::CHAINx_DONE;
    pub use super::TRIG0_CTRL::SW_TRIG;
    pub use super::TRIG0_CTRL::SYNC_MODE;
    pub use super::TRIG0_CTRL::TRIG_CHAIN;
    pub use super::TRIG0_CTRL::TRIG_MODE;
    pub use super::TRIG0_CTRL::TRIG_PRIORITY;
}

/// ETC_TRIG Counter Register
pub mod TRIG2_COUNTER {
    pub use super::TRIG0_COUNTER::INIT_DELAY;
    pub use super::TRIG0_COUNTER::SAMPLE_INTERVAL;
}

/// ETC_TRIG Chain 0/1 Register
pub mod TRIG2_CHAIN_1_0 {
    pub use super::TRIG0_CHAIN_1_0::B2B0;
    pub use super::TRIG0_CHAIN_1_0::B2B1;
    pub use super::TRIG0_CHAIN_1_0::CSEL0;
    pub use super::TRIG0_CHAIN_1_0::CSEL1;
    pub use super::TRIG0_CHAIN_1_0::HWTS0;
    pub use super::TRIG0_CHAIN_1_0::HWTS1;
    pub use super::TRIG0_CHAIN_1_0::IE0;
    pub use super::TRIG0_CHAIN_1_0::IE0_EN;
    pub use super::TRIG0_CHAIN_1_0::IE1;
    pub use super::TRIG0_CHAIN_1_0::IE1_EN;
}

/// ETC_TRIG Chain 2/3 Register
pub mod TRIG2_CHAIN_3_2 {
    pub use super::TRIG0_CHAIN_3_2::B2B2;
    pub use super::TRIG0_CHAIN_3_2::B2B3;
    pub use super::TRIG0_CHAIN_3_2::CSEL2;
    pub use super::TRIG0_CHAIN_3_2::CSEL3;
    pub use super::TRIG0_CHAIN_3_2::HWTS2;
    pub use super::TRIG0_CHAIN_3_2::HWTS3;
    pub use super::TRIG0_CHAIN_3_2::IE2;
    pub use super::TRIG0_CHAIN_3_2::IE2_EN;
    pub use super::TRIG0_CHAIN_3_2::IE3;
    pub use super::TRIG0_CHAIN_3_2::IE3_EN;
}

/// ETC_TRIG Chain 4/5 Register
pub mod TRIG2_CHAIN_5_4 {
    pub use super::TRIG0_CHAIN_5_4::B2B4;
    pub use super::TRIG0_CHAIN_5_4::B2B5;
    pub use super::TRIG0_CHAIN_5_4::CSEL4;
    pub use super::TRIG0_CHAIN_5_4::CSEL5;
    pub use super::TRIG0_CHAIN_5_4::HWTS4;
    pub use super::TRIG0_CHAIN_5_4::HWTS5;
    pub use super::TRIG0_CHAIN_5_4::IE4;
    pub use super::TRIG0_CHAIN_5_4::IE4_EN;
    pub use super::TRIG0_CHAIN_5_4::IE5;
    pub use super::TRIG0_CHAIN_5_4::IE5_EN;
}

/// ETC_TRIG Chain 6/7 Register
pub mod TRIG2_CHAIN_7_6 {
    pub use super::TRIG0_CHAIN_7_6::B2B6;
    pub use super::TRIG0_CHAIN_7_6::B2B7;
    pub use super::TRIG0_CHAIN_7_6::CSEL6;
    pub use super::TRIG0_CHAIN_7_6::CSEL7;
    pub use super::TRIG0_CHAIN_7_6::HWTS6;
    pub use super::TRIG0_CHAIN_7_6::HWTS7;
    pub use super::TRIG0_CHAIN_7_6::IE6;
    pub use super::TRIG0_CHAIN_7_6::IE6_EN;
    pub use super::TRIG0_CHAIN_7_6::IE7;
    pub use super::TRIG0_CHAIN_7_6::IE7_EN;
}

/// ETC_TRIG Result Data 1/0 Register
pub mod TRIG2_RESULT_1_0 {
    pub use super::TRIG0_RESULT_1_0::DATA0;
    pub use super::TRIG0_RESULT_1_0::DATA1;
}

/// ETC_TRIG Result Data 3/2 Register
pub mod TRIG2_RESULT_3_2 {
    pub use super::TRIG0_RESULT_3_2::DATA2;
    pub use super::TRIG0_RESULT_3_2::DATA3;
}

/// ETC_TRIG Result Data 5/4 Register
pub mod TRIG2_RESULT_5_4 {
    pub use super::TRIG0_RESULT_5_4::DATA4;
    pub use super::TRIG0_RESULT_5_4::DATA5;
}

/// ETC_TRIG Result Data 7/6 Register
pub mod TRIG2_RESULT_7_6 {
    pub use super::TRIG0_RESULT_7_6::DATA6;
    pub use super::TRIG0_RESULT_7_6::DATA7;
}

/// ETC_TRIG Control Register
pub mod TRIG3_CTRL {
    pub use super::TRIG0_CTRL::CHAINx_DONE;
    pub use super::TRIG0_CTRL::SW_TRIG;
    pub use super::TRIG0_CTRL::SYNC_MODE;
    pub use super::TRIG0_CTRL::TRIG_CHAIN;
    pub use super::TRIG0_CTRL::TRIG_MODE;
    pub use super::TRIG0_CTRL::TRIG_PRIORITY;
}

/// ETC_TRIG Counter Register
pub mod TRIG3_COUNTER {
    pub use super::TRIG0_COUNTER::INIT_DELAY;
    pub use super::TRIG0_COUNTER::SAMPLE_INTERVAL;
}

/// ETC_TRIG Chain 0/1 Register
pub mod TRIG3_CHAIN_1_0 {
    pub use super::TRIG0_CHAIN_1_0::B2B0;
    pub use super::TRIG0_CHAIN_1_0::B2B1;
    pub use super::TRIG0_CHAIN_1_0::CSEL0;
    pub use super::TRIG0_CHAIN_1_0::CSEL1;
    pub use super::TRIG0_CHAIN_1_0::HWTS0;
    pub use super::TRIG0_CHAIN_1_0::HWTS1;
    pub use super::TRIG0_CHAIN_1_0::IE0;
    pub use super::TRIG0_CHAIN_1_0::IE0_EN;
    pub use super::TRIG0_CHAIN_1_0::IE1;
    pub use super::TRIG0_CHAIN_1_0::IE1_EN;
}

/// ETC_TRIG Chain 2/3 Register
pub mod TRIG3_CHAIN_3_2 {
    pub use super::TRIG0_CHAIN_3_2::B2B2;
    pub use super::TRIG0_CHAIN_3_2::B2B3;
    pub use super::TRIG0_CHAIN_3_2::CSEL2;
    pub use super::TRIG0_CHAIN_3_2::CSEL3;
    pub use super::TRIG0_CHAIN_3_2::HWTS2;
    pub use super::TRIG0_CHAIN_3_2::HWTS3;
    pub use super::TRIG0_CHAIN_3_2::IE2;
    pub use super::TRIG0_CHAIN_3_2::IE2_EN;
    pub use super::TRIG0_CHAIN_3_2::IE3;
    pub use super::TRIG0_CHAIN_3_2::IE3_EN;
}

/// ETC_TRIG Chain 4/5 Register
pub mod TRIG3_CHAIN_5_4 {
    pub use super::TRIG0_CHAIN_5_4::B2B4;
    pub use super::TRIG0_CHAIN_5_4::B2B5;
    pub use super::TRIG0_CHAIN_5_4::CSEL4;
    pub use super::TRIG0_CHAIN_5_4::CSEL5;
    pub use super::TRIG0_CHAIN_5_4::HWTS4;
    pub use super::TRIG0_CHAIN_5_4::HWTS5;
    pub use super::TRIG0_CHAIN_5_4::IE4;
    pub use super::TRIG0_CHAIN_5_4::IE4_EN;
    pub use super::TRIG0_CHAIN_5_4::IE5;
    pub use super::TRIG0_CHAIN_5_4::IE5_EN;
}

/// ETC_TRIG Chain 6/7 Register
pub mod TRIG3_CHAIN_7_6 {
    pub use super::TRIG0_CHAIN_7_6::B2B6;
    pub use super::TRIG0_CHAIN_7_6::B2B7;
    pub use super::TRIG0_CHAIN_7_6::CSEL6;
    pub use super::TRIG0_CHAIN_7_6::CSEL7;
    pub use super::TRIG0_CHAIN_7_6::HWTS6;
    pub use super::TRIG0_CHAIN_7_6::HWTS7;
    pub use super::TRIG0_CHAIN_7_6::IE6;
    pub use super::TRIG0_CHAIN_7_6::IE6_EN;
    pub use super::TRIG0_CHAIN_7_6::IE7;
    pub use super::TRIG0_CHAIN_7_6::IE7_EN;
}

/// ETC_TRIG Result Data 1/0 Register
pub mod TRIG3_RESULT_1_0 {
    pub use super::TRIG0_RESULT_1_0::DATA0;
    pub use super::TRIG0_RESULT_1_0::DATA1;
}

/// ETC_TRIG Result Data 3/2 Register
pub mod TRIG3_RESULT_3_2 {
    pub use super::TRIG0_RESULT_3_2::DATA2;
    pub use super::TRIG0_RESULT_3_2::DATA3;
}

/// ETC_TRIG Result Data 5/4 Register
pub mod TRIG3_RESULT_5_4 {
    pub use super::TRIG0_RESULT_5_4::DATA4;
    pub use super::TRIG0_RESULT_5_4::DATA5;
}

/// ETC_TRIG Result Data 7/6 Register
pub mod TRIG3_RESULT_7_6 {
    pub use super::TRIG0_RESULT_7_6::DATA6;
    pub use super::TRIG0_RESULT_7_6::DATA7;
}

/// ETC_TRIG Control Register
pub mod TRIG4_CTRL {
    pub use super::TRIG0_CTRL::CHAINx_DONE;
    pub use super::TRIG0_CTRL::SW_TRIG;
    pub use super::TRIG0_CTRL::SYNC_MODE;
    pub use super::TRIG0_CTRL::TRIG_CHAIN;
    pub use super::TRIG0_CTRL::TRIG_MODE;
    pub use super::TRIG0_CTRL::TRIG_PRIORITY;
}

/// ETC_TRIG Counter Register
pub mod TRIG4_COUNTER {
    pub use super::TRIG0_COUNTER::INIT_DELAY;
    pub use super::TRIG0_COUNTER::SAMPLE_INTERVAL;
}

/// ETC_TRIG Chain 0/1 Register
pub mod TRIG4_CHAIN_1_0 {
    pub use super::TRIG0_CHAIN_1_0::B2B0;
    pub use super::TRIG0_CHAIN_1_0::B2B1;
    pub use super::TRIG0_CHAIN_1_0::CSEL0;
    pub use super::TRIG0_CHAIN_1_0::CSEL1;
    pub use super::TRIG0_CHAIN_1_0::HWTS0;
    pub use super::TRIG0_CHAIN_1_0::HWTS1;
    pub use super::TRIG0_CHAIN_1_0::IE0;
    pub use super::TRIG0_CHAIN_1_0::IE0_EN;
    pub use super::TRIG0_CHAIN_1_0::IE1;
    pub use super::TRIG0_CHAIN_1_0::IE1_EN;
}

/// ETC_TRIG Chain 2/3 Register
pub mod TRIG4_CHAIN_3_2 {
    pub use super::TRIG0_CHAIN_3_2::B2B2;
    pub use super::TRIG0_CHAIN_3_2::B2B3;
    pub use super::TRIG0_CHAIN_3_2::CSEL2;
    pub use super::TRIG0_CHAIN_3_2::CSEL3;
    pub use super::TRIG0_CHAIN_3_2::HWTS2;
    pub use super::TRIG0_CHAIN_3_2::HWTS3;
    pub use super::TRIG0_CHAIN_3_2::IE2;
    pub use super::TRIG0_CHAIN_3_2::IE2_EN;
    pub use super::TRIG0_CHAIN_3_2::IE3;
    pub use super::TRIG0_CHAIN_3_2::IE3_EN;
}

/// ETC_TRIG Chain 4/5 Register
pub mod TRIG4_CHAIN_5_4 {
    pub use super::TRIG0_CHAIN_5_4::B2B4;
    pub use super::TRIG0_CHAIN_5_4::B2B5;
    pub use super::TRIG0_CHAIN_5_4::CSEL4;
    pub use super::TRIG0_CHAIN_5_4::CSEL5;
    pub use super::TRIG0_CHAIN_5_4::HWTS4;
    pub use super::TRIG0_CHAIN_5_4::HWTS5;
    pub use super::TRIG0_CHAIN_5_4::IE4;
    pub use super::TRIG0_CHAIN_5_4::IE4_EN;
    pub use super::TRIG0_CHAIN_5_4::IE5;
    pub use super::TRIG0_CHAIN_5_4::IE5_EN;
}

/// ETC_TRIG Chain 6/7 Register
pub mod TRIG4_CHAIN_7_6 {
    pub use super::TRIG0_CHAIN_7_6::B2B6;
    pub use super::TRIG0_CHAIN_7_6::B2B7;
    pub use super::TRIG0_CHAIN_7_6::CSEL6;
    pub use super::TRIG0_CHAIN_7_6::CSEL7;
    pub use super::TRIG0_CHAIN_7_6::HWTS6;
    pub use super::TRIG0_CHAIN_7_6::HWTS7;
    pub use super::TRIG0_CHAIN_7_6::IE6;
    pub use super::TRIG0_CHAIN_7_6::IE6_EN;
    pub use super::TRIG0_CHAIN_7_6::IE7;
    pub use super::TRIG0_CHAIN_7_6::IE7_EN;
}

/// ETC_TRIG Result Data 1/0 Register
pub mod TRIG4_RESULT_1_0 {
    pub use super::TRIG0_RESULT_1_0::DATA0;
    pub use super::TRIG0_RESULT_1_0::DATA1;
}

/// ETC_TRIG Result Data 3/2 Register
pub mod TRIG4_RESULT_3_2 {
    pub use super::TRIG0_RESULT_3_2::DATA2;
    pub use super::TRIG0_RESULT_3_2::DATA3;
}

/// ETC_TRIG Result Data 5/4 Register
pub mod TRIG4_RESULT_5_4 {
    pub use super::TRIG0_RESULT_5_4::DATA4;
    pub use super::TRIG0_RESULT_5_4::DATA5;
}

/// ETC_TRIG Result Data 7/6 Register
pub mod TRIG4_RESULT_7_6 {
    pub use super::TRIG0_RESULT_7_6::DATA6;
    pub use super::TRIG0_RESULT_7_6::DATA7;
}

/// ETC_TRIG Control Register
pub mod TRIG5_CTRL {
    pub use super::TRIG0_CTRL::CHAINx_DONE;
    pub use super::TRIG0_CTRL::SW_TRIG;
    pub use super::TRIG0_CTRL::SYNC_MODE;
    pub use super::TRIG0_CTRL::TRIG_CHAIN;
    pub use super::TRIG0_CTRL::TRIG_MODE;
    pub use super::TRIG0_CTRL::TRIG_PRIORITY;
}

/// ETC_TRIG Counter Register
pub mod TRIG5_COUNTER {
    pub use super::TRIG0_COUNTER::INIT_DELAY;
    pub use super::TRIG0_COUNTER::SAMPLE_INTERVAL;
}

/// ETC_TRIG Chain 0/1 Register
pub mod TRIG5_CHAIN_1_0 {
    pub use super::TRIG0_CHAIN_1_0::B2B0;
    pub use super::TRIG0_CHAIN_1_0::B2B1;
    pub use super::TRIG0_CHAIN_1_0::CSEL0;
    pub use super::TRIG0_CHAIN_1_0::CSEL1;
    pub use super::TRIG0_CHAIN_1_0::HWTS0;
    pub use super::TRIG0_CHAIN_1_0::HWTS1;
    pub use super::TRIG0_CHAIN_1_0::IE0;
    pub use super::TRIG0_CHAIN_1_0::IE0_EN;
    pub use super::TRIG0_CHAIN_1_0::IE1;
    pub use super::TRIG0_CHAIN_1_0::IE1_EN;
}

/// ETC_TRIG Chain 2/3 Register
pub mod TRIG5_CHAIN_3_2 {
    pub use super::TRIG0_CHAIN_3_2::B2B2;
    pub use super::TRIG0_CHAIN_3_2::B2B3;
    pub use super::TRIG0_CHAIN_3_2::CSEL2;
    pub use super::TRIG0_CHAIN_3_2::CSEL3;
    pub use super::TRIG0_CHAIN_3_2::HWTS2;
    pub use super::TRIG0_CHAIN_3_2::HWTS3;
    pub use super::TRIG0_CHAIN_3_2::IE2;
    pub use super::TRIG0_CHAIN_3_2::IE2_EN;
    pub use super::TRIG0_CHAIN_3_2::IE3;
    pub use super::TRIG0_CHAIN_3_2::IE3_EN;
}

/// ETC_TRIG Chain 4/5 Register
pub mod TRIG5_CHAIN_5_4 {
    pub use super::TRIG0_CHAIN_5_4::B2B4;
    pub use super::TRIG0_CHAIN_5_4::B2B5;
    pub use super::TRIG0_CHAIN_5_4::CSEL4;
    pub use super::TRIG0_CHAIN_5_4::CSEL5;
    pub use super::TRIG0_CHAIN_5_4::HWTS4;
    pub use super::TRIG0_CHAIN_5_4::HWTS5;
    pub use super::TRIG0_CHAIN_5_4::IE4;
    pub use super::TRIG0_CHAIN_5_4::IE4_EN;
    pub use super::TRIG0_CHAIN_5_4::IE5;
    pub use super::TRIG0_CHAIN_5_4::IE5_EN;
}

/// ETC_TRIG Chain 6/7 Register
pub mod TRIG5_CHAIN_7_6 {
    pub use super::TRIG0_CHAIN_7_6::B2B6;
    pub use super::TRIG0_CHAIN_7_6::B2B7;
    pub use super::TRIG0_CHAIN_7_6::CSEL6;
    pub use super::TRIG0_CHAIN_7_6::CSEL7;
    pub use super::TRIG0_CHAIN_7_6::HWTS6;
    pub use super::TRIG0_CHAIN_7_6::HWTS7;
    pub use super::TRIG0_CHAIN_7_6::IE6;
    pub use super::TRIG0_CHAIN_7_6::IE6_EN;
    pub use super::TRIG0_CHAIN_7_6::IE7;
    pub use super::TRIG0_CHAIN_7_6::IE7_EN;
}

/// ETC_TRIG Result Data 1/0 Register
pub mod TRIG5_RESULT_1_0 {
    pub use super::TRIG0_RESULT_1_0::DATA0;
    pub use super::TRIG0_RESULT_1_0::DATA1;
}

/// ETC_TRIG Result Data 3/2 Register
pub mod TRIG5_RESULT_3_2 {
    pub use super::TRIG0_RESULT_3_2::DATA2;
    pub use super::TRIG0_RESULT_3_2::DATA3;
}

/// ETC_TRIG Result Data 5/4 Register
pub mod TRIG5_RESULT_5_4 {
    pub use super::TRIG0_RESULT_5_4::DATA4;
    pub use super::TRIG0_RESULT_5_4::DATA5;
}

/// ETC_TRIG Result Data 7/6 Register
pub mod TRIG5_RESULT_7_6 {
    pub use super::TRIG0_RESULT_7_6::DATA6;
    pub use super::TRIG0_RESULT_7_6::DATA7;
}

/// ETC_TRIG Control Register
pub mod TRIG6_CTRL {
    pub use super::TRIG0_CTRL::CHAINx_DONE;
    pub use super::TRIG0_CTRL::SW_TRIG;
    pub use super::TRIG0_CTRL::SYNC_MODE;
    pub use super::TRIG0_CTRL::TRIG_CHAIN;
    pub use super::TRIG0_CTRL::TRIG_MODE;
    pub use super::TRIG0_CTRL::TRIG_PRIORITY;
}

/// ETC_TRIG Counter Register
pub mod TRIG6_COUNTER {
    pub use super::TRIG0_COUNTER::INIT_DELAY;
    pub use super::TRIG0_COUNTER::SAMPLE_INTERVAL;
}

/// ETC_TRIG Chain 0/1 Register
pub mod TRIG6_CHAIN_1_0 {
    pub use super::TRIG0_CHAIN_1_0::B2B0;
    pub use super::TRIG0_CHAIN_1_0::B2B1;
    pub use super::TRIG0_CHAIN_1_0::CSEL0;
    pub use super::TRIG0_CHAIN_1_0::CSEL1;
    pub use super::TRIG0_CHAIN_1_0::HWTS0;
    pub use super::TRIG0_CHAIN_1_0::HWTS1;
    pub use super::TRIG0_CHAIN_1_0::IE0;
    pub use super::TRIG0_CHAIN_1_0::IE0_EN;
    pub use super::TRIG0_CHAIN_1_0::IE1;
    pub use super::TRIG0_CHAIN_1_0::IE1_EN;
}

/// ETC_TRIG Chain 2/3 Register
pub mod TRIG6_CHAIN_3_2 {
    pub use super::TRIG0_CHAIN_3_2::B2B2;
    pub use super::TRIG0_CHAIN_3_2::B2B3;
    pub use super::TRIG0_CHAIN_3_2::CSEL2;
    pub use super::TRIG0_CHAIN_3_2::CSEL3;
    pub use super::TRIG0_CHAIN_3_2::HWTS2;
    pub use super::TRIG0_CHAIN_3_2::HWTS3;
    pub use super::TRIG0_CHAIN_3_2::IE2;
    pub use super::TRIG0_CHAIN_3_2::IE2_EN;
    pub use super::TRIG0_CHAIN_3_2::IE3;
    pub use super::TRIG0_CHAIN_3_2::IE3_EN;
}

/// ETC_TRIG Chain 4/5 Register
pub mod TRIG6_CHAIN_5_4 {
    pub use super::TRIG0_CHAIN_5_4::B2B4;
    pub use super::TRIG0_CHAIN_5_4::B2B5;
    pub use super::TRIG0_CHAIN_5_4::CSEL4;
    pub use super::TRIG0_CHAIN_5_4::CSEL5;
    pub use super::TRIG0_CHAIN_5_4::HWTS4;
    pub use super::TRIG0_CHAIN_5_4::HWTS5;
    pub use super::TRIG0_CHAIN_5_4::IE4;
    pub use super::TRIG0_CHAIN_5_4::IE4_EN;
    pub use super::TRIG0_CHAIN_5_4::IE5;
    pub use super::TRIG0_CHAIN_5_4::IE5_EN;
}

/// ETC_TRIG Chain 6/7 Register
pub mod TRIG6_CHAIN_7_6 {
    pub use super::TRIG0_CHAIN_7_6::B2B6;
    pub use super::TRIG0_CHAIN_7_6::B2B7;
    pub use super::TRIG0_CHAIN_7_6::CSEL6;
    pub use super::TRIG0_CHAIN_7_6::CSEL7;
    pub use super::TRIG0_CHAIN_7_6::HWTS6;
    pub use super::TRIG0_CHAIN_7_6::HWTS7;
    pub use super::TRIG0_CHAIN_7_6::IE6;
    pub use super::TRIG0_CHAIN_7_6::IE6_EN;
    pub use super::TRIG0_CHAIN_7_6::IE7;
    pub use super::TRIG0_CHAIN_7_6::IE7_EN;
}

/// ETC_TRIG Result Data 1/0 Register
pub mod TRIG6_RESULT_1_0 {
    pub use super::TRIG0_RESULT_1_0::DATA0;
    pub use super::TRIG0_RESULT_1_0::DATA1;
}

/// ETC_TRIG Result Data 3/2 Register
pub mod TRIG6_RESULT_3_2 {
    pub use super::TRIG0_RESULT_3_2::DATA2;
    pub use super::TRIG0_RESULT_3_2::DATA3;
}

/// ETC_TRIG Result Data 5/4 Register
pub mod TRIG6_RESULT_5_4 {
    pub use super::TRIG0_RESULT_5_4::DATA4;
    pub use super::TRIG0_RESULT_5_4::DATA5;
}

/// ETC_TRIG Result Data 7/6 Register
pub mod TRIG6_RESULT_7_6 {
    pub use super::TRIG0_RESULT_7_6::DATA6;
    pub use super::TRIG0_RESULT_7_6::DATA7;
}

/// ETC_TRIG Control Register
pub mod TRIG7_CTRL {
    pub use super::TRIG0_CTRL::CHAINx_DONE;
    pub use super::TRIG0_CTRL::SW_TRIG;
    pub use super::TRIG0_CTRL::SYNC_MODE;
    pub use super::TRIG0_CTRL::TRIG_CHAIN;
    pub use super::TRIG0_CTRL::TRIG_MODE;
    pub use super::TRIG0_CTRL::TRIG_PRIORITY;
}

/// ETC_TRIG Counter Register
pub mod TRIG7_COUNTER {
    pub use super::TRIG0_COUNTER::INIT_DELAY;
    pub use super::TRIG0_COUNTER::SAMPLE_INTERVAL;
}

/// ETC_TRIG Chain 0/1 Register
pub mod TRIG7_CHAIN_1_0 {
    pub use super::TRIG0_CHAIN_1_0::B2B0;
    pub use super::TRIG0_CHAIN_1_0::B2B1;
    pub use super::TRIG0_CHAIN_1_0::CSEL0;
    pub use super::TRIG0_CHAIN_1_0::CSEL1;
    pub use super::TRIG0_CHAIN_1_0::HWTS0;
    pub use super::TRIG0_CHAIN_1_0::HWTS1;
    pub use super::TRIG0_CHAIN_1_0::IE0;
    pub use super::TRIG0_CHAIN_1_0::IE0_EN;
    pub use super::TRIG0_CHAIN_1_0::IE1;
    pub use super::TRIG0_CHAIN_1_0::IE1_EN;
}

/// ETC_TRIG Chain 2/3 Register
pub mod TRIG7_CHAIN_3_2 {
    pub use super::TRIG0_CHAIN_3_2::B2B2;
    pub use super::TRIG0_CHAIN_3_2::B2B3;
    pub use super::TRIG0_CHAIN_3_2::CSEL2;
    pub use super::TRIG0_CHAIN_3_2::CSEL3;
    pub use super::TRIG0_CHAIN_3_2::HWTS2;
    pub use super::TRIG0_CHAIN_3_2::HWTS3;
    pub use super::TRIG0_CHAIN_3_2::IE2;
    pub use super::TRIG0_CHAIN_3_2::IE2_EN;
    pub use super::TRIG0_CHAIN_3_2::IE3;
    pub use super::TRIG0_CHAIN_3_2::IE3_EN;
}

/// ETC_TRIG Chain 4/5 Register
pub mod TRIG7_CHAIN_5_4 {
    pub use super::TRIG0_CHAIN_5_4::B2B4;
    pub use super::TRIG0_CHAIN_5_4::B2B5;
    pub use super::TRIG0_CHAIN_5_4::CSEL4;
    pub use super::TRIG0_CHAIN_5_4::CSEL5;
    pub use super::TRIG0_CHAIN_5_4::HWTS4;
    pub use super::TRIG0_CHAIN_5_4::HWTS5;
    pub use super::TRIG0_CHAIN_5_4::IE4;
    pub use super::TRIG0_CHAIN_5_4::IE4_EN;
    pub use super::TRIG0_CHAIN_5_4::IE5;
    pub use super::TRIG0_CHAIN_5_4::IE5_EN;
}

/// ETC_TRIG Chain 6/7 Register
pub mod TRIG7_CHAIN_7_6 {
    pub use super::TRIG0_CHAIN_7_6::B2B6;
    pub use super::TRIG0_CHAIN_7_6::B2B7;
    pub use super::TRIG0_CHAIN_7_6::CSEL6;
    pub use super::TRIG0_CHAIN_7_6::CSEL7;
    pub use super::TRIG0_CHAIN_7_6::HWTS6;
    pub use super::TRIG0_CHAIN_7_6::HWTS7;
    pub use super::TRIG0_CHAIN_7_6::IE6;
    pub use super::TRIG0_CHAIN_7_6::IE6_EN;
    pub use super::TRIG0_CHAIN_7_6::IE7;
    pub use super::TRIG0_CHAIN_7_6::IE7_EN;
}

/// ETC_TRIG Result Data 1/0 Register
pub mod TRIG7_RESULT_1_0 {
    pub use super::TRIG0_RESULT_1_0::DATA0;
    pub use super::TRIG0_RESULT_1_0::DATA1;
}

/// ETC_TRIG Result Data 3/2 Register
pub mod TRIG7_RESULT_3_2 {
    pub use super::TRIG0_RESULT_3_2::DATA2;
    pub use super::TRIG0_RESULT_3_2::DATA3;
}

/// ETC_TRIG Result Data 5/4 Register
pub mod TRIG7_RESULT_5_4 {
    pub use super::TRIG0_RESULT_5_4::DATA4;
    pub use super::TRIG0_RESULT_5_4::DATA5;
}

/// ETC_TRIG Result Data 7/6 Register
pub mod TRIG7_RESULT_7_6 {
    pub use super::TRIG0_RESULT_7_6::DATA6;
    pub use super::TRIG0_RESULT_7_6::DATA7;
}
#[repr(C)]
pub struct RegisterBlock {
    /// ADC_ETC Global Control Register
    pub CTRL: RWRegister<u32>,

    /// ETC DONE0 and DONE1 IRQ State Register
    pub DONE0_1_IRQ: RWRegister<u32>,

    /// ETC DONE_2, DONE_3 and DONE_ERR IRQ State Register
    pub DONE2_3_ERR_IRQ: RWRegister<u32>,

    /// ETC DMA control Register
    pub DMA_CTRL: RWRegister<u32>,

    /// ETC_TRIG Control Register
    pub TRIG0_CTRL: RWRegister<u32>,

    /// ETC_TRIG Counter Register
    pub TRIG0_COUNTER: RWRegister<u32>,

    /// ETC_TRIG Chain 0/1 Register
    pub TRIG0_CHAIN_1_0: RWRegister<u32>,

    /// ETC_TRIG Chain 2/3 Register
    pub TRIG0_CHAIN_3_2: RWRegister<u32>,

    /// ETC_TRIG Chain 4/5 Register
    pub TRIG0_CHAIN_5_4: RWRegister<u32>,

    /// ETC_TRIG Chain 6/7 Register
    pub TRIG0_CHAIN_7_6: RWRegister<u32>,

    /// ETC_TRIG Result Data 1/0 Register
    pub TRIG0_RESULT_1_0: RORegister<u32>,

    /// ETC_TRIG Result Data 3/2 Register
    pub TRIG0_RESULT_3_2: RORegister<u32>,

    /// ETC_TRIG Result Data 5/4 Register
    pub TRIG0_RESULT_5_4: RORegister<u32>,

    /// ETC_TRIG Result Data 7/6 Register
    pub TRIG0_RESULT_7_6: RORegister<u32>,

    /// ETC_TRIG Control Register
    pub TRIG1_CTRL: RWRegister<u32>,

    /// ETC_TRIG Counter Register
    pub TRIG1_COUNTER: RWRegister<u32>,

    /// ETC_TRIG Chain 0/1 Register
    pub TRIG1_CHAIN_1_0: RWRegister<u32>,

    /// ETC_TRIG Chain 2/3 Register
    pub TRIG1_CHAIN_3_2: RWRegister<u32>,

    /// ETC_TRIG Chain 4/5 Register
    pub TRIG1_CHAIN_5_4: RWRegister<u32>,

    /// ETC_TRIG Chain 6/7 Register
    pub TRIG1_CHAIN_7_6: RWRegister<u32>,

    /// ETC_TRIG Result Data 1/0 Register
    pub TRIG1_RESULT_1_0: RORegister<u32>,

    /// ETC_TRIG Result Data 3/2 Register
    pub TRIG1_RESULT_3_2: RORegister<u32>,

    /// ETC_TRIG Result Data 5/4 Register
    pub TRIG1_RESULT_5_4: RORegister<u32>,

    /// ETC_TRIG Result Data 7/6 Register
    pub TRIG1_RESULT_7_6: RORegister<u32>,

    /// ETC_TRIG Control Register
    pub TRIG2_CTRL: RWRegister<u32>,

    /// ETC_TRIG Counter Register
    pub TRIG2_COUNTER: RWRegister<u32>,

    /// ETC_TRIG Chain 0/1 Register
    pub TRIG2_CHAIN_1_0: RWRegister<u32>,

    /// ETC_TRIG Chain 2/3 Register
    pub TRIG2_CHAIN_3_2: RWRegister<u32>,

    /// ETC_TRIG Chain 4/5 Register
    pub TRIG2_CHAIN_5_4: RWRegister<u32>,

    /// ETC_TRIG Chain 6/7 Register
    pub TRIG2_CHAIN_7_6: RWRegister<u32>,

    /// ETC_TRIG Result Data 1/0 Register
    pub TRIG2_RESULT_1_0: RORegister<u32>,

    /// ETC_TRIG Result Data 3/2 Register
    pub TRIG2_RESULT_3_2: RORegister<u32>,

    /// ETC_TRIG Result Data 5/4 Register
    pub TRIG2_RESULT_5_4: RORegister<u32>,

    /// ETC_TRIG Result Data 7/6 Register
    pub TRIG2_RESULT_7_6: RORegister<u32>,

    /// ETC_TRIG Control Register
    pub TRIG3_CTRL: RWRegister<u32>,

    /// ETC_TRIG Counter Register
    pub TRIG3_COUNTER: RWRegister<u32>,

    /// ETC_TRIG Chain 0/1 Register
    pub TRIG3_CHAIN_1_0: RWRegister<u32>,

    /// ETC_TRIG Chain 2/3 Register
    pub TRIG3_CHAIN_3_2: RWRegister<u32>,

    /// ETC_TRIG Chain 4/5 Register
    pub TRIG3_CHAIN_5_4: RWRegister<u32>,

    /// ETC_TRIG Chain 6/7 Register
    pub TRIG3_CHAIN_7_6: RWRegister<u32>,

    /// ETC_TRIG Result Data 1/0 Register
    pub TRIG3_RESULT_1_0: RORegister<u32>,

    /// ETC_TRIG Result Data 3/2 Register
    pub TRIG3_RESULT_3_2: RORegister<u32>,

    /// ETC_TRIG Result Data 5/4 Register
    pub TRIG3_RESULT_5_4: RORegister<u32>,

    /// ETC_TRIG Result Data 7/6 Register
    pub TRIG3_RESULT_7_6: RORegister<u32>,

    /// ETC_TRIG Control Register
    pub TRIG4_CTRL: RWRegister<u32>,

    /// ETC_TRIG Counter Register
    pub TRIG4_COUNTER: RWRegister<u32>,

    /// ETC_TRIG Chain 0/1 Register
    pub TRIG4_CHAIN_1_0: RWRegister<u32>,

    /// ETC_TRIG Chain 2/3 Register
    pub TRIG4_CHAIN_3_2: RWRegister<u32>,

    /// ETC_TRIG Chain 4/5 Register
    pub TRIG4_CHAIN_5_4: RWRegister<u32>,

    /// ETC_TRIG Chain 6/7 Register
    pub TRIG4_CHAIN_7_6: RWRegister<u32>,

    /// ETC_TRIG Result Data 1/0 Register
    pub TRIG4_RESULT_1_0: RORegister<u32>,

    /// ETC_TRIG Result Data 3/2 Register
    pub TRIG4_RESULT_3_2: RORegister<u32>,

    /// ETC_TRIG Result Data 5/4 Register
    pub TRIG4_RESULT_5_4: RORegister<u32>,

    /// ETC_TRIG Result Data 7/6 Register
    pub TRIG4_RESULT_7_6: RORegister<u32>,

    /// ETC_TRIG Control Register
    pub TRIG5_CTRL: RWRegister<u32>,

    /// ETC_TRIG Counter Register
    pub TRIG5_COUNTER: RWRegister<u32>,

    /// ETC_TRIG Chain 0/1 Register
    pub TRIG5_CHAIN_1_0: RWRegister<u32>,

    /// ETC_TRIG Chain 2/3 Register
    pub TRIG5_CHAIN_3_2: RWRegister<u32>,

    /// ETC_TRIG Chain 4/5 Register
    pub TRIG5_CHAIN_5_4: RWRegister<u32>,

    /// ETC_TRIG Chain 6/7 Register
    pub TRIG5_CHAIN_7_6: RWRegister<u32>,

    /// ETC_TRIG Result Data 1/0 Register
    pub TRIG5_RESULT_1_0: RORegister<u32>,

    /// ETC_TRIG Result Data 3/2 Register
    pub TRIG5_RESULT_3_2: RORegister<u32>,

    /// ETC_TRIG Result Data 5/4 Register
    pub TRIG5_RESULT_5_4: RORegister<u32>,

    /// ETC_TRIG Result Data 7/6 Register
    pub TRIG5_RESULT_7_6: RORegister<u32>,

    /// ETC_TRIG Control Register
    pub TRIG6_CTRL: RWRegister<u32>,

    /// ETC_TRIG Counter Register
    pub TRIG6_COUNTER: RWRegister<u32>,

    /// ETC_TRIG Chain 0/1 Register
    pub TRIG6_CHAIN_1_0: RWRegister<u32>,

    /// ETC_TRIG Chain 2/3 Register
    pub TRIG6_CHAIN_3_2: RWRegister<u32>,

    /// ETC_TRIG Chain 4/5 Register
    pub TRIG6_CHAIN_5_4: RWRegister<u32>,

    /// ETC_TRIG Chain 6/7 Register
    pub TRIG6_CHAIN_7_6: RWRegister<u32>,

    /// ETC_TRIG Result Data 1/0 Register
    pub TRIG6_RESULT_1_0: RORegister<u32>,

    /// ETC_TRIG Result Data 3/2 Register
    pub TRIG6_RESULT_3_2: RORegister<u32>,

    /// ETC_TRIG Result Data 5/4 Register
    pub TRIG6_RESULT_5_4: RORegister<u32>,

    /// ETC_TRIG Result Data 7/6 Register
    pub TRIG6_RESULT_7_6: RORegister<u32>,

    /// ETC_TRIG Control Register
    pub TRIG7_CTRL: RWRegister<u32>,

    /// ETC_TRIG Counter Register
    pub TRIG7_COUNTER: RWRegister<u32>,

    /// ETC_TRIG Chain 0/1 Register
    pub TRIG7_CHAIN_1_0: RWRegister<u32>,

    /// ETC_TRIG Chain 2/3 Register
    pub TRIG7_CHAIN_3_2: RWRegister<u32>,

    /// ETC_TRIG Chain 4/5 Register
    pub TRIG7_CHAIN_5_4: RWRegister<u32>,

    /// ETC_TRIG Chain 6/7 Register
    pub TRIG7_CHAIN_7_6: RWRegister<u32>,

    /// ETC_TRIG Result Data 1/0 Register
    pub TRIG7_RESULT_1_0: RORegister<u32>,

    /// ETC_TRIG Result Data 3/2 Register
    pub TRIG7_RESULT_3_2: RORegister<u32>,

    /// ETC_TRIG Result Data 5/4 Register
    pub TRIG7_RESULT_5_4: RORegister<u32>,

    /// ETC_TRIG Result Data 7/6 Register
    pub TRIG7_RESULT_7_6: RORegister<u32>,
}
pub struct ResetValues {
    pub CTRL: u32,
    pub DONE0_1_IRQ: u32,
    pub DONE2_3_ERR_IRQ: u32,
    pub DMA_CTRL: u32,
    pub TRIG0_CTRL: u32,
    pub TRIG0_COUNTER: u32,
    pub TRIG0_CHAIN_1_0: u32,
    pub TRIG0_CHAIN_3_2: u32,
    pub TRIG0_CHAIN_5_4: u32,
    pub TRIG0_CHAIN_7_6: u32,
    pub TRIG0_RESULT_1_0: u32,
    pub TRIG0_RESULT_3_2: u32,
    pub TRIG0_RESULT_5_4: u32,
    pub TRIG0_RESULT_7_6: u32,
    pub TRIG1_CTRL: u32,
    pub TRIG1_COUNTER: u32,
    pub TRIG1_CHAIN_1_0: u32,
    pub TRIG1_CHAIN_3_2: u32,
    pub TRIG1_CHAIN_5_4: u32,
    pub TRIG1_CHAIN_7_6: u32,
    pub TRIG1_RESULT_1_0: u32,
    pub TRIG1_RESULT_3_2: u32,
    pub TRIG1_RESULT_5_4: u32,
    pub TRIG1_RESULT_7_6: u32,
    pub TRIG2_CTRL: u32,
    pub TRIG2_COUNTER: u32,
    pub TRIG2_CHAIN_1_0: u32,
    pub TRIG2_CHAIN_3_2: u32,
    pub TRIG2_CHAIN_5_4: u32,
    pub TRIG2_CHAIN_7_6: u32,
    pub TRIG2_RESULT_1_0: u32,
    pub TRIG2_RESULT_3_2: u32,
    pub TRIG2_RESULT_5_4: u32,
    pub TRIG2_RESULT_7_6: u32,
    pub TRIG3_CTRL: u32,
    pub TRIG3_COUNTER: u32,
    pub TRIG3_CHAIN_1_0: u32,
    pub TRIG3_CHAIN_3_2: u32,
    pub TRIG3_CHAIN_5_4: u32,
    pub TRIG3_CHAIN_7_6: u32,
    pub TRIG3_RESULT_1_0: u32,
    pub TRIG3_RESULT_3_2: u32,
    pub TRIG3_RESULT_5_4: u32,
    pub TRIG3_RESULT_7_6: u32,
    pub TRIG4_CTRL: u32,
    pub TRIG4_COUNTER: u32,
    pub TRIG4_CHAIN_1_0: u32,
    pub TRIG4_CHAIN_3_2: u32,
    pub TRIG4_CHAIN_5_4: u32,
    pub TRIG4_CHAIN_7_6: u32,
    pub TRIG4_RESULT_1_0: u32,
    pub TRIG4_RESULT_3_2: u32,
    pub TRIG4_RESULT_5_4: u32,
    pub TRIG4_RESULT_7_6: u32,
    pub TRIG5_CTRL: u32,
    pub TRIG5_COUNTER: u32,
    pub TRIG5_CHAIN_1_0: u32,
    pub TRIG5_CHAIN_3_2: u32,
    pub TRIG5_CHAIN_5_4: u32,
    pub TRIG5_CHAIN_7_6: u32,
    pub TRIG5_RESULT_1_0: u32,
    pub TRIG5_RESULT_3_2: u32,
    pub TRIG5_RESULT_5_4: u32,
    pub TRIG5_RESULT_7_6: u32,
    pub TRIG6_CTRL: u32,
    pub TRIG6_COUNTER: u32,
    pub TRIG6_CHAIN_1_0: u32,
    pub TRIG6_CHAIN_3_2: u32,
    pub TRIG6_CHAIN_5_4: u32,
    pub TRIG6_CHAIN_7_6: u32,
    pub TRIG6_RESULT_1_0: u32,
    pub TRIG6_RESULT_3_2: u32,
    pub TRIG6_RESULT_5_4: u32,
    pub TRIG6_RESULT_7_6: u32,
    pub TRIG7_CTRL: u32,
    pub TRIG7_COUNTER: u32,
    pub TRIG7_CHAIN_1_0: u32,
    pub TRIG7_CHAIN_3_2: u32,
    pub TRIG7_CHAIN_5_4: u32,
    pub TRIG7_CHAIN_7_6: u32,
    pub TRIG7_RESULT_1_0: u32,
    pub TRIG7_RESULT_3_2: u32,
    pub TRIG7_RESULT_5_4: u32,
    pub TRIG7_RESULT_7_6: u32,
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
