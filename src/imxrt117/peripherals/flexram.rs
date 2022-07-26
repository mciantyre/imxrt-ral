#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FLEXRAM
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// TCM CRTL Register
pub mod TCM_CTRL {

    /// TCM Write Wait Mode Enable
    pub mod TCM_WWAIT_EN {
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

            /// 0b0: TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle.
            pub const TCM_WWAIT_EN_0: u32 = 0b0;

            /// 0b1: TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles.
            pub const TCM_WWAIT_EN_1: u32 = 0b1;
        }
    }

    /// TCM Read Wait Mode Enable
    pub mod TCM_RWAIT_EN {
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

            /// 0b0: TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle.
            pub const TCM_RWAIT_EN_0: u32 = 0b0;

            /// 0b1: TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles.
            pub const TCM_RWAIT_EN_1: u32 = 0b1;
        }
    }

    /// Force RAM Clock Always On
    pub mod FORCE_CLK_ON {
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

    /// Reserved
    pub mod Reserved {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OCRAM Magic Address Register
pub mod OCRAM_MAGIC_ADDR {

    /// OCRAM Write Read Select
    pub mod OCRAM_WR_RD_SEL {
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

            /// 0b0: When OCRAM read access hits magic address, it will generate interrupt.
            pub const OCRAM_WR_RD_SEL_0: u32 = 0b0;

            /// 0b1: When OCRAM write access hits magic address, it will generate interrupt.
            pub const OCRAM_WR_RD_SEL_1: u32 = 0b1;
        }
    }

    /// OCRAM Magic Address
    pub mod OCRAM_MAGIC_ADDR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (17 bits: 0x1ffff << 1)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved
    pub mod Reserved {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (14 bits: 0x3fff << 18)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DTCM Magic Address Register
pub mod DTCM_MAGIC_ADDR {

    /// DTCM Write Read Select
    pub mod DTCM_WR_RD_SEL {
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

            /// 0b0: When DTCM read access hits magic address, it will generate interrupt.
            pub const DTCM_WR_RD_SEL_0: u32 = 0b0;

            /// 0b1: When DTCM write access hits magic address, it will generate interrupt.
            pub const DTCM_WR_RD_SEL_1: u32 = 0b1;
        }
    }

    /// DTCM Magic Address
    pub mod DTCM_MAGIC_ADDR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (16 bits: 0xffff << 1)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved
    pub mod Reserved {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (15 bits: 0x7fff << 17)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ITCM Magic Address Register
pub mod ITCM_MAGIC_ADDR {

    /// ITCM Write Read Select
    pub mod ITCM_WR_RD_SEL {
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

            /// 0b0: When ITCM read access hits magic address, it will generate interrupt.
            pub const ITCM_WR_RD_SEL_0: u32 = 0b0;

            /// 0b1: When ITCM write access hits magic address, it will generate interrupt.
            pub const ITCM_WR_RD_SEL_1: u32 = 0b1;
        }
    }

    /// ITCM Magic Address
    pub mod ITCM_MAGIC_ADDR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (16 bits: 0xffff << 1)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved
    pub mod Reserved {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (15 bits: 0x7fff << 17)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Status Register
pub mod INT_STATUS {

    /// ITCM Magic Address Match Status
    pub mod ITCM_MAM_STATUS {
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

            /// 0b0: ITCM did not access magic address.
            pub const ITCM_MAM_STATUS_0: u32 = 0b0;

            /// 0b1: ITCM accessed magic address.
            pub const ITCM_MAM_STATUS_1: u32 = 0b1;
        }
    }

    /// DTCM Magic Address Match Status
    pub mod DTCM_MAM_STATUS {
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

            /// 0b0: DTCM did not access magic address.
            pub const DTCM_MAM_STATUS_0: u32 = 0b0;

            /// 0b1: DTCM accessed magic address.
            pub const DTCM_MAM_STATUS_1: u32 = 0b1;
        }
    }

    /// OCRAM Magic Address Match Status
    pub mod OCRAM_MAM_STATUS {
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

            /// 0b0: OCRAM did not access magic address.
            pub const OCRAM_MAM_STATUS_0: u32 = 0b0;

            /// 0b1: OCRAM accessed magic address.
            pub const OCRAM_MAM_STATUS_1: u32 = 0b1;
        }
    }

    /// ITCM Access Error Status
    pub mod ITCM_ERR_STATUS {
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

            /// 0b0: ITCM access error does not happen
            pub const ITCM_ERR_STATUS_0: u32 = 0b0;

            /// 0b1: ITCM access error happens.
            pub const ITCM_ERR_STATUS_1: u32 = 0b1;
        }
    }

    /// DTCM Access Error Status
    pub mod DTCM_ERR_STATUS {
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

            /// 0b0: DTCM access error does not happen
            pub const DTCM_ERR_STATUS_0: u32 = 0b0;

            /// 0b1: DTCM access error happens.
            pub const DTCM_ERR_STATUS_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Status
    pub mod OCRAM_ERR_STATUS {
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

            /// 0b0: OCRAM access error does not happen
            pub const OCRAM_ERR_STATUS_0: u32 = 0b0;

            /// 0b1: OCRAM access error happens.
            pub const OCRAM_ERR_STATUS_1: u32 = 0b1;
        }
    }

    /// OCRAM access multi-bit ECC Error Interrupt Status
    pub mod OCRAM_ECC_ERRM_INT {
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

            /// 0b0: OCRAM multi-bit ECC error does not happen
            pub const OCRAM_ECC_ERRM_INT_0: u32 = 0b0;

            /// 0b1: OCRAM multi-bit ECC error happens.
            pub const OCRAM_ECC_ERRM_INT_1: u32 = 0b1;
        }
    }

    /// OCRAM access single-bit ECC Error Interrupt Status
    pub mod OCRAM_ECC_ERRS_INT {
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

            /// 0b0: OCRAM single-bit ECC error does not happen
            pub const OCRAM_ECC_ERRS_INT_0: u32 = 0b0;

            /// 0b1: OCRAM single-bit ECC error happens.
            pub const OCRAM_ECC_ERRS_INT_1: u32 = 0b1;
        }
    }

    /// ITCM Access multi-bit ECC Error Interrupt Status
    pub mod ITCM_ECC_ERRM_INT {
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

            /// 0b0: ITCM multi-bit ECC error does not happen
            pub const ITCM_ECC_ERRM_INT_0: u32 = 0b0;

            /// 0b1: ITCM multi-bit ECC error happens.
            pub const ITCM_ECC_ERRM_INT_1: u32 = 0b1;
        }
    }

    /// ITCM access single-bit ECC Error Interrupt Status
    pub mod ITCM_ECC_ERRS_INT {
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

            /// 0b0: ITCM single-bit ECC error does not happen
            pub const ITCM_ECC_ERRS_INT_0: u32 = 0b0;

            /// 0b1: ITCM single-bit ECC error happens.
            pub const ITCM_ECC_ERRS_INT_1: u32 = 0b1;
        }
    }

    /// D0TCM access multi-bit ECC Error Interrupt Status
    pub mod D0TCM_ECC_ERRM_INT {
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

            /// 0b0: D0TCM multi-bit ECC error does not happen
            pub const D0TCM_ECC_ERRM_INT_0: u32 = 0b0;

            /// 0b1: D0TCM multi-bit ECC error happens.
            pub const D0TCM_ECC_ERRM_INT_1: u32 = 0b1;
        }
    }

    /// D0TCM access single-bit ECC Error Interrupt Status
    pub mod D0TCM_ECC_ERRS_INT {
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

            /// 0b0: D0TCM single-bit ECC error does not happen
            pub const D0TCM_ECC_ERRS_INT_0: u32 = 0b0;

            /// 0b1: D0TCM single-bit ECC error happens.
            pub const D0TCM_ECC_ERRS_INT_1: u32 = 0b1;
        }
    }

    /// D1TCM access multi-bit ECC Error Interrupt Status
    pub mod D1TCM_ECC_ERRM_INT {
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

            /// 0b0: D1TCM multi-bit ECC error does not happen
            pub const D1TCM_ECC_ERRM_INT_0: u32 = 0b0;

            /// 0b1: D1TCM multi-bit ECC error happens.
            pub const D1TCM_ECC_ERRM_INT_1: u32 = 0b1;
        }
    }

    /// D1TCM access single-bit ECC Error Interrupt Status
    pub mod D1TCM_ECC_ERRS_INT {
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

            /// 0b0: D1TCM single-bit ECC error does not happen
            pub const D1TCM_ECC_ERRS_INT_0: u32 = 0b0;

            /// 0b1: D1TCM single-bit ECC error happens.
            pub const D1TCM_ECC_ERRS_INT_1: u32 = 0b1;
        }
    }

    /// ITCM Partial Write Interrupt Status
    pub mod ITCM_PARTIAL_WR_INT_S {
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

            /// 0b0: ITCM Partial Write does not happen
            pub const ITCM_PARTIAL_WR_INT_S_0: u32 = 0b0;

            /// 0b1: ITCM Partial Write happens.
            pub const ITCM_PARTIAL_WR_INT_S_1: u32 = 0b1;
        }
    }

    /// D0TCM Partial Write Interrupt Status
    pub mod D0TCM_PARTIAL_WR_INT_S {
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

            /// 0b0: D0TCM Partial Write does not happen
            pub const D0TCM_PARTIAL_WR_INT_S_0: u32 = 0b0;

            /// 0b1: D0TCM Partial Write happens.
            pub const D0TCM_PARTIAL_WR_INT_S_1: u32 = 0b1;
        }
    }

    /// D1TCM Partial Write Interrupt Status
    pub mod D1TCM_PARTIAL_WR_INT_S {
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

            /// 0b0: D1TCM Partial Write does not happen
            pub const D1TCM_PARTIAL_WR_INT_S_0: u32 = 0b0;

            /// 0b1: D1TCM Partial Write happens.
            pub const D1TCM_PARTIAL_WR_INT_S_1: u32 = 0b1;
        }
    }

    /// OCRAM Partial Write Interrupt Status
    pub mod OCRAM_PARTIAL_WR_INT_S {
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

            /// 0b0: OCRAM Partial Write does not happen
            pub const OCRAM_PARTIAL_WR_INT_S_0: u32 = 0b0;

            /// 0b1: OCRAM Partial Write happens.
            pub const OCRAM_PARTIAL_WR_INT_S_1: u32 = 0b1;
        }
    }

    /// Reserved
    pub mod Reserved {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (14 bits: 0x3fff << 18)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Status Enable Register
pub mod INT_STAT_EN {

    /// ITCM Magic Address Match Status Enable
    pub mod ITCM_MAM_STAT_EN {
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
            pub const ITCM_MAM_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_MAM_STAT_EN_1: u32 = 0b1;
        }
    }

    /// DTCM Magic Address Match Status Enable
    pub mod DTCM_MAM_STAT_EN {
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
            pub const DTCM_MAM_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DTCM_MAM_STAT_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Magic Address Match Status Enable
    pub mod OCRAM_MAM_STAT_EN {
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
            pub const OCRAM_MAM_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_MAM_STAT_EN_1: u32 = 0b1;
        }
    }

    /// ITCM Access Error Status Enable
    pub mod ITCM_ERR_STAT_EN {
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
            pub const ITCM_ERR_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_ERR_STAT_EN_1: u32 = 0b1;
        }
    }

    /// DTCM Access Error Status Enable
    pub mod DTCM_ERR_STAT_EN {
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
            pub const DTCM_ERR_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DTCM_ERR_STAT_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Status Enable
    pub mod OCRAM_ERR_STAT_EN {
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
            pub const OCRAM_ERR_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_ERR_STAT_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access multi-bit ECC Error Interrupt Status Enable
    pub mod OCRAM_ERRM_INT_EN {
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
            pub const OCRAM_ERRM_INT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_ERRM_INT_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access single-bit ECC Error Interrupt Status Enable
    pub mod OCRAM_ERRS_INT_EN {
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
            pub const OCRAM_ERRS_INT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_ERRS_INT_EN_1: u32 = 0b1;
        }
    }

    /// ITCM Access multi-bit ECC Error Interrupt Status Enable
    pub mod ITCM_ERRM_INT_EN {
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
            pub const ITCM_ERRM_INT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_ERRM_INT_EN_1: u32 = 0b1;
        }
    }

    /// ITCM Access single-bit ECC Error Interrupt Status Enable
    pub mod ITCM_ERRS_INT_EN {
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

            /// 0b0: Masked
            pub const ITCM_ERRS_INT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_ERRS_INT_EN_1: u32 = 0b1;
        }
    }

    /// D0TCM Access multi-bit ECC Error Interrupt Status Enable
    pub mod D0TCM_ERRM_INT_EN {
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

            /// 0b0: Masked
            pub const D0TCM_ERRM_INT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const D0TCM_ERRM_INT_EN_1: u32 = 0b1;
        }
    }

    /// D0TCM Access single-bit ECC Error Interrupt Status Enable
    pub mod D0TCM_ERRS_INT_EN {
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

            /// 0b0: Masked
            pub const D0TCM_ERRS_INT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const D0TCM_ERRS_INT_EN_1: u32 = 0b1;
        }
    }

    /// D1TCM Access multi-bit ECC Error Interrupt Status Enable
    pub mod D1TCM_ERRM_INT_EN {
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
            pub const D1TCM_ERRM_INT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const D1TCM_ERRM_INT_EN_1: u32 = 0b1;
        }
    }

    /// D1TCM Access single-bit ECC Error Interrupt Status Enable
    pub mod D1TCM_ERRS_INT_EN {
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

            /// 0b0: Masked
            pub const D1TCM_ERRS_INT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const D1TCM_ERRS_INT_EN_1: u32 = 0b1;
        }
    }

    /// ITCM Partial Write Interrupt Status Enable
    pub mod ITCM_PARTIAL_WR_INT_S_EN {
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
            pub const ITCM_PARTIAL_WR_INT_S_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_PARTIAL_WR_INT_S_EN_1: u32 = 0b1;
        }
    }

    /// D0TCM Partial Write Interrupt Status Enable
    pub mod D0TCM_PARTIAL_WR_INT_S_EN {
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

            /// 0b0: Masked
            pub const D0TCM_PARTIAL_WR_INT_S_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const D0TCM_PARTIAL_WR_INT_S_EN_1: u32 = 0b1;
        }
    }

    /// D1TCM Partial Write Interrupt Status EN
    pub mod D1TCM_PARTIAL_WR_INT_S_EN {
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
            pub const D1TCM_PARTIAL_WR_INT_S_EN_0: u32 = 0b0;

            /// 0b1: Enbaled
            pub const D1TCM_PARTIAL_WR_INT_S_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Partial Write Interrupt Status
    pub mod OCRAM_PARTIAL_WR_INT_S_EN {
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
            pub const OCRAM_PARTIAL_WR_INT_S_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_PARTIAL_WR_INT_S_EN_1: u32 = 0b1;
        }
    }

    /// Reserved
    pub mod Reserved {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (14 bits: 0x3fff << 18)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Enable Register
pub mod INT_SIG_EN {

    /// ITCM Magic Address Match Interrupt Enable
    pub mod ITCM_MAM_SIG_EN {
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
            pub const ITCM_MAM_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_MAM_SIG_EN_1: u32 = 0b1;
        }
    }

    /// DTCM Magic Address Match Interrupt Enable
    pub mod DTCM_MAM_SIG_EN {
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
            pub const DTCM_MAM_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DTCM_MAM_SIG_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Magic Address Match Interrupt Enable
    pub mod OCRAM_MAM_SIG_EN {
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
            pub const OCRAM_MAM_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_MAM_SIG_EN_1: u32 = 0b1;
        }
    }

    /// ITCM Access Error Interrupt Enable
    pub mod ITCM_ERR_SIG_EN {
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
            pub const ITCM_ERR_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_ERR_SIG_EN_1: u32 = 0b1;
        }
    }

    /// DTCM Access Error Interrupt Enable
    pub mod DTCM_ERR_SIG_EN {
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
            pub const DTCM_ERR_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DTCM_ERR_SIG_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Interrupt Enable
    pub mod OCRAM_ERR_SIG_EN {
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
            pub const OCRAM_ERR_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_ERR_SIG_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access multi-bit ECC Error Interrupt Signal Enable
    pub mod OCRAM_ERRM_INT_SIG_EN {
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
            pub const OCRAM_ERRM_INT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_ERRM_INT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access single-bit ECC Error Interrupt Signal Enable
    pub mod OCRAM_ERRS_INT_SIG_EN {
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
            pub const OCRAM_ERRS_INT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_ERRS_INT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// ITCM Access multi-bit ECC Error Interrupt Signal Enable
    pub mod ITCM_ERRM_INT_SIG_EN {
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
            pub const ITCM_ERRM_INT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_ERRM_INT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// ITCM Access single-bit ECC Error Interrupt Signal Enable
    pub mod ITCM_ERRS_INT_SIG_EN {
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

            /// 0b0: Masked
            pub const ITCM_ERRS_INT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_ERRS_INT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// D0TCM Access multi-bit ECC Error Interrupt Signal Enable
    pub mod D0TCM_ERRM_INT_SIG_EN {
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

            /// 0b0: Masked
            pub const D0TCM_ERRM_INT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const D0TCM_ERRM_INT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// D0TCM Access single-bit ECC Error Interrupt Signal Enable
    pub mod D0TCM_ERRS_INT_SIG_EN {
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

            /// 0b0: Masked
            pub const D0TCM_ERRS_INT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const D0TCM_ERRS_INT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// D1TCM Access multi-bit ECC Error Interrupt Signal Enable
    pub mod D1TCM_ERRM_INT_SIG_EN {
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
            pub const D1TCM_ERRM_INT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const D1TCM_ERRM_INT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// D1TCM Access single-bit ECC Error Interrupt Signal Enable
    pub mod D1TCM_ERRS_INT_SIG_EN {
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

            /// 0b0: Masked
            pub const D1TCM_ERRS_INT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const D1TCM_ERRS_INT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// ITCM Partial Write Interrupt Signal Enable Enable
    pub mod ITCM_PARTIAL_WR_INT_SIG_EN {
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
            pub const ITCM_PARTIAL_WR_INT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_PARTIAL_WR_INT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// D0TCM Partial Write Interrupt Signal Enable Enable
    pub mod D0TCM_PARTIAL_WR_INT_SIG_EN {
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

            /// 0b0: Masked
            pub const D0TCM_PARTIAL_WR_INT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const D0TCM_PARTIAL_WR_INT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// D1TCM Partial Write Interrupt Signal Enable EN
    pub mod D1TCM_PARTIAL_WR_INT_SIG_EN {
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
            pub const D1TCM_PARTIAL_WR_INT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enbaled
            pub const D1TCM_PARTIAL_WR_INT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Partial Write Interrupt Signal Enable
    pub mod OCRAM_PARTIAL_WR_INT_SIG_EN {
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
            pub const OCRAM_PARTIAL_WR_INT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_PARTIAL_WR_INT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// Reserved
    pub mod Reserved {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (14 bits: 0x3fff << 18)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OCRAM single-bit ECC Error Information Register
pub mod OCRAM_ECC_SINGLE_ERROR_INFO {

    /// corresponding ECC cipher of OCRAM single-bit ECC error
    pub mod OCRAM_ECCS_ERRED_ECC {
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

    /// corresponding ECC syndrome of OCRAM single-bit ECC error
    pub mod OCRAM_ECCS_ERRED_SYN {
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

    /// Reserved
    pub mod Reserved {
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

/// OCRAM single-bit ECC Error Address Register
pub mod OCRAM_ECC_SINGLE_ERROR_ADDR {

    /// OCRAM single-bit ECC error address
    pub mod OCRAM_ECCS_ERRED_ADDR {
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

/// OCRAM single-bit ECC Error Data Register
pub mod OCRAM_ECC_SINGLE_ERROR_DATA_LSB {

    /// OCRAM single-bit ECC error data \[31:0\]
    pub mod OCRAM_ECCS_ERRED_DATA_LSB {
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

/// OCRAM single-bit ECC Error Data Register
pub mod OCRAM_ECC_SINGLE_ERROR_DATA_MSB {

    /// OCRAM single-bit ECC error data \[63:32\]
    pub mod OCRAM_ECCS_ERRED_DATA_MSB {
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

/// OCRAM multi-bit ECC Error Information Register
pub mod OCRAM_ECC_MULTI_ERROR_INFO {

    /// OCRAM multi-bit ECC error corresponding ECC value
    pub mod OCRAM_ECCM_ERRED_ECC {
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

    /// Reserved
    pub mod Reserved {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OCRAM multi-bit ECC Error Address Register
pub mod OCRAM_ECC_MULTI_ERROR_ADDR {

    /// OCRAM multi-bit ECC error address
    pub mod OCRAM_ECCM_ERRED_ADDR {
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

/// OCRAM multi-bit ECC Error Data Register
pub mod OCRAM_ECC_MULTI_ERROR_DATA_LSB {

    /// OCRAM multi-bit ECC error data \[31:0\]
    pub mod OCRAM_ECCM_ERRED_DATA_LSB {
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

/// OCRAM multi-bit ECC Error Data Register
pub mod OCRAM_ECC_MULTI_ERROR_DATA_MSB {

    /// OCRAM multi-bit ECC error data \[63:32\]
    pub mod OCRAM_ECCM_ERRED_DATA_MSB {
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

/// ITCM single-bit ECC Error Information Register
pub mod ITCM_ECC_SINGLE_ERROR_INFO {

    /// ITCM single-bit ECC error corresponding TCM_WR value.
    pub mod ITCM_ECCS_EFW {
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

    /// ITCM single-bit ECC error corresponding TCM size
    pub mod ITCM_ECCS_EFSIZ {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ITCM single-bit ECC error corresponding TCM_MASTER.
    pub mod ITCM_ECCS_EFMST {
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

    /// ITCM single-bit ECC error corresponding TCM_PRIV.
    pub mod ITCM_ECCS_EFPRT {
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

    /// ITCM single-bit ECC error corresponding syndrome
    pub mod ITCM_ECCS_EFSYN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (8 bits: 0xff << 12)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved
    pub mod Reserved {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (12 bits: 0xfff << 20)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ITCM single-bit ECC Error Address Register
pub mod ITCM_ECC_SINGLE_ERROR_ADDR {

    /// ITCM single-bit ECC error address
    pub mod ITCM_ECCS_ERRED_ADDR {
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

/// ITCM single-bit ECC Error Data Register
pub mod ITCM_ECC_SINGLE_ERROR_DATA_LSB {

    /// ITCM single-bit ECC error data \[31:0\]
    pub mod ITCM_ECCS_ERRED_DATA_LSB {
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

/// ITCM single-bit ECC Error Data Register
pub mod ITCM_ECC_SINGLE_ERROR_DATA_MSB {

    /// ITCM single-bit ECC error data \[63:32\]
    pub mod ITCM_ECCS_ERRED_DATA_MSB {
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

/// ITCM multi-bit ECC Error Information Register
pub mod ITCM_ECC_MULTI_ERROR_INFO {

    /// ITCM multi-bit ECC error corresponding TCM_WR value
    pub mod ITCM_ECCM_EFW {
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

    /// ITCM multi-bit ECC error corresponding tcm access size
    pub mod ITCM_ECCM_EFSIZ {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ITCM multi-bit ECC error corresponding TCM_MASTER
    pub mod ITCM_ECCM_EFMST {
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

    /// ITCM multi-bit ECC error corresponding TCM_PRIV
    pub mod ITCM_ECCM_EFPRT {
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

    /// ITCM multi-bit ECC error corresponding syndrome
    pub mod ITCM_ECCM_EFSYN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (8 bits: 0xff << 12)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved
    pub mod Reserved {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (12 bits: 0xfff << 20)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ITCM multi-bit ECC Error Address Register
pub mod ITCM_ECC_MULTI_ERROR_ADDR {

    /// ITCM multi-bit ECC error address
    pub mod ITCM_ECCM_ERRED_ADDR {
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

/// ITCM multi-bit ECC Error Data Register
pub mod ITCM_ECC_MULTI_ERROR_DATA_LSB {

    /// ITCM multi-bit ECC error data \[31:0\]
    pub mod ITCM_ECCM_ERRED_DATA_LSB {
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

/// ITCM multi-bit ECC Error Data Register
pub mod ITCM_ECC_MULTI_ERROR_DATA_MSB {

    /// ITCM multi-bit ECC error data \[63:32\]
    pub mod ITCM_ECCM_ERRED_DATA_MSB {
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

/// D0TCM single-bit ECC Error Information Register
pub mod D0TCM_ECC_SINGLE_ERROR_INFO {

    /// D0TCM single-bit ECC error corresponding TCM_WR value
    pub mod D0TCM_ECCS_EFW {
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

    /// D0TCM single-bit ECC error corresponding tcm access size
    pub mod D0TCM_ECCS_EFSIZ {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// D0TCM single-bit ECC error corresponding TCM_MASTER
    pub mod D0TCM_ECCS_EFMST {
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

    /// D0TCM single-bit ECC error corresponding TCM_PRIV
    pub mod D0TCM_ECCS_EFPRT {
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

    /// D0TCM single-bit ECC error corresponding syndrome
    pub mod D0TCM_ECCS_EFSYN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (7 bits: 0x7f << 12)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved
    pub mod Reserved {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (13 bits: 0x1fff << 19)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// D0TCM single-bit ECC Error Address Register
pub mod D0TCM_ECC_SINGLE_ERROR_ADDR {

    /// D0TCM single-bit ECC error address
    pub mod D0TCM_ECCS_ERRED_ADDR {
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

/// D0TCM single-bit ECC Error Data Register
pub mod D0TCM_ECC_SINGLE_ERROR_DATA {

    /// D0TCM single-bit ECC error data
    pub mod D0TCM_ECCS_ERRED_DATA {
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

/// D0TCM multi-bit ECC Error Information Register
pub mod D0TCM_ECC_MULTI_ERROR_INFO {

    /// D0TCM multi-bit ECC error corresponding TCM_WR value
    pub mod D0TCM_ECCM_EFW {
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

    /// D0TCM multi-bit ECC error corresponding tcm access size
    pub mod D0TCM_ECCM_EFSIZ {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// D0TCM multi-bit ECC error corresponding TCM_MASTER
    pub mod D0TCM_ECCM_EFMST {
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

    /// D0TCM multi-bit ECC error corresponding TCM_PRIV
    pub mod D0TCM_ECCM_EFPRT {
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

    /// D0TCM multi-bit ECC error corresponding syndrome
    pub mod D0TCM_ECCM_EFSYN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (7 bits: 0x7f << 12)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved
    pub mod Reserved {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (13 bits: 0x1fff << 19)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// D0TCM multi-bit ECC Error Address Register
pub mod D0TCM_ECC_MULTI_ERROR_ADDR {

    /// D0TCM multi-bit ECC error address
    pub mod D0TCM_ECCM_ERRED_ADDR {
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

/// D0TCM multi-bit ECC Error Data Register
pub mod D0TCM_ECC_MULTI_ERROR_DATA {

    /// D0TCM multi-bit ECC error data
    pub mod D0TCM_ECCM_ERRED_DATA {
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

/// D1TCM single-bit ECC Error Information Register
pub mod D1TCM_ECC_SINGLE_ERROR_INFO {

    /// D1TCM single-bit ECC error corresponding TCM_WR value
    pub mod D1TCM_ECCS_EFW {
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

    /// D1TCM single-bit ECC error corresponding tcm access size
    pub mod D1TCM_ECCS_EFSIZ {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// D1TCM single-bit ECC error corresponding TCM_MASTER
    pub mod D1TCM_ECCS_EFMST {
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

    /// D1TCM single-bit ECC error corresponding TCM_PRIV
    pub mod D1TCM_ECCS_EFPRT {
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

    /// D1TCM single-bit ECC error corresponding syndrome
    pub mod D1TCM_ECCS_EFSYN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (7 bits: 0x7f << 12)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved
    pub mod Reserved {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (13 bits: 0x1fff << 19)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// D1TCM single-bit ECC Error Address Register
pub mod D1TCM_ECC_SINGLE_ERROR_ADDR {

    /// D1TCM single-bit ECC error address
    pub mod D1TCM_ECCS_ERRED_ADDR {
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

/// D1TCM single-bit ECC Error Data Register
pub mod D1TCM_ECC_SINGLE_ERROR_DATA {

    /// D1TCM single-bit ECC error data
    pub mod D1TCM_ECCS_ERRED_DATA {
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

/// D1TCM multi-bit ECC Error Information Register
pub mod D1TCM_ECC_MULTI_ERROR_INFO {

    /// D1TCM multi-bit ECC error corresponding TCM_WR value
    pub mod D1TCM_ECCM_EFW {
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

    /// D1TCM multi-bit ECC error corresponding tcm access size
    pub mod D1TCM_ECCM_EFSIZ {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// D1TCM multi-bit ECC error corresponding TCM_MASTER
    pub mod D1TCM_ECCM_EFMST {
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

    /// D1TCM multi-bit ECC error corresponding TCM_PRIV
    pub mod D1TCM_ECCM_EFPRT {
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

    /// D1TCM multi-bit ECC error corresponding syndrome
    pub mod D1TCM_ECCM_EFSYN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (7 bits: 0x7f << 12)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved
    pub mod Reserved {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (13 bits: 0x1fff << 19)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// D1TCM multi-bit ECC Error Address Register
pub mod D1TCM_ECC_MULTI_ERROR_ADDR {

    /// D1TCM multi-bit ECC error address
    pub mod D1TCM_ECCM_ERRED_ADDR {
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

/// D1TCM multi-bit ECC Error Data Register
pub mod D1TCM_ECC_MULTI_ERROR_DATA {

    /// D1TCM multi-bit ECC error data
    pub mod D1TCM_ECCM_ERRED_DATA {
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

/// FlexRAM feature Control register
pub mod FLEXRAM_CTRL {

    /// Read Data Wait Enable
    pub mod OCRAM_RDATA_WAIT_EN {
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

    /// Read Address Pipeline Enable
    pub mod OCRAM_RADDR_PIPELINE_EN {
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

    /// Write Data Pipeline Enable
    pub mod OCRAM_WRDATA_PIPELINE_EN {
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

    /// Write Address Pipeline Enable
    pub mod OCRAM_WRADDR_PIPELINE_EN {
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

    /// OCRAM ECC enable
    pub mod OCRAM_ECC_EN {
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

    /// TCM ECC enable
    pub mod TCM_ECC_EN {
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

    /// Reserved
    pub mod Reserved {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (26 bits: 0x3ffffff << 6)
        pub const mask: u32 = 0x3ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OCRAM Pipeline Status register
pub mod OCRAM_PIPELINE_STATUS {

    /// Read Data Wait Enable Pending
    pub mod OCRAM_RDATA_WAIT_EN_UPDATA_PENDING {
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

    /// Read Address Pipeline Enable Pending
    pub mod OCRAM_RADDR_PIPELINE_EN_UPDATA_PENDING {
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

    /// Write Data Pipeline Enable Pending
    pub mod OCRAM_WRDATA_PIPELINE_EN_UPDATA_PENDING {
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

    /// Write Address Pipeline Enable Pending
    pub mod OCRAM_WRADDR_PIPELINE_EN_UPDATA_PENDING {
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

    /// Reserved
    pub mod Reserved {
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
#[repr(C)]
pub struct RegisterBlock {
    /// TCM CRTL Register
    pub TCM_CTRL: RWRegister<u32>,

    /// OCRAM Magic Address Register
    pub OCRAM_MAGIC_ADDR: RWRegister<u32>,

    /// DTCM Magic Address Register
    pub DTCM_MAGIC_ADDR: RWRegister<u32>,

    /// ITCM Magic Address Register
    pub ITCM_MAGIC_ADDR: RWRegister<u32>,

    /// Interrupt Status Register
    pub INT_STATUS: RWRegister<u32>,

    /// Interrupt Status Enable Register
    pub INT_STAT_EN: RWRegister<u32>,

    /// Interrupt Enable Register
    pub INT_SIG_EN: RWRegister<u32>,

    /// OCRAM single-bit ECC Error Information Register
    pub OCRAM_ECC_SINGLE_ERROR_INFO: RORegister<u32>,

    /// OCRAM single-bit ECC Error Address Register
    pub OCRAM_ECC_SINGLE_ERROR_ADDR: RORegister<u32>,

    /// OCRAM single-bit ECC Error Data Register
    pub OCRAM_ECC_SINGLE_ERROR_DATA_LSB: RORegister<u32>,

    /// OCRAM single-bit ECC Error Data Register
    pub OCRAM_ECC_SINGLE_ERROR_DATA_MSB: RORegister<u32>,

    /// OCRAM multi-bit ECC Error Information Register
    pub OCRAM_ECC_MULTI_ERROR_INFO: RORegister<u32>,

    /// OCRAM multi-bit ECC Error Address Register
    pub OCRAM_ECC_MULTI_ERROR_ADDR: RORegister<u32>,

    /// OCRAM multi-bit ECC Error Data Register
    pub OCRAM_ECC_MULTI_ERROR_DATA_LSB: RORegister<u32>,

    /// OCRAM multi-bit ECC Error Data Register
    pub OCRAM_ECC_MULTI_ERROR_DATA_MSB: RORegister<u32>,

    /// ITCM single-bit ECC Error Information Register
    pub ITCM_ECC_SINGLE_ERROR_INFO: RORegister<u32>,

    /// ITCM single-bit ECC Error Address Register
    pub ITCM_ECC_SINGLE_ERROR_ADDR: RORegister<u32>,

    /// ITCM single-bit ECC Error Data Register
    pub ITCM_ECC_SINGLE_ERROR_DATA_LSB: RORegister<u32>,

    /// ITCM single-bit ECC Error Data Register
    pub ITCM_ECC_SINGLE_ERROR_DATA_MSB: RORegister<u32>,

    /// ITCM multi-bit ECC Error Information Register
    pub ITCM_ECC_MULTI_ERROR_INFO: RORegister<u32>,

    /// ITCM multi-bit ECC Error Address Register
    pub ITCM_ECC_MULTI_ERROR_ADDR: RORegister<u32>,

    /// ITCM multi-bit ECC Error Data Register
    pub ITCM_ECC_MULTI_ERROR_DATA_LSB: RORegister<u32>,

    /// ITCM multi-bit ECC Error Data Register
    pub ITCM_ECC_MULTI_ERROR_DATA_MSB: RORegister<u32>,

    /// D0TCM single-bit ECC Error Information Register
    pub D0TCM_ECC_SINGLE_ERROR_INFO: RORegister<u32>,

    /// D0TCM single-bit ECC Error Address Register
    pub D0TCM_ECC_SINGLE_ERROR_ADDR: RORegister<u32>,

    /// D0TCM single-bit ECC Error Data Register
    pub D0TCM_ECC_SINGLE_ERROR_DATA: RORegister<u32>,

    /// D0TCM multi-bit ECC Error Information Register
    pub D0TCM_ECC_MULTI_ERROR_INFO: RORegister<u32>,

    /// D0TCM multi-bit ECC Error Address Register
    pub D0TCM_ECC_MULTI_ERROR_ADDR: RORegister<u32>,

    /// D0TCM multi-bit ECC Error Data Register
    pub D0TCM_ECC_MULTI_ERROR_DATA: RORegister<u32>,

    /// D1TCM single-bit ECC Error Information Register
    pub D1TCM_ECC_SINGLE_ERROR_INFO: RORegister<u32>,

    /// D1TCM single-bit ECC Error Address Register
    pub D1TCM_ECC_SINGLE_ERROR_ADDR: RORegister<u32>,

    /// D1TCM single-bit ECC Error Data Register
    pub D1TCM_ECC_SINGLE_ERROR_DATA: RORegister<u32>,

    /// D1TCM multi-bit ECC Error Information Register
    pub D1TCM_ECC_MULTI_ERROR_INFO: RORegister<u32>,

    /// D1TCM multi-bit ECC Error Address Register
    pub D1TCM_ECC_MULTI_ERROR_ADDR: RORegister<u32>,

    /// D1TCM multi-bit ECC Error Data Register
    pub D1TCM_ECC_MULTI_ERROR_DATA: RORegister<u32>,

    _reserved1: [u32; 31],

    /// FlexRAM feature Control register
    pub FLEXRAM_CTRL: RWRegister<u32>,

    /// OCRAM Pipeline Status register
    pub OCRAM_PIPELINE_STATUS: RORegister<u32>,
}
pub struct ResetValues {
    pub TCM_CTRL: u32,
    pub OCRAM_MAGIC_ADDR: u32,
    pub DTCM_MAGIC_ADDR: u32,
    pub ITCM_MAGIC_ADDR: u32,
    pub INT_STATUS: u32,
    pub INT_STAT_EN: u32,
    pub INT_SIG_EN: u32,
    pub OCRAM_ECC_SINGLE_ERROR_INFO: u32,
    pub OCRAM_ECC_SINGLE_ERROR_ADDR: u32,
    pub OCRAM_ECC_SINGLE_ERROR_DATA_LSB: u32,
    pub OCRAM_ECC_SINGLE_ERROR_DATA_MSB: u32,
    pub OCRAM_ECC_MULTI_ERROR_INFO: u32,
    pub OCRAM_ECC_MULTI_ERROR_ADDR: u32,
    pub OCRAM_ECC_MULTI_ERROR_DATA_LSB: u32,
    pub OCRAM_ECC_MULTI_ERROR_DATA_MSB: u32,
    pub ITCM_ECC_SINGLE_ERROR_INFO: u32,
    pub ITCM_ECC_SINGLE_ERROR_ADDR: u32,
    pub ITCM_ECC_SINGLE_ERROR_DATA_LSB: u32,
    pub ITCM_ECC_SINGLE_ERROR_DATA_MSB: u32,
    pub ITCM_ECC_MULTI_ERROR_INFO: u32,
    pub ITCM_ECC_MULTI_ERROR_ADDR: u32,
    pub ITCM_ECC_MULTI_ERROR_DATA_LSB: u32,
    pub ITCM_ECC_MULTI_ERROR_DATA_MSB: u32,
    pub D0TCM_ECC_SINGLE_ERROR_INFO: u32,
    pub D0TCM_ECC_SINGLE_ERROR_ADDR: u32,
    pub D0TCM_ECC_SINGLE_ERROR_DATA: u32,
    pub D0TCM_ECC_MULTI_ERROR_INFO: u32,
    pub D0TCM_ECC_MULTI_ERROR_ADDR: u32,
    pub D0TCM_ECC_MULTI_ERROR_DATA: u32,
    pub D1TCM_ECC_SINGLE_ERROR_INFO: u32,
    pub D1TCM_ECC_SINGLE_ERROR_ADDR: u32,
    pub D1TCM_ECC_SINGLE_ERROR_DATA: u32,
    pub D1TCM_ECC_MULTI_ERROR_INFO: u32,
    pub D1TCM_ECC_MULTI_ERROR_ADDR: u32,
    pub D1TCM_ECC_MULTI_ERROR_DATA: u32,
    pub FLEXRAM_CTRL: u32,
    pub OCRAM_PIPELINE_STATUS: u32,
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
