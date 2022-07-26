#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PDM
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// PDM Control register 1
pub mod CTRL_1 {

    /// Channel 0 Enable
    pub mod CH0EN {
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

    /// Channel 1 Enable
    pub mod CH1EN {
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

    /// Channel 2 Enable
    pub mod CH2EN {
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

    /// Channel 3 Enable
    pub mod CH3EN {
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

    /// Channel 4 Enable
    pub mod CH4EN {
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

    /// Channel 5 Enable
    pub mod CH5EN {
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

    /// Channel 6 Enable
    pub mod CH6EN {
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

    /// Channel 7 Enable
    pub mod CH7EN {
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

    /// Error Interruption Enable
    pub mod ERREN {
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

            /// 0b0: Error Interrupts disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: Error Interrupts enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// DMA Interrupt Selection
    pub mod DISEL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: DMA and interrupt requests disabled
            pub const all_disabled: u32 = 0b00;

            /// 0b01: DMA requests enabled
            pub const dmareq_enabled: u32 = 0b01;

            /// 0b10: Interrupt requests enabled
            pub const intreq_enabled: u32 = 0b10;
        }
    }

    /// Module Enable in Debug
    pub mod DBGE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled after completing the current frame
            pub const disabled: u32 = 0b0;

            /// 0b1: Enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Software-reset bit
    pub mod SRES {
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

            /// 0b0: No action
            pub const no_action: u32 = 0b0;

            /// 0b1: Software reset
            pub const sw_reset: u32 = 0b1;
        }
    }

    /// Debug Mode
    pub mod DBG {
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

            /// 0b0: Normal Mode
            pub const normal: u32 = 0b0;

            /// 0b1: Debug Mode
            pub const debug: u32 = 0b1;
        }
    }

    /// PDM Enable
    pub mod PDMIEN {
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

            /// 0b0: PDM stopped
            pub const stopped: u32 = 0b0;

            /// 0b1: PDM operation started
            pub const started: u32 = 0b1;
        }
    }

    /// DOZE enable
    pub mod DOZEN {
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

    /// Module Disable
    pub mod MDIS {
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

            /// 0b0: Normal Mode
            pub const normal: u32 = 0b0;

            /// 0b1: Disable/Low Leakage Mode
            pub const low_leakage: u32 = 0b1;
        }
    }
}

/// PDM Control register 2
pub mod CTRL_2 {

    /// Clock Divider
    pub mod CLKDIV {
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

    /// CIC Decimation Rate
    pub mod CICOSR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Quality Mode
    pub mod QSEL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (3 bits: 0b111 << 25)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Medium quality mode
            pub const mq_mode: u32 = 0b000;

            /// 0b001: High quality mode
            pub const hq_mode: u32 = 0b001;

            /// 0b100: Very low quality 2 mode
            pub const vlq2_mode: u32 = 0b100;

            /// 0b101: Very low quality 1 mode
            pub const vlq1_mode: u32 = 0b101;

            /// 0b110: Very low quality 0 mode
            pub const vlq0_mode: u32 = 0b110;

            /// 0b111: Low quality mode
            pub const lq_mode: u32 = 0b111;
        }
    }
}

/// PDM Status register
pub mod STAT {

    /// Channel 0 Output Data Flag
    pub mod CH0F {
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

            /// 0b0: Channel's FIFO did not reach the number of elements configured in watermark bit-field
            pub const wm_notreached: u32 = 0b0;

            /// 0b1: Channel's FIFO reached the number of elements configured in watermark bit-field
            pub const wm_reached: u32 = 0b1;
        }
    }

    /// Channel 1 Output Data Flag
    pub mod CH1F {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH0F::RW;
    }

    /// Channel 2 Output Data Flag
    pub mod CH2F {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH0F::RW;
    }

    /// Channel 3 Output Data Flag
    pub mod CH3F {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH0F::RW;
    }

    /// Channel 4 Output Data Flag
    pub mod CH4F {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH0F::RW;
    }

    /// Channel 5 Output Data Flag
    pub mod CH5F {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH0F::RW;
    }

    /// Channel 6 Output Data Flag
    pub mod CH6F {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH0F::RW;
    }

    /// Channel 7 Output Data Flag
    pub mod CH7F {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH0F::RW;
    }

    /// Low Frequency Flag
    pub mod LOWFREQF {
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

            /// 0b0: CLKDIV value is OK
            pub const clkdiv_ok: u32 = 0b0;

            /// 0b1: CLKDIV value is too low
            pub const clkdiv_low: u32 = 0b1;
        }
    }

    /// Filter Data Ready
    pub mod FIR_RDY {
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

            /// 0b0: Filter data is not reliable
            pub const not_reliable: u32 = 0b0;

            /// 0b1: Filter data is reliable
            pub const reliable: u32 = 0b1;
        }
    }

    /// Busy Flag
    pub mod BSY_FIL {
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

            /// 0b0: PDM is stopped
            pub const stopped: u32 = 0b0;

            /// 0b1: PDM is running
            pub const running: u32 = 0b1;
        }
    }
}

/// PDM FIFO Control register
pub mod FIFO_CTRL {

    /// FIFO Watermark Control
    pub mod FIFOWMK {
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

/// PDM FIFO Status register
pub mod FIFO_STAT {

    /// FIFO Overflow Exception flag for Channel 0
    pub mod FIFOOVF0 {
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

            /// 0b0: No exception by FIFO overflow
            pub const no_exception: u32 = 0b0;

            /// 0b1: Exception by FIFO overflow
            pub const exception: u32 = 0b1;
        }
    }

    /// FIFO Overflow Exception flag for Channel 1
    pub mod FIFOOVF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOOVF0::RW;
    }

    /// FIFO Overflow Exception flag for Channel 2
    pub mod FIFOOVF2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOOVF0::RW;
    }

    /// FIFO Overflow Exception flag for Channel 3
    pub mod FIFOOVF3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOOVF0::RW;
    }

    /// FIFO Overflow Exception flag for Channel 4
    pub mod FIFOOVF4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOOVF0::RW;
    }

    /// FIFO Overflow Exception flag for Channel 5
    pub mod FIFOOVF5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOOVF0::RW;
    }

    /// FIFO Overflow Exception flag for Channel 6
    pub mod FIFOOVF6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOOVF0::RW;
    }

    /// FIFO Overflow Exception flag for Channel 7
    pub mod FIFOOVF7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOOVF0::RW;
    }

    /// FIFO Underflow Exception flag for Channel 0
    pub mod FIFOUND0 {
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

            /// 0b0: No exception by FIFO Underflow
            pub const no_exception: u32 = 0b0;

            /// 0b1: Exception by FIFO underflow
            pub const exception: u32 = 0b1;
        }
    }

    /// FIFO Underflow Exception flag for Channel 1
    pub mod FIFOUND1 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOUND0::RW;
    }

    /// FIFO Underflow Exception flag for Channel 2
    pub mod FIFOUND2 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOUND0::RW;
    }

    /// FIFO Underflow Exception flag for Channel 3
    pub mod FIFOUND3 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOUND0::RW;
    }

    /// FIFO Underflow Exception flag for Channel 4
    pub mod FIFOUND4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOUND0::RW;
    }

    /// FIFO Underflow Exception flag for Channel 5
    pub mod FIFOUND5 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOUND0::RW;
    }

    /// FIFO Underflow Exception flag for Channel 6
    pub mod FIFOUND6 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOUND0::RW;
    }

    /// FIFO Underflow Exception flag for Channel 7
    pub mod FIFOUND7 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FIFOUND0::RW;
    }
}

/// PDM Output Result Register
pub mod DATACH0 {

    /// Channel n Data
    pub mod DATA {
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

/// PDM Output Result Register
pub mod DATACH1 {
    pub use super::DATACH0::DATA;
}

/// PDM Output Result Register
pub mod DATACH2 {
    pub use super::DATACH0::DATA;
}

/// PDM Output Result Register
pub mod DATACH3 {
    pub use super::DATACH0::DATA;
}

/// PDM Output Result Register
pub mod DATACH4 {
    pub use super::DATACH0::DATA;
}

/// PDM Output Result Register
pub mod DATACH5 {
    pub use super::DATACH0::DATA;
}

/// PDM Output Result Register
pub mod DATACH6 {
    pub use super::DATACH0::DATA;
}

/// PDM Output Result Register
pub mod DATACH7 {
    pub use super::DATACH0::DATA;
}

/// PDM DC Remover Control register
pub mod DC_CTRL {

    /// Channel 0 DC Remover Configuration
    pub mod DCCONFIG0 {
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

            /// 0b00: DC Remover cut-off at 21Hz
            pub const dc_rem_21Hz: u32 = 0b00;

            /// 0b01: DC Remover cut-off at 83Hz
            pub const dc_rem_83Hz: u32 = 0b01;

            /// 0b10: DC Remover cut-off at 152Hz
            pub const dc_rem_152Hz: u32 = 0b10;

            /// 0b11: DC Remover is bypassed
            pub const dc_rem_bypass: u32 = 0b11;
        }
    }

    /// Channel 1 DC Remover Configuration
    pub mod DCCONFIG1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCCONFIG0::RW;
    }

    /// Channel 2 DC Remover Configuration
    pub mod DCCONFIG2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCCONFIG0::RW;
    }

    /// Channel 3 DC Remover Configuration
    pub mod DCCONFIG3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCCONFIG0::RW;
    }

    /// Channel 4 DC Remover Configuration
    pub mod DCCONFIG4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCCONFIG0::RW;
    }

    /// Channel 5 DC Remover Configuration
    pub mod DCCONFIG5 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCCONFIG0::RW;
    }

    /// Channel 6 DC Remover Configuration
    pub mod DCCONFIG6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCCONFIG0::RW;
    }

    /// Channel 7 DC Remover Configuration
    pub mod DCCONFIG7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DCCONFIG0::RW;
    }
}

/// PDM Range Control register
pub mod RANGE_CTRL {

    /// Channel 0 Range Adjustment
    pub mod RANGEADJ0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel 1 Range Adjustment
    pub mod RANGEADJ1 {
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

    /// Channel 2 Range Adjustment
    pub mod RANGEADJ2 {
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

    /// Channel 3 Range Adjustment
    pub mod RANGEADJ3 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel 4 Range Adjustment
    pub mod RANGEADJ4 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel 5 Range Adjustment
    pub mod RANGEADJ5 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel 6 Range Adjustment
    pub mod RANGEADJ6 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel 7 Range Adjustment
    pub mod RANGEADJ7 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PDM Range Status register
pub mod RANGE_STAT {

    /// Channel 0 Range Overflow Error Flag
    pub mod RANGEOVF0 {
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

            /// 0b0: No exception by range overflow
            pub const no_exception: u32 = 0b0;

            /// 0b1: Exception by range overflow
            pub const exception: u32 = 0b1;
        }
    }

    /// Channel 1 Range Overflow Error Flag
    pub mod RANGEOVF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEOVF0::RW;
    }

    /// Channel 2 Range Overflow Error Flag
    pub mod RANGEOVF2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEOVF0::RW;
    }

    /// Channel 3 Range Overflow Error Flag
    pub mod RANGEOVF3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEOVF0::RW;
    }

    /// Channel 4 Range Overflow Error Flag
    pub mod RANGEOVF4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEOVF0::RW;
    }

    /// Channel 5 Range Overflow Error Flag
    pub mod RANGEOVF5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEOVF0::RW;
    }

    /// Channel 6 Range Overflow Error Flag
    pub mod RANGEOVF6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEOVF0::RW;
    }

    /// Channel 7 Range Overflow Error Flag
    pub mod RANGEOVF7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEOVF0::RW;
    }

    /// Channel 0 Range Underflow Error Flag
    pub mod RANGEUNF0 {
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

            /// 0b0: No exception by range underflow
            pub const no_exception: u32 = 0b0;

            /// 0b1: Exception by range underflow
            pub const exception: u32 = 0b1;
        }
    }

    /// Channel 1 Range Underflow Error Flag
    pub mod RANGEUNF1 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEUNF0::RW;
    }

    /// Channel 2 Range Underflow Error Flag
    pub mod RANGEUNF2 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEUNF0::RW;
    }

    /// Channel 3 Range Underflow Error Flag
    pub mod RANGEUNF3 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEUNF0::RW;
    }

    /// Channel 4 Range Underflow Error Flag
    pub mod RANGEUNF4 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEUNF0::RW;
    }

    /// Channel 5 Range Underflow Error Flag
    pub mod RANGEUNF5 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEUNF0::RW;
    }

    /// Channel 6 Range Underflow Error Flag
    pub mod RANGEUNF6 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEUNF0::RW;
    }

    /// Channel 7 Range Underflow Error Flag
    pub mod RANGEUNF7 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RANGEUNF0::RW;
    }
}

/// Voice Activity Detector 0 Control register
pub mod VAD0_CTRL_1 {

    /// Voice Activity Detector Enable
    pub mod VADEN {
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

            /// 0b0: The HWVAD is disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: The HWVAD is enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Voice Activity Detector Reset
    pub mod VADRST {
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

    /// Voice Activity Detector Interruption Enable
    pub mod VADIE {
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

            /// 0b0: HWVAD Interrupts disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: HWVAD Interrupts enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Voice Activity Detector Error Interruption Enable
    pub mod VADERIE {
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

            /// 0b0: HWVAD Error Interrupts disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: HWVAD Error Interrupts enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Voice Activity Detector Internal Filters Initialization
    pub mod VADST10 {
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

            /// 0b0: Normal operation.
            pub const normal_op: u32 = 0b0;

            /// 0b1: Filters are initialized.
            pub const filt_init: u32 = 0b1;
        }
    }

    /// Voice Activity Detector Initialization Time
    pub mod VADINITT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Voice Activity Detector CIC Oversampling Rate
    pub mod VADCICOSR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Voice Activity Detector Channel Selector
    pub mod VADCHSEL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Voice Activity Detector 0 Control register
pub mod VAD0_CTRL_2 {

    /// Voice Activity Detector High-Pass Filter
    pub mod VADHPF {
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

            /// 0b00: Filter bypassed.
            pub const filt_bypass: u32 = 0b00;

            /// 0b01: Cut-off frequency at 1750Hz.
            pub const cutoff_1750Hz: u32 = 0b01;

            /// 0b10: Cut-off frequency at 215Hz.
            pub const cutoff_215Hz: u32 = 0b10;

            /// 0b11: Cut-off frequency at 102Hz.
            pub const cutoff_102Hz: u32 = 0b11;
        }
    }

    /// Voice Activity Detector Input Gain
    pub mod VADINPGAIN {
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

    /// Voice Activity Detector Frame Time
    pub mod VADFRAMET {
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

    /// Voice Activity Detector Force Output Disable
    pub mod VADFOUTDIS {
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

            /// 0b0: Output is enabled.
            pub const out_enabled: u32 = 0b0;

            /// 0b1: Output is disabled.
            pub const out_disabled: u32 = 0b1;
        }
    }

    /// Voice Activity Detector Pre Filter Enable
    pub mod VADPREFEN {
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

            /// 0b0: Pre-filter is bypassed.
            pub const bypassed: u32 = 0b0;

            /// 0b1: Pre-filter is enabled.
            pub const enabled: u32 = 0b1;
        }
    }

    /// Voice Activity Detector Frame Energy Disable
    pub mod VADFRENDIS {
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

            /// 0b0: Frame energy calculus enabled.
            pub const enabled: u32 = 0b0;

            /// 0b1: Frame energy calculus disabled.
            pub const disabled: u32 = 0b1;
        }
    }
}

/// Voice Activity Detector 0 Status register
pub mod VAD0_STAT {

    /// Voice Activity Detector Interrupt Flag
    pub mod VADIF {
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

            /// 0b0: Voice activity not detected
            pub const no_detect: u32 = 0b0;

            /// 0b1: Voice activity detected
            pub const detect: u32 = 0b1;
        }
    }

    /// Voice Activity Detector Event Flag
    pub mod VADEF {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::VADIF::RW;
    }

    /// Voice Activity Detector Input Saturation Flag
    pub mod VADINSATF {
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

            /// 0b0: No exception
            pub const no_exception: u32 = 0b0;

            /// 0b1: Exception
            pub const exception: u32 = 0b1;
        }
    }

    /// Voice Activity Detector Initialization Flag
    pub mod VADINITF {
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

            /// 0b0: HWVAD is not being initialized.
            pub const not_init: u32 = 0b0;

            /// 0b1: HWVAD is being initialized.
            pub const init: u32 = 0b1;
        }
    }
}

/// Voice Activity Detector 0 Signal Configuration
pub mod VAD0_SCONFIG {

    /// Voice Activity Detector Signal Gain
    pub mod VADSGAIN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Voice Activity Detector Signal Maximum Enable
    pub mod VADSMAXEN {
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

            /// 0b0: Maximum block is bypassed.
            pub const bypassed: u32 = 0b0;

            /// 0b1: Maximum block is enabled.
            pub const enabled: u32 = 0b1;
        }
    }

    /// Voice Activity Detector Signal Filter Enable
    pub mod VADSFILEN {
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

            /// 0b0: Signal filter is disabled.
            pub const disabled: u32 = 0b0;

            /// 0b1: Signal filter is enabled.
            pub const enabled: u32 = 0b1;
        }
    }
}

/// Voice Activity Detector 0 Noise Configuration
pub mod VAD0_NCONFIG {

    /// Voice Activity Detector Noise Gain
    pub mod VADNGAIN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Voice Activity Detector Noise Filter Adjustment
    pub mod VADNFILADJ {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Voice Activity Detector Noise OR Enable
    pub mod VADNOREN {
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

            /// 0b0: Noise input is not decimated.
            pub const not_decimated: u32 = 0b0;

            /// 0b1: Noise input is decimated.
            pub const decimated: u32 = 0b1;
        }
    }

    /// Voice Activity Detector Noise Decimation Enable
    pub mod VADNDECEN {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::VADNOREN::RW;
    }

    /// Voice Activity Detector Noise Minimum Enable
    pub mod VADNMINEN {
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

            /// 0b0: Minimum block is bypassed.
            pub const bypassed: u32 = 0b0;

            /// 0b1: Minimum block is enabled.
            pub const enabled: u32 = 0b1;
        }
    }

    /// Voice Activity Detector Noise Filter Auto
    pub mod VADNFILAUTO {
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

            /// 0b0: Noise filter is always enabled.
            pub const nf_always_en: u32 = 0b0;

            /// 0b1: Noise filter is enabled/disabled based on voice activity information.
            pub const nf_cond_en: u32 = 0b1;
        }
    }
}

/// Voice Activity Detector 0 Noise Data
pub mod VAD0_NDATA {

    /// Voice Activity Detector Noise Data
    pub mod VADNDATA {
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
}

/// Voice Activity Detector 0 Zero-Crossing Detector
pub mod VAD0_ZCD {

    /// Zero-Crossing Detector Enable
    pub mod VADZCDEN {
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

            /// 0b0: The ZCD is disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: The ZCD is enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Zero-Crossing Detector Automatic Threshold
    pub mod VADZCDAUTO {
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

            /// 0b0: The ZCD threshold is not estimated automatically
            pub const not_estimated: u32 = 0b0;

            /// 0b1: The ZCD threshold is estimated automatically
            pub const estimated: u32 = 0b1;
        }
    }

    /// Zero-Crossing Detector AND Behavior
    pub mod VADZCDAND {
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

            /// 0b0: The ZCD result is OR'ed with the energy-based detection.
            pub const ored: u32 = 0b0;

            /// 0b1: The ZCD result is AND'ed with the energy-based detection.
            pub const anded: u32 = 0b1;
        }
    }

    /// Zero-Crossing Detector Adjustment
    pub mod VADZCDADJ {
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

    /// Zero-Crossing Detector Threshold
    pub mod VADZCDTH {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (10 bits: 0x3ff << 16)
        pub const mask: u32 = 0x3ff << offset;
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
    /// PDM Control register 1
    pub CTRL_1: RWRegister<u32>,

    /// PDM Control register 2
    pub CTRL_2: RWRegister<u32>,

    /// PDM Status register
    pub STAT: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// PDM FIFO Control register
    pub FIFO_CTRL: RWRegister<u32>,

    /// PDM FIFO Status register
    pub FIFO_STAT: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// PDM Output Result Register
    pub DATACH0: RORegister<u32>,

    /// PDM Output Result Register
    pub DATACH1: RORegister<u32>,

    /// PDM Output Result Register
    pub DATACH2: RORegister<u32>,

    /// PDM Output Result Register
    pub DATACH3: RORegister<u32>,

    /// PDM Output Result Register
    pub DATACH4: RORegister<u32>,

    /// PDM Output Result Register
    pub DATACH5: RORegister<u32>,

    /// PDM Output Result Register
    pub DATACH6: RORegister<u32>,

    /// PDM Output Result Register
    pub DATACH7: RORegister<u32>,

    _reserved3: [u32; 8],

    /// PDM DC Remover Control register
    pub DC_CTRL: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// PDM Range Control register
    pub RANGE_CTRL: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// PDM Range Status register
    pub RANGE_STAT: RWRegister<u32>,

    _reserved6: [u32; 4],

    /// Voice Activity Detector 0 Control register
    pub VAD0_CTRL_1: RWRegister<u32>,

    /// Voice Activity Detector 0 Control register
    pub VAD0_CTRL_2: RWRegister<u32>,

    /// Voice Activity Detector 0 Status register
    pub VAD0_STAT: RWRegister<u32>,

    /// Voice Activity Detector 0 Signal Configuration
    pub VAD0_SCONFIG: RWRegister<u32>,

    /// Voice Activity Detector 0 Noise Configuration
    pub VAD0_NCONFIG: RWRegister<u32>,

    /// Voice Activity Detector 0 Noise Data
    pub VAD0_NDATA: RORegister<u32>,

    /// Voice Activity Detector 0 Zero-Crossing Detector
    pub VAD0_ZCD: RWRegister<u32>,
}
pub struct ResetValues {
    pub CTRL_1: u32,
    pub CTRL_2: u32,
    pub STAT: u32,
    pub FIFO_CTRL: u32,
    pub FIFO_STAT: u32,
    pub DATACH0: u32,
    pub DATACH1: u32,
    pub DATACH2: u32,
    pub DATACH3: u32,
    pub DATACH4: u32,
    pub DATACH5: u32,
    pub DATACH6: u32,
    pub DATACH7: u32,
    pub DC_CTRL: u32,
    pub RANGE_CTRL: u32,
    pub RANGE_STAT: u32,
    pub VAD0_CTRL_1: u32,
    pub VAD0_CTRL_2: u32,
    pub VAD0_STAT: u32,
    pub VAD0_SCONFIG: u32,
    pub VAD0_NCONFIG: u32,
    pub VAD0_NDATA: u32,
    pub VAD0_ZCD: u32,
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
