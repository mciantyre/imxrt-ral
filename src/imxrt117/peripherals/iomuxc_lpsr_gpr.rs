#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC LPSR GPR
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// GPR0 General Purpose Register
pub mod GPR0 {

    /// CM4 Vector table offset value lower bits out of reset
    pub mod CM4_INIT_VTOR_LOW {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (13 bits: 0x1fff << 3)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR1 General Purpose Register
pub mod GPR1 {

    /// CM4 Vector table offset value higher bits out of reset
    pub mod CM4_INIT_VTOR_HIGH {
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

    /// Domain write protection
    pub mod DWP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR2 General Purpose Register
pub mod GPR2 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC start address of memory region-0
    pub mod APC_AC_R0_BOT {
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

/// GPR3 General Purpose Register
pub mod GPR3 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC end address of memory region-0
    pub mod APC_AC_R0_TOP {
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

/// GPR4 General Purpose Register
pub mod GPR4 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC start address of memory region-1
    pub mod APC_AC_R1_BOT {
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

/// GPR5 General Purpose Register
pub mod GPR5 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC end address of memory region-1
    pub mod APC_AC_R1_TOP {
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

/// GPR6 General Purpose Register
pub mod GPR6 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC start address of memory region-2
    pub mod APC_AC_R2_BOT {
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

/// GPR7 General Purpose Register
pub mod GPR7 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC end address of memory region-2
    pub mod APC_AC_R2_TOP {
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

/// GPR8 General Purpose Register
pub mod GPR8 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC start address of memory region-3
    pub mod APC_AC_R3_BOT {
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

/// GPR9 General Purpose Register
pub mod GPR9 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC end address of memory region-3
    pub mod APC_AC_R3_TOP {
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

/// GPR10 General Purpose Register
pub mod GPR10 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC start address of memory region-4
    pub mod APC_AC_R4_BOT {
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

/// GPR11 General Purpose Register
pub mod GPR11 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC end address of memory region-4
    pub mod APC_AC_R4_TOP {
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

/// GPR12 General Purpose Register
pub mod GPR12 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC start address of memory region-5
    pub mod APC_AC_R5_BOT {
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

/// GPR13 General Purpose Register
pub mod GPR13 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC end address of memory region-5
    pub mod APC_AC_R5_TOP {
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

/// GPR14 General Purpose Register
pub mod GPR14 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC start address of memory region-6
    pub mod APC_AC_R6_BOT {
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

/// GPR15 General Purpose Register
pub mod GPR15 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC end address of memory region-6
    pub mod APC_AC_R6_TOP {
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

/// GPR16 General Purpose Register
pub mod GPR16 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC start address of memory region-7
    pub mod APC_AC_R7_BOT {
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

/// GPR17 General Purpose Register
pub mod GPR17 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
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

            /// 0b0: Write access to bit 31:1 is not blocked
            pub const NO: u32 = 0b0;

            /// 0b1: Write access to bit 31:1 is blocked
            pub const BLOCK: u32 = 0b1;
        }
    }

    /// APC end address of memory region-7
    pub mod APC_AC_R7_TOP {
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

/// GPR18 General Purpose Register
pub mod GPR18 {

    /// APC memory region-0 encryption enable
    pub mod APC_R0_ENCRYPT_ENABLE {
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

            /// 0b0: No effect
            pub const DIS: u32 = 0b0;

            /// 0b1: Encryption enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Lock the write to bit 15:0
    pub mod LOCK {
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

/// GPR19 General Purpose Register
pub mod GPR19 {

    /// APC memory region-1 encryption enable
    pub mod APC_R1_ENCRYPT_ENABLE {
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

            /// 0b0: No effect
            pub const DIS: u32 = 0b0;

            /// 0b1: Encryption enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Lock the write to bit 15:0
    pub mod LOCK {
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

/// GPR20 General Purpose Register
pub mod GPR20 {

    /// APC memory region-2 encryption enable
    pub mod APC_R2_ENCRYPT_ENABLE {
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

            /// 0b0: No effect
            pub const DIS: u32 = 0b0;

            /// 0b1: Encryption enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Lock the write to bit 15:0
    pub mod LOCK {
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

/// GPR21 General Purpose Register
pub mod GPR21 {

    /// APC memory region-3 encryption enable
    pub mod APC_R3_ENCRYPT_ENABLE {
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

            /// 0b0: No effect
            pub const DIS: u32 = 0b0;

            /// 0b1: Encryption enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Lock the write to bit 15:0
    pub mod LOCK {
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

/// GPR22 General Purpose Register
pub mod GPR22 {

    /// APC memory region-4 encryption enable
    pub mod APC_R4_ENCRYPT_ENABLE {
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

            /// 0b0: No effect
            pub const DIS: u32 = 0b0;

            /// 0b1: Encryption enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Lock the write to bit 15:0
    pub mod LOCK {
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

/// GPR23 General Purpose Register
pub mod GPR23 {

    /// APC memory region-5 encryption enable
    pub mod APC_R5_ENCRYPT_ENABLE {
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

            /// 0b0: No effect
            pub const DIS: u32 = 0b0;

            /// 0b1: Encryption enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Lock the write to bit 15:0
    pub mod LOCK {
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

/// GPR24 General Purpose Register
pub mod GPR24 {

    /// APC memory region-6 encryption enable
    pub mod APC_R6_ENCRYPT_ENABLE {
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

            /// 0b0: No effect
            pub const DIS: u32 = 0b0;

            /// 0b1: Encryption enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Lock the write to bit 15:0
    pub mod LOCK {
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

/// GPR25 General Purpose Register
pub mod GPR25 {

    /// APC memory region-7 encryption enable
    pub mod APC_R7_ENCRYPT_ENABLE {
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

            /// 0b0: No effect
            pub const DIS: u32 = 0b0;

            /// 0b1: Encryption enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// APC global enable bit
    pub mod APC_VALID {
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

            /// 0b0: No effect
            pub const DIS: u32 = 0b0;

            /// 0b1: Enable encryption for GPRx\[APC_x_ENCRYPT_ENABLE\] (valid for GPR2-GPR25)
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Lock the write to bit 15:0
    pub mod LOCK {
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

/// GPR26 General Purpose Register
pub mod GPR26 {

    /// Vector table offset register out of reset. See the ARM v7-M Architecture Reference Manual for more information about the vector table offset register (VTOR).
    pub mod CM7_INIT_VTOR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (25 bits: 0x1ffffff << 0)
        pub const mask: u32 = 0x1ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// General purpose bits
    pub mod FIELD_0 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (3 bits: 0b111 << 25)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain write protection
    pub mod DWP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR33 General Purpose Register
pub mod GPR33 {

    /// Clear CM4 NMI holding register
    pub mod M4_NMI_CLEAR {
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

    /// Clear USBPHY1 wakeup interrupt holding register
    pub mod USBPHY1_WAKEUP_IRQ_CLEAR {
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

    /// Clear USBPHY1 wakeup interrupt holding register
    pub mod USBPHY2_WAKEUP_IRQ_CLEAR {
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

    /// Domain write protection
    pub mod DWP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR34 General Purpose Register
pub mod GPR34 {

    /// GPIO_LPSR IO bank supply voltage range selection
    pub mod GPIO_LPSR_HIGH_RANGE {
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

    /// GPIO_LPSR IO bank supply voltage range selection
    pub mod GPIO_LPSR_LOW_RANGE {
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

    /// Mask CM7 NMI pin input
    pub mod M7_NMI_MASK {
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

            /// 0b0: NMI input from IO to CM7 is not blocked
            pub const DISABLE: u32 = 0b0;

            /// 0b1: NMI input from IO to CM7 is blocked
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Mask CM4 NMI pin input
    pub mod M4_NMI_MASK {
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

            /// 0b0: NMI input from IO to CM4 is not blocked
            pub const DISABLE: u32 = 0b0;

            /// 0b1: NMI input from IO to CM4 is blocked
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// CM4 sleep request selection
    pub mod M4_GPC_SLEEP_SEL {
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

            /// 0b0: CM4 SLEEPDEEP is sent to GPC
            pub const DISABLE: u32 = 0b0;

            /// 0b1: CM4 SLEEPING is sent to GPC
            pub const NABLE: u32 = 0b1;
        }
    }

    /// Security error response enable
    pub mod SEC_ERR_RESP {
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

            /// 0b0: OKEY response
            pub const DISABLE: u32 = 0b0;

            /// 0b1: SLVError (default)
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Domain write protection
    pub mod DWP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR35 General Purpose Register
pub mod GPR35 {

    /// ADC1 doze mode
    pub mod ADC1_IPG_DOZE {
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

            /// 0b0: Not in doze mode
            pub const DISABLE: u32 = 0b0;

            /// 0b1: In doze mode
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// ADC1 stop request
    pub mod ADC1_STOP_REQ {
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

            /// 0b0: Stop request off
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Stop request on
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// ADC1 stop mode selection. This bitfield cannot change when ADC1_STOP_REQ is asserted.
    pub mod ADC1_IPG_STOP_MODE {
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

            /// 0b0: This module is functional in Stop Mode
            pub const FUNC: u32 = 0b0;

            /// 0b1: This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'.
            pub const NONFUNC: u32 = 0b1;
        }
    }

    /// ADC2 doze mode
    pub mod ADC2_IPG_DOZE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_DOZE::RW;
    }

    /// ADC2 stop request
    pub mod ADC2_STOP_REQ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_STOP_REQ::RW;
    }

    /// ADC2 stop mode selection. This bitfield cannot change when ADC2_STOP_REQ is asserted.
    pub mod ADC2_IPG_STOP_MODE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_STOP_MODE::RW;
    }

    /// CAN3 doze mode
    pub mod CAAM_IPG_DOZE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_DOZE::RW;
    }

    /// CAAM stop request
    pub mod CAAM_STOP_REQ {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_STOP_REQ::RW;
    }

    /// CAN1 doze mode
    pub mod CAN1_IPG_DOZE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_DOZE::RW;
    }

    /// CAN1 stop request
    pub mod CAN1_STOP_REQ {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_STOP_REQ::RW;
    }

    /// CAN2 doze mode
    pub mod CAN2_IPG_DOZE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_DOZE::RW;
    }

    /// CAN2 stop request
    pub mod CAN2_STOP_REQ {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_STOP_REQ::RW;
    }

    /// CAN3 doze mode
    pub mod CAN3_IPG_DOZE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_DOZE::RW;
    }

    /// CAN3 stop request
    pub mod CAN3_STOP_REQ {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_STOP_REQ::RW;
    }

    /// EDMA stop request
    pub mod EDMA_STOP_REQ {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_STOP_REQ::RW;
    }

    /// EDMA_LPSR stop request
    pub mod EDMA_LPSR_STOP_REQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_STOP_REQ::RW;
    }

    /// ENET doze mode
    pub mod ENET_IPG_DOZE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_DOZE::RW;
    }

    /// ENET stop request
    pub mod ENET_STOP_REQ {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_STOP_REQ::RW;
    }

    /// ENET1G doze mode
    pub mod ENET1G_IPG_DOZE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_DOZE::RW;
    }

    /// ENET1G stop request
    pub mod ENET1G_STOP_REQ {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_STOP_REQ::RW;
    }

    /// FLEXIO2 doze mode
    pub mod FLEXIO1_IPG_DOZE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_DOZE::RW;
    }

    /// FLEXIO2 doze mode
    pub mod FLEXIO2_IPG_DOZE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_DOZE::RW;
    }

    /// FLEXSPI1 doze mode
    pub mod FLEXSPI1_IPG_DOZE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_DOZE::RW;
    }

    /// FLEXSPI1 stop request
    pub mod FLEXSPI1_STOP_REQ {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_STOP_REQ::RW;
    }

    /// FLEXSPI2 doze mode
    pub mod FLEXSPI2_IPG_DOZE {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_IPG_DOZE::RW;
    }

    /// FLEXSPI2 stop request
    pub mod FLEXSPI2_STOP_REQ {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ADC1_STOP_REQ::RW;
    }

    /// Domain write protection
    pub mod DWP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR36 General Purpose Register
pub mod GPR36 {

    /// GPT1 doze mode
    pub mod GPT1_IPG_DOZE {
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

            /// 0b0: Not in doze mode
            pub const DISABLE: u32 = 0b0;

            /// 0b1: In doze mode
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// GPT2 doze mode
    pub mod GPT2_IPG_DOZE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPT1_IPG_DOZE::RW;
    }

    /// GPT3 doze mode
    pub mod GPT3_IPG_DOZE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPT1_IPG_DOZE::RW;
    }

    /// GPT4 doze mode
    pub mod GPT4_IPG_DOZE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPT1_IPG_DOZE::RW;
    }

    /// GPT5 doze mode
    pub mod GPT5_IPG_DOZE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPT1_IPG_DOZE::RW;
    }

    /// GPT6 doze mode
    pub mod GPT6_IPG_DOZE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPT1_IPG_DOZE::RW;
    }

    /// LPI2C1 doze mode
    pub mod LPI2C1_IPG_DOZE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPT1_IPG_DOZE::RW;
    }

    /// LPI2C1 stop request
    pub mod LPI2C1_STOP_REQ {
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

            /// 0b0: Stop request off
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Stop request on
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// LPI2C1 stop mode selection. This bitfield cannot change when LPI2C1_STOP_REQ is asserted.
    pub mod LPI2C1_IPG_STOP_MODE {
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

            /// 0b0: This module is functional in Stop Mode
            pub const FUNC: u32 = 0b0;

            /// 0b1: This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'.
            pub const NONFUNC: u32 = 0b1;
        }
    }

    /// LPI2C2 doze mode
    pub mod LPI2C2_IPG_DOZE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPT1_IPG_DOZE::RW;
    }

    /// LPI2C2 stop request
    pub mod LPI2C2_STOP_REQ {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_STOP_REQ::RW;
    }

    /// LPI2C2 stop mode selection. This bitfield cannot change when LPI2C2_STOP_REQ is asserted.
    pub mod LPI2C2_IPG_STOP_MODE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_IPG_STOP_MODE::RW;
    }

    /// LPI2C3 doze mode
    pub mod LPI2C3_IPG_DOZE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPT1_IPG_DOZE::RW;
    }

    /// LPI2C3 stop request
    pub mod LPI2C3_STOP_REQ {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_STOP_REQ::RW;
    }

    /// LPI2C3 stop mode selection. This bitfield cannot change when LPI2C3_STOP_REQ is asserted.
    pub mod LPI2C3_IPG_STOP_MODE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_IPG_STOP_MODE::RW;
    }

    /// LPI2C4 doze mode
    pub mod LPI2C4_IPG_DOZE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPT1_IPG_DOZE::RW;
    }

    /// LPI2C4 stop request
    pub mod LPI2C4_STOP_REQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_STOP_REQ::RW;
    }

    /// LPI2C4 stop mode selection. This bitfield cannot change when LPI2C4_STOP_REQ is asserted.
    pub mod LPI2C4_IPG_STOP_MODE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_IPG_STOP_MODE::RW;
    }

    /// LPI2C5 doze mode
    pub mod LPI2C5_IPG_DOZE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPT1_IPG_DOZE::RW;
    }

    /// LPI2C5 stop request
    pub mod LPI2C5_STOP_REQ {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_STOP_REQ::RW;
    }

    /// LPI2C5 stop mode selection. This bitfield cannot change when LPI2C5_STOP_REQ is asserted.
    pub mod LPI2C5_IPG_STOP_MODE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_IPG_STOP_MODE::RW;
    }

    /// LPI2C6 doze mode
    pub mod LPI2C6_IPG_DOZE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPT1_IPG_DOZE::RW;
    }

    /// LPI2C6 stop request
    pub mod LPI2C6_STOP_REQ {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_STOP_REQ::RW;
    }

    /// LPI2C6 stop mode selection. This bitfield cannot change when LPI2C6_STOP_REQ is asserted.
    pub mod LPI2C6_IPG_STOP_MODE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_IPG_STOP_MODE::RW;
    }

    /// LPSPI1 doze mode
    pub mod LPSPI1_IPG_DOZE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GPT1_IPG_DOZE::RW;
    }

    /// LPSPI1 stop request
    pub mod LPSPI1_STOP_REQ {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_STOP_REQ::RW;
    }

    /// LPSPI1 stop mode selection. This bitfield cannot change when LPSPI1_STOP_REQ is asserted.
    pub mod LPSPI1_IPG_STOP_MODE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPI2C1_IPG_STOP_MODE::RW;
    }

    /// Domain write protection
    pub mod DWP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR37 General Purpose Register
pub mod GPR37 {

    /// LPSPI2 doze mode
    pub mod LPSPI2_IPG_DOZE {
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

            /// 0b0: Not in doze mode
            pub const DISABLE: u32 = 0b0;

            /// 0b1: In doze mode
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// LPSPI2 stop request
    pub mod LPSPI2_STOP_REQ {
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

            /// 0b0: Stop request off
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Stop request on
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// LPSPI2 stop mode selection. This bitfield cannot change when LPSPI2_STOP_REQ is asserted.
    pub mod LPSPI2_IPG_STOP_MODE {
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

            /// 0b0: This module is functional in Stop Mode
            pub const FUNC: u32 = 0b0;

            /// 0b1: This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'.
            pub const NONFUNC: u32 = 0b1;
        }
    }

    /// LPSPI3 doze mode
    pub mod LPSPI3_IPG_DOZE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_DOZE::RW;
    }

    /// LPSPI3 stop request
    pub mod LPSPI3_STOP_REQ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_STOP_REQ::RW;
    }

    /// LPSPI3 stop mode selection. This bitfield cannot change when LPSPI3_STOP_REQ is asserted.
    pub mod LPSPI3_IPG_STOP_MODE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPSPI4 doze mode
    pub mod LPSPI4_IPG_DOZE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_DOZE::RW;
    }

    /// LPSPI4 stop request
    pub mod LPSPI4_STOP_REQ {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_STOP_REQ::RW;
    }

    /// LPSPI4 stop mode selection. This bitfield cannot change when LPSPI4_STOP_REQ is asserted.
    pub mod LPSPI4_IPG_STOP_MODE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPSPI5 doze mode
    pub mod LPSPI5_IPG_DOZE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_DOZE::RW;
    }

    /// LPSPI5 stop request
    pub mod LPSPI5_STOP_REQ {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_STOP_REQ::RW;
    }

    /// LPSPI5 stop mode selection. This bitfield cannot change when LPSPI5_STOP_REQ is asserted.
    pub mod LPSPI5_IPG_STOP_MODE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPSPI6 doze mode
    pub mod LPSPI6_IPG_DOZE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_DOZE::RW;
    }

    /// LPSPI6 stop request
    pub mod LPSPI6_STOP_REQ {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_STOP_REQ::RW;
    }

    /// LPSPI6 stop mode selection. This bitfield cannot change when LPSPI6_STOP_REQ is asserted.
    pub mod LPSPI6_IPG_STOP_MODE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPUART1 doze mode
    pub mod LPUART1_IPG_DOZE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_DOZE::RW;
    }

    /// LPUART1 stop request
    pub mod LPUART1_STOP_REQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_STOP_REQ::RW;
    }

    /// LPUART1 stop mode selection. This bitfield cannot change when LPUART1_STOP_REQ is asserted.
    pub mod LPUART1_IPG_STOP_MODE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPUART2 doze mode
    pub mod LPUART2_IPG_DOZE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_DOZE::RW;
    }

    /// LPUART2 stop request
    pub mod LPUART2_STOP_REQ {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_STOP_REQ::RW;
    }

    /// LPUART2 stop mode selection. This bitfield cannot change when LPUART2_STOP_REQ is asserted.
    pub mod LPUART2_IPG_STOP_MODE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPUART3 doze mode
    pub mod LPUART3_IPG_DOZE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_DOZE::RW;
    }

    /// LPUART3 stop request
    pub mod LPUART3_STOP_REQ {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_STOP_REQ::RW;
    }

    /// LPUART3 stop mode selection. This bitfield cannot change when LPUART3_STOP_REQ is asserted.
    pub mod LPUART3_IPG_STOP_MODE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// LPUART4 doze mode
    pub mod LPUART4_IPG_DOZE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_DOZE::RW;
    }

    /// LPUART4 stop request
    pub mod LPUART4_STOP_REQ {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_STOP_REQ::RW;
    }

    /// LPUART4 stop mode selection. This bitfield cannot change when LPUART4_STOP_REQ is asserted.
    pub mod LPUART4_IPG_STOP_MODE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSPI2_IPG_STOP_MODE::RW;
    }

    /// Domain write protection
    pub mod DWP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR38 General Purpose Register
pub mod GPR38 {

    /// LPUART5 doze mode
    pub mod LPUART5_IPG_DOZE {
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

            /// 0b0: Not in doze mode
            pub const DISABLE: u32 = 0b0;

            /// 0b1: In doze mode
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// LPUART5 stop request
    pub mod LPUART5_STOP_REQ {
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

            /// 0b0: Stop request off
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Stop request on
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// LPUART5 stop mode selection. This bitfield cannot change when LPUART5_STOP_REQ is asserted.
    pub mod LPUART5_IPG_STOP_MODE {
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

            /// 0b0: This module is functional in Stop Mode
            pub const FUNC: u32 = 0b0;

            /// 0b1: This module is not functional in Stop Mode and the corresponding x_STOP_REQ field is set to '1'.
            pub const NONFUNC: u32 = 0b1;
        }
    }

    /// LPUART6 doze mode
    pub mod LPUART6_IPG_DOZE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_DOZE::RW;
    }

    /// LPUART6 stop request
    pub mod LPUART6_STOP_REQ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_STOP_REQ::RW;
    }

    /// LPUART6 stop mode selection. This bitfield cannot change when LPUART6_STOP_REQ is asserted.
    pub mod LPUART6_IPG_STOP_MODE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// LPUART7 doze mode
    pub mod LPUART7_IPG_DOZE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_DOZE::RW;
    }

    /// LPUART7 stop request
    pub mod LPUART7_STOP_REQ {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_STOP_REQ::RW;
    }

    /// LPUART7 stop mode selection. This bitfield cannot change when LPUART7_STOP_REQ is asserted.
    pub mod LPUART7_IPG_STOP_MODE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// LPUART8 doze mode
    pub mod LPUART8_IPG_DOZE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_DOZE::RW;
    }

    /// LPUART8 stop request
    pub mod LPUART8_STOP_REQ {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_STOP_REQ::RW;
    }

    /// LPUART8 stop mode selection. This bitfield cannot change when LPUART8_STOP_REQ is asserted.
    pub mod LPUART8_IPG_STOP_MODE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// LPUART9 doze mode
    pub mod LPUART9_IPG_DOZE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_DOZE::RW;
    }

    /// LPUART9 stop request
    pub mod LPUART9_STOP_REQ {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_STOP_REQ::RW;
    }

    /// LPUART9 stop mode selection. This bitfield cannot change when LPUART9_STOP_REQ is asserted.
    pub mod LPUART9_IPG_STOP_MODE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// LPUART10 doze mode
    pub mod LPUART10_IPG_DOZE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_DOZE::RW;
    }

    /// LPUART10 stop request
    pub mod LPUART10_STOP_REQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_STOP_REQ::RW;
    }

    /// LPUART10 stop mode selection. This bitfield cannot change when LPUART10_STOP_REQ is asserted.
    pub mod LPUART10_IPG_STOP_MODE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// LPUART11 doze mode
    pub mod LPUART11_IPG_DOZE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_DOZE::RW;
    }

    /// LPUART11 stop request
    pub mod LPUART11_STOP_REQ {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_STOP_REQ::RW;
    }

    /// LPUART11 stop mode selection. This bitfield cannot change when LPUART11_STOP_REQ is asserted.
    pub mod LPUART11_IPG_STOP_MODE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// LPUART12 doze mode
    pub mod LPUART12_IPG_DOZE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_DOZE::RW;
    }

    /// LPUART12 stop request
    pub mod LPUART12_STOP_REQ {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_STOP_REQ::RW;
    }

    /// LPUART12 stop mode selection. This bitfield cannot change when LPUART12_STOP_REQ is asserted.
    pub mod LPUART12_IPG_STOP_MODE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// MIC doze mode
    pub mod MIC_IPG_DOZE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_DOZE::RW;
    }

    /// MIC stop request
    pub mod MIC_STOP_REQ {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_STOP_REQ::RW;
    }

    /// MIC stop mode selection. This bitfield cannot change when MIC_STOP_REQ is asserted.
    pub mod MIC_IPG_STOP_MODE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPUART5_IPG_STOP_MODE::RW;
    }

    /// Domain write protection
    pub mod DWP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR39 General Purpose Register
pub mod GPR39 {

    /// PIT1 stop request
    pub mod PIT1_STOP_REQ {
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

            /// 0b0: Stop request off
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Stop request on
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// PIT2 stop request
    pub mod PIT2_STOP_REQ {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PIT1_STOP_REQ::RW;
    }

    /// SEMC stop request
    pub mod SEMC_STOP_REQ {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PIT1_STOP_REQ::RW;
    }

    /// SIM1 doze mode
    pub mod SIM1_IPG_DOZE {
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

            /// 0b0: Not in doze mode
            pub const DISABLE: u32 = 0b0;

            /// 0b1: In doze mode
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// SIM2 doze mode
    pub mod SIM2_IPG_DOZE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SIM1_IPG_DOZE::RW;
    }

    /// SNVS_HP doze mode
    pub mod SNVS_HP_IPG_DOZE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SIM1_IPG_DOZE::RW;
    }

    /// SNVS_HP stop request
    pub mod SNVS_HP_STOP_REQ {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PIT1_STOP_REQ::RW;
    }

    /// WDOG1 doze mode
    pub mod WDOG1_IPG_DOZE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SIM1_IPG_DOZE::RW;
    }

    /// WDOG2 doze mode
    pub mod WDOG2_IPG_DOZE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SIM1_IPG_DOZE::RW;
    }

    /// SAI1 stop request
    pub mod SAI1_STOP_REQ {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PIT1_STOP_REQ::RW;
    }

    /// SAI2 stop request
    pub mod SAI2_STOP_REQ {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PIT1_STOP_REQ::RW;
    }

    /// SAI3 stop request
    pub mod SAI3_STOP_REQ {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PIT1_STOP_REQ::RW;
    }

    /// SAI4 stop request
    pub mod SAI4_STOP_REQ {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PIT1_STOP_REQ::RW;
    }

    /// FLEXIO1 bus clock domain stop request
    pub mod FLEXIO1_STOP_REQ_BUS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PIT1_STOP_REQ::RW;
    }

    /// FLEXIO1 peripheral clock domain stop request
    pub mod FLEXIO1_STOP_REQ_PER {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PIT1_STOP_REQ::RW;
    }

    /// FLEXIO2 bus clock domain stop request
    pub mod FLEXIO2_STOP_REQ_BUS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PIT1_STOP_REQ::RW;
    }

    /// FLEXIO2 peripheral clock domain stop request
    pub mod FLEXIO2_STOP_REQ_PER {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PIT1_STOP_REQ::RW;
    }

    /// Domain write protection
    pub mod DWP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Both cores are allowed
            pub const forbid_none: u32 = 0b00;

            /// 0b01: CM7 is forbidden
            pub const forbid_CM7: u32 = 0b01;

            /// 0b10: CM4 is forbidden
            pub const forbid_CM4: u32 = 0b10;

            /// 0b11: Both cores are forbidden
            pub const forbid_both: u32 = 0b11;
        }
    }

    /// Domain write protection lock
    pub mod DWP_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Neither of DWP bits is locked
            pub const lock_none: u32 = 0b00;

            /// 0b01: The lower DWP bit is locked
            pub const lock_low: u32 = 0b01;

            /// 0b10: The higher DWP bit is locked
            pub const lock_high: u32 = 0b10;

            /// 0b11: Both DWP bits are locked
            pub const lock_both: u32 = 0b11;
        }
    }
}

/// GPR40 General Purpose Register
pub mod GPR40 {

    /// ADC1 stop acknowledge
    pub mod ADC1_STOP_ACK {
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

    /// ADC2 stop acknowledge
    pub mod ADC2_STOP_ACK {
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

    /// CAAM stop acknowledge
    pub mod CAAM_STOP_ACK {
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

    /// CAN1 stop acknowledge
    pub mod CAN1_STOP_ACK {
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

    /// CAN2 stop acknowledge
    pub mod CAN2_STOP_ACK {
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

    /// CAN3 stop acknowledge
    pub mod CAN3_STOP_ACK {
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

    /// EDMA stop acknowledge
    pub mod EDMA_STOP_ACK {
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

    /// EDMA_LPSR stop acknowledge
    pub mod EDMA_LPSR_STOP_ACK {
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

    /// ENET stop acknowledge
    pub mod ENET_STOP_ACK {
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

    /// ENET1G stop acknowledge
    pub mod ENET1G_STOP_ACK {
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

    /// FLEXSPI1 stop acknowledge
    pub mod FLEXSPI1_STOP_ACK {
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

    /// FLEXSPI2 stop acknowledge
    pub mod FLEXSPI2_STOP_ACK {
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

    /// LPI2C1 stop acknowledge
    pub mod LPI2C1_STOP_ACK {
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

    /// LPI2C2 stop acknowledge
    pub mod LPI2C2_STOP_ACK {
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

    /// LPI2C3 stop acknowledge
    pub mod LPI2C3_STOP_ACK {
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

    /// LPI2C4 stop acknowledge
    pub mod LPI2C4_STOP_ACK {
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

    /// LPI2C5 stop acknowledge
    pub mod LPI2C5_STOP_ACK {
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

    /// LPI2C6 stop acknowledge
    pub mod LPI2C6_STOP_ACK {
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

    /// LPSPI1 stop acknowledge
    pub mod LPSPI1_STOP_ACK {
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

    /// LPSPI2 stop acknowledge
    pub mod LPSPI2_STOP_ACK {
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

    /// LPSPI3 stop acknowledge
    pub mod LPSPI3_STOP_ACK {
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

    /// LPSPI4 stop acknowledge
    pub mod LPSPI4_STOP_ACK {
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

    /// LPSPI5 stop acknowledge
    pub mod LPSPI5_STOP_ACK {
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

    /// LPSPI6 stop acknowledge
    pub mod LPSPI6_STOP_ACK {
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

    /// LPUART1 stop acknowledge
    pub mod LPUART1_STOP_ACK {
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

    /// LPUART2 stop acknowledge
    pub mod LPUART2_STOP_ACK {
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

    /// LPUART3 stop acknowledge
    pub mod LPUART3_STOP_ACK {
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

    /// LPUART4 stop acknowledge
    pub mod LPUART4_STOP_ACK {
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

    /// LPUART5 stop acknowledge
    pub mod LPUART5_STOP_ACK {
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

    /// LPUART6 stop acknowledge
    pub mod LPUART6_STOP_ACK {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPUART7 stop acknowledge
    pub mod LPUART7_STOP_ACK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPUART8 stop acknowledge
    pub mod LPUART8_STOP_ACK {
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

/// GPR41 General Purpose Register
pub mod GPR41 {

    /// LPUART9 stop acknowledge
    pub mod LPUART9_STOP_ACK {
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

    /// LPUART10 stop acknowledge
    pub mod LPUART10_STOP_ACK {
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

    /// LPUART11 stop acknowledge
    pub mod LPUART11_STOP_ACK {
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

    /// LPUART12 stop acknowledge
    pub mod LPUART12_STOP_ACK {
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

    /// MIC stop acknowledge
    pub mod MIC_STOP_ACK {
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

    /// PIT1 stop acknowledge
    pub mod PIT1_STOP_ACK {
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

    /// PIT2 stop acknowledge
    pub mod PIT2_STOP_ACK {
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

    /// SEMC stop acknowledge
    pub mod SEMC_STOP_ACK {
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

    /// SNVS_HP stop acknowledge
    pub mod SNVS_HP_STOP_ACK {
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

    /// SAI1 stop acknowledge
    pub mod SAI1_STOP_ACK {
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

    /// SAI2 stop acknowledge
    pub mod SAI2_STOP_ACK {
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

    /// SAI3 stop acknowledge
    pub mod SAI3_STOP_ACK {
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

    /// SAI4 stop acknowledge
    pub mod SAI4_STOP_ACK {
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

    /// FLEXIO1 stop acknowledge of bus clock domain
    pub mod FLEXIO1_STOP_ACK_BUS {
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

    /// FLEXIO1 stop acknowledge of peripheral clock domain
    pub mod FLEXIO1_STOP_ACK_PER {
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

    /// FLEXIO2 stop acknowledge of bus clock domain
    pub mod FLEXIO2_STOP_ACK_BUS {
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

    /// FLEXIO2 stop acknowledge of peripheral clock domain
    pub mod FLEXIO2_STOP_ACK_PER {
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

    /// ROM read lock status bit
    pub mod ROM_READ_LOCKED {
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
    /// GPR0 General Purpose Register
    pub GPR0: RWRegister<u32>,

    /// GPR1 General Purpose Register
    pub GPR1: RWRegister<u32>,

    /// GPR2 General Purpose Register
    pub GPR2: RWRegister<u32>,

    /// GPR3 General Purpose Register
    pub GPR3: RWRegister<u32>,

    /// GPR4 General Purpose Register
    pub GPR4: RWRegister<u32>,

    /// GPR5 General Purpose Register
    pub GPR5: RWRegister<u32>,

    /// GPR6 General Purpose Register
    pub GPR6: RWRegister<u32>,

    /// GPR7 General Purpose Register
    pub GPR7: RWRegister<u32>,

    /// GPR8 General Purpose Register
    pub GPR8: RWRegister<u32>,

    /// GPR9 General Purpose Register
    pub GPR9: RWRegister<u32>,

    /// GPR10 General Purpose Register
    pub GPR10: RWRegister<u32>,

    /// GPR11 General Purpose Register
    pub GPR11: RWRegister<u32>,

    /// GPR12 General Purpose Register
    pub GPR12: RWRegister<u32>,

    /// GPR13 General Purpose Register
    pub GPR13: RWRegister<u32>,

    /// GPR14 General Purpose Register
    pub GPR14: RWRegister<u32>,

    /// GPR15 General Purpose Register
    pub GPR15: RWRegister<u32>,

    /// GPR16 General Purpose Register
    pub GPR16: RWRegister<u32>,

    /// GPR17 General Purpose Register
    pub GPR17: RWRegister<u32>,

    /// GPR18 General Purpose Register
    pub GPR18: RWRegister<u32>,

    /// GPR19 General Purpose Register
    pub GPR19: RWRegister<u32>,

    /// GPR20 General Purpose Register
    pub GPR20: RWRegister<u32>,

    /// GPR21 General Purpose Register
    pub GPR21: RWRegister<u32>,

    /// GPR22 General Purpose Register
    pub GPR22: RWRegister<u32>,

    /// GPR23 General Purpose Register
    pub GPR23: RWRegister<u32>,

    /// GPR24 General Purpose Register
    pub GPR24: RWRegister<u32>,

    /// GPR25 General Purpose Register
    pub GPR25: RWRegister<u32>,

    /// GPR26 General Purpose Register
    pub GPR26: RWRegister<u32>,

    _reserved1: [u32; 6],

    /// GPR33 General Purpose Register
    pub GPR33: RWRegister<u32>,

    /// GPR34 General Purpose Register
    pub GPR34: RWRegister<u32>,

    /// GPR35 General Purpose Register
    pub GPR35: RWRegister<u32>,

    /// GPR36 General Purpose Register
    pub GPR36: RWRegister<u32>,

    /// GPR37 General Purpose Register
    pub GPR37: RWRegister<u32>,

    /// GPR38 General Purpose Register
    pub GPR38: RWRegister<u32>,

    /// GPR39 General Purpose Register
    pub GPR39: RWRegister<u32>,

    /// GPR40 General Purpose Register
    pub GPR40: RORegister<u32>,

    /// GPR41 General Purpose Register
    pub GPR41: RORegister<u32>,
}
pub struct ResetValues {
    pub GPR0: u32,
    pub GPR1: u32,
    pub GPR2: u32,
    pub GPR3: u32,
    pub GPR4: u32,
    pub GPR5: u32,
    pub GPR6: u32,
    pub GPR7: u32,
    pub GPR8: u32,
    pub GPR9: u32,
    pub GPR10: u32,
    pub GPR11: u32,
    pub GPR12: u32,
    pub GPR13: u32,
    pub GPR14: u32,
    pub GPR15: u32,
    pub GPR16: u32,
    pub GPR17: u32,
    pub GPR18: u32,
    pub GPR19: u32,
    pub GPR20: u32,
    pub GPR21: u32,
    pub GPR22: u32,
    pub GPR23: u32,
    pub GPR24: u32,
    pub GPR25: u32,
    pub GPR26: u32,
    pub GPR33: u32,
    pub GPR34: u32,
    pub GPR35: u32,
    pub GPR36: u32,
    pub GPR37: u32,
    pub GPR38: u32,
    pub GPR39: u32,
    pub GPR40: u32,
    pub GPR41: u32,
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
