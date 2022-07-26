#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! no description available
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// OTP Controller Control and Status Register
pub mod CTRL {

    /// OTP write and read access address register
    pub mod ADDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OTP controller status bit
    pub mod BUSY {
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

            /// 0b0: No write or read access to OTP started.
            pub const not_busy: u32 = 0b0;

            /// 0b1: Write or read access to OTP started.
            pub const busy: u32 = 0b1;
        }
    }

    /// Locked Region Access Error
    pub mod ERROR {
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

            /// 0b0: No error.
            pub const no_error: u32 = 0b0;

            /// 0b1: Error - access to a locked region requested.
            pub const error: u32 = 0b1;
        }
    }

    /// Reload Shadow Registers
    pub mod RELOAD_SHADOWS {
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

            /// 0b0: Do not force shadow register re-load.
            pub const shadow_noforce_reload: u32 = 0b0;

            /// 0b1: Force shadow register re-load. This bit is cleared automatically after shadow registers are re-loaded.
            pub const shadow_force_reload: u32 = 0b1;
        }
    }

    /// Lock fuse word
    pub mod WORDLOCK {
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

            /// 0b0: No change to LOCK bit when programming a word using redundancy
            pub const NO_CHANGE: u32 = 0b0;

            /// 0b1: LOCK bit for fuse word will be set after successfully programming a word using redundancy
            pub const LOCK: u32 = 0b1;
        }
    }

    /// Write unlock
    pub mod WR_UNLOCK {
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

            /// 0b0000000000000000: OTP write access is locked.
            pub const otp_w_locked: u32 = 0b0000000000000000;

            /// 0b0011111001110111: OTP write access is unlocked.
            pub const otp_w_unlocked: u32 = 0b0011111001110111;
        }
    }
}

/// OTP Controller Control and Status Register
pub mod CTRL_SET {

    /// OTP write and read access address register
    pub mod ADDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OTP controller status bit
    pub mod BUSY {
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

    /// Locked Region Access Error
    pub mod ERROR {
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

    /// Reload Shadow Registers
    pub mod RELOAD_SHADOWS {
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

    /// Lock fuse word
    pub mod WORDLOCK {
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

    /// Write unlock
    pub mod WR_UNLOCK {
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

/// OTP Controller Control and Status Register
pub mod CTRL_CLR {
    pub use super::CTRL_SET::ADDR;
    pub use super::CTRL_SET::BUSY;
    pub use super::CTRL_SET::ERROR;
    pub use super::CTRL_SET::RELOAD_SHADOWS;
    pub use super::CTRL_SET::WORDLOCK;
    pub use super::CTRL_SET::WR_UNLOCK;
}

/// OTP Controller Control and Status Register
pub mod CTRL_TOG {
    pub use super::CTRL_SET::ADDR;
    pub use super::CTRL_SET::BUSY;
    pub use super::CTRL_SET::ERROR;
    pub use super::CTRL_SET::RELOAD_SHADOWS;
    pub use super::CTRL_SET::WORDLOCK;
    pub use super::CTRL_SET::WR_UNLOCK;
}

/// OTP Controller PDN Register
pub mod PDN {

    /// PDN value
    pub mod PDN {
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

            /// 0b0: OTP memory is not powered
            pub const power_off: u32 = 0b0;

            /// 0b1: OTP memory is powered
            pub const power_on: u32 = 0b1;
        }
    }
}

/// OTP Controller Write Data Register
pub mod DATA {

    /// Data
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

/// OTP Controller Read Control Register
pub mod READ_CTRL {

    /// Read Fuse
    pub mod READ_FUSE {
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

            /// 0b0: Do not initiate a read from OTP
            pub const DO_NOT_START_RD_OP: u32 = 0b0;

            /// 0b1: Initiate a read from OTP
            pub const START_RD_OP: u32 = 0b1;
        }
    }

    /// Number of words to read.
    pub mod READ_FUSE_CNTR {
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

            /// 0b00: 1 word
            pub const ONE_WORD: u32 = 0b00;

            /// 0b01: 2 words
            pub const TWO_WORDS: u32 = 0b01;

            /// 0b10: 3 words
            pub const THREE_WORDS: u32 = 0b10;

            /// 0b11: 4 words
            pub const FOUR_WORDS: u32 = 0b11;
        }
    }

    /// Enable read-done interrupt
    pub mod READ_FUSE_DONE_INTR_ENA {
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

            /// 0b0: Disable
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable read-error interrupt
    pub mod READ_FUSE_ERROR_INTR_ENA {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::READ_FUSE_DONE_INTR_ENA::RW;
    }
}

/// 8K OTP Memory STATUS Register
pub mod OUT_STATUS {

    /// Single Error Correct
    pub mod SEC {
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

    /// Double error detect
    pub mod DED {
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

    /// Word Locked
    pub mod LOCKED {
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

    /// Programming failed
    pub mod PROGFAIL {
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

    /// Acknowledge
    pub mod ACK {
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

    /// Power OK
    pub mod PWOK {
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

    /// Flag state
    pub mod FLAGSTATE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (4 bits: 0b1111 << 15)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates single error correction occured on reload
    pub mod SEC_RELOAD {
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

    /// Indicates double error detection occured on reload
    pub mod DED_RELOAD {
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

    /// Calibrated status
    pub mod CALIBRATED {
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

    /// Read fuse done
    pub mod READ_DONE_INTR {
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

    /// Fuse read error
    pub mod READ_ERROR_INTR {
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

            /// 0b0: Read operation finished with out any error
            pub const no_error: u32 = 0b0;

            /// 0b1: Read operation finished with an error
            pub const error: u32 = 0b1;
        }
    }

    /// Double error detect
    pub mod DED0 {
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

    /// Double error detect
    pub mod DED1 {
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

    /// Double error detect
    pub mod DED2 {
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

    /// Double error detect
    pub mod DED3 {
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
}

/// 8K OTP Memory STATUS Register
pub mod OUT_STATUS_SET {

    /// Single Error Correct
    pub mod SEC {
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

    /// Double error detect
    pub mod DED {
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

    /// Word Locked
    pub mod LOCKED {
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

    /// Programming failed
    pub mod PROGFAIL {
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

    /// Acknowledge
    pub mod ACK {
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

    /// Power OK
    pub mod PWOK {
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

    /// Flag state
    pub mod FLAGSTATE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (4 bits: 0b1111 << 15)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates single error correction occured on reload
    pub mod SEC_RELOAD {
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

    /// Indicates double error detection occured on reload
    pub mod DED_RELOAD {
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

    /// Calibrated status
    pub mod CALIBRATED {
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

    /// Read fuse done
    pub mod READ_DONE_INTR {
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

    /// Fuse read error
    pub mod READ_ERROR_INTR {
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

    /// Double error detect
    pub mod DED0 {
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

    /// Double error detect
    pub mod DED1 {
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

    /// Double error detect
    pub mod DED2 {
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

    /// Double error detect
    pub mod DED3 {
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
}

/// 8K OTP Memory STATUS Register
pub mod OUT_STATUS_CLR {
    pub use super::OUT_STATUS_SET::ACK;
    pub use super::OUT_STATUS_SET::CALIBRATED;
    pub use super::OUT_STATUS_SET::DED;
    pub use super::OUT_STATUS_SET::DED0;
    pub use super::OUT_STATUS_SET::DED1;
    pub use super::OUT_STATUS_SET::DED2;
    pub use super::OUT_STATUS_SET::DED3;
    pub use super::OUT_STATUS_SET::DED_RELOAD;
    pub use super::OUT_STATUS_SET::FLAGSTATE;
    pub use super::OUT_STATUS_SET::LOCKED;
    pub use super::OUT_STATUS_SET::PROGFAIL;
    pub use super::OUT_STATUS_SET::PWOK;
    pub use super::OUT_STATUS_SET::READ_DONE_INTR;
    pub use super::OUT_STATUS_SET::READ_ERROR_INTR;
    pub use super::OUT_STATUS_SET::SEC;
    pub use super::OUT_STATUS_SET::SEC_RELOAD;
}

/// 8K OTP Memory STATUS Register
pub mod OUT_STATUS_TOG {
    pub use super::OUT_STATUS_SET::ACK;
    pub use super::OUT_STATUS_SET::CALIBRATED;
    pub use super::OUT_STATUS_SET::DED;
    pub use super::OUT_STATUS_SET::DED0;
    pub use super::OUT_STATUS_SET::DED1;
    pub use super::OUT_STATUS_SET::DED2;
    pub use super::OUT_STATUS_SET::DED3;
    pub use super::OUT_STATUS_SET::DED_RELOAD;
    pub use super::OUT_STATUS_SET::FLAGSTATE;
    pub use super::OUT_STATUS_SET::LOCKED;
    pub use super::OUT_STATUS_SET::PROGFAIL;
    pub use super::OUT_STATUS_SET::PWOK;
    pub use super::OUT_STATUS_SET::READ_DONE_INTR;
    pub use super::OUT_STATUS_SET::READ_ERROR_INTR;
    pub use super::OUT_STATUS_SET::SEC;
    pub use super::OUT_STATUS_SET::SEC_RELOAD;
}

/// OTP Controller Version Register
pub mod VERSION {

    /// RTL Version Stepping
    pub mod STEP {
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

    /// Minor RTL Version
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

    /// Major RTL Version
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

/// OTP Controller Read Data 0 Register
pub mod READ_FUSE_DATA0 {
    pub use super::DATA::DATA;
}

/// OTP Controller Read Data 1 Register
pub mod READ_FUSE_DATA1 {
    pub use super::DATA::DATA;
}

/// OTP Controller Read Data 2 Register
pub mod READ_FUSE_DATA2 {
    pub use super::DATA::DATA;
}

/// OTP Controller Read Data 3 Register
pub mod READ_FUSE_DATA3 {
    pub use super::DATA::DATA;
}

/// SW_LOCK Register
pub mod SW_LOCK {

    /// This register contains lock information, which has the same function as the RLOCK fuse words (supplementary fuse words 8 (0x880) and 9 (0x890)) in fuse memory
    pub mod SW_LOCK {
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

/// BIT_LOCK Register
pub mod BIT_LOCK {

    /// Each bit controls the corresponding bit in supplementary fuse word 13 and its shadow register
    pub mod BIT_LOCK {
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

/// OTP Controller Program Locked Status 0 Register
pub mod LOCKED0 {

    /// Stores program locked status for fuse words 0-15.
    pub mod LOCKED {
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

/// OTP Controller Program Locked Status 1 Register
pub mod LOCKED1 {

    /// Stores program locked status for fuse words 16-47
    pub mod LOCKED {
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

/// OTP Controller Program Locked Status 2 Register
pub mod LOCKED2 {
    pub use super::LOCKED1::LOCKED;
}

/// OTP Controller Program Locked Status 3 Register
pub mod LOCKED3 {
    pub use super::LOCKED1::LOCKED;
}

/// OTP Controller Program Locked Status 4 Register
pub mod LOCKED4 {
    pub use super::LOCKED1::LOCKED;
}

/// Value of fuse word index
pub mod FUSE_0 {

    /// Reflects value of the fuse word
    pub mod BITS {
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

/// Value of fuse word index
pub mod FUSE_1 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_2 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_3 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_4 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_5 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_6 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_7 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_8 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_9 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_10 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_11 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_12 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_13 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_14 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_15 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_16 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_17 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_18 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_19 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_20 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_21 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_22 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_23 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_24 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_25 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_26 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_27 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_28 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_29 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_30 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_31 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_32 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_33 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_34 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_35 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_36 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_37 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_38 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_39 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_40 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_41 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_42 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_43 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_44 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_45 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_46 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_47 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_48 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_49 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_50 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_51 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_52 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_53 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_54 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_55 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_56 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_57 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_58 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_59 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_60 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_61 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_62 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_63 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_64 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_65 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_66 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_67 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_68 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_69 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_70 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_71 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_72 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_73 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_74 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_75 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_76 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_77 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_78 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_79 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_80 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_81 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_82 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_83 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_84 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_85 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_86 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_87 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_88 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_89 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_90 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_91 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_92 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_93 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_94 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_95 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_96 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_97 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_98 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_99 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_100 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_101 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_102 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_103 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_104 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_105 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_106 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_107 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_108 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_109 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_110 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_111 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_112 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_113 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_114 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_115 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_116 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_117 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_118 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_119 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_120 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_121 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_122 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_123 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_124 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_125 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_126 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_127 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_128 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_129 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_130 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_131 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_132 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_133 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_134 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_135 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_136 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_137 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_138 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_139 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_140 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_141 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_142 {
    pub use super::FUSE_0::BITS;
}

/// Value of fuse word index
pub mod FUSE_143 {
    pub use super::FUSE_0::BITS;
}
#[repr(C)]
pub struct RegisterBlock {
    /// OTP Controller Control and Status Register
    pub CTRL: RWRegister<u32>,

    /// OTP Controller Control and Status Register
    pub CTRL_SET: RWRegister<u32>,

    /// OTP Controller Control and Status Register
    pub CTRL_CLR: RWRegister<u32>,

    /// OTP Controller Control and Status Register
    pub CTRL_TOG: RWRegister<u32>,

    /// OTP Controller PDN Register
    pub PDN: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// OTP Controller Write Data Register
    pub DATA: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// OTP Controller Read Control Register
    pub READ_CTRL: RWRegister<u32>,

    _reserved3: [u32; 23],

    /// 8K OTP Memory STATUS Register
    pub OUT_STATUS: RWRegister<u32>,

    /// 8K OTP Memory STATUS Register
    pub OUT_STATUS_SET: RWRegister<u32>,

    /// 8K OTP Memory STATUS Register
    pub OUT_STATUS_CLR: RWRegister<u32>,

    /// 8K OTP Memory STATUS Register
    pub OUT_STATUS_TOG: RWRegister<u32>,

    _reserved4: [u32; 4],

    /// OTP Controller Version Register
    pub VERSION: RORegister<u32>,

    _reserved5: [u32; 19],

    /// OTP Controller Read Data 0 Register
    pub READ_FUSE_DATA0: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// OTP Controller Read Data 1 Register
    pub READ_FUSE_DATA1: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// OTP Controller Read Data 2 Register
    pub READ_FUSE_DATA2: RWRegister<u32>,

    _reserved8: [u32; 3],

    /// OTP Controller Read Data 3 Register
    pub READ_FUSE_DATA3: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// SW_LOCK Register
    pub SW_LOCK: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// BIT_LOCK Register
    pub BIT_LOCK: RWRegister<u32>,

    _reserved11: [u32; 299],

    /// OTP Controller Program Locked Status 0 Register
    pub LOCKED0: RORegister<u32>,

    _reserved12: [u32; 3],

    /// OTP Controller Program Locked Status 1 Register
    pub LOCKED1: RORegister<u32>,

    _reserved13: [u32; 3],

    /// OTP Controller Program Locked Status 2 Register
    pub LOCKED2: RORegister<u32>,

    _reserved14: [u32; 3],

    /// OTP Controller Program Locked Status 3 Register
    pub LOCKED3: RORegister<u32>,

    _reserved15: [u32; 3],

    /// OTP Controller Program Locked Status 4 Register
    pub LOCKED4: RORegister<u32>,

    _reserved16: [u32; 111],

    /// Value of fuse word index
    pub FUSE_0: RORegister<u32>,

    _reserved17: [u32; 3],

    /// Value of fuse word index
    pub FUSE_1: RORegister<u32>,

    _reserved18: [u32; 3],

    /// Value of fuse word index
    pub FUSE_2: RORegister<u32>,

    _reserved19: [u32; 3],

    /// Value of fuse word index
    pub FUSE_3: RORegister<u32>,

    _reserved20: [u32; 3],

    /// Value of fuse word index
    pub FUSE_4: RORegister<u32>,

    _reserved21: [u32; 3],

    /// Value of fuse word index
    pub FUSE_5: RORegister<u32>,

    _reserved22: [u32; 3],

    /// Value of fuse word index
    pub FUSE_6: RORegister<u32>,

    _reserved23: [u32; 3],

    /// Value of fuse word index
    pub FUSE_7: RORegister<u32>,

    _reserved24: [u32; 3],

    /// Value of fuse word index
    pub FUSE_8: RORegister<u32>,

    _reserved25: [u32; 3],

    /// Value of fuse word index
    pub FUSE_9: RORegister<u32>,

    _reserved26: [u32; 3],

    /// Value of fuse word index
    pub FUSE_10: RORegister<u32>,

    _reserved27: [u32; 3],

    /// Value of fuse word index
    pub FUSE_11: RORegister<u32>,

    _reserved28: [u32; 3],

    /// Value of fuse word index
    pub FUSE_12: RORegister<u32>,

    _reserved29: [u32; 3],

    /// Value of fuse word index
    pub FUSE_13: RORegister<u32>,

    _reserved30: [u32; 3],

    /// Value of fuse word index
    pub FUSE_14: RORegister<u32>,

    _reserved31: [u32; 3],

    /// Value of fuse word index
    pub FUSE_15: RORegister<u32>,

    _reserved32: [u32; 3],

    /// Value of fuse word index
    pub FUSE_16: RORegister<u32>,

    _reserved33: [u32; 3],

    /// Value of fuse word index
    pub FUSE_17: RORegister<u32>,

    _reserved34: [u32; 3],

    /// Value of fuse word index
    pub FUSE_18: RORegister<u32>,

    _reserved35: [u32; 3],

    /// Value of fuse word index
    pub FUSE_19: RORegister<u32>,

    _reserved36: [u32; 3],

    /// Value of fuse word index
    pub FUSE_20: RORegister<u32>,

    _reserved37: [u32; 3],

    /// Value of fuse word index
    pub FUSE_21: RORegister<u32>,

    _reserved38: [u32; 3],

    /// Value of fuse word index
    pub FUSE_22: RORegister<u32>,

    _reserved39: [u32; 3],

    /// Value of fuse word index
    pub FUSE_23: RORegister<u32>,

    _reserved40: [u32; 3],

    /// Value of fuse word index
    pub FUSE_24: RORegister<u32>,

    _reserved41: [u32; 3],

    /// Value of fuse word index
    pub FUSE_25: RORegister<u32>,

    _reserved42: [u32; 3],

    /// Value of fuse word index
    pub FUSE_26: RORegister<u32>,

    _reserved43: [u32; 3],

    /// Value of fuse word index
    pub FUSE_27: RORegister<u32>,

    _reserved44: [u32; 3],

    /// Value of fuse word index
    pub FUSE_28: RORegister<u32>,

    _reserved45: [u32; 3],

    /// Value of fuse word index
    pub FUSE_29: RORegister<u32>,

    _reserved46: [u32; 3],

    /// Value of fuse word index
    pub FUSE_30: RORegister<u32>,

    _reserved47: [u32; 3],

    /// Value of fuse word index
    pub FUSE_31: RORegister<u32>,

    _reserved48: [u32; 3],

    /// Value of fuse word index
    pub FUSE_32: RORegister<u32>,

    _reserved49: [u32; 3],

    /// Value of fuse word index
    pub FUSE_33: RORegister<u32>,

    _reserved50: [u32; 3],

    /// Value of fuse word index
    pub FUSE_34: RORegister<u32>,

    _reserved51: [u32; 3],

    /// Value of fuse word index
    pub FUSE_35: RORegister<u32>,

    _reserved52: [u32; 3],

    /// Value of fuse word index
    pub FUSE_36: RORegister<u32>,

    _reserved53: [u32; 3],

    /// Value of fuse word index
    pub FUSE_37: RORegister<u32>,

    _reserved54: [u32; 3],

    /// Value of fuse word index
    pub FUSE_38: RORegister<u32>,

    _reserved55: [u32; 3],

    /// Value of fuse word index
    pub FUSE_39: RORegister<u32>,

    _reserved56: [u32; 3],

    /// Value of fuse word index
    pub FUSE_40: RORegister<u32>,

    _reserved57: [u32; 3],

    /// Value of fuse word index
    pub FUSE_41: RORegister<u32>,

    _reserved58: [u32; 3],

    /// Value of fuse word index
    pub FUSE_42: RORegister<u32>,

    _reserved59: [u32; 3],

    /// Value of fuse word index
    pub FUSE_43: RORegister<u32>,

    _reserved60: [u32; 3],

    /// Value of fuse word index
    pub FUSE_44: RORegister<u32>,

    _reserved61: [u32; 3],

    /// Value of fuse word index
    pub FUSE_45: RORegister<u32>,

    _reserved62: [u32; 3],

    /// Value of fuse word index
    pub FUSE_46: RORegister<u32>,

    _reserved63: [u32; 3],

    /// Value of fuse word index
    pub FUSE_47: RORegister<u32>,

    _reserved64: [u32; 3],

    /// Value of fuse word index
    pub FUSE_48: RORegister<u32>,

    _reserved65: [u32; 3],

    /// Value of fuse word index
    pub FUSE_49: RORegister<u32>,

    _reserved66: [u32; 3],

    /// Value of fuse word index
    pub FUSE_50: RORegister<u32>,

    _reserved67: [u32; 3],

    /// Value of fuse word index
    pub FUSE_51: RORegister<u32>,

    _reserved68: [u32; 3],

    /// Value of fuse word index
    pub FUSE_52: RORegister<u32>,

    _reserved69: [u32; 3],

    /// Value of fuse word index
    pub FUSE_53: RORegister<u32>,

    _reserved70: [u32; 3],

    /// Value of fuse word index
    pub FUSE_54: RORegister<u32>,

    _reserved71: [u32; 3],

    /// Value of fuse word index
    pub FUSE_55: RORegister<u32>,

    _reserved72: [u32; 3],

    /// Value of fuse word index
    pub FUSE_56: RORegister<u32>,

    _reserved73: [u32; 3],

    /// Value of fuse word index
    pub FUSE_57: RORegister<u32>,

    _reserved74: [u32; 3],

    /// Value of fuse word index
    pub FUSE_58: RORegister<u32>,

    _reserved75: [u32; 3],

    /// Value of fuse word index
    pub FUSE_59: RORegister<u32>,

    _reserved76: [u32; 3],

    /// Value of fuse word index
    pub FUSE_60: RORegister<u32>,

    _reserved77: [u32; 3],

    /// Value of fuse word index
    pub FUSE_61: RORegister<u32>,

    _reserved78: [u32; 3],

    /// Value of fuse word index
    pub FUSE_62: RORegister<u32>,

    _reserved79: [u32; 3],

    /// Value of fuse word index
    pub FUSE_63: RORegister<u32>,

    _reserved80: [u32; 3],

    /// Value of fuse word index
    pub FUSE_64: RORegister<u32>,

    _reserved81: [u32; 3],

    /// Value of fuse word index
    pub FUSE_65: RORegister<u32>,

    _reserved82: [u32; 3],

    /// Value of fuse word index
    pub FUSE_66: RORegister<u32>,

    _reserved83: [u32; 3],

    /// Value of fuse word index
    pub FUSE_67: RORegister<u32>,

    _reserved84: [u32; 3],

    /// Value of fuse word index
    pub FUSE_68: RORegister<u32>,

    _reserved85: [u32; 3],

    /// Value of fuse word index
    pub FUSE_69: RORegister<u32>,

    _reserved86: [u32; 3],

    /// Value of fuse word index
    pub FUSE_70: RORegister<u32>,

    _reserved87: [u32; 3],

    /// Value of fuse word index
    pub FUSE_71: RORegister<u32>,

    _reserved88: [u32; 3],

    /// Value of fuse word index
    pub FUSE_72: RORegister<u32>,

    _reserved89: [u32; 3],

    /// Value of fuse word index
    pub FUSE_73: RORegister<u32>,

    _reserved90: [u32; 3],

    /// Value of fuse word index
    pub FUSE_74: RORegister<u32>,

    _reserved91: [u32; 3],

    /// Value of fuse word index
    pub FUSE_75: RORegister<u32>,

    _reserved92: [u32; 3],

    /// Value of fuse word index
    pub FUSE_76: RORegister<u32>,

    _reserved93: [u32; 3],

    /// Value of fuse word index
    pub FUSE_77: RORegister<u32>,

    _reserved94: [u32; 3],

    /// Value of fuse word index
    pub FUSE_78: RORegister<u32>,

    _reserved95: [u32; 3],

    /// Value of fuse word index
    pub FUSE_79: RORegister<u32>,

    _reserved96: [u32; 3],

    /// Value of fuse word index
    pub FUSE_80: RORegister<u32>,

    _reserved97: [u32; 3],

    /// Value of fuse word index
    pub FUSE_81: RORegister<u32>,

    _reserved98: [u32; 3],

    /// Value of fuse word index
    pub FUSE_82: RORegister<u32>,

    _reserved99: [u32; 3],

    /// Value of fuse word index
    pub FUSE_83: RORegister<u32>,

    _reserved100: [u32; 3],

    /// Value of fuse word index
    pub FUSE_84: RORegister<u32>,

    _reserved101: [u32; 3],

    /// Value of fuse word index
    pub FUSE_85: RORegister<u32>,

    _reserved102: [u32; 3],

    /// Value of fuse word index
    pub FUSE_86: RORegister<u32>,

    _reserved103: [u32; 3],

    /// Value of fuse word index
    pub FUSE_87: RORegister<u32>,

    _reserved104: [u32; 3],

    /// Value of fuse word index
    pub FUSE_88: RORegister<u32>,

    _reserved105: [u32; 3],

    /// Value of fuse word index
    pub FUSE_89: RORegister<u32>,

    _reserved106: [u32; 3],

    /// Value of fuse word index
    pub FUSE_90: RORegister<u32>,

    _reserved107: [u32; 3],

    /// Value of fuse word index
    pub FUSE_91: RORegister<u32>,

    _reserved108: [u32; 3],

    /// Value of fuse word index
    pub FUSE_92: RORegister<u32>,

    _reserved109: [u32; 3],

    /// Value of fuse word index
    pub FUSE_93: RORegister<u32>,

    _reserved110: [u32; 3],

    /// Value of fuse word index
    pub FUSE_94: RORegister<u32>,

    _reserved111: [u32; 3],

    /// Value of fuse word index
    pub FUSE_95: RORegister<u32>,

    _reserved112: [u32; 3],

    /// Value of fuse word index
    pub FUSE_96: RORegister<u32>,

    _reserved113: [u32; 3],

    /// Value of fuse word index
    pub FUSE_97: RORegister<u32>,

    _reserved114: [u32; 3],

    /// Value of fuse word index
    pub FUSE_98: RORegister<u32>,

    _reserved115: [u32; 3],

    /// Value of fuse word index
    pub FUSE_99: RORegister<u32>,

    _reserved116: [u32; 3],

    /// Value of fuse word index
    pub FUSE_100: RORegister<u32>,

    _reserved117: [u32; 3],

    /// Value of fuse word index
    pub FUSE_101: RORegister<u32>,

    _reserved118: [u32; 3],

    /// Value of fuse word index
    pub FUSE_102: RORegister<u32>,

    _reserved119: [u32; 3],

    /// Value of fuse word index
    pub FUSE_103: RORegister<u32>,

    _reserved120: [u32; 3],

    /// Value of fuse word index
    pub FUSE_104: RORegister<u32>,

    _reserved121: [u32; 3],

    /// Value of fuse word index
    pub FUSE_105: RORegister<u32>,

    _reserved122: [u32; 3],

    /// Value of fuse word index
    pub FUSE_106: RORegister<u32>,

    _reserved123: [u32; 3],

    /// Value of fuse word index
    pub FUSE_107: RORegister<u32>,

    _reserved124: [u32; 3],

    /// Value of fuse word index
    pub FUSE_108: RORegister<u32>,

    _reserved125: [u32; 3],

    /// Value of fuse word index
    pub FUSE_109: RORegister<u32>,

    _reserved126: [u32; 3],

    /// Value of fuse word index
    pub FUSE_110: RORegister<u32>,

    _reserved127: [u32; 3],

    /// Value of fuse word index
    pub FUSE_111: RORegister<u32>,

    _reserved128: [u32; 3],

    /// Value of fuse word index
    pub FUSE_112: RORegister<u32>,

    _reserved129: [u32; 3],

    /// Value of fuse word index
    pub FUSE_113: RORegister<u32>,

    _reserved130: [u32; 3],

    /// Value of fuse word index
    pub FUSE_114: RORegister<u32>,

    _reserved131: [u32; 3],

    /// Value of fuse word index
    pub FUSE_115: RORegister<u32>,

    _reserved132: [u32; 3],

    /// Value of fuse word index
    pub FUSE_116: RORegister<u32>,

    _reserved133: [u32; 3],

    /// Value of fuse word index
    pub FUSE_117: RORegister<u32>,

    _reserved134: [u32; 3],

    /// Value of fuse word index
    pub FUSE_118: RORegister<u32>,

    _reserved135: [u32; 3],

    /// Value of fuse word index
    pub FUSE_119: RORegister<u32>,

    _reserved136: [u32; 3],

    /// Value of fuse word index
    pub FUSE_120: RORegister<u32>,

    _reserved137: [u32; 3],

    /// Value of fuse word index
    pub FUSE_121: RORegister<u32>,

    _reserved138: [u32; 3],

    /// Value of fuse word index
    pub FUSE_122: RORegister<u32>,

    _reserved139: [u32; 3],

    /// Value of fuse word index
    pub FUSE_123: RORegister<u32>,

    _reserved140: [u32; 3],

    /// Value of fuse word index
    pub FUSE_124: RORegister<u32>,

    _reserved141: [u32; 3],

    /// Value of fuse word index
    pub FUSE_125: RORegister<u32>,

    _reserved142: [u32; 3],

    /// Value of fuse word index
    pub FUSE_126: RORegister<u32>,

    _reserved143: [u32; 3],

    /// Value of fuse word index
    pub FUSE_127: RORegister<u32>,

    _reserved144: [u32; 3],

    /// Value of fuse word index
    pub FUSE_128: RORegister<u32>,

    _reserved145: [u32; 3],

    /// Value of fuse word index
    pub FUSE_129: RORegister<u32>,

    _reserved146: [u32; 3],

    /// Value of fuse word index
    pub FUSE_130: RORegister<u32>,

    _reserved147: [u32; 3],

    /// Value of fuse word index
    pub FUSE_131: RORegister<u32>,

    _reserved148: [u32; 3],

    /// Value of fuse word index
    pub FUSE_132: RORegister<u32>,

    _reserved149: [u32; 3],

    /// Value of fuse word index
    pub FUSE_133: RORegister<u32>,

    _reserved150: [u32; 3],

    /// Value of fuse word index
    pub FUSE_134: RORegister<u32>,

    _reserved151: [u32; 3],

    /// Value of fuse word index
    pub FUSE_135: RORegister<u32>,

    _reserved152: [u32; 3],

    /// Value of fuse word index
    pub FUSE_136: RORegister<u32>,

    _reserved153: [u32; 3],

    /// Value of fuse word index
    pub FUSE_137: RORegister<u32>,

    _reserved154: [u32; 3],

    /// Value of fuse word index
    pub FUSE_138: RORegister<u32>,

    _reserved155: [u32; 3],

    /// Value of fuse word index
    pub FUSE_139: RORegister<u32>,

    _reserved156: [u32; 3],

    /// Value of fuse word index
    pub FUSE_140: RORegister<u32>,

    _reserved157: [u32; 3],

    /// Value of fuse word index
    pub FUSE_141: RORegister<u32>,

    _reserved158: [u32; 3],

    /// Value of fuse word index
    pub FUSE_142: RORegister<u32>,

    _reserved159: [u32; 3],

    /// Value of fuse word index
    pub FUSE_143: RORegister<u32>,
}
pub struct ResetValues {
    pub CTRL: u32,
    pub CTRL_SET: u32,
    pub CTRL_CLR: u32,
    pub CTRL_TOG: u32,
    pub PDN: u32,
    pub DATA: u32,
    pub READ_CTRL: u32,
    pub OUT_STATUS: u32,
    pub OUT_STATUS_SET: u32,
    pub OUT_STATUS_CLR: u32,
    pub OUT_STATUS_TOG: u32,
    pub VERSION: u32,
    pub READ_FUSE_DATA0: u32,
    pub READ_FUSE_DATA1: u32,
    pub READ_FUSE_DATA2: u32,
    pub READ_FUSE_DATA3: u32,
    pub SW_LOCK: u32,
    pub BIT_LOCK: u32,
    pub LOCKED0: u32,
    pub LOCKED1: u32,
    pub LOCKED2: u32,
    pub LOCKED3: u32,
    pub LOCKED4: u32,
    pub FUSE_0: u32,
    pub FUSE_1: u32,
    pub FUSE_2: u32,
    pub FUSE_3: u32,
    pub FUSE_4: u32,
    pub FUSE_5: u32,
    pub FUSE_6: u32,
    pub FUSE_7: u32,
    pub FUSE_8: u32,
    pub FUSE_9: u32,
    pub FUSE_10: u32,
    pub FUSE_11: u32,
    pub FUSE_12: u32,
    pub FUSE_13: u32,
    pub FUSE_14: u32,
    pub FUSE_15: u32,
    pub FUSE_16: u32,
    pub FUSE_17: u32,
    pub FUSE_18: u32,
    pub FUSE_19: u32,
    pub FUSE_20: u32,
    pub FUSE_21: u32,
    pub FUSE_22: u32,
    pub FUSE_23: u32,
    pub FUSE_24: u32,
    pub FUSE_25: u32,
    pub FUSE_26: u32,
    pub FUSE_27: u32,
    pub FUSE_28: u32,
    pub FUSE_29: u32,
    pub FUSE_30: u32,
    pub FUSE_31: u32,
    pub FUSE_32: u32,
    pub FUSE_33: u32,
    pub FUSE_34: u32,
    pub FUSE_35: u32,
    pub FUSE_36: u32,
    pub FUSE_37: u32,
    pub FUSE_38: u32,
    pub FUSE_39: u32,
    pub FUSE_40: u32,
    pub FUSE_41: u32,
    pub FUSE_42: u32,
    pub FUSE_43: u32,
    pub FUSE_44: u32,
    pub FUSE_45: u32,
    pub FUSE_46: u32,
    pub FUSE_47: u32,
    pub FUSE_48: u32,
    pub FUSE_49: u32,
    pub FUSE_50: u32,
    pub FUSE_51: u32,
    pub FUSE_52: u32,
    pub FUSE_53: u32,
    pub FUSE_54: u32,
    pub FUSE_55: u32,
    pub FUSE_56: u32,
    pub FUSE_57: u32,
    pub FUSE_58: u32,
    pub FUSE_59: u32,
    pub FUSE_60: u32,
    pub FUSE_61: u32,
    pub FUSE_62: u32,
    pub FUSE_63: u32,
    pub FUSE_64: u32,
    pub FUSE_65: u32,
    pub FUSE_66: u32,
    pub FUSE_67: u32,
    pub FUSE_68: u32,
    pub FUSE_69: u32,
    pub FUSE_70: u32,
    pub FUSE_71: u32,
    pub FUSE_72: u32,
    pub FUSE_73: u32,
    pub FUSE_74: u32,
    pub FUSE_75: u32,
    pub FUSE_76: u32,
    pub FUSE_77: u32,
    pub FUSE_78: u32,
    pub FUSE_79: u32,
    pub FUSE_80: u32,
    pub FUSE_81: u32,
    pub FUSE_82: u32,
    pub FUSE_83: u32,
    pub FUSE_84: u32,
    pub FUSE_85: u32,
    pub FUSE_86: u32,
    pub FUSE_87: u32,
    pub FUSE_88: u32,
    pub FUSE_89: u32,
    pub FUSE_90: u32,
    pub FUSE_91: u32,
    pub FUSE_92: u32,
    pub FUSE_93: u32,
    pub FUSE_94: u32,
    pub FUSE_95: u32,
    pub FUSE_96: u32,
    pub FUSE_97: u32,
    pub FUSE_98: u32,
    pub FUSE_99: u32,
    pub FUSE_100: u32,
    pub FUSE_101: u32,
    pub FUSE_102: u32,
    pub FUSE_103: u32,
    pub FUSE_104: u32,
    pub FUSE_105: u32,
    pub FUSE_106: u32,
    pub FUSE_107: u32,
    pub FUSE_108: u32,
    pub FUSE_109: u32,
    pub FUSE_110: u32,
    pub FUSE_111: u32,
    pub FUSE_112: u32,
    pub FUSE_113: u32,
    pub FUSE_114: u32,
    pub FUSE_115: u32,
    pub FUSE_116: u32,
    pub FUSE_117: u32,
    pub FUSE_118: u32,
    pub FUSE_119: u32,
    pub FUSE_120: u32,
    pub FUSE_121: u32,
    pub FUSE_122: u32,
    pub FUSE_123: u32,
    pub FUSE_124: u32,
    pub FUSE_125: u32,
    pub FUSE_126: u32,
    pub FUSE_127: u32,
    pub FUSE_128: u32,
    pub FUSE_129: u32,
    pub FUSE_130: u32,
    pub FUSE_131: u32,
    pub FUSE_132: u32,
    pub FUSE_133: u32,
    pub FUSE_134: u32,
    pub FUSE_135: u32,
    pub FUSE_136: u32,
    pub FUSE_137: u32,
    pub FUSE_138: u32,
    pub FUSE_139: u32,
    pub FUSE_140: u32,
    pub FUSE_141: u32,
    pub FUSE_142: u32,
    pub FUSE_143: u32,
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
