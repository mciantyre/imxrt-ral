#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DCIC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// DCIC Control Register
pub mod DCICC {

    /// Integrity Check enable. Main enable switch.
    pub mod IC_EN {
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

            /// 0b0: Disabled
            pub const IC_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const IC_EN_1: u32 = 0b1;
        }
    }

    /// DATA_EN_IN signal polarity.
    pub mod DE_POL {
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

            /// 0b0: Active High.
            pub const DE_POL_0: u32 = 0b0;

            /// 0b1: Active Low.
            pub const DE_POL_1: u32 = 0b1;
        }
    }

    /// HSYNC_IN signal polarity.
    pub mod HSYNC_POL {
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

            /// 0b0: Active High.
            pub const HSYNC_POL_0: u32 = 0b0;

            /// 0b1: Active Low.
            pub const HSYNC_POL_1: u32 = 0b1;
        }
    }

    /// VSYNC_IN signal polarity.
    pub mod VSYNC_POL {
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

            /// 0b0: Active High.
            pub const VSYNC_POL_0: u32 = 0b0;

            /// 0b1: Active Low.
            pub const VSYNC_POL_1: u32 = 0b1;
        }
    }

    /// DISP_CLK signal polarity.
    pub mod CLK_POL {
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

            /// 0b0: Not inverted (default).
            pub const CLK_POL_0: u32 = 0b0;

            /// 0b1: Inverted.
            pub const CLK_POL_1: u32 = 0b1;
        }
    }
}

/// DCIC Interrupt Control Register
pub mod DCICIC {

    /// Error Interrupt mask. Can be changed only while FREEZE_MASK = 0.
    pub mod EI_MASK {
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

            /// 0b0: Mask disabled - Interrupt assertion enabled
            pub const EI_MASK_0: u32 = 0b0;

            /// 0b1: Mask enabled - Interrupt assertion disabled
            pub const EI_MASK_1: u32 = 0b1;
        }
    }

    /// Functional Interrupt mask. Can be changed only while FREEZE_MASK = 0.
    pub mod FI_MASK {
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

            /// 0b0: Mask disabled - Interrupt assertion enabled
            pub const FI_MASK_0: u32 = 0b0;

            /// 0b1: Mask enabled - Interrupt assertion disabled
            pub const FI_MASK_1: u32 = 0b1;
        }
    }

    /// Disable change of interrupt masks. "Sticky" bit which can be set once and cleared by reset only.
    pub mod FREEZE_MASK {
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

            /// 0b0: Masks change allowed
            pub const FREEZE_MASK_0: u32 = 0b0;

            /// 0b1: Masks are frozen
            pub const FREEZE_MASK_1: u32 = 0b1;
        }
    }

    /// External controller mismatch indication signal.
    pub mod EXT_SIG_EN {
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

            /// 0b0: Disabled
            pub const EXT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const EXT_SIG_EN_1: u32 = 0b1;
        }
    }
}

/// DCIC Status Register
pub mod DCICS {

    /// Each set bit of this field indicates there was a mismatch at the appropriate ROIs signature during the last frame
    pub mod ROI_MATCH_STAT {
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

            /// 0b0000000000000000: ROI calculated CRC matches expected signature
            pub const ROI_MATCH_STAT_0: u32 = 0b0000000000000000;

            /// 0b0000000000000001: Mismatch at ROI calculated CRC
            pub const ROI_MATCH_STAT_1: u32 = 0b0000000000000001;
        }
    }

    /// Error Interrupt status
    pub mod EI_STAT {
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

            /// 0b0: No pending Interrupt
            pub const EI_STAT_0: u32 = 0b0;

            /// 0b1: Pending Interrupt
            pub const EI_STAT_1: u32 = 0b1;
        }
    }

    /// Functional Interrupt status. Write "1" to clear.
    pub mod FI_STAT {
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

            /// 0b0: No pending Interrupt
            pub const FI_STAT_0: u32 = 0b0;

            /// 0b1: Pending Interrupt
            pub const FI_STAT_1: u32 = 0b1;
        }
    }
}

/// DCIC ROI Config Register
pub mod DCICRC1 {

    /// Column number of ROIs upper-left corner (X coordinate) Range: 0 to 2^13-1
    pub mod START_OFFSET_X {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Row number of ROIs upper-left corner (Y coordinate) Range: 0 to 2^12-1
    pub mod START_OFFSET_Y {
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

    /// When set, the only parameter of the ROI that can be changed is the reference signature
    pub mod ROI_FREEZE {
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

            /// 0b0: ROI configuration can be changed
            pub const ROI_FREEZE_0: u32 = 0b0;

            /// 0b1: ROI configuration is frozen
            pub const ROI_FREEZE_1: u32 = 0b1;
        }
    }

    /// ROI tracking enable
    pub mod ROI_EN {
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

            /// 0b0: Disabled
            pub const ROI_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ROI_EN_1: u32 = 0b1;
        }
    }
}

/// DCIC ROI Config Register
pub mod DCICRC2 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC3 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC4 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC5 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC6 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC7 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC8 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC9 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC10 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC11 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC12 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC13 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC14 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC15 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Config Register
pub mod DCICRC16 {
    pub use super::DCICRC1::ROI_EN;
    pub use super::DCICRC1::ROI_FREEZE;
    pub use super::DCICRC1::START_OFFSET_X;
    pub use super::DCICRC1::START_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS1 {

    /// Column number of ROIs lower-right corner (X coordinate) Range: 1 to 2^13-1
    pub mod END_OFFSET_X {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Row number of ROIs lower-right corner (Y coordinate) Range: 1 to 2^12-1
    pub mod END_OFFSET_Y {
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

/// DCIC ROI Size Register
pub mod DCICRS2 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS3 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS4 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS5 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS6 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS7 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS8 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS9 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS10 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS11 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS12 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS13 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS14 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS15 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Size Register
pub mod DCICRS16 {
    pub use super::DCICRS1::END_OFFSET_X;
    pub use super::DCICRS1::END_OFFSET_Y;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS1 {

    /// 32-bit expected signature (CRC calculation result) for the ROI
    pub mod REFERENCE_SIGNATURE {
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

/// DCIC ROI Reference Signature Register
pub mod DCICRRS2 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS3 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS4 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS5 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS6 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS7 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS8 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS9 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS10 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS11 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS12 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS13 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS14 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS15 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Reference Signature Register
pub mod DCICRRS16 {
    pub use super::DCICRRS1::REFERENCE_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS1 {

    /// 32-bit actual signature (CRC calculation result) for the ROI during the last frame
    pub mod CALCULATED_SIGNATURE {
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

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS2 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS3 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS4 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS5 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS6 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS7 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS8 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS9 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS10 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS11 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS12 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS13 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS14 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS15 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}

/// DCIC ROI Calculated Signature Register
pub mod DCICRCS16 {
    pub use super::DCICRCS1::CALCULATED_SIGNATURE;
}
#[repr(C)]
pub struct RegisterBlock {
    /// DCIC Control Register
    pub DCICC: RWRegister<u32>,

    /// DCIC Interrupt Control Register
    pub DCICIC: RWRegister<u32>,

    /// DCIC Status Register
    pub DCICS: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// DCIC ROI Config Register
    pub DCICRC1: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS1: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS1: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS1: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC2: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS2: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS2: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS2: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC3: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS3: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS3: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS3: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC4: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS4: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS4: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS4: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC5: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS5: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS5: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS5: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC6: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS6: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS6: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS6: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC7: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS7: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS7: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS7: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC8: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS8: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS8: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS8: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC9: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS9: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS9: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS9: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC10: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS10: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS10: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS10: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC11: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS11: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS11: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS11: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC12: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS12: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS12: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS12: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC13: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS13: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS13: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS13: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC14: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS14: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS14: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS14: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC15: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS15: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS15: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS15: RORegister<u32>,

    /// DCIC ROI Config Register
    pub DCICRC16: RWRegister<u32>,

    /// DCIC ROI Size Register
    pub DCICRS16: RWRegister<u32>,

    /// DCIC ROI Reference Signature Register
    pub DCICRRS16: RWRegister<u32>,

    /// DCIC ROI Calculated Signature Register
    pub DCICRCS16: RORegister<u32>,
}
pub struct ResetValues {
    pub DCICC: u32,
    pub DCICIC: u32,
    pub DCICS: u32,
    pub DCICRC1: u32,
    pub DCICRS1: u32,
    pub DCICRRS1: u32,
    pub DCICRCS1: u32,
    pub DCICRC2: u32,
    pub DCICRS2: u32,
    pub DCICRRS2: u32,
    pub DCICRCS2: u32,
    pub DCICRC3: u32,
    pub DCICRS3: u32,
    pub DCICRRS3: u32,
    pub DCICRCS3: u32,
    pub DCICRC4: u32,
    pub DCICRS4: u32,
    pub DCICRRS4: u32,
    pub DCICRCS4: u32,
    pub DCICRC5: u32,
    pub DCICRS5: u32,
    pub DCICRRS5: u32,
    pub DCICRCS5: u32,
    pub DCICRC6: u32,
    pub DCICRS6: u32,
    pub DCICRRS6: u32,
    pub DCICRCS6: u32,
    pub DCICRC7: u32,
    pub DCICRS7: u32,
    pub DCICRRS7: u32,
    pub DCICRCS7: u32,
    pub DCICRC8: u32,
    pub DCICRS8: u32,
    pub DCICRRS8: u32,
    pub DCICRCS8: u32,
    pub DCICRC9: u32,
    pub DCICRS9: u32,
    pub DCICRRS9: u32,
    pub DCICRCS9: u32,
    pub DCICRC10: u32,
    pub DCICRS10: u32,
    pub DCICRRS10: u32,
    pub DCICRCS10: u32,
    pub DCICRC11: u32,
    pub DCICRS11: u32,
    pub DCICRRS11: u32,
    pub DCICRCS11: u32,
    pub DCICRC12: u32,
    pub DCICRS12: u32,
    pub DCICRRS12: u32,
    pub DCICRCS12: u32,
    pub DCICRC13: u32,
    pub DCICRS13: u32,
    pub DCICRRS13: u32,
    pub DCICRCS13: u32,
    pub DCICRC14: u32,
    pub DCICRS14: u32,
    pub DCICRRS14: u32,
    pub DCICRCS14: u32,
    pub DCICRC15: u32,
    pub DCICRS15: u32,
    pub DCICRRS15: u32,
    pub DCICRCS15: u32,
    pub DCICRC16: u32,
    pub DCICRS16: u32,
    pub DCICRRS16: u32,
    pub DCICRCS16: u32,
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
