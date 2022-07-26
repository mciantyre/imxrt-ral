#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! uSDHC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// DMA System Address
pub mod DS_ADDR {

    /// System address
    pub mod DS_ADDR {
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

/// Block Attributes
pub mod BLK_ATT {

    /// Transfer block size
    pub mod BLKSIZE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000000: No data transfer
            pub const BLKSIZE_0: u32 = 0b0000000000000;

            /// 0b0000000000001: 1 byte
            pub const BLKSIZE_1: u32 = 0b0000000000001;

            /// 0b0000000000010: 2 bytes
            pub const BLKSIZE_2: u32 = 0b0000000000010;

            /// 0b0000000000011: 3 bytes
            pub const BLKSIZE_3: u32 = 0b0000000000011;

            /// 0b0000000000100: 4 bytes
            pub const BLKSIZE_4: u32 = 0b0000000000100;

            /// 0b0000111111111: 511 bytes
            pub const BLKSIZE_511: u32 = 0b0000111111111;

            /// 0b0001000000000: 512 bytes
            pub const BLKSIZE_512: u32 = 0b0001000000000;

            /// 0b0100000000000: 2048 bytes
            pub const BLKSIZE_2048: u32 = 0b0100000000000;

            /// 0b1000000000000: 4096 bytes
            pub const BLKSIZE_4096: u32 = 0b1000000000000;
        }
    }

    /// Blocks count for current transfer
    pub mod BLKCNT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000000000: Stop count
            pub const BLKCNT_0: u32 = 0b0000000000000000;

            /// 0b0000000000000001: 1 block
            pub const BLKCNT_1: u32 = 0b0000000000000001;

            /// 0b0000000000000010: 2 blocks
            pub const BLKCNT_2: u32 = 0b0000000000000010;

            /// 0b1111111111111111: 65535 blocks
            pub const BLKCNT_65535: u32 = 0b1111111111111111;
        }
    }
}

/// Command Argument
pub mod CMD_ARG {

    /// Command argument
    pub mod CMDARG {
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

/// Command Transfer Type
pub mod CMD_XFR_TYP {

    /// Response type select
    pub mod RSPTYP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No response
            pub const RSPTYP_0: u32 = 0b00;

            /// 0b01: Response length 136
            pub const RSPTYP_1: u32 = 0b01;

            /// 0b10: Response length 48
            pub const RSPTYP_2: u32 = 0b10;

            /// 0b11: Response length 48, check busy after response
            pub const RSPTYP_3: u32 = 0b11;
        }
    }

    /// Command CRC check enable
    pub mod CCCEN {
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

            /// 0b0: Disables command CRC check
            pub const CCCEN_0: u32 = 0b0;

            /// 0b1: Enables command CRC check
            pub const CCCEN_1: u32 = 0b1;
        }
    }

    /// Command index check enable
    pub mod CICEN {
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

            /// 0b0: Disable command index check
            pub const CICEN_0: u32 = 0b0;

            /// 0b1: Enables command index check
            pub const CICEN_1: u32 = 0b1;
        }
    }

    /// Data present select
    pub mod DPSEL {
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

            /// 0b0: No data present
            pub const DPSEL_0: u32 = 0b0;

            /// 0b1: Data present
            pub const DPSEL_1: u32 = 0b1;
        }
    }

    /// Command type
    pub mod CMDTYP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Normal other commands
            pub const CMDTYP_0: u32 = 0b00;

            /// 0b01: Suspend CMD52 for writing bus suspend in CCCR
            pub const CMDTYP_1: u32 = 0b01;

            /// 0b10: Resume CMD52 for writing function select in CCCR
            pub const CMDTYP_2: u32 = 0b10;

            /// 0b11: Abort CMD12, CMD52 for writing I/O Abort in CCCR
            pub const CMDTYP_3: u32 = 0b11;
        }
    }

    /// Command index
    pub mod CMDINX {
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
}

/// Command Response0
pub mod CMD_RSP0 {

    /// Command response 0
    pub mod CMDRSP0 {
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

/// Command Response1
pub mod CMD_RSP1 {

    /// Command response 1
    pub mod CMDRSP1 {
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

/// Command Response2
pub mod CMD_RSP2 {

    /// Command response 2
    pub mod CMDRSP2 {
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

/// Command Response3
pub mod CMD_RSP3 {

    /// Command response 3
    pub mod CMDRSP3 {
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

/// Data Buffer Access Port
pub mod DATA_BUFF_ACC_PORT {

    /// Data content
    pub mod DATCONT {
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

/// Present State
pub mod PRES_STATE {

    /// Command inhibit (CMD)
    pub mod CIHB {
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

            /// 0b0: Can issue command using only CMD line
            pub const CIHB_0: u32 = 0b0;

            /// 0b1: Cannot issue command
            pub const CIHB_1: u32 = 0b1;
        }
    }

    /// Command Inhibit Data (DATA)
    pub mod CDIHB {
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

            /// 0b0: Can issue command that uses the DATA line
            pub const CDIHB_0: u32 = 0b0;

            /// 0b1: Cannot issue command that uses the DATA line
            pub const CDIHB_1: u32 = 0b1;
        }
    }

    /// Data line active
    pub mod DLA {
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

            /// 0b0: DATA line inactive
            pub const DLA_0: u32 = 0b0;

            /// 0b1: DATA line active
            pub const DLA_1: u32 = 0b1;
        }
    }

    /// SD clock stable
    pub mod SDSTB {
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

            /// 0b0: Clock is changing frequency and not stable.
            pub const SDSTB_0: u32 = 0b0;

            /// 0b1: Clock is stable.
            pub const SDSTB_1: u32 = 0b1;
        }
    }

    /// Peripheral clock gated off internally
    pub mod IPGOFF {
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

            /// 0b0: Peripheral clock is active.
            pub const IPGOFF_0: u32 = 0b0;

            /// 0b1: Peripheral clock is gated off.
            pub const IPGOFF_1: u32 = 0b1;
        }
    }

    /// HCLK gated off internally
    pub mod HCKOFF {
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

            /// 0b0: HCLK is active.
            pub const HCKOFF_0: u32 = 0b0;

            /// 0b1: HCLK is gated off.
            pub const HCKOFF_1: u32 = 0b1;
        }
    }

    /// IPG_PERCLK gated off internally
    pub mod PEROFF {
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

            /// 0b0: IPG_PERCLK is active.
            pub const PEROFF_0: u32 = 0b0;

            /// 0b1: IPG_PERCLK is gated off.
            pub const PEROFF_1: u32 = 0b1;
        }
    }

    /// SD clock gated off internally
    pub mod SDOFF {
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

            /// 0b0: SD clock is active.
            pub const SDOFF_0: u32 = 0b0;

            /// 0b1: SD clock is gated off.
            pub const SDOFF_1: u32 = 0b1;
        }
    }

    /// Write transfer active
    pub mod WTA {
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

            /// 0b0: No valid data
            pub const WTA_0: u32 = 0b0;

            /// 0b1: Transferring data
            pub const WTA_1: u32 = 0b1;
        }
    }

    /// Read transfer active
    pub mod RTA {
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

            /// 0b0: No valid data
            pub const RTA_0: u32 = 0b0;

            /// 0b1: Transferring data
            pub const RTA_1: u32 = 0b1;
        }
    }

    /// Buffer write enable
    pub mod BWEN {
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

            /// 0b0: Write disable
            pub const BWEN_0: u32 = 0b0;

            /// 0b1: Write enable
            pub const BWEN_1: u32 = 0b1;
        }
    }

    /// Buffer read enable
    pub mod BREN {
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

            /// 0b0: Read disable
            pub const BREN_0: u32 = 0b0;

            /// 0b1: Read enable
            pub const BREN_1: u32 = 0b1;
        }
    }

    /// Re-Tuning Request (only for SD3.0 SDR104 mode,and EMMC HS200 mode)
    pub mod RTR {
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

            /// 0b0: Fixed or well tuned sampling clock
            pub const RTR_0: u32 = 0b0;

            /// 0b1: Sampling clock needs re-tuning
            pub const RTR_1: u32 = 0b1;
        }
    }

    /// Tap select change done
    pub mod TSCD {
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

            /// 0b0: Delay cell select change is not finished.
            pub const TSCD_0: u32 = 0b0;

            /// 0b1: Delay cell select change is finished.
            pub const TSCD_1: u32 = 0b1;
        }
    }

    /// Card inserted
    pub mod CINST {
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

            /// 0b0: Power on reset or no card
            pub const CINST_0: u32 = 0b0;

            /// 0b1: Card inserted
            pub const CINST_1: u32 = 0b1;
        }
    }

    /// Card detect pin level
    pub mod CDPL {
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

            /// 0b0: No card present (CD_B = 1)
            pub const CDPL_0: u32 = 0b0;

            /// 0b1: Card present (CD_B = 0)
            pub const CDPL_1: u32 = 0b1;
        }
    }

    /// Write protect switch pin level
    pub mod WPSPL {
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

            /// 0b0: Write protected (WP = 1)
            pub const WPSPL_0: u32 = 0b0;

            /// 0b1: Write enabled (WP = 0)
            pub const WPSPL_1: u32 = 0b1;
        }
    }

    /// CMD line signal level
    pub mod CLSL {
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

    /// DATA\[7:0\] line signal level
    pub mod DLSL {
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

            /// 0b00000000: Data 0 line signal level
            pub const DATA0: u32 = 0b00000000;

            /// 0b00000001: Data 1 line signal level
            pub const DATA1: u32 = 0b00000001;

            /// 0b00000010: Data 2 line signal level
            pub const DATA2: u32 = 0b00000010;

            /// 0b00000011: Data 3 line signal level
            pub const DATA3: u32 = 0b00000011;

            /// 0b00000100: Data 4 line signal level
            pub const DATA4: u32 = 0b00000100;

            /// 0b00000101: Data 5 line signal level
            pub const DATA5: u32 = 0b00000101;

            /// 0b00000110: Data 6 line signal level
            pub const DATA6: u32 = 0b00000110;

            /// 0b00000111: Data 7 line signal level
            pub const DATA7: u32 = 0b00000111;
        }
    }
}

/// Protocol Control
pub mod PROT_CTRL {

    /// Data transfer width
    pub mod DTW {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 1-bit mode
            pub const DTW_0: u32 = 0b00;

            /// 0b01: 4-bit mode
            pub const DTW_1: u32 = 0b01;

            /// 0b10: 8-bit mode
            pub const DTW_2: u32 = 0b10;
        }
    }

    /// DATA3 as card detection pin
    pub mod D3CD {
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

            /// 0b0: DATA3 does not monitor card insertion
            pub const D3CD_0: u32 = 0b0;

            /// 0b1: DATA3 as card detection pin
            pub const D3CD_1: u32 = 0b1;
        }
    }

    /// Endian mode
    pub mod EMODE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Big endian mode
            pub const EMODE_0: u32 = 0b00;

            /// 0b01: Half word big endian mode
            pub const EMODE_1: u32 = 0b01;

            /// 0b10: Little endian mode
            pub const EMODE_2: u32 = 0b10;
        }
    }

    /// Card detect test level
    pub mod CDTL {
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

            /// 0b0: Card detect test level is 0, no card inserted
            pub const CDTL_0: u32 = 0b0;

            /// 0b1: Card detect test level is 1, card inserted
            pub const CDTL_1: u32 = 0b1;
        }
    }

    /// Card detect signal selection
    pub mod CDSS {
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

            /// 0b0: Card detection level is selected (for normal purpose).
            pub const CDSS_0: u32 = 0b0;

            /// 0b1: Card detection test level is selected (for test purpose).
            pub const CDSS_1: u32 = 0b1;
        }
    }

    /// DMA select
    pub mod DMASEL {
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

            /// 0b00: No DMA or simple DMA is selected.
            pub const DMASEL_0: u32 = 0b00;

            /// 0b01: ADMA1 is selected.
            pub const DMASEL_1: u32 = 0b01;

            /// 0b10: ADMA2 is selected.
            pub const DMASEL_2: u32 = 0b10;
        }
    }

    /// Stop at block gap request
    pub mod SABGREQ {
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

            /// 0b0: Transfer
            pub const SABGREQ_0: u32 = 0b0;

            /// 0b1: Stop
            pub const SABGREQ_1: u32 = 0b1;
        }
    }

    /// Continue request
    pub mod CREQ {
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

            /// 0b0: No effect
            pub const CREQ_0: u32 = 0b0;

            /// 0b1: Restart
            pub const CREQ_1: u32 = 0b1;
        }
    }

    /// Read wait control
    pub mod RWCTL {
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

            /// 0b0: Disables read wait control and stop SD clock at block gap when SABGREQ field is set
            pub const RWCTL_0: u32 = 0b0;

            /// 0b1: Enables read wait control and assert read wait without stopping SD clock at block gap when SABGREQ field is set
            pub const RWCTL_1: u32 = 0b1;
        }
    }

    /// Interrupt at block gap
    pub mod IABG {
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

            /// 0b0: Disables interrupt at block gap
            pub const IABG_0: u32 = 0b0;

            /// 0b1: Enables interrupt at block gap
            pub const IABG_1: u32 = 0b1;
        }
    }

    /// Read performed number 8 clock
    pub mod RD_DONE_NO_8CLK {
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

    /// Wakeup event enable on card interrupt
    pub mod WECINT {
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

            /// 0b0: Disables wakeup event enable on card interrupt
            pub const WECINT_0: u32 = 0b0;

            /// 0b1: Enables wakeup event enable on card interrupt
            pub const WECINT_1: u32 = 0b1;
        }
    }

    /// Wakeup event enable on SD card insertion
    pub mod WECINS {
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

            /// 0b0: Disable wakeup event enable on SD card insertion
            pub const WECINS_0: u32 = 0b0;

            /// 0b1: Enable wakeup event enable on SD card insertion
            pub const WECINS_1: u32 = 0b1;
        }
    }

    /// Wakeup event enable on SD card removal
    pub mod WECRM {
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

            /// 0b0: Disables wakeup event enable on SD card removal
            pub const WECRM_0: u32 = 0b0;

            /// 0b1: Enables wakeup event enable on SD card removal
            pub const WECRM_1: u32 = 0b1;
        }
    }

    /// Non-exact block read
    pub mod NON_EXACT_BLK_RD {
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

            /// 0b0: The block read is exact block read. Host driver does not need to issue abort command to terminate this multi-block read.
            pub const NON_EXACT_BLK_RD_0: u32 = 0b0;

            /// 0b1: The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read.
            pub const NON_EXACT_BLK_RD_1: u32 = 0b1;
        }
    }
}

/// System Control
pub mod SYS_CTRL {

    /// Divisor
    pub mod DVS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Divide-by-1
            pub const DVS_0: u32 = 0b0000;

            /// 0b0001: Divide-by-2
            pub const DVS_1: u32 = 0b0001;

            /// 0b1110: Divide-by-15
            pub const DVS_14: u32 = 0b1110;

            /// 0b1111: Divide-by-16
            pub const DVS_15: u32 = 0b1111;
        }
    }

    /// SDCLK frequency select
    pub mod SDCLKFS {
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

    /// Data timeout counter value
    pub mod DTOCV {
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

            /// 0b0000: SDCLK x 2 14
            pub const DTOCV_0: u32 = 0b0000;

            /// 0b0001: SDCLK x 2 15
            pub const DTOCV_1: u32 = 0b0001;

            /// 0b0010: SDCLK x 2 16
            pub const DTOCV_2: u32 = 0b0010;

            /// 0b0011: SDCLK x 2 17
            pub const DTOCV_3: u32 = 0b0011;

            /// 0b0100: SDCLK x 2 18
            pub const DTOCV_4: u32 = 0b0100;

            /// 0b0101: SDCLK x 2 19
            pub const DTOCV_5: u32 = 0b0101;

            /// 0b0110: SDCLK x 2 20
            pub const DTOCV_6: u32 = 0b0110;

            /// 0b0111: SDCLK x 2 21
            pub const DTOCV_7: u32 = 0b0111;

            /// 0b1000: SDCLK x 2 22
            pub const DTOCV_8: u32 = 0b1000;

            /// 0b1001: SDCLK x 2 23
            pub const DTOCV_9: u32 = 0b1001;

            /// 0b1010: SDCLK x 2 24
            pub const DTOCV_10: u32 = 0b1010;

            /// 0b1011: SDCLK x 2 25
            pub const DTOCV_11: u32 = 0b1011;

            /// 0b1100: SDCLK x 2 26
            pub const DTOCV_12: u32 = 0b1100;

            /// 0b1101: SDCLK x 2 27
            pub const DTOCV_13: u32 = 0b1101;

            /// 0b1110: SDCLK x 2 28
            pub const DTOCV_14: u32 = 0b1110;

            /// 0b1111: SDCLK x 2 29
            pub const DTOCV_15: u32 = 0b1111;
        }
    }

    /// Hardware reset
    pub mod IPP_RST_N {
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

    /// Software reset for all
    pub mod RSTA {
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

            /// 0b0: No reset
            pub const RSTA_0: u32 = 0b0;

            /// 0b1: Reset
            pub const RSTA_1: u32 = 0b1;
        }
    }

    /// Software reset for CMD line
    pub mod RSTC {
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

            /// 0b0: No reset
            pub const RSTC_0: u32 = 0b0;

            /// 0b1: Reset
            pub const RSTC_1: u32 = 0b1;
        }
    }

    /// Software reset for data line
    pub mod RSTD {
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

            /// 0b0: No reset
            pub const RSTD_0: u32 = 0b0;

            /// 0b1: Reset
            pub const RSTD_1: u32 = 0b1;
        }
    }

    /// Initialization active
    pub mod INITA {
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

    /// Reset tuning
    pub mod RSTT {
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
}

/// Interrupt Status
pub mod INT_STATUS {

    /// Command complete
    pub mod CC {
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

            /// 0b0: Command not complete
            pub const CC_0: u32 = 0b0;

            /// 0b1: Command complete
            pub const CC_1: u32 = 0b1;
        }
    }

    /// Transfer complete
    pub mod TC {
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

            /// 0b0: Transfer does not complete
            pub const TC_0: u32 = 0b0;

            /// 0b1: Transfer complete
            pub const TC_1: u32 = 0b1;
        }
    }

    /// Block gap event
    pub mod BGE {
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

            /// 0b0: No block gap event
            pub const BGE_0: u32 = 0b0;

            /// 0b1: Transaction stopped at block gap
            pub const BGE_1: u32 = 0b1;
        }
    }

    /// DMA interrupt
    pub mod DINT {
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

            /// 0b0: No DMA interrupt
            pub const DINT_0: u32 = 0b0;

            /// 0b1: DMA interrupt is generated.
            pub const DINT_1: u32 = 0b1;
        }
    }

    /// Buffer write ready
    pub mod BWR {
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

            /// 0b0: Not ready to write buffer
            pub const BWR_0: u32 = 0b0;

            /// 0b1: Ready to write buffer
            pub const BWR_1: u32 = 0b1;
        }
    }

    /// Buffer read ready
    pub mod BRR {
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

            /// 0b0: Not ready to read buffer
            pub const BRR_0: u32 = 0b0;

            /// 0b1: Ready to read buffer
            pub const BRR_1: u32 = 0b1;
        }
    }

    /// Card insertion
    pub mod CINS {
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

            /// 0b0: Card state unstable or removed
            pub const CINS_0: u32 = 0b0;

            /// 0b1: Card inserted
            pub const CINS_1: u32 = 0b1;
        }
    }

    /// Card removal
    pub mod CRM {
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

            /// 0b0: Card state unstable or inserted
            pub const CRM_0: u32 = 0b0;

            /// 0b1: Card removed
            pub const CRM_1: u32 = 0b1;
        }
    }

    /// Card interrupt
    pub mod CINT {
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

            /// 0b0: No card interrupt
            pub const CINT_0: u32 = 0b0;

            /// 0b1: Generate card interrupt
            pub const CINT_1: u32 = 0b1;
        }
    }

    /// Re-tuning event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)
    pub mod RTE {
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

            /// 0b0: Re-tuning is not required.
            pub const RTE_0: u32 = 0b0;

            /// 0b1: Re-tuning should be performed.
            pub const RTE_1: u32 = 0b1;
        }
    }

    /// Tuning pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)
    pub mod TP {
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

    /// Command timeout error
    pub mod CTOE {
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

            /// 0b0: No error
            pub const CTOE_0: u32 = 0b0;

            /// 0b1: Time out
            pub const CTOE_1: u32 = 0b1;
        }
    }

    /// Command CRC error
    pub mod CCE {
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

            /// 0b0: No error
            pub const CCE_0: u32 = 0b0;

            /// 0b1: CRC error generated
            pub const CCE_1: u32 = 0b1;
        }
    }

    /// Command end bit error
    pub mod CEBE {
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

            /// 0b0: No error
            pub const CEBE_0: u32 = 0b0;

            /// 0b1: End bit error generated
            pub const CEBE_1: u32 = 0b1;
        }
    }

    /// Command index error
    pub mod CIE {
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

            /// 0b0: No error
            pub const CIE_0: u32 = 0b0;

            /// 0b1: Error
            pub const CIE_1: u32 = 0b1;
        }
    }

    /// Data timeout error
    pub mod DTOE {
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

            /// 0b0: No error
            pub const DTOE_0: u32 = 0b0;

            /// 0b1: Time out
            pub const DTOE_1: u32 = 0b1;
        }
    }

    /// Data CRC error
    pub mod DCE {
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

            /// 0b0: No error
            pub const DCE_0: u32 = 0b0;

            /// 0b1: Error
            pub const DCE_1: u32 = 0b1;
        }
    }

    /// Data end bit error
    pub mod DEBE {
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

            /// 0b0: No error
            pub const DEBE_0: u32 = 0b0;

            /// 0b1: Error
            pub const DEBE_1: u32 = 0b1;
        }
    }

    /// Auto CMD12 error
    pub mod AC12E {
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

            /// 0b0: No error
            pub const AC12E_0: u32 = 0b0;

            /// 0b1: Error
            pub const AC12E_1: u32 = 0b1;
        }
    }

    /// Tuning error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)
    pub mod TNE {
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

    /// DMA error
    pub mod DMAE {
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

            /// 0b0: No error
            pub const DMAE_0: u32 = 0b0;

            /// 0b1: Error
            pub const DMAE_1: u32 = 0b1;
        }
    }
}

/// Interrupt Status Enable
pub mod INT_STATUS_EN {

    /// Command complete status enable
    pub mod CCSEN {
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

            /// 0b0: Masked
            pub const CCSEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CCSEN_1: u32 = 0b1;
        }
    }

    /// Transfer complete status enable
    pub mod TCSEN {
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

            /// 0b0: Masked
            pub const TCSEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const TCSEN_1: u32 = 0b1;
        }
    }

    /// Block gap event status enable
    pub mod BGESEN {
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

            /// 0b0: Masked
            pub const BGESEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const BGESEN_1: u32 = 0b1;
        }
    }

    /// DMA interrupt status enable
    pub mod DINTSEN {
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

            /// 0b0: Masked
            pub const DINTSEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DINTSEN_1: u32 = 0b1;
        }
    }

    /// Buffer write ready status enable
    pub mod BWRSEN {
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

            /// 0b0: Masked
            pub const BWRSEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const BWRSEN_1: u32 = 0b1;
        }
    }

    /// Buffer read ready status enable
    pub mod BRRSEN {
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

            /// 0b0: Masked
            pub const BRRSEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const BRRSEN_1: u32 = 0b1;
        }
    }

    /// Card insertion status enable
    pub mod CINSSEN {
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

            /// 0b0: Masked
            pub const CINSSEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CINSSEN_1: u32 = 0b1;
        }
    }

    /// Card removal status enable
    pub mod CRMSEN {
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

            /// 0b0: Masked
            pub const CRMSEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CRMSEN_1: u32 = 0b1;
        }
    }

    /// Card interrupt status enable
    pub mod CINTSEN {
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

            /// 0b0: Masked
            pub const CINTSEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CINTSEN_1: u32 = 0b1;
        }
    }

    /// Re-tuning event status enable
    pub mod RTESEN {
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

            /// 0b0: Masked
            pub const RTESEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const RTESEN_1: u32 = 0b1;
        }
    }

    /// Tuning pass status enable
    pub mod TPSEN {
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

            /// 0b0: Masked
            pub const TPSEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const TPSEN_1: u32 = 0b1;
        }
    }

    /// Command timeout error status enable
    pub mod CTOESEN {
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

            /// 0b0: Masked
            pub const CTOESEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CTOESEN_1: u32 = 0b1;
        }
    }

    /// Command CRC error status enable
    pub mod CCESEN {
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

            /// 0b0: Masked
            pub const CCESEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CCESEN_1: u32 = 0b1;
        }
    }

    /// Command end bit error status enable
    pub mod CEBESEN {
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

            /// 0b0: Masked
            pub const CEBESEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CEBESEN_1: u32 = 0b1;
        }
    }

    /// Command index error status enable
    pub mod CIESEN {
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

            /// 0b0: Masked
            pub const CIESEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CIESEN_1: u32 = 0b1;
        }
    }

    /// Data timeout error status enable
    pub mod DTOESEN {
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

            /// 0b0: Masked
            pub const DTOESEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DTOESEN_1: u32 = 0b1;
        }
    }

    /// Data CRC error status enable
    pub mod DCESEN {
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

            /// 0b0: Masked
            pub const DCESEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DCESEN_1: u32 = 0b1;
        }
    }

    /// Data end bit error status enable
    pub mod DEBESEN {
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

            /// 0b0: Masked
            pub const DEBESEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DEBESEN_1: u32 = 0b1;
        }
    }

    /// Auto CMD12 error status enable
    pub mod AC12ESEN {
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

            /// 0b0: Masked
            pub const AC12ESEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const AC12ESEN_1: u32 = 0b1;
        }
    }

    /// Tuning error status enable
    pub mod TNESEN {
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

            /// 0b0: Masked
            pub const TNESEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const TNESEN_1: u32 = 0b1;
        }
    }

    /// DMA error status enable
    pub mod DMAESEN {
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

            /// 0b0: Masked
            pub const DMAESEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DMAESEN_1: u32 = 0b1;
        }
    }
}

/// Interrupt Signal Enable
pub mod INT_SIGNAL_EN {

    /// Command complete interrupt enable
    pub mod CCIEN {
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

            /// 0b0: Masked
            pub const CCIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CCIEN_1: u32 = 0b1;
        }
    }

    /// Transfer complete interrupt enable
    pub mod TCIEN {
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

            /// 0b0: Masked
            pub const TCIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const TCIEN_1: u32 = 0b1;
        }
    }

    /// Block gap event interrupt enable
    pub mod BGEIEN {
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

            /// 0b0: Masked
            pub const BGEIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const BGEIEN_1: u32 = 0b1;
        }
    }

    /// DMA interrupt enable
    pub mod DINTIEN {
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

            /// 0b0: Masked
            pub const DINTIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DINTIEN_1: u32 = 0b1;
        }
    }

    /// Buffer write ready interrupt enable
    pub mod BWRIEN {
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

            /// 0b0: Masked
            pub const BWRIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const BWRIEN_1: u32 = 0b1;
        }
    }

    /// Buffer read ready interrupt enable
    pub mod BRRIEN {
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

            /// 0b0: Masked
            pub const BRRIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const BRRIEN_1: u32 = 0b1;
        }
    }

    /// Card insertion interrupt enable
    pub mod CINSIEN {
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

            /// 0b0: Masked
            pub const CINSIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CINSIEN_1: u32 = 0b1;
        }
    }

    /// Card removal interrupt enable
    pub mod CRMIEN {
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

            /// 0b0: Masked
            pub const CRMIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CRMIEN_1: u32 = 0b1;
        }
    }

    /// Card interrupt enable
    pub mod CINTIEN {
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

            /// 0b0: Masked
            pub const CINTIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CINTIEN_1: u32 = 0b1;
        }
    }

    /// Re-tuning event interrupt enable
    pub mod RTEIEN {
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

            /// 0b0: Masked
            pub const RTEIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const RTEIEN_1: u32 = 0b1;
        }
    }

    /// Tuning Pass interrupt enable
    pub mod TPIEN {
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

            /// 0b0: Masked
            pub const TPIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const TPIEN_1: u32 = 0b1;
        }
    }

    /// Command timeout error interrupt enable
    pub mod CTOEIEN {
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

            /// 0b0: Masked
            pub const CTOEIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CTOEIEN_1: u32 = 0b1;
        }
    }

    /// Command CRC error interrupt enable
    pub mod CCEIEN {
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

            /// 0b0: Masked
            pub const CCEIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CCEIEN_1: u32 = 0b1;
        }
    }

    /// Command end bit error interrupt enable
    pub mod CEBEIEN {
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

            /// 0b0: Masked
            pub const CEBEIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CEBEIEN_1: u32 = 0b1;
        }
    }

    /// Command index error interrupt enable
    pub mod CIEIEN {
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

            /// 0b0: Masked
            pub const CIEIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const CIEIEN_1: u32 = 0b1;
        }
    }

    /// Data timeout error interrupt enable
    pub mod DTOEIEN {
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

            /// 0b0: Masked
            pub const DTOEIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DTOEIEN_1: u32 = 0b1;
        }
    }

    /// Data CRC error interrupt enable
    pub mod DCEIEN {
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

            /// 0b0: Masked
            pub const DCEIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DCEIEN_1: u32 = 0b1;
        }
    }

    /// Data end bit error interrupt enable
    pub mod DEBEIEN {
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

            /// 0b0: Masked
            pub const DEBEIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DEBEIEN_1: u32 = 0b1;
        }
    }

    /// Auto CMD12 error interrupt enable
    pub mod AC12EIEN {
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

            /// 0b0: Masked
            pub const AC12EIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const AC12EIEN_1: u32 = 0b1;
        }
    }

    /// Tuning error interrupt enable
    pub mod TNEIEN {
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

            /// 0b0: Masked
            pub const TNEIEN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const TNEIEN_1: u32 = 0b1;
        }
    }

    /// DMA error interrupt enable
    pub mod DMAEIEN {
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

            /// 0b0: Masked
            pub const DMAEIEN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const DMAEIEN_1: u32 = 0b1;
        }
    }
}

/// Auto CMD12 Error Status
pub mod AUTOCMD12_ERR_STATUS {

    /// Auto CMD12 not executed
    pub mod AC12NE {
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

            /// 0b0: Executed
            pub const AC12NE_0: u32 = 0b0;

            /// 0b1: Not executed
            pub const AC12NE_1: u32 = 0b1;
        }
    }

    /// Auto CMD12 / 23 timeout error
    pub mod AC12TOE {
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

            /// 0b0: No error
            pub const AC12TOE_0: u32 = 0b0;

            /// 0b1: Time out
            pub const AC12TOE_1: u32 = 0b1;
        }
    }

    /// Auto CMD12 / 23 end bit error
    pub mod AC12EBE {
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

            /// 0b0: No error
            pub const AC12EBE_0: u32 = 0b0;

            /// 0b1: End bit error generated
            pub const AC12EBE_1: u32 = 0b1;
        }
    }

    /// Auto CMD12 / 23 CRC error
    pub mod AC12CE {
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

            /// 0b0: No CRC error
            pub const AC12CE_0: u32 = 0b0;

            /// 0b1: CRC error met in Auto CMD12/23 response
            pub const AC12CE_1: u32 = 0b1;
        }
    }

    /// Auto CMD12 / 23 index error
    pub mod AC12IE {
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

            /// 0b0: No error
            pub const AC12IE_0: u32 = 0b0;

            /// 0b1: Error, the CMD index in response is not CMD12/23
            pub const AC12IE_1: u32 = 0b1;
        }
    }

    /// Command not issued by Auto CMD12 error
    pub mod CNIBAC12E {
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

            /// 0b0: No error
            pub const CNIBAC12E_0: u32 = 0b0;

            /// 0b1: Not issued
            pub const CNIBAC12E_1: u32 = 0b1;
        }
    }

    /// Execute tuning
    pub mod EXECUTE_TUNING {
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

            /// 0b0: Tuning procedure is aborted
            pub const EXECUTE_TUNING_0: u32 = 0b0;

            /// 0b1: Start tuning procedure
            pub const EXECUTE_TUNING_1: u32 = 0b1;
        }
    }

    /// Sample clock select
    pub mod SMP_CLK_SEL {
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

            /// 0b0: Fixed clock is used to sample data
            pub const SMP_CLK_SEL_0: u32 = 0b0;

            /// 0b1: Tuned clock is used to sample data
            pub const SMP_CLK_SEL_1: u32 = 0b1;
        }
    }
}

/// Host Controller Capabilities
pub mod HOST_CTRL_CAP {

    /// SDR50 support
    pub mod SDR50_SUPPORT {
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

    /// SDR104 support
    pub mod SDR104_SUPPORT {
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

    /// DDR50 support
    pub mod DDR50_SUPPORT {
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

    /// Use Tuning for SDR50
    pub mod USE_TUNING_SDR50 {
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

            /// 0b0: SDR50 does not support tuning
            pub const USE_TUNING_SDR50_0: u32 = 0b0;

            /// 0b1: SDR50 supports tuning
            pub const USE_TUNING_SDR50_1: u32 = 0b1;
        }
    }

    /// Max block length
    pub mod MBL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 512 bytes
            pub const MBL_0: u32 = 0b000;

            /// 0b001: 1024 bytes
            pub const MBL_1: u32 = 0b001;

            /// 0b010: 2048 bytes
            pub const MBL_2: u32 = 0b010;

            /// 0b011: 4096 bytes
            pub const MBL_3: u32 = 0b011;
        }
    }

    /// ADMA support
    pub mod ADMAS {
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

            /// 0b0: Advanced DMA not supported
            pub const ADMAS_0: u32 = 0b0;

            /// 0b1: Advanced DMA supported
            pub const ADMAS_1: u32 = 0b1;
        }
    }

    /// High speed support
    pub mod HSS {
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

            /// 0b0: High speed not supported
            pub const HSS_0: u32 = 0b0;

            /// 0b1: High speed supported
            pub const HSS_1: u32 = 0b1;
        }
    }

    /// DMA support
    pub mod DMAS {
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

            /// 0b0: DMA not supported
            pub const DMAS_0: u32 = 0b0;

            /// 0b1: DMA supported
            pub const DMAS_1: u32 = 0b1;
        }
    }

    /// Suspend / resume support
    pub mod SRS {
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

            /// 0b0: Not supported
            pub const SRS_0: u32 = 0b0;

            /// 0b1: Supported
            pub const SRS_1: u32 = 0b1;
        }
    }

    /// Voltage support 3.3 V
    pub mod VS33 {
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

            /// 0b0: 3.3 V not supported
            pub const VS33_0: u32 = 0b0;

            /// 0b1: 3.3 V supported
            pub const VS33_1: u32 = 0b1;
        }
    }

    /// Voltage support 3.0 V
    pub mod VS30 {
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

            /// 0b0: 3.0 V not supported
            pub const VS30_0: u32 = 0b0;

            /// 0b1: 3.0 V supported
            pub const VS30_1: u32 = 0b1;
        }
    }

    /// Voltage support 1.8 V
    pub mod VS18 {
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

            /// 0b0: 1.8 V not supported
            pub const VS18_0: u32 = 0b0;

            /// 0b1: 1.8 V supported
            pub const VS18_1: u32 = 0b1;
        }
    }
}

/// Watermark Level
pub mod WTMK_LVL {

    /// Read watermark level
    pub mod RD_WML {
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

    /// Write watermark level
    pub mod WR_WML {
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
}

/// Mixer Control
pub mod MIX_CTRL {

    /// DMA enable
    pub mod DMAEN {
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

            /// 0b0: Disable
            pub const DMAEN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const DMAEN_1: u32 = 0b1;
        }
    }

    /// Block count enable
    pub mod BCEN {
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

            /// 0b0: Disable
            pub const BCEN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const BCEN_1: u32 = 0b1;
        }
    }

    /// Auto CMD12 enable
    pub mod AC12EN {
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

            /// 0b0: Disable
            pub const AC12EN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const AC12EN_1: u32 = 0b1;
        }
    }

    /// Dual data rate mode selection
    pub mod DDR_EN {
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

    /// Data transfer direction select
    pub mod DTDSEL {
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

            /// 0b0: Write (Host to card)
            pub const DTDSEL_0: u32 = 0b0;

            /// 0b1: Read (Card to host)
            pub const DTDSEL_1: u32 = 0b1;
        }
    }

    /// Multi / Single block select
    pub mod MSBSEL {
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

            /// 0b0: Single block
            pub const MSBSEL_0: u32 = 0b0;

            /// 0b1: Multiple blocks
            pub const MSBSEL_1: u32 = 0b1;
        }
    }

    /// Nibble position indication
    pub mod NIBBLE_POS {
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

    /// Auto CMD23 enable
    pub mod AC23EN {
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

    /// Execute tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)
    pub mod EXE_TUNE {
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

            /// 0b0: Not tuned or tuning completed
            pub const EXE_TUNE_0: u32 = 0b0;

            /// 0b1: Execute tuning
            pub const EXE_TUNE_1: u32 = 0b1;
        }
    }

    /// Clock selection
    pub mod SMP_CLK_SEL {
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

            /// 0b0: Fixed clock is used to sample data / cmd
            pub const SMP_CLK_SEL_0: u32 = 0b0;

            /// 0b1: Tuned clock is used to sample data / cmd
            pub const SMP_CLK_SEL_1: u32 = 0b1;
        }
    }

    /// Auto tuning enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)
    pub mod AUTO_TUNE_EN {
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

            /// 0b0: Disable auto tuning
            pub const AUTO_TUNE_EN_0: u32 = 0b0;

            /// 0b1: Enable auto tuning
            pub const AUTO_TUNE_EN_1: u32 = 0b1;
        }
    }

    /// Feedback clock source selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)
    pub mod FBCLK_SEL {
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

            /// 0b0: Feedback clock comes from the loopback CLK
            pub const FBCLK_SEL_0: u32 = 0b0;

            /// 0b1: Feedback clock comes from the ipp_card_clk_out
            pub const FBCLK_SEL_1: u32 = 0b1;
        }
    }

    /// Enable HS400 mode
    pub mod HS400_MODE {
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
}

/// Force Event
pub mod FORCE_EVENT {

    /// Force event auto command 12 not executed
    pub mod FEVTAC12NE {
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

    /// Force event auto command 12 time out error
    pub mod FEVTAC12TOE {
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

    /// Force event auto command 12 CRC error
    pub mod FEVTAC12CE {
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

    /// Force event Auto Command 12 end bit error
    pub mod FEVTAC12EBE {
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

    /// Force event Auto Command 12 index error
    pub mod FEVTAC12IE {
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

    /// Force event command not executed by Auto Command 12 error
    pub mod FEVTCNIBAC12E {
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

    /// Force event command time out error
    pub mod FEVTCTOE {
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

    /// Force event command CRC error
    pub mod FEVTCCE {
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

    /// Force event command end bit error
    pub mod FEVTCEBE {
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

    /// Force event command index error
    pub mod FEVTCIE {
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

    /// Force event data time out error
    pub mod FEVTDTOE {
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

    /// Force event data CRC error
    pub mod FEVTDCE {
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

    /// Force event data end bit error
    pub mod FEVTDEBE {
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

    /// Force event Auto Command 12 error
    pub mod FEVTAC12E {
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

    /// Force tuning error
    pub mod FEVTTNE {
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

    /// Force event DMA error
    pub mod FEVTDMAE {
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

    /// Force event card interrupt
    pub mod FEVTCINT {
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

/// ADMA Error Status
pub mod ADMA_ERR_STATUS {

    /// ADMA error state (when ADMA error is occurred)
    pub mod ADMAES {
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

    /// ADMA length mismatch error
    pub mod ADMALME {
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

            /// 0b0: No error
            pub const ADMALME_0: u32 = 0b0;

            /// 0b1: Error
            pub const ADMALME_1: u32 = 0b1;
        }
    }

    /// ADMA descriptor error
    pub mod ADMADCE {
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

            /// 0b0: No error
            pub const ADMADCE_0: u32 = 0b0;

            /// 0b1: Error
            pub const ADMADCE_1: u32 = 0b1;
        }
    }
}

/// ADMA System Address
pub mod ADMA_SYS_ADDR {

    /// ADMA system address
    pub mod ADS_ADDR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (30 bits: 0x3fffffff << 2)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DLL (Delay Line) Control
pub mod DLL_CTRL {

    /// DLL and delay chain
    pub mod DLL_CTRL_ENABLE {
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

    /// DLL reset
    pub mod DLL_CTRL_RESET {
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

    /// DLL slave delay line
    pub mod DLL_CTRL_SLV_FORCE_UPD {
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

    /// DLL slave delay target0
    pub mod DLL_CTRL_SLV_DLY_TARGET0 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (4 bits: 0b1111 << 3)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DLL gate update
    pub mod DLL_CTRL_GATE_UPDATE {
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

    /// DLL slave override
    pub mod DLL_CTRL_SLV_OVERRIDE {
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

    /// DLL slave override val
    pub mod DLL_CTRL_SLV_OVERRIDE_VAL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (7 bits: 0x7f << 9)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DLL slave delay target1
    pub mod DLL_CTRL_SLV_DLY_TARGET1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Slave delay line update interval
    pub mod DLL_CTRL_SLV_UPDATE_INT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (8 bits: 0xff << 20)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DLL control loop update interval
    pub mod DLL_CTRL_REF_UPDATE_INT {
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

/// DLL Status
pub mod DLL_STATUS {

    /// Slave delay-line lock status
    pub mod DLL_STS_SLV_LOCK {
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

    /// Reference DLL lock status
    pub mod DLL_STS_REF_LOCK {
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

    /// Slave delay line select status
    pub mod DLL_STS_SLV_SEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (7 bits: 0x7f << 2)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reference delay line select taps
    pub mod DLL_STS_REF_SEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (7 bits: 0x7f << 9)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CLK Tuning Control and Status
pub mod CLK_TUNE_CTRL_STATUS {

    /// Delay cells on the feedback clock between CLK_OUT and CLK_POST
    pub mod DLY_CELL_SET_POST {
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

    /// Delay cells on the feedback clock between CLK_PRE and CLK_OUT
    pub mod DLY_CELL_SET_OUT {
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

    /// delay cells on the feedback clock between the feedback clock and CLK_PRE
    pub mod DLY_CELL_SET_PRE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NXT error
    pub mod NXT_ERR {
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

    /// Delay cells added on the feedback clock between CLK_OUT and CLK_POST
    pub mod TAP_SEL_POST {
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

    /// Delay cells added on the feedback clock between CLK_PRE and CLK_OUT
    pub mod TAP_SEL_OUT {
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

    /// TAP_SEL_PRE
    pub mod TAP_SEL_PRE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PRE error
    pub mod PRE_ERR {
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

/// Strobe DLL control
pub mod STROBE_DLL_CTRL {

    /// Strobe DLL control enable
    pub mod STROBE_DLL_CTRL_ENABLE {
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

    /// Strobe DLL control reset
    pub mod STROBE_DLL_CTRL_RESET {
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

    /// Strobe DLL control slave force updated
    pub mod STROBE_DLL_CTRL_SLV_FORCE_UPD {
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

    /// Strobe DLL Control Slave Delay Target
    pub mod STROBE_DLL_CTRL_SLV_DLY_TARGET {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (4 bits: 0b1111 << 3)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Strobe DLL control gate update
    pub mod STROBE_DLL_CTRL_GATE_UPDATE {
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

    /// Strobe DLL control slave override
    pub mod STROBE_DLL_CTRL_SLV_OVERRIDE {
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

    /// Strobe DLL control slave Override value
    pub mod STROBE_DLL_CTRL_SLV_OVERRIDE_VAL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (7 bits: 0x7f << 9)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Strobe DLL control slave update interval
    pub mod STROBE_DLL_CTRL_SLV_UPDATE_INT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (8 bits: 0xff << 20)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Strobe DLL control reference update interval
    pub mod STROBE_DLL_CTRL_REF_UPDATE_INT {
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

/// Strobe DLL status
pub mod STROBE_DLL_STATUS {

    /// Strobe DLL status slave lock
    pub mod STROBE_DLL_STS_SLV_LOCK {
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

    /// Strobe DLL status reference lock
    pub mod STROBE_DLL_STS_REF_LOCK {
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

    /// Strobe DLL status slave select
    pub mod STROBE_DLL_STS_SLV_SEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (7 bits: 0x7f << 2)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Strobe DLL status reference select
    pub mod STROBE_DLL_STS_REF_SEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (7 bits: 0x7f << 9)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Vendor Specific Register
pub mod VEND_SPEC {

    /// Voltage selection
    pub mod VSELECT {
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

            /// 0b0: Change the voltage to high voltage range, around 3.0 V
            pub const VSELECT_0: u32 = 0b0;

            /// 0b1: Change the voltage to low voltage range, around 1.8 V
            pub const VSELECT_1: u32 = 0b1;
        }
    }

    /// Conflict check enable
    pub mod CONFLICT_CHK_EN {
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

            /// 0b0: Conflict check disable
            pub const CONFLICT_CHK_EN_0: u32 = 0b0;

            /// 0b1: Conflict check enable
            pub const CONFLICT_CHK_EN_1: u32 = 0b1;
        }
    }

    /// Check busy enable
    pub mod AC12_WR_CHKBUSY_EN {
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

            /// 0b0: Do not check busy after auto CMD12 for write data packet
            pub const AC12_WR_CHKBUSY_EN_0: u32 = 0b0;

            /// 0b1: Check busy after auto CMD12 for write data packet
            pub const AC12_WR_CHKBUSY_EN_1: u32 = 0b1;
        }
    }

    /// Force CLK
    pub mod FRC_SDCLK_ON {
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

            /// 0b0: CLK active or inactive is fully controlled by the hardware.
            pub const FRC_SDCLK_ON_0: u32 = 0b0;

            /// 0b1: Force CLK active
            pub const FRC_SDCLK_ON_1: u32 = 0b1;
        }
    }

    /// CRC Check Disable
    pub mod CRC_CHK_DIS {
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

            /// 0b0: Check CRC16 for every read data packet and check CRC fields for every write data packet
            pub const CRC_CHK_DIS_0: u32 = 0b0;

            /// 0b1: Ignore CRC16 check for every read data packet and ignore CRC fields check for every write data packet
            pub const CRC_CHK_DIS_1: u32 = 0b1;
        }
    }

    /// Byte access
    pub mod CMD_BYTE_EN {
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

            /// 0b0: Disable
            pub const CMD_BYTE_EN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const CMD_BYTE_EN_1: u32 = 0b1;
        }
    }
}

/// MMC Boot
pub mod MMC_BOOT {

    /// Boot ACK time out
    pub mod DTOCV_ACK {
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

            /// 0b0000: SDCLK x 2^14
            pub const DTOCV_ACK_0: u32 = 0b0000;

            /// 0b0001: SDCLK x 2^15
            pub const DTOCV_ACK_1: u32 = 0b0001;

            /// 0b0010: SDCLK x 2^16
            pub const DTOCV_ACK_2: u32 = 0b0010;

            /// 0b0011: SDCLK x 2^17
            pub const DTOCV_ACK_3: u32 = 0b0011;

            /// 0b0100: SDCLK x 2^18
            pub const DTOCV_ACK_4: u32 = 0b0100;

            /// 0b0101: SDCLK x 2^19
            pub const DTOCV_ACK_5: u32 = 0b0101;

            /// 0b0110: SDCLK x 2^20
            pub const DTOCV_ACK_6: u32 = 0b0110;

            /// 0b0111: SDCLK x 2^21
            pub const DTOCV_ACK_7: u32 = 0b0111;

            /// 0b1110: SDCLK x 2^28
            pub const DTOCV_ACK_14: u32 = 0b1110;

            /// 0b1111: SDCLK x 2^29
            pub const DTOCV_ACK_15: u32 = 0b1111;
        }
    }

    /// BOOT ACK
    pub mod BOOT_ACK {
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

            /// 0b0: No ack
            pub const BOOT_ACK_0: u32 = 0b0;

            /// 0b1: Ack
            pub const BOOT_ACK_1: u32 = 0b1;
        }
    }

    /// Boot mode
    pub mod BOOT_MODE {
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

            /// 0b0: Normal boot
            pub const BOOT_MODE_0: u32 = 0b0;

            /// 0b1: Alternative boot
            pub const BOOT_MODE_1: u32 = 0b1;
        }
    }

    /// Boot enable
    pub mod BOOT_EN {
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

            /// 0b0: Fast boot disable
            pub const BOOT_EN_0: u32 = 0b0;

            /// 0b1: Fast boot enable
            pub const BOOT_EN_1: u32 = 0b1;
        }
    }

    /// Auto stop at block gap
    pub mod AUTO_SABG_EN {
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

    /// Time out
    pub mod DISABLE_TIME_OUT {
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

            /// 0b0: Enable time out
            pub const DISABLE_TIME_OUT_0: u32 = 0b0;

            /// 0b1: Disable time out
            pub const DISABLE_TIME_OUT_1: u32 = 0b1;
        }
    }

    /// Stop At Block Gap value of automatic mode
    pub mod BOOT_BLK_CNT {
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

/// Vendor Specific 2 Register
pub mod VEND_SPEC2 {

    /// Card interrupt detection test
    pub mod CARD_INT_D3_TEST {
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

            /// 0b0: Check the card interrupt only when DATA3 is high.
            pub const CARD_INT_D3_TEST_0: u32 = 0b0;

            /// 0b1: Check the card interrupt by ignoring the status of DATA3.
            pub const CARD_INT_D3_TEST_1: u32 = 0b1;
        }
    }

    /// Tuning 8bit enable
    pub mod TUNING_8bit_EN {
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

    /// Tuning 1bit enable
    pub mod TUNING_1bit_EN {
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

    /// Tuning command enable
    pub mod TUNING_CMD_EN {
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

            /// 0b0: Auto tuning circuit does not check the CMD line.
            pub const TUNING_CMD_EN_0: u32 = 0b0;

            /// 0b1: Auto tuning circuit checks the CMD line.
            pub const TUNING_CMD_EN_1: u32 = 0b1;
        }
    }

    /// HS400 write clock stop enable
    pub mod HS400_WR_CLK_STOP_EN {
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

    /// HS400 read clock stop enable
    pub mod HS400_RD_CLK_STOP_EN {
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

    /// Argument2 register enable for ACMD23
    pub mod ACMD23_ARGU2_EN {
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

            /// 0b0: Disable
            pub const ACMD23_ARGU2_EN_0: u32 = 0b0;

            /// 0b1: Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enabled.
            pub const ACMD23_ARGU2_EN_1: u32 = 0b1;
        }
    }
}

/// Tuning Control
pub mod TUNING_CTRL {

    /// Tuning start
    pub mod TUNING_START_TAP {
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

    /// Disable command check for standard tuning
    pub mod DIS_CMD_CHK_FOR_STD_TUNING {
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

    /// Tuning counter
    pub mod TUNING_COUNTER {
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

    /// TUNING_STEP
    pub mod TUNING_STEP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data window
    pub mod TUNING_WINDOW {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Standard tuning circuit and procedure enable
    pub mod STD_TUNING_EN {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// DMA System Address
    pub DS_ADDR: RWRegister<u32>,

    /// Block Attributes
    pub BLK_ATT: RWRegister<u32>,

    /// Command Argument
    pub CMD_ARG: RWRegister<u32>,

    /// Command Transfer Type
    pub CMD_XFR_TYP: RWRegister<u32>,

    /// Command Response0
    pub CMD_RSP0: RORegister<u32>,

    /// Command Response1
    pub CMD_RSP1: RORegister<u32>,

    /// Command Response2
    pub CMD_RSP2: RORegister<u32>,

    /// Command Response3
    pub CMD_RSP3: RORegister<u32>,

    /// Data Buffer Access Port
    pub DATA_BUFF_ACC_PORT: RWRegister<u32>,

    /// Present State
    pub PRES_STATE: RORegister<u32>,

    /// Protocol Control
    pub PROT_CTRL: RWRegister<u32>,

    /// System Control
    pub SYS_CTRL: RWRegister<u32>,

    /// Interrupt Status
    pub INT_STATUS: RWRegister<u32>,

    /// Interrupt Status Enable
    pub INT_STATUS_EN: RWRegister<u32>,

    /// Interrupt Signal Enable
    pub INT_SIGNAL_EN: RWRegister<u32>,

    /// Auto CMD12 Error Status
    pub AUTOCMD12_ERR_STATUS: RWRegister<u32>,

    /// Host Controller Capabilities
    pub HOST_CTRL_CAP: RWRegister<u32>,

    /// Watermark Level
    pub WTMK_LVL: RWRegister<u32>,

    /// Mixer Control
    pub MIX_CTRL: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// Force Event
    pub FORCE_EVENT: RWRegister<u32>,

    /// ADMA Error Status
    pub ADMA_ERR_STATUS: RORegister<u32>,

    /// ADMA System Address
    pub ADMA_SYS_ADDR: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// DLL (Delay Line) Control
    pub DLL_CTRL: RWRegister<u32>,

    /// DLL Status
    pub DLL_STATUS: RORegister<u32>,

    /// CLK Tuning Control and Status
    pub CLK_TUNE_CTRL_STATUS: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// Strobe DLL control
    pub STROBE_DLL_CTRL: RWRegister<u32>,

    /// Strobe DLL status
    pub STROBE_DLL_STATUS: RORegister<u32>,

    _reserved4: [u32; 18],

    /// Vendor Specific Register
    pub VEND_SPEC: RWRegister<u32>,

    /// MMC Boot
    pub MMC_BOOT: RWRegister<u32>,

    /// Vendor Specific 2 Register
    pub VEND_SPEC2: RWRegister<u32>,

    /// Tuning Control
    pub TUNING_CTRL: RWRegister<u32>,
}
pub struct ResetValues {
    pub DS_ADDR: u32,
    pub BLK_ATT: u32,
    pub CMD_ARG: u32,
    pub CMD_XFR_TYP: u32,
    pub CMD_RSP0: u32,
    pub CMD_RSP1: u32,
    pub CMD_RSP2: u32,
    pub CMD_RSP3: u32,
    pub DATA_BUFF_ACC_PORT: u32,
    pub PRES_STATE: u32,
    pub PROT_CTRL: u32,
    pub SYS_CTRL: u32,
    pub INT_STATUS: u32,
    pub INT_STATUS_EN: u32,
    pub INT_SIGNAL_EN: u32,
    pub AUTOCMD12_ERR_STATUS: u32,
    pub HOST_CTRL_CAP: u32,
    pub WTMK_LVL: u32,
    pub MIX_CTRL: u32,
    pub FORCE_EVENT: u32,
    pub ADMA_ERR_STATUS: u32,
    pub ADMA_SYS_ADDR: u32,
    pub DLL_CTRL: u32,
    pub DLL_STATUS: u32,
    pub CLK_TUNE_CTRL_STATUS: u32,
    pub STROBE_DLL_CTRL: u32,
    pub STROBE_DLL_STATUS: u32,
    pub VEND_SPEC: u32,
    pub MMC_BOOT: u32,
    pub VEND_SPEC2: u32,
    pub TUNING_CTRL: u32,
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
