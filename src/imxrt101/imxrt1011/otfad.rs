#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! OTFAD

use crate::RWRegister;

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// Control Register
pub mod CR {

    /// IRQE
    pub mod IRQE {
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

            /// 0b0: SR\[KBERR\] = 1 does not generate an interrupt request.
            pub const IRQE_0: u32 = 0b0;

            /// 0b1: SR\[KBERR\] = 1 generates an interrupt request.
            pub const IRQE_1: u32 = 0b1;
        }
    }

    /// Force Error
    pub mod FERR {
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

            /// 0b0: No effect on the SR\[KBERE\] indicator.
            pub const FERR_0: u32 = 0b0;

            /// 0b1: SR\[KBERR\] is immediately set after a write with this data bit set.
            pub const FERR_1: u32 = 0b1;
        }
    }

    /// Force Security Violation Mode
    pub mod FSVM {
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

            /// 0b0: No effect on the operating mode.
            pub const FSVM_0: u32 = 0b0;

            /// 0b1: Force entry into SVM after a write with this data bit set and the data bit associated with FLDM cleared. SR\[MODE\] signals the operating mode.
            pub const FSVM_1: u32 = 0b1;
        }
    }

    /// Force Logically Disabled Mode
    pub mod FLDM {
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

            /// 0b0: No effect on the operating mode.
            pub const FLDM_0: u32 = 0b0;

            /// 0b1: Force entry into LDM after a write with this data bit set. SR\[MODE\] signals the operating mode.
            pub const FLDM_1: u32 = 0b1;
        }
    }

    /// Key Blob Scramble Enable
    pub mod KBSE {
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

            /// 0b0: Key blob KEK scrambling is disabled.
            pub const KBSE_0: u32 = 0b0;

            /// 0b1: Key blob KEK scrambling is enabled.
            pub const KBSE_1: u32 = 0b1;
        }
    }

    /// Key Blob Processing Enable
    pub mod KBPE {
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

            /// 0b0: Key blob processing is disabled.
            pub const KBPE_0: u32 = 0b0;

            /// 0b1: Key blob processing is enabled.
            pub const KBPE_1: u32 = 0b1;
        }
    }

    /// Key Blob CRC Enable
    pub mod KBCE {
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

            /// 0b0: CRC-32 during key blob processing is disabled.
            pub const KBCE_0: u32 = 0b0;

            /// 0b1: CRC-32 during key blob processing is enabled.
            pub const KBCE_1: u32 = 0b1;
        }
    }

    /// Restricted Register Access Enable
    pub mod RRAE {
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

            /// 0b0: Register access is fully enabled. The OTFAD programming model registers can be accessed "normally".
            pub const RRAE_0: u32 = 0b0;

            /// 0b1: Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI.
            pub const RRAE_1: u32 = 0b1;
        }
    }

    /// Start key blob processing
    pub mod SKBP {
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

            /// 0b0: Key blob processing is not initiated.
            pub const SKBP_0: u32 = 0b0;

            /// 0b1: Properly-enabled key blob processing is initiated.
            pub const SKBP_1: u32 = 0b1;
        }
    }

    /// Global OTFAD Enable
    pub mod GE {
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

            /// 0b0: OTFAD has decryption disabled. All data fetched by the FlexSPI bypasses OTFAD processing.
            pub const GE_0: u32 = 0b0;

            /// 0b1: OTFAD has decryption enabled, and processes data fetched by the FlexSPI as defined by the hardware configuration.
            pub const GE_1: u32 = 0b1;
        }
    }
}

/// Status Register
pub mod SR {

    /// Key Blob Error
    pub mod KBERR {
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

            /// 0b0: No key blob error detected.
            pub const KBERR_0: u32 = 0b0;

            /// 0b1: One or more key blob errors has been detected.
            pub const KBERR_1: u32 = 0b1;
        }
    }

    /// MDPC Present
    pub mod MDPCP {
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

    /// Operating Mode
    pub mod MODE {
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

            /// 0b00: Operating in Normal mode (NRM)
            pub const MODE_0: u32 = 0b00;

            /// 0b01: Unused (reserved)
            pub const MODE_1: u32 = 0b01;

            /// 0b10: Operating in Security Violation Mode (SVM)
            pub const MODE_2: u32 = 0b10;

            /// 0b11: Operating in Logically Disabled Mode (LDM)
            pub const MODE_3: u32 = 0b11;
        }
    }

    /// Number of Contexts
    pub mod NCTX {
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

    /// Context Error
    pub mod CTXER0 {
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

            /// 0b0: No key blob error was detected for context "n".
            pub const NOERROR: u32 = 0b0;

            /// 0b1: Either a key blob integrity error or a key blob CRC error was detected in context "n".
            pub const ERROR: u32 = 0b1;
        }
    }

    /// Context Error
    pub mod CTXER1 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTXER0::RW;
    }

    /// Context Error
    pub mod CTXER2 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTXER0::RW;
    }

    /// Context Error
    pub mod CTXER3 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTXER0::RW;
    }

    /// Context Integrity Error
    pub mod CTXIE0 {
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

            /// 0b0: No key blob integrity error was detected for context "n".
            pub const NOINTEGRITYERR: u32 = 0b0;

            /// 0b1: A key blob integrity error was detected in context "n".
            pub const INTEGRITYERR: u32 = 0b1;
        }
    }

    /// Context Integrity Error
    pub mod CTXIE1 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTXIE0::RW;
    }

    /// Context Integrity Error
    pub mod CTXIE2 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTXIE0::RW;
    }

    /// Context Integrity Error
    pub mod CTXIE3 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTXIE0::RW;
    }

    /// Hardware Revision Level
    pub mod HRL {
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

    /// Restricted Register Access Mode
    pub mod RRAM {
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

            /// 0b0: Register access is fully enabled. The OTFAD programming model registers can be accessed "normally".
            pub const RRAM_0: u32 = 0b0;

            /// 0b1: Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI.
            pub const RRAM_1: u32 = 0b1;
        }
    }

    /// Global Enable Mode
    pub mod GEM {
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

            /// 0b0: OTFAD is disabled. All data fetched by the FlexSPI bypasses OTFAD processing.
            pub const GEM_0: u32 = 0b0;

            /// 0b1: OTFAD is enabled, and processes data fetched by the FlexSPI as defined by the hardware configuration.
            pub const GEM_1: u32 = 0b1;
        }
    }

    /// Key Blob Processing Enable
    pub mod KBPE {
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

            /// 0b0: Key blob processing is not enabled.
            pub const KBPE_0: u32 = 0b0;

            /// 0b1: Key blob processing is enabled.
            pub const KBPE_1: u32 = 0b1;
        }
    }

    /// Key Blob Processing Done
    pub mod KBD {
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

            /// 0b0: Key blob processing was not enabled, or is not complete.
            pub const KBD_0: u32 = 0b0;

            /// 0b1: Key blob processing was enabled and is complete.
            pub const KBD_1: u32 = 0b1;
        }
    }
}

/// AES Key Word
pub mod CTX_KEY0_0 {

    /// AES Key
    pub mod KEY {
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

/// AES Key Word
pub mod CTX_KEY1_0 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Key Word
pub mod CTX_KEY2_0 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Key Word
pub mod CTX_KEY3_0 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Counter Word
pub mod CTX_CTR0_0 {

    /// AES Counter
    pub mod CTR {
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

/// AES Counter Word
pub mod CTX_CTR1_0 {
    pub use super::CTX_CTR0_0::CTR;
}

/// AES Region Descriptor Word0
pub mod CTX_RGD_W0_0 {

    /// Start Address
    pub mod SRTADDR {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (22 bits: 0x3fffff << 10)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AES Region Descriptor Word1
pub mod CTX_RGD_W1_0 {

    /// Valid
    pub mod VLD {
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

            /// 0b0: Context is invalid.
            pub const VLD_0: u32 = 0b0;

            /// 0b1: Context is valid.
            pub const VLD_1: u32 = 0b1;
        }
    }

    /// AES Decryption Enable.
    pub mod ADE {
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

            /// 0b0: Bypass the fetched data.
            pub const ADE_0: u32 = 0b0;

            /// 0b1: Perform the CTR-AES128 mode decryption on the fetched data.
            pub const ADE_1: u32 = 0b1;
        }
    }

    /// Read-Only
    pub mod RO {
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

            /// 0b0: The context registers can be accessed normally (as defined by SR\[RRAM\]).
            pub const RO_0: u32 = 0b0;

            /// 0b1: The context registers are read-only and accesses may be further restricted based on SR\[RRAM\].
            pub const RO_1: u32 = 0b1;
        }
    }

    /// End Address
    pub mod ENDADDR {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (22 bits: 0x3fffff << 10)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AES Key Word
pub mod CTX_KEY0_1 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Key Word
pub mod CTX_KEY1_1 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Key Word
pub mod CTX_KEY2_1 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Key Word
pub mod CTX_KEY3_1 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Counter Word
pub mod CTX_CTR0_1 {
    pub use super::CTX_CTR0_0::CTR;
}

/// AES Counter Word
pub mod CTX_CTR1_1 {
    pub use super::CTX_CTR0_0::CTR;
}

/// AES Region Descriptor Word0
pub mod CTX_RGD_W0_1 {
    pub use super::CTX_RGD_W0_0::SRTADDR;
}

/// AES Region Descriptor Word1
pub mod CTX_RGD_W1_1 {
    pub use super::CTX_RGD_W1_0::ADE;
    pub use super::CTX_RGD_W1_0::ENDADDR;
    pub use super::CTX_RGD_W1_0::RO;
    pub use super::CTX_RGD_W1_0::VLD;
}

/// AES Key Word
pub mod CTX_KEY0_2 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Key Word
pub mod CTX_KEY1_2 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Key Word
pub mod CTX_KEY2_2 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Key Word
pub mod CTX_KEY3_2 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Counter Word
pub mod CTX_CTR0_2 {
    pub use super::CTX_CTR0_0::CTR;
}

/// AES Counter Word
pub mod CTX_CTR1_2 {
    pub use super::CTX_CTR0_0::CTR;
}

/// AES Region Descriptor Word0
pub mod CTX_RGD_W0_2 {
    pub use super::CTX_RGD_W0_0::SRTADDR;
}

/// AES Region Descriptor Word1
pub mod CTX_RGD_W1_2 {
    pub use super::CTX_RGD_W1_0::ADE;
    pub use super::CTX_RGD_W1_0::ENDADDR;
    pub use super::CTX_RGD_W1_0::RO;
    pub use super::CTX_RGD_W1_0::VLD;
}

/// AES Key Word
pub mod CTX_KEY0_3 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Key Word
pub mod CTX_KEY1_3 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Key Word
pub mod CTX_KEY2_3 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Key Word
pub mod CTX_KEY3_3 {
    pub use super::CTX_KEY0_0::KEY;
}

/// AES Counter Word
pub mod CTX_CTR0_3 {
    pub use super::CTX_CTR0_0::CTR;
}

/// AES Counter Word
pub mod CTX_CTR1_3 {
    pub use super::CTX_CTR0_0::CTR;
}

/// AES Region Descriptor Word0
pub mod CTX_RGD_W0_3 {
    pub use super::CTX_RGD_W0_0::SRTADDR;
}

/// AES Region Descriptor Word1
pub mod CTX_RGD_W1_3 {
    pub use super::CTX_RGD_W1_0::ADE;
    pub use super::CTX_RGD_W1_0::ENDADDR;
    pub use super::CTX_RGD_W1_0::RO;
    pub use super::CTX_RGD_W1_0::VLD;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 768],

    /// Control Register
    pub CR: RWRegister<u32>,

    /// Status Register
    pub SR: RWRegister<u32>,

    _reserved2: [u32; 62],

    /// AES Key Word
    pub CTX_KEY0_0: RWRegister<u32>,

    /// AES Key Word
    pub CTX_KEY1_0: RWRegister<u32>,

    /// AES Key Word
    pub CTX_KEY2_0: RWRegister<u32>,

    /// AES Key Word
    pub CTX_KEY3_0: RWRegister<u32>,

    /// AES Counter Word
    pub CTX_CTR0_0: RWRegister<u32>,

    /// AES Counter Word
    pub CTX_CTR1_0: RWRegister<u32>,

    /// AES Region Descriptor Word0
    pub CTX_RGD_W0_0: RWRegister<u32>,

    /// AES Region Descriptor Word1
    pub CTX_RGD_W1_0: RWRegister<u32>,

    _reserved3: [u32; 8],

    /// AES Key Word
    pub CTX_KEY0_1: RWRegister<u32>,

    /// AES Key Word
    pub CTX_KEY1_1: RWRegister<u32>,

    /// AES Key Word
    pub CTX_KEY2_1: RWRegister<u32>,

    /// AES Key Word
    pub CTX_KEY3_1: RWRegister<u32>,

    /// AES Counter Word
    pub CTX_CTR0_1: RWRegister<u32>,

    /// AES Counter Word
    pub CTX_CTR1_1: RWRegister<u32>,

    /// AES Region Descriptor Word0
    pub CTX_RGD_W0_1: RWRegister<u32>,

    /// AES Region Descriptor Word1
    pub CTX_RGD_W1_1: RWRegister<u32>,

    _reserved4: [u32; 8],

    /// AES Key Word
    pub CTX_KEY0_2: RWRegister<u32>,

    /// AES Key Word
    pub CTX_KEY1_2: RWRegister<u32>,

    /// AES Key Word
    pub CTX_KEY2_2: RWRegister<u32>,

    /// AES Key Word
    pub CTX_KEY3_2: RWRegister<u32>,

    /// AES Counter Word
    pub CTX_CTR0_2: RWRegister<u32>,

    /// AES Counter Word
    pub CTX_CTR1_2: RWRegister<u32>,

    /// AES Region Descriptor Word0
    pub CTX_RGD_W0_2: RWRegister<u32>,

    /// AES Region Descriptor Word1
    pub CTX_RGD_W1_2: RWRegister<u32>,

    _reserved5: [u32; 8],

    /// AES Key Word
    pub CTX_KEY0_3: RWRegister<u32>,

    /// AES Key Word
    pub CTX_KEY1_3: RWRegister<u32>,

    /// AES Key Word
    pub CTX_KEY2_3: RWRegister<u32>,

    /// AES Key Word
    pub CTX_KEY3_3: RWRegister<u32>,

    /// AES Counter Word
    pub CTX_CTR0_3: RWRegister<u32>,

    /// AES Counter Word
    pub CTX_CTR1_3: RWRegister<u32>,

    /// AES Region Descriptor Word0
    pub CTX_RGD_W0_3: RWRegister<u32>,

    /// AES Region Descriptor Word1
    pub CTX_RGD_W1_3: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SR: u32,
    pub CTX_KEY0_0: u32,
    pub CTX_KEY1_0: u32,
    pub CTX_KEY2_0: u32,
    pub CTX_KEY3_0: u32,
    pub CTX_CTR0_0: u32,
    pub CTX_CTR1_0: u32,
    pub CTX_RGD_W0_0: u32,
    pub CTX_RGD_W1_0: u32,
    pub CTX_KEY0_1: u32,
    pub CTX_KEY1_1: u32,
    pub CTX_KEY2_1: u32,
    pub CTX_KEY3_1: u32,
    pub CTX_CTR0_1: u32,
    pub CTX_CTR1_1: u32,
    pub CTX_RGD_W0_1: u32,
    pub CTX_RGD_W1_1: u32,
    pub CTX_KEY0_2: u32,
    pub CTX_KEY1_2: u32,
    pub CTX_KEY2_2: u32,
    pub CTX_KEY3_2: u32,
    pub CTX_CTR0_2: u32,
    pub CTX_CTR1_2: u32,
    pub CTX_RGD_W0_2: u32,
    pub CTX_RGD_W1_2: u32,
    pub CTX_KEY0_3: u32,
    pub CTX_KEY1_3: u32,
    pub CTX_KEY2_3: u32,
    pub CTX_KEY3_3: u32,
    pub CTX_CTR0_3: u32,
    pub CTX_CTR1_3: u32,
    pub CTX_RGD_W0_3: u32,
    pub CTX_RGD_W1_3: u32,
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

/// The OTFAD peripheral instance.
#[cfg(not(feature = "doc"))]
pub type OTFAD = Instance<0>;

/// The OTFAD peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type OTFAD = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct OTFAD {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for OTFAD {}
impl crate::Valid for OTFAD {
    fn take() -> Option<Self> {
        <OTFAD>::take()
    }
    fn release(self) {
        <OTFAD>::release(self);
    }
    unsafe fn steal() -> Self {
        <OTFAD>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static OTFAD_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the OTFAD peripheral instance
#[cfg(not(feature = "nosync"))]
impl OTFAD {
    const INSTANCE: Self = Self {
        addr: 0x400a0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in OTFAD
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000040,
        CTX_KEY0_0: 0x00000000,
        CTX_KEY1_0: 0x00000000,
        CTX_KEY2_0: 0x00000000,
        CTX_KEY3_0: 0x00000000,
        CTX_CTR0_0: 0x00000000,
        CTX_CTR1_0: 0x00000000,
        CTX_RGD_W0_0: 0x00000000,
        CTX_RGD_W1_0: 0x000003F8,
        CTX_KEY0_1: 0x00000000,
        CTX_KEY1_1: 0x00000000,
        CTX_KEY2_1: 0x00000000,
        CTX_KEY3_1: 0x00000000,
        CTX_CTR0_1: 0x00000000,
        CTX_CTR1_1: 0x00000000,
        CTX_RGD_W0_1: 0x00000000,
        CTX_RGD_W1_1: 0x000003F8,
        CTX_KEY0_2: 0x00000000,
        CTX_KEY1_2: 0x00000000,
        CTX_KEY2_2: 0x00000000,
        CTX_KEY3_2: 0x00000000,
        CTX_CTR0_2: 0x00000000,
        CTX_CTR1_2: 0x00000000,
        CTX_RGD_W0_2: 0x00000000,
        CTX_RGD_W1_2: 0x000003F8,
        CTX_KEY0_3: 0x00000000,
        CTX_KEY1_3: 0x00000000,
        CTX_KEY2_3: 0x00000000,
        CTX_KEY3_3: 0x00000000,
        CTX_CTR0_3: 0x00000000,
        CTX_CTR1_3: 0x00000000,
        CTX_RGD_W0_3: 0x00000000,
        CTX_RGD_W1_3: 0x000003F8,
    };

    /// Safe access to OTFAD
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Self> {
        let taken = OTFAD_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to OTFAD
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = OTFAD_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal OTFAD
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        OTFAD_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl OTFAD {
    /// The interrupts associated with OTFAD
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with OTFAD
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to OTFAD
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTFAD: *const RegisterBlock = 0x400a0000 as *const _;
