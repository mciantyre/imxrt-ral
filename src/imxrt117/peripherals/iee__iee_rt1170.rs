#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IEE
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister, WORegister};

/// IEE Global Configuration
pub mod GCFG {

    /// Region lock 0 bit
    pub mod RL0 {
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

            /// 0b0: Unlocked.
            pub const RL0_0: u32 = 0b0;

            /// 0b1: Key, Offset and Attribute registers are locked.
            pub const RL0_1: u32 = 0b1;
        }
    }

    /// Region lock 1 bit
    pub mod RL1 {
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

            /// 0b0: Unlocked.
            pub const RL1_0: u32 = 0b0;

            /// 0b1: Key, Offset and Attribute registers are locked.
            pub const RL1_1: u32 = 0b1;
        }
    }

    /// Region lock 2 bit
    pub mod RL2 {
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

            /// 0b0: Unlocked.
            pub const RL2_0: u32 = 0b0;

            /// 0b1: Key, Offset and Attribute registers are locked.
            pub const RL2_1: u32 = 0b1;
        }
    }

    /// Region lock 3 bit
    pub mod RL3 {
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

            /// 0b0: Unlocked.
            pub const RL3_0: u32 = 0b0;

            /// 0b1: Key, Offset and Attribute registers are locked.
            pub const RL3_1: u32 = 0b1;
        }
    }

    /// Region lock 4 bit
    pub mod RL4 {
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

            /// 0b0: Unlocked.
            pub const RL4_0: u32 = 0b0;

            /// 0b1: Key, Offset and Attribute registers are locked.
            pub const RL4_1: u32 = 0b1;
        }
    }

    /// Region lock 5 bit
    pub mod RL5 {
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

            /// 0b0: Unlocked.
            pub const RL5_0: u32 = 0b0;

            /// 0b1: Key, Offset and Attribute registers are locked.
            pub const RL5_1: u32 = 0b1;
        }
    }

    /// Region lock 6 bit
    pub mod RL6 {
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

            /// 0b0: Unlocked.
            pub const RL6_0: u32 = 0b0;

            /// 0b1: Key, Offset and Attribute registers are locked.
            pub const RL6_1: u32 = 0b1;
        }
    }

    /// Region lock 7 bit
    pub mod RL7 {
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

            /// 0b0: Unlocked.
            pub const RL7_0: u32 = 0b0;

            /// 0b1: Key, Offset and Attribute registers are locked.
            pub const RL7_1: u32 = 0b1;
        }
    }

    /// Test mode enable bit
    pub mod TME {
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

            /// 0b0: Disabled.
            pub const TME_0: u32 = 0b0;

            /// 0b1: Enabled.
            pub const TME_1: u32 = 0b1;
        }
    }

    /// Test mode disable bit
    pub mod TMD {
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

            /// 0b0: Test mode is usable.
            pub const TMD_0: u32 = 0b0;

            /// 0b1: Test mode is disabled.
            pub const TMD_1: u32 = 0b1;
        }
    }

    /// Key read disable bit
    pub mod KEY_RD_DIS {
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

            /// 0b0: Key read enabled. Reading the key registers is allowed.
            pub const KEY_RD_DIS_0: u32 = 0b0;

            /// 0b1: Key read disabled. Reading the key registers is disabled.
            pub const KEY_RD_DIS_1: u32 = 0b1;
        }
    }

    /// Monitor enable bit
    pub mod MON_EN {
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

            /// 0b0: Performance monitoring disabled. Writing of the performance counter registers is enabled.
            pub const MON_EN_0: u32 = 0b0;

            /// 0b1: Performance monitoring enabled. Writing of the performance counter registers is disabled.
            pub const MON_EN_1: u32 = 0b1;
        }
    }

    /// Clear monitor bit
    pub mod CLR_MON {
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

            /// 0b0: Do not reset.
            pub const CLR_MON_0: u32 = 0b0;

            /// 0b1: Reset performance counters.
            pub const CLR_MON_1: u32 = 0b1;
        }
    }

    /// Reset bit
    pub mod RST {
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

            /// 0b0: Do Not Reset.
            pub const RST_0: u32 = 0b0;

            /// 0b1: Reset IEE.
            pub const RST_1: u32 = 0b1;
        }
    }
}

/// IEE Status
pub mod STA {

    /// DPA seed request bit
    pub mod DSR {
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

            /// 0b0: No seed request present
            pub const DSR_0: u32 = 0b0;

            /// 0b1: Seed request present
            pub const DSR_1: u32 = 0b1;
        }
    }

    /// AES fault detected bit
    pub mod AFD {
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

            /// 0b0: No fault detected
            pub const AFD_0: u32 = 0b0;

            /// 0b1: Fault detected
            pub const AFD_1: u32 = 0b1;
        }
    }
}

/// IEE Test Mode Register
pub mod TSTMD {

    /// Test mode ready bit. All AXI transactions have stopped and test can begin.
    pub mod TMRDY {
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

            /// 0b0: Not Ready.
            pub const TMRDY_0: u32 = 0b0;

            /// 0b1: Ready.
            pub const TMRDY_1: u32 = 0b1;
        }
    }

    /// Test mode run bit
    pub mod TMR {
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

            /// 0b0: Not running. May be written if IEE_GCFG\[TME\] = 1
            pub const TMR_0: u32 = 0b0;

            /// 0b1: Run AES Test until TMDONE is indicated.
            pub const TMR_1: u32 = 0b1;
        }
    }

    /// Test mode encrypt/decrypt bit.
    pub mod TMENCR {
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

            /// 0b0: AES Test mode will do decryption.
            pub const TMENCR_0: u32 = 0b0;

            /// 0b1: AES Test mode will do encryption.
            pub const TMENCR_1: u32 = 0b1;
        }
    }

    /// Test mode continue bit. Set to indicate that operation will be followed by more data.
    pub mod TMCONT {
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

            /// 0b0: Do not continue. This is the last block of data for AES.
            pub const TMCONT_0: u32 = 0b0;

            /// 0b1: Continue. Do not initialize AES after this block.
            pub const TMCONT_1: u32 = 0b1;
        }
    }

    /// Test mode done bit
    pub mod TMDONE {
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

            /// 0b0: Not Done.
            pub const TMDONE_0: u32 = 0b0;

            /// 0b1: Test Done.
            pub const TMDONE_1: u32 = 0b1;
        }
    }

    /// Test mode length field
    pub mod TMLEN {
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

/// AES Mask Generation Seed
pub mod DPAMS {

    /// DPA mask seed
    pub mod DPAMS {
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

/// Performance Counter, AES Slave Latency Threshold Value
pub mod PC_S_LT {

    /// Slave write latency threshold in AXI clock cycles.
    pub mod SW_LT {
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

    /// Slave read latency threshold in AXI clock cycles.
    pub mod SR_LT {
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

/// Performance Counter, AES Master Latency Threshold
pub mod PC_M_LT {

    /// Master write latency threshold in AXI clock cycles.
    pub mod MW_LT {
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

    /// Master read latency threshold in AXI clock cycles.
    pub mod MR_LT {
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

/// Performance Counter, Number of AES Block Encryptions
pub mod PC_BLK_ENC {

    /// Number of AES block encryptions. Does not roll over if value maxes out.
    pub mod BLK_ENC {
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

/// Performance Counter, Number of AES Block Decryptions
pub mod PC_BLK_DEC {

    /// Number of AES block decryptions. Does not roll over if value maxes out.
    pub mod BLK_DEC {
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

/// Performance Counter, Number of AXI Slave Read Transactions
pub mod PC_SR_TRANS {

    /// Number of slave read transactions.
    pub mod SR_TRANS {
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

/// Performance Counter, Number of AXI Slave Write Transactions
pub mod PC_SW_TRANS {

    /// Number of slave write transactions.
    pub mod SW_TRANS {
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

/// Performance Counter, Number of AXI Master Read Transactions
pub mod PC_MR_TRANS {

    /// Number of master read transactions.
    pub mod MR_TRANS {
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

/// Performance Counter, Number of AXI Master Write Transactions
pub mod PC_MW_TRANS {

    /// Number of master write transactions.
    pub mod MW_TRANS {
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

/// Performance Counter, Number of AXI Master Merge Buffer Read Transactions
pub mod PC_M_MBR {

    /// Number of master merge buffer read transactions.
    pub mod M_MBR {
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

/// Performance Counter, Upper Slave Read Transactions Byte Count
pub mod PC_SR_TBC_U {

    /// Number of bytes in slave read transactions. Upper 16 bits of SR_TBC\[47:0\].
    pub mod SR_TBC {
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

/// Performance Counter, Lower Slave Read Transactions Byte Count
pub mod PC_SR_TBC_L {

    /// Number of bytes in slave read transactions. Lower 32 bits of SR_TBC\[47:0\].
    pub mod SR_TBC {
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

/// Performance Counter, Upper Slave Write Transactions Byte Count
pub mod PC_SW_TBC_U {

    /// Number of bytes in slave write transactions. Upper 16 bits of SW_TBC\[47:0\].
    pub mod SW_TBC {
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

/// Performance Counter, Lower Slave Write Transactions Byte Count
pub mod PC_SW_TBC_L {

    /// Number of bytes in slave write transactions. Lower 32 bits of SW_TBC\[47:0\].
    pub mod SW_TBC {
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

/// Performance Counter, Upper Master Read Transactions Byte Count
pub mod PC_MR_TBC_U {

    /// Number of bytes in master read transactions. 44 MSBs. Upper 16 bits of MR_TBC\[43:0\].
    pub mod MR_TBC {
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

/// Performance Counter, Lower Master Read Transactions Byte Count
pub mod PC_MR_TBC_L {

    /// Number of bytes in master read transactions. 4 LSBs, always 0.
    pub mod MR_TBC_LSB {
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

    /// Number of bytes in master read transactions. 44 MSBs. Lower 28 bits of MR_TBC\[43:0\].
    pub mod MR_TBC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (28 bits: 0xfffffff << 4)
        pub const mask: u32 = 0xfffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Performance Counter, Upper Master Write Transactions Byte Count
pub mod PC_MW_TBC_U {

    /// Number of bytes in master write transactions. 44 MSBs. Upper 16 bits of MW_TBC\[43:0\].
    pub mod MW_TBC {
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

/// Performance Counter, Lower Master Write Transactions Byte Count
pub mod PC_MW_TBC_L {

    /// Number of bytes in master write transactions. 4 LSBs, always 0.
    pub mod MW_TBC_LSB {
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

    /// Number of bytes in master write transactions. 44 MSBs. Lower 28 bits of MR_TBC\[43:0\].
    pub mod MW_TBC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (28 bits: 0xfffffff << 4)
        pub const mask: u32 = 0xfffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Performance Counter, Number of AXI Slave Read Transactions with Latency Greater than the Threshold
pub mod PC_SR_TLGTT {

    /// Number of slave read transactions with latency greater than the threshold.
    pub mod SR_TLGTT {
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

/// Performance Counter, Number of AXI Slave Write Transactions with Latency Greater than the Threshold
pub mod PC_SW_TLGTT {

    /// Number of slave write transactions with latency greater than the threshold.
    pub mod SW_TLGTT {
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

/// Performance Counter, Number of AXI Master Read Transactions with Latency Greater than the Threshold
pub mod PC_MR_TLGTT {

    /// Number of master read transactions with latency greater than the threshold.
    pub mod MR_TLGTT {
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

/// Performance Counter, Number of AXI Master Write Transactions with Latency Greater than the Threshold
pub mod PC_MW_TLGTT {

    /// Number of master write transactions with latency greater than the threshold.
    pub mod MW_TGTT {
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

/// Performance Counter, Upper Slave Read Latency Count
pub mod PC_SR_TLAT_U {

    /// Total slave read latency in AXI clock cycles. Upper 16 bits of SR_TLAT\[47:0\].
    pub mod SR_TLAT {
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

/// Performance Counter, Lower Slave Read Latency Count
pub mod PC_SR_TLAT_L {

    /// Total slave read latency in AXI clock cycles. Lower 32 bits of SR_TLAT\[47:0\].
    pub mod SR_TLAT {
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

/// Performance Counter, Upper Slave Write Latency Count
pub mod PC_SW_TLAT_U {

    /// Total slave write latency in AXI clock cycles. Upper 16 bits of SW_TLAT\[47:0\].
    pub mod SW_TLAT {
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

/// Performance Counter, Lower Slave Write Latency Count
pub mod PC_SW_TLAT_L {

    /// Total slave write latency in AXI clock cycles. Lower 32 bits of SW_TLAT\[47:0\].
    pub mod SW_TLAT {
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

/// Performance Counter, Upper Master Read Latency Count
pub mod PC_MR_TLAT_U {

    /// Total master read latency in AXI clock cycles. Upper 16 bits of MR_TLAT\[47:0\].
    pub mod MR_TLAT {
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

/// Performance Counter, Lower Master Read Latency Count
pub mod PC_MR_TLAT_L {

    /// Total master read latency in AXI clock cycles. Lower 32 bits of MR_TLAT\[47:0\].
    pub mod MR_TLAT {
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

/// Performance Counter, Upper Master Write Latency Count
pub mod PC_MW_TLAT_U {

    /// Total master write latency in AXI clock cycles. Upper 16 bits of MW_TLAT\[47:0\].
    pub mod MW_TLAT {
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

/// Performance Counter, Lower Master Write Latency Count
pub mod PC_MW_TLAT_L {

    /// Total master write latency in AXI clock cycles. Lower 32 bits of MW_TLAT\[47:0\].
    pub mod MW_TLAT {
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

/// Performance Counter, Upper Slave Read Total Non-Responding Time
pub mod PC_SR_TNRT_U {

    /// Total slave read non-responding time in AXI clock cycles. Upper 16 bits of SR_TNRT\[47:0\].
    pub mod SR_TNRT {
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

/// Performance Counter, Lower Slave Read Total Non-Responding Time
pub mod PC_SR_TNRT_L {

    /// Total slave read non-responding time in AXI clock cycles. Lower 32 bits of SR_TNRT\[47:0\].
    pub mod SR_TNRT {
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

/// Performance Counter, Upper Slave Write Total Non-Responding Time
pub mod PC_SW_TNRT_U {

    /// Total slave write non-responding time in AXI clock cycles. Upper 16 bits of SW_TNRT\[47:0\].
    pub mod SW_TNRT {
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

/// Performance Counter, Lower Slave Write Total Non-Responding Time
pub mod PC_SW_TNRT_L {

    /// Total slave write non-responding time in AXI clock cycles. Lower 32 bits of SW_TNRT\[47:0\].
    pub mod SW_TNRT {
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

/// IEE Version ID Register 1
pub mod VIDR1 {

    /// Minor revision number for IEE.
    pub mod MIN_REV {
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

    /// Major revision number for IEE.
    pub mod MAJ_REV {
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

    /// ID for IEE.
    pub mod IP_ID {
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

/// IEE AES Version ID Register
pub mod AESVID {

    /// AES revision number.
    pub mod AESRN {
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

    /// AES version ID.
    pub mod AESVID {
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

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB0 {

    /// AES test mode data buffer.
    pub mod AES_TST_DB0 {
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

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB1 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB2 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB3 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB4 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB5 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB6 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB7 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB8 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB9 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB10 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB11 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB12 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB13 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB14 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB15 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB16 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB17 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB18 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB19 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB20 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB21 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB22 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB23 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB24 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB25 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB26 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB27 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB28 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB29 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB30 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE AES Test Mode Data Buffer
pub mod AES_TST_DB31 {
    pub use super::AES_TST_DB0::AES_TST_DB0;
}

/// IEE Region REGION Attribute Register.
pub mod REGATTR_0 {

    /// AES key size.
    pub mod KS {
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

            /// 0b0: 128 bits (CTR), 256 bits (XTS).
            pub const KS_0: u32 = 0b0;

            /// 0b1: 256 bits (CTR), 512 bits (XTS).
            pub const KS_1: u32 = 0b1;
        }
    }

    /// AES Mode.
    pub mod MD {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: None (AXI error if accessed)
            pub const MD_0: u32 = 0b000;

            /// 0b001: XTS
            pub const MD_1: u32 = 0b001;

            /// 0b010: CTR w/ address binding
            pub const MD_2: u32 = 0b010;

            /// 0b011: CTR w/o address binding
            pub const MD_3: u32 = 0b011;

            /// 0b100: CTR keystream only
            pub const MD_4: u32 = 0b100;

            /// 0b101: Undefined, AXI error if used
            pub const MD_5: u32 = 0b101;

            /// 0b110: Undefined, AXI error if used
            pub const MD_6: u32 = 0b110;

            /// 0b111: Undefined, AXI error if used
            pub const MD_7: u32 = 0b111;
        }
    }

    /// AES Bypass.
    pub mod BYP {
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

            /// 0b0: use MD field
            pub const BYP_0: u32 = 0b0;

            /// 0b1: Bypass AES, no encrypt/decrypt
            pub const BYP_1: u32 = 0b1;
        }
    }
}

/// IEE Region REGION Page Offset Register
pub mod REGPO_0 {

    /// This field represents a 4Kb page offset
    pub mod PGOFF {
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

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_0_0 {

    /// Key 1.
    pub mod KEY1 {
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

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_1_0 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_2_0 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_3_0 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_4_0 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_5_0 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_6_0 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_7_0 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_0_0 {

    /// Key 2.
    pub mod KEY2 {
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

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_1_0 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_2_0 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_3_0 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_4_0 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_5_0 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_6_0 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_7_0 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Attribute Register.
pub mod REGATTR_1 {
    pub use super::REGATTR_0::BYP;
    pub use super::REGATTR_0::KS;
    pub use super::REGATTR_0::MD;
}

/// IEE Region REGION Page Offset Register
pub mod REGPO_1 {
    pub use super::REGPO_0::PGOFF;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_0_1 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_1_1 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_2_1 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_3_1 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_4_1 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_5_1 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_6_1 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_7_1 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_0_1 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_1_1 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_2_1 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_3_1 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_4_1 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_5_1 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_6_1 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_7_1 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Attribute Register.
pub mod REGATTR_2 {
    pub use super::REGATTR_0::BYP;
    pub use super::REGATTR_0::KS;
    pub use super::REGATTR_0::MD;
}

/// IEE Region REGION Page Offset Register
pub mod REGPO_2 {
    pub use super::REGPO_0::PGOFF;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_0_2 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_1_2 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_2_2 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_3_2 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_4_2 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_5_2 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_6_2 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_7_2 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_0_2 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_1_2 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_2_2 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_3_2 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_4_2 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_5_2 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_6_2 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_7_2 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Attribute Register.
pub mod REGATTR_3 {
    pub use super::REGATTR_0::BYP;
    pub use super::REGATTR_0::KS;
    pub use super::REGATTR_0::MD;
}

/// IEE Region REGION Page Offset Register
pub mod REGPO_3 {
    pub use super::REGPO_0::PGOFF;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_0_3 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_1_3 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_2_3 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_3_3 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_4_3 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_5_3 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_6_3 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_7_3 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_0_3 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_1_3 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_2_3 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_3_3 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_4_3 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_5_3 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_6_3 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_7_3 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Attribute Register.
pub mod REGATTR_4 {
    pub use super::REGATTR_0::BYP;
    pub use super::REGATTR_0::KS;
    pub use super::REGATTR_0::MD;
}

/// IEE Region REGION Page Offset Register
pub mod REGPO_4 {
    pub use super::REGPO_0::PGOFF;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_0_4 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_1_4 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_2_4 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_3_4 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_4_4 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_5_4 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_6_4 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_7_4 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_0_4 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_1_4 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_2_4 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_3_4 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_4_4 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_5_4 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_6_4 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_7_4 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Attribute Register.
pub mod REGATTR_5 {
    pub use super::REGATTR_0::BYP;
    pub use super::REGATTR_0::KS;
    pub use super::REGATTR_0::MD;
}

/// IEE Region REGION Page Offset Register
pub mod REGPO_5 {
    pub use super::REGPO_0::PGOFF;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_0_5 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_1_5 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_2_5 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_3_5 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_4_5 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_5_5 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_6_5 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_7_5 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_0_5 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_1_5 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_2_5 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_3_5 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_4_5 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_5_5 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_6_5 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_7_5 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Attribute Register.
pub mod REGATTR_6 {
    pub use super::REGATTR_0::BYP;
    pub use super::REGATTR_0::KS;
    pub use super::REGATTR_0::MD;
}

/// IEE Region REGION Page Offset Register
pub mod REGPO_6 {
    pub use super::REGPO_0::PGOFF;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_0_6 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_1_6 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_2_6 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_3_6 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_4_6 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_5_6 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_6_6 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_7_6 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_0_6 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_1_6 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_2_6 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_3_6 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_4_6 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_5_6 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_6_6 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_7_6 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Attribute Register.
pub mod REGATTR_7 {
    pub use super::REGATTR_0::BYP;
    pub use super::REGATTR_0::KS;
    pub use super::REGATTR_0::MD;
}

/// IEE Region REGION Page Offset Register
pub mod REGPO_7 {
    pub use super::REGPO_0::PGOFF;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_0_7 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_1_7 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_2_7 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_3_7 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_4_7 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_5_7 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_6_7 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 1 Register
pub mod REGKEY1_7_7 {
    pub use super::REGKEY1_0_0::KEY1;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_0_7 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_1_7 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_2_7 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_3_7 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_4_7 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_5_7 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_6_7 {
    pub use super::REGKEY2_0_0::KEY2;
}

/// IEE Region REGION Key 2 Register
pub mod REGKEY2_7_7 {
    pub use super::REGKEY2_0_0::KEY2;
}
#[repr(C)]
pub struct RegisterBlock {
    /// IEE Global Configuration
    pub GCFG: RWRegister<u32>,

    /// IEE Status
    pub STA: RORegister<u32>,

    /// IEE Test Mode Register
    pub TSTMD: RWRegister<u32>,

    /// AES Mask Generation Seed
    pub DPAMS: WORegister<u32>,

    _reserved1: [u32; 4],

    /// Performance Counter, AES Slave Latency Threshold Value
    pub PC_S_LT: RWRegister<u32>,

    /// Performance Counter, AES Master Latency Threshold
    pub PC_M_LT: RWRegister<u32>,

    _reserved2: [u32; 6],

    /// Performance Counter, Number of AES Block Encryptions
    pub PC_BLK_ENC: RWRegister<u32>,

    /// Performance Counter, Number of AES Block Decryptions
    pub PC_BLK_DEC: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// Performance Counter, Number of AXI Slave Read Transactions
    pub PC_SR_TRANS: RWRegister<u32>,

    /// Performance Counter, Number of AXI Slave Write Transactions
    pub PC_SW_TRANS: RWRegister<u32>,

    /// Performance Counter, Number of AXI Master Read Transactions
    pub PC_MR_TRANS: RWRegister<u32>,

    /// Performance Counter, Number of AXI Master Write Transactions
    pub PC_MW_TRANS: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// Performance Counter, Number of AXI Master Merge Buffer Read Transactions
    pub PC_M_MBR: RWRegister<u32>,

    _reserved5: [u32; 2],

    /// Performance Counter, Upper Slave Read Transactions Byte Count
    pub PC_SR_TBC_U: RWRegister<u32>,

    /// Performance Counter, Lower Slave Read Transactions Byte Count
    pub PC_SR_TBC_L: RWRegister<u32>,

    /// Performance Counter, Upper Slave Write Transactions Byte Count
    pub PC_SW_TBC_U: RWRegister<u32>,

    /// Performance Counter, Lower Slave Write Transactions Byte Count
    pub PC_SW_TBC_L: RWRegister<u32>,

    /// Performance Counter, Upper Master Read Transactions Byte Count
    pub PC_MR_TBC_U: RWRegister<u32>,

    /// Performance Counter, Lower Master Read Transactions Byte Count
    pub PC_MR_TBC_L: RWRegister<u32>,

    /// Performance Counter, Upper Master Write Transactions Byte Count
    pub PC_MW_TBC_U: RWRegister<u32>,

    /// Performance Counter, Lower Master Write Transactions Byte Count
    pub PC_MW_TBC_L: RWRegister<u32>,

    /// Performance Counter, Number of AXI Slave Read Transactions with Latency Greater than the Threshold
    pub PC_SR_TLGTT: RWRegister<u32>,

    /// Performance Counter, Number of AXI Slave Write Transactions with Latency Greater than the Threshold
    pub PC_SW_TLGTT: RWRegister<u32>,

    /// Performance Counter, Number of AXI Master Read Transactions with Latency Greater than the Threshold
    pub PC_MR_TLGTT: RWRegister<u32>,

    /// Performance Counter, Number of AXI Master Write Transactions with Latency Greater than the Threshold
    pub PC_MW_TLGTT: RWRegister<u32>,

    /// Performance Counter, Upper Slave Read Latency Count
    pub PC_SR_TLAT_U: RWRegister<u32>,

    /// Performance Counter, Lower Slave Read Latency Count
    pub PC_SR_TLAT_L: RWRegister<u32>,

    /// Performance Counter, Upper Slave Write Latency Count
    pub PC_SW_TLAT_U: RWRegister<u32>,

    /// Performance Counter, Lower Slave Write Latency Count
    pub PC_SW_TLAT_L: RWRegister<u32>,

    /// Performance Counter, Upper Master Read Latency Count
    pub PC_MR_TLAT_U: RWRegister<u32>,

    /// Performance Counter, Lower Master Read Latency Count
    pub PC_MR_TLAT_L: RWRegister<u32>,

    /// Performance Counter, Upper Master Write Latency Count
    pub PC_MW_TLAT_U: RWRegister<u32>,

    /// Performance Counter, Lower Master Write Latency Count
    pub PC_MW_TLAT_L: RWRegister<u32>,

    /// Performance Counter, Upper Slave Read Total Non-Responding Time
    pub PC_SR_TNRT_U: RWRegister<u32>,

    /// Performance Counter, Lower Slave Read Total Non-Responding Time
    pub PC_SR_TNRT_L: RWRegister<u32>,

    /// Performance Counter, Upper Slave Write Total Non-Responding Time
    pub PC_SW_TNRT_U: RWRegister<u32>,

    /// Performance Counter, Lower Slave Write Total Non-Responding Time
    pub PC_SW_TNRT_L: RWRegister<u32>,

    _reserved6: [u32; 8],

    /// IEE Version ID Register 1
    pub VIDR1: RORegister<u32>,

    _reserved7: [u32; 1],

    /// IEE AES Version ID Register
    pub AESVID: RORegister<u32>,

    _reserved8: [u32; 1],

    /// IEE Region REGION Attribute Register.
    pub REGATTR_0: RWRegister<u32>,

    _reserved9: [u32; 1],

    /// IEE Region REGION Page Offset Register
    pub REGPO_0: RWRegister<u32>,

    _reserved10: [u32; 13],

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_0_0: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_1_0: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_2_0: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_3_0: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_4_0: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_5_0: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_6_0: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_7_0: WORegister<u32>,

    _reserved11: [u32; 8],

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_0_0: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_1_0: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_2_0: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_3_0: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_4_0: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_5_0: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_6_0: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_7_0: WORegister<u32>,

    _reserved12: [u32; 24],

    /// IEE Region REGION Attribute Register.
    pub REGATTR_1: RWRegister<u32>,

    _reserved13: [u32; 1],

    /// IEE Region REGION Page Offset Register
    pub REGPO_1: RWRegister<u32>,

    _reserved14: [u32; 13],

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_0_1: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_1_1: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_2_1: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_3_1: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_4_1: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_5_1: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_6_1: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_7_1: WORegister<u32>,

    _reserved15: [u32; 8],

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_0_1: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_1_1: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_2_1: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_3_1: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_4_1: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_5_1: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_6_1: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_7_1: WORegister<u32>,

    _reserved16: [u32; 24],

    /// IEE Region REGION Attribute Register.
    pub REGATTR_2: RWRegister<u32>,

    _reserved17: [u32; 1],

    /// IEE Region REGION Page Offset Register
    pub REGPO_2: RWRegister<u32>,

    _reserved18: [u32; 13],

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_0_2: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_1_2: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_2_2: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_3_2: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_4_2: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_5_2: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_6_2: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_7_2: WORegister<u32>,

    _reserved19: [u32; 8],

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_0_2: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_1_2: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_2_2: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_3_2: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_4_2: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_5_2: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_6_2: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_7_2: WORegister<u32>,

    _reserved20: [u32; 24],

    /// IEE Region REGION Attribute Register.
    pub REGATTR_3: RWRegister<u32>,

    _reserved21: [u32; 1],

    /// IEE Region REGION Page Offset Register
    pub REGPO_3: RWRegister<u32>,

    _reserved22: [u32; 13],

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_0_3: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_1_3: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_2_3: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_3_3: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_4_3: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_5_3: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_6_3: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_7_3: WORegister<u32>,

    _reserved23: [u32; 8],

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_0_3: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_1_3: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_2_3: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_3_3: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_4_3: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_5_3: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_6_3: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_7_3: WORegister<u32>,

    _reserved24: [u32; 24],

    /// IEE Region REGION Attribute Register.
    pub REGATTR_4: RWRegister<u32>,

    _reserved25: [u32; 1],

    /// IEE Region REGION Page Offset Register
    pub REGPO_4: RWRegister<u32>,

    _reserved26: [u32; 13],

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_0_4: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_1_4: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_2_4: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_3_4: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_4_4: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_5_4: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_6_4: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_7_4: WORegister<u32>,

    _reserved27: [u32; 8],

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_0_4: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_1_4: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_2_4: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_3_4: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_4_4: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_5_4: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_6_4: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_7_4: WORegister<u32>,

    _reserved28: [u32; 24],

    /// IEE Region REGION Attribute Register.
    pub REGATTR_5: RWRegister<u32>,

    _reserved29: [u32; 1],

    /// IEE Region REGION Page Offset Register
    pub REGPO_5: RWRegister<u32>,

    _reserved30: [u32; 13],

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_0_5: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_1_5: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_2_5: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_3_5: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_4_5: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_5_5: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_6_5: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_7_5: WORegister<u32>,

    _reserved31: [u32; 8],

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_0_5: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_1_5: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_2_5: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_3_5: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_4_5: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_5_5: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_6_5: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_7_5: WORegister<u32>,

    _reserved32: [u32; 24],

    /// IEE Region REGION Attribute Register.
    pub REGATTR_6: RWRegister<u32>,

    _reserved33: [u32; 1],

    /// IEE Region REGION Page Offset Register
    pub REGPO_6: RWRegister<u32>,

    _reserved34: [u32; 13],

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_0_6: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_1_6: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_2_6: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_3_6: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_4_6: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_5_6: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_6_6: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_7_6: WORegister<u32>,

    _reserved35: [u32; 8],

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_0_6: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_1_6: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_2_6: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_3_6: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_4_6: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_5_6: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_6_6: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_7_6: WORegister<u32>,

    _reserved36: [u32; 24],

    /// IEE Region REGION Attribute Register.
    pub REGATTR_7: RWRegister<u32>,

    _reserved37: [u32; 1],

    /// IEE Region REGION Page Offset Register
    pub REGPO_7: RWRegister<u32>,

    _reserved38: [u32; 13],

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_0_7: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_1_7: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_2_7: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_3_7: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_4_7: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_5_7: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_6_7: WORegister<u32>,

    /// IEE Region REGION Key 1 Register
    pub REGKEY1_7_7: WORegister<u32>,

    _reserved39: [u32; 8],

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_0_7: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_1_7: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_2_7: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_3_7: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_4_7: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_5_7: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_6_7: WORegister<u32>,

    /// IEE Region REGION Key 2 Register
    pub REGKEY2_7_7: WORegister<u32>,

    _reserved40: [u32; 408],

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB0: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB1: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB2: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB3: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB4: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB5: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB6: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB7: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB8: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB9: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB10: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB11: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB12: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB13: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB14: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB15: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB16: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB17: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB18: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB19: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB20: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB21: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB22: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB23: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB24: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB25: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB26: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB27: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB28: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB29: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB30: RWRegister<u32>,

    /// IEE AES Test Mode Data Buffer
    pub AES_TST_DB31: RWRegister<u32>,
}
pub struct ResetValues {
    pub GCFG: u32,
    pub STA: u32,
    pub TSTMD: u32,
    pub DPAMS: u32,
    pub PC_S_LT: u32,
    pub PC_M_LT: u32,
    pub PC_BLK_ENC: u32,
    pub PC_BLK_DEC: u32,
    pub PC_SR_TRANS: u32,
    pub PC_SW_TRANS: u32,
    pub PC_MR_TRANS: u32,
    pub PC_MW_TRANS: u32,
    pub PC_M_MBR: u32,
    pub PC_SR_TBC_U: u32,
    pub PC_SR_TBC_L: u32,
    pub PC_SW_TBC_U: u32,
    pub PC_SW_TBC_L: u32,
    pub PC_MR_TBC_U: u32,
    pub PC_MR_TBC_L: u32,
    pub PC_MW_TBC_U: u32,
    pub PC_MW_TBC_L: u32,
    pub PC_SR_TLGTT: u32,
    pub PC_SW_TLGTT: u32,
    pub PC_MR_TLGTT: u32,
    pub PC_MW_TLGTT: u32,
    pub PC_SR_TLAT_U: u32,
    pub PC_SR_TLAT_L: u32,
    pub PC_SW_TLAT_U: u32,
    pub PC_SW_TLAT_L: u32,
    pub PC_MR_TLAT_U: u32,
    pub PC_MR_TLAT_L: u32,
    pub PC_MW_TLAT_U: u32,
    pub PC_MW_TLAT_L: u32,
    pub PC_SR_TNRT_U: u32,
    pub PC_SR_TNRT_L: u32,
    pub PC_SW_TNRT_U: u32,
    pub PC_SW_TNRT_L: u32,
    pub VIDR1: u32,
    pub AESVID: u32,
    pub REGATTR_0: u32,
    pub REGPO_0: u32,
    pub REGKEY1_0_0: u32,
    pub REGKEY1_1_0: u32,
    pub REGKEY1_2_0: u32,
    pub REGKEY1_3_0: u32,
    pub REGKEY1_4_0: u32,
    pub REGKEY1_5_0: u32,
    pub REGKEY1_6_0: u32,
    pub REGKEY1_7_0: u32,
    pub REGKEY2_0_0: u32,
    pub REGKEY2_1_0: u32,
    pub REGKEY2_2_0: u32,
    pub REGKEY2_3_0: u32,
    pub REGKEY2_4_0: u32,
    pub REGKEY2_5_0: u32,
    pub REGKEY2_6_0: u32,
    pub REGKEY2_7_0: u32,
    pub REGATTR_1: u32,
    pub REGPO_1: u32,
    pub REGKEY1_0_1: u32,
    pub REGKEY1_1_1: u32,
    pub REGKEY1_2_1: u32,
    pub REGKEY1_3_1: u32,
    pub REGKEY1_4_1: u32,
    pub REGKEY1_5_1: u32,
    pub REGKEY1_6_1: u32,
    pub REGKEY1_7_1: u32,
    pub REGKEY2_0_1: u32,
    pub REGKEY2_1_1: u32,
    pub REGKEY2_2_1: u32,
    pub REGKEY2_3_1: u32,
    pub REGKEY2_4_1: u32,
    pub REGKEY2_5_1: u32,
    pub REGKEY2_6_1: u32,
    pub REGKEY2_7_1: u32,
    pub REGATTR_2: u32,
    pub REGPO_2: u32,
    pub REGKEY1_0_2: u32,
    pub REGKEY1_1_2: u32,
    pub REGKEY1_2_2: u32,
    pub REGKEY1_3_2: u32,
    pub REGKEY1_4_2: u32,
    pub REGKEY1_5_2: u32,
    pub REGKEY1_6_2: u32,
    pub REGKEY1_7_2: u32,
    pub REGKEY2_0_2: u32,
    pub REGKEY2_1_2: u32,
    pub REGKEY2_2_2: u32,
    pub REGKEY2_3_2: u32,
    pub REGKEY2_4_2: u32,
    pub REGKEY2_5_2: u32,
    pub REGKEY2_6_2: u32,
    pub REGKEY2_7_2: u32,
    pub REGATTR_3: u32,
    pub REGPO_3: u32,
    pub REGKEY1_0_3: u32,
    pub REGKEY1_1_3: u32,
    pub REGKEY1_2_3: u32,
    pub REGKEY1_3_3: u32,
    pub REGKEY1_4_3: u32,
    pub REGKEY1_5_3: u32,
    pub REGKEY1_6_3: u32,
    pub REGKEY1_7_3: u32,
    pub REGKEY2_0_3: u32,
    pub REGKEY2_1_3: u32,
    pub REGKEY2_2_3: u32,
    pub REGKEY2_3_3: u32,
    pub REGKEY2_4_3: u32,
    pub REGKEY2_5_3: u32,
    pub REGKEY2_6_3: u32,
    pub REGKEY2_7_3: u32,
    pub REGATTR_4: u32,
    pub REGPO_4: u32,
    pub REGKEY1_0_4: u32,
    pub REGKEY1_1_4: u32,
    pub REGKEY1_2_4: u32,
    pub REGKEY1_3_4: u32,
    pub REGKEY1_4_4: u32,
    pub REGKEY1_5_4: u32,
    pub REGKEY1_6_4: u32,
    pub REGKEY1_7_4: u32,
    pub REGKEY2_0_4: u32,
    pub REGKEY2_1_4: u32,
    pub REGKEY2_2_4: u32,
    pub REGKEY2_3_4: u32,
    pub REGKEY2_4_4: u32,
    pub REGKEY2_5_4: u32,
    pub REGKEY2_6_4: u32,
    pub REGKEY2_7_4: u32,
    pub REGATTR_5: u32,
    pub REGPO_5: u32,
    pub REGKEY1_0_5: u32,
    pub REGKEY1_1_5: u32,
    pub REGKEY1_2_5: u32,
    pub REGKEY1_3_5: u32,
    pub REGKEY1_4_5: u32,
    pub REGKEY1_5_5: u32,
    pub REGKEY1_6_5: u32,
    pub REGKEY1_7_5: u32,
    pub REGKEY2_0_5: u32,
    pub REGKEY2_1_5: u32,
    pub REGKEY2_2_5: u32,
    pub REGKEY2_3_5: u32,
    pub REGKEY2_4_5: u32,
    pub REGKEY2_5_5: u32,
    pub REGKEY2_6_5: u32,
    pub REGKEY2_7_5: u32,
    pub REGATTR_6: u32,
    pub REGPO_6: u32,
    pub REGKEY1_0_6: u32,
    pub REGKEY1_1_6: u32,
    pub REGKEY1_2_6: u32,
    pub REGKEY1_3_6: u32,
    pub REGKEY1_4_6: u32,
    pub REGKEY1_5_6: u32,
    pub REGKEY1_6_6: u32,
    pub REGKEY1_7_6: u32,
    pub REGKEY2_0_6: u32,
    pub REGKEY2_1_6: u32,
    pub REGKEY2_2_6: u32,
    pub REGKEY2_3_6: u32,
    pub REGKEY2_4_6: u32,
    pub REGKEY2_5_6: u32,
    pub REGKEY2_6_6: u32,
    pub REGKEY2_7_6: u32,
    pub REGATTR_7: u32,
    pub REGPO_7: u32,
    pub REGKEY1_0_7: u32,
    pub REGKEY1_1_7: u32,
    pub REGKEY1_2_7: u32,
    pub REGKEY1_3_7: u32,
    pub REGKEY1_4_7: u32,
    pub REGKEY1_5_7: u32,
    pub REGKEY1_6_7: u32,
    pub REGKEY1_7_7: u32,
    pub REGKEY2_0_7: u32,
    pub REGKEY2_1_7: u32,
    pub REGKEY2_2_7: u32,
    pub REGKEY2_3_7: u32,
    pub REGKEY2_4_7: u32,
    pub REGKEY2_5_7: u32,
    pub REGKEY2_6_7: u32,
    pub REGKEY2_7_7: u32,
    pub AES_TST_DB0: u32,
    pub AES_TST_DB1: u32,
    pub AES_TST_DB2: u32,
    pub AES_TST_DB3: u32,
    pub AES_TST_DB4: u32,
    pub AES_TST_DB5: u32,
    pub AES_TST_DB6: u32,
    pub AES_TST_DB7: u32,
    pub AES_TST_DB8: u32,
    pub AES_TST_DB9: u32,
    pub AES_TST_DB10: u32,
    pub AES_TST_DB11: u32,
    pub AES_TST_DB12: u32,
    pub AES_TST_DB13: u32,
    pub AES_TST_DB14: u32,
    pub AES_TST_DB15: u32,
    pub AES_TST_DB16: u32,
    pub AES_TST_DB17: u32,
    pub AES_TST_DB18: u32,
    pub AES_TST_DB19: u32,
    pub AES_TST_DB20: u32,
    pub AES_TST_DB21: u32,
    pub AES_TST_DB22: u32,
    pub AES_TST_DB23: u32,
    pub AES_TST_DB24: u32,
    pub AES_TST_DB25: u32,
    pub AES_TST_DB26: u32,
    pub AES_TST_DB27: u32,
    pub AES_TST_DB28: u32,
    pub AES_TST_DB29: u32,
    pub AES_TST_DB30: u32,
    pub AES_TST_DB31: u32,
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
