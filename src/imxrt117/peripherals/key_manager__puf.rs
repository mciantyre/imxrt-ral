#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PUF
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister, WORegister};

/// PUF Control Register
pub mod CTRL {

    /// Begin Zeroize operation for PUF and go to Error state
    pub mod ZEROIZE {
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

            /// 0b0: No Zeroize operation in progress
            pub const UNSET: u32 = 0b0;

            /// 0b1: Zeroize operation in progress
            pub const SET: u32 = 0b1;
        }
    }

    /// Begin Enroll operation
    pub mod ENROLL {
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

            /// 0b0: No Enroll operation in progress
            pub const UNSET: u32 = 0b0;

            /// 0b1: Enroll operation in progress
            pub const SET: u32 = 0b1;
        }
    }

    /// Begin Start operation
    pub mod START {
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

            /// 0b0: No Start operation in progress
            pub const UNSET: u32 = 0b0;

            /// 0b1: Start operation in progress
            pub const SET: u32 = 0b1;
        }
    }

    /// Begin Set Intrinsic Key operation
    pub mod GENERATEKEY {
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

            /// 0b0: No Set Intrinsic Key operation in progress
            pub const UNSET: u32 = 0b0;

            /// 0b1: Set Intrinsic Key operation in progress
            pub const SET: u32 = 0b1;
        }
    }

    /// Begin Set User Key operation
    pub mod SETKEY {
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

            /// 0b0: No Set Key operation in progress
            pub const UNSET: u32 = 0b0;

            /// 0b1: Set Key operation in progress
            pub const SET: u32 = 0b1;
        }
    }

    /// Begin Get Key operation
    pub mod GETKEY {
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

            /// 0b0: No Get Key operation in progress
            pub const UNSET: u32 = 0b0;

            /// 0b1: Get Key operation in progress
            pub const SET: u32 = 0b1;
        }
    }
}

/// PUF Key Index Register
pub mod KEYINDEX {

    /// PUF Key Index
    pub mod KEYIDX {
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

            /// 0b0000: USE INDEX0
            pub const INDEX0: u32 = 0b0000;

            /// 0b0001: USE INDEX1
            pub const INDEX1: u32 = 0b0001;

            /// 0b0010: USE INDEX2
            pub const INDEX2: u32 = 0b0010;

            /// 0b0011: USE INDEX3
            pub const INDEX3: u32 = 0b0011;

            /// 0b0100: USE INDEX4
            pub const INDEX4: u32 = 0b0100;

            /// 0b0101: USE INDEX5
            pub const INDEX5: u32 = 0b0101;

            /// 0b0110: USE INDEX6
            pub const INDEX6: u32 = 0b0110;

            /// 0b0111: USE INDEX7
            pub const INDEX7: u32 = 0b0111;

            /// 0b1000: USE INDEX8
            pub const INDEX8: u32 = 0b1000;

            /// 0b1001: USE INDEX9
            pub const INDEX9: u32 = 0b1001;

            /// 0b1010: USE INDEX10
            pub const INDEX10: u32 = 0b1010;

            /// 0b1011: USE INDEX11
            pub const INDEX11: u32 = 0b1011;

            /// 0b1100: USE INDEX12
            pub const INDEX12: u32 = 0b1100;

            /// 0b1101: USE INDEX13
            pub const INDEX13: u32 = 0b1101;

            /// 0b1110: USE INDEX14
            pub const INDEX14: u32 = 0b1110;

            /// 0b1111: USE INDEX15
            pub const INDEX15: u32 = 0b1111;
        }
    }
}

/// PUF Key Size Register
pub mod KEYSIZE {

    /// PUF Key Size
    pub mod KEYSIZE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: Key Size is 512 Bytes and KC Size is 532 Bytes
            pub const SIZE64: u32 = 0b000000;

            /// 0b000001: Key Size is 8 Bytes and KC Size is 52 Bytes
            pub const SIZE1: u32 = 0b000001;

            /// 0b000010: Key Size is 16 Bytes and KC Size is 52 Bytes
            pub const SIZE2: u32 = 0b000010;

            /// 0b000011: Key Size is 24 Bytes and KC Size is 52 Bytes
            pub const SIZE3: u32 = 0b000011;

            /// 0b000100: Key Size is 32 Bytes and KC Size is 52 Bytes
            pub const SIZE4: u32 = 0b000100;

            /// 0b000101: Key Size is 40 Bytes and KC Size is 84 Bytes
            pub const SIZE5: u32 = 0b000101;

            /// 0b000110: Key Size is 48 Bytes and KC Size is 84 Bytes
            pub const SIZE6: u32 = 0b000110;

            /// 0b000111: Key Size is 56 Bytes and KC Size is 84 Bytes
            pub const SIZE7: u32 = 0b000111;

            /// 0b001000: Key Size is 64 Bytes and KC Size is 84 Bytes
            pub const SIZE8: u32 = 0b001000;

            /// 0b001001: Key Size is 72 Bytes and KC Size is 116 Bytes
            pub const SIZE9: u32 = 0b001001;

            /// 0b001010: Key Size is 80 Bytes and KC Size is 116 Bytes
            pub const SIZE10: u32 = 0b001010;

            /// 0b001011: Key Size is 88 Bytes and KC Size is 116 Bytes
            pub const SIZE11: u32 = 0b001011;

            /// 0b001100: Key Size is 96 Bytes and KC Size is 116 Bytes
            pub const SIZE12: u32 = 0b001100;

            /// 0b001101: Key Size is 104 Bytes and KC Size is 148 Bytes
            pub const SIZE13: u32 = 0b001101;

            /// 0b001110: Key Size is 112 Bytes and KC Size is 148 Bytes
            pub const SIZE14: u32 = 0b001110;

            /// 0b001111: Key Size is 120 Bytes and KC Size is 148 Bytes
            pub const SIZE15: u32 = 0b001111;

            /// 0b010000: Key Size is 128 Bytes and KC Size is 148 Bytes
            pub const SIZE16: u32 = 0b010000;

            /// 0b010001: Key Size is 136 Bytes and KC Size is 180 Bytes
            pub const SIZE17: u32 = 0b010001;

            /// 0b010010: Key Size is 144 Bytes and KC Size is 180 Bytes
            pub const SIZE18: u32 = 0b010010;

            /// 0b010011: Key Size is 152 Bytes and KC Size is 180 Bytes
            pub const SIZE19: u32 = 0b010011;

            /// 0b010100: Key Size is 160 Bytes and KC Size is 180 Bytes
            pub const SIZE20: u32 = 0b010100;

            /// 0b010101: Key Size is 168 Bytes and KC Size is 212 Bytes
            pub const SIZE21: u32 = 0b010101;

            /// 0b010110: Key Size is 176 Bytes and KC Size is 212 Bytes
            pub const SIZE22: u32 = 0b010110;

            /// 0b010111: Key Size is 184 Bytes and KC Size is 212 Bytes
            pub const SIZE23: u32 = 0b010111;

            /// 0b011000: Key Size is 192 Bytes and KC Size is 212 Bytes
            pub const SIZE24: u32 = 0b011000;

            /// 0b011001: Key Size is 200 Bytes and KC Size is 244 Bytes
            pub const SIZE25: u32 = 0b011001;

            /// 0b011010: Key Size is 208 Bytes and KC Size is 244 Bytes
            pub const SIZE26: u32 = 0b011010;

            /// 0b011011: Key Size is 216 Bytes and KC Size is 244 Bytes
            pub const SIZE27: u32 = 0b011011;

            /// 0b011100: Key Size is 224 Bytes and KC Size is 244 Bytes
            pub const SIZE28: u32 = 0b011100;

            /// 0b011101: Key Size is 232 Bytes and KC Size is 276 Bytes
            pub const SIZE29: u32 = 0b011101;

            /// 0b011110: Key Size is 240 Bytes and KC Size is 276 Bytes
            pub const SIZE30: u32 = 0b011110;

            /// 0b011111: Key Size is 248 Bytes and KC Size is 276 Bytes
            pub const SIZE31: u32 = 0b011111;

            /// 0b100000: Key Size is 256 Bytes and KC Size is 276 Bytes
            pub const SIZE32: u32 = 0b100000;

            /// 0b100001: Key Size is 264 Bytes and KC Size is 308 Bytes
            pub const SIZE33: u32 = 0b100001;

            /// 0b100010: Key Size is 272 Bytes and KC Size is 308 Bytes
            pub const SIZE34: u32 = 0b100010;

            /// 0b100011: Key Size is 280 Bytes and KC Size is 308 Bytes
            pub const SIZE35: u32 = 0b100011;

            /// 0b100100: Key Size is 288 Bytes and KC Size is 308 Bytes
            pub const SIZE36: u32 = 0b100100;

            /// 0b100101: Key Size is 296 Bytes and KC Size is 340 Bytes
            pub const SIZE37: u32 = 0b100101;

            /// 0b100110: Key Size is 304 Bytes and KC Size is 340 Bytes
            pub const SIZE38: u32 = 0b100110;

            /// 0b100111: Key Size is 312 Bytes and KC Size is 340 Bytes
            pub const SIZE39: u32 = 0b100111;

            /// 0b101000: Key Size is 320 Bytes and KC Size is 340 Bytes
            pub const SIZE40: u32 = 0b101000;

            /// 0b101001: Key Size is 328 Bytes and KC Size is 372 Bytes
            pub const SIZE41: u32 = 0b101001;

            /// 0b101010: Key Size is 336 Bytes and KC Size is 372 Bytes
            pub const SIZE42: u32 = 0b101010;

            /// 0b101011: Key Size is 344 Bytes and KC Size is 372 Bytes
            pub const SIZE43: u32 = 0b101011;

            /// 0b101100: Key Size is 352 Bytes and KC Size is 372 Bytes
            pub const SIZE44: u32 = 0b101100;

            /// 0b101101: Key Size is 360 Bytes and KC Size is 404 Bytes
            pub const SIZE45: u32 = 0b101101;

            /// 0b101110: Key Size is 368 Bytes and KC Size is 404 Bytes
            pub const SIZE46: u32 = 0b101110;

            /// 0b101111: Key Size is 376 Bytes and KC Size is 404 Bytes
            pub const SIZE47: u32 = 0b101111;

            /// 0b110000: Key Size is 384 Bytes and KC Size is 404 Bytes
            pub const SIZE48: u32 = 0b110000;

            /// 0b110001: Key Size is 392 Bytes and KC Size is 436 Bytes
            pub const SIZE49: u32 = 0b110001;

            /// 0b110010: Key Size is 400 Bytes and KC Size is 436 Bytes
            pub const SIZE50: u32 = 0b110010;

            /// 0b110011: Key Size is 408 Bytes and KC Size is 436 Bytes
            pub const SIZE51: u32 = 0b110011;

            /// 0b110100: Key Size is 416 Bytes and KC Size is 436 Bytes
            pub const SIZE52: u32 = 0b110100;

            /// 0b110101: Key Size is 424 Bytes and KC Size is 468 Bytes
            pub const SIZE53: u32 = 0b110101;

            /// 0b110110: Key Size is 432 Bytes and KC Size is 468 Bytes
            pub const SIZE54: u32 = 0b110110;

            /// 0b110111: Key Size is 440 Bytes and KC Size is 468 Bytes
            pub const SIZE55: u32 = 0b110111;

            /// 0b111000: Key Size is 448 Bytes and KC Size is 468 Bytes
            pub const SIZE56: u32 = 0b111000;

            /// 0b111001: Key Size is 456 Bytes and KC Size is 500 Bytes
            pub const SIZE57: u32 = 0b111001;

            /// 0b111010: Key Size is 464 Bytes and KC Size is 500 Bytes
            pub const SIZE58: u32 = 0b111010;

            /// 0b111011: Key Size is 472 Bytes and KC Size is 500 Bytes
            pub const SIZE59: u32 = 0b111011;

            /// 0b111100: Key Size is 480 Bytes and KC Size is 500 Bytes
            pub const SIZE60: u32 = 0b111100;

            /// 0b111101: Key Size is 488 Bytes and KC Size is 532 Bytes
            pub const SIZE61: u32 = 0b111101;

            /// 0b111110: Key Size is 496 Bytes and KC Size is 532 Bytes
            pub const SIZE62: u32 = 0b111110;

            /// 0b111111: Key Size is 504 Bytes and KC Size is 532 Bytes
            pub const SIZE63: u32 = 0b111111;
        }
    }
}

/// PUF Status Register
pub mod STAT {

    /// puf_busy
    pub mod BUSY {
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

            /// 0b0: IDLE
            pub const IDLE: u32 = 0b0;

            /// 0b1: BUSY
            pub const BUSY: u32 = 0b1;
        }
    }

    /// puf_ok
    pub mod SUCCESS {
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

            /// 0b0: Last operation was unsuccessful
            pub const NO: u32 = 0b0;

            /// 0b1: Last operation was successful
            pub const SUCCESSFUL: u32 = 0b1;
        }
    }

    /// puf_error
    pub mod ERROR {
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

            /// 0b0: PUF is not in the Error state
            pub const NO_IN_ERROR: u32 = 0b0;

            /// 0b1: PUF is in the Error state
            pub const IN_ERROR: u32 = 0b1;
        }
    }

    /// KI_ir
    pub mod KEYINREQ {
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

            /// 0b0: No request for next part of key
            pub const NOREQUEST: u32 = 0b0;

            /// 0b1: Request for next part of key in KEYINPUT register
            pub const REQUEST: u32 = 0b1;
        }
    }

    /// KO_or
    pub mod KEYOUTAVAIL {
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

            /// 0b0: Next part of key is not available
            pub const NOAVAILABLE: u32 = 0b0;

            /// 0b1: Next part of key is available in KEYOUTPUT register
            pub const AVAILABLE: u32 = 0b1;
        }
    }

    /// CI_ir
    pub mod CODEINREQ {
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

            /// 0b0: No request for next part of Activation Code/Key Code
            pub const NOREQUEST: u32 = 0b0;

            /// 0b1: request for next part of Activation Code/Key Code in CODEINPUT register
            pub const REQUEST: u32 = 0b1;
        }
    }

    /// CO_or
    pub mod CODEOUTAVAIL {
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

            /// 0b0: Next part of Activation Code/Key Code is not available
            pub const NOAVAILABLE: u32 = 0b0;

            /// 0b1: Next part of Activation Code/Key Code is available in CODEOUTPUT register
            pub const AVAILABLE: u32 = 0b1;
        }
    }
}

/// PUF Allow Register
pub mod ALLOW {

    /// Allow Enroll operation
    pub mod ALLOWENROLL {
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

            /// 0b0: Specified operation is not currently allowed
            pub const NOALLOW: u32 = 0b0;

            /// 0b1: Specified operation is allowed
            pub const ALLOW: u32 = 0b1;
        }
    }

    /// Allow Start operation
    pub mod ALLOWSTART {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ALLOWENROLL::RW;
    }

    /// Allow Set Key operations
    pub mod ALLOWSETKEY {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ALLOWENROLL::RW;
    }

    /// Allow Get Key operation
    pub mod ALLOWGETKEY {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ALLOWENROLL::RW;
    }
}

/// PUF Key Input Register
pub mod KEYINPUT {

    /// Key input data
    pub mod KEYIN {
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

/// PUF Code Input Register
pub mod CODEINPUT {

    /// AC/KC input data
    pub mod CODEIN {
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

/// PUF Code Output Register
pub mod CODEOUTPUT {

    /// AC/KC output data
    pub mod CODEOUT {
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

/// PUF Key Output Index Register
pub mod KEYOUTINDEX {

    /// Output Key index
    pub mod KEYOUTIDX {
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

/// PUF Key Output Register
pub mod KEYOUTPUT {

    /// Key output data from a Get Key operation
    pub mod KEYOUT {
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

/// PUF Interface Status Register
pub mod IFSTAT {

    /// APB error has occurred
    pub mod ERROR {
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

            /// 0b0: NOERROR
            pub const NOERROR: u32 = 0b0;

            /// 0b1: ERROR
            pub const ERROR: u32 = 0b1;
        }
    }
}

/// PUF Version Register
pub mod VERSION {

    /// Version of PUF
    pub mod VERSION {
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

/// PUF Interrupt Enable
pub mod INTEN {

    /// PUF Ready Interrupt Enable
    pub mod READYEN {
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

            /// 0b0: PUF ready interrupt disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: PUF ready interrupt enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// PUF_OK Interrupt Enable
    pub mod SUCCESSEN {
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

            /// 0b0: PUF successful interrupt disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: PUF successful interrupt enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// PUF Error Interrupt Enable
    pub mod ERROREN {
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

            /// 0b0: PUF error interrupt disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: PUF error interrupt enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// PUF Key Input Register Interrupt Enable
    pub mod KEYINREQEN {
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

            /// 0b0: Key interrupt request disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Key interrupt request enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// PUF Key Output Register Interrupt Enable
    pub mod KEYOUTAVAILEN {
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

            /// 0b0: Key available interrupt disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Key available interrupt enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// PUF Code Input Register Interrupt Enable
    pub mod CODEINREQEN {
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

            /// 0b0: AC/KC interrupt request disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: AC/KC interrupt request enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// PUF Code Output Register Interrupt Enable
    pub mod CODEOUTAVAILEN {
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

            /// 0b0: AC/KC available interrupt disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: AC/KC available interrupt enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// PUF Interrupt Status
pub mod INTSTAT {

    /// PUF_FINISH Interrupt Status
    pub mod READY {
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

            /// 0b0: Indicates that last operation not finished
            pub const NOT_FINISHED: u32 = 0b0;

            /// 0b1: Indicates that last operation is finished
            pub const FINISHED: u32 = 0b1;
        }
    }

    /// PUF_OK Interrupt Status
    pub mod SUCCESS {
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

            /// 0b0: Indicates that last operation was not successful
            pub const UNSUCCESSFUL: u32 = 0b0;

            /// 0b1: Indicates that last operation was successful
            pub const SUCCESSFUL: u32 = 0b1;
        }
    }

    /// PUF_ERROR Interrupt Status
    pub mod ERROR {
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

            /// 0b0: PUF is not in the Error state and operations can be performed
            pub const NO_ERROR: u32 = 0b0;

            /// 0b1: PUF is in the Error state and no operations can be performed
            pub const ERROR: u32 = 0b1;
        }
    }

    /// PUF Key Input Register Interrupt Status
    pub mod KEYINREQ {
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

            /// 0b0: No request for next part of key
            pub const NO_REQUEST: u32 = 0b0;

            /// 0b1: Request for next part of key
            pub const REQUEST: u32 = 0b1;
        }
    }

    /// PUF Key Output Register Interrupt Status
    pub mod KEYOUTAVAIL {
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

            /// 0b0: Next part of key is not available
            pub const NOT_AVAILABLE: u32 = 0b0;

            /// 0b1: Next part of key is available
            pub const AVAILABLE: u32 = 0b1;
        }
    }

    /// PUF Code Input Register Interrupt Status
    pub mod CODEINREQ {
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

            /// 0b0: No request for next part of AC/KC
            pub const NO_REQUEST: u32 = 0b0;

            /// 0b1: Request for next part of AC/KC
            pub const REQUEST: u32 = 0b1;
        }
    }

    /// PUF Code Output Register Interrupt Status
    pub mod CODEOUTAVAIL {
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

            /// 0b0: Next part of AC/KC is not available
            pub const NOT_AVAILABLE: u32 = 0b0;

            /// 0b1: Next part of AC/KC is available
            pub const AVAILABLE: u32 = 0b1;
        }
    }
}

/// PUF Power Control Of RAM
pub mod PWRCTRL {

    /// PUF RAM on
    pub mod RAM_ON {
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

            /// 0b0: PUF RAM is in sleep mode (PUF operation disabled)
            pub const SLEEP: u32 = 0b0;

            /// 0b1: PUF RAM is awake (normal PUF operation enabled)
            pub const WAKE: u32 = 0b1;
        }
    }

    /// Clock disable
    pub mod CK_DIS {
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

            /// 0b0: PUF RAM is clocked (normal PUF operation enabled)
            pub const ENABLE: u32 = 0b0;

            /// 0b1: PUF RAM clock is gated/disabled (PUF operation disabled)
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// RAM initialization
    pub mod RAM_INITN {
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

            /// 0b0: Reset the PUF RAM (PUF operation disabled)
            pub const RESET: u32 = 0b0;

            /// 0b1: Do not reset the PUF RAM (normal PUF operation enabled)
            pub const DO_NOT_RESET: u32 = 0b1;
        }
    }

    /// PUF RAM power switches
    pub mod RAM_PSW {
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
}

/// PUF Configuration Register
pub mod CFG {

    /// PUF Block Set Key Disable
    pub mod PUF_BLOCK_SET_KEY {
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

            /// 0b0: Enable the Set Key state
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Disable the Set Key state
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// PUF Block Enroll Disable
    pub mod PUF_BLOCK_ENROLL {
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

            /// 0b0: Enable the Enrollment state
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Disable the Enrollment state
            pub const DISABLE: u32 = 0b1;
        }
    }
}

/// PUF Key Manager Lock
pub mod KEYLOCK {

    /// Lock Block 0
    pub mod LOCK0 {
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

            /// 0b00: SNVS Key block locked
            pub const LOCKED_00: u32 = 0b00;

            /// 0b01: SNVS Key block locked
            pub const LOCKED_01: u32 = 0b01;

            /// 0b10: SNVS Key block unlocked
            pub const UNLOCKED: u32 = 0b10;

            /// 0b11: SNVS Key block locked
            pub const LOCKED_11: u32 = 0b11;
        }
    }

    /// Lock Block 1
    pub mod LOCK1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: OTFAD Key block locked
            pub const LOCKED_00: u32 = 0b00;

            /// 0b01: OTFAD Key block locked
            pub const LOCKED_01: u32 = 0b01;

            /// 0b10: OTFAD Key block unlocked
            pub const UNLOCKED: u32 = 0b10;

            /// 0b11: OTFAD Key block locked
            pub const LOCKED_11: u32 = 0b11;
        }
    }
}

/// PUF Key Manager Enable
pub mod KEYENABLE {

    /// Enable Block 0
    pub mod ENABLE0 {
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

            /// 0b00: Key block 0 disabled
            pub const DISABLED_00: u32 = 0b00;

            /// 0b01: Key block 0 disabled
            pub const DISABLED_01: u32 = 0b01;

            /// 0b10: Key block 0 enabled
            pub const ENABLED: u32 = 0b10;

            /// 0b11: Key block 0 disabled
            pub const DISABLED_11: u32 = 0b11;
        }
    }

    /// Enable Block 1
    pub mod ENABLE1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Key block 1 disabled
            pub const DISABLED_00: u32 = 0b00;

            /// 0b01: Key block 1 disabled
            pub const DISABLED_01: u32 = 0b01;

            /// 0b10: Key block 1 enabled
            pub const ENABLED: u32 = 0b10;

            /// 0b11: Key block 1 disabled
            pub const DISABLED_11: u32 = 0b11;
        }
    }
}

/// PUF Key Manager Reset
pub mod KEYRESET {

    /// Reset Block 0
    pub mod RESET0 {
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

            /// 0b00: Do not reset key block 0
            pub const NORESET_00: u32 = 0b00;

            /// 0b01: Do not reset key block 0
            pub const NORESET_01: u32 = 0b01;

            /// 0b10: Reset key block 0
            pub const RESET: u32 = 0b10;

            /// 0b11: Do not reset key block 0
            pub const NORESET_11: u32 = 0b11;
        }
    }

    /// Reset Block 1
    pub mod RESET1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Do not reset key block 1
            pub const NORESET_00: u32 = 0b00;

            /// 0b01: Do not reset key block 1
            pub const NORESET_01: u32 = 0b01;

            /// 0b10: Reset key block 1
            pub const RESET: u32 = 0b10;

            /// 0b11: Do not reset key block 1
            pub const NORESET_11: u32 = 0b11;
        }
    }
}

/// PUF Index Block Key Output
pub mod IDXBLK {

    /// idxblk0
    pub mod IDXBLK0 {
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

    /// idxblk1
    pub mod IDXBLK1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk2
    pub mod IDXBLK2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk3
    pub mod IDXBLK3 {
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

    /// idxblk4
    pub mod IDXBLK4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk5
    pub mod IDXBLK5 {
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

    /// idxblk6
    pub mod IDXBLK6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk7
    pub mod IDXBLK7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk8
    pub mod IDXBLK8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk9
    pub mod IDXBLK9 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk10
    pub mod IDXBLK10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk11
    pub mod IDXBLK11 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk12
    pub mod IDXBLK12 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk13
    pub mod IDXBLK13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk14
    pub mod IDXBLK14 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk15
    pub mod IDXBLK15 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PUF Index Block Key Output
pub mod IDXBLK_DP {

    /// idxblk_dp0
    pub mod IDXBLK_DP0 {
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

    /// idxblk_dp1
    pub mod IDXBLK_DP1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_dp2
    pub mod IDXBLK_DP2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_dp3
    pub mod IDXBLK_DP3 {
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

    /// idxblk_dp4
    pub mod IDXBLK_DP4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_dp5
    pub mod IDXBLK_DP5 {
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

    /// idxblk_dp6
    pub mod IDXBLK_DP6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_dp7
    pub mod IDXBLK_DP7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_dp8
    pub mod IDXBLK_DP8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_dp9
    pub mod IDXBLK_DP9 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_dp10
    pub mod IDXBLK_DP10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_dp11
    pub mod IDXBLK_DP11 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_dp12
    pub mod IDXBLK_DP12 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_dp13
    pub mod IDXBLK_DP13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_dp14
    pub mod IDXBLK_DP14 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_dp15
    pub mod IDXBLK_DP15 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PUF Key Block 0 Mask Enable
pub mod KEYMASK0 {

    /// KEYMASK0
    pub mod KEYMASK {
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

/// PUF Key Block 1 Mask Enable
pub mod KEYMASK1 {
    pub use super::KEYMASK0::KEYMASK;
}

/// PUF Index Block Setting Status Register
pub mod IDXBLK_STATUS {

    /// idxblk_status0
    pub mod IDXBLK_STATUS0 {
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

    /// idxblk_status1
    pub mod IDXBLK_STATUS1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_status2
    pub mod IDXBLK_STATUS2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_status3
    pub mod IDXBLK_STATUS3 {
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

    /// idxblk_status4
    pub mod IDXBLK_STATUS4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_status5
    pub mod IDXBLK_STATUS5 {
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

    /// idxblk_status6
    pub mod IDXBLK_STATUS6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_status7
    pub mod IDXBLK_STATUS7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_status8
    pub mod IDXBLK_STATUS8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_status9
    pub mod IDXBLK_STATUS9 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_status10
    pub mod IDXBLK_STATUS10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_status11
    pub mod IDXBLK_STATUS11 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_status12
    pub mod IDXBLK_STATUS12 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_status13
    pub mod IDXBLK_STATUS13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_status14
    pub mod IDXBLK_STATUS14 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// idxblk_status15
    pub mod IDXBLK_STATUS15 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PUF Key Manager Shift Status
pub mod IDXBLK_SHIFT {

    /// Index of key space in block 0
    pub mod IND_KEY0 {
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

    /// Index of key space in block 1
    pub mod IND_KEY1 {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// PUF Control Register
    pub CTRL: RWRegister<u32>,

    /// PUF Key Index Register
    pub KEYINDEX: RWRegister<u32>,

    /// PUF Key Size Register
    pub KEYSIZE: RWRegister<u32>,

    _reserved1: [u32; 5],

    /// PUF Status Register
    pub STAT: RORegister<u32>,

    _reserved2: [u32; 1],

    /// PUF Allow Register
    pub ALLOW: RORegister<u32>,

    _reserved3: [u32; 5],

    /// PUF Key Input Register
    pub KEYINPUT: WORegister<u32>,

    /// PUF Code Input Register
    pub CODEINPUT: WORegister<u32>,

    /// PUF Code Output Register
    pub CODEOUTPUT: RORegister<u32>,

    _reserved4: [u32; 5],

    /// PUF Key Output Index Register
    pub KEYOUTINDEX: RORegister<u32>,

    /// PUF Key Output Register
    pub KEYOUTPUT: RORegister<u32>,

    _reserved5: [u32; 29],

    /// PUF Interface Status Register
    pub IFSTAT: RWRegister<u32>,

    _reserved6: [u32; 7],

    /// PUF Version Register
    pub VERSION: RORegister<u32>,

    /// PUF Interrupt Enable
    pub INTEN: RWRegister<u32>,

    /// PUF Interrupt Status
    pub INTSTAT: RWRegister<u32>,

    /// PUF Power Control Of RAM
    pub PWRCTRL: RWRegister<u32>,

    /// PUF Configuration Register
    pub CFG: RWRegister<u32>,

    _reserved7: [u32; 60],

    /// PUF Key Manager Lock
    pub KEYLOCK: RWRegister<u32>,

    /// PUF Key Manager Enable
    pub KEYENABLE: RWRegister<u32>,

    /// PUF Key Manager Reset
    pub KEYRESET: RWRegister<u32>,

    /// PUF Index Block Key Output
    pub IDXBLK: RWRegister<u32>,

    /// PUF Index Block Key Output
    pub IDXBLK_DP: RWRegister<u32>,

    /// PUF Key Block 0 Mask Enable
    pub KEYMASK0: RWRegister<u32>,

    /// PUF Key Block 1 Mask Enable
    pub KEYMASK1: RWRegister<u32>,

    _reserved8: [u32; 14],

    /// PUF Index Block Setting Status Register
    pub IDXBLK_STATUS: RORegister<u32>,

    /// PUF Key Manager Shift Status
    pub IDXBLK_SHIFT: RORegister<u32>,
}
pub struct ResetValues {
    pub CTRL: u32,
    pub KEYINDEX: u32,
    pub KEYSIZE: u32,
    pub STAT: u32,
    pub ALLOW: u32,
    pub KEYINPUT: u32,
    pub CODEINPUT: u32,
    pub CODEOUTPUT: u32,
    pub KEYOUTINDEX: u32,
    pub KEYOUTPUT: u32,
    pub IFSTAT: u32,
    pub VERSION: u32,
    pub INTEN: u32,
    pub INTSTAT: u32,
    pub PWRCTRL: u32,
    pub CFG: u32,
    pub KEYLOCK: u32,
    pub KEYENABLE: u32,
    pub KEYRESET: u32,
    pub IDXBLK: u32,
    pub IDXBLK_DP: u32,
    pub KEYMASK0: u32,
    pub KEYMASK1: u32,
    pub IDXBLK_STATUS: u32,
    pub IDXBLK_SHIFT: u32,
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
