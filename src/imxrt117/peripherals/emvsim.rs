#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! EMVSIM
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Version ID Register
pub mod VER_ID {

    /// Version ID of the module
    pub mod VER {
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

/// Parameter Register
pub mod PARAM {

    /// Receive FIFO Depth
    pub mod RX_FIFO_DEPTH {
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

    /// Transmit FIFO Depth
    pub mod TX_FIFO_DEPTH {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Clock Configuration Register
pub mod CLKCFG {

    /// Clock Prescaler Value
    pub mod CLK_PRSC {
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

    /// General Purpose Counter 1 Clock Select
    pub mod GPCNT1_CLK_SEL {
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

            /// 0b00: Disabled / Reset
            pub const disabled: u32 = 0b00;

            /// 0b01: Card Clock
            pub const cardclk: u32 = 0b01;

            /// 0b10: Receive Clock
            pub const rxclk: u32 = 0b10;

            /// 0b11: ETU Clock (transmit clock)
            pub const txclk: u32 = 0b11;
        }
    }

    /// General Purpose Counter 0 Clock Select
    pub mod GPCNT0_CLK_SEL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPCNT1_CLK_SEL::RW;
    }
}

/// Baud Rate Divisor Register
pub mod DIVISOR {

    /// Divisor (F/D) Value
    pub mod DIVISOR_VALUE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Control Register
pub mod CTRL {

    /// Inverse Convention
    pub mod IC {
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

            /// 0b0: Direction convention transfers enabled
            pub const dir_convention: u32 = 0b0;

            /// 0b1: Inverse convention transfers enabled
            pub const inv_convention: u32 = 0b1;
        }
    }

    /// Initial Character Mode
    pub mod ICM {
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

            /// 0b0: Initial Character Mode disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: Initial Character Mode enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Auto NACK Enable
    pub mod ANACK {
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

            /// 0b0: NACK generation on errors disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: NACK generation on errors enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Overrun NACK Enable
    pub mod ONACK {
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

            /// 0b0: NACK generation on overrun is disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: NACK generation on overrun is enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Flush Receiver Bit
    pub mod FLSH_RX {
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

            /// 0b0: EMVSIM Receiver normal operation
            pub const normalop: u32 = 0b0;

            /// 0b1: EMVSIM Receiver held in Reset
            pub const resethold: u32 = 0b1;
        }
    }

    /// Flush Transmitter Bit
    pub mod FLSH_TX {
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

            /// 0b0: EMVSIM Transmitter normal operation
            pub const normalop: u32 = 0b0;

            /// 0b1: EMVSIM Transmitter held in Reset
            pub const resethold: u32 = 0b1;
        }
    }

    /// Software Reset Bit
    pub mod SW_RST {
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

            /// 0b0: EMVSIM Normal operation
            pub const normalop: u32 = 0b0;

            /// 0b1: EMVSIM held in Reset
            pub const resethold: u32 = 0b1;
        }
    }

    /// Kill all internal clocks
    pub mod KILL_CLOCKS {
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

            /// 0b0: EMVSIM input clock enabled
            pub const inclk_enabled: u32 = 0b0;

            /// 0b1: EMVSIM input clock is disabled
            pub const inclk_disabled: u32 = 0b1;
        }
    }

    /// Doze Enable
    pub mod DOZE_EN {
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

            /// 0b0: DOZE instruction gates all internal EMVSIM clocks as well as the Smart Card clock when the transmit FIFO is empty
            pub const doze_gate: u32 = 0b0;

            /// 0b1: DOZE instruction has no effect on EMVSIM module
            pub const doze_nogate: u32 = 0b1;
        }
    }

    /// STOP Enable
    pub mod STOP_EN {
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

            /// 0b0: STOP instruction shuts down all EMVSIM clocks
            pub const stop_all_clks: u32 = 0b0;

            /// 0b1: STOP instruction shuts down all clocks except for the Smart Card Clock (SCK) (clock provided to Smart Card)
            pub const only_sck_on: u32 = 0b1;
        }
    }

    /// Receiver Enable
    pub mod RCV_EN {
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

            /// 0b0: EMVSIM Receiver disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: EMVSIM Receiver enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Transmitter Enable
    pub mod XMT_EN {
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

            /// 0b0: EMVSIM Transmitter disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: EMVSIM Transmitter enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Receiver 11 ETU Mode Enable
    pub mod RCVR_11 {
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

            /// 0b0: Receiver configured for 12 ETU operation mode
            pub const rcvr_12: u32 = 0b0;

            /// 0b1: Receiver configured for 11 ETU operation mode
            pub const rcvr_11: u32 = 0b1;
        }
    }

    /// Receive DMA Enable
    pub mod RX_DMA_EN {
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

            /// 0b0: No DMA Read Request asserted for Receiver
            pub const no_dmaread_req: u32 = 0b0;

            /// 0b1: DMA Read Request asserted for Receiver
            pub const dmaread_req: u32 = 0b1;
        }
    }

    /// Transmit DMA Enable
    pub mod TX_DMA_EN {
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

            /// 0b0: No DMA Write Request asserted for Transmitter
            pub const no_dmawrite_req: u32 = 0b0;

            /// 0b1: DMA Write Request asserted for Transmitter
            pub const dmawrite_req: u32 = 0b1;
        }
    }

    /// Invert bits in the CRC Output Value
    pub mod INV_CRC_VAL {
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

            /// 0b0: Bits in CRC Output value are not inverted.
            pub const no_invert: u32 = 0b0;

            /// 0b1: Bits in CRC Output value are inverted.
            pub const invert: u32 = 0b1;
        }
    }

    /// CRC Output Value Bit Reversal or Flip
    pub mod CRC_OUT_FLIP {
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

            /// 0b0: Bits within the CRC output bytes are not reversed i.e. 15:0 remains 15:0
            pub const not_reversed: u32 = 0b0;

            /// 0b1: Bits within the CRC output bytes are reversed i.e. 15:0 becomes {8:15,0:7}
            pub const reversed: u32 = 0b1;
        }
    }

    /// CRC Input Byte's Bit Reversal or Flip Control
    pub mod CRC_IN_FLIP {
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

            /// 0b0: Bits in the input byte are not reversed (i.e. 7:0 remain 7:0) before the CRC calculation
            pub const not_reversed: u32 = 0b0;

            /// 0b1: Bits in the input byte are reversed (i.e. 7:0 becomes 0:7) before CRC calculation
            pub const reversed: u32 = 0b1;
        }
    }

    /// Character Wait Time Counter Enable
    pub mod CWT_EN {
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

            /// 0b0: Character Wait time Counter is disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: Character Wait time counter is enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// LRC Enable
    pub mod LRC_EN {
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

            /// 0b0: 8-bit Linear Redundancy Checking disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: 8-bit Linear Redundancy Checking enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// CRC Enable
    pub mod CRC_EN {
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

            /// 0b0: 16-bit Cyclic Redundancy Checking disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: 16-bit Cyclic Redundancy Checking enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Transmit CRC or LRC Enable
    pub mod XMT_CRC_LRC {
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

            /// 0b0: No CRC or LRC value is transmitted
            pub const no_crc_lrc_tx: u32 = 0b0;

            /// 0b1: Transmit LRC or CRC info when FIFO empties (whichever is enabled)
            pub const crc_lrc_tx: u32 = 0b1;
        }
    }

    /// Block Wait Time Counter Enable
    pub mod BWT_EN {
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

            /// 0b0: Disable BWT, BGT Counters
            pub const disabled: u32 = 0b0;

            /// 0b1: Enable BWT, BGT Counters
            pub const enabled: u32 = 0b1;
        }
    }
}

/// Interrupt Mask Register
pub mod INT_MASK {

    /// Receive Data Threshold Interrupt Mask
    pub mod RDT_IM {
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

            /// 0b0: RDTF interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: RDTF interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Transmit Complete Interrupt Mask
    pub mod TC_IM {
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

            /// 0b0: TCF interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: TCF interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Receive FIFO Overflow Interrupt Mask
    pub mod RFO_IM {
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

            /// 0b0: RFO interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: RFO interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Early Transmit Complete Interrupt Mask
    pub mod ETC_IM {
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

            /// 0b0: ETC interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: ETC interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Transmit FIFO Empty Interrupt Mask
    pub mod TFE_IM {
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

            /// 0b0: TFE interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: TFE interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Transmit NACK Threshold Interrupt Mask
    pub mod TNACK_IM {
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

            /// 0b0: TNTE interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: TNTE interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Transmit FIFO Full Interrupt Mask
    pub mod TFF_IM {
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

            /// 0b0: TFF interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: TFF interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Transmit Data Threshold Interrupt Mask
    pub mod TDT_IM {
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

            /// 0b0: TDTF interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: TDTF interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// General Purpose Timer 0 Timeout Interrupt Mask
    pub mod GPCNT0_IM {
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

            /// 0b0: GPCNT0_TO interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: GPCNT0_TO interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Character Wait Time Error Interrupt Mask
    pub mod CWT_ERR_IM {
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

            /// 0b0: CWT_ERR interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: CWT_ERR interrupt masked
            pub const int_disabled: u32 = 0b1;
        }
    }

    /// Receiver NACK Threshold Interrupt Mask
    pub mod RNACK_IM {
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

            /// 0b0: RTE interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: RTE interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Block Wait Time Error Interrupt Mask
    pub mod BWT_ERR_IM {
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

            /// 0b0: BWT_ERR interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: BWT_ERR interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Block Guard Time Error Interrupt
    pub mod BGT_ERR_IM {
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

            /// 0b0: BGT_ERR interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: BGT_ERR interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// General Purpose Counter 1 Timeout Interrupt Mask
    pub mod GPCNT1_IM {
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

            /// 0b0: GPCNT1_TO interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: GPCNT1_TO interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Receive Data Interrupt Mask
    pub mod RX_DATA_IM {
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

            /// 0b0: RX_DATA interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: RX_DATA interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Parity Error Interrupt Mask
    pub mod PEF_IM {
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

            /// 0b0: PEF interrupt enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: PEF interrupt masked
            pub const int_masked: u32 = 0b1;
        }
    }
}

/// Receiver Threshold Register
pub mod RX_THD {

    /// Receiver Data Threshold Value
    pub mod RDT {
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

    /// Receiver NACK Threshold Value
    pub mod RNCK_THD {
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
}

/// Transmitter Threshold Register
pub mod TX_THD {

    /// Transmitter Data Threshold Value
    pub mod TDT {
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

    /// Transmitter NACK Threshold Value
    pub mod TNCK_THD {
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
}

/// Receive Status Register
pub mod RX_STATUS {

    /// Receive FIFO Overflow Flag
    pub mod RFO {
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

            /// 0b0: No overrun error has occurred
            pub const no_overrun: u32 = 0b0;

            /// 0b1: A byte was received when the received FIFO was already full
            pub const overflow: u32 = 0b1;
        }
    }

    /// Receive Data Interrupt Flag
    pub mod RX_DATA {
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

            /// 0b0: No new byte is received
            pub const no_byte_rx: u32 = 0b0;

            /// 0b1: New byte is received ans stored in Receive FIFO
            pub const byte_rx: u32 = 0b1;
        }
    }

    /// Receive Data Threshold Interrupt Flag
    pub mod RDTF {
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

            /// 0b0: Number of unread bytes in receive FIFO less than the value set by RDT
            pub const lessthan_rxthresh: u32 = 0b0;

            /// 0b1: Number of unread bytes in receive FIFO greater or than equal to value set by RDT.
            pub const greater_eq_rxthresh: u32 = 0b1;
        }
    }

    /// LRC Check OK Flag
    pub mod LRC_OK {
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

            /// 0b0: Current LRC value does not match remainder.
            pub const lrc_notok: u32 = 0b0;

            /// 0b1: Current calculated LRC value matches the expected result (i.e. zero).
            pub const lrc_ok: u32 = 0b1;
        }
    }

    /// CRC Check OK Flag
    pub mod CRC_OK {
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

            /// 0b0: Current CRC value does not match remainder.
            pub const crc_notok: u32 = 0b0;

            /// 0b1: Current calculated CRC value matches the expected result.
            pub const crc_ok: u32 = 0b1;
        }
    }

    /// Character Wait Time Error Flag
    pub mod CWT_ERR {
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

            /// 0b0: No CWT violation has occurred
            pub const no_cwt_err: u32 = 0b0;

            /// 0b1: Time between two consecutive characters has exceeded the value in CWT_VAL.
            pub const cwt_err: u32 = 0b1;
        }
    }

    /// Received NACK Threshold Error Flag
    pub mod RTE {
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

            /// 0b0: Number of NACKs generated by the receiver is less than the value programmed in RNCK_THD
            pub const lessthan_nackthresh: u32 = 0b0;

            /// 0b1: Number of NACKs generated by the receiver is equal to the value programmed in RNCK_THD
            pub const greater_eq_nackthresh: u32 = 0b1;
        }
    }

    /// Block Wait Time Error Flag
    pub mod BWT_ERR {
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

            /// 0b0: Block wait time not exceeded
            pub const bwt_err_no: u32 = 0b0;

            /// 0b1: Block wait time was exceeded
            pub const bwt_err_yes: u32 = 0b1;
        }
    }

    /// Block Guard Time Error Flag
    pub mod BGT_ERR {
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

            /// 0b0: Block guard time was sufficient
            pub const bgt_err_sufficient: u32 = 0b0;

            /// 0b1: Block guard time was too small
            pub const bgt_err_toosmall: u32 = 0b1;
        }
    }

    /// Parity Error Flag
    pub mod PEF {
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

            /// 0b0: No parity error detected
            pub const no_parity_detect: u32 = 0b0;

            /// 0b1: Parity error detected
            pub const parity_detect: u32 = 0b1;
        }
    }

    /// Frame Error Flag
    pub mod FEF {
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

            /// 0b0: No frame error detected
            pub const no_fef_detect: u32 = 0b0;

            /// 0b1: Frame error detected
            pub const fef_detect: u32 = 0b1;
        }
    }

    /// Receive FIFO Write Pointer Value
    pub mod RX_WPTR {
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

    /// Receive FIFO Byte Count
    pub mod RX_CNT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: FIFO is emtpy
            pub const fifo_empty: u32 = 0b0000;
        }
    }
}

/// Transmitter Status Register
pub mod TX_STATUS {

    /// Transmit NACK Threshold Error Flag
    pub mod TNTE {
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

            /// 0b0: Transmit NACK threshold has not been reached
            pub const lessthan_nackthresh: u32 = 0b0;

            /// 0b1: Transmit NACK threshold reached; transmitter frozen
            pub const greater_eq_nackthresh: u32 = 0b1;
        }
    }

    /// Transmit FIFO Empty Flag
    pub mod TFE {
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

            /// 0b0: Transmit FIFO is not empty
            pub const fifo_empty: u32 = 0b0;

            /// 0b1: Transmit FIFO is empty
            pub const fifo_notempty: u32 = 0b1;
        }
    }

    /// Early Transmit Complete Flag
    pub mod ETCF {
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

            /// 0b0: Transmit pending or in progress
            pub const etx_pending: u32 = 0b0;

            /// 0b1: Transmit complete
            pub const etx_complete: u32 = 0b1;
        }
    }

    /// Transmit Complete Flag
    pub mod TCF {
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

            /// 0b0: Transmit pending or in progress
            pub const tx_pending: u32 = 0b0;

            /// 0b1: Transmit complete
            pub const tx_complete: u32 = 0b1;
        }
    }

    /// Transmit FIFO Full Flag
    pub mod TFF {
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

            /// 0b0: Transmit FIFO Full condition has not occurred
            pub const tx_fifo_notfull: u32 = 0b0;

            /// 0b1: A Transmit FIFO Full condition has occurred
            pub const tx_fifo_full: u32 = 0b1;
        }
    }

    /// Transmit Data Threshold Flag
    pub mod TDTF {
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

            /// 0b0: Number of bytes in FIFO is greater than TDT, or bit has been cleared
            pub const lessthan_txthresh: u32 = 0b0;

            /// 0b1: Number of bytes in FIFO is less than or equal to TDT
            pub const greater_eq_txthresh: u32 = 0b1;
        }
    }

    /// General Purpose Counter 0 Timeout Flag
    pub mod GPCNT0_TO {
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

            /// 0b0: GPCNT0 time not reached, or bit has been cleared.
            pub const gpcnt0_to_notreached: u32 = 0b0;

            /// 0b1: General Purpose counter has reached the GPCNT0 value
            pub const gpcnt0_to_reached: u32 = 0b1;
        }
    }

    /// General Purpose Counter 1 Timeout Flag
    pub mod GPCNT1_TO {
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

            /// 0b0: GPCNT1 time not reached, or bit has been cleared.
            pub const gpcnt1_to_notreached: u32 = 0b0;

            /// 0b1: General Purpose counter has reached the GPCNT1 value
            pub const gpcnt1_to_reached: u32 = 0b1;
        }
    }

    /// Transmit FIFO Read Pointer
    pub mod TX_RPTR {
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

    /// Transmit FIFO Byte Count
    pub mod TX_CNT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: FIFO is emtpy
            pub const fifo_empty: u32 = 0b0000;
        }
    }
}

/// Port Control and Status Register
pub mod PCSR {

    /// Auto Power Down Enable
    pub mod SAPD {
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

            /// 0b0: Auto power down disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: Auto power down enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Vcc Enable for Smart Card
    pub mod SVCC_EN {
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

            /// 0b0: Smart Card Voltage disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: Smart Card Voltage enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// VCC Enable Polarity Control
    pub mod VCCENP {
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

            /// 0b0: SVCC_EN is active high. Polarity of SVCC_EN is unchanged.
            pub const active_high: u32 = 0b0;

            /// 0b1: SVCC_EN is active low. Polarity of SVCC_EN is inverted.
            pub const active_low: u32 = 0b1;
        }
    }

    /// Reset to Smart Card
    pub mod SRST {
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

            /// 0b0: Smart Card Reset is asserted
            pub const asserted: u32 = 0b0;

            /// 0b1: Smart Card Reset is de-asserted
            pub const de_asserted: u32 = 0b1;
        }
    }

    /// Clock Enable for Smart Card
    pub mod SCEN {
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

            /// 0b0: Smart Card Clock Disabled
            pub const disabled: u32 = 0b0;

            /// 0b1: Smart Card Clock Enabled
            pub const enabled: u32 = 0b1;
        }
    }

    /// Smart Card Clock Stop Polarity
    pub mod SCSP {
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

            /// 0b0: Clock is logic 0 when stopped by SCEN
            pub const scsp_logic0: u32 = 0b0;

            /// 0b1: Clock is logic 1 when stopped by SCEN
            pub const scsp_logic1: u32 = 0b1;
        }
    }

    /// Auto Power Down Control
    pub mod SPD {
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

            /// 0b0: No effect
            pub const no_effect: u32 = 0b0;

            /// 0b1: Start Auto Powerdown or Power Down is in progress
            pub const powerdown: u32 = 0b1;
        }
    }

    /// Smart Card Presence Detect Interrupt Mask
    pub mod SPDIM {
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

            /// 0b0: SIM presence detect interrupt is enabled
            pub const int_enabled: u32 = 0b0;

            /// 0b1: SIM presence detect interrupt is masked
            pub const int_masked: u32 = 0b1;
        }
    }

    /// Smart Card Presence Detect Interrupt Flag
    pub mod SPDIF {
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

            /// 0b0: No insertion or removal of Smart Card detected on Port
            pub const no_insert_remove_detect: u32 = 0b0;

            /// 0b1: Insertion or removal of Smart Card detected on Port
            pub const insert_remove_detect: u32 = 0b1;
        }
    }

    /// Smart Card Presence Detect Pin Status
    pub mod SPDP {
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

            /// 0b0: SIM Presence Detect pin is logic low
            pub const logic_low: u32 = 0b0;

            /// 0b1: SIM Presence Detectpin is logic high
            pub const logic_high: u32 = 0b1;
        }
    }

    /// SIM Presence Detect Edge Select
    pub mod SPDES {
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

            /// 0b0: Falling edge on the pin
            pub const falling_edge: u32 = 0b0;

            /// 0b1: Rising edge on the pin
            pub const rising_edge: u32 = 0b1;
        }
    }
}

/// Receive Data Read Buffer
pub mod RX_BUF {

    /// Receive Data Byte Read
    pub mod RX_BYTE {
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

/// Transmit Data Buffer
pub mod TX_BUF {

    /// Transmit Data Byte
    pub mod TX_BYTE {
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

/// Transmitter Guard ETU Value Register
pub mod TX_GETU {

    /// Transmitter Guard Time Value in ETU
    pub mod GETU {
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

/// Character Wait Time Value Register
pub mod CWT_VAL {

    /// Character Wait Time Value
    pub mod CWT {
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

/// Block Wait Time Value Register
pub mod BWT_VAL {

    /// Block Wait Time Value
    pub mod BWT {
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

/// Block Guard Time Value Register
pub mod BGT_VAL {

    /// Block Guard Time Value
    pub mod BGT {
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

/// General Purpose Counter 0 Timeout Value Register
pub mod GPCNT0_VAL {

    /// General Purpose Counter 0 Timeout Value
    pub mod GPCNT0 {
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

/// General Purpose Counter 1 Timeout Value
pub mod GPCNT1_VAL {

    /// General Purpose Counter 1 Timeout Value
    pub mod GPCNT1 {
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
#[repr(C)]
pub struct RegisterBlock {
    /// Version ID Register
    pub VER_ID: RORegister<u32>,

    /// Parameter Register
    pub PARAM: RORegister<u32>,

    /// Clock Configuration Register
    pub CLKCFG: RWRegister<u32>,

    /// Baud Rate Divisor Register
    pub DIVISOR: RWRegister<u32>,

    /// Control Register
    pub CTRL: RWRegister<u32>,

    /// Interrupt Mask Register
    pub INT_MASK: RWRegister<u32>,

    /// Receiver Threshold Register
    pub RX_THD: RWRegister<u32>,

    /// Transmitter Threshold Register
    pub TX_THD: RWRegister<u32>,

    /// Receive Status Register
    pub RX_STATUS: RWRegister<u32>,

    /// Transmitter Status Register
    pub TX_STATUS: RWRegister<u32>,

    /// Port Control and Status Register
    pub PCSR: RWRegister<u32>,

    /// Receive Data Read Buffer
    pub RX_BUF: RORegister<u32>,

    /// Transmit Data Buffer
    pub TX_BUF: RWRegister<u32>,

    /// Transmitter Guard ETU Value Register
    pub TX_GETU: RWRegister<u32>,

    /// Character Wait Time Value Register
    pub CWT_VAL: RWRegister<u32>,

    /// Block Wait Time Value Register
    pub BWT_VAL: RWRegister<u32>,

    /// Block Guard Time Value Register
    pub BGT_VAL: RWRegister<u32>,

    /// General Purpose Counter 0 Timeout Value Register
    pub GPCNT0_VAL: RWRegister<u32>,

    /// General Purpose Counter 1 Timeout Value
    pub GPCNT1_VAL: RWRegister<u32>,
}
pub struct ResetValues {
    pub VER_ID: u32,
    pub PARAM: u32,
    pub CLKCFG: u32,
    pub DIVISOR: u32,
    pub CTRL: u32,
    pub INT_MASK: u32,
    pub RX_THD: u32,
    pub TX_THD: u32,
    pub RX_STATUS: u32,
    pub TX_STATUS: u32,
    pub PCSR: u32,
    pub RX_BUF: u32,
    pub TX_BUF: u32,
    pub TX_GETU: u32,
    pub CWT_VAL: u32,
    pub BWT_VAL: u32,
    pub BGT_VAL: u32,
    pub GPCNT0_VAL: u32,
    pub GPCNT1_VAL: u32,
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
