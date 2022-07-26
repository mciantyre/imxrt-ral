#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SAI
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Version ID
pub mod VERID {

    /// Feature Specification Number
    pub mod FEATURE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000000000: Standard feature set.
            pub const STD: u32 = 0b0000000000000000;
        }
    }

    /// Minor Version Number
    pub mod MINOR {
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

    /// Major Version Number
    pub mod MAJOR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Parameter
pub mod PARAM {

    /// Number of Datalines
    pub mod DATALINE {
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

    /// FIFO Size
    pub mod FIFO {
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

    /// Frame Size
    pub mod FRAME {
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
}

/// Transmit Control
pub mod TCSR {

    /// FIFO Request DMA Enable
    pub mod FRDE {
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

            /// 0b0: Disables the DMA request.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enables the DMA request.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// FIFO Warning DMA Enable
    pub mod FWDE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FRDE::RW;
    }

    /// FIFO Request Interrupt Enable
    pub mod FRIE {
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

            /// 0b0: Disables the interrupt.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enables the interrupt.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// FIFO Warning Interrupt Enable
    pub mod FWIE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FRIE::RW;
    }

    /// FIFO Error Interrupt Enable
    pub mod FEIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FRIE::RW;
    }

    /// Sync Error Interrupt Enable
    pub mod SEIE {
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

            /// 0b0: Disables interrupt.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enables interrupt.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Word Start Interrupt Enable
    pub mod WSIE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SEIE::RW;
    }

    /// FIFO Request Flag
    pub mod FRF {
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

            /// 0b0: Transmit FIFO watermark has not been reached.
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Transmit FIFO watermark has been reached.
            pub const FLAG: u32 = 0b1;
        }
    }

    /// FIFO Warning Flag
    pub mod FWF {
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

            /// 0b0: No enabled transmit FIFO is empty.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enabled transmit FIFO is empty.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// FIFO Error Flag
    pub mod FEF {
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

            /// 0b0: Transmit underrun not detected.
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Transmit underrun detected.
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Sync Error Flag
    pub mod SEF {
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

            /// 0b0: Sync error not detected.
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Frame sync error detected.
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Word Start Flag
    pub mod WSF {
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

            /// 0b0: Start of word not detected.
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Start of word detected.
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Software Reset
    pub mod SR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Software reset.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// FIFO Reset
    pub mod FR {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect.
            pub const NO_EFFECT: u32 = 0b0;

            /// 0b1: FIFO reset.
            pub const RESET: u32 = 0b1;
        }
    }

    /// Bit Clock Enable
    pub mod BCE {
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

            /// 0b0: Transmit bit clock is disabled.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmit bit clock is enabled.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Debug Enable
    pub mod DBGE {
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

            /// 0b0: Transmitter is disabled in Debug mode, after completing the current frame.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmitter is enabled in Debug mode.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Stop Enable
    pub mod STOPE {
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

            /// 0b0: Transmitter disabled in Stop mode.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmitter enabled in Stop mode.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Transmitter Enable
    pub mod TE {
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

            /// 0b0: Transmitter is disabled.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmitter is enabled, or transmitter has been disabled and has not yet reached end of frame.
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Transmit Configuration 1
pub mod TCR1 {

    /// Transmit FIFO Watermark
    pub mod TFW {
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
}

/// Transmit Configuration 2
pub mod TCR2 {

    /// Bit Clock Divide
    pub mod DIV {
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

    /// Bit Clock Bypass
    pub mod BYP {
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

            /// 0b0: Internal bit clock is generated from bit clock divider.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Internal bit clock is divide by one of the audio master clock.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Bit Clock Direction
    pub mod BCD {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Bit clock is generated externally in Slave mode.
            pub const EXT_IN_SLAVE: u32 = 0b0;

            /// 0b1: Bit clock is generated internally in Master mode.
            pub const INT_IN_MASTER: u32 = 0b1;
        }
    }

    /// Bit Clock Polarity
    pub mod BCP {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge.
            pub const ACTIVE_HIGH: u32 = 0b0;

            /// 0b1: Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge.
            pub const ACTIVE_LOW: u32 = 0b1;
        }
    }

    /// MCLK Select
    pub mod MSEL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Bus Clock selected.
            pub const BUS_CLOCK: u32 = 0b00;

            /// 0b01: Master Clock (MCLK) 1 option selected.
            pub const MCLK1: u32 = 0b01;

            /// 0b10: Master Clock (MCLK) 2 option selected.
            pub const MCLK2: u32 = 0b10;

            /// 0b11: Master Clock (MCLK) 3 option selected.
            pub const MCLK3: u32 = 0b11;
        }
    }

    /// Bit Clock Input
    pub mod BCI {
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

            /// 0b0: No effect.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Internal logic is clocked as if bit clock was externally generated.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Bit Clock Swap
    pub mod BCS {
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

            /// 0b0: Use the normal bit clock source.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Swap the bit clock source.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Synchronous Mode
    pub mod SYNC {
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

            /// 0b0: Asynchronous mode.
            pub const ASYNC: u32 = 0b0;

            /// 0b1: Synchronous with receiver.
            pub const SYNC_W_RX: u32 = 0b1;
        }
    }
}

/// Transmit Configuration 3
pub mod TCR3 {

    /// Word Flag Configuration
    pub mod WDFL {
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

    /// Transmit Channel Enable
    pub mod TCE {
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

/// Transmit Configuration 4
pub mod TCR4 {

    /// Frame Sync Direction
    pub mod FSD {
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

            /// 0b0: Frame sync is generated externally in Slave mode.
            pub const EXT_IN_SLAVE_MODE: u32 = 0b0;

            /// 0b1: Frame sync is generated internally in Master mode.
            pub const INT_IN_MASTER_MODE: u32 = 0b1;
        }
    }

    /// Frame Sync Polarity
    pub mod FSP {
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

            /// 0b0: Frame sync is active high.
            pub const ACTIVE_HIGH: u32 = 0b0;

            /// 0b1: Frame sync is active low.
            pub const ACTIVE_LOW: u32 = 0b1;
        }
    }

    /// On Demand Mode
    pub mod ONDEM {
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

            /// 0b0: Internal frame sync is generated continuously.
            pub const CONTINUOUS_FRAME_SYNC: u32 = 0b0;

            /// 0b1: Internal frame sync is generated when the FIFO warning flag is clear.
            pub const ON_DEMAND_FRAME_SYNC: u32 = 0b1;
        }
    }

    /// Frame Sync Early
    pub mod FSE {
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

            /// 0b0: Frame sync asserts with the first bit of the frame.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Frame sync asserts one bit before the first bit of the frame.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MSB First
    pub mod MF {
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

            /// 0b0: LSB is transmitted first.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MSB is transmitted first.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Channel Mode
    pub mod CHMOD {
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

            /// 0b0: TDM mode, transmit data pins are tri-stated when slots are masked or channels are disabled.
            pub const TDM_MODE: u32 = 0b0;

            /// 0b1: Output mode, transmit data pins are never tri-stated and will output zero when slots are masked or channels are disabled.
            pub const OUTPUT_MODE: u32 = 0b1;
        }
    }

    /// Sync Width
    pub mod SYWD {
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

    /// Frame size
    pub mod FRSZ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FIFO Packing Mode
    pub mod FPACK {
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

            /// 0b00: FIFO packing is disabled.
            pub const DISABLED: u32 = 0b00;

            /// 0b10: 8-bit FIFO packing is enabled.
            pub const EIGHT_BIT_FIFO_PACKING: u32 = 0b10;

            /// 0b11: 16-bit FIFO packing is enabled.
            pub const SIXTEEN_BIT_FIFO_PACKING: u32 = 0b11;
        }
    }

    /// FIFO Continue on Error
    pub mod FCONT {
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

            /// 0b0: On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared.
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Transmit Configuration 5
pub mod TCR5 {

    /// First Bit Shifted
    pub mod FBT {
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

    /// Word 0 Width
    pub mod W0W {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Word N Width
    pub mod WNW {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmit Data
pub mod TDR0 {

    /// Transmit Data Register
    pub mod TDR {
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

/// Transmit FIFO
pub mod TFR0 {

    /// Read FIFO Pointer
    pub mod RFP {
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

    /// Write FIFO Pointer
    pub mod WFP {
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
}

/// Transmit Mask
pub mod TMR {

    /// Transmit Word Mask
    pub mod TWM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: Word N is enabled.
            pub const WORD_N_ENABLED: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Word N is masked. The transmit data pins are tri-stated or drive zero when masked.
            pub const WORD_N_MASKED: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// Receive Control
pub mod RCSR {

    /// FIFO Request DMA Enable
    pub mod FRDE {
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

            /// 0b0: Disables the DMA request.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enables the DMA request.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// FIFO Warning DMA Enable
    pub mod FWDE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FRDE::RW;
    }

    /// FIFO Request Interrupt Enable
    pub mod FRIE {
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

            /// 0b0: Disables the interrupt.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enables the interrupt.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// FIFO Warning Interrupt Enable
    pub mod FWIE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FRIE::RW;
    }

    /// FIFO Error Interrupt Enable
    pub mod FEIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FRIE::RW;
    }

    /// Sync Error Interrupt Enable
    pub mod SEIE {
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

            /// 0b0: Disables interrupt.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enables interrupt.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Word Start Interrupt Enable
    pub mod WSIE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SEIE::RW;
    }

    /// FIFO Request Flag
    pub mod FRF {
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

            /// 0b0: Receive FIFO watermark not reached.
            pub const BELOW_WATERMARK: u32 = 0b0;

            /// 0b1: Receive FIFO watermark has been reached.
            pub const WATERMARK_REACHED: u32 = 0b1;
        }
    }

    /// FIFO Warning Flag
    pub mod FWF {
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

            /// 0b0: No enabled receive FIFO is full.
            pub const NOT_FULL: u32 = 0b0;

            /// 0b1: Enabled receive FIFO is full.
            pub const FULL: u32 = 0b1;
        }
    }

    /// FIFO Error Flag
    pub mod FEF {
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

            /// 0b0: Receive overflow not detected.
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Receive overflow detected.
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Sync Error Flag
    pub mod SEF {
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

            /// 0b0: Sync error not detected.
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Frame sync error detected.
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Word Start Flag
    pub mod WSF {
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

            /// 0b0: Start of word not detected.
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: Start of word detected.
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Software Reset
    pub mod SR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect.
            pub const NO_EFFECT: u32 = 0b0;

            /// 0b1: Software reset.
            pub const SW_RESET: u32 = 0b1;
        }
    }

    /// FIFO Reset
    pub mod FR {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect.
            pub const NO_EFFECT: u32 = 0b0;

            /// 0b1: FIFO reset.
            pub const FIFO_RESET: u32 = 0b1;
        }
    }

    /// Bit Clock Enable
    pub mod BCE {
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

            /// 0b0: Receive bit clock is disabled.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive bit clock is enabled.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Debug Enable
    pub mod DBGE {
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

            /// 0b0: Receiver is disabled in Debug mode, after completing the current frame.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receiver is enabled in Debug mode.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Stop Enable
    pub mod STOPE {
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

            /// 0b0: Receiver disabled in Stop mode.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receiver enabled in Stop mode.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Receiver Enable
    pub mod RE {
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

            /// 0b0: Receiver is disabled.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receiver is enabled, or receiver has been disabled and has not yet reached end of frame.
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Receive Configuration 1
pub mod RCR1 {

    /// Receive FIFO Watermark
    pub mod RFW {
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
}

/// Receive Configuration 2
pub mod RCR2 {

    /// Bit Clock Divide
    pub mod DIV {
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

    /// Bit Clock Bypass
    pub mod BYP {
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

            /// 0b0: Internal bit clock is generated from bit clock divider.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Internal bit clock is divide by one of the audio master clock.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Bit Clock Direction
    pub mod BCD {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Bit clock is generated externally in Slave mode.
            pub const EXT_SLAVE_MODE: u32 = 0b0;

            /// 0b1: Bit clock is generated internally in Master mode.
            pub const INT_MASTER_MODE: u32 = 0b1;
        }
    }

    /// Bit Clock Polarity
    pub mod BCP {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge.
            pub const ACTIVE_HIGH: u32 = 0b0;

            /// 0b1: Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge.
            pub const ACTIVE_LOW: u32 = 0b1;
        }
    }

    /// MCLK Select
    pub mod MSEL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Bus Clock selected.
            pub const BUS_CLOCK: u32 = 0b00;

            /// 0b01: Master Clock (MCLK) 1 option selected.
            pub const MCLK1: u32 = 0b01;

            /// 0b10: Master Clock (MCLK) 2 option selected.
            pub const MCLK2: u32 = 0b10;

            /// 0b11: Master Clock (MCLK) 3 option selected.
            pub const MCLK3: u32 = 0b11;
        }
    }

    /// Bit Clock Input
    pub mod BCI {
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

            /// 0b0: No effect.
            pub const NO_EFFECT: u32 = 0b0;

            /// 0b1: Internal logic is clocked as if bit clock was externally generated.
            pub const CLOCKED_AS_IF_EXT_GENERATED: u32 = 0b1;
        }
    }

    /// Bit Clock Swap
    pub mod BCS {
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

            /// 0b0: Use the normal bit clock source.
            pub const NORMAL: u32 = 0b0;

            /// 0b1: Swap the bit clock source.
            pub const SWAP_BIT_CLK_SOURCE: u32 = 0b1;
        }
    }

    /// Synchronous Mode
    pub mod SYNC {
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

            /// 0b0: Asynchronous mode.
            pub const Async: u32 = 0b0;

            /// 0b1: Synchronous with transmitter.
            pub const SYNC_W_TX: u32 = 0b1;
        }
    }
}

/// Receive Configuration 3
pub mod RCR3 {

    /// Word Flag Configuration
    pub mod WDFL {
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

    /// Receive Channel Enable
    pub mod RCE {
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

/// Receive Configuration 4
pub mod RCR4 {

    /// Frame Sync Direction
    pub mod FSD {
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

            /// 0b0: Frame Sync is generated externally in Slave mode.
            pub const EXT_SLAVE_MODE: u32 = 0b0;

            /// 0b1: Frame Sync is generated internally in Master mode.
            pub const INT_MASTER_MODE: u32 = 0b1;
        }
    }

    /// Frame Sync Polarity
    pub mod FSP {
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

            /// 0b0: Frame sync is active high.
            pub const ACTIVE_HIGH: u32 = 0b0;

            /// 0b1: Frame sync is active low.
            pub const ACTIVE_LOW: u32 = 0b1;
        }
    }

    /// On Demand Mode
    pub mod ONDEM {
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

            /// 0b0: Internal frame sync is generated continuously.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Internal frame sync is generated when the FIFO warning flag is clear.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Frame Sync Early
    pub mod FSE {
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

            /// 0b0: Frame sync asserts with the first bit of the frame.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Frame sync asserts one bit before the first bit of the frame.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MSB First
    pub mod MF {
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

            /// 0b0: LSB is received first.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MSB is received first.
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Sync Width
    pub mod SYWD {
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

    /// Frame Size
    pub mod FRSZ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FIFO Packing Mode
    pub mod FPACK {
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

            /// 0b00: FIFO packing is disabled
            pub const DISABLED: u32 = 0b00;

            /// 0b10: 8-bit FIFO packing is enabled
            pub const EIGHT_BIT_PACKING: u32 = 0b10;

            /// 0b11: 16-bit FIFO packing is enabled
            pub const SIXTEEN_BIT_PACKING: u32 = 0b11;
        }
    }

    /// FIFO Continue on Error
    pub mod FCONT {
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

            /// 0b0: On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared.
            pub const DISABLE: u32 = 0b0;

            /// 0b1: On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared.
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Receive Configuration 5
pub mod RCR5 {
    pub use super::TCR5::FBT;
    pub use super::TCR5::W0W;
    pub use super::TCR5::WNW;
}

/// Receive Data
pub mod RDR0 {

    /// Receive Data Register
    pub mod RDR {
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

/// Receive FIFO
pub mod RFR0 {
    pub use super::TFR0::RFP;
    pub use super::TFR0::WFP;
}

/// Receive Mask
pub mod RMR {

    /// Receive Word Mask
    pub mod RWM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: Word N is enabled.
            pub const WORD_N_ENABLED: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Word N is masked.
            pub const WORD_N_MASKED: u32 = 0b00000000000000000000000000000001;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Version ID
    pub VERID: RORegister<u32>,

    /// Parameter
    pub PARAM: RORegister<u32>,

    /// Transmit Control
    pub TCSR: RWRegister<u32>,

    /// Transmit Configuration 1
    pub TCR1: RWRegister<u32>,

    /// Transmit Configuration 2
    pub TCR2: RWRegister<u32>,

    /// Transmit Configuration 3
    pub TCR3: RWRegister<u32>,

    /// Transmit Configuration 4
    pub TCR4: RWRegister<u32>,

    /// Transmit Configuration 5
    pub TCR5: RWRegister<u32>,

    /// Transmit Data
    pub TDR0: RWRegister<u32>,

    _reserved1: [u32; 7],

    /// Transmit FIFO
    pub TFR0: RORegister<u32>,

    _reserved2: [u32; 7],

    /// Transmit Mask
    pub TMR: RWRegister<u32>,

    _reserved3: [u32; 9],

    /// Receive Control
    pub RCSR: RWRegister<u32>,

    /// Receive Configuration 1
    pub RCR1: RWRegister<u32>,

    /// Receive Configuration 2
    pub RCR2: RWRegister<u32>,

    /// Receive Configuration 3
    pub RCR3: RWRegister<u32>,

    /// Receive Configuration 4
    pub RCR4: RWRegister<u32>,

    /// Receive Configuration 5
    pub RCR5: RWRegister<u32>,

    /// Receive Data
    pub RDR0: RORegister<u32>,

    _reserved4: [u32; 7],

    /// Receive FIFO
    pub RFR0: RORegister<u32>,

    _reserved5: [u32; 7],

    /// Receive Mask
    pub RMR: RWRegister<u32>,
}
pub struct ResetValues {
    pub VERID: u32,
    pub PARAM: u32,
    pub TCSR: u32,
    pub TCR1: u32,
    pub TCR2: u32,
    pub TCR3: u32,
    pub TCR4: u32,
    pub TCR5: u32,
    pub TDR0: u32,
    pub TFR0: u32,
    pub TMR: u32,
    pub RCSR: u32,
    pub RCR1: u32,
    pub RCR2: u32,
    pub RCR3: u32,
    pub RCR4: u32,
    pub RCR5: u32,
    pub RDR0: u32,
    pub RFR0: u32,
    pub RMR: u32,
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
