#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! XECC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// ECC Control Register
pub mod ECC_CTRL {

    /// ECC Function Enable
    pub mod ECC_EN {
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
            pub const ECC_EN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const ECC_EN_1: u32 = 0b1;
        }
    }

    /// Write ECC Encode Function Enable
    pub mod WECC_EN {
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
            pub const WECC_EN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const WECC_EN_1: u32 = 0b1;
        }
    }

    /// Read ECC Function Enable
    pub mod RECC_EN {
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
            pub const RECC_EN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const RECC_EN_1: u32 = 0b1;
        }
    }

    /// Swap Data Enable
    pub mod SWAP_EN {
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
            pub const SWAP_EN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const SWAP_EN_1: u32 = 0b1;
        }
    }
}

/// Error Interrupt Status Register
pub mod ERR_STATUS {

    /// Single Bit Error
    pub mod SINGLE_ERR {
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

            /// 0b0: Single bit error does not happen.
            pub const SINGLE_ERR_0: u32 = 0b0;

            /// 0b1: Single bit error happens.
            pub const SINGLE_ERR_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error
    pub mod MULTI_ERR {
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

            /// 0b0: Multiple bits error does not happen.
            pub const MULTI_ERR_0: u32 = 0b0;

            /// 0b1: Multiple bits error happens.
            pub const MULTI_ERR_1: u32 = 0b1;
        }
    }

    /// Reserved
    pub mod Reserved1 {
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

/// Error Interrupt Status Enable Register
pub mod ERR_STAT_EN {

    /// Single Bit Error Status Enable
    pub mod SINGLE_ERR_STAT_EN {
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
            pub const SINGLE_ERR_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SINGLE_ERR_STAT_EN_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error Status Enable
    pub mod MULIT_ERR_STAT_EN {
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
            pub const MULIT_ERR_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const MULIT_ERR_STAT_EN_1: u32 = 0b1;
        }
    }

    /// Reserved
    pub mod Reserved1 {
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

/// Error Interrupt Enable Register
pub mod ERR_SIG_EN {

    /// Single Bit Error Interrupt Enable
    pub mod SINGLE_ERR_SIG_EN {
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
            pub const SINGLE_ERR_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SINGLE_ERR_SIG_EN_1: u32 = 0b1;
        }
    }

    /// Multiple Bits Error Interrupt Enable
    pub mod MULTI_ERR_SIG_EN {
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
            pub const MULTI_ERR_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const MULTI_ERR_SIG_EN_1: u32 = 0b1;
        }
    }

    /// Reserved
    pub mod Reserved1 {
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

/// Error Injection On Write Data
pub mod ERR_DATA_INJ {

    /// Error Injection On Write Data
    pub mod ERR_DATA_INJ {
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

/// Error Injection On ECC Code of Write Data
pub mod ERR_ECC_INJ {

    /// Error Injection On ECC Code of Write Data
    pub mod ERR_ECC_INJ {
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

/// Single Error Address
pub mod SINGLE_ERR_ADDR {

    /// Single Error Address
    pub mod SINGLE_ERR_ADDR {
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

/// Single Error Read Data
pub mod SINGLE_ERR_DATA {

    /// Single Error Read Data
    pub mod SINGLE_ERR_DATA {
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

/// Single Error ECC Code
pub mod SINGLE_ERR_ECC {

    /// Single Error ECC code
    pub mod SINGLE_ERR_ECC {
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

/// Single Error Bit Position
pub mod SINGLE_ERR_POS {

    /// Single Error bit Position
    pub mod SINGLE_ERR_POS {
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

/// Single Error Bit Field
pub mod SINGLE_ERR_BIT_FIELD {

    /// Single Error Bit Field
    pub mod SINGLE_ERR_BIT_FIELD {
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
    pub mod Reserved1 {
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

/// Multiple Error Address
pub mod MULTI_ERR_ADDR {

    /// Multiple Error Address
    pub mod MULTI_ERR_ADDR {
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

/// Multiple Error Read Data
pub mod MULTI_ERR_DATA {

    /// Multiple Error Read Data
    pub mod MULTI_ERR_DATA {
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

/// Multiple Error ECC code
pub mod MULTI_ERR_ECC {

    /// Multiple Error ECC code
    pub mod MULTI_ERR_ECC {
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

/// Multiple Error Bit Field
pub mod MULTI_ERR_BIT_FIELD {

    /// Multiple Error Bit Field
    pub mod MULTI_ERR_BIT_FIELD {
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
    pub mod Reserved1 {
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

/// ECC Region 0 Base Address
pub mod ECC_BASE_ADDR0 {

    /// ECC Region 0 Base Address
    pub mod ECC_BASE_ADDR0 {
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

/// ECC Region 0 End Address
pub mod ECC_END_ADDR0 {

    /// ECC Region 0 End Address
    pub mod ECC_END_ADDR0 {
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

/// ECC Region 1 Base Address
pub mod ECC_BASE_ADDR1 {

    /// ECC Region 1 Base Address
    pub mod ECC_BASE_ADDR1 {
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

/// ECC Region 1 End Address
pub mod ECC_END_ADDR1 {

    /// ECC Region 1 End Address
    pub mod ECC_END_ADDR1 {
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

/// ECC Region 2 Base Address
pub mod ECC_BASE_ADDR2 {

    /// ECC Region 2 Base Address
    pub mod ECC_BASE_ADDR2 {
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

/// ECC Region 2 End Address
pub mod ECC_END_ADDR2 {

    /// ECC Region 2 End Address
    pub mod ECC_END_ADDR2 {
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

/// ECC Region 3 Base Address
pub mod ECC_BASE_ADDR3 {

    /// ECC Region 3 Base Address
    pub mod ECC_BASE_ADDR3 {
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

/// ECC Region 3 End Address
pub mod ECC_END_ADDR3 {

    /// ECC Region 3 End Address
    pub mod ECC_END_ADDR3 {
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
#[repr(C)]
pub struct RegisterBlock {
    /// ECC Control Register
    pub ECC_CTRL: RWRegister<u32>,

    /// Error Interrupt Status Register
    pub ERR_STATUS: RWRegister<u32>,

    /// Error Interrupt Status Enable Register
    pub ERR_STAT_EN: RWRegister<u32>,

    /// Error Interrupt Enable Register
    pub ERR_SIG_EN: RWRegister<u32>,

    /// Error Injection On Write Data
    pub ERR_DATA_INJ: RWRegister<u32>,

    /// Error Injection On ECC Code of Write Data
    pub ERR_ECC_INJ: RWRegister<u32>,

    /// Single Error Address
    pub SINGLE_ERR_ADDR: RORegister<u32>,

    /// Single Error Read Data
    pub SINGLE_ERR_DATA: RORegister<u32>,

    /// Single Error ECC Code
    pub SINGLE_ERR_ECC: RORegister<u32>,

    /// Single Error Bit Position
    pub SINGLE_ERR_POS: RORegister<u32>,

    /// Single Error Bit Field
    pub SINGLE_ERR_BIT_FIELD: RORegister<u32>,

    /// Multiple Error Address
    pub MULTI_ERR_ADDR: RORegister<u32>,

    /// Multiple Error Read Data
    pub MULTI_ERR_DATA: RORegister<u32>,

    /// Multiple Error ECC code
    pub MULTI_ERR_ECC: RORegister<u32>,

    /// Multiple Error Bit Field
    pub MULTI_ERR_BIT_FIELD: RORegister<u32>,

    /// ECC Region 0 Base Address
    pub ECC_BASE_ADDR0: RWRegister<u32>,

    /// ECC Region 0 End Address
    pub ECC_END_ADDR0: RWRegister<u32>,

    /// ECC Region 1 Base Address
    pub ECC_BASE_ADDR1: RWRegister<u32>,

    /// ECC Region 1 End Address
    pub ECC_END_ADDR1: RWRegister<u32>,

    /// ECC Region 2 Base Address
    pub ECC_BASE_ADDR2: RWRegister<u32>,

    /// ECC Region 2 End Address
    pub ECC_END_ADDR2: RWRegister<u32>,

    /// ECC Region 3 Base Address
    pub ECC_BASE_ADDR3: RWRegister<u32>,

    /// ECC Region 3 End Address
    pub ECC_END_ADDR3: RWRegister<u32>,
}
pub struct ResetValues {
    pub ECC_CTRL: u32,
    pub ERR_STATUS: u32,
    pub ERR_STAT_EN: u32,
    pub ERR_SIG_EN: u32,
    pub ERR_DATA_INJ: u32,
    pub ERR_ECC_INJ: u32,
    pub SINGLE_ERR_ADDR: u32,
    pub SINGLE_ERR_DATA: u32,
    pub SINGLE_ERR_ECC: u32,
    pub SINGLE_ERR_POS: u32,
    pub SINGLE_ERR_BIT_FIELD: u32,
    pub MULTI_ERR_ADDR: u32,
    pub MULTI_ERR_DATA: u32,
    pub MULTI_ERR_ECC: u32,
    pub MULTI_ERR_BIT_FIELD: u32,
    pub ECC_BASE_ADDR0: u32,
    pub ECC_END_ADDR0: u32,
    pub ECC_BASE_ADDR1: u32,
    pub ECC_END_ADDR1: u32,
    pub ECC_BASE_ADDR2: u32,
    pub ECC_END_ADDR2: u32,
    pub ECC_BASE_ADDR3: u32,
    pub ECC_END_ADDR3: u32,
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
