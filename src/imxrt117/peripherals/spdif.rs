#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SPDIF
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister, WORegister};

/// SPDIF Configuration Register
pub mod SCR {

    /// USrc_Sel
    pub mod USrc_Sel {
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

            /// 0b00: No embedded U channel
            pub const none: u32 = 0b00;

            /// 0b01: U channel from SPDIF receive block (CD mode)
            pub const spdif_rxblock: u32 = 0b01;

            /// 0b11: U channel from on chip transmitter
            pub const chip_transmit: u32 = 0b11;
        }
    }

    /// TxSel
    pub mod TxSel {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (3 bits: 0b111 << 2)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Off and output 0
            pub const off_out0: u32 = 0b000;

            /// 0b001: Feed-through SPDIFIN
            pub const feedthru: u32 = 0b001;

            /// 0b101: Tx Normal operation
            pub const normal_op: u32 = 0b101;
        }
    }

    /// ValCtrl
    pub mod ValCtrl {
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

            /// 0b0: Outgoing Validity always set
            pub const always_set: u32 = 0b0;

            /// 0b1: Outgoing Validity always clear
            pub const always_clear: u32 = 0b1;
        }
    }

    /// InputSrcSel
    pub mod InputSrcSel {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: SPDIF_IN
            pub const SPDIF_IN: u32 = 0b00;
        }
    }

    /// DMA_TX_En
    pub mod DMA_TX_En {
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

    /// DMA_Rx_En
    pub mod DMA_Rx_En {
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

    /// TxFIFO_Ctrl
    pub mod TxFIFO_Ctrl {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Send out digital zero on SPDIF Tx
            pub const send_zero: u32 = 0b00;

            /// 0b01: Tx Normal operation
            pub const normal: u32 = 0b01;

            /// 0b10: Reset to 1 sample remaining
            pub const reset_one: u32 = 0b10;
        }
    }

    /// soft_reset
    pub mod soft_reset {
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

    /// LOW_POWER
    pub mod LOW_POWER {
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

    /// TxFIFOEmpty_Sel
    pub mod TxFIFOEmpty_Sel {
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

            /// 0b00: Empty interrupt if 0 sample in Tx left and right FIFOs
            pub const empty_int_0: u32 = 0b00;

            /// 0b01: Empty interrupt if at most 4 sample in Tx left and right FIFOs
            pub const empty_int_4: u32 = 0b01;

            /// 0b10: Empty interrupt if at most 8 sample in Tx left and right FIFOs
            pub const empty_int_8: u32 = 0b10;

            /// 0b11: Empty interrupt if at most 12 sample in Tx left and right FIFOs
            pub const empty_int_12: u32 = 0b11;
        }
    }

    /// TxAutoSync
    pub mod TxAutoSync {
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

            /// 0b0: Tx FIFO auto sync off
            pub const off: u32 = 0b0;

            /// 0b1: Tx FIFO auto sync on
            pub const on: u32 = 0b1;
        }
    }

    /// RxAutoSync
    pub mod RxAutoSync {
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

            /// 0b0: Rx FIFO auto sync off
            pub const off: u32 = 0b0;

            /// 0b1: RxFIFO auto sync on
            pub const on: u32 = 0b1;
        }
    }

    /// RxFIFOFull_Sel
    pub mod RxFIFOFull_Sel {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (2 bits: 0b11 << 19)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Full interrupt if at least 1 sample in Rx left and right FIFOs
            pub const full_int_1: u32 = 0b00;

            /// 0b01: Full interrupt if at least 4 sample in Rx left and right FIFOs
            pub const full_int_4: u32 = 0b01;

            /// 0b10: Full interrupt if at least 8 sample in Rx left and right FIFOs
            pub const full_int_8: u32 = 0b10;

            /// 0b11: Full interrupt if at least 16 sample in Rx left and right FIFO
            pub const full_int_16: u32 = 0b11;
        }
    }

    /// RxFIFO_Rst
    pub mod RxFIFO_Rst {
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

            /// 0b0: Normal operation
            pub const normal: u32 = 0b0;

            /// 0b1: Reset register to 1 sample remaining
            pub const reset_one: u32 = 0b1;
        }
    }

    /// RxFIFO_Off_On
    pub mod RxFIFO_Off_On {
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

            /// 0b0: SPDIF Rx FIFO is on
            pub const on_0: u32 = 0b0;

            /// 0b1: SPDIF Rx FIFO is off. Does not accept data from interface
            pub const off_1: u32 = 0b1;
        }
    }

    /// RxFIFO_Ctrl
    pub mod RxFIFO_Ctrl {
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

            /// 0b0: Normal operation
            pub const normal: u32 = 0b0;

            /// 0b1: Always read zero from Rx data register
            pub const always_zero: u32 = 0b1;
        }
    }
}

/// CDText Control Register
pub mod SRCD {

    /// USyncMode
    pub mod USyncMode {
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

            /// 0b0: Non-CD data
            pub const non_cddata: u32 = 0b0;

            /// 0b1: CD user channel subcode
            pub const cduser_chsubcode: u32 = 0b1;
        }
    }
}

/// PhaseConfig Register
pub mod SRPC {

    /// GainSel
    pub mod GainSel {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 24*(2**10)
            pub const gainsel_0b000: u32 = 0b000;

            /// 0b001: 16*(2**10)
            pub const gainsel_0b001: u32 = 0b001;

            /// 0b010: 12*(2**10)
            pub const gainsel_0b010: u32 = 0b010;

            /// 0b011: 8*(2**10)
            pub const gainsel_0b011: u32 = 0b011;

            /// 0b100: 6*(2**10)
            pub const gainsel_0b100: u32 = 0b100;

            /// 0b101: 4*(2**10)
            pub const gainsel_0b101: u32 = 0b101;

            /// 0b110: 3*(2**10)
            pub const gainsel_0b110: u32 = 0b110;
        }
    }

    /// LOCK
    pub mod LOCK {
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

    /// ClkSrc_Sel
    pub mod ClkSrc_Sel {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (4 bits: 0b1111 << 7)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)
            pub const clksrc_0b0000: u32 = 0b0000;

            /// 0b0001: if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)
            pub const clksrc_0b0001: u32 = 0b0001;

            /// 0b0011: if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK
            pub const clksrc_0b0011: u32 = 0b0011;

            /// 0b0101: REF_CLK_32K (XTALOSC)
            pub const clksrc_0b0101: u32 = 0b0101;

            /// 0b0110: tx_clk (SPDIF0_CLK_ROOT)
            pub const clksrc_0b0110: u32 = 0b0110;

            /// 0b1000: SPDIF_EXT_CLK
            pub const clksrc_0b1000: u32 = 0b1000;
        }
    }
}

/// InterruptEn Register
pub mod SIE {

    /// RxFIFOFul
    pub mod RxFIFOFul {
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

    /// TxEm
    pub mod TxEm {
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

    /// LockLoss
    pub mod LockLoss {
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

    /// RxFIFOResyn
    pub mod RxFIFOResyn {
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

    /// RxFIFOUnOv
    pub mod RxFIFOUnOv {
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

    /// UQErr
    pub mod UQErr {
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

    /// UQSync
    pub mod UQSync {
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

    /// QRxOv
    pub mod QRxOv {
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

    /// QRxFul
    pub mod QRxFul {
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

    /// URxOv
    pub mod URxOv {
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

    /// URxFul
    pub mod URxFul {
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

    /// BitErr
    pub mod BitErr {
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

    /// SymErr
    pub mod SymErr {
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

    /// ValNoGood
    pub mod ValNoGood {
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

    /// CNew
    pub mod CNew {
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

    /// TxResyn
    pub mod TxResyn {
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

    /// TxUnOv
    pub mod TxUnOv {
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

    /// Lock
    pub mod Lock {
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

/// SIC and SIS
/// SIC: InterruptClear Register
/// SIS: InterruptStat Register
pub mod SI {

    /// LockLoss
    pub mod LockLoss {
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

    /// RxFIFOResyn
    pub mod RxFIFOResyn {
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

    /// RxFIFOUnOv
    pub mod RxFIFOUnOv {
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

    /// UQErr
    pub mod UQErr {
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

    /// UQSync
    pub mod UQSync {
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

    /// QRxOv
    pub mod QRxOv {
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

    /// URxOv
    pub mod URxOv {
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

    /// BitErr
    pub mod BitErr {
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

    /// SymErr
    pub mod SymErr {
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

    /// ValNoGood
    pub mod ValNoGood {
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

    /// CNew
    pub mod CNew {
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

    /// TxResyn
    pub mod TxResyn {
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

    /// TxUnOv
    pub mod TxUnOv {
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

    /// Lock
    pub mod Lock {
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

    /// RxFIFOFul
    pub mod RxFIFOFul {
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

    /// TxEm
    pub mod TxEm {
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

    /// QRxFul
    pub mod QRxFul {
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

    /// URxFul
    pub mod URxFul {
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
}

/// SPDIFRxLeft Register
pub mod SRL {

    /// RxDataLeft
    pub mod RxDataLeft {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFRxRight Register
pub mod SRR {

    /// RxDataRight
    pub mod RxDataRight {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFRxCChannel_h Register
pub mod SRCSH {

    /// RxCChannel_h
    pub mod RxCChannel_h {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFRxCChannel_l Register
pub mod SRCSL {

    /// RxCChannel_l
    pub mod RxCChannel_l {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// UchannelRx Register
pub mod SRU {

    /// RxUChannel
    pub mod RxUChannel {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// QchannelRx Register
pub mod SRQ {

    /// RxQChannel
    pub mod RxQChannel {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFTxLeft Register
pub mod STL {

    /// TxDataLeft
    pub mod TxDataLeft {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFTxRight Register
pub mod STR {

    /// TxDataRight
    pub mod TxDataRight {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFTxCChannelCons_h Register
pub mod STCSCH {

    /// TxCChannelCons_h
    pub mod TxCChannelCons_h {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFTxCChannelCons_l Register
pub mod STCSCL {

    /// TxCChannelCons_l
    pub mod TxCChannelCons_l {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// FreqMeas Register
pub mod SRFM {

    /// FreqMeas
    pub mod FreqMeas {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFTxClk Register
pub mod STC {

    /// TxClk_DF
    pub mod TxClk_DF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000: divider factor is 1
            pub const div1: u32 = 0b0000000;

            /// 0b0000001: divider factor is 2
            pub const div2: u32 = 0b0000001;

            /// 0b1111111: divider factor is 128
            pub const div128: u32 = 0b1111111;
        }
    }

    /// tx_all_clk_en
    pub mod tx_all_clk_en {
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

            /// 0b0: disable transfer clock.
            pub const disable: u32 = 0b0;

            /// 0b1: enable transfer clock.
            pub const enable: u32 = 0b1;
        }
    }

    /// TxClk_Source
    pub mod TxClk_Source {
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

            /// 0b000: REF_CLK_32K input (XTALOSC 32 kHz clock)
            pub const txclk_src_0b000: u32 = 0b000;

            /// 0b001: tx_clk input (from SPDIF0_CLK_ROOT. See clock control block for more information.)
            pub const txclk_src_0b001: u32 = 0b001;

            /// 0b011: SPDIF_EXT_CLK, from pads
            pub const txclk_src_0b011: u32 = 0b011;

            /// 0b101: ipg_clk input (frequency divided)
            pub const txclk_src_0b101: u32 = 0b101;
        }
    }

    /// SYSCLK_DF
    pub mod SYSCLK_DF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (9 bits: 0x1ff << 11)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000000: no clock signal
            pub const no_clk: u32 = 0b000000000;

            /// 0b000000001: divider factor is 2
            pub const div2: u32 = 0b000000001;

            /// 0b111111111: divider factor is 512
            pub const div512: u32 = 0b111111111;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// SPDIF Configuration Register
    pub SCR: RWRegister<u32>,

    /// CDText Control Register
    pub SRCD: RWRegister<u32>,

    /// PhaseConfig Register
    pub SRPC: RWRegister<u32>,

    /// InterruptEn Register
    pub SIE: RWRegister<u32>,

    /// SIC and SIS
    /// SIC: InterruptClear Register
    /// SIS: InterruptStat Register
    pub SI: RWRegister<u32>,

    /// SPDIFRxLeft Register
    pub SRL: RORegister<u32>,

    /// SPDIFRxRight Register
    pub SRR: RORegister<u32>,

    /// SPDIFRxCChannel_h Register
    pub SRCSH: RORegister<u32>,

    /// SPDIFRxCChannel_l Register
    pub SRCSL: RORegister<u32>,

    /// UchannelRx Register
    pub SRU: RORegister<u32>,

    /// QchannelRx Register
    pub SRQ: RORegister<u32>,

    /// SPDIFTxLeft Register
    pub STL: WORegister<u32>,

    /// SPDIFTxRight Register
    pub STR: WORegister<u32>,

    /// SPDIFTxCChannelCons_h Register
    pub STCSCH: RWRegister<u32>,

    /// SPDIFTxCChannelCons_l Register
    pub STCSCL: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// FreqMeas Register
    pub SRFM: RORegister<u32>,

    _reserved2: [u32; 2],

    /// SPDIFTxClk Register
    pub STC: RWRegister<u32>,
}
pub struct ResetValues {
    pub SCR: u32,
    pub SRCD: u32,
    pub SRPC: u32,
    pub SIE: u32,
    pub SI: u32,
    pub SRL: u32,
    pub SRR: u32,
    pub SRCSH: u32,
    pub SRCSL: u32,
    pub SRU: u32,
    pub SRQ: u32,
    pub STL: u32,
    pub STR: u32,
    pub STCSCH: u32,
    pub STCSCL: u32,
    pub SRFM: u32,
    pub STC: u32,
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
