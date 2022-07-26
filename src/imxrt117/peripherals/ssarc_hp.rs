#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SRAM Registers
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// Description Address Register
pub mod SRAM0__0 {

    /// Address field
    pub mod ADDR {
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

/// Description Data Register
pub mod SRAM1__0 {

    /// Data field
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

/// Description Control Register
pub mod SRAM2__0 {

    /// Type field
    pub mod TYPE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: SR
            pub const SR: u32 = 0b000;

            /// 0b001: WO
            pub const WO: u32 = 0b001;

            /// 0b010: RMW_OR
            pub const RMW_OR: u32 = 0b010;

            /// 0b011: RMW_AND
            pub const RMW_AND: u32 = 0b011;

            /// 0b100: DELAY
            pub const DELAY: u32 = 0b100;

            /// 0b101: POLLING_0
            pub const POLLING_0: u32 = 0b101;

            /// 0b110: POLLING_1
            pub const POLLING_1: u32 = 0b110;
        }
    }

    /// Save Enable
    pub mod SV_EN {
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

            /// 0b0: Do not use this descriptor in the save operation
            pub const SV_EN_0: u32 = 0b0;

            /// 0b1: Use this descriptor in the save operation
            pub const SV_EN_1: u32 = 0b1;
        }
    }

    /// Restore Enable
    pub mod RT_EN {
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

            /// 0b0: Do not use this descriptor for the restore operation
            pub const RT_EN_0: u32 = 0b0;

            /// 0b1: Use this descriptor for the restore operation
            pub const RT_EN_1: u32 = 0b1;
        }
    }

    /// Size field
    pub mod SIZE {
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

            /// 0b00: 8-bit
            pub const size_0: u32 = 0b00;

            /// 0b01: 16-bit
            pub const size_1: u32 = 0b01;

            /// 0b10: 32-bit
            pub const size_2: u32 = 0b10;
        }
    }
}

/// Description Address Register
pub mod SRAM0__1 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__2 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__2 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__2 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__3 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__3 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__3 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__4 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__4 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__4 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__5 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__5 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__5 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__6 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__6 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__6 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__7 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__7 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__7 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__8 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__8 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__8 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__9 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__9 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__9 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__10 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__10 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__10 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__11 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__11 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__11 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__12 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__12 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__12 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__13 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__13 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__13 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__14 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__14 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__14 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__15 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__15 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__15 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__16 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__16 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__16 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__17 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__17 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__17 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__18 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__18 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__18 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__19 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__19 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__19 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__20 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__20 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__20 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__21 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__21 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__21 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__22 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__22 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__22 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__23 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__23 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__23 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__24 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__24 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__24 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__25 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__25 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__25 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__26 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__26 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__26 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__27 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__27 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__27 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__28 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__28 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__28 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__29 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__29 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__29 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__30 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__30 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__30 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__31 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__31 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__31 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__32 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__32 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__32 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__33 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__33 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__33 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__34 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__34 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__34 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__35 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__35 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__35 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__36 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__36 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__36 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__37 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__37 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__37 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__38 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__38 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__38 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__39 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__39 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__39 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__40 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__40 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__40 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__41 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__41 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__41 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__42 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__42 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__42 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__43 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__43 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__43 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__44 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__44 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__44 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__45 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__45 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__45 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__46 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__46 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__46 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__47 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__47 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__47 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__48 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__48 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__48 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__49 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__49 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__49 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__50 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__50 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__50 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__51 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__51 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__51 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__52 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__52 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__52 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__53 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__53 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__53 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__54 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__54 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__54 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__55 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__55 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__55 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__56 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__56 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__56 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__57 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__57 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__57 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__58 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__58 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__58 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__59 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__59 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__59 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__60 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__60 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__60 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__61 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__61 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__61 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__62 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__62 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__62 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__63 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__63 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__63 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__64 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__64 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__64 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__65 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__65 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__65 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__66 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__66 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__66 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__67 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__67 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__67 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__68 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__68 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__68 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__69 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__69 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__69 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__70 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__70 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__70 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__71 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__71 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__71 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__72 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__72 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__72 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__73 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__73 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__73 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__74 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__74 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__74 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__75 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__75 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__75 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__76 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__76 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__76 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__77 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__77 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__77 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__78 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__78 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__78 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__79 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__79 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__79 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__80 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__80 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__80 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__81 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__81 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__81 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__82 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__82 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__82 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__83 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__83 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__83 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__84 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__84 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__84 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__85 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__85 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__85 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__86 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__86 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__86 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__87 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__87 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__87 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__88 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__88 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__88 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__89 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__89 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__89 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__90 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__90 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__90 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__91 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__91 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__91 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__92 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__92 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__92 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__93 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__93 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__93 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__94 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__94 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__94 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__95 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__95 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__95 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__96 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__96 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__96 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__97 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__97 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__97 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__98 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__98 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__98 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__99 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__99 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__99 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__100 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__100 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__100 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__101 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__101 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__101 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__102 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__102 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__102 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__103 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__103 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__103 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__104 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__104 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__104 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__105 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__105 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__105 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__106 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__106 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__106 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__107 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__107 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__107 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__108 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__108 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__108 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__109 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__109 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__109 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__110 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__110 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__110 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__111 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__111 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__111 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__112 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__112 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__112 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__113 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__113 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__113 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__114 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__114 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__114 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__115 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__115 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__115 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__116 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__116 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__116 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__117 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__117 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__117 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__118 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__118 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__118 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__119 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__119 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__119 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__120 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__120 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__120 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__121 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__121 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__121 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__122 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__122 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__122 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__123 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__123 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__123 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__124 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__124 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__124 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__125 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__125 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__125 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__126 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__126 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__126 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__127 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__127 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__127 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__128 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__128 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__128 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__129 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__129 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__129 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__130 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__130 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__130 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__131 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__131 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__131 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__132 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__132 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__132 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__133 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__133 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__133 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__134 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__134 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__134 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__135 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__135 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__135 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__136 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__136 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__136 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__137 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__137 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__137 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__138 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__138 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__138 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__139 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__139 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__139 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__140 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__140 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__140 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__141 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__141 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__141 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__142 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__142 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__142 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__143 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__143 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__143 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__144 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__144 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__144 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__145 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__145 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__145 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__146 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__146 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__146 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__147 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__147 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__147 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__148 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__148 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__148 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__149 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__149 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__149 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__150 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__150 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__150 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__151 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__151 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__151 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__152 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__152 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__152 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__153 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__153 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__153 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__154 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__154 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__154 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__155 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__155 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__155 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__156 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__156 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__156 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__157 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__157 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__157 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__158 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__158 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__158 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__159 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__159 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__159 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__160 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__160 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__160 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__161 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__161 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__161 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__162 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__162 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__162 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__163 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__163 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__163 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__164 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__164 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__164 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__165 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__165 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__165 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__166 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__166 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__166 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__167 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__167 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__167 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__168 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__168 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__168 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__169 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__169 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__169 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__170 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__170 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__170 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__171 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__171 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__171 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__172 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__172 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__172 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__173 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__173 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__173 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__174 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__174 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__174 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__175 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__175 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__175 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__176 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__176 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__176 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__177 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__177 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__177 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__178 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__178 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__178 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__179 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__179 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__179 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__180 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__180 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__180 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__181 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__181 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__181 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__182 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__182 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__182 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__183 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__183 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__183 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__184 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__184 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__184 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__185 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__185 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__185 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__186 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__186 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__186 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__187 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__187 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__187 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__188 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__188 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__188 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__189 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__189 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__189 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__190 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__190 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__190 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__191 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__191 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__191 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__192 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__192 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__192 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__193 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__193 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__193 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__194 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__194 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__194 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__195 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__195 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__195 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__196 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__196 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__196 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__197 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__197 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__197 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__198 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__198 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__198 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__199 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__199 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__199 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__200 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__200 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__200 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__201 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__201 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__201 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__202 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__202 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__202 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__203 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__203 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__203 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__204 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__204 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__204 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__205 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__205 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__205 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__206 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__206 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__206 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__207 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__207 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__207 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__208 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__208 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__208 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__209 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__209 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__209 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__210 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__210 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__210 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__211 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__211 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__211 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__212 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__212 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__212 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__213 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__213 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__213 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__214 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__214 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__214 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__215 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__215 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__215 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__216 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__216 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__216 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__217 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__217 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__217 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__218 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__218 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__218 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__219 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__219 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__219 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__220 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__220 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__220 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__221 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__221 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__221 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__222 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__222 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__222 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__223 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__223 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__223 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__224 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__224 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__224 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__225 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__225 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__225 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__226 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__226 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__226 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__227 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__227 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__227 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__228 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__228 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__228 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__229 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__229 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__229 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__230 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__230 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__230 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__231 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__231 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__231 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__232 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__232 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__232 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__233 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__233 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__233 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__234 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__234 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__234 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__235 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__235 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__235 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__236 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__236 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__236 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__237 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__237 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__237 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__238 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__238 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__238 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__239 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__239 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__239 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__240 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__240 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__240 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__241 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__241 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__241 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__242 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__242 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__242 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__243 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__243 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__243 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__244 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__244 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__244 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__245 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__245 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__245 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__246 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__246 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__246 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__247 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__247 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__247 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__248 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__248 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__248 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__249 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__249 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__249 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__250 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__250 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__250 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__251 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__251 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__251 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__252 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__252 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__252 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__253 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__253 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__253 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__254 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__254 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__254 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__255 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__255 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__255 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__256 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__256 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__256 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__257 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__257 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__257 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__258 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__258 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__258 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__259 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__259 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__259 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__260 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__260 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__260 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__261 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__261 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__261 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__262 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__262 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__262 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__263 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__263 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__263 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__264 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__264 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__264 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__265 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__265 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__265 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__266 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__266 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__266 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__267 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__267 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__267 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__268 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__268 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__268 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__269 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__269 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__269 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__270 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__270 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__270 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__271 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__271 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__271 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__272 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__272 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__272 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__273 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__273 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__273 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__274 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__274 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__274 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__275 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__275 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__275 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__276 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__276 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__276 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__277 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__277 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__277 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__278 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__278 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__278 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__279 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__279 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__279 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__280 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__280 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__280 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__281 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__281 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__281 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__282 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__282 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__282 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__283 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__283 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__283 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__284 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__284 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__284 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__285 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__285 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__285 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__286 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__286 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__286 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__287 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__287 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__287 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__288 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__288 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__288 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__289 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__289 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__289 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__290 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__290 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__290 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__291 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__291 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__291 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__292 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__292 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__292 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__293 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__293 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__293 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__294 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__294 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__294 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__295 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__295 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__295 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__296 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__296 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__296 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__297 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__297 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__297 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__298 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__298 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__298 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__299 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__299 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__299 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__300 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__300 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__300 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__301 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__301 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__301 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__302 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__302 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__302 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__303 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__303 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__303 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__304 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__304 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__304 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__305 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__305 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__305 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__306 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__306 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__306 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__307 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__307 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__307 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__308 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__308 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__308 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__309 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__309 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__309 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__310 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__310 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__310 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__311 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__311 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__311 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__312 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__312 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__312 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__313 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__313 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__313 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__314 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__314 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__314 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__315 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__315 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__315 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__316 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__316 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__316 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__317 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__317 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__317 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__318 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__318 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__318 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__319 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__319 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__319 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__320 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__320 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__320 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__321 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__321 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__321 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__322 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__322 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__322 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__323 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__323 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__323 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__324 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__324 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__324 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__325 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__325 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__325 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__326 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__326 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__326 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__327 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__327 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__327 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__328 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__328 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__328 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__329 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__329 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__329 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__330 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__330 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__330 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__331 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__331 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__331 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__332 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__332 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__332 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__333 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__333 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__333 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__334 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__334 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__334 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__335 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__335 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__335 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__336 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__336 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__336 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__337 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__337 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__337 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__338 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__338 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__338 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__339 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__339 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__339 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__340 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__340 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__340 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__341 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__341 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__341 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__342 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__342 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__342 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__343 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__343 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__343 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__344 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__344 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__344 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__345 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__345 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__345 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__346 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__346 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__346 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__347 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__347 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__347 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__348 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__348 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__348 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__349 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__349 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__349 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__350 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__350 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__350 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__351 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__351 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__351 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__352 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__352 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__352 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__353 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__353 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__353 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__354 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__354 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__354 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__355 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__355 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__355 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__356 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__356 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__356 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__357 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__357 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__357 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__358 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__358 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__358 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__359 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__359 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__359 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__360 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__360 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__360 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__361 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__361 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__361 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__362 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__362 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__362 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__363 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__363 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__363 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__364 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__364 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__364 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__365 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__365 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__365 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__366 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__366 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__366 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__367 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__367 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__367 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__368 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__368 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__368 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__369 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__369 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__369 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__370 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__370 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__370 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__371 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__371 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__371 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__372 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__372 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__372 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__373 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__373 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__373 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__374 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__374 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__374 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__375 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__375 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__375 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__376 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__376 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__376 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__377 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__377 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__377 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__378 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__378 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__378 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__379 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__379 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__379 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__380 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__380 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__380 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__381 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__381 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__381 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__382 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__382 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__382 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__383 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__383 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__383 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__384 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__384 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__384 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__385 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__385 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__385 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__386 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__386 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__386 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__387 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__387 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__387 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__388 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__388 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__388 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__389 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__389 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__389 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__390 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__390 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__390 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__391 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__391 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__391 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__392 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__392 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__392 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__393 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__393 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__393 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__394 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__394 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__394 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__395 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__395 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__395 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__396 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__396 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__396 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__397 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__397 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__397 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__398 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__398 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__398 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__399 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__399 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__399 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__400 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__400 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__400 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__401 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__401 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__401 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__402 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__402 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__402 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__403 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__403 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__403 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__404 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__404 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__404 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__405 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__405 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__405 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__406 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__406 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__406 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__407 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__407 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__407 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__408 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__408 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__408 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__409 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__409 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__409 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__410 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__410 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__410 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__411 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__411 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__411 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__412 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__412 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__412 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__413 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__413 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__413 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__414 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__414 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__414 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__415 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__415 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__415 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__416 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__416 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__416 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__417 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__417 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__417 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__418 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__418 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__418 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__419 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__419 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__419 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__420 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__420 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__420 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__421 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__421 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__421 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__422 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__422 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__422 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__423 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__423 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__423 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__424 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__424 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__424 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__425 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__425 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__425 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__426 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__426 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__426 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__427 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__427 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__427 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__428 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__428 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__428 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__429 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__429 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__429 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__430 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__430 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__430 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__431 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__431 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__431 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__432 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__432 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__432 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__433 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__433 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__433 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__434 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__434 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__434 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__435 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__435 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__435 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__436 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__436 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__436 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__437 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__437 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__437 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__438 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__438 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__438 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__439 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__439 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__439 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__440 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__440 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__440 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__441 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__441 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__441 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__442 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__442 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__442 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__443 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__443 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__443 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__444 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__444 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__444 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__445 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__445 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__445 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__446 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__446 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__446 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__447 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__447 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__447 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__448 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__448 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__448 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__449 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__449 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__449 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__450 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__450 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__450 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__451 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__451 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__451 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__452 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__452 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__452 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__453 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__453 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__453 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__454 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__454 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__454 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__455 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__455 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__455 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__456 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__456 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__456 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__457 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__457 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__457 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__458 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__458 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__458 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__459 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__459 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__459 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__460 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__460 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__460 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__461 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__461 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__461 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__462 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__462 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__462 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__463 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__463 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__463 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__464 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__464 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__464 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__465 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__465 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__465 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__466 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__466 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__466 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__467 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__467 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__467 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__468 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__468 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__468 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__469 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__469 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__469 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__470 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__470 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__470 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__471 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__471 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__471 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__472 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__472 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__472 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__473 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__473 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__473 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__474 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__474 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__474 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__475 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__475 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__475 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__476 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__476 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__476 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__477 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__477 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__477 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__478 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__478 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__478 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__479 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__479 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__479 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__480 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__480 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__480 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__481 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__481 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__481 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__482 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__482 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__482 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__483 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__483 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__483 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__484 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__484 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__484 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__485 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__485 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__485 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__486 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__486 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__486 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__487 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__487 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__487 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__488 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__488 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__488 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__489 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__489 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__489 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__490 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__490 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__490 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__491 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__491 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__491 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__492 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__492 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__492 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__493 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__493 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__493 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__494 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__494 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__494 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__495 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__495 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__495 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__496 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__496 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__496 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__497 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__497 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__497 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__498 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__498 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__498 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__499 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__499 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__499 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__500 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__500 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__500 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__501 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__501 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__501 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__502 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__502 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__502 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__503 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__503 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__503 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__504 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__504 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__504 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__505 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__505 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__505 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__506 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__506 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__506 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__507 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__507 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__507 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__508 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__508 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__508 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__509 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__509 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__509 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__510 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__510 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__510 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__511 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__511 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__511 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__512 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__512 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__512 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__513 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__513 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__513 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__514 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__514 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__514 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__515 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__515 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__515 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__516 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__516 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__516 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__517 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__517 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__517 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__518 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__518 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__518 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__519 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__519 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__519 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__520 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__520 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__520 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__521 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__521 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__521 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__522 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__522 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__522 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__523 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__523 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__523 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__524 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__524 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__524 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__525 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__525 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__525 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__526 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__526 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__526 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__527 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__527 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__527 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__528 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__528 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__528 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__529 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__529 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__529 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__530 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__530 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__530 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__531 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__531 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__531 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__532 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__532 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__532 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__533 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__533 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__533 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__534 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__534 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__534 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__535 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__535 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__535 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__536 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__536 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__536 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__537 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__537 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__537 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__538 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__538 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__538 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__539 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__539 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__539 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__540 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__540 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__540 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__541 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__541 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__541 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__542 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__542 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__542 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__543 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__543 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__543 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__544 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__544 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__544 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__545 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__545 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__545 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__546 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__546 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__546 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__547 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__547 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__547 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__548 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__548 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__548 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__549 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__549 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__549 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__550 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__550 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__550 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__551 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__551 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__551 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__552 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__552 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__552 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__553 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__553 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__553 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__554 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__554 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__554 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__555 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__555 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__555 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__556 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__556 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__556 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__557 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__557 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__557 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__558 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__558 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__558 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__559 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__559 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__559 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__560 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__560 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__560 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__561 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__561 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__561 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__562 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__562 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__562 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__563 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__563 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__563 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__564 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__564 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__564 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__565 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__565 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__565 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__566 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__566 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__566 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__567 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__567 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__567 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__568 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__568 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__568 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__569 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__569 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__569 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__570 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__570 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__570 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__571 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__571 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__571 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__572 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__572 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__572 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__573 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__573 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__573 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__574 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__574 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__574 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__575 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__575 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__575 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__576 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__576 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__576 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__577 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__577 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__577 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__578 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__578 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__578 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__579 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__579 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__579 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__580 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__580 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__580 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__581 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__581 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__581 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__582 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__582 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__582 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__583 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__583 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__583 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__584 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__584 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__584 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__585 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__585 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__585 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__586 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__586 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__586 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__587 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__587 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__587 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__588 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__588 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__588 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__589 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__589 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__589 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__590 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__590 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__590 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__591 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__591 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__591 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__592 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__592 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__592 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__593 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__593 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__593 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__594 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__594 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__594 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__595 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__595 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__595 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__596 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__596 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__596 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__597 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__597 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__597 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__598 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__598 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__598 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__599 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__599 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__599 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__600 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__600 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__600 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__601 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__601 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__601 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__602 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__602 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__602 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__603 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__603 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__603 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__604 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__604 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__604 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__605 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__605 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__605 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__606 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__606 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__606 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__607 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__607 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__607 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__608 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__608 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__608 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__609 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__609 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__609 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__610 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__610 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__610 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__611 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__611 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__611 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__612 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__612 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__612 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__613 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__613 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__613 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__614 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__614 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__614 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__615 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__615 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__615 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__616 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__616 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__616 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__617 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__617 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__617 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__618 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__618 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__618 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__619 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__619 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__619 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__620 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__620 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__620 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__621 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__621 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__621 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__622 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__622 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__622 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__623 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__623 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__623 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__624 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__624 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__624 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__625 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__625 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__625 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__626 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__626 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__626 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__627 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__627 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__627 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__628 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__628 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__628 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__629 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__629 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__629 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__630 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__630 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__630 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__631 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__631 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__631 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__632 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__632 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__632 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__633 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__633 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__633 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__634 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__634 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__634 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__635 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__635 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__635 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__636 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__636 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__636 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__637 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__637 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__637 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__638 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__638 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__638 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__639 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__639 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__639 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__640 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__640 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__640 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__641 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__641 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__641 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__642 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__642 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__642 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__643 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__643 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__643 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__644 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__644 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__644 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__645 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__645 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__645 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__646 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__646 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__646 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__647 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__647 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__647 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__648 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__648 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__648 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__649 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__649 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__649 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__650 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__650 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__650 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__651 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__651 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__651 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__652 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__652 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__652 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__653 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__653 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__653 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__654 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__654 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__654 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__655 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__655 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__655 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__656 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__656 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__656 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__657 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__657 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__657 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__658 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__658 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__658 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__659 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__659 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__659 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__660 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__660 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__660 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__661 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__661 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__661 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__662 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__662 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__662 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__663 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__663 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__663 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__664 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__664 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__664 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__665 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__665 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__665 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__666 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__666 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__666 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__667 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__667 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__667 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__668 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__668 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__668 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__669 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__669 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__669 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__670 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__670 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__670 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__671 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__671 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__671 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__672 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__672 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__672 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__673 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__673 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__673 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__674 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__674 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__674 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__675 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__675 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__675 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__676 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__676 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__676 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__677 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__677 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__677 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__678 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__678 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__678 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__679 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__679 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__679 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__680 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__680 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__680 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__681 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__681 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__681 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__682 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__682 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__682 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__683 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__683 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__683 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__684 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__684 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__684 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__685 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__685 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__685 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__686 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__686 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__686 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__687 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__687 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__687 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__688 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__688 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__688 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__689 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__689 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__689 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__690 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__690 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__690 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__691 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__691 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__691 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__692 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__692 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__692 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__693 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__693 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__693 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__694 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__694 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__694 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__695 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__695 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__695 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__696 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__696 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__696 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__697 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__697 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__697 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__698 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__698 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__698 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__699 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__699 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__699 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__700 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__700 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__700 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__701 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__701 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__701 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__702 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__702 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__702 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__703 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__703 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__703 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__704 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__704 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__704 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__705 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__705 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__705 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__706 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__706 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__706 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__707 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__707 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__707 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__708 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__708 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__708 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__709 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__709 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__709 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__710 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__710 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__710 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__711 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__711 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__711 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__712 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__712 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__712 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__713 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__713 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__713 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__714 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__714 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__714 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__715 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__715 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__715 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__716 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__716 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__716 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__717 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__717 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__717 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__718 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__718 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__718 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__719 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__719 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__719 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__720 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__720 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__720 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__721 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__721 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__721 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__722 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__722 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__722 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__723 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__723 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__723 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__724 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__724 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__724 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__725 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__725 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__725 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__726 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__726 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__726 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__727 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__727 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__727 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__728 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__728 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__728 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__729 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__729 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__729 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__730 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__730 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__730 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__731 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__731 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__731 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__732 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__732 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__732 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__733 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__733 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__733 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__734 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__734 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__734 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__735 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__735 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__735 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__736 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__736 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__736 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__737 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__737 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__737 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__738 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__738 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__738 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__739 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__739 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__739 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__740 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__740 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__740 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__741 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__741 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__741 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__742 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__742 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__742 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__743 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__743 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__743 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__744 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__744 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__744 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__745 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__745 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__745 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__746 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__746 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__746 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__747 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__747 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__747 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__748 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__748 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__748 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__749 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__749 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__749 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__750 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__750 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__750 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__751 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__751 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__751 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__752 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__752 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__752 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__753 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__753 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__753 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__754 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__754 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__754 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__755 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__755 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__755 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__756 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__756 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__756 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__757 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__757 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__757 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__758 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__758 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__758 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__759 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__759 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__759 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__760 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__760 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__760 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__761 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__761 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__761 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__762 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__762 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__762 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__763 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__763 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__763 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__764 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__764 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__764 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__765 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__765 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__765 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__766 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__766 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__766 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__767 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__767 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__767 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__768 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__768 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__768 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__769 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__769 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__769 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__770 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__770 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__770 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__771 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__771 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__771 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__772 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__772 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__772 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__773 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__773 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__773 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__774 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__774 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__774 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__775 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__775 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__775 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__776 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__776 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__776 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__777 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__777 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__777 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__778 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__778 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__778 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__779 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__779 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__779 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__780 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__780 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__780 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__781 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__781 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__781 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__782 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__782 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__782 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__783 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__783 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__783 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__784 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__784 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__784 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__785 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__785 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__785 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__786 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__786 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__786 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__787 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__787 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__787 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__788 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__788 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__788 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__789 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__789 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__789 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__790 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__790 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__790 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__791 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__791 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__791 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__792 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__792 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__792 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__793 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__793 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__793 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__794 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__794 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__794 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__795 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__795 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__795 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__796 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__796 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__796 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__797 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__797 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__797 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__798 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__798 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__798 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__799 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__799 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__799 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__800 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__800 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__800 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__801 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__801 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__801 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__802 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__802 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__802 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__803 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__803 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__803 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__804 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__804 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__804 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__805 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__805 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__805 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__806 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__806 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__806 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__807 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__807 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__807 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__808 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__808 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__808 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__809 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__809 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__809 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__810 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__810 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__810 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__811 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__811 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__811 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__812 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__812 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__812 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__813 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__813 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__813 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__814 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__814 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__814 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__815 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__815 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__815 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__816 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__816 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__816 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__817 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__817 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__817 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__818 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__818 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__818 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__819 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__819 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__819 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__820 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__820 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__820 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__821 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__821 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__821 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__822 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__822 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__822 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__823 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__823 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__823 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__824 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__824 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__824 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__825 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__825 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__825 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__826 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__826 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__826 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__827 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__827 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__827 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__828 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__828 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__828 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__829 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__829 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__829 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__830 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__830 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__830 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__831 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__831 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__831 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__832 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__832 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__832 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__833 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__833 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__833 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__834 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__834 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__834 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__835 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__835 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__835 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__836 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__836 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__836 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__837 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__837 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__837 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__838 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__838 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__838 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__839 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__839 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__839 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__840 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__840 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__840 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__841 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__841 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__841 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__842 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__842 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__842 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__843 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__843 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__843 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__844 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__844 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__844 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__845 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__845 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__845 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__846 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__846 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__846 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__847 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__847 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__847 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__848 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__848 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__848 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__849 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__849 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__849 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__850 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__850 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__850 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__851 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__851 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__851 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__852 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__852 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__852 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__853 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__853 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__853 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__854 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__854 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__854 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__855 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__855 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__855 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__856 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__856 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__856 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__857 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__857 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__857 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__858 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__858 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__858 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__859 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__859 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__859 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__860 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__860 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__860 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__861 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__861 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__861 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__862 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__862 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__862 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__863 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__863 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__863 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__864 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__864 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__864 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__865 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__865 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__865 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__866 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__866 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__866 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__867 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__867 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__867 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__868 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__868 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__868 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__869 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__869 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__869 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__870 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__870 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__870 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__871 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__871 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__871 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__872 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__872 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__872 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__873 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__873 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__873 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__874 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__874 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__874 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__875 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__875 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__875 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__876 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__876 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__876 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__877 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__877 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__877 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__878 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__878 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__878 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__879 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__879 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__879 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__880 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__880 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__880 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__881 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__881 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__881 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__882 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__882 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__882 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__883 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__883 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__883 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__884 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__884 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__884 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__885 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__885 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__885 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__886 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__886 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__886 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__887 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__887 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__887 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__888 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__888 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__888 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__889 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__889 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__889 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__890 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__890 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__890 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__891 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__891 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__891 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__892 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__892 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__892 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__893 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__893 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__893 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__894 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__894 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__894 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__895 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__895 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__895 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__896 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__896 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__896 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__897 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__897 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__897 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__898 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__898 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__898 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__899 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__899 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__899 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__900 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__900 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__900 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__901 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__901 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__901 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__902 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__902 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__902 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__903 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__903 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__903 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__904 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__904 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__904 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__905 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__905 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__905 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__906 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__906 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__906 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__907 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__907 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__907 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__908 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__908 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__908 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__909 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__909 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__909 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__910 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__910 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__910 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__911 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__911 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__911 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__912 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__912 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__912 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__913 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__913 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__913 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__914 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__914 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__914 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__915 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__915 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__915 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__916 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__916 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__916 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__917 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__917 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__917 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__918 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__918 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__918 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__919 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__919 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__919 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__920 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__920 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__920 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__921 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__921 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__921 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__922 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__922 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__922 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__923 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__923 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__923 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__924 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__924 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__924 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__925 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__925 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__925 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__926 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__926 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__926 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__927 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__927 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__927 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__928 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__928 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__928 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__929 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__929 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__929 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__930 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__930 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__930 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__931 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__931 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__931 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__932 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__932 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__932 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__933 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__933 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__933 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__934 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__934 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__934 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__935 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__935 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__935 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__936 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__936 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__936 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__937 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__937 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__937 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__938 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__938 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__938 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__939 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__939 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__939 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__940 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__940 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__940 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__941 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__941 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__941 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__942 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__942 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__942 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__943 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__943 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__943 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__944 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__944 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__944 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__945 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__945 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__945 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__946 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__946 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__946 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__947 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__947 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__947 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__948 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__948 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__948 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__949 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__949 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__949 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__950 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__950 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__950 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__951 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__951 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__951 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__952 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__952 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__952 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__953 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__953 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__953 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__954 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__954 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__954 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__955 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__955 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__955 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__956 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__956 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__956 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__957 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__957 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__957 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__958 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__958 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__958 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__959 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__959 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__959 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__960 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__960 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__960 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__961 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__961 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__961 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__962 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__962 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__962 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__963 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__963 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__963 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__964 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__964 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__964 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__965 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__965 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__965 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__966 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__966 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__966 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__967 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__967 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__967 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__968 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__968 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__968 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__969 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__969 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__969 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__970 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__970 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__970 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__971 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__971 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__971 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__972 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__972 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__972 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__973 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__973 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__973 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__974 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__974 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__974 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__975 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__975 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__975 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__976 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__976 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__976 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__977 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__977 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__977 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__978 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__978 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__978 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__979 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__979 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__979 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__980 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__980 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__980 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__981 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__981 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__981 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__982 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__982 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__982 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__983 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__983 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__983 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__984 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__984 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__984 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__985 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__985 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__985 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__986 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__986 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__986 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__987 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__987 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__987 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__988 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__988 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__988 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__989 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__989 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__989 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__990 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__990 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__990 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__991 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__991 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__991 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__992 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__992 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__992 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__993 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__993 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__993 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__994 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__994 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__994 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__995 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__995 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__995 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__996 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__996 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__996 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__997 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__997 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__997 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__998 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__998 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__998 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__999 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__999 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__999 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1000 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1000 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1000 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1001 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1001 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1001 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1002 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1002 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1002 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1003 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1003 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1003 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1004 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1004 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1004 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1005 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1005 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1005 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1006 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1006 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1006 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1007 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1007 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1007 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1008 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1008 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1008 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1009 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1009 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1009 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1010 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1010 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1010 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1011 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1011 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1011 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1012 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1012 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1012 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1013 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1013 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1013 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1014 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1014 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1014 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1015 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1015 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1015 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1016 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1016 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1016 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1017 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1017 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1017 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1018 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1018 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1018 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1019 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1019 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1019 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1020 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1020 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1020 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1021 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1021 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1021 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1022 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1022 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1022 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}

/// Description Address Register
pub mod SRAM0__1023 {
    pub use super::SRAM0__0::ADDR;
}

/// Description Data Register
pub mod SRAM1__1023 {
    pub use super::SRAM1__0::DATA;
}

/// Description Control Register
pub mod SRAM2__1023 {
    pub use super::SRAM2__0::RT_EN;
    pub use super::SRAM2__0::SIZE;
    pub use super::SRAM2__0::SV_EN;
    pub use super::SRAM2__0::TYPE;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Description Address Register
    pub SRAM0__0: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__0: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__0: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// Description Address Register
    pub SRAM0__1: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// Description Address Register
    pub SRAM0__2: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__2: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__2: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// Description Address Register
    pub SRAM0__3: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__3: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__3: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// Description Address Register
    pub SRAM0__4: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__4: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__4: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// Description Address Register
    pub SRAM0__5: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__5: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__5: RWRegister<u32>,

    _reserved6: [u32; 1],

    /// Description Address Register
    pub SRAM0__6: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__6: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__6: RWRegister<u32>,

    _reserved7: [u32; 1],

    /// Description Address Register
    pub SRAM0__7: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__7: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__7: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// Description Address Register
    pub SRAM0__8: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__8: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__8: RWRegister<u32>,

    _reserved9: [u32; 1],

    /// Description Address Register
    pub SRAM0__9: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__9: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__9: RWRegister<u32>,

    _reserved10: [u32; 1],

    /// Description Address Register
    pub SRAM0__10: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__10: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__10: RWRegister<u32>,

    _reserved11: [u32; 1],

    /// Description Address Register
    pub SRAM0__11: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__11: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__11: RWRegister<u32>,

    _reserved12: [u32; 1],

    /// Description Address Register
    pub SRAM0__12: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__12: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__12: RWRegister<u32>,

    _reserved13: [u32; 1],

    /// Description Address Register
    pub SRAM0__13: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__13: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__13: RWRegister<u32>,

    _reserved14: [u32; 1],

    /// Description Address Register
    pub SRAM0__14: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__14: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__14: RWRegister<u32>,

    _reserved15: [u32; 1],

    /// Description Address Register
    pub SRAM0__15: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__15: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__15: RWRegister<u32>,

    _reserved16: [u32; 1],

    /// Description Address Register
    pub SRAM0__16: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__16: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__16: RWRegister<u32>,

    _reserved17: [u32; 1],

    /// Description Address Register
    pub SRAM0__17: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__17: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__17: RWRegister<u32>,

    _reserved18: [u32; 1],

    /// Description Address Register
    pub SRAM0__18: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__18: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__18: RWRegister<u32>,

    _reserved19: [u32; 1],

    /// Description Address Register
    pub SRAM0__19: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__19: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__19: RWRegister<u32>,

    _reserved20: [u32; 1],

    /// Description Address Register
    pub SRAM0__20: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__20: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__20: RWRegister<u32>,

    _reserved21: [u32; 1],

    /// Description Address Register
    pub SRAM0__21: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__21: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__21: RWRegister<u32>,

    _reserved22: [u32; 1],

    /// Description Address Register
    pub SRAM0__22: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__22: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__22: RWRegister<u32>,

    _reserved23: [u32; 1],

    /// Description Address Register
    pub SRAM0__23: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__23: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__23: RWRegister<u32>,

    _reserved24: [u32; 1],

    /// Description Address Register
    pub SRAM0__24: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__24: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__24: RWRegister<u32>,

    _reserved25: [u32; 1],

    /// Description Address Register
    pub SRAM0__25: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__25: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__25: RWRegister<u32>,

    _reserved26: [u32; 1],

    /// Description Address Register
    pub SRAM0__26: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__26: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__26: RWRegister<u32>,

    _reserved27: [u32; 1],

    /// Description Address Register
    pub SRAM0__27: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__27: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__27: RWRegister<u32>,

    _reserved28: [u32; 1],

    /// Description Address Register
    pub SRAM0__28: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__28: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__28: RWRegister<u32>,

    _reserved29: [u32; 1],

    /// Description Address Register
    pub SRAM0__29: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__29: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__29: RWRegister<u32>,

    _reserved30: [u32; 1],

    /// Description Address Register
    pub SRAM0__30: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__30: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__30: RWRegister<u32>,

    _reserved31: [u32; 1],

    /// Description Address Register
    pub SRAM0__31: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__31: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__31: RWRegister<u32>,

    _reserved32: [u32; 1],

    /// Description Address Register
    pub SRAM0__32: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__32: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__32: RWRegister<u32>,

    _reserved33: [u32; 1],

    /// Description Address Register
    pub SRAM0__33: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__33: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__33: RWRegister<u32>,

    _reserved34: [u32; 1],

    /// Description Address Register
    pub SRAM0__34: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__34: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__34: RWRegister<u32>,

    _reserved35: [u32; 1],

    /// Description Address Register
    pub SRAM0__35: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__35: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__35: RWRegister<u32>,

    _reserved36: [u32; 1],

    /// Description Address Register
    pub SRAM0__36: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__36: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__36: RWRegister<u32>,

    _reserved37: [u32; 1],

    /// Description Address Register
    pub SRAM0__37: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__37: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__37: RWRegister<u32>,

    _reserved38: [u32; 1],

    /// Description Address Register
    pub SRAM0__38: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__38: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__38: RWRegister<u32>,

    _reserved39: [u32; 1],

    /// Description Address Register
    pub SRAM0__39: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__39: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__39: RWRegister<u32>,

    _reserved40: [u32; 1],

    /// Description Address Register
    pub SRAM0__40: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__40: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__40: RWRegister<u32>,

    _reserved41: [u32; 1],

    /// Description Address Register
    pub SRAM0__41: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__41: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__41: RWRegister<u32>,

    _reserved42: [u32; 1],

    /// Description Address Register
    pub SRAM0__42: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__42: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__42: RWRegister<u32>,

    _reserved43: [u32; 1],

    /// Description Address Register
    pub SRAM0__43: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__43: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__43: RWRegister<u32>,

    _reserved44: [u32; 1],

    /// Description Address Register
    pub SRAM0__44: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__44: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__44: RWRegister<u32>,

    _reserved45: [u32; 1],

    /// Description Address Register
    pub SRAM0__45: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__45: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__45: RWRegister<u32>,

    _reserved46: [u32; 1],

    /// Description Address Register
    pub SRAM0__46: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__46: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__46: RWRegister<u32>,

    _reserved47: [u32; 1],

    /// Description Address Register
    pub SRAM0__47: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__47: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__47: RWRegister<u32>,

    _reserved48: [u32; 1],

    /// Description Address Register
    pub SRAM0__48: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__48: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__48: RWRegister<u32>,

    _reserved49: [u32; 1],

    /// Description Address Register
    pub SRAM0__49: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__49: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__49: RWRegister<u32>,

    _reserved50: [u32; 1],

    /// Description Address Register
    pub SRAM0__50: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__50: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__50: RWRegister<u32>,

    _reserved51: [u32; 1],

    /// Description Address Register
    pub SRAM0__51: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__51: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__51: RWRegister<u32>,

    _reserved52: [u32; 1],

    /// Description Address Register
    pub SRAM0__52: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__52: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__52: RWRegister<u32>,

    _reserved53: [u32; 1],

    /// Description Address Register
    pub SRAM0__53: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__53: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__53: RWRegister<u32>,

    _reserved54: [u32; 1],

    /// Description Address Register
    pub SRAM0__54: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__54: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__54: RWRegister<u32>,

    _reserved55: [u32; 1],

    /// Description Address Register
    pub SRAM0__55: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__55: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__55: RWRegister<u32>,

    _reserved56: [u32; 1],

    /// Description Address Register
    pub SRAM0__56: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__56: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__56: RWRegister<u32>,

    _reserved57: [u32; 1],

    /// Description Address Register
    pub SRAM0__57: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__57: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__57: RWRegister<u32>,

    _reserved58: [u32; 1],

    /// Description Address Register
    pub SRAM0__58: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__58: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__58: RWRegister<u32>,

    _reserved59: [u32; 1],

    /// Description Address Register
    pub SRAM0__59: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__59: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__59: RWRegister<u32>,

    _reserved60: [u32; 1],

    /// Description Address Register
    pub SRAM0__60: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__60: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__60: RWRegister<u32>,

    _reserved61: [u32; 1],

    /// Description Address Register
    pub SRAM0__61: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__61: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__61: RWRegister<u32>,

    _reserved62: [u32; 1],

    /// Description Address Register
    pub SRAM0__62: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__62: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__62: RWRegister<u32>,

    _reserved63: [u32; 1],

    /// Description Address Register
    pub SRAM0__63: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__63: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__63: RWRegister<u32>,

    _reserved64: [u32; 1],

    /// Description Address Register
    pub SRAM0__64: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__64: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__64: RWRegister<u32>,

    _reserved65: [u32; 1],

    /// Description Address Register
    pub SRAM0__65: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__65: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__65: RWRegister<u32>,

    _reserved66: [u32; 1],

    /// Description Address Register
    pub SRAM0__66: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__66: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__66: RWRegister<u32>,

    _reserved67: [u32; 1],

    /// Description Address Register
    pub SRAM0__67: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__67: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__67: RWRegister<u32>,

    _reserved68: [u32; 1],

    /// Description Address Register
    pub SRAM0__68: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__68: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__68: RWRegister<u32>,

    _reserved69: [u32; 1],

    /// Description Address Register
    pub SRAM0__69: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__69: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__69: RWRegister<u32>,

    _reserved70: [u32; 1],

    /// Description Address Register
    pub SRAM0__70: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__70: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__70: RWRegister<u32>,

    _reserved71: [u32; 1],

    /// Description Address Register
    pub SRAM0__71: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__71: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__71: RWRegister<u32>,

    _reserved72: [u32; 1],

    /// Description Address Register
    pub SRAM0__72: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__72: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__72: RWRegister<u32>,

    _reserved73: [u32; 1],

    /// Description Address Register
    pub SRAM0__73: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__73: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__73: RWRegister<u32>,

    _reserved74: [u32; 1],

    /// Description Address Register
    pub SRAM0__74: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__74: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__74: RWRegister<u32>,

    _reserved75: [u32; 1],

    /// Description Address Register
    pub SRAM0__75: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__75: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__75: RWRegister<u32>,

    _reserved76: [u32; 1],

    /// Description Address Register
    pub SRAM0__76: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__76: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__76: RWRegister<u32>,

    _reserved77: [u32; 1],

    /// Description Address Register
    pub SRAM0__77: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__77: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__77: RWRegister<u32>,

    _reserved78: [u32; 1],

    /// Description Address Register
    pub SRAM0__78: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__78: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__78: RWRegister<u32>,

    _reserved79: [u32; 1],

    /// Description Address Register
    pub SRAM0__79: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__79: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__79: RWRegister<u32>,

    _reserved80: [u32; 1],

    /// Description Address Register
    pub SRAM0__80: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__80: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__80: RWRegister<u32>,

    _reserved81: [u32; 1],

    /// Description Address Register
    pub SRAM0__81: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__81: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__81: RWRegister<u32>,

    _reserved82: [u32; 1],

    /// Description Address Register
    pub SRAM0__82: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__82: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__82: RWRegister<u32>,

    _reserved83: [u32; 1],

    /// Description Address Register
    pub SRAM0__83: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__83: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__83: RWRegister<u32>,

    _reserved84: [u32; 1],

    /// Description Address Register
    pub SRAM0__84: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__84: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__84: RWRegister<u32>,

    _reserved85: [u32; 1],

    /// Description Address Register
    pub SRAM0__85: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__85: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__85: RWRegister<u32>,

    _reserved86: [u32; 1],

    /// Description Address Register
    pub SRAM0__86: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__86: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__86: RWRegister<u32>,

    _reserved87: [u32; 1],

    /// Description Address Register
    pub SRAM0__87: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__87: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__87: RWRegister<u32>,

    _reserved88: [u32; 1],

    /// Description Address Register
    pub SRAM0__88: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__88: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__88: RWRegister<u32>,

    _reserved89: [u32; 1],

    /// Description Address Register
    pub SRAM0__89: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__89: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__89: RWRegister<u32>,

    _reserved90: [u32; 1],

    /// Description Address Register
    pub SRAM0__90: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__90: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__90: RWRegister<u32>,

    _reserved91: [u32; 1],

    /// Description Address Register
    pub SRAM0__91: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__91: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__91: RWRegister<u32>,

    _reserved92: [u32; 1],

    /// Description Address Register
    pub SRAM0__92: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__92: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__92: RWRegister<u32>,

    _reserved93: [u32; 1],

    /// Description Address Register
    pub SRAM0__93: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__93: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__93: RWRegister<u32>,

    _reserved94: [u32; 1],

    /// Description Address Register
    pub SRAM0__94: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__94: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__94: RWRegister<u32>,

    _reserved95: [u32; 1],

    /// Description Address Register
    pub SRAM0__95: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__95: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__95: RWRegister<u32>,

    _reserved96: [u32; 1],

    /// Description Address Register
    pub SRAM0__96: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__96: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__96: RWRegister<u32>,

    _reserved97: [u32; 1],

    /// Description Address Register
    pub SRAM0__97: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__97: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__97: RWRegister<u32>,

    _reserved98: [u32; 1],

    /// Description Address Register
    pub SRAM0__98: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__98: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__98: RWRegister<u32>,

    _reserved99: [u32; 1],

    /// Description Address Register
    pub SRAM0__99: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__99: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__99: RWRegister<u32>,

    _reserved100: [u32; 1],

    /// Description Address Register
    pub SRAM0__100: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__100: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__100: RWRegister<u32>,

    _reserved101: [u32; 1],

    /// Description Address Register
    pub SRAM0__101: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__101: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__101: RWRegister<u32>,

    _reserved102: [u32; 1],

    /// Description Address Register
    pub SRAM0__102: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__102: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__102: RWRegister<u32>,

    _reserved103: [u32; 1],

    /// Description Address Register
    pub SRAM0__103: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__103: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__103: RWRegister<u32>,

    _reserved104: [u32; 1],

    /// Description Address Register
    pub SRAM0__104: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__104: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__104: RWRegister<u32>,

    _reserved105: [u32; 1],

    /// Description Address Register
    pub SRAM0__105: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__105: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__105: RWRegister<u32>,

    _reserved106: [u32; 1],

    /// Description Address Register
    pub SRAM0__106: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__106: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__106: RWRegister<u32>,

    _reserved107: [u32; 1],

    /// Description Address Register
    pub SRAM0__107: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__107: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__107: RWRegister<u32>,

    _reserved108: [u32; 1],

    /// Description Address Register
    pub SRAM0__108: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__108: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__108: RWRegister<u32>,

    _reserved109: [u32; 1],

    /// Description Address Register
    pub SRAM0__109: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__109: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__109: RWRegister<u32>,

    _reserved110: [u32; 1],

    /// Description Address Register
    pub SRAM0__110: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__110: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__110: RWRegister<u32>,

    _reserved111: [u32; 1],

    /// Description Address Register
    pub SRAM0__111: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__111: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__111: RWRegister<u32>,

    _reserved112: [u32; 1],

    /// Description Address Register
    pub SRAM0__112: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__112: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__112: RWRegister<u32>,

    _reserved113: [u32; 1],

    /// Description Address Register
    pub SRAM0__113: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__113: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__113: RWRegister<u32>,

    _reserved114: [u32; 1],

    /// Description Address Register
    pub SRAM0__114: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__114: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__114: RWRegister<u32>,

    _reserved115: [u32; 1],

    /// Description Address Register
    pub SRAM0__115: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__115: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__115: RWRegister<u32>,

    _reserved116: [u32; 1],

    /// Description Address Register
    pub SRAM0__116: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__116: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__116: RWRegister<u32>,

    _reserved117: [u32; 1],

    /// Description Address Register
    pub SRAM0__117: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__117: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__117: RWRegister<u32>,

    _reserved118: [u32; 1],

    /// Description Address Register
    pub SRAM0__118: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__118: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__118: RWRegister<u32>,

    _reserved119: [u32; 1],

    /// Description Address Register
    pub SRAM0__119: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__119: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__119: RWRegister<u32>,

    _reserved120: [u32; 1],

    /// Description Address Register
    pub SRAM0__120: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__120: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__120: RWRegister<u32>,

    _reserved121: [u32; 1],

    /// Description Address Register
    pub SRAM0__121: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__121: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__121: RWRegister<u32>,

    _reserved122: [u32; 1],

    /// Description Address Register
    pub SRAM0__122: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__122: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__122: RWRegister<u32>,

    _reserved123: [u32; 1],

    /// Description Address Register
    pub SRAM0__123: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__123: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__123: RWRegister<u32>,

    _reserved124: [u32; 1],

    /// Description Address Register
    pub SRAM0__124: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__124: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__124: RWRegister<u32>,

    _reserved125: [u32; 1],

    /// Description Address Register
    pub SRAM0__125: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__125: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__125: RWRegister<u32>,

    _reserved126: [u32; 1],

    /// Description Address Register
    pub SRAM0__126: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__126: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__126: RWRegister<u32>,

    _reserved127: [u32; 1],

    /// Description Address Register
    pub SRAM0__127: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__127: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__127: RWRegister<u32>,

    _reserved128: [u32; 1],

    /// Description Address Register
    pub SRAM0__128: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__128: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__128: RWRegister<u32>,

    _reserved129: [u32; 1],

    /// Description Address Register
    pub SRAM0__129: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__129: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__129: RWRegister<u32>,

    _reserved130: [u32; 1],

    /// Description Address Register
    pub SRAM0__130: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__130: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__130: RWRegister<u32>,

    _reserved131: [u32; 1],

    /// Description Address Register
    pub SRAM0__131: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__131: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__131: RWRegister<u32>,

    _reserved132: [u32; 1],

    /// Description Address Register
    pub SRAM0__132: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__132: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__132: RWRegister<u32>,

    _reserved133: [u32; 1],

    /// Description Address Register
    pub SRAM0__133: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__133: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__133: RWRegister<u32>,

    _reserved134: [u32; 1],

    /// Description Address Register
    pub SRAM0__134: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__134: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__134: RWRegister<u32>,

    _reserved135: [u32; 1],

    /// Description Address Register
    pub SRAM0__135: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__135: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__135: RWRegister<u32>,

    _reserved136: [u32; 1],

    /// Description Address Register
    pub SRAM0__136: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__136: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__136: RWRegister<u32>,

    _reserved137: [u32; 1],

    /// Description Address Register
    pub SRAM0__137: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__137: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__137: RWRegister<u32>,

    _reserved138: [u32; 1],

    /// Description Address Register
    pub SRAM0__138: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__138: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__138: RWRegister<u32>,

    _reserved139: [u32; 1],

    /// Description Address Register
    pub SRAM0__139: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__139: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__139: RWRegister<u32>,

    _reserved140: [u32; 1],

    /// Description Address Register
    pub SRAM0__140: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__140: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__140: RWRegister<u32>,

    _reserved141: [u32; 1],

    /// Description Address Register
    pub SRAM0__141: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__141: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__141: RWRegister<u32>,

    _reserved142: [u32; 1],

    /// Description Address Register
    pub SRAM0__142: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__142: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__142: RWRegister<u32>,

    _reserved143: [u32; 1],

    /// Description Address Register
    pub SRAM0__143: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__143: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__143: RWRegister<u32>,

    _reserved144: [u32; 1],

    /// Description Address Register
    pub SRAM0__144: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__144: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__144: RWRegister<u32>,

    _reserved145: [u32; 1],

    /// Description Address Register
    pub SRAM0__145: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__145: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__145: RWRegister<u32>,

    _reserved146: [u32; 1],

    /// Description Address Register
    pub SRAM0__146: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__146: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__146: RWRegister<u32>,

    _reserved147: [u32; 1],

    /// Description Address Register
    pub SRAM0__147: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__147: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__147: RWRegister<u32>,

    _reserved148: [u32; 1],

    /// Description Address Register
    pub SRAM0__148: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__148: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__148: RWRegister<u32>,

    _reserved149: [u32; 1],

    /// Description Address Register
    pub SRAM0__149: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__149: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__149: RWRegister<u32>,

    _reserved150: [u32; 1],

    /// Description Address Register
    pub SRAM0__150: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__150: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__150: RWRegister<u32>,

    _reserved151: [u32; 1],

    /// Description Address Register
    pub SRAM0__151: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__151: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__151: RWRegister<u32>,

    _reserved152: [u32; 1],

    /// Description Address Register
    pub SRAM0__152: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__152: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__152: RWRegister<u32>,

    _reserved153: [u32; 1],

    /// Description Address Register
    pub SRAM0__153: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__153: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__153: RWRegister<u32>,

    _reserved154: [u32; 1],

    /// Description Address Register
    pub SRAM0__154: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__154: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__154: RWRegister<u32>,

    _reserved155: [u32; 1],

    /// Description Address Register
    pub SRAM0__155: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__155: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__155: RWRegister<u32>,

    _reserved156: [u32; 1],

    /// Description Address Register
    pub SRAM0__156: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__156: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__156: RWRegister<u32>,

    _reserved157: [u32; 1],

    /// Description Address Register
    pub SRAM0__157: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__157: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__157: RWRegister<u32>,

    _reserved158: [u32; 1],

    /// Description Address Register
    pub SRAM0__158: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__158: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__158: RWRegister<u32>,

    _reserved159: [u32; 1],

    /// Description Address Register
    pub SRAM0__159: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__159: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__159: RWRegister<u32>,

    _reserved160: [u32; 1],

    /// Description Address Register
    pub SRAM0__160: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__160: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__160: RWRegister<u32>,

    _reserved161: [u32; 1],

    /// Description Address Register
    pub SRAM0__161: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__161: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__161: RWRegister<u32>,

    _reserved162: [u32; 1],

    /// Description Address Register
    pub SRAM0__162: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__162: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__162: RWRegister<u32>,

    _reserved163: [u32; 1],

    /// Description Address Register
    pub SRAM0__163: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__163: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__163: RWRegister<u32>,

    _reserved164: [u32; 1],

    /// Description Address Register
    pub SRAM0__164: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__164: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__164: RWRegister<u32>,

    _reserved165: [u32; 1],

    /// Description Address Register
    pub SRAM0__165: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__165: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__165: RWRegister<u32>,

    _reserved166: [u32; 1],

    /// Description Address Register
    pub SRAM0__166: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__166: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__166: RWRegister<u32>,

    _reserved167: [u32; 1],

    /// Description Address Register
    pub SRAM0__167: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__167: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__167: RWRegister<u32>,

    _reserved168: [u32; 1],

    /// Description Address Register
    pub SRAM0__168: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__168: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__168: RWRegister<u32>,

    _reserved169: [u32; 1],

    /// Description Address Register
    pub SRAM0__169: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__169: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__169: RWRegister<u32>,

    _reserved170: [u32; 1],

    /// Description Address Register
    pub SRAM0__170: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__170: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__170: RWRegister<u32>,

    _reserved171: [u32; 1],

    /// Description Address Register
    pub SRAM0__171: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__171: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__171: RWRegister<u32>,

    _reserved172: [u32; 1],

    /// Description Address Register
    pub SRAM0__172: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__172: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__172: RWRegister<u32>,

    _reserved173: [u32; 1],

    /// Description Address Register
    pub SRAM0__173: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__173: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__173: RWRegister<u32>,

    _reserved174: [u32; 1],

    /// Description Address Register
    pub SRAM0__174: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__174: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__174: RWRegister<u32>,

    _reserved175: [u32; 1],

    /// Description Address Register
    pub SRAM0__175: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__175: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__175: RWRegister<u32>,

    _reserved176: [u32; 1],

    /// Description Address Register
    pub SRAM0__176: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__176: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__176: RWRegister<u32>,

    _reserved177: [u32; 1],

    /// Description Address Register
    pub SRAM0__177: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__177: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__177: RWRegister<u32>,

    _reserved178: [u32; 1],

    /// Description Address Register
    pub SRAM0__178: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__178: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__178: RWRegister<u32>,

    _reserved179: [u32; 1],

    /// Description Address Register
    pub SRAM0__179: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__179: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__179: RWRegister<u32>,

    _reserved180: [u32; 1],

    /// Description Address Register
    pub SRAM0__180: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__180: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__180: RWRegister<u32>,

    _reserved181: [u32; 1],

    /// Description Address Register
    pub SRAM0__181: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__181: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__181: RWRegister<u32>,

    _reserved182: [u32; 1],

    /// Description Address Register
    pub SRAM0__182: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__182: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__182: RWRegister<u32>,

    _reserved183: [u32; 1],

    /// Description Address Register
    pub SRAM0__183: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__183: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__183: RWRegister<u32>,

    _reserved184: [u32; 1],

    /// Description Address Register
    pub SRAM0__184: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__184: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__184: RWRegister<u32>,

    _reserved185: [u32; 1],

    /// Description Address Register
    pub SRAM0__185: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__185: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__185: RWRegister<u32>,

    _reserved186: [u32; 1],

    /// Description Address Register
    pub SRAM0__186: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__186: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__186: RWRegister<u32>,

    _reserved187: [u32; 1],

    /// Description Address Register
    pub SRAM0__187: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__187: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__187: RWRegister<u32>,

    _reserved188: [u32; 1],

    /// Description Address Register
    pub SRAM0__188: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__188: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__188: RWRegister<u32>,

    _reserved189: [u32; 1],

    /// Description Address Register
    pub SRAM0__189: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__189: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__189: RWRegister<u32>,

    _reserved190: [u32; 1],

    /// Description Address Register
    pub SRAM0__190: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__190: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__190: RWRegister<u32>,

    _reserved191: [u32; 1],

    /// Description Address Register
    pub SRAM0__191: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__191: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__191: RWRegister<u32>,

    _reserved192: [u32; 1],

    /// Description Address Register
    pub SRAM0__192: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__192: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__192: RWRegister<u32>,

    _reserved193: [u32; 1],

    /// Description Address Register
    pub SRAM0__193: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__193: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__193: RWRegister<u32>,

    _reserved194: [u32; 1],

    /// Description Address Register
    pub SRAM0__194: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__194: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__194: RWRegister<u32>,

    _reserved195: [u32; 1],

    /// Description Address Register
    pub SRAM0__195: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__195: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__195: RWRegister<u32>,

    _reserved196: [u32; 1],

    /// Description Address Register
    pub SRAM0__196: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__196: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__196: RWRegister<u32>,

    _reserved197: [u32; 1],

    /// Description Address Register
    pub SRAM0__197: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__197: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__197: RWRegister<u32>,

    _reserved198: [u32; 1],

    /// Description Address Register
    pub SRAM0__198: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__198: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__198: RWRegister<u32>,

    _reserved199: [u32; 1],

    /// Description Address Register
    pub SRAM0__199: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__199: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__199: RWRegister<u32>,

    _reserved200: [u32; 1],

    /// Description Address Register
    pub SRAM0__200: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__200: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__200: RWRegister<u32>,

    _reserved201: [u32; 1],

    /// Description Address Register
    pub SRAM0__201: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__201: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__201: RWRegister<u32>,

    _reserved202: [u32; 1],

    /// Description Address Register
    pub SRAM0__202: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__202: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__202: RWRegister<u32>,

    _reserved203: [u32; 1],

    /// Description Address Register
    pub SRAM0__203: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__203: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__203: RWRegister<u32>,

    _reserved204: [u32; 1],

    /// Description Address Register
    pub SRAM0__204: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__204: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__204: RWRegister<u32>,

    _reserved205: [u32; 1],

    /// Description Address Register
    pub SRAM0__205: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__205: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__205: RWRegister<u32>,

    _reserved206: [u32; 1],

    /// Description Address Register
    pub SRAM0__206: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__206: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__206: RWRegister<u32>,

    _reserved207: [u32; 1],

    /// Description Address Register
    pub SRAM0__207: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__207: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__207: RWRegister<u32>,

    _reserved208: [u32; 1],

    /// Description Address Register
    pub SRAM0__208: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__208: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__208: RWRegister<u32>,

    _reserved209: [u32; 1],

    /// Description Address Register
    pub SRAM0__209: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__209: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__209: RWRegister<u32>,

    _reserved210: [u32; 1],

    /// Description Address Register
    pub SRAM0__210: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__210: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__210: RWRegister<u32>,

    _reserved211: [u32; 1],

    /// Description Address Register
    pub SRAM0__211: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__211: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__211: RWRegister<u32>,

    _reserved212: [u32; 1],

    /// Description Address Register
    pub SRAM0__212: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__212: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__212: RWRegister<u32>,

    _reserved213: [u32; 1],

    /// Description Address Register
    pub SRAM0__213: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__213: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__213: RWRegister<u32>,

    _reserved214: [u32; 1],

    /// Description Address Register
    pub SRAM0__214: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__214: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__214: RWRegister<u32>,

    _reserved215: [u32; 1],

    /// Description Address Register
    pub SRAM0__215: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__215: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__215: RWRegister<u32>,

    _reserved216: [u32; 1],

    /// Description Address Register
    pub SRAM0__216: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__216: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__216: RWRegister<u32>,

    _reserved217: [u32; 1],

    /// Description Address Register
    pub SRAM0__217: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__217: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__217: RWRegister<u32>,

    _reserved218: [u32; 1],

    /// Description Address Register
    pub SRAM0__218: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__218: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__218: RWRegister<u32>,

    _reserved219: [u32; 1],

    /// Description Address Register
    pub SRAM0__219: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__219: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__219: RWRegister<u32>,

    _reserved220: [u32; 1],

    /// Description Address Register
    pub SRAM0__220: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__220: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__220: RWRegister<u32>,

    _reserved221: [u32; 1],

    /// Description Address Register
    pub SRAM0__221: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__221: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__221: RWRegister<u32>,

    _reserved222: [u32; 1],

    /// Description Address Register
    pub SRAM0__222: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__222: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__222: RWRegister<u32>,

    _reserved223: [u32; 1],

    /// Description Address Register
    pub SRAM0__223: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__223: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__223: RWRegister<u32>,

    _reserved224: [u32; 1],

    /// Description Address Register
    pub SRAM0__224: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__224: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__224: RWRegister<u32>,

    _reserved225: [u32; 1],

    /// Description Address Register
    pub SRAM0__225: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__225: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__225: RWRegister<u32>,

    _reserved226: [u32; 1],

    /// Description Address Register
    pub SRAM0__226: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__226: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__226: RWRegister<u32>,

    _reserved227: [u32; 1],

    /// Description Address Register
    pub SRAM0__227: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__227: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__227: RWRegister<u32>,

    _reserved228: [u32; 1],

    /// Description Address Register
    pub SRAM0__228: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__228: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__228: RWRegister<u32>,

    _reserved229: [u32; 1],

    /// Description Address Register
    pub SRAM0__229: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__229: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__229: RWRegister<u32>,

    _reserved230: [u32; 1],

    /// Description Address Register
    pub SRAM0__230: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__230: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__230: RWRegister<u32>,

    _reserved231: [u32; 1],

    /// Description Address Register
    pub SRAM0__231: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__231: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__231: RWRegister<u32>,

    _reserved232: [u32; 1],

    /// Description Address Register
    pub SRAM0__232: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__232: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__232: RWRegister<u32>,

    _reserved233: [u32; 1],

    /// Description Address Register
    pub SRAM0__233: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__233: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__233: RWRegister<u32>,

    _reserved234: [u32; 1],

    /// Description Address Register
    pub SRAM0__234: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__234: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__234: RWRegister<u32>,

    _reserved235: [u32; 1],

    /// Description Address Register
    pub SRAM0__235: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__235: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__235: RWRegister<u32>,

    _reserved236: [u32; 1],

    /// Description Address Register
    pub SRAM0__236: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__236: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__236: RWRegister<u32>,

    _reserved237: [u32; 1],

    /// Description Address Register
    pub SRAM0__237: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__237: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__237: RWRegister<u32>,

    _reserved238: [u32; 1],

    /// Description Address Register
    pub SRAM0__238: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__238: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__238: RWRegister<u32>,

    _reserved239: [u32; 1],

    /// Description Address Register
    pub SRAM0__239: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__239: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__239: RWRegister<u32>,

    _reserved240: [u32; 1],

    /// Description Address Register
    pub SRAM0__240: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__240: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__240: RWRegister<u32>,

    _reserved241: [u32; 1],

    /// Description Address Register
    pub SRAM0__241: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__241: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__241: RWRegister<u32>,

    _reserved242: [u32; 1],

    /// Description Address Register
    pub SRAM0__242: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__242: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__242: RWRegister<u32>,

    _reserved243: [u32; 1],

    /// Description Address Register
    pub SRAM0__243: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__243: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__243: RWRegister<u32>,

    _reserved244: [u32; 1],

    /// Description Address Register
    pub SRAM0__244: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__244: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__244: RWRegister<u32>,

    _reserved245: [u32; 1],

    /// Description Address Register
    pub SRAM0__245: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__245: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__245: RWRegister<u32>,

    _reserved246: [u32; 1],

    /// Description Address Register
    pub SRAM0__246: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__246: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__246: RWRegister<u32>,

    _reserved247: [u32; 1],

    /// Description Address Register
    pub SRAM0__247: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__247: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__247: RWRegister<u32>,

    _reserved248: [u32; 1],

    /// Description Address Register
    pub SRAM0__248: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__248: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__248: RWRegister<u32>,

    _reserved249: [u32; 1],

    /// Description Address Register
    pub SRAM0__249: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__249: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__249: RWRegister<u32>,

    _reserved250: [u32; 1],

    /// Description Address Register
    pub SRAM0__250: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__250: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__250: RWRegister<u32>,

    _reserved251: [u32; 1],

    /// Description Address Register
    pub SRAM0__251: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__251: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__251: RWRegister<u32>,

    _reserved252: [u32; 1],

    /// Description Address Register
    pub SRAM0__252: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__252: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__252: RWRegister<u32>,

    _reserved253: [u32; 1],

    /// Description Address Register
    pub SRAM0__253: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__253: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__253: RWRegister<u32>,

    _reserved254: [u32; 1],

    /// Description Address Register
    pub SRAM0__254: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__254: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__254: RWRegister<u32>,

    _reserved255: [u32; 1],

    /// Description Address Register
    pub SRAM0__255: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__255: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__255: RWRegister<u32>,

    _reserved256: [u32; 1],

    /// Description Address Register
    pub SRAM0__256: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__256: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__256: RWRegister<u32>,

    _reserved257: [u32; 1],

    /// Description Address Register
    pub SRAM0__257: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__257: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__257: RWRegister<u32>,

    _reserved258: [u32; 1],

    /// Description Address Register
    pub SRAM0__258: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__258: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__258: RWRegister<u32>,

    _reserved259: [u32; 1],

    /// Description Address Register
    pub SRAM0__259: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__259: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__259: RWRegister<u32>,

    _reserved260: [u32; 1],

    /// Description Address Register
    pub SRAM0__260: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__260: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__260: RWRegister<u32>,

    _reserved261: [u32; 1],

    /// Description Address Register
    pub SRAM0__261: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__261: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__261: RWRegister<u32>,

    _reserved262: [u32; 1],

    /// Description Address Register
    pub SRAM0__262: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__262: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__262: RWRegister<u32>,

    _reserved263: [u32; 1],

    /// Description Address Register
    pub SRAM0__263: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__263: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__263: RWRegister<u32>,

    _reserved264: [u32; 1],

    /// Description Address Register
    pub SRAM0__264: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__264: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__264: RWRegister<u32>,

    _reserved265: [u32; 1],

    /// Description Address Register
    pub SRAM0__265: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__265: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__265: RWRegister<u32>,

    _reserved266: [u32; 1],

    /// Description Address Register
    pub SRAM0__266: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__266: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__266: RWRegister<u32>,

    _reserved267: [u32; 1],

    /// Description Address Register
    pub SRAM0__267: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__267: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__267: RWRegister<u32>,

    _reserved268: [u32; 1],

    /// Description Address Register
    pub SRAM0__268: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__268: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__268: RWRegister<u32>,

    _reserved269: [u32; 1],

    /// Description Address Register
    pub SRAM0__269: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__269: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__269: RWRegister<u32>,

    _reserved270: [u32; 1],

    /// Description Address Register
    pub SRAM0__270: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__270: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__270: RWRegister<u32>,

    _reserved271: [u32; 1],

    /// Description Address Register
    pub SRAM0__271: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__271: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__271: RWRegister<u32>,

    _reserved272: [u32; 1],

    /// Description Address Register
    pub SRAM0__272: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__272: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__272: RWRegister<u32>,

    _reserved273: [u32; 1],

    /// Description Address Register
    pub SRAM0__273: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__273: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__273: RWRegister<u32>,

    _reserved274: [u32; 1],

    /// Description Address Register
    pub SRAM0__274: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__274: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__274: RWRegister<u32>,

    _reserved275: [u32; 1],

    /// Description Address Register
    pub SRAM0__275: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__275: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__275: RWRegister<u32>,

    _reserved276: [u32; 1],

    /// Description Address Register
    pub SRAM0__276: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__276: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__276: RWRegister<u32>,

    _reserved277: [u32; 1],

    /// Description Address Register
    pub SRAM0__277: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__277: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__277: RWRegister<u32>,

    _reserved278: [u32; 1],

    /// Description Address Register
    pub SRAM0__278: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__278: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__278: RWRegister<u32>,

    _reserved279: [u32; 1],

    /// Description Address Register
    pub SRAM0__279: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__279: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__279: RWRegister<u32>,

    _reserved280: [u32; 1],

    /// Description Address Register
    pub SRAM0__280: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__280: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__280: RWRegister<u32>,

    _reserved281: [u32; 1],

    /// Description Address Register
    pub SRAM0__281: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__281: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__281: RWRegister<u32>,

    _reserved282: [u32; 1],

    /// Description Address Register
    pub SRAM0__282: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__282: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__282: RWRegister<u32>,

    _reserved283: [u32; 1],

    /// Description Address Register
    pub SRAM0__283: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__283: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__283: RWRegister<u32>,

    _reserved284: [u32; 1],

    /// Description Address Register
    pub SRAM0__284: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__284: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__284: RWRegister<u32>,

    _reserved285: [u32; 1],

    /// Description Address Register
    pub SRAM0__285: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__285: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__285: RWRegister<u32>,

    _reserved286: [u32; 1],

    /// Description Address Register
    pub SRAM0__286: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__286: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__286: RWRegister<u32>,

    _reserved287: [u32; 1],

    /// Description Address Register
    pub SRAM0__287: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__287: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__287: RWRegister<u32>,

    _reserved288: [u32; 1],

    /// Description Address Register
    pub SRAM0__288: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__288: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__288: RWRegister<u32>,

    _reserved289: [u32; 1],

    /// Description Address Register
    pub SRAM0__289: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__289: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__289: RWRegister<u32>,

    _reserved290: [u32; 1],

    /// Description Address Register
    pub SRAM0__290: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__290: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__290: RWRegister<u32>,

    _reserved291: [u32; 1],

    /// Description Address Register
    pub SRAM0__291: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__291: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__291: RWRegister<u32>,

    _reserved292: [u32; 1],

    /// Description Address Register
    pub SRAM0__292: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__292: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__292: RWRegister<u32>,

    _reserved293: [u32; 1],

    /// Description Address Register
    pub SRAM0__293: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__293: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__293: RWRegister<u32>,

    _reserved294: [u32; 1],

    /// Description Address Register
    pub SRAM0__294: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__294: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__294: RWRegister<u32>,

    _reserved295: [u32; 1],

    /// Description Address Register
    pub SRAM0__295: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__295: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__295: RWRegister<u32>,

    _reserved296: [u32; 1],

    /// Description Address Register
    pub SRAM0__296: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__296: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__296: RWRegister<u32>,

    _reserved297: [u32; 1],

    /// Description Address Register
    pub SRAM0__297: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__297: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__297: RWRegister<u32>,

    _reserved298: [u32; 1],

    /// Description Address Register
    pub SRAM0__298: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__298: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__298: RWRegister<u32>,

    _reserved299: [u32; 1],

    /// Description Address Register
    pub SRAM0__299: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__299: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__299: RWRegister<u32>,

    _reserved300: [u32; 1],

    /// Description Address Register
    pub SRAM0__300: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__300: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__300: RWRegister<u32>,

    _reserved301: [u32; 1],

    /// Description Address Register
    pub SRAM0__301: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__301: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__301: RWRegister<u32>,

    _reserved302: [u32; 1],

    /// Description Address Register
    pub SRAM0__302: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__302: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__302: RWRegister<u32>,

    _reserved303: [u32; 1],

    /// Description Address Register
    pub SRAM0__303: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__303: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__303: RWRegister<u32>,

    _reserved304: [u32; 1],

    /// Description Address Register
    pub SRAM0__304: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__304: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__304: RWRegister<u32>,

    _reserved305: [u32; 1],

    /// Description Address Register
    pub SRAM0__305: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__305: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__305: RWRegister<u32>,

    _reserved306: [u32; 1],

    /// Description Address Register
    pub SRAM0__306: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__306: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__306: RWRegister<u32>,

    _reserved307: [u32; 1],

    /// Description Address Register
    pub SRAM0__307: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__307: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__307: RWRegister<u32>,

    _reserved308: [u32; 1],

    /// Description Address Register
    pub SRAM0__308: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__308: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__308: RWRegister<u32>,

    _reserved309: [u32; 1],

    /// Description Address Register
    pub SRAM0__309: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__309: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__309: RWRegister<u32>,

    _reserved310: [u32; 1],

    /// Description Address Register
    pub SRAM0__310: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__310: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__310: RWRegister<u32>,

    _reserved311: [u32; 1],

    /// Description Address Register
    pub SRAM0__311: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__311: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__311: RWRegister<u32>,

    _reserved312: [u32; 1],

    /// Description Address Register
    pub SRAM0__312: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__312: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__312: RWRegister<u32>,

    _reserved313: [u32; 1],

    /// Description Address Register
    pub SRAM0__313: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__313: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__313: RWRegister<u32>,

    _reserved314: [u32; 1],

    /// Description Address Register
    pub SRAM0__314: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__314: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__314: RWRegister<u32>,

    _reserved315: [u32; 1],

    /// Description Address Register
    pub SRAM0__315: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__315: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__315: RWRegister<u32>,

    _reserved316: [u32; 1],

    /// Description Address Register
    pub SRAM0__316: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__316: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__316: RWRegister<u32>,

    _reserved317: [u32; 1],

    /// Description Address Register
    pub SRAM0__317: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__317: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__317: RWRegister<u32>,

    _reserved318: [u32; 1],

    /// Description Address Register
    pub SRAM0__318: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__318: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__318: RWRegister<u32>,

    _reserved319: [u32; 1],

    /// Description Address Register
    pub SRAM0__319: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__319: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__319: RWRegister<u32>,

    _reserved320: [u32; 1],

    /// Description Address Register
    pub SRAM0__320: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__320: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__320: RWRegister<u32>,

    _reserved321: [u32; 1],

    /// Description Address Register
    pub SRAM0__321: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__321: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__321: RWRegister<u32>,

    _reserved322: [u32; 1],

    /// Description Address Register
    pub SRAM0__322: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__322: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__322: RWRegister<u32>,

    _reserved323: [u32; 1],

    /// Description Address Register
    pub SRAM0__323: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__323: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__323: RWRegister<u32>,

    _reserved324: [u32; 1],

    /// Description Address Register
    pub SRAM0__324: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__324: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__324: RWRegister<u32>,

    _reserved325: [u32; 1],

    /// Description Address Register
    pub SRAM0__325: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__325: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__325: RWRegister<u32>,

    _reserved326: [u32; 1],

    /// Description Address Register
    pub SRAM0__326: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__326: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__326: RWRegister<u32>,

    _reserved327: [u32; 1],

    /// Description Address Register
    pub SRAM0__327: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__327: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__327: RWRegister<u32>,

    _reserved328: [u32; 1],

    /// Description Address Register
    pub SRAM0__328: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__328: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__328: RWRegister<u32>,

    _reserved329: [u32; 1],

    /// Description Address Register
    pub SRAM0__329: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__329: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__329: RWRegister<u32>,

    _reserved330: [u32; 1],

    /// Description Address Register
    pub SRAM0__330: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__330: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__330: RWRegister<u32>,

    _reserved331: [u32; 1],

    /// Description Address Register
    pub SRAM0__331: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__331: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__331: RWRegister<u32>,

    _reserved332: [u32; 1],

    /// Description Address Register
    pub SRAM0__332: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__332: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__332: RWRegister<u32>,

    _reserved333: [u32; 1],

    /// Description Address Register
    pub SRAM0__333: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__333: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__333: RWRegister<u32>,

    _reserved334: [u32; 1],

    /// Description Address Register
    pub SRAM0__334: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__334: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__334: RWRegister<u32>,

    _reserved335: [u32; 1],

    /// Description Address Register
    pub SRAM0__335: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__335: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__335: RWRegister<u32>,

    _reserved336: [u32; 1],

    /// Description Address Register
    pub SRAM0__336: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__336: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__336: RWRegister<u32>,

    _reserved337: [u32; 1],

    /// Description Address Register
    pub SRAM0__337: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__337: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__337: RWRegister<u32>,

    _reserved338: [u32; 1],

    /// Description Address Register
    pub SRAM0__338: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__338: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__338: RWRegister<u32>,

    _reserved339: [u32; 1],

    /// Description Address Register
    pub SRAM0__339: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__339: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__339: RWRegister<u32>,

    _reserved340: [u32; 1],

    /// Description Address Register
    pub SRAM0__340: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__340: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__340: RWRegister<u32>,

    _reserved341: [u32; 1],

    /// Description Address Register
    pub SRAM0__341: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__341: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__341: RWRegister<u32>,

    _reserved342: [u32; 1],

    /// Description Address Register
    pub SRAM0__342: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__342: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__342: RWRegister<u32>,

    _reserved343: [u32; 1],

    /// Description Address Register
    pub SRAM0__343: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__343: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__343: RWRegister<u32>,

    _reserved344: [u32; 1],

    /// Description Address Register
    pub SRAM0__344: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__344: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__344: RWRegister<u32>,

    _reserved345: [u32; 1],

    /// Description Address Register
    pub SRAM0__345: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__345: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__345: RWRegister<u32>,

    _reserved346: [u32; 1],

    /// Description Address Register
    pub SRAM0__346: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__346: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__346: RWRegister<u32>,

    _reserved347: [u32; 1],

    /// Description Address Register
    pub SRAM0__347: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__347: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__347: RWRegister<u32>,

    _reserved348: [u32; 1],

    /// Description Address Register
    pub SRAM0__348: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__348: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__348: RWRegister<u32>,

    _reserved349: [u32; 1],

    /// Description Address Register
    pub SRAM0__349: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__349: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__349: RWRegister<u32>,

    _reserved350: [u32; 1],

    /// Description Address Register
    pub SRAM0__350: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__350: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__350: RWRegister<u32>,

    _reserved351: [u32; 1],

    /// Description Address Register
    pub SRAM0__351: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__351: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__351: RWRegister<u32>,

    _reserved352: [u32; 1],

    /// Description Address Register
    pub SRAM0__352: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__352: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__352: RWRegister<u32>,

    _reserved353: [u32; 1],

    /// Description Address Register
    pub SRAM0__353: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__353: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__353: RWRegister<u32>,

    _reserved354: [u32; 1],

    /// Description Address Register
    pub SRAM0__354: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__354: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__354: RWRegister<u32>,

    _reserved355: [u32; 1],

    /// Description Address Register
    pub SRAM0__355: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__355: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__355: RWRegister<u32>,

    _reserved356: [u32; 1],

    /// Description Address Register
    pub SRAM0__356: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__356: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__356: RWRegister<u32>,

    _reserved357: [u32; 1],

    /// Description Address Register
    pub SRAM0__357: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__357: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__357: RWRegister<u32>,

    _reserved358: [u32; 1],

    /// Description Address Register
    pub SRAM0__358: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__358: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__358: RWRegister<u32>,

    _reserved359: [u32; 1],

    /// Description Address Register
    pub SRAM0__359: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__359: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__359: RWRegister<u32>,

    _reserved360: [u32; 1],

    /// Description Address Register
    pub SRAM0__360: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__360: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__360: RWRegister<u32>,

    _reserved361: [u32; 1],

    /// Description Address Register
    pub SRAM0__361: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__361: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__361: RWRegister<u32>,

    _reserved362: [u32; 1],

    /// Description Address Register
    pub SRAM0__362: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__362: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__362: RWRegister<u32>,

    _reserved363: [u32; 1],

    /// Description Address Register
    pub SRAM0__363: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__363: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__363: RWRegister<u32>,

    _reserved364: [u32; 1],

    /// Description Address Register
    pub SRAM0__364: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__364: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__364: RWRegister<u32>,

    _reserved365: [u32; 1],

    /// Description Address Register
    pub SRAM0__365: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__365: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__365: RWRegister<u32>,

    _reserved366: [u32; 1],

    /// Description Address Register
    pub SRAM0__366: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__366: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__366: RWRegister<u32>,

    _reserved367: [u32; 1],

    /// Description Address Register
    pub SRAM0__367: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__367: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__367: RWRegister<u32>,

    _reserved368: [u32; 1],

    /// Description Address Register
    pub SRAM0__368: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__368: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__368: RWRegister<u32>,

    _reserved369: [u32; 1],

    /// Description Address Register
    pub SRAM0__369: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__369: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__369: RWRegister<u32>,

    _reserved370: [u32; 1],

    /// Description Address Register
    pub SRAM0__370: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__370: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__370: RWRegister<u32>,

    _reserved371: [u32; 1],

    /// Description Address Register
    pub SRAM0__371: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__371: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__371: RWRegister<u32>,

    _reserved372: [u32; 1],

    /// Description Address Register
    pub SRAM0__372: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__372: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__372: RWRegister<u32>,

    _reserved373: [u32; 1],

    /// Description Address Register
    pub SRAM0__373: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__373: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__373: RWRegister<u32>,

    _reserved374: [u32; 1],

    /// Description Address Register
    pub SRAM0__374: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__374: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__374: RWRegister<u32>,

    _reserved375: [u32; 1],

    /// Description Address Register
    pub SRAM0__375: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__375: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__375: RWRegister<u32>,

    _reserved376: [u32; 1],

    /// Description Address Register
    pub SRAM0__376: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__376: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__376: RWRegister<u32>,

    _reserved377: [u32; 1],

    /// Description Address Register
    pub SRAM0__377: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__377: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__377: RWRegister<u32>,

    _reserved378: [u32; 1],

    /// Description Address Register
    pub SRAM0__378: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__378: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__378: RWRegister<u32>,

    _reserved379: [u32; 1],

    /// Description Address Register
    pub SRAM0__379: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__379: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__379: RWRegister<u32>,

    _reserved380: [u32; 1],

    /// Description Address Register
    pub SRAM0__380: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__380: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__380: RWRegister<u32>,

    _reserved381: [u32; 1],

    /// Description Address Register
    pub SRAM0__381: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__381: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__381: RWRegister<u32>,

    _reserved382: [u32; 1],

    /// Description Address Register
    pub SRAM0__382: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__382: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__382: RWRegister<u32>,

    _reserved383: [u32; 1],

    /// Description Address Register
    pub SRAM0__383: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__383: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__383: RWRegister<u32>,

    _reserved384: [u32; 1],

    /// Description Address Register
    pub SRAM0__384: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__384: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__384: RWRegister<u32>,

    _reserved385: [u32; 1],

    /// Description Address Register
    pub SRAM0__385: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__385: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__385: RWRegister<u32>,

    _reserved386: [u32; 1],

    /// Description Address Register
    pub SRAM0__386: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__386: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__386: RWRegister<u32>,

    _reserved387: [u32; 1],

    /// Description Address Register
    pub SRAM0__387: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__387: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__387: RWRegister<u32>,

    _reserved388: [u32; 1],

    /// Description Address Register
    pub SRAM0__388: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__388: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__388: RWRegister<u32>,

    _reserved389: [u32; 1],

    /// Description Address Register
    pub SRAM0__389: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__389: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__389: RWRegister<u32>,

    _reserved390: [u32; 1],

    /// Description Address Register
    pub SRAM0__390: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__390: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__390: RWRegister<u32>,

    _reserved391: [u32; 1],

    /// Description Address Register
    pub SRAM0__391: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__391: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__391: RWRegister<u32>,

    _reserved392: [u32; 1],

    /// Description Address Register
    pub SRAM0__392: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__392: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__392: RWRegister<u32>,

    _reserved393: [u32; 1],

    /// Description Address Register
    pub SRAM0__393: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__393: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__393: RWRegister<u32>,

    _reserved394: [u32; 1],

    /// Description Address Register
    pub SRAM0__394: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__394: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__394: RWRegister<u32>,

    _reserved395: [u32; 1],

    /// Description Address Register
    pub SRAM0__395: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__395: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__395: RWRegister<u32>,

    _reserved396: [u32; 1],

    /// Description Address Register
    pub SRAM0__396: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__396: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__396: RWRegister<u32>,

    _reserved397: [u32; 1],

    /// Description Address Register
    pub SRAM0__397: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__397: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__397: RWRegister<u32>,

    _reserved398: [u32; 1],

    /// Description Address Register
    pub SRAM0__398: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__398: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__398: RWRegister<u32>,

    _reserved399: [u32; 1],

    /// Description Address Register
    pub SRAM0__399: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__399: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__399: RWRegister<u32>,

    _reserved400: [u32; 1],

    /// Description Address Register
    pub SRAM0__400: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__400: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__400: RWRegister<u32>,

    _reserved401: [u32; 1],

    /// Description Address Register
    pub SRAM0__401: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__401: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__401: RWRegister<u32>,

    _reserved402: [u32; 1],

    /// Description Address Register
    pub SRAM0__402: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__402: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__402: RWRegister<u32>,

    _reserved403: [u32; 1],

    /// Description Address Register
    pub SRAM0__403: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__403: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__403: RWRegister<u32>,

    _reserved404: [u32; 1],

    /// Description Address Register
    pub SRAM0__404: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__404: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__404: RWRegister<u32>,

    _reserved405: [u32; 1],

    /// Description Address Register
    pub SRAM0__405: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__405: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__405: RWRegister<u32>,

    _reserved406: [u32; 1],

    /// Description Address Register
    pub SRAM0__406: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__406: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__406: RWRegister<u32>,

    _reserved407: [u32; 1],

    /// Description Address Register
    pub SRAM0__407: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__407: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__407: RWRegister<u32>,

    _reserved408: [u32; 1],

    /// Description Address Register
    pub SRAM0__408: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__408: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__408: RWRegister<u32>,

    _reserved409: [u32; 1],

    /// Description Address Register
    pub SRAM0__409: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__409: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__409: RWRegister<u32>,

    _reserved410: [u32; 1],

    /// Description Address Register
    pub SRAM0__410: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__410: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__410: RWRegister<u32>,

    _reserved411: [u32; 1],

    /// Description Address Register
    pub SRAM0__411: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__411: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__411: RWRegister<u32>,

    _reserved412: [u32; 1],

    /// Description Address Register
    pub SRAM0__412: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__412: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__412: RWRegister<u32>,

    _reserved413: [u32; 1],

    /// Description Address Register
    pub SRAM0__413: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__413: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__413: RWRegister<u32>,

    _reserved414: [u32; 1],

    /// Description Address Register
    pub SRAM0__414: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__414: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__414: RWRegister<u32>,

    _reserved415: [u32; 1],

    /// Description Address Register
    pub SRAM0__415: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__415: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__415: RWRegister<u32>,

    _reserved416: [u32; 1],

    /// Description Address Register
    pub SRAM0__416: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__416: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__416: RWRegister<u32>,

    _reserved417: [u32; 1],

    /// Description Address Register
    pub SRAM0__417: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__417: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__417: RWRegister<u32>,

    _reserved418: [u32; 1],

    /// Description Address Register
    pub SRAM0__418: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__418: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__418: RWRegister<u32>,

    _reserved419: [u32; 1],

    /// Description Address Register
    pub SRAM0__419: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__419: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__419: RWRegister<u32>,

    _reserved420: [u32; 1],

    /// Description Address Register
    pub SRAM0__420: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__420: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__420: RWRegister<u32>,

    _reserved421: [u32; 1],

    /// Description Address Register
    pub SRAM0__421: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__421: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__421: RWRegister<u32>,

    _reserved422: [u32; 1],

    /// Description Address Register
    pub SRAM0__422: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__422: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__422: RWRegister<u32>,

    _reserved423: [u32; 1],

    /// Description Address Register
    pub SRAM0__423: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__423: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__423: RWRegister<u32>,

    _reserved424: [u32; 1],

    /// Description Address Register
    pub SRAM0__424: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__424: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__424: RWRegister<u32>,

    _reserved425: [u32; 1],

    /// Description Address Register
    pub SRAM0__425: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__425: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__425: RWRegister<u32>,

    _reserved426: [u32; 1],

    /// Description Address Register
    pub SRAM0__426: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__426: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__426: RWRegister<u32>,

    _reserved427: [u32; 1],

    /// Description Address Register
    pub SRAM0__427: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__427: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__427: RWRegister<u32>,

    _reserved428: [u32; 1],

    /// Description Address Register
    pub SRAM0__428: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__428: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__428: RWRegister<u32>,

    _reserved429: [u32; 1],

    /// Description Address Register
    pub SRAM0__429: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__429: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__429: RWRegister<u32>,

    _reserved430: [u32; 1],

    /// Description Address Register
    pub SRAM0__430: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__430: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__430: RWRegister<u32>,

    _reserved431: [u32; 1],

    /// Description Address Register
    pub SRAM0__431: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__431: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__431: RWRegister<u32>,

    _reserved432: [u32; 1],

    /// Description Address Register
    pub SRAM0__432: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__432: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__432: RWRegister<u32>,

    _reserved433: [u32; 1],

    /// Description Address Register
    pub SRAM0__433: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__433: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__433: RWRegister<u32>,

    _reserved434: [u32; 1],

    /// Description Address Register
    pub SRAM0__434: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__434: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__434: RWRegister<u32>,

    _reserved435: [u32; 1],

    /// Description Address Register
    pub SRAM0__435: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__435: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__435: RWRegister<u32>,

    _reserved436: [u32; 1],

    /// Description Address Register
    pub SRAM0__436: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__436: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__436: RWRegister<u32>,

    _reserved437: [u32; 1],

    /// Description Address Register
    pub SRAM0__437: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__437: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__437: RWRegister<u32>,

    _reserved438: [u32; 1],

    /// Description Address Register
    pub SRAM0__438: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__438: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__438: RWRegister<u32>,

    _reserved439: [u32; 1],

    /// Description Address Register
    pub SRAM0__439: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__439: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__439: RWRegister<u32>,

    _reserved440: [u32; 1],

    /// Description Address Register
    pub SRAM0__440: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__440: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__440: RWRegister<u32>,

    _reserved441: [u32; 1],

    /// Description Address Register
    pub SRAM0__441: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__441: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__441: RWRegister<u32>,

    _reserved442: [u32; 1],

    /// Description Address Register
    pub SRAM0__442: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__442: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__442: RWRegister<u32>,

    _reserved443: [u32; 1],

    /// Description Address Register
    pub SRAM0__443: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__443: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__443: RWRegister<u32>,

    _reserved444: [u32; 1],

    /// Description Address Register
    pub SRAM0__444: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__444: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__444: RWRegister<u32>,

    _reserved445: [u32; 1],

    /// Description Address Register
    pub SRAM0__445: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__445: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__445: RWRegister<u32>,

    _reserved446: [u32; 1],

    /// Description Address Register
    pub SRAM0__446: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__446: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__446: RWRegister<u32>,

    _reserved447: [u32; 1],

    /// Description Address Register
    pub SRAM0__447: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__447: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__447: RWRegister<u32>,

    _reserved448: [u32; 1],

    /// Description Address Register
    pub SRAM0__448: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__448: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__448: RWRegister<u32>,

    _reserved449: [u32; 1],

    /// Description Address Register
    pub SRAM0__449: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__449: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__449: RWRegister<u32>,

    _reserved450: [u32; 1],

    /// Description Address Register
    pub SRAM0__450: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__450: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__450: RWRegister<u32>,

    _reserved451: [u32; 1],

    /// Description Address Register
    pub SRAM0__451: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__451: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__451: RWRegister<u32>,

    _reserved452: [u32; 1],

    /// Description Address Register
    pub SRAM0__452: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__452: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__452: RWRegister<u32>,

    _reserved453: [u32; 1],

    /// Description Address Register
    pub SRAM0__453: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__453: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__453: RWRegister<u32>,

    _reserved454: [u32; 1],

    /// Description Address Register
    pub SRAM0__454: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__454: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__454: RWRegister<u32>,

    _reserved455: [u32; 1],

    /// Description Address Register
    pub SRAM0__455: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__455: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__455: RWRegister<u32>,

    _reserved456: [u32; 1],

    /// Description Address Register
    pub SRAM0__456: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__456: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__456: RWRegister<u32>,

    _reserved457: [u32; 1],

    /// Description Address Register
    pub SRAM0__457: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__457: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__457: RWRegister<u32>,

    _reserved458: [u32; 1],

    /// Description Address Register
    pub SRAM0__458: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__458: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__458: RWRegister<u32>,

    _reserved459: [u32; 1],

    /// Description Address Register
    pub SRAM0__459: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__459: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__459: RWRegister<u32>,

    _reserved460: [u32; 1],

    /// Description Address Register
    pub SRAM0__460: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__460: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__460: RWRegister<u32>,

    _reserved461: [u32; 1],

    /// Description Address Register
    pub SRAM0__461: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__461: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__461: RWRegister<u32>,

    _reserved462: [u32; 1],

    /// Description Address Register
    pub SRAM0__462: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__462: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__462: RWRegister<u32>,

    _reserved463: [u32; 1],

    /// Description Address Register
    pub SRAM0__463: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__463: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__463: RWRegister<u32>,

    _reserved464: [u32; 1],

    /// Description Address Register
    pub SRAM0__464: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__464: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__464: RWRegister<u32>,

    _reserved465: [u32; 1],

    /// Description Address Register
    pub SRAM0__465: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__465: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__465: RWRegister<u32>,

    _reserved466: [u32; 1],

    /// Description Address Register
    pub SRAM0__466: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__466: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__466: RWRegister<u32>,

    _reserved467: [u32; 1],

    /// Description Address Register
    pub SRAM0__467: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__467: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__467: RWRegister<u32>,

    _reserved468: [u32; 1],

    /// Description Address Register
    pub SRAM0__468: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__468: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__468: RWRegister<u32>,

    _reserved469: [u32; 1],

    /// Description Address Register
    pub SRAM0__469: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__469: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__469: RWRegister<u32>,

    _reserved470: [u32; 1],

    /// Description Address Register
    pub SRAM0__470: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__470: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__470: RWRegister<u32>,

    _reserved471: [u32; 1],

    /// Description Address Register
    pub SRAM0__471: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__471: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__471: RWRegister<u32>,

    _reserved472: [u32; 1],

    /// Description Address Register
    pub SRAM0__472: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__472: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__472: RWRegister<u32>,

    _reserved473: [u32; 1],

    /// Description Address Register
    pub SRAM0__473: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__473: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__473: RWRegister<u32>,

    _reserved474: [u32; 1],

    /// Description Address Register
    pub SRAM0__474: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__474: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__474: RWRegister<u32>,

    _reserved475: [u32; 1],

    /// Description Address Register
    pub SRAM0__475: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__475: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__475: RWRegister<u32>,

    _reserved476: [u32; 1],

    /// Description Address Register
    pub SRAM0__476: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__476: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__476: RWRegister<u32>,

    _reserved477: [u32; 1],

    /// Description Address Register
    pub SRAM0__477: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__477: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__477: RWRegister<u32>,

    _reserved478: [u32; 1],

    /// Description Address Register
    pub SRAM0__478: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__478: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__478: RWRegister<u32>,

    _reserved479: [u32; 1],

    /// Description Address Register
    pub SRAM0__479: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__479: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__479: RWRegister<u32>,

    _reserved480: [u32; 1],

    /// Description Address Register
    pub SRAM0__480: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__480: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__480: RWRegister<u32>,

    _reserved481: [u32; 1],

    /// Description Address Register
    pub SRAM0__481: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__481: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__481: RWRegister<u32>,

    _reserved482: [u32; 1],

    /// Description Address Register
    pub SRAM0__482: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__482: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__482: RWRegister<u32>,

    _reserved483: [u32; 1],

    /// Description Address Register
    pub SRAM0__483: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__483: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__483: RWRegister<u32>,

    _reserved484: [u32; 1],

    /// Description Address Register
    pub SRAM0__484: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__484: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__484: RWRegister<u32>,

    _reserved485: [u32; 1],

    /// Description Address Register
    pub SRAM0__485: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__485: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__485: RWRegister<u32>,

    _reserved486: [u32; 1],

    /// Description Address Register
    pub SRAM0__486: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__486: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__486: RWRegister<u32>,

    _reserved487: [u32; 1],

    /// Description Address Register
    pub SRAM0__487: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__487: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__487: RWRegister<u32>,

    _reserved488: [u32; 1],

    /// Description Address Register
    pub SRAM0__488: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__488: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__488: RWRegister<u32>,

    _reserved489: [u32; 1],

    /// Description Address Register
    pub SRAM0__489: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__489: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__489: RWRegister<u32>,

    _reserved490: [u32; 1],

    /// Description Address Register
    pub SRAM0__490: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__490: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__490: RWRegister<u32>,

    _reserved491: [u32; 1],

    /// Description Address Register
    pub SRAM0__491: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__491: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__491: RWRegister<u32>,

    _reserved492: [u32; 1],

    /// Description Address Register
    pub SRAM0__492: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__492: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__492: RWRegister<u32>,

    _reserved493: [u32; 1],

    /// Description Address Register
    pub SRAM0__493: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__493: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__493: RWRegister<u32>,

    _reserved494: [u32; 1],

    /// Description Address Register
    pub SRAM0__494: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__494: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__494: RWRegister<u32>,

    _reserved495: [u32; 1],

    /// Description Address Register
    pub SRAM0__495: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__495: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__495: RWRegister<u32>,

    _reserved496: [u32; 1],

    /// Description Address Register
    pub SRAM0__496: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__496: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__496: RWRegister<u32>,

    _reserved497: [u32; 1],

    /// Description Address Register
    pub SRAM0__497: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__497: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__497: RWRegister<u32>,

    _reserved498: [u32; 1],

    /// Description Address Register
    pub SRAM0__498: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__498: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__498: RWRegister<u32>,

    _reserved499: [u32; 1],

    /// Description Address Register
    pub SRAM0__499: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__499: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__499: RWRegister<u32>,

    _reserved500: [u32; 1],

    /// Description Address Register
    pub SRAM0__500: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__500: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__500: RWRegister<u32>,

    _reserved501: [u32; 1],

    /// Description Address Register
    pub SRAM0__501: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__501: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__501: RWRegister<u32>,

    _reserved502: [u32; 1],

    /// Description Address Register
    pub SRAM0__502: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__502: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__502: RWRegister<u32>,

    _reserved503: [u32; 1],

    /// Description Address Register
    pub SRAM0__503: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__503: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__503: RWRegister<u32>,

    _reserved504: [u32; 1],

    /// Description Address Register
    pub SRAM0__504: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__504: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__504: RWRegister<u32>,

    _reserved505: [u32; 1],

    /// Description Address Register
    pub SRAM0__505: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__505: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__505: RWRegister<u32>,

    _reserved506: [u32; 1],

    /// Description Address Register
    pub SRAM0__506: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__506: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__506: RWRegister<u32>,

    _reserved507: [u32; 1],

    /// Description Address Register
    pub SRAM0__507: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__507: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__507: RWRegister<u32>,

    _reserved508: [u32; 1],

    /// Description Address Register
    pub SRAM0__508: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__508: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__508: RWRegister<u32>,

    _reserved509: [u32; 1],

    /// Description Address Register
    pub SRAM0__509: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__509: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__509: RWRegister<u32>,

    _reserved510: [u32; 1],

    /// Description Address Register
    pub SRAM0__510: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__510: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__510: RWRegister<u32>,

    _reserved511: [u32; 1],

    /// Description Address Register
    pub SRAM0__511: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__511: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__511: RWRegister<u32>,

    _reserved512: [u32; 1],

    /// Description Address Register
    pub SRAM0__512: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__512: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__512: RWRegister<u32>,

    _reserved513: [u32; 1],

    /// Description Address Register
    pub SRAM0__513: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__513: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__513: RWRegister<u32>,

    _reserved514: [u32; 1],

    /// Description Address Register
    pub SRAM0__514: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__514: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__514: RWRegister<u32>,

    _reserved515: [u32; 1],

    /// Description Address Register
    pub SRAM0__515: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__515: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__515: RWRegister<u32>,

    _reserved516: [u32; 1],

    /// Description Address Register
    pub SRAM0__516: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__516: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__516: RWRegister<u32>,

    _reserved517: [u32; 1],

    /// Description Address Register
    pub SRAM0__517: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__517: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__517: RWRegister<u32>,

    _reserved518: [u32; 1],

    /// Description Address Register
    pub SRAM0__518: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__518: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__518: RWRegister<u32>,

    _reserved519: [u32; 1],

    /// Description Address Register
    pub SRAM0__519: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__519: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__519: RWRegister<u32>,

    _reserved520: [u32; 1],

    /// Description Address Register
    pub SRAM0__520: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__520: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__520: RWRegister<u32>,

    _reserved521: [u32; 1],

    /// Description Address Register
    pub SRAM0__521: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__521: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__521: RWRegister<u32>,

    _reserved522: [u32; 1],

    /// Description Address Register
    pub SRAM0__522: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__522: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__522: RWRegister<u32>,

    _reserved523: [u32; 1],

    /// Description Address Register
    pub SRAM0__523: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__523: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__523: RWRegister<u32>,

    _reserved524: [u32; 1],

    /// Description Address Register
    pub SRAM0__524: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__524: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__524: RWRegister<u32>,

    _reserved525: [u32; 1],

    /// Description Address Register
    pub SRAM0__525: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__525: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__525: RWRegister<u32>,

    _reserved526: [u32; 1],

    /// Description Address Register
    pub SRAM0__526: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__526: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__526: RWRegister<u32>,

    _reserved527: [u32; 1],

    /// Description Address Register
    pub SRAM0__527: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__527: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__527: RWRegister<u32>,

    _reserved528: [u32; 1],

    /// Description Address Register
    pub SRAM0__528: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__528: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__528: RWRegister<u32>,

    _reserved529: [u32; 1],

    /// Description Address Register
    pub SRAM0__529: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__529: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__529: RWRegister<u32>,

    _reserved530: [u32; 1],

    /// Description Address Register
    pub SRAM0__530: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__530: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__530: RWRegister<u32>,

    _reserved531: [u32; 1],

    /// Description Address Register
    pub SRAM0__531: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__531: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__531: RWRegister<u32>,

    _reserved532: [u32; 1],

    /// Description Address Register
    pub SRAM0__532: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__532: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__532: RWRegister<u32>,

    _reserved533: [u32; 1],

    /// Description Address Register
    pub SRAM0__533: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__533: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__533: RWRegister<u32>,

    _reserved534: [u32; 1],

    /// Description Address Register
    pub SRAM0__534: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__534: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__534: RWRegister<u32>,

    _reserved535: [u32; 1],

    /// Description Address Register
    pub SRAM0__535: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__535: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__535: RWRegister<u32>,

    _reserved536: [u32; 1],

    /// Description Address Register
    pub SRAM0__536: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__536: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__536: RWRegister<u32>,

    _reserved537: [u32; 1],

    /// Description Address Register
    pub SRAM0__537: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__537: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__537: RWRegister<u32>,

    _reserved538: [u32; 1],

    /// Description Address Register
    pub SRAM0__538: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__538: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__538: RWRegister<u32>,

    _reserved539: [u32; 1],

    /// Description Address Register
    pub SRAM0__539: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__539: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__539: RWRegister<u32>,

    _reserved540: [u32; 1],

    /// Description Address Register
    pub SRAM0__540: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__540: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__540: RWRegister<u32>,

    _reserved541: [u32; 1],

    /// Description Address Register
    pub SRAM0__541: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__541: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__541: RWRegister<u32>,

    _reserved542: [u32; 1],

    /// Description Address Register
    pub SRAM0__542: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__542: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__542: RWRegister<u32>,

    _reserved543: [u32; 1],

    /// Description Address Register
    pub SRAM0__543: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__543: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__543: RWRegister<u32>,

    _reserved544: [u32; 1],

    /// Description Address Register
    pub SRAM0__544: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__544: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__544: RWRegister<u32>,

    _reserved545: [u32; 1],

    /// Description Address Register
    pub SRAM0__545: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__545: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__545: RWRegister<u32>,

    _reserved546: [u32; 1],

    /// Description Address Register
    pub SRAM0__546: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__546: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__546: RWRegister<u32>,

    _reserved547: [u32; 1],

    /// Description Address Register
    pub SRAM0__547: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__547: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__547: RWRegister<u32>,

    _reserved548: [u32; 1],

    /// Description Address Register
    pub SRAM0__548: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__548: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__548: RWRegister<u32>,

    _reserved549: [u32; 1],

    /// Description Address Register
    pub SRAM0__549: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__549: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__549: RWRegister<u32>,

    _reserved550: [u32; 1],

    /// Description Address Register
    pub SRAM0__550: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__550: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__550: RWRegister<u32>,

    _reserved551: [u32; 1],

    /// Description Address Register
    pub SRAM0__551: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__551: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__551: RWRegister<u32>,

    _reserved552: [u32; 1],

    /// Description Address Register
    pub SRAM0__552: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__552: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__552: RWRegister<u32>,

    _reserved553: [u32; 1],

    /// Description Address Register
    pub SRAM0__553: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__553: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__553: RWRegister<u32>,

    _reserved554: [u32; 1],

    /// Description Address Register
    pub SRAM0__554: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__554: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__554: RWRegister<u32>,

    _reserved555: [u32; 1],

    /// Description Address Register
    pub SRAM0__555: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__555: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__555: RWRegister<u32>,

    _reserved556: [u32; 1],

    /// Description Address Register
    pub SRAM0__556: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__556: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__556: RWRegister<u32>,

    _reserved557: [u32; 1],

    /// Description Address Register
    pub SRAM0__557: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__557: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__557: RWRegister<u32>,

    _reserved558: [u32; 1],

    /// Description Address Register
    pub SRAM0__558: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__558: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__558: RWRegister<u32>,

    _reserved559: [u32; 1],

    /// Description Address Register
    pub SRAM0__559: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__559: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__559: RWRegister<u32>,

    _reserved560: [u32; 1],

    /// Description Address Register
    pub SRAM0__560: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__560: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__560: RWRegister<u32>,

    _reserved561: [u32; 1],

    /// Description Address Register
    pub SRAM0__561: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__561: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__561: RWRegister<u32>,

    _reserved562: [u32; 1],

    /// Description Address Register
    pub SRAM0__562: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__562: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__562: RWRegister<u32>,

    _reserved563: [u32; 1],

    /// Description Address Register
    pub SRAM0__563: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__563: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__563: RWRegister<u32>,

    _reserved564: [u32; 1],

    /// Description Address Register
    pub SRAM0__564: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__564: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__564: RWRegister<u32>,

    _reserved565: [u32; 1],

    /// Description Address Register
    pub SRAM0__565: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__565: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__565: RWRegister<u32>,

    _reserved566: [u32; 1],

    /// Description Address Register
    pub SRAM0__566: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__566: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__566: RWRegister<u32>,

    _reserved567: [u32; 1],

    /// Description Address Register
    pub SRAM0__567: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__567: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__567: RWRegister<u32>,

    _reserved568: [u32; 1],

    /// Description Address Register
    pub SRAM0__568: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__568: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__568: RWRegister<u32>,

    _reserved569: [u32; 1],

    /// Description Address Register
    pub SRAM0__569: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__569: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__569: RWRegister<u32>,

    _reserved570: [u32; 1],

    /// Description Address Register
    pub SRAM0__570: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__570: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__570: RWRegister<u32>,

    _reserved571: [u32; 1],

    /// Description Address Register
    pub SRAM0__571: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__571: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__571: RWRegister<u32>,

    _reserved572: [u32; 1],

    /// Description Address Register
    pub SRAM0__572: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__572: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__572: RWRegister<u32>,

    _reserved573: [u32; 1],

    /// Description Address Register
    pub SRAM0__573: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__573: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__573: RWRegister<u32>,

    _reserved574: [u32; 1],

    /// Description Address Register
    pub SRAM0__574: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__574: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__574: RWRegister<u32>,

    _reserved575: [u32; 1],

    /// Description Address Register
    pub SRAM0__575: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__575: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__575: RWRegister<u32>,

    _reserved576: [u32; 1],

    /// Description Address Register
    pub SRAM0__576: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__576: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__576: RWRegister<u32>,

    _reserved577: [u32; 1],

    /// Description Address Register
    pub SRAM0__577: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__577: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__577: RWRegister<u32>,

    _reserved578: [u32; 1],

    /// Description Address Register
    pub SRAM0__578: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__578: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__578: RWRegister<u32>,

    _reserved579: [u32; 1],

    /// Description Address Register
    pub SRAM0__579: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__579: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__579: RWRegister<u32>,

    _reserved580: [u32; 1],

    /// Description Address Register
    pub SRAM0__580: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__580: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__580: RWRegister<u32>,

    _reserved581: [u32; 1],

    /// Description Address Register
    pub SRAM0__581: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__581: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__581: RWRegister<u32>,

    _reserved582: [u32; 1],

    /// Description Address Register
    pub SRAM0__582: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__582: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__582: RWRegister<u32>,

    _reserved583: [u32; 1],

    /// Description Address Register
    pub SRAM0__583: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__583: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__583: RWRegister<u32>,

    _reserved584: [u32; 1],

    /// Description Address Register
    pub SRAM0__584: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__584: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__584: RWRegister<u32>,

    _reserved585: [u32; 1],

    /// Description Address Register
    pub SRAM0__585: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__585: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__585: RWRegister<u32>,

    _reserved586: [u32; 1],

    /// Description Address Register
    pub SRAM0__586: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__586: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__586: RWRegister<u32>,

    _reserved587: [u32; 1],

    /// Description Address Register
    pub SRAM0__587: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__587: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__587: RWRegister<u32>,

    _reserved588: [u32; 1],

    /// Description Address Register
    pub SRAM0__588: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__588: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__588: RWRegister<u32>,

    _reserved589: [u32; 1],

    /// Description Address Register
    pub SRAM0__589: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__589: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__589: RWRegister<u32>,

    _reserved590: [u32; 1],

    /// Description Address Register
    pub SRAM0__590: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__590: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__590: RWRegister<u32>,

    _reserved591: [u32; 1],

    /// Description Address Register
    pub SRAM0__591: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__591: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__591: RWRegister<u32>,

    _reserved592: [u32; 1],

    /// Description Address Register
    pub SRAM0__592: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__592: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__592: RWRegister<u32>,

    _reserved593: [u32; 1],

    /// Description Address Register
    pub SRAM0__593: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__593: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__593: RWRegister<u32>,

    _reserved594: [u32; 1],

    /// Description Address Register
    pub SRAM0__594: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__594: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__594: RWRegister<u32>,

    _reserved595: [u32; 1],

    /// Description Address Register
    pub SRAM0__595: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__595: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__595: RWRegister<u32>,

    _reserved596: [u32; 1],

    /// Description Address Register
    pub SRAM0__596: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__596: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__596: RWRegister<u32>,

    _reserved597: [u32; 1],

    /// Description Address Register
    pub SRAM0__597: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__597: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__597: RWRegister<u32>,

    _reserved598: [u32; 1],

    /// Description Address Register
    pub SRAM0__598: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__598: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__598: RWRegister<u32>,

    _reserved599: [u32; 1],

    /// Description Address Register
    pub SRAM0__599: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__599: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__599: RWRegister<u32>,

    _reserved600: [u32; 1],

    /// Description Address Register
    pub SRAM0__600: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__600: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__600: RWRegister<u32>,

    _reserved601: [u32; 1],

    /// Description Address Register
    pub SRAM0__601: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__601: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__601: RWRegister<u32>,

    _reserved602: [u32; 1],

    /// Description Address Register
    pub SRAM0__602: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__602: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__602: RWRegister<u32>,

    _reserved603: [u32; 1],

    /// Description Address Register
    pub SRAM0__603: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__603: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__603: RWRegister<u32>,

    _reserved604: [u32; 1],

    /// Description Address Register
    pub SRAM0__604: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__604: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__604: RWRegister<u32>,

    _reserved605: [u32; 1],

    /// Description Address Register
    pub SRAM0__605: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__605: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__605: RWRegister<u32>,

    _reserved606: [u32; 1],

    /// Description Address Register
    pub SRAM0__606: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__606: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__606: RWRegister<u32>,

    _reserved607: [u32; 1],

    /// Description Address Register
    pub SRAM0__607: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__607: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__607: RWRegister<u32>,

    _reserved608: [u32; 1],

    /// Description Address Register
    pub SRAM0__608: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__608: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__608: RWRegister<u32>,

    _reserved609: [u32; 1],

    /// Description Address Register
    pub SRAM0__609: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__609: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__609: RWRegister<u32>,

    _reserved610: [u32; 1],

    /// Description Address Register
    pub SRAM0__610: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__610: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__610: RWRegister<u32>,

    _reserved611: [u32; 1],

    /// Description Address Register
    pub SRAM0__611: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__611: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__611: RWRegister<u32>,

    _reserved612: [u32; 1],

    /// Description Address Register
    pub SRAM0__612: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__612: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__612: RWRegister<u32>,

    _reserved613: [u32; 1],

    /// Description Address Register
    pub SRAM0__613: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__613: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__613: RWRegister<u32>,

    _reserved614: [u32; 1],

    /// Description Address Register
    pub SRAM0__614: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__614: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__614: RWRegister<u32>,

    _reserved615: [u32; 1],

    /// Description Address Register
    pub SRAM0__615: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__615: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__615: RWRegister<u32>,

    _reserved616: [u32; 1],

    /// Description Address Register
    pub SRAM0__616: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__616: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__616: RWRegister<u32>,

    _reserved617: [u32; 1],

    /// Description Address Register
    pub SRAM0__617: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__617: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__617: RWRegister<u32>,

    _reserved618: [u32; 1],

    /// Description Address Register
    pub SRAM0__618: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__618: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__618: RWRegister<u32>,

    _reserved619: [u32; 1],

    /// Description Address Register
    pub SRAM0__619: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__619: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__619: RWRegister<u32>,

    _reserved620: [u32; 1],

    /// Description Address Register
    pub SRAM0__620: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__620: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__620: RWRegister<u32>,

    _reserved621: [u32; 1],

    /// Description Address Register
    pub SRAM0__621: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__621: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__621: RWRegister<u32>,

    _reserved622: [u32; 1],

    /// Description Address Register
    pub SRAM0__622: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__622: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__622: RWRegister<u32>,

    _reserved623: [u32; 1],

    /// Description Address Register
    pub SRAM0__623: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__623: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__623: RWRegister<u32>,

    _reserved624: [u32; 1],

    /// Description Address Register
    pub SRAM0__624: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__624: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__624: RWRegister<u32>,

    _reserved625: [u32; 1],

    /// Description Address Register
    pub SRAM0__625: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__625: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__625: RWRegister<u32>,

    _reserved626: [u32; 1],

    /// Description Address Register
    pub SRAM0__626: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__626: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__626: RWRegister<u32>,

    _reserved627: [u32; 1],

    /// Description Address Register
    pub SRAM0__627: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__627: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__627: RWRegister<u32>,

    _reserved628: [u32; 1],

    /// Description Address Register
    pub SRAM0__628: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__628: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__628: RWRegister<u32>,

    _reserved629: [u32; 1],

    /// Description Address Register
    pub SRAM0__629: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__629: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__629: RWRegister<u32>,

    _reserved630: [u32; 1],

    /// Description Address Register
    pub SRAM0__630: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__630: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__630: RWRegister<u32>,

    _reserved631: [u32; 1],

    /// Description Address Register
    pub SRAM0__631: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__631: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__631: RWRegister<u32>,

    _reserved632: [u32; 1],

    /// Description Address Register
    pub SRAM0__632: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__632: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__632: RWRegister<u32>,

    _reserved633: [u32; 1],

    /// Description Address Register
    pub SRAM0__633: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__633: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__633: RWRegister<u32>,

    _reserved634: [u32; 1],

    /// Description Address Register
    pub SRAM0__634: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__634: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__634: RWRegister<u32>,

    _reserved635: [u32; 1],

    /// Description Address Register
    pub SRAM0__635: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__635: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__635: RWRegister<u32>,

    _reserved636: [u32; 1],

    /// Description Address Register
    pub SRAM0__636: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__636: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__636: RWRegister<u32>,

    _reserved637: [u32; 1],

    /// Description Address Register
    pub SRAM0__637: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__637: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__637: RWRegister<u32>,

    _reserved638: [u32; 1],

    /// Description Address Register
    pub SRAM0__638: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__638: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__638: RWRegister<u32>,

    _reserved639: [u32; 1],

    /// Description Address Register
    pub SRAM0__639: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__639: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__639: RWRegister<u32>,

    _reserved640: [u32; 1],

    /// Description Address Register
    pub SRAM0__640: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__640: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__640: RWRegister<u32>,

    _reserved641: [u32; 1],

    /// Description Address Register
    pub SRAM0__641: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__641: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__641: RWRegister<u32>,

    _reserved642: [u32; 1],

    /// Description Address Register
    pub SRAM0__642: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__642: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__642: RWRegister<u32>,

    _reserved643: [u32; 1],

    /// Description Address Register
    pub SRAM0__643: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__643: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__643: RWRegister<u32>,

    _reserved644: [u32; 1],

    /// Description Address Register
    pub SRAM0__644: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__644: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__644: RWRegister<u32>,

    _reserved645: [u32; 1],

    /// Description Address Register
    pub SRAM0__645: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__645: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__645: RWRegister<u32>,

    _reserved646: [u32; 1],

    /// Description Address Register
    pub SRAM0__646: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__646: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__646: RWRegister<u32>,

    _reserved647: [u32; 1],

    /// Description Address Register
    pub SRAM0__647: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__647: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__647: RWRegister<u32>,

    _reserved648: [u32; 1],

    /// Description Address Register
    pub SRAM0__648: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__648: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__648: RWRegister<u32>,

    _reserved649: [u32; 1],

    /// Description Address Register
    pub SRAM0__649: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__649: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__649: RWRegister<u32>,

    _reserved650: [u32; 1],

    /// Description Address Register
    pub SRAM0__650: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__650: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__650: RWRegister<u32>,

    _reserved651: [u32; 1],

    /// Description Address Register
    pub SRAM0__651: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__651: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__651: RWRegister<u32>,

    _reserved652: [u32; 1],

    /// Description Address Register
    pub SRAM0__652: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__652: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__652: RWRegister<u32>,

    _reserved653: [u32; 1],

    /// Description Address Register
    pub SRAM0__653: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__653: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__653: RWRegister<u32>,

    _reserved654: [u32; 1],

    /// Description Address Register
    pub SRAM0__654: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__654: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__654: RWRegister<u32>,

    _reserved655: [u32; 1],

    /// Description Address Register
    pub SRAM0__655: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__655: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__655: RWRegister<u32>,

    _reserved656: [u32; 1],

    /// Description Address Register
    pub SRAM0__656: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__656: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__656: RWRegister<u32>,

    _reserved657: [u32; 1],

    /// Description Address Register
    pub SRAM0__657: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__657: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__657: RWRegister<u32>,

    _reserved658: [u32; 1],

    /// Description Address Register
    pub SRAM0__658: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__658: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__658: RWRegister<u32>,

    _reserved659: [u32; 1],

    /// Description Address Register
    pub SRAM0__659: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__659: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__659: RWRegister<u32>,

    _reserved660: [u32; 1],

    /// Description Address Register
    pub SRAM0__660: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__660: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__660: RWRegister<u32>,

    _reserved661: [u32; 1],

    /// Description Address Register
    pub SRAM0__661: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__661: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__661: RWRegister<u32>,

    _reserved662: [u32; 1],

    /// Description Address Register
    pub SRAM0__662: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__662: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__662: RWRegister<u32>,

    _reserved663: [u32; 1],

    /// Description Address Register
    pub SRAM0__663: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__663: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__663: RWRegister<u32>,

    _reserved664: [u32; 1],

    /// Description Address Register
    pub SRAM0__664: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__664: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__664: RWRegister<u32>,

    _reserved665: [u32; 1],

    /// Description Address Register
    pub SRAM0__665: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__665: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__665: RWRegister<u32>,

    _reserved666: [u32; 1],

    /// Description Address Register
    pub SRAM0__666: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__666: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__666: RWRegister<u32>,

    _reserved667: [u32; 1],

    /// Description Address Register
    pub SRAM0__667: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__667: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__667: RWRegister<u32>,

    _reserved668: [u32; 1],

    /// Description Address Register
    pub SRAM0__668: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__668: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__668: RWRegister<u32>,

    _reserved669: [u32; 1],

    /// Description Address Register
    pub SRAM0__669: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__669: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__669: RWRegister<u32>,

    _reserved670: [u32; 1],

    /// Description Address Register
    pub SRAM0__670: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__670: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__670: RWRegister<u32>,

    _reserved671: [u32; 1],

    /// Description Address Register
    pub SRAM0__671: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__671: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__671: RWRegister<u32>,

    _reserved672: [u32; 1],

    /// Description Address Register
    pub SRAM0__672: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__672: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__672: RWRegister<u32>,

    _reserved673: [u32; 1],

    /// Description Address Register
    pub SRAM0__673: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__673: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__673: RWRegister<u32>,

    _reserved674: [u32; 1],

    /// Description Address Register
    pub SRAM0__674: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__674: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__674: RWRegister<u32>,

    _reserved675: [u32; 1],

    /// Description Address Register
    pub SRAM0__675: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__675: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__675: RWRegister<u32>,

    _reserved676: [u32; 1],

    /// Description Address Register
    pub SRAM0__676: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__676: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__676: RWRegister<u32>,

    _reserved677: [u32; 1],

    /// Description Address Register
    pub SRAM0__677: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__677: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__677: RWRegister<u32>,

    _reserved678: [u32; 1],

    /// Description Address Register
    pub SRAM0__678: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__678: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__678: RWRegister<u32>,

    _reserved679: [u32; 1],

    /// Description Address Register
    pub SRAM0__679: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__679: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__679: RWRegister<u32>,

    _reserved680: [u32; 1],

    /// Description Address Register
    pub SRAM0__680: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__680: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__680: RWRegister<u32>,

    _reserved681: [u32; 1],

    /// Description Address Register
    pub SRAM0__681: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__681: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__681: RWRegister<u32>,

    _reserved682: [u32; 1],

    /// Description Address Register
    pub SRAM0__682: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__682: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__682: RWRegister<u32>,

    _reserved683: [u32; 1],

    /// Description Address Register
    pub SRAM0__683: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__683: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__683: RWRegister<u32>,

    _reserved684: [u32; 1],

    /// Description Address Register
    pub SRAM0__684: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__684: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__684: RWRegister<u32>,

    _reserved685: [u32; 1],

    /// Description Address Register
    pub SRAM0__685: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__685: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__685: RWRegister<u32>,

    _reserved686: [u32; 1],

    /// Description Address Register
    pub SRAM0__686: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__686: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__686: RWRegister<u32>,

    _reserved687: [u32; 1],

    /// Description Address Register
    pub SRAM0__687: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__687: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__687: RWRegister<u32>,

    _reserved688: [u32; 1],

    /// Description Address Register
    pub SRAM0__688: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__688: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__688: RWRegister<u32>,

    _reserved689: [u32; 1],

    /// Description Address Register
    pub SRAM0__689: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__689: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__689: RWRegister<u32>,

    _reserved690: [u32; 1],

    /// Description Address Register
    pub SRAM0__690: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__690: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__690: RWRegister<u32>,

    _reserved691: [u32; 1],

    /// Description Address Register
    pub SRAM0__691: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__691: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__691: RWRegister<u32>,

    _reserved692: [u32; 1],

    /// Description Address Register
    pub SRAM0__692: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__692: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__692: RWRegister<u32>,

    _reserved693: [u32; 1],

    /// Description Address Register
    pub SRAM0__693: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__693: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__693: RWRegister<u32>,

    _reserved694: [u32; 1],

    /// Description Address Register
    pub SRAM0__694: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__694: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__694: RWRegister<u32>,

    _reserved695: [u32; 1],

    /// Description Address Register
    pub SRAM0__695: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__695: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__695: RWRegister<u32>,

    _reserved696: [u32; 1],

    /// Description Address Register
    pub SRAM0__696: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__696: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__696: RWRegister<u32>,

    _reserved697: [u32; 1],

    /// Description Address Register
    pub SRAM0__697: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__697: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__697: RWRegister<u32>,

    _reserved698: [u32; 1],

    /// Description Address Register
    pub SRAM0__698: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__698: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__698: RWRegister<u32>,

    _reserved699: [u32; 1],

    /// Description Address Register
    pub SRAM0__699: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__699: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__699: RWRegister<u32>,

    _reserved700: [u32; 1],

    /// Description Address Register
    pub SRAM0__700: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__700: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__700: RWRegister<u32>,

    _reserved701: [u32; 1],

    /// Description Address Register
    pub SRAM0__701: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__701: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__701: RWRegister<u32>,

    _reserved702: [u32; 1],

    /// Description Address Register
    pub SRAM0__702: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__702: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__702: RWRegister<u32>,

    _reserved703: [u32; 1],

    /// Description Address Register
    pub SRAM0__703: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__703: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__703: RWRegister<u32>,

    _reserved704: [u32; 1],

    /// Description Address Register
    pub SRAM0__704: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__704: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__704: RWRegister<u32>,

    _reserved705: [u32; 1],

    /// Description Address Register
    pub SRAM0__705: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__705: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__705: RWRegister<u32>,

    _reserved706: [u32; 1],

    /// Description Address Register
    pub SRAM0__706: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__706: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__706: RWRegister<u32>,

    _reserved707: [u32; 1],

    /// Description Address Register
    pub SRAM0__707: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__707: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__707: RWRegister<u32>,

    _reserved708: [u32; 1],

    /// Description Address Register
    pub SRAM0__708: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__708: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__708: RWRegister<u32>,

    _reserved709: [u32; 1],

    /// Description Address Register
    pub SRAM0__709: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__709: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__709: RWRegister<u32>,

    _reserved710: [u32; 1],

    /// Description Address Register
    pub SRAM0__710: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__710: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__710: RWRegister<u32>,

    _reserved711: [u32; 1],

    /// Description Address Register
    pub SRAM0__711: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__711: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__711: RWRegister<u32>,

    _reserved712: [u32; 1],

    /// Description Address Register
    pub SRAM0__712: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__712: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__712: RWRegister<u32>,

    _reserved713: [u32; 1],

    /// Description Address Register
    pub SRAM0__713: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__713: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__713: RWRegister<u32>,

    _reserved714: [u32; 1],

    /// Description Address Register
    pub SRAM0__714: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__714: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__714: RWRegister<u32>,

    _reserved715: [u32; 1],

    /// Description Address Register
    pub SRAM0__715: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__715: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__715: RWRegister<u32>,

    _reserved716: [u32; 1],

    /// Description Address Register
    pub SRAM0__716: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__716: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__716: RWRegister<u32>,

    _reserved717: [u32; 1],

    /// Description Address Register
    pub SRAM0__717: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__717: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__717: RWRegister<u32>,

    _reserved718: [u32; 1],

    /// Description Address Register
    pub SRAM0__718: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__718: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__718: RWRegister<u32>,

    _reserved719: [u32; 1],

    /// Description Address Register
    pub SRAM0__719: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__719: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__719: RWRegister<u32>,

    _reserved720: [u32; 1],

    /// Description Address Register
    pub SRAM0__720: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__720: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__720: RWRegister<u32>,

    _reserved721: [u32; 1],

    /// Description Address Register
    pub SRAM0__721: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__721: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__721: RWRegister<u32>,

    _reserved722: [u32; 1],

    /// Description Address Register
    pub SRAM0__722: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__722: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__722: RWRegister<u32>,

    _reserved723: [u32; 1],

    /// Description Address Register
    pub SRAM0__723: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__723: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__723: RWRegister<u32>,

    _reserved724: [u32; 1],

    /// Description Address Register
    pub SRAM0__724: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__724: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__724: RWRegister<u32>,

    _reserved725: [u32; 1],

    /// Description Address Register
    pub SRAM0__725: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__725: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__725: RWRegister<u32>,

    _reserved726: [u32; 1],

    /// Description Address Register
    pub SRAM0__726: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__726: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__726: RWRegister<u32>,

    _reserved727: [u32; 1],

    /// Description Address Register
    pub SRAM0__727: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__727: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__727: RWRegister<u32>,

    _reserved728: [u32; 1],

    /// Description Address Register
    pub SRAM0__728: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__728: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__728: RWRegister<u32>,

    _reserved729: [u32; 1],

    /// Description Address Register
    pub SRAM0__729: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__729: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__729: RWRegister<u32>,

    _reserved730: [u32; 1],

    /// Description Address Register
    pub SRAM0__730: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__730: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__730: RWRegister<u32>,

    _reserved731: [u32; 1],

    /// Description Address Register
    pub SRAM0__731: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__731: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__731: RWRegister<u32>,

    _reserved732: [u32; 1],

    /// Description Address Register
    pub SRAM0__732: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__732: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__732: RWRegister<u32>,

    _reserved733: [u32; 1],

    /// Description Address Register
    pub SRAM0__733: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__733: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__733: RWRegister<u32>,

    _reserved734: [u32; 1],

    /// Description Address Register
    pub SRAM0__734: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__734: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__734: RWRegister<u32>,

    _reserved735: [u32; 1],

    /// Description Address Register
    pub SRAM0__735: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__735: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__735: RWRegister<u32>,

    _reserved736: [u32; 1],

    /// Description Address Register
    pub SRAM0__736: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__736: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__736: RWRegister<u32>,

    _reserved737: [u32; 1],

    /// Description Address Register
    pub SRAM0__737: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__737: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__737: RWRegister<u32>,

    _reserved738: [u32; 1],

    /// Description Address Register
    pub SRAM0__738: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__738: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__738: RWRegister<u32>,

    _reserved739: [u32; 1],

    /// Description Address Register
    pub SRAM0__739: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__739: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__739: RWRegister<u32>,

    _reserved740: [u32; 1],

    /// Description Address Register
    pub SRAM0__740: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__740: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__740: RWRegister<u32>,

    _reserved741: [u32; 1],

    /// Description Address Register
    pub SRAM0__741: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__741: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__741: RWRegister<u32>,

    _reserved742: [u32; 1],

    /// Description Address Register
    pub SRAM0__742: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__742: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__742: RWRegister<u32>,

    _reserved743: [u32; 1],

    /// Description Address Register
    pub SRAM0__743: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__743: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__743: RWRegister<u32>,

    _reserved744: [u32; 1],

    /// Description Address Register
    pub SRAM0__744: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__744: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__744: RWRegister<u32>,

    _reserved745: [u32; 1],

    /// Description Address Register
    pub SRAM0__745: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__745: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__745: RWRegister<u32>,

    _reserved746: [u32; 1],

    /// Description Address Register
    pub SRAM0__746: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__746: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__746: RWRegister<u32>,

    _reserved747: [u32; 1],

    /// Description Address Register
    pub SRAM0__747: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__747: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__747: RWRegister<u32>,

    _reserved748: [u32; 1],

    /// Description Address Register
    pub SRAM0__748: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__748: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__748: RWRegister<u32>,

    _reserved749: [u32; 1],

    /// Description Address Register
    pub SRAM0__749: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__749: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__749: RWRegister<u32>,

    _reserved750: [u32; 1],

    /// Description Address Register
    pub SRAM0__750: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__750: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__750: RWRegister<u32>,

    _reserved751: [u32; 1],

    /// Description Address Register
    pub SRAM0__751: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__751: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__751: RWRegister<u32>,

    _reserved752: [u32; 1],

    /// Description Address Register
    pub SRAM0__752: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__752: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__752: RWRegister<u32>,

    _reserved753: [u32; 1],

    /// Description Address Register
    pub SRAM0__753: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__753: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__753: RWRegister<u32>,

    _reserved754: [u32; 1],

    /// Description Address Register
    pub SRAM0__754: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__754: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__754: RWRegister<u32>,

    _reserved755: [u32; 1],

    /// Description Address Register
    pub SRAM0__755: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__755: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__755: RWRegister<u32>,

    _reserved756: [u32; 1],

    /// Description Address Register
    pub SRAM0__756: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__756: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__756: RWRegister<u32>,

    _reserved757: [u32; 1],

    /// Description Address Register
    pub SRAM0__757: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__757: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__757: RWRegister<u32>,

    _reserved758: [u32; 1],

    /// Description Address Register
    pub SRAM0__758: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__758: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__758: RWRegister<u32>,

    _reserved759: [u32; 1],

    /// Description Address Register
    pub SRAM0__759: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__759: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__759: RWRegister<u32>,

    _reserved760: [u32; 1],

    /// Description Address Register
    pub SRAM0__760: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__760: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__760: RWRegister<u32>,

    _reserved761: [u32; 1],

    /// Description Address Register
    pub SRAM0__761: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__761: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__761: RWRegister<u32>,

    _reserved762: [u32; 1],

    /// Description Address Register
    pub SRAM0__762: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__762: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__762: RWRegister<u32>,

    _reserved763: [u32; 1],

    /// Description Address Register
    pub SRAM0__763: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__763: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__763: RWRegister<u32>,

    _reserved764: [u32; 1],

    /// Description Address Register
    pub SRAM0__764: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__764: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__764: RWRegister<u32>,

    _reserved765: [u32; 1],

    /// Description Address Register
    pub SRAM0__765: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__765: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__765: RWRegister<u32>,

    _reserved766: [u32; 1],

    /// Description Address Register
    pub SRAM0__766: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__766: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__766: RWRegister<u32>,

    _reserved767: [u32; 1],

    /// Description Address Register
    pub SRAM0__767: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__767: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__767: RWRegister<u32>,

    _reserved768: [u32; 1],

    /// Description Address Register
    pub SRAM0__768: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__768: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__768: RWRegister<u32>,

    _reserved769: [u32; 1],

    /// Description Address Register
    pub SRAM0__769: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__769: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__769: RWRegister<u32>,

    _reserved770: [u32; 1],

    /// Description Address Register
    pub SRAM0__770: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__770: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__770: RWRegister<u32>,

    _reserved771: [u32; 1],

    /// Description Address Register
    pub SRAM0__771: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__771: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__771: RWRegister<u32>,

    _reserved772: [u32; 1],

    /// Description Address Register
    pub SRAM0__772: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__772: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__772: RWRegister<u32>,

    _reserved773: [u32; 1],

    /// Description Address Register
    pub SRAM0__773: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__773: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__773: RWRegister<u32>,

    _reserved774: [u32; 1],

    /// Description Address Register
    pub SRAM0__774: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__774: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__774: RWRegister<u32>,

    _reserved775: [u32; 1],

    /// Description Address Register
    pub SRAM0__775: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__775: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__775: RWRegister<u32>,

    _reserved776: [u32; 1],

    /// Description Address Register
    pub SRAM0__776: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__776: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__776: RWRegister<u32>,

    _reserved777: [u32; 1],

    /// Description Address Register
    pub SRAM0__777: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__777: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__777: RWRegister<u32>,

    _reserved778: [u32; 1],

    /// Description Address Register
    pub SRAM0__778: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__778: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__778: RWRegister<u32>,

    _reserved779: [u32; 1],

    /// Description Address Register
    pub SRAM0__779: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__779: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__779: RWRegister<u32>,

    _reserved780: [u32; 1],

    /// Description Address Register
    pub SRAM0__780: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__780: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__780: RWRegister<u32>,

    _reserved781: [u32; 1],

    /// Description Address Register
    pub SRAM0__781: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__781: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__781: RWRegister<u32>,

    _reserved782: [u32; 1],

    /// Description Address Register
    pub SRAM0__782: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__782: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__782: RWRegister<u32>,

    _reserved783: [u32; 1],

    /// Description Address Register
    pub SRAM0__783: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__783: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__783: RWRegister<u32>,

    _reserved784: [u32; 1],

    /// Description Address Register
    pub SRAM0__784: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__784: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__784: RWRegister<u32>,

    _reserved785: [u32; 1],

    /// Description Address Register
    pub SRAM0__785: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__785: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__785: RWRegister<u32>,

    _reserved786: [u32; 1],

    /// Description Address Register
    pub SRAM0__786: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__786: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__786: RWRegister<u32>,

    _reserved787: [u32; 1],

    /// Description Address Register
    pub SRAM0__787: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__787: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__787: RWRegister<u32>,

    _reserved788: [u32; 1],

    /// Description Address Register
    pub SRAM0__788: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__788: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__788: RWRegister<u32>,

    _reserved789: [u32; 1],

    /// Description Address Register
    pub SRAM0__789: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__789: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__789: RWRegister<u32>,

    _reserved790: [u32; 1],

    /// Description Address Register
    pub SRAM0__790: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__790: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__790: RWRegister<u32>,

    _reserved791: [u32; 1],

    /// Description Address Register
    pub SRAM0__791: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__791: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__791: RWRegister<u32>,

    _reserved792: [u32; 1],

    /// Description Address Register
    pub SRAM0__792: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__792: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__792: RWRegister<u32>,

    _reserved793: [u32; 1],

    /// Description Address Register
    pub SRAM0__793: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__793: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__793: RWRegister<u32>,

    _reserved794: [u32; 1],

    /// Description Address Register
    pub SRAM0__794: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__794: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__794: RWRegister<u32>,

    _reserved795: [u32; 1],

    /// Description Address Register
    pub SRAM0__795: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__795: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__795: RWRegister<u32>,

    _reserved796: [u32; 1],

    /// Description Address Register
    pub SRAM0__796: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__796: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__796: RWRegister<u32>,

    _reserved797: [u32; 1],

    /// Description Address Register
    pub SRAM0__797: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__797: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__797: RWRegister<u32>,

    _reserved798: [u32; 1],

    /// Description Address Register
    pub SRAM0__798: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__798: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__798: RWRegister<u32>,

    _reserved799: [u32; 1],

    /// Description Address Register
    pub SRAM0__799: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__799: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__799: RWRegister<u32>,

    _reserved800: [u32; 1],

    /// Description Address Register
    pub SRAM0__800: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__800: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__800: RWRegister<u32>,

    _reserved801: [u32; 1],

    /// Description Address Register
    pub SRAM0__801: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__801: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__801: RWRegister<u32>,

    _reserved802: [u32; 1],

    /// Description Address Register
    pub SRAM0__802: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__802: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__802: RWRegister<u32>,

    _reserved803: [u32; 1],

    /// Description Address Register
    pub SRAM0__803: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__803: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__803: RWRegister<u32>,

    _reserved804: [u32; 1],

    /// Description Address Register
    pub SRAM0__804: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__804: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__804: RWRegister<u32>,

    _reserved805: [u32; 1],

    /// Description Address Register
    pub SRAM0__805: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__805: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__805: RWRegister<u32>,

    _reserved806: [u32; 1],

    /// Description Address Register
    pub SRAM0__806: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__806: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__806: RWRegister<u32>,

    _reserved807: [u32; 1],

    /// Description Address Register
    pub SRAM0__807: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__807: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__807: RWRegister<u32>,

    _reserved808: [u32; 1],

    /// Description Address Register
    pub SRAM0__808: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__808: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__808: RWRegister<u32>,

    _reserved809: [u32; 1],

    /// Description Address Register
    pub SRAM0__809: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__809: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__809: RWRegister<u32>,

    _reserved810: [u32; 1],

    /// Description Address Register
    pub SRAM0__810: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__810: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__810: RWRegister<u32>,

    _reserved811: [u32; 1],

    /// Description Address Register
    pub SRAM0__811: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__811: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__811: RWRegister<u32>,

    _reserved812: [u32; 1],

    /// Description Address Register
    pub SRAM0__812: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__812: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__812: RWRegister<u32>,

    _reserved813: [u32; 1],

    /// Description Address Register
    pub SRAM0__813: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__813: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__813: RWRegister<u32>,

    _reserved814: [u32; 1],

    /// Description Address Register
    pub SRAM0__814: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__814: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__814: RWRegister<u32>,

    _reserved815: [u32; 1],

    /// Description Address Register
    pub SRAM0__815: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__815: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__815: RWRegister<u32>,

    _reserved816: [u32; 1],

    /// Description Address Register
    pub SRAM0__816: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__816: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__816: RWRegister<u32>,

    _reserved817: [u32; 1],

    /// Description Address Register
    pub SRAM0__817: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__817: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__817: RWRegister<u32>,

    _reserved818: [u32; 1],

    /// Description Address Register
    pub SRAM0__818: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__818: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__818: RWRegister<u32>,

    _reserved819: [u32; 1],

    /// Description Address Register
    pub SRAM0__819: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__819: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__819: RWRegister<u32>,

    _reserved820: [u32; 1],

    /// Description Address Register
    pub SRAM0__820: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__820: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__820: RWRegister<u32>,

    _reserved821: [u32; 1],

    /// Description Address Register
    pub SRAM0__821: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__821: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__821: RWRegister<u32>,

    _reserved822: [u32; 1],

    /// Description Address Register
    pub SRAM0__822: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__822: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__822: RWRegister<u32>,

    _reserved823: [u32; 1],

    /// Description Address Register
    pub SRAM0__823: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__823: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__823: RWRegister<u32>,

    _reserved824: [u32; 1],

    /// Description Address Register
    pub SRAM0__824: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__824: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__824: RWRegister<u32>,

    _reserved825: [u32; 1],

    /// Description Address Register
    pub SRAM0__825: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__825: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__825: RWRegister<u32>,

    _reserved826: [u32; 1],

    /// Description Address Register
    pub SRAM0__826: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__826: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__826: RWRegister<u32>,

    _reserved827: [u32; 1],

    /// Description Address Register
    pub SRAM0__827: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__827: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__827: RWRegister<u32>,

    _reserved828: [u32; 1],

    /// Description Address Register
    pub SRAM0__828: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__828: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__828: RWRegister<u32>,

    _reserved829: [u32; 1],

    /// Description Address Register
    pub SRAM0__829: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__829: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__829: RWRegister<u32>,

    _reserved830: [u32; 1],

    /// Description Address Register
    pub SRAM0__830: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__830: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__830: RWRegister<u32>,

    _reserved831: [u32; 1],

    /// Description Address Register
    pub SRAM0__831: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__831: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__831: RWRegister<u32>,

    _reserved832: [u32; 1],

    /// Description Address Register
    pub SRAM0__832: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__832: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__832: RWRegister<u32>,

    _reserved833: [u32; 1],

    /// Description Address Register
    pub SRAM0__833: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__833: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__833: RWRegister<u32>,

    _reserved834: [u32; 1],

    /// Description Address Register
    pub SRAM0__834: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__834: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__834: RWRegister<u32>,

    _reserved835: [u32; 1],

    /// Description Address Register
    pub SRAM0__835: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__835: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__835: RWRegister<u32>,

    _reserved836: [u32; 1],

    /// Description Address Register
    pub SRAM0__836: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__836: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__836: RWRegister<u32>,

    _reserved837: [u32; 1],

    /// Description Address Register
    pub SRAM0__837: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__837: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__837: RWRegister<u32>,

    _reserved838: [u32; 1],

    /// Description Address Register
    pub SRAM0__838: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__838: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__838: RWRegister<u32>,

    _reserved839: [u32; 1],

    /// Description Address Register
    pub SRAM0__839: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__839: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__839: RWRegister<u32>,

    _reserved840: [u32; 1],

    /// Description Address Register
    pub SRAM0__840: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__840: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__840: RWRegister<u32>,

    _reserved841: [u32; 1],

    /// Description Address Register
    pub SRAM0__841: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__841: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__841: RWRegister<u32>,

    _reserved842: [u32; 1],

    /// Description Address Register
    pub SRAM0__842: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__842: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__842: RWRegister<u32>,

    _reserved843: [u32; 1],

    /// Description Address Register
    pub SRAM0__843: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__843: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__843: RWRegister<u32>,

    _reserved844: [u32; 1],

    /// Description Address Register
    pub SRAM0__844: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__844: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__844: RWRegister<u32>,

    _reserved845: [u32; 1],

    /// Description Address Register
    pub SRAM0__845: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__845: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__845: RWRegister<u32>,

    _reserved846: [u32; 1],

    /// Description Address Register
    pub SRAM0__846: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__846: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__846: RWRegister<u32>,

    _reserved847: [u32; 1],

    /// Description Address Register
    pub SRAM0__847: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__847: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__847: RWRegister<u32>,

    _reserved848: [u32; 1],

    /// Description Address Register
    pub SRAM0__848: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__848: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__848: RWRegister<u32>,

    _reserved849: [u32; 1],

    /// Description Address Register
    pub SRAM0__849: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__849: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__849: RWRegister<u32>,

    _reserved850: [u32; 1],

    /// Description Address Register
    pub SRAM0__850: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__850: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__850: RWRegister<u32>,

    _reserved851: [u32; 1],

    /// Description Address Register
    pub SRAM0__851: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__851: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__851: RWRegister<u32>,

    _reserved852: [u32; 1],

    /// Description Address Register
    pub SRAM0__852: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__852: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__852: RWRegister<u32>,

    _reserved853: [u32; 1],

    /// Description Address Register
    pub SRAM0__853: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__853: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__853: RWRegister<u32>,

    _reserved854: [u32; 1],

    /// Description Address Register
    pub SRAM0__854: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__854: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__854: RWRegister<u32>,

    _reserved855: [u32; 1],

    /// Description Address Register
    pub SRAM0__855: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__855: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__855: RWRegister<u32>,

    _reserved856: [u32; 1],

    /// Description Address Register
    pub SRAM0__856: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__856: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__856: RWRegister<u32>,

    _reserved857: [u32; 1],

    /// Description Address Register
    pub SRAM0__857: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__857: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__857: RWRegister<u32>,

    _reserved858: [u32; 1],

    /// Description Address Register
    pub SRAM0__858: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__858: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__858: RWRegister<u32>,

    _reserved859: [u32; 1],

    /// Description Address Register
    pub SRAM0__859: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__859: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__859: RWRegister<u32>,

    _reserved860: [u32; 1],

    /// Description Address Register
    pub SRAM0__860: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__860: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__860: RWRegister<u32>,

    _reserved861: [u32; 1],

    /// Description Address Register
    pub SRAM0__861: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__861: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__861: RWRegister<u32>,

    _reserved862: [u32; 1],

    /// Description Address Register
    pub SRAM0__862: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__862: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__862: RWRegister<u32>,

    _reserved863: [u32; 1],

    /// Description Address Register
    pub SRAM0__863: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__863: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__863: RWRegister<u32>,

    _reserved864: [u32; 1],

    /// Description Address Register
    pub SRAM0__864: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__864: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__864: RWRegister<u32>,

    _reserved865: [u32; 1],

    /// Description Address Register
    pub SRAM0__865: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__865: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__865: RWRegister<u32>,

    _reserved866: [u32; 1],

    /// Description Address Register
    pub SRAM0__866: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__866: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__866: RWRegister<u32>,

    _reserved867: [u32; 1],

    /// Description Address Register
    pub SRAM0__867: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__867: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__867: RWRegister<u32>,

    _reserved868: [u32; 1],

    /// Description Address Register
    pub SRAM0__868: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__868: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__868: RWRegister<u32>,

    _reserved869: [u32; 1],

    /// Description Address Register
    pub SRAM0__869: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__869: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__869: RWRegister<u32>,

    _reserved870: [u32; 1],

    /// Description Address Register
    pub SRAM0__870: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__870: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__870: RWRegister<u32>,

    _reserved871: [u32; 1],

    /// Description Address Register
    pub SRAM0__871: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__871: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__871: RWRegister<u32>,

    _reserved872: [u32; 1],

    /// Description Address Register
    pub SRAM0__872: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__872: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__872: RWRegister<u32>,

    _reserved873: [u32; 1],

    /// Description Address Register
    pub SRAM0__873: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__873: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__873: RWRegister<u32>,

    _reserved874: [u32; 1],

    /// Description Address Register
    pub SRAM0__874: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__874: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__874: RWRegister<u32>,

    _reserved875: [u32; 1],

    /// Description Address Register
    pub SRAM0__875: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__875: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__875: RWRegister<u32>,

    _reserved876: [u32; 1],

    /// Description Address Register
    pub SRAM0__876: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__876: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__876: RWRegister<u32>,

    _reserved877: [u32; 1],

    /// Description Address Register
    pub SRAM0__877: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__877: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__877: RWRegister<u32>,

    _reserved878: [u32; 1],

    /// Description Address Register
    pub SRAM0__878: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__878: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__878: RWRegister<u32>,

    _reserved879: [u32; 1],

    /// Description Address Register
    pub SRAM0__879: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__879: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__879: RWRegister<u32>,

    _reserved880: [u32; 1],

    /// Description Address Register
    pub SRAM0__880: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__880: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__880: RWRegister<u32>,

    _reserved881: [u32; 1],

    /// Description Address Register
    pub SRAM0__881: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__881: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__881: RWRegister<u32>,

    _reserved882: [u32; 1],

    /// Description Address Register
    pub SRAM0__882: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__882: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__882: RWRegister<u32>,

    _reserved883: [u32; 1],

    /// Description Address Register
    pub SRAM0__883: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__883: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__883: RWRegister<u32>,

    _reserved884: [u32; 1],

    /// Description Address Register
    pub SRAM0__884: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__884: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__884: RWRegister<u32>,

    _reserved885: [u32; 1],

    /// Description Address Register
    pub SRAM0__885: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__885: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__885: RWRegister<u32>,

    _reserved886: [u32; 1],

    /// Description Address Register
    pub SRAM0__886: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__886: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__886: RWRegister<u32>,

    _reserved887: [u32; 1],

    /// Description Address Register
    pub SRAM0__887: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__887: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__887: RWRegister<u32>,

    _reserved888: [u32; 1],

    /// Description Address Register
    pub SRAM0__888: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__888: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__888: RWRegister<u32>,

    _reserved889: [u32; 1],

    /// Description Address Register
    pub SRAM0__889: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__889: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__889: RWRegister<u32>,

    _reserved890: [u32; 1],

    /// Description Address Register
    pub SRAM0__890: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__890: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__890: RWRegister<u32>,

    _reserved891: [u32; 1],

    /// Description Address Register
    pub SRAM0__891: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__891: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__891: RWRegister<u32>,

    _reserved892: [u32; 1],

    /// Description Address Register
    pub SRAM0__892: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__892: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__892: RWRegister<u32>,

    _reserved893: [u32; 1],

    /// Description Address Register
    pub SRAM0__893: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__893: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__893: RWRegister<u32>,

    _reserved894: [u32; 1],

    /// Description Address Register
    pub SRAM0__894: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__894: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__894: RWRegister<u32>,

    _reserved895: [u32; 1],

    /// Description Address Register
    pub SRAM0__895: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__895: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__895: RWRegister<u32>,

    _reserved896: [u32; 1],

    /// Description Address Register
    pub SRAM0__896: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__896: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__896: RWRegister<u32>,

    _reserved897: [u32; 1],

    /// Description Address Register
    pub SRAM0__897: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__897: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__897: RWRegister<u32>,

    _reserved898: [u32; 1],

    /// Description Address Register
    pub SRAM0__898: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__898: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__898: RWRegister<u32>,

    _reserved899: [u32; 1],

    /// Description Address Register
    pub SRAM0__899: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__899: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__899: RWRegister<u32>,

    _reserved900: [u32; 1],

    /// Description Address Register
    pub SRAM0__900: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__900: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__900: RWRegister<u32>,

    _reserved901: [u32; 1],

    /// Description Address Register
    pub SRAM0__901: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__901: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__901: RWRegister<u32>,

    _reserved902: [u32; 1],

    /// Description Address Register
    pub SRAM0__902: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__902: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__902: RWRegister<u32>,

    _reserved903: [u32; 1],

    /// Description Address Register
    pub SRAM0__903: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__903: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__903: RWRegister<u32>,

    _reserved904: [u32; 1],

    /// Description Address Register
    pub SRAM0__904: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__904: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__904: RWRegister<u32>,

    _reserved905: [u32; 1],

    /// Description Address Register
    pub SRAM0__905: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__905: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__905: RWRegister<u32>,

    _reserved906: [u32; 1],

    /// Description Address Register
    pub SRAM0__906: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__906: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__906: RWRegister<u32>,

    _reserved907: [u32; 1],

    /// Description Address Register
    pub SRAM0__907: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__907: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__907: RWRegister<u32>,

    _reserved908: [u32; 1],

    /// Description Address Register
    pub SRAM0__908: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__908: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__908: RWRegister<u32>,

    _reserved909: [u32; 1],

    /// Description Address Register
    pub SRAM0__909: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__909: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__909: RWRegister<u32>,

    _reserved910: [u32; 1],

    /// Description Address Register
    pub SRAM0__910: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__910: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__910: RWRegister<u32>,

    _reserved911: [u32; 1],

    /// Description Address Register
    pub SRAM0__911: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__911: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__911: RWRegister<u32>,

    _reserved912: [u32; 1],

    /// Description Address Register
    pub SRAM0__912: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__912: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__912: RWRegister<u32>,

    _reserved913: [u32; 1],

    /// Description Address Register
    pub SRAM0__913: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__913: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__913: RWRegister<u32>,

    _reserved914: [u32; 1],

    /// Description Address Register
    pub SRAM0__914: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__914: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__914: RWRegister<u32>,

    _reserved915: [u32; 1],

    /// Description Address Register
    pub SRAM0__915: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__915: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__915: RWRegister<u32>,

    _reserved916: [u32; 1],

    /// Description Address Register
    pub SRAM0__916: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__916: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__916: RWRegister<u32>,

    _reserved917: [u32; 1],

    /// Description Address Register
    pub SRAM0__917: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__917: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__917: RWRegister<u32>,

    _reserved918: [u32; 1],

    /// Description Address Register
    pub SRAM0__918: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__918: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__918: RWRegister<u32>,

    _reserved919: [u32; 1],

    /// Description Address Register
    pub SRAM0__919: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__919: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__919: RWRegister<u32>,

    _reserved920: [u32; 1],

    /// Description Address Register
    pub SRAM0__920: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__920: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__920: RWRegister<u32>,

    _reserved921: [u32; 1],

    /// Description Address Register
    pub SRAM0__921: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__921: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__921: RWRegister<u32>,

    _reserved922: [u32; 1],

    /// Description Address Register
    pub SRAM0__922: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__922: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__922: RWRegister<u32>,

    _reserved923: [u32; 1],

    /// Description Address Register
    pub SRAM0__923: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__923: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__923: RWRegister<u32>,

    _reserved924: [u32; 1],

    /// Description Address Register
    pub SRAM0__924: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__924: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__924: RWRegister<u32>,

    _reserved925: [u32; 1],

    /// Description Address Register
    pub SRAM0__925: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__925: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__925: RWRegister<u32>,

    _reserved926: [u32; 1],

    /// Description Address Register
    pub SRAM0__926: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__926: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__926: RWRegister<u32>,

    _reserved927: [u32; 1],

    /// Description Address Register
    pub SRAM0__927: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__927: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__927: RWRegister<u32>,

    _reserved928: [u32; 1],

    /// Description Address Register
    pub SRAM0__928: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__928: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__928: RWRegister<u32>,

    _reserved929: [u32; 1],

    /// Description Address Register
    pub SRAM0__929: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__929: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__929: RWRegister<u32>,

    _reserved930: [u32; 1],

    /// Description Address Register
    pub SRAM0__930: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__930: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__930: RWRegister<u32>,

    _reserved931: [u32; 1],

    /// Description Address Register
    pub SRAM0__931: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__931: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__931: RWRegister<u32>,

    _reserved932: [u32; 1],

    /// Description Address Register
    pub SRAM0__932: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__932: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__932: RWRegister<u32>,

    _reserved933: [u32; 1],

    /// Description Address Register
    pub SRAM0__933: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__933: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__933: RWRegister<u32>,

    _reserved934: [u32; 1],

    /// Description Address Register
    pub SRAM0__934: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__934: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__934: RWRegister<u32>,

    _reserved935: [u32; 1],

    /// Description Address Register
    pub SRAM0__935: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__935: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__935: RWRegister<u32>,

    _reserved936: [u32; 1],

    /// Description Address Register
    pub SRAM0__936: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__936: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__936: RWRegister<u32>,

    _reserved937: [u32; 1],

    /// Description Address Register
    pub SRAM0__937: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__937: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__937: RWRegister<u32>,

    _reserved938: [u32; 1],

    /// Description Address Register
    pub SRAM0__938: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__938: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__938: RWRegister<u32>,

    _reserved939: [u32; 1],

    /// Description Address Register
    pub SRAM0__939: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__939: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__939: RWRegister<u32>,

    _reserved940: [u32; 1],

    /// Description Address Register
    pub SRAM0__940: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__940: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__940: RWRegister<u32>,

    _reserved941: [u32; 1],

    /// Description Address Register
    pub SRAM0__941: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__941: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__941: RWRegister<u32>,

    _reserved942: [u32; 1],

    /// Description Address Register
    pub SRAM0__942: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__942: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__942: RWRegister<u32>,

    _reserved943: [u32; 1],

    /// Description Address Register
    pub SRAM0__943: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__943: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__943: RWRegister<u32>,

    _reserved944: [u32; 1],

    /// Description Address Register
    pub SRAM0__944: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__944: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__944: RWRegister<u32>,

    _reserved945: [u32; 1],

    /// Description Address Register
    pub SRAM0__945: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__945: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__945: RWRegister<u32>,

    _reserved946: [u32; 1],

    /// Description Address Register
    pub SRAM0__946: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__946: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__946: RWRegister<u32>,

    _reserved947: [u32; 1],

    /// Description Address Register
    pub SRAM0__947: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__947: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__947: RWRegister<u32>,

    _reserved948: [u32; 1],

    /// Description Address Register
    pub SRAM0__948: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__948: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__948: RWRegister<u32>,

    _reserved949: [u32; 1],

    /// Description Address Register
    pub SRAM0__949: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__949: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__949: RWRegister<u32>,

    _reserved950: [u32; 1],

    /// Description Address Register
    pub SRAM0__950: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__950: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__950: RWRegister<u32>,

    _reserved951: [u32; 1],

    /// Description Address Register
    pub SRAM0__951: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__951: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__951: RWRegister<u32>,

    _reserved952: [u32; 1],

    /// Description Address Register
    pub SRAM0__952: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__952: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__952: RWRegister<u32>,

    _reserved953: [u32; 1],

    /// Description Address Register
    pub SRAM0__953: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__953: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__953: RWRegister<u32>,

    _reserved954: [u32; 1],

    /// Description Address Register
    pub SRAM0__954: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__954: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__954: RWRegister<u32>,

    _reserved955: [u32; 1],

    /// Description Address Register
    pub SRAM0__955: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__955: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__955: RWRegister<u32>,

    _reserved956: [u32; 1],

    /// Description Address Register
    pub SRAM0__956: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__956: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__956: RWRegister<u32>,

    _reserved957: [u32; 1],

    /// Description Address Register
    pub SRAM0__957: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__957: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__957: RWRegister<u32>,

    _reserved958: [u32; 1],

    /// Description Address Register
    pub SRAM0__958: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__958: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__958: RWRegister<u32>,

    _reserved959: [u32; 1],

    /// Description Address Register
    pub SRAM0__959: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__959: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__959: RWRegister<u32>,

    _reserved960: [u32; 1],

    /// Description Address Register
    pub SRAM0__960: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__960: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__960: RWRegister<u32>,

    _reserved961: [u32; 1],

    /// Description Address Register
    pub SRAM0__961: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__961: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__961: RWRegister<u32>,

    _reserved962: [u32; 1],

    /// Description Address Register
    pub SRAM0__962: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__962: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__962: RWRegister<u32>,

    _reserved963: [u32; 1],

    /// Description Address Register
    pub SRAM0__963: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__963: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__963: RWRegister<u32>,

    _reserved964: [u32; 1],

    /// Description Address Register
    pub SRAM0__964: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__964: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__964: RWRegister<u32>,

    _reserved965: [u32; 1],

    /// Description Address Register
    pub SRAM0__965: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__965: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__965: RWRegister<u32>,

    _reserved966: [u32; 1],

    /// Description Address Register
    pub SRAM0__966: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__966: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__966: RWRegister<u32>,

    _reserved967: [u32; 1],

    /// Description Address Register
    pub SRAM0__967: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__967: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__967: RWRegister<u32>,

    _reserved968: [u32; 1],

    /// Description Address Register
    pub SRAM0__968: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__968: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__968: RWRegister<u32>,

    _reserved969: [u32; 1],

    /// Description Address Register
    pub SRAM0__969: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__969: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__969: RWRegister<u32>,

    _reserved970: [u32; 1],

    /// Description Address Register
    pub SRAM0__970: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__970: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__970: RWRegister<u32>,

    _reserved971: [u32; 1],

    /// Description Address Register
    pub SRAM0__971: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__971: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__971: RWRegister<u32>,

    _reserved972: [u32; 1],

    /// Description Address Register
    pub SRAM0__972: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__972: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__972: RWRegister<u32>,

    _reserved973: [u32; 1],

    /// Description Address Register
    pub SRAM0__973: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__973: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__973: RWRegister<u32>,

    _reserved974: [u32; 1],

    /// Description Address Register
    pub SRAM0__974: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__974: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__974: RWRegister<u32>,

    _reserved975: [u32; 1],

    /// Description Address Register
    pub SRAM0__975: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__975: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__975: RWRegister<u32>,

    _reserved976: [u32; 1],

    /// Description Address Register
    pub SRAM0__976: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__976: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__976: RWRegister<u32>,

    _reserved977: [u32; 1],

    /// Description Address Register
    pub SRAM0__977: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__977: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__977: RWRegister<u32>,

    _reserved978: [u32; 1],

    /// Description Address Register
    pub SRAM0__978: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__978: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__978: RWRegister<u32>,

    _reserved979: [u32; 1],

    /// Description Address Register
    pub SRAM0__979: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__979: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__979: RWRegister<u32>,

    _reserved980: [u32; 1],

    /// Description Address Register
    pub SRAM0__980: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__980: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__980: RWRegister<u32>,

    _reserved981: [u32; 1],

    /// Description Address Register
    pub SRAM0__981: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__981: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__981: RWRegister<u32>,

    _reserved982: [u32; 1],

    /// Description Address Register
    pub SRAM0__982: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__982: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__982: RWRegister<u32>,

    _reserved983: [u32; 1],

    /// Description Address Register
    pub SRAM0__983: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__983: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__983: RWRegister<u32>,

    _reserved984: [u32; 1],

    /// Description Address Register
    pub SRAM0__984: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__984: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__984: RWRegister<u32>,

    _reserved985: [u32; 1],

    /// Description Address Register
    pub SRAM0__985: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__985: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__985: RWRegister<u32>,

    _reserved986: [u32; 1],

    /// Description Address Register
    pub SRAM0__986: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__986: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__986: RWRegister<u32>,

    _reserved987: [u32; 1],

    /// Description Address Register
    pub SRAM0__987: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__987: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__987: RWRegister<u32>,

    _reserved988: [u32; 1],

    /// Description Address Register
    pub SRAM0__988: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__988: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__988: RWRegister<u32>,

    _reserved989: [u32; 1],

    /// Description Address Register
    pub SRAM0__989: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__989: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__989: RWRegister<u32>,

    _reserved990: [u32; 1],

    /// Description Address Register
    pub SRAM0__990: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__990: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__990: RWRegister<u32>,

    _reserved991: [u32; 1],

    /// Description Address Register
    pub SRAM0__991: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__991: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__991: RWRegister<u32>,

    _reserved992: [u32; 1],

    /// Description Address Register
    pub SRAM0__992: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__992: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__992: RWRegister<u32>,

    _reserved993: [u32; 1],

    /// Description Address Register
    pub SRAM0__993: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__993: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__993: RWRegister<u32>,

    _reserved994: [u32; 1],

    /// Description Address Register
    pub SRAM0__994: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__994: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__994: RWRegister<u32>,

    _reserved995: [u32; 1],

    /// Description Address Register
    pub SRAM0__995: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__995: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__995: RWRegister<u32>,

    _reserved996: [u32; 1],

    /// Description Address Register
    pub SRAM0__996: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__996: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__996: RWRegister<u32>,

    _reserved997: [u32; 1],

    /// Description Address Register
    pub SRAM0__997: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__997: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__997: RWRegister<u32>,

    _reserved998: [u32; 1],

    /// Description Address Register
    pub SRAM0__998: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__998: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__998: RWRegister<u32>,

    _reserved999: [u32; 1],

    /// Description Address Register
    pub SRAM0__999: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__999: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__999: RWRegister<u32>,

    _reserved1000: [u32; 1],

    /// Description Address Register
    pub SRAM0__1000: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1000: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1000: RWRegister<u32>,

    _reserved1001: [u32; 1],

    /// Description Address Register
    pub SRAM0__1001: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1001: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1001: RWRegister<u32>,

    _reserved1002: [u32; 1],

    /// Description Address Register
    pub SRAM0__1002: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1002: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1002: RWRegister<u32>,

    _reserved1003: [u32; 1],

    /// Description Address Register
    pub SRAM0__1003: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1003: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1003: RWRegister<u32>,

    _reserved1004: [u32; 1],

    /// Description Address Register
    pub SRAM0__1004: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1004: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1004: RWRegister<u32>,

    _reserved1005: [u32; 1],

    /// Description Address Register
    pub SRAM0__1005: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1005: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1005: RWRegister<u32>,

    _reserved1006: [u32; 1],

    /// Description Address Register
    pub SRAM0__1006: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1006: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1006: RWRegister<u32>,

    _reserved1007: [u32; 1],

    /// Description Address Register
    pub SRAM0__1007: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1007: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1007: RWRegister<u32>,

    _reserved1008: [u32; 1],

    /// Description Address Register
    pub SRAM0__1008: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1008: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1008: RWRegister<u32>,

    _reserved1009: [u32; 1],

    /// Description Address Register
    pub SRAM0__1009: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1009: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1009: RWRegister<u32>,

    _reserved1010: [u32; 1],

    /// Description Address Register
    pub SRAM0__1010: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1010: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1010: RWRegister<u32>,

    _reserved1011: [u32; 1],

    /// Description Address Register
    pub SRAM0__1011: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1011: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1011: RWRegister<u32>,

    _reserved1012: [u32; 1],

    /// Description Address Register
    pub SRAM0__1012: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1012: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1012: RWRegister<u32>,

    _reserved1013: [u32; 1],

    /// Description Address Register
    pub SRAM0__1013: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1013: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1013: RWRegister<u32>,

    _reserved1014: [u32; 1],

    /// Description Address Register
    pub SRAM0__1014: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1014: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1014: RWRegister<u32>,

    _reserved1015: [u32; 1],

    /// Description Address Register
    pub SRAM0__1015: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1015: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1015: RWRegister<u32>,

    _reserved1016: [u32; 1],

    /// Description Address Register
    pub SRAM0__1016: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1016: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1016: RWRegister<u32>,

    _reserved1017: [u32; 1],

    /// Description Address Register
    pub SRAM0__1017: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1017: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1017: RWRegister<u32>,

    _reserved1018: [u32; 1],

    /// Description Address Register
    pub SRAM0__1018: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1018: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1018: RWRegister<u32>,

    _reserved1019: [u32; 1],

    /// Description Address Register
    pub SRAM0__1019: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1019: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1019: RWRegister<u32>,

    _reserved1020: [u32; 1],

    /// Description Address Register
    pub SRAM0__1020: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1020: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1020: RWRegister<u32>,

    _reserved1021: [u32; 1],

    /// Description Address Register
    pub SRAM0__1021: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1021: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1021: RWRegister<u32>,

    _reserved1022: [u32; 1],

    /// Description Address Register
    pub SRAM0__1022: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1022: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1022: RWRegister<u32>,

    _reserved1023: [u32; 1],

    /// Description Address Register
    pub SRAM0__1023: RWRegister<u32>,

    /// Description Data Register
    pub SRAM1__1023: RWRegister<u32>,

    /// Description Control Register
    pub SRAM2__1023: RWRegister<u32>,
}
pub struct ResetValues {
    pub SRAM0__0: u32,
    pub SRAM1__0: u32,
    pub SRAM2__0: u32,
    pub SRAM0__1: u32,
    pub SRAM1__1: u32,
    pub SRAM2__1: u32,
    pub SRAM0__2: u32,
    pub SRAM1__2: u32,
    pub SRAM2__2: u32,
    pub SRAM0__3: u32,
    pub SRAM1__3: u32,
    pub SRAM2__3: u32,
    pub SRAM0__4: u32,
    pub SRAM1__4: u32,
    pub SRAM2__4: u32,
    pub SRAM0__5: u32,
    pub SRAM1__5: u32,
    pub SRAM2__5: u32,
    pub SRAM0__6: u32,
    pub SRAM1__6: u32,
    pub SRAM2__6: u32,
    pub SRAM0__7: u32,
    pub SRAM1__7: u32,
    pub SRAM2__7: u32,
    pub SRAM0__8: u32,
    pub SRAM1__8: u32,
    pub SRAM2__8: u32,
    pub SRAM0__9: u32,
    pub SRAM1__9: u32,
    pub SRAM2__9: u32,
    pub SRAM0__10: u32,
    pub SRAM1__10: u32,
    pub SRAM2__10: u32,
    pub SRAM0__11: u32,
    pub SRAM1__11: u32,
    pub SRAM2__11: u32,
    pub SRAM0__12: u32,
    pub SRAM1__12: u32,
    pub SRAM2__12: u32,
    pub SRAM0__13: u32,
    pub SRAM1__13: u32,
    pub SRAM2__13: u32,
    pub SRAM0__14: u32,
    pub SRAM1__14: u32,
    pub SRAM2__14: u32,
    pub SRAM0__15: u32,
    pub SRAM1__15: u32,
    pub SRAM2__15: u32,
    pub SRAM0__16: u32,
    pub SRAM1__16: u32,
    pub SRAM2__16: u32,
    pub SRAM0__17: u32,
    pub SRAM1__17: u32,
    pub SRAM2__17: u32,
    pub SRAM0__18: u32,
    pub SRAM1__18: u32,
    pub SRAM2__18: u32,
    pub SRAM0__19: u32,
    pub SRAM1__19: u32,
    pub SRAM2__19: u32,
    pub SRAM0__20: u32,
    pub SRAM1__20: u32,
    pub SRAM2__20: u32,
    pub SRAM0__21: u32,
    pub SRAM1__21: u32,
    pub SRAM2__21: u32,
    pub SRAM0__22: u32,
    pub SRAM1__22: u32,
    pub SRAM2__22: u32,
    pub SRAM0__23: u32,
    pub SRAM1__23: u32,
    pub SRAM2__23: u32,
    pub SRAM0__24: u32,
    pub SRAM1__24: u32,
    pub SRAM2__24: u32,
    pub SRAM0__25: u32,
    pub SRAM1__25: u32,
    pub SRAM2__25: u32,
    pub SRAM0__26: u32,
    pub SRAM1__26: u32,
    pub SRAM2__26: u32,
    pub SRAM0__27: u32,
    pub SRAM1__27: u32,
    pub SRAM2__27: u32,
    pub SRAM0__28: u32,
    pub SRAM1__28: u32,
    pub SRAM2__28: u32,
    pub SRAM0__29: u32,
    pub SRAM1__29: u32,
    pub SRAM2__29: u32,
    pub SRAM0__30: u32,
    pub SRAM1__30: u32,
    pub SRAM2__30: u32,
    pub SRAM0__31: u32,
    pub SRAM1__31: u32,
    pub SRAM2__31: u32,
    pub SRAM0__32: u32,
    pub SRAM1__32: u32,
    pub SRAM2__32: u32,
    pub SRAM0__33: u32,
    pub SRAM1__33: u32,
    pub SRAM2__33: u32,
    pub SRAM0__34: u32,
    pub SRAM1__34: u32,
    pub SRAM2__34: u32,
    pub SRAM0__35: u32,
    pub SRAM1__35: u32,
    pub SRAM2__35: u32,
    pub SRAM0__36: u32,
    pub SRAM1__36: u32,
    pub SRAM2__36: u32,
    pub SRAM0__37: u32,
    pub SRAM1__37: u32,
    pub SRAM2__37: u32,
    pub SRAM0__38: u32,
    pub SRAM1__38: u32,
    pub SRAM2__38: u32,
    pub SRAM0__39: u32,
    pub SRAM1__39: u32,
    pub SRAM2__39: u32,
    pub SRAM0__40: u32,
    pub SRAM1__40: u32,
    pub SRAM2__40: u32,
    pub SRAM0__41: u32,
    pub SRAM1__41: u32,
    pub SRAM2__41: u32,
    pub SRAM0__42: u32,
    pub SRAM1__42: u32,
    pub SRAM2__42: u32,
    pub SRAM0__43: u32,
    pub SRAM1__43: u32,
    pub SRAM2__43: u32,
    pub SRAM0__44: u32,
    pub SRAM1__44: u32,
    pub SRAM2__44: u32,
    pub SRAM0__45: u32,
    pub SRAM1__45: u32,
    pub SRAM2__45: u32,
    pub SRAM0__46: u32,
    pub SRAM1__46: u32,
    pub SRAM2__46: u32,
    pub SRAM0__47: u32,
    pub SRAM1__47: u32,
    pub SRAM2__47: u32,
    pub SRAM0__48: u32,
    pub SRAM1__48: u32,
    pub SRAM2__48: u32,
    pub SRAM0__49: u32,
    pub SRAM1__49: u32,
    pub SRAM2__49: u32,
    pub SRAM0__50: u32,
    pub SRAM1__50: u32,
    pub SRAM2__50: u32,
    pub SRAM0__51: u32,
    pub SRAM1__51: u32,
    pub SRAM2__51: u32,
    pub SRAM0__52: u32,
    pub SRAM1__52: u32,
    pub SRAM2__52: u32,
    pub SRAM0__53: u32,
    pub SRAM1__53: u32,
    pub SRAM2__53: u32,
    pub SRAM0__54: u32,
    pub SRAM1__54: u32,
    pub SRAM2__54: u32,
    pub SRAM0__55: u32,
    pub SRAM1__55: u32,
    pub SRAM2__55: u32,
    pub SRAM0__56: u32,
    pub SRAM1__56: u32,
    pub SRAM2__56: u32,
    pub SRAM0__57: u32,
    pub SRAM1__57: u32,
    pub SRAM2__57: u32,
    pub SRAM0__58: u32,
    pub SRAM1__58: u32,
    pub SRAM2__58: u32,
    pub SRAM0__59: u32,
    pub SRAM1__59: u32,
    pub SRAM2__59: u32,
    pub SRAM0__60: u32,
    pub SRAM1__60: u32,
    pub SRAM2__60: u32,
    pub SRAM0__61: u32,
    pub SRAM1__61: u32,
    pub SRAM2__61: u32,
    pub SRAM0__62: u32,
    pub SRAM1__62: u32,
    pub SRAM2__62: u32,
    pub SRAM0__63: u32,
    pub SRAM1__63: u32,
    pub SRAM2__63: u32,
    pub SRAM0__64: u32,
    pub SRAM1__64: u32,
    pub SRAM2__64: u32,
    pub SRAM0__65: u32,
    pub SRAM1__65: u32,
    pub SRAM2__65: u32,
    pub SRAM0__66: u32,
    pub SRAM1__66: u32,
    pub SRAM2__66: u32,
    pub SRAM0__67: u32,
    pub SRAM1__67: u32,
    pub SRAM2__67: u32,
    pub SRAM0__68: u32,
    pub SRAM1__68: u32,
    pub SRAM2__68: u32,
    pub SRAM0__69: u32,
    pub SRAM1__69: u32,
    pub SRAM2__69: u32,
    pub SRAM0__70: u32,
    pub SRAM1__70: u32,
    pub SRAM2__70: u32,
    pub SRAM0__71: u32,
    pub SRAM1__71: u32,
    pub SRAM2__71: u32,
    pub SRAM0__72: u32,
    pub SRAM1__72: u32,
    pub SRAM2__72: u32,
    pub SRAM0__73: u32,
    pub SRAM1__73: u32,
    pub SRAM2__73: u32,
    pub SRAM0__74: u32,
    pub SRAM1__74: u32,
    pub SRAM2__74: u32,
    pub SRAM0__75: u32,
    pub SRAM1__75: u32,
    pub SRAM2__75: u32,
    pub SRAM0__76: u32,
    pub SRAM1__76: u32,
    pub SRAM2__76: u32,
    pub SRAM0__77: u32,
    pub SRAM1__77: u32,
    pub SRAM2__77: u32,
    pub SRAM0__78: u32,
    pub SRAM1__78: u32,
    pub SRAM2__78: u32,
    pub SRAM0__79: u32,
    pub SRAM1__79: u32,
    pub SRAM2__79: u32,
    pub SRAM0__80: u32,
    pub SRAM1__80: u32,
    pub SRAM2__80: u32,
    pub SRAM0__81: u32,
    pub SRAM1__81: u32,
    pub SRAM2__81: u32,
    pub SRAM0__82: u32,
    pub SRAM1__82: u32,
    pub SRAM2__82: u32,
    pub SRAM0__83: u32,
    pub SRAM1__83: u32,
    pub SRAM2__83: u32,
    pub SRAM0__84: u32,
    pub SRAM1__84: u32,
    pub SRAM2__84: u32,
    pub SRAM0__85: u32,
    pub SRAM1__85: u32,
    pub SRAM2__85: u32,
    pub SRAM0__86: u32,
    pub SRAM1__86: u32,
    pub SRAM2__86: u32,
    pub SRAM0__87: u32,
    pub SRAM1__87: u32,
    pub SRAM2__87: u32,
    pub SRAM0__88: u32,
    pub SRAM1__88: u32,
    pub SRAM2__88: u32,
    pub SRAM0__89: u32,
    pub SRAM1__89: u32,
    pub SRAM2__89: u32,
    pub SRAM0__90: u32,
    pub SRAM1__90: u32,
    pub SRAM2__90: u32,
    pub SRAM0__91: u32,
    pub SRAM1__91: u32,
    pub SRAM2__91: u32,
    pub SRAM0__92: u32,
    pub SRAM1__92: u32,
    pub SRAM2__92: u32,
    pub SRAM0__93: u32,
    pub SRAM1__93: u32,
    pub SRAM2__93: u32,
    pub SRAM0__94: u32,
    pub SRAM1__94: u32,
    pub SRAM2__94: u32,
    pub SRAM0__95: u32,
    pub SRAM1__95: u32,
    pub SRAM2__95: u32,
    pub SRAM0__96: u32,
    pub SRAM1__96: u32,
    pub SRAM2__96: u32,
    pub SRAM0__97: u32,
    pub SRAM1__97: u32,
    pub SRAM2__97: u32,
    pub SRAM0__98: u32,
    pub SRAM1__98: u32,
    pub SRAM2__98: u32,
    pub SRAM0__99: u32,
    pub SRAM1__99: u32,
    pub SRAM2__99: u32,
    pub SRAM0__100: u32,
    pub SRAM1__100: u32,
    pub SRAM2__100: u32,
    pub SRAM0__101: u32,
    pub SRAM1__101: u32,
    pub SRAM2__101: u32,
    pub SRAM0__102: u32,
    pub SRAM1__102: u32,
    pub SRAM2__102: u32,
    pub SRAM0__103: u32,
    pub SRAM1__103: u32,
    pub SRAM2__103: u32,
    pub SRAM0__104: u32,
    pub SRAM1__104: u32,
    pub SRAM2__104: u32,
    pub SRAM0__105: u32,
    pub SRAM1__105: u32,
    pub SRAM2__105: u32,
    pub SRAM0__106: u32,
    pub SRAM1__106: u32,
    pub SRAM2__106: u32,
    pub SRAM0__107: u32,
    pub SRAM1__107: u32,
    pub SRAM2__107: u32,
    pub SRAM0__108: u32,
    pub SRAM1__108: u32,
    pub SRAM2__108: u32,
    pub SRAM0__109: u32,
    pub SRAM1__109: u32,
    pub SRAM2__109: u32,
    pub SRAM0__110: u32,
    pub SRAM1__110: u32,
    pub SRAM2__110: u32,
    pub SRAM0__111: u32,
    pub SRAM1__111: u32,
    pub SRAM2__111: u32,
    pub SRAM0__112: u32,
    pub SRAM1__112: u32,
    pub SRAM2__112: u32,
    pub SRAM0__113: u32,
    pub SRAM1__113: u32,
    pub SRAM2__113: u32,
    pub SRAM0__114: u32,
    pub SRAM1__114: u32,
    pub SRAM2__114: u32,
    pub SRAM0__115: u32,
    pub SRAM1__115: u32,
    pub SRAM2__115: u32,
    pub SRAM0__116: u32,
    pub SRAM1__116: u32,
    pub SRAM2__116: u32,
    pub SRAM0__117: u32,
    pub SRAM1__117: u32,
    pub SRAM2__117: u32,
    pub SRAM0__118: u32,
    pub SRAM1__118: u32,
    pub SRAM2__118: u32,
    pub SRAM0__119: u32,
    pub SRAM1__119: u32,
    pub SRAM2__119: u32,
    pub SRAM0__120: u32,
    pub SRAM1__120: u32,
    pub SRAM2__120: u32,
    pub SRAM0__121: u32,
    pub SRAM1__121: u32,
    pub SRAM2__121: u32,
    pub SRAM0__122: u32,
    pub SRAM1__122: u32,
    pub SRAM2__122: u32,
    pub SRAM0__123: u32,
    pub SRAM1__123: u32,
    pub SRAM2__123: u32,
    pub SRAM0__124: u32,
    pub SRAM1__124: u32,
    pub SRAM2__124: u32,
    pub SRAM0__125: u32,
    pub SRAM1__125: u32,
    pub SRAM2__125: u32,
    pub SRAM0__126: u32,
    pub SRAM1__126: u32,
    pub SRAM2__126: u32,
    pub SRAM0__127: u32,
    pub SRAM1__127: u32,
    pub SRAM2__127: u32,
    pub SRAM0__128: u32,
    pub SRAM1__128: u32,
    pub SRAM2__128: u32,
    pub SRAM0__129: u32,
    pub SRAM1__129: u32,
    pub SRAM2__129: u32,
    pub SRAM0__130: u32,
    pub SRAM1__130: u32,
    pub SRAM2__130: u32,
    pub SRAM0__131: u32,
    pub SRAM1__131: u32,
    pub SRAM2__131: u32,
    pub SRAM0__132: u32,
    pub SRAM1__132: u32,
    pub SRAM2__132: u32,
    pub SRAM0__133: u32,
    pub SRAM1__133: u32,
    pub SRAM2__133: u32,
    pub SRAM0__134: u32,
    pub SRAM1__134: u32,
    pub SRAM2__134: u32,
    pub SRAM0__135: u32,
    pub SRAM1__135: u32,
    pub SRAM2__135: u32,
    pub SRAM0__136: u32,
    pub SRAM1__136: u32,
    pub SRAM2__136: u32,
    pub SRAM0__137: u32,
    pub SRAM1__137: u32,
    pub SRAM2__137: u32,
    pub SRAM0__138: u32,
    pub SRAM1__138: u32,
    pub SRAM2__138: u32,
    pub SRAM0__139: u32,
    pub SRAM1__139: u32,
    pub SRAM2__139: u32,
    pub SRAM0__140: u32,
    pub SRAM1__140: u32,
    pub SRAM2__140: u32,
    pub SRAM0__141: u32,
    pub SRAM1__141: u32,
    pub SRAM2__141: u32,
    pub SRAM0__142: u32,
    pub SRAM1__142: u32,
    pub SRAM2__142: u32,
    pub SRAM0__143: u32,
    pub SRAM1__143: u32,
    pub SRAM2__143: u32,
    pub SRAM0__144: u32,
    pub SRAM1__144: u32,
    pub SRAM2__144: u32,
    pub SRAM0__145: u32,
    pub SRAM1__145: u32,
    pub SRAM2__145: u32,
    pub SRAM0__146: u32,
    pub SRAM1__146: u32,
    pub SRAM2__146: u32,
    pub SRAM0__147: u32,
    pub SRAM1__147: u32,
    pub SRAM2__147: u32,
    pub SRAM0__148: u32,
    pub SRAM1__148: u32,
    pub SRAM2__148: u32,
    pub SRAM0__149: u32,
    pub SRAM1__149: u32,
    pub SRAM2__149: u32,
    pub SRAM0__150: u32,
    pub SRAM1__150: u32,
    pub SRAM2__150: u32,
    pub SRAM0__151: u32,
    pub SRAM1__151: u32,
    pub SRAM2__151: u32,
    pub SRAM0__152: u32,
    pub SRAM1__152: u32,
    pub SRAM2__152: u32,
    pub SRAM0__153: u32,
    pub SRAM1__153: u32,
    pub SRAM2__153: u32,
    pub SRAM0__154: u32,
    pub SRAM1__154: u32,
    pub SRAM2__154: u32,
    pub SRAM0__155: u32,
    pub SRAM1__155: u32,
    pub SRAM2__155: u32,
    pub SRAM0__156: u32,
    pub SRAM1__156: u32,
    pub SRAM2__156: u32,
    pub SRAM0__157: u32,
    pub SRAM1__157: u32,
    pub SRAM2__157: u32,
    pub SRAM0__158: u32,
    pub SRAM1__158: u32,
    pub SRAM2__158: u32,
    pub SRAM0__159: u32,
    pub SRAM1__159: u32,
    pub SRAM2__159: u32,
    pub SRAM0__160: u32,
    pub SRAM1__160: u32,
    pub SRAM2__160: u32,
    pub SRAM0__161: u32,
    pub SRAM1__161: u32,
    pub SRAM2__161: u32,
    pub SRAM0__162: u32,
    pub SRAM1__162: u32,
    pub SRAM2__162: u32,
    pub SRAM0__163: u32,
    pub SRAM1__163: u32,
    pub SRAM2__163: u32,
    pub SRAM0__164: u32,
    pub SRAM1__164: u32,
    pub SRAM2__164: u32,
    pub SRAM0__165: u32,
    pub SRAM1__165: u32,
    pub SRAM2__165: u32,
    pub SRAM0__166: u32,
    pub SRAM1__166: u32,
    pub SRAM2__166: u32,
    pub SRAM0__167: u32,
    pub SRAM1__167: u32,
    pub SRAM2__167: u32,
    pub SRAM0__168: u32,
    pub SRAM1__168: u32,
    pub SRAM2__168: u32,
    pub SRAM0__169: u32,
    pub SRAM1__169: u32,
    pub SRAM2__169: u32,
    pub SRAM0__170: u32,
    pub SRAM1__170: u32,
    pub SRAM2__170: u32,
    pub SRAM0__171: u32,
    pub SRAM1__171: u32,
    pub SRAM2__171: u32,
    pub SRAM0__172: u32,
    pub SRAM1__172: u32,
    pub SRAM2__172: u32,
    pub SRAM0__173: u32,
    pub SRAM1__173: u32,
    pub SRAM2__173: u32,
    pub SRAM0__174: u32,
    pub SRAM1__174: u32,
    pub SRAM2__174: u32,
    pub SRAM0__175: u32,
    pub SRAM1__175: u32,
    pub SRAM2__175: u32,
    pub SRAM0__176: u32,
    pub SRAM1__176: u32,
    pub SRAM2__176: u32,
    pub SRAM0__177: u32,
    pub SRAM1__177: u32,
    pub SRAM2__177: u32,
    pub SRAM0__178: u32,
    pub SRAM1__178: u32,
    pub SRAM2__178: u32,
    pub SRAM0__179: u32,
    pub SRAM1__179: u32,
    pub SRAM2__179: u32,
    pub SRAM0__180: u32,
    pub SRAM1__180: u32,
    pub SRAM2__180: u32,
    pub SRAM0__181: u32,
    pub SRAM1__181: u32,
    pub SRAM2__181: u32,
    pub SRAM0__182: u32,
    pub SRAM1__182: u32,
    pub SRAM2__182: u32,
    pub SRAM0__183: u32,
    pub SRAM1__183: u32,
    pub SRAM2__183: u32,
    pub SRAM0__184: u32,
    pub SRAM1__184: u32,
    pub SRAM2__184: u32,
    pub SRAM0__185: u32,
    pub SRAM1__185: u32,
    pub SRAM2__185: u32,
    pub SRAM0__186: u32,
    pub SRAM1__186: u32,
    pub SRAM2__186: u32,
    pub SRAM0__187: u32,
    pub SRAM1__187: u32,
    pub SRAM2__187: u32,
    pub SRAM0__188: u32,
    pub SRAM1__188: u32,
    pub SRAM2__188: u32,
    pub SRAM0__189: u32,
    pub SRAM1__189: u32,
    pub SRAM2__189: u32,
    pub SRAM0__190: u32,
    pub SRAM1__190: u32,
    pub SRAM2__190: u32,
    pub SRAM0__191: u32,
    pub SRAM1__191: u32,
    pub SRAM2__191: u32,
    pub SRAM0__192: u32,
    pub SRAM1__192: u32,
    pub SRAM2__192: u32,
    pub SRAM0__193: u32,
    pub SRAM1__193: u32,
    pub SRAM2__193: u32,
    pub SRAM0__194: u32,
    pub SRAM1__194: u32,
    pub SRAM2__194: u32,
    pub SRAM0__195: u32,
    pub SRAM1__195: u32,
    pub SRAM2__195: u32,
    pub SRAM0__196: u32,
    pub SRAM1__196: u32,
    pub SRAM2__196: u32,
    pub SRAM0__197: u32,
    pub SRAM1__197: u32,
    pub SRAM2__197: u32,
    pub SRAM0__198: u32,
    pub SRAM1__198: u32,
    pub SRAM2__198: u32,
    pub SRAM0__199: u32,
    pub SRAM1__199: u32,
    pub SRAM2__199: u32,
    pub SRAM0__200: u32,
    pub SRAM1__200: u32,
    pub SRAM2__200: u32,
    pub SRAM0__201: u32,
    pub SRAM1__201: u32,
    pub SRAM2__201: u32,
    pub SRAM0__202: u32,
    pub SRAM1__202: u32,
    pub SRAM2__202: u32,
    pub SRAM0__203: u32,
    pub SRAM1__203: u32,
    pub SRAM2__203: u32,
    pub SRAM0__204: u32,
    pub SRAM1__204: u32,
    pub SRAM2__204: u32,
    pub SRAM0__205: u32,
    pub SRAM1__205: u32,
    pub SRAM2__205: u32,
    pub SRAM0__206: u32,
    pub SRAM1__206: u32,
    pub SRAM2__206: u32,
    pub SRAM0__207: u32,
    pub SRAM1__207: u32,
    pub SRAM2__207: u32,
    pub SRAM0__208: u32,
    pub SRAM1__208: u32,
    pub SRAM2__208: u32,
    pub SRAM0__209: u32,
    pub SRAM1__209: u32,
    pub SRAM2__209: u32,
    pub SRAM0__210: u32,
    pub SRAM1__210: u32,
    pub SRAM2__210: u32,
    pub SRAM0__211: u32,
    pub SRAM1__211: u32,
    pub SRAM2__211: u32,
    pub SRAM0__212: u32,
    pub SRAM1__212: u32,
    pub SRAM2__212: u32,
    pub SRAM0__213: u32,
    pub SRAM1__213: u32,
    pub SRAM2__213: u32,
    pub SRAM0__214: u32,
    pub SRAM1__214: u32,
    pub SRAM2__214: u32,
    pub SRAM0__215: u32,
    pub SRAM1__215: u32,
    pub SRAM2__215: u32,
    pub SRAM0__216: u32,
    pub SRAM1__216: u32,
    pub SRAM2__216: u32,
    pub SRAM0__217: u32,
    pub SRAM1__217: u32,
    pub SRAM2__217: u32,
    pub SRAM0__218: u32,
    pub SRAM1__218: u32,
    pub SRAM2__218: u32,
    pub SRAM0__219: u32,
    pub SRAM1__219: u32,
    pub SRAM2__219: u32,
    pub SRAM0__220: u32,
    pub SRAM1__220: u32,
    pub SRAM2__220: u32,
    pub SRAM0__221: u32,
    pub SRAM1__221: u32,
    pub SRAM2__221: u32,
    pub SRAM0__222: u32,
    pub SRAM1__222: u32,
    pub SRAM2__222: u32,
    pub SRAM0__223: u32,
    pub SRAM1__223: u32,
    pub SRAM2__223: u32,
    pub SRAM0__224: u32,
    pub SRAM1__224: u32,
    pub SRAM2__224: u32,
    pub SRAM0__225: u32,
    pub SRAM1__225: u32,
    pub SRAM2__225: u32,
    pub SRAM0__226: u32,
    pub SRAM1__226: u32,
    pub SRAM2__226: u32,
    pub SRAM0__227: u32,
    pub SRAM1__227: u32,
    pub SRAM2__227: u32,
    pub SRAM0__228: u32,
    pub SRAM1__228: u32,
    pub SRAM2__228: u32,
    pub SRAM0__229: u32,
    pub SRAM1__229: u32,
    pub SRAM2__229: u32,
    pub SRAM0__230: u32,
    pub SRAM1__230: u32,
    pub SRAM2__230: u32,
    pub SRAM0__231: u32,
    pub SRAM1__231: u32,
    pub SRAM2__231: u32,
    pub SRAM0__232: u32,
    pub SRAM1__232: u32,
    pub SRAM2__232: u32,
    pub SRAM0__233: u32,
    pub SRAM1__233: u32,
    pub SRAM2__233: u32,
    pub SRAM0__234: u32,
    pub SRAM1__234: u32,
    pub SRAM2__234: u32,
    pub SRAM0__235: u32,
    pub SRAM1__235: u32,
    pub SRAM2__235: u32,
    pub SRAM0__236: u32,
    pub SRAM1__236: u32,
    pub SRAM2__236: u32,
    pub SRAM0__237: u32,
    pub SRAM1__237: u32,
    pub SRAM2__237: u32,
    pub SRAM0__238: u32,
    pub SRAM1__238: u32,
    pub SRAM2__238: u32,
    pub SRAM0__239: u32,
    pub SRAM1__239: u32,
    pub SRAM2__239: u32,
    pub SRAM0__240: u32,
    pub SRAM1__240: u32,
    pub SRAM2__240: u32,
    pub SRAM0__241: u32,
    pub SRAM1__241: u32,
    pub SRAM2__241: u32,
    pub SRAM0__242: u32,
    pub SRAM1__242: u32,
    pub SRAM2__242: u32,
    pub SRAM0__243: u32,
    pub SRAM1__243: u32,
    pub SRAM2__243: u32,
    pub SRAM0__244: u32,
    pub SRAM1__244: u32,
    pub SRAM2__244: u32,
    pub SRAM0__245: u32,
    pub SRAM1__245: u32,
    pub SRAM2__245: u32,
    pub SRAM0__246: u32,
    pub SRAM1__246: u32,
    pub SRAM2__246: u32,
    pub SRAM0__247: u32,
    pub SRAM1__247: u32,
    pub SRAM2__247: u32,
    pub SRAM0__248: u32,
    pub SRAM1__248: u32,
    pub SRAM2__248: u32,
    pub SRAM0__249: u32,
    pub SRAM1__249: u32,
    pub SRAM2__249: u32,
    pub SRAM0__250: u32,
    pub SRAM1__250: u32,
    pub SRAM2__250: u32,
    pub SRAM0__251: u32,
    pub SRAM1__251: u32,
    pub SRAM2__251: u32,
    pub SRAM0__252: u32,
    pub SRAM1__252: u32,
    pub SRAM2__252: u32,
    pub SRAM0__253: u32,
    pub SRAM1__253: u32,
    pub SRAM2__253: u32,
    pub SRAM0__254: u32,
    pub SRAM1__254: u32,
    pub SRAM2__254: u32,
    pub SRAM0__255: u32,
    pub SRAM1__255: u32,
    pub SRAM2__255: u32,
    pub SRAM0__256: u32,
    pub SRAM1__256: u32,
    pub SRAM2__256: u32,
    pub SRAM0__257: u32,
    pub SRAM1__257: u32,
    pub SRAM2__257: u32,
    pub SRAM0__258: u32,
    pub SRAM1__258: u32,
    pub SRAM2__258: u32,
    pub SRAM0__259: u32,
    pub SRAM1__259: u32,
    pub SRAM2__259: u32,
    pub SRAM0__260: u32,
    pub SRAM1__260: u32,
    pub SRAM2__260: u32,
    pub SRAM0__261: u32,
    pub SRAM1__261: u32,
    pub SRAM2__261: u32,
    pub SRAM0__262: u32,
    pub SRAM1__262: u32,
    pub SRAM2__262: u32,
    pub SRAM0__263: u32,
    pub SRAM1__263: u32,
    pub SRAM2__263: u32,
    pub SRAM0__264: u32,
    pub SRAM1__264: u32,
    pub SRAM2__264: u32,
    pub SRAM0__265: u32,
    pub SRAM1__265: u32,
    pub SRAM2__265: u32,
    pub SRAM0__266: u32,
    pub SRAM1__266: u32,
    pub SRAM2__266: u32,
    pub SRAM0__267: u32,
    pub SRAM1__267: u32,
    pub SRAM2__267: u32,
    pub SRAM0__268: u32,
    pub SRAM1__268: u32,
    pub SRAM2__268: u32,
    pub SRAM0__269: u32,
    pub SRAM1__269: u32,
    pub SRAM2__269: u32,
    pub SRAM0__270: u32,
    pub SRAM1__270: u32,
    pub SRAM2__270: u32,
    pub SRAM0__271: u32,
    pub SRAM1__271: u32,
    pub SRAM2__271: u32,
    pub SRAM0__272: u32,
    pub SRAM1__272: u32,
    pub SRAM2__272: u32,
    pub SRAM0__273: u32,
    pub SRAM1__273: u32,
    pub SRAM2__273: u32,
    pub SRAM0__274: u32,
    pub SRAM1__274: u32,
    pub SRAM2__274: u32,
    pub SRAM0__275: u32,
    pub SRAM1__275: u32,
    pub SRAM2__275: u32,
    pub SRAM0__276: u32,
    pub SRAM1__276: u32,
    pub SRAM2__276: u32,
    pub SRAM0__277: u32,
    pub SRAM1__277: u32,
    pub SRAM2__277: u32,
    pub SRAM0__278: u32,
    pub SRAM1__278: u32,
    pub SRAM2__278: u32,
    pub SRAM0__279: u32,
    pub SRAM1__279: u32,
    pub SRAM2__279: u32,
    pub SRAM0__280: u32,
    pub SRAM1__280: u32,
    pub SRAM2__280: u32,
    pub SRAM0__281: u32,
    pub SRAM1__281: u32,
    pub SRAM2__281: u32,
    pub SRAM0__282: u32,
    pub SRAM1__282: u32,
    pub SRAM2__282: u32,
    pub SRAM0__283: u32,
    pub SRAM1__283: u32,
    pub SRAM2__283: u32,
    pub SRAM0__284: u32,
    pub SRAM1__284: u32,
    pub SRAM2__284: u32,
    pub SRAM0__285: u32,
    pub SRAM1__285: u32,
    pub SRAM2__285: u32,
    pub SRAM0__286: u32,
    pub SRAM1__286: u32,
    pub SRAM2__286: u32,
    pub SRAM0__287: u32,
    pub SRAM1__287: u32,
    pub SRAM2__287: u32,
    pub SRAM0__288: u32,
    pub SRAM1__288: u32,
    pub SRAM2__288: u32,
    pub SRAM0__289: u32,
    pub SRAM1__289: u32,
    pub SRAM2__289: u32,
    pub SRAM0__290: u32,
    pub SRAM1__290: u32,
    pub SRAM2__290: u32,
    pub SRAM0__291: u32,
    pub SRAM1__291: u32,
    pub SRAM2__291: u32,
    pub SRAM0__292: u32,
    pub SRAM1__292: u32,
    pub SRAM2__292: u32,
    pub SRAM0__293: u32,
    pub SRAM1__293: u32,
    pub SRAM2__293: u32,
    pub SRAM0__294: u32,
    pub SRAM1__294: u32,
    pub SRAM2__294: u32,
    pub SRAM0__295: u32,
    pub SRAM1__295: u32,
    pub SRAM2__295: u32,
    pub SRAM0__296: u32,
    pub SRAM1__296: u32,
    pub SRAM2__296: u32,
    pub SRAM0__297: u32,
    pub SRAM1__297: u32,
    pub SRAM2__297: u32,
    pub SRAM0__298: u32,
    pub SRAM1__298: u32,
    pub SRAM2__298: u32,
    pub SRAM0__299: u32,
    pub SRAM1__299: u32,
    pub SRAM2__299: u32,
    pub SRAM0__300: u32,
    pub SRAM1__300: u32,
    pub SRAM2__300: u32,
    pub SRAM0__301: u32,
    pub SRAM1__301: u32,
    pub SRAM2__301: u32,
    pub SRAM0__302: u32,
    pub SRAM1__302: u32,
    pub SRAM2__302: u32,
    pub SRAM0__303: u32,
    pub SRAM1__303: u32,
    pub SRAM2__303: u32,
    pub SRAM0__304: u32,
    pub SRAM1__304: u32,
    pub SRAM2__304: u32,
    pub SRAM0__305: u32,
    pub SRAM1__305: u32,
    pub SRAM2__305: u32,
    pub SRAM0__306: u32,
    pub SRAM1__306: u32,
    pub SRAM2__306: u32,
    pub SRAM0__307: u32,
    pub SRAM1__307: u32,
    pub SRAM2__307: u32,
    pub SRAM0__308: u32,
    pub SRAM1__308: u32,
    pub SRAM2__308: u32,
    pub SRAM0__309: u32,
    pub SRAM1__309: u32,
    pub SRAM2__309: u32,
    pub SRAM0__310: u32,
    pub SRAM1__310: u32,
    pub SRAM2__310: u32,
    pub SRAM0__311: u32,
    pub SRAM1__311: u32,
    pub SRAM2__311: u32,
    pub SRAM0__312: u32,
    pub SRAM1__312: u32,
    pub SRAM2__312: u32,
    pub SRAM0__313: u32,
    pub SRAM1__313: u32,
    pub SRAM2__313: u32,
    pub SRAM0__314: u32,
    pub SRAM1__314: u32,
    pub SRAM2__314: u32,
    pub SRAM0__315: u32,
    pub SRAM1__315: u32,
    pub SRAM2__315: u32,
    pub SRAM0__316: u32,
    pub SRAM1__316: u32,
    pub SRAM2__316: u32,
    pub SRAM0__317: u32,
    pub SRAM1__317: u32,
    pub SRAM2__317: u32,
    pub SRAM0__318: u32,
    pub SRAM1__318: u32,
    pub SRAM2__318: u32,
    pub SRAM0__319: u32,
    pub SRAM1__319: u32,
    pub SRAM2__319: u32,
    pub SRAM0__320: u32,
    pub SRAM1__320: u32,
    pub SRAM2__320: u32,
    pub SRAM0__321: u32,
    pub SRAM1__321: u32,
    pub SRAM2__321: u32,
    pub SRAM0__322: u32,
    pub SRAM1__322: u32,
    pub SRAM2__322: u32,
    pub SRAM0__323: u32,
    pub SRAM1__323: u32,
    pub SRAM2__323: u32,
    pub SRAM0__324: u32,
    pub SRAM1__324: u32,
    pub SRAM2__324: u32,
    pub SRAM0__325: u32,
    pub SRAM1__325: u32,
    pub SRAM2__325: u32,
    pub SRAM0__326: u32,
    pub SRAM1__326: u32,
    pub SRAM2__326: u32,
    pub SRAM0__327: u32,
    pub SRAM1__327: u32,
    pub SRAM2__327: u32,
    pub SRAM0__328: u32,
    pub SRAM1__328: u32,
    pub SRAM2__328: u32,
    pub SRAM0__329: u32,
    pub SRAM1__329: u32,
    pub SRAM2__329: u32,
    pub SRAM0__330: u32,
    pub SRAM1__330: u32,
    pub SRAM2__330: u32,
    pub SRAM0__331: u32,
    pub SRAM1__331: u32,
    pub SRAM2__331: u32,
    pub SRAM0__332: u32,
    pub SRAM1__332: u32,
    pub SRAM2__332: u32,
    pub SRAM0__333: u32,
    pub SRAM1__333: u32,
    pub SRAM2__333: u32,
    pub SRAM0__334: u32,
    pub SRAM1__334: u32,
    pub SRAM2__334: u32,
    pub SRAM0__335: u32,
    pub SRAM1__335: u32,
    pub SRAM2__335: u32,
    pub SRAM0__336: u32,
    pub SRAM1__336: u32,
    pub SRAM2__336: u32,
    pub SRAM0__337: u32,
    pub SRAM1__337: u32,
    pub SRAM2__337: u32,
    pub SRAM0__338: u32,
    pub SRAM1__338: u32,
    pub SRAM2__338: u32,
    pub SRAM0__339: u32,
    pub SRAM1__339: u32,
    pub SRAM2__339: u32,
    pub SRAM0__340: u32,
    pub SRAM1__340: u32,
    pub SRAM2__340: u32,
    pub SRAM0__341: u32,
    pub SRAM1__341: u32,
    pub SRAM2__341: u32,
    pub SRAM0__342: u32,
    pub SRAM1__342: u32,
    pub SRAM2__342: u32,
    pub SRAM0__343: u32,
    pub SRAM1__343: u32,
    pub SRAM2__343: u32,
    pub SRAM0__344: u32,
    pub SRAM1__344: u32,
    pub SRAM2__344: u32,
    pub SRAM0__345: u32,
    pub SRAM1__345: u32,
    pub SRAM2__345: u32,
    pub SRAM0__346: u32,
    pub SRAM1__346: u32,
    pub SRAM2__346: u32,
    pub SRAM0__347: u32,
    pub SRAM1__347: u32,
    pub SRAM2__347: u32,
    pub SRAM0__348: u32,
    pub SRAM1__348: u32,
    pub SRAM2__348: u32,
    pub SRAM0__349: u32,
    pub SRAM1__349: u32,
    pub SRAM2__349: u32,
    pub SRAM0__350: u32,
    pub SRAM1__350: u32,
    pub SRAM2__350: u32,
    pub SRAM0__351: u32,
    pub SRAM1__351: u32,
    pub SRAM2__351: u32,
    pub SRAM0__352: u32,
    pub SRAM1__352: u32,
    pub SRAM2__352: u32,
    pub SRAM0__353: u32,
    pub SRAM1__353: u32,
    pub SRAM2__353: u32,
    pub SRAM0__354: u32,
    pub SRAM1__354: u32,
    pub SRAM2__354: u32,
    pub SRAM0__355: u32,
    pub SRAM1__355: u32,
    pub SRAM2__355: u32,
    pub SRAM0__356: u32,
    pub SRAM1__356: u32,
    pub SRAM2__356: u32,
    pub SRAM0__357: u32,
    pub SRAM1__357: u32,
    pub SRAM2__357: u32,
    pub SRAM0__358: u32,
    pub SRAM1__358: u32,
    pub SRAM2__358: u32,
    pub SRAM0__359: u32,
    pub SRAM1__359: u32,
    pub SRAM2__359: u32,
    pub SRAM0__360: u32,
    pub SRAM1__360: u32,
    pub SRAM2__360: u32,
    pub SRAM0__361: u32,
    pub SRAM1__361: u32,
    pub SRAM2__361: u32,
    pub SRAM0__362: u32,
    pub SRAM1__362: u32,
    pub SRAM2__362: u32,
    pub SRAM0__363: u32,
    pub SRAM1__363: u32,
    pub SRAM2__363: u32,
    pub SRAM0__364: u32,
    pub SRAM1__364: u32,
    pub SRAM2__364: u32,
    pub SRAM0__365: u32,
    pub SRAM1__365: u32,
    pub SRAM2__365: u32,
    pub SRAM0__366: u32,
    pub SRAM1__366: u32,
    pub SRAM2__366: u32,
    pub SRAM0__367: u32,
    pub SRAM1__367: u32,
    pub SRAM2__367: u32,
    pub SRAM0__368: u32,
    pub SRAM1__368: u32,
    pub SRAM2__368: u32,
    pub SRAM0__369: u32,
    pub SRAM1__369: u32,
    pub SRAM2__369: u32,
    pub SRAM0__370: u32,
    pub SRAM1__370: u32,
    pub SRAM2__370: u32,
    pub SRAM0__371: u32,
    pub SRAM1__371: u32,
    pub SRAM2__371: u32,
    pub SRAM0__372: u32,
    pub SRAM1__372: u32,
    pub SRAM2__372: u32,
    pub SRAM0__373: u32,
    pub SRAM1__373: u32,
    pub SRAM2__373: u32,
    pub SRAM0__374: u32,
    pub SRAM1__374: u32,
    pub SRAM2__374: u32,
    pub SRAM0__375: u32,
    pub SRAM1__375: u32,
    pub SRAM2__375: u32,
    pub SRAM0__376: u32,
    pub SRAM1__376: u32,
    pub SRAM2__376: u32,
    pub SRAM0__377: u32,
    pub SRAM1__377: u32,
    pub SRAM2__377: u32,
    pub SRAM0__378: u32,
    pub SRAM1__378: u32,
    pub SRAM2__378: u32,
    pub SRAM0__379: u32,
    pub SRAM1__379: u32,
    pub SRAM2__379: u32,
    pub SRAM0__380: u32,
    pub SRAM1__380: u32,
    pub SRAM2__380: u32,
    pub SRAM0__381: u32,
    pub SRAM1__381: u32,
    pub SRAM2__381: u32,
    pub SRAM0__382: u32,
    pub SRAM1__382: u32,
    pub SRAM2__382: u32,
    pub SRAM0__383: u32,
    pub SRAM1__383: u32,
    pub SRAM2__383: u32,
    pub SRAM0__384: u32,
    pub SRAM1__384: u32,
    pub SRAM2__384: u32,
    pub SRAM0__385: u32,
    pub SRAM1__385: u32,
    pub SRAM2__385: u32,
    pub SRAM0__386: u32,
    pub SRAM1__386: u32,
    pub SRAM2__386: u32,
    pub SRAM0__387: u32,
    pub SRAM1__387: u32,
    pub SRAM2__387: u32,
    pub SRAM0__388: u32,
    pub SRAM1__388: u32,
    pub SRAM2__388: u32,
    pub SRAM0__389: u32,
    pub SRAM1__389: u32,
    pub SRAM2__389: u32,
    pub SRAM0__390: u32,
    pub SRAM1__390: u32,
    pub SRAM2__390: u32,
    pub SRAM0__391: u32,
    pub SRAM1__391: u32,
    pub SRAM2__391: u32,
    pub SRAM0__392: u32,
    pub SRAM1__392: u32,
    pub SRAM2__392: u32,
    pub SRAM0__393: u32,
    pub SRAM1__393: u32,
    pub SRAM2__393: u32,
    pub SRAM0__394: u32,
    pub SRAM1__394: u32,
    pub SRAM2__394: u32,
    pub SRAM0__395: u32,
    pub SRAM1__395: u32,
    pub SRAM2__395: u32,
    pub SRAM0__396: u32,
    pub SRAM1__396: u32,
    pub SRAM2__396: u32,
    pub SRAM0__397: u32,
    pub SRAM1__397: u32,
    pub SRAM2__397: u32,
    pub SRAM0__398: u32,
    pub SRAM1__398: u32,
    pub SRAM2__398: u32,
    pub SRAM0__399: u32,
    pub SRAM1__399: u32,
    pub SRAM2__399: u32,
    pub SRAM0__400: u32,
    pub SRAM1__400: u32,
    pub SRAM2__400: u32,
    pub SRAM0__401: u32,
    pub SRAM1__401: u32,
    pub SRAM2__401: u32,
    pub SRAM0__402: u32,
    pub SRAM1__402: u32,
    pub SRAM2__402: u32,
    pub SRAM0__403: u32,
    pub SRAM1__403: u32,
    pub SRAM2__403: u32,
    pub SRAM0__404: u32,
    pub SRAM1__404: u32,
    pub SRAM2__404: u32,
    pub SRAM0__405: u32,
    pub SRAM1__405: u32,
    pub SRAM2__405: u32,
    pub SRAM0__406: u32,
    pub SRAM1__406: u32,
    pub SRAM2__406: u32,
    pub SRAM0__407: u32,
    pub SRAM1__407: u32,
    pub SRAM2__407: u32,
    pub SRAM0__408: u32,
    pub SRAM1__408: u32,
    pub SRAM2__408: u32,
    pub SRAM0__409: u32,
    pub SRAM1__409: u32,
    pub SRAM2__409: u32,
    pub SRAM0__410: u32,
    pub SRAM1__410: u32,
    pub SRAM2__410: u32,
    pub SRAM0__411: u32,
    pub SRAM1__411: u32,
    pub SRAM2__411: u32,
    pub SRAM0__412: u32,
    pub SRAM1__412: u32,
    pub SRAM2__412: u32,
    pub SRAM0__413: u32,
    pub SRAM1__413: u32,
    pub SRAM2__413: u32,
    pub SRAM0__414: u32,
    pub SRAM1__414: u32,
    pub SRAM2__414: u32,
    pub SRAM0__415: u32,
    pub SRAM1__415: u32,
    pub SRAM2__415: u32,
    pub SRAM0__416: u32,
    pub SRAM1__416: u32,
    pub SRAM2__416: u32,
    pub SRAM0__417: u32,
    pub SRAM1__417: u32,
    pub SRAM2__417: u32,
    pub SRAM0__418: u32,
    pub SRAM1__418: u32,
    pub SRAM2__418: u32,
    pub SRAM0__419: u32,
    pub SRAM1__419: u32,
    pub SRAM2__419: u32,
    pub SRAM0__420: u32,
    pub SRAM1__420: u32,
    pub SRAM2__420: u32,
    pub SRAM0__421: u32,
    pub SRAM1__421: u32,
    pub SRAM2__421: u32,
    pub SRAM0__422: u32,
    pub SRAM1__422: u32,
    pub SRAM2__422: u32,
    pub SRAM0__423: u32,
    pub SRAM1__423: u32,
    pub SRAM2__423: u32,
    pub SRAM0__424: u32,
    pub SRAM1__424: u32,
    pub SRAM2__424: u32,
    pub SRAM0__425: u32,
    pub SRAM1__425: u32,
    pub SRAM2__425: u32,
    pub SRAM0__426: u32,
    pub SRAM1__426: u32,
    pub SRAM2__426: u32,
    pub SRAM0__427: u32,
    pub SRAM1__427: u32,
    pub SRAM2__427: u32,
    pub SRAM0__428: u32,
    pub SRAM1__428: u32,
    pub SRAM2__428: u32,
    pub SRAM0__429: u32,
    pub SRAM1__429: u32,
    pub SRAM2__429: u32,
    pub SRAM0__430: u32,
    pub SRAM1__430: u32,
    pub SRAM2__430: u32,
    pub SRAM0__431: u32,
    pub SRAM1__431: u32,
    pub SRAM2__431: u32,
    pub SRAM0__432: u32,
    pub SRAM1__432: u32,
    pub SRAM2__432: u32,
    pub SRAM0__433: u32,
    pub SRAM1__433: u32,
    pub SRAM2__433: u32,
    pub SRAM0__434: u32,
    pub SRAM1__434: u32,
    pub SRAM2__434: u32,
    pub SRAM0__435: u32,
    pub SRAM1__435: u32,
    pub SRAM2__435: u32,
    pub SRAM0__436: u32,
    pub SRAM1__436: u32,
    pub SRAM2__436: u32,
    pub SRAM0__437: u32,
    pub SRAM1__437: u32,
    pub SRAM2__437: u32,
    pub SRAM0__438: u32,
    pub SRAM1__438: u32,
    pub SRAM2__438: u32,
    pub SRAM0__439: u32,
    pub SRAM1__439: u32,
    pub SRAM2__439: u32,
    pub SRAM0__440: u32,
    pub SRAM1__440: u32,
    pub SRAM2__440: u32,
    pub SRAM0__441: u32,
    pub SRAM1__441: u32,
    pub SRAM2__441: u32,
    pub SRAM0__442: u32,
    pub SRAM1__442: u32,
    pub SRAM2__442: u32,
    pub SRAM0__443: u32,
    pub SRAM1__443: u32,
    pub SRAM2__443: u32,
    pub SRAM0__444: u32,
    pub SRAM1__444: u32,
    pub SRAM2__444: u32,
    pub SRAM0__445: u32,
    pub SRAM1__445: u32,
    pub SRAM2__445: u32,
    pub SRAM0__446: u32,
    pub SRAM1__446: u32,
    pub SRAM2__446: u32,
    pub SRAM0__447: u32,
    pub SRAM1__447: u32,
    pub SRAM2__447: u32,
    pub SRAM0__448: u32,
    pub SRAM1__448: u32,
    pub SRAM2__448: u32,
    pub SRAM0__449: u32,
    pub SRAM1__449: u32,
    pub SRAM2__449: u32,
    pub SRAM0__450: u32,
    pub SRAM1__450: u32,
    pub SRAM2__450: u32,
    pub SRAM0__451: u32,
    pub SRAM1__451: u32,
    pub SRAM2__451: u32,
    pub SRAM0__452: u32,
    pub SRAM1__452: u32,
    pub SRAM2__452: u32,
    pub SRAM0__453: u32,
    pub SRAM1__453: u32,
    pub SRAM2__453: u32,
    pub SRAM0__454: u32,
    pub SRAM1__454: u32,
    pub SRAM2__454: u32,
    pub SRAM0__455: u32,
    pub SRAM1__455: u32,
    pub SRAM2__455: u32,
    pub SRAM0__456: u32,
    pub SRAM1__456: u32,
    pub SRAM2__456: u32,
    pub SRAM0__457: u32,
    pub SRAM1__457: u32,
    pub SRAM2__457: u32,
    pub SRAM0__458: u32,
    pub SRAM1__458: u32,
    pub SRAM2__458: u32,
    pub SRAM0__459: u32,
    pub SRAM1__459: u32,
    pub SRAM2__459: u32,
    pub SRAM0__460: u32,
    pub SRAM1__460: u32,
    pub SRAM2__460: u32,
    pub SRAM0__461: u32,
    pub SRAM1__461: u32,
    pub SRAM2__461: u32,
    pub SRAM0__462: u32,
    pub SRAM1__462: u32,
    pub SRAM2__462: u32,
    pub SRAM0__463: u32,
    pub SRAM1__463: u32,
    pub SRAM2__463: u32,
    pub SRAM0__464: u32,
    pub SRAM1__464: u32,
    pub SRAM2__464: u32,
    pub SRAM0__465: u32,
    pub SRAM1__465: u32,
    pub SRAM2__465: u32,
    pub SRAM0__466: u32,
    pub SRAM1__466: u32,
    pub SRAM2__466: u32,
    pub SRAM0__467: u32,
    pub SRAM1__467: u32,
    pub SRAM2__467: u32,
    pub SRAM0__468: u32,
    pub SRAM1__468: u32,
    pub SRAM2__468: u32,
    pub SRAM0__469: u32,
    pub SRAM1__469: u32,
    pub SRAM2__469: u32,
    pub SRAM0__470: u32,
    pub SRAM1__470: u32,
    pub SRAM2__470: u32,
    pub SRAM0__471: u32,
    pub SRAM1__471: u32,
    pub SRAM2__471: u32,
    pub SRAM0__472: u32,
    pub SRAM1__472: u32,
    pub SRAM2__472: u32,
    pub SRAM0__473: u32,
    pub SRAM1__473: u32,
    pub SRAM2__473: u32,
    pub SRAM0__474: u32,
    pub SRAM1__474: u32,
    pub SRAM2__474: u32,
    pub SRAM0__475: u32,
    pub SRAM1__475: u32,
    pub SRAM2__475: u32,
    pub SRAM0__476: u32,
    pub SRAM1__476: u32,
    pub SRAM2__476: u32,
    pub SRAM0__477: u32,
    pub SRAM1__477: u32,
    pub SRAM2__477: u32,
    pub SRAM0__478: u32,
    pub SRAM1__478: u32,
    pub SRAM2__478: u32,
    pub SRAM0__479: u32,
    pub SRAM1__479: u32,
    pub SRAM2__479: u32,
    pub SRAM0__480: u32,
    pub SRAM1__480: u32,
    pub SRAM2__480: u32,
    pub SRAM0__481: u32,
    pub SRAM1__481: u32,
    pub SRAM2__481: u32,
    pub SRAM0__482: u32,
    pub SRAM1__482: u32,
    pub SRAM2__482: u32,
    pub SRAM0__483: u32,
    pub SRAM1__483: u32,
    pub SRAM2__483: u32,
    pub SRAM0__484: u32,
    pub SRAM1__484: u32,
    pub SRAM2__484: u32,
    pub SRAM0__485: u32,
    pub SRAM1__485: u32,
    pub SRAM2__485: u32,
    pub SRAM0__486: u32,
    pub SRAM1__486: u32,
    pub SRAM2__486: u32,
    pub SRAM0__487: u32,
    pub SRAM1__487: u32,
    pub SRAM2__487: u32,
    pub SRAM0__488: u32,
    pub SRAM1__488: u32,
    pub SRAM2__488: u32,
    pub SRAM0__489: u32,
    pub SRAM1__489: u32,
    pub SRAM2__489: u32,
    pub SRAM0__490: u32,
    pub SRAM1__490: u32,
    pub SRAM2__490: u32,
    pub SRAM0__491: u32,
    pub SRAM1__491: u32,
    pub SRAM2__491: u32,
    pub SRAM0__492: u32,
    pub SRAM1__492: u32,
    pub SRAM2__492: u32,
    pub SRAM0__493: u32,
    pub SRAM1__493: u32,
    pub SRAM2__493: u32,
    pub SRAM0__494: u32,
    pub SRAM1__494: u32,
    pub SRAM2__494: u32,
    pub SRAM0__495: u32,
    pub SRAM1__495: u32,
    pub SRAM2__495: u32,
    pub SRAM0__496: u32,
    pub SRAM1__496: u32,
    pub SRAM2__496: u32,
    pub SRAM0__497: u32,
    pub SRAM1__497: u32,
    pub SRAM2__497: u32,
    pub SRAM0__498: u32,
    pub SRAM1__498: u32,
    pub SRAM2__498: u32,
    pub SRAM0__499: u32,
    pub SRAM1__499: u32,
    pub SRAM2__499: u32,
    pub SRAM0__500: u32,
    pub SRAM1__500: u32,
    pub SRAM2__500: u32,
    pub SRAM0__501: u32,
    pub SRAM1__501: u32,
    pub SRAM2__501: u32,
    pub SRAM0__502: u32,
    pub SRAM1__502: u32,
    pub SRAM2__502: u32,
    pub SRAM0__503: u32,
    pub SRAM1__503: u32,
    pub SRAM2__503: u32,
    pub SRAM0__504: u32,
    pub SRAM1__504: u32,
    pub SRAM2__504: u32,
    pub SRAM0__505: u32,
    pub SRAM1__505: u32,
    pub SRAM2__505: u32,
    pub SRAM0__506: u32,
    pub SRAM1__506: u32,
    pub SRAM2__506: u32,
    pub SRAM0__507: u32,
    pub SRAM1__507: u32,
    pub SRAM2__507: u32,
    pub SRAM0__508: u32,
    pub SRAM1__508: u32,
    pub SRAM2__508: u32,
    pub SRAM0__509: u32,
    pub SRAM1__509: u32,
    pub SRAM2__509: u32,
    pub SRAM0__510: u32,
    pub SRAM1__510: u32,
    pub SRAM2__510: u32,
    pub SRAM0__511: u32,
    pub SRAM1__511: u32,
    pub SRAM2__511: u32,
    pub SRAM0__512: u32,
    pub SRAM1__512: u32,
    pub SRAM2__512: u32,
    pub SRAM0__513: u32,
    pub SRAM1__513: u32,
    pub SRAM2__513: u32,
    pub SRAM0__514: u32,
    pub SRAM1__514: u32,
    pub SRAM2__514: u32,
    pub SRAM0__515: u32,
    pub SRAM1__515: u32,
    pub SRAM2__515: u32,
    pub SRAM0__516: u32,
    pub SRAM1__516: u32,
    pub SRAM2__516: u32,
    pub SRAM0__517: u32,
    pub SRAM1__517: u32,
    pub SRAM2__517: u32,
    pub SRAM0__518: u32,
    pub SRAM1__518: u32,
    pub SRAM2__518: u32,
    pub SRAM0__519: u32,
    pub SRAM1__519: u32,
    pub SRAM2__519: u32,
    pub SRAM0__520: u32,
    pub SRAM1__520: u32,
    pub SRAM2__520: u32,
    pub SRAM0__521: u32,
    pub SRAM1__521: u32,
    pub SRAM2__521: u32,
    pub SRAM0__522: u32,
    pub SRAM1__522: u32,
    pub SRAM2__522: u32,
    pub SRAM0__523: u32,
    pub SRAM1__523: u32,
    pub SRAM2__523: u32,
    pub SRAM0__524: u32,
    pub SRAM1__524: u32,
    pub SRAM2__524: u32,
    pub SRAM0__525: u32,
    pub SRAM1__525: u32,
    pub SRAM2__525: u32,
    pub SRAM0__526: u32,
    pub SRAM1__526: u32,
    pub SRAM2__526: u32,
    pub SRAM0__527: u32,
    pub SRAM1__527: u32,
    pub SRAM2__527: u32,
    pub SRAM0__528: u32,
    pub SRAM1__528: u32,
    pub SRAM2__528: u32,
    pub SRAM0__529: u32,
    pub SRAM1__529: u32,
    pub SRAM2__529: u32,
    pub SRAM0__530: u32,
    pub SRAM1__530: u32,
    pub SRAM2__530: u32,
    pub SRAM0__531: u32,
    pub SRAM1__531: u32,
    pub SRAM2__531: u32,
    pub SRAM0__532: u32,
    pub SRAM1__532: u32,
    pub SRAM2__532: u32,
    pub SRAM0__533: u32,
    pub SRAM1__533: u32,
    pub SRAM2__533: u32,
    pub SRAM0__534: u32,
    pub SRAM1__534: u32,
    pub SRAM2__534: u32,
    pub SRAM0__535: u32,
    pub SRAM1__535: u32,
    pub SRAM2__535: u32,
    pub SRAM0__536: u32,
    pub SRAM1__536: u32,
    pub SRAM2__536: u32,
    pub SRAM0__537: u32,
    pub SRAM1__537: u32,
    pub SRAM2__537: u32,
    pub SRAM0__538: u32,
    pub SRAM1__538: u32,
    pub SRAM2__538: u32,
    pub SRAM0__539: u32,
    pub SRAM1__539: u32,
    pub SRAM2__539: u32,
    pub SRAM0__540: u32,
    pub SRAM1__540: u32,
    pub SRAM2__540: u32,
    pub SRAM0__541: u32,
    pub SRAM1__541: u32,
    pub SRAM2__541: u32,
    pub SRAM0__542: u32,
    pub SRAM1__542: u32,
    pub SRAM2__542: u32,
    pub SRAM0__543: u32,
    pub SRAM1__543: u32,
    pub SRAM2__543: u32,
    pub SRAM0__544: u32,
    pub SRAM1__544: u32,
    pub SRAM2__544: u32,
    pub SRAM0__545: u32,
    pub SRAM1__545: u32,
    pub SRAM2__545: u32,
    pub SRAM0__546: u32,
    pub SRAM1__546: u32,
    pub SRAM2__546: u32,
    pub SRAM0__547: u32,
    pub SRAM1__547: u32,
    pub SRAM2__547: u32,
    pub SRAM0__548: u32,
    pub SRAM1__548: u32,
    pub SRAM2__548: u32,
    pub SRAM0__549: u32,
    pub SRAM1__549: u32,
    pub SRAM2__549: u32,
    pub SRAM0__550: u32,
    pub SRAM1__550: u32,
    pub SRAM2__550: u32,
    pub SRAM0__551: u32,
    pub SRAM1__551: u32,
    pub SRAM2__551: u32,
    pub SRAM0__552: u32,
    pub SRAM1__552: u32,
    pub SRAM2__552: u32,
    pub SRAM0__553: u32,
    pub SRAM1__553: u32,
    pub SRAM2__553: u32,
    pub SRAM0__554: u32,
    pub SRAM1__554: u32,
    pub SRAM2__554: u32,
    pub SRAM0__555: u32,
    pub SRAM1__555: u32,
    pub SRAM2__555: u32,
    pub SRAM0__556: u32,
    pub SRAM1__556: u32,
    pub SRAM2__556: u32,
    pub SRAM0__557: u32,
    pub SRAM1__557: u32,
    pub SRAM2__557: u32,
    pub SRAM0__558: u32,
    pub SRAM1__558: u32,
    pub SRAM2__558: u32,
    pub SRAM0__559: u32,
    pub SRAM1__559: u32,
    pub SRAM2__559: u32,
    pub SRAM0__560: u32,
    pub SRAM1__560: u32,
    pub SRAM2__560: u32,
    pub SRAM0__561: u32,
    pub SRAM1__561: u32,
    pub SRAM2__561: u32,
    pub SRAM0__562: u32,
    pub SRAM1__562: u32,
    pub SRAM2__562: u32,
    pub SRAM0__563: u32,
    pub SRAM1__563: u32,
    pub SRAM2__563: u32,
    pub SRAM0__564: u32,
    pub SRAM1__564: u32,
    pub SRAM2__564: u32,
    pub SRAM0__565: u32,
    pub SRAM1__565: u32,
    pub SRAM2__565: u32,
    pub SRAM0__566: u32,
    pub SRAM1__566: u32,
    pub SRAM2__566: u32,
    pub SRAM0__567: u32,
    pub SRAM1__567: u32,
    pub SRAM2__567: u32,
    pub SRAM0__568: u32,
    pub SRAM1__568: u32,
    pub SRAM2__568: u32,
    pub SRAM0__569: u32,
    pub SRAM1__569: u32,
    pub SRAM2__569: u32,
    pub SRAM0__570: u32,
    pub SRAM1__570: u32,
    pub SRAM2__570: u32,
    pub SRAM0__571: u32,
    pub SRAM1__571: u32,
    pub SRAM2__571: u32,
    pub SRAM0__572: u32,
    pub SRAM1__572: u32,
    pub SRAM2__572: u32,
    pub SRAM0__573: u32,
    pub SRAM1__573: u32,
    pub SRAM2__573: u32,
    pub SRAM0__574: u32,
    pub SRAM1__574: u32,
    pub SRAM2__574: u32,
    pub SRAM0__575: u32,
    pub SRAM1__575: u32,
    pub SRAM2__575: u32,
    pub SRAM0__576: u32,
    pub SRAM1__576: u32,
    pub SRAM2__576: u32,
    pub SRAM0__577: u32,
    pub SRAM1__577: u32,
    pub SRAM2__577: u32,
    pub SRAM0__578: u32,
    pub SRAM1__578: u32,
    pub SRAM2__578: u32,
    pub SRAM0__579: u32,
    pub SRAM1__579: u32,
    pub SRAM2__579: u32,
    pub SRAM0__580: u32,
    pub SRAM1__580: u32,
    pub SRAM2__580: u32,
    pub SRAM0__581: u32,
    pub SRAM1__581: u32,
    pub SRAM2__581: u32,
    pub SRAM0__582: u32,
    pub SRAM1__582: u32,
    pub SRAM2__582: u32,
    pub SRAM0__583: u32,
    pub SRAM1__583: u32,
    pub SRAM2__583: u32,
    pub SRAM0__584: u32,
    pub SRAM1__584: u32,
    pub SRAM2__584: u32,
    pub SRAM0__585: u32,
    pub SRAM1__585: u32,
    pub SRAM2__585: u32,
    pub SRAM0__586: u32,
    pub SRAM1__586: u32,
    pub SRAM2__586: u32,
    pub SRAM0__587: u32,
    pub SRAM1__587: u32,
    pub SRAM2__587: u32,
    pub SRAM0__588: u32,
    pub SRAM1__588: u32,
    pub SRAM2__588: u32,
    pub SRAM0__589: u32,
    pub SRAM1__589: u32,
    pub SRAM2__589: u32,
    pub SRAM0__590: u32,
    pub SRAM1__590: u32,
    pub SRAM2__590: u32,
    pub SRAM0__591: u32,
    pub SRAM1__591: u32,
    pub SRAM2__591: u32,
    pub SRAM0__592: u32,
    pub SRAM1__592: u32,
    pub SRAM2__592: u32,
    pub SRAM0__593: u32,
    pub SRAM1__593: u32,
    pub SRAM2__593: u32,
    pub SRAM0__594: u32,
    pub SRAM1__594: u32,
    pub SRAM2__594: u32,
    pub SRAM0__595: u32,
    pub SRAM1__595: u32,
    pub SRAM2__595: u32,
    pub SRAM0__596: u32,
    pub SRAM1__596: u32,
    pub SRAM2__596: u32,
    pub SRAM0__597: u32,
    pub SRAM1__597: u32,
    pub SRAM2__597: u32,
    pub SRAM0__598: u32,
    pub SRAM1__598: u32,
    pub SRAM2__598: u32,
    pub SRAM0__599: u32,
    pub SRAM1__599: u32,
    pub SRAM2__599: u32,
    pub SRAM0__600: u32,
    pub SRAM1__600: u32,
    pub SRAM2__600: u32,
    pub SRAM0__601: u32,
    pub SRAM1__601: u32,
    pub SRAM2__601: u32,
    pub SRAM0__602: u32,
    pub SRAM1__602: u32,
    pub SRAM2__602: u32,
    pub SRAM0__603: u32,
    pub SRAM1__603: u32,
    pub SRAM2__603: u32,
    pub SRAM0__604: u32,
    pub SRAM1__604: u32,
    pub SRAM2__604: u32,
    pub SRAM0__605: u32,
    pub SRAM1__605: u32,
    pub SRAM2__605: u32,
    pub SRAM0__606: u32,
    pub SRAM1__606: u32,
    pub SRAM2__606: u32,
    pub SRAM0__607: u32,
    pub SRAM1__607: u32,
    pub SRAM2__607: u32,
    pub SRAM0__608: u32,
    pub SRAM1__608: u32,
    pub SRAM2__608: u32,
    pub SRAM0__609: u32,
    pub SRAM1__609: u32,
    pub SRAM2__609: u32,
    pub SRAM0__610: u32,
    pub SRAM1__610: u32,
    pub SRAM2__610: u32,
    pub SRAM0__611: u32,
    pub SRAM1__611: u32,
    pub SRAM2__611: u32,
    pub SRAM0__612: u32,
    pub SRAM1__612: u32,
    pub SRAM2__612: u32,
    pub SRAM0__613: u32,
    pub SRAM1__613: u32,
    pub SRAM2__613: u32,
    pub SRAM0__614: u32,
    pub SRAM1__614: u32,
    pub SRAM2__614: u32,
    pub SRAM0__615: u32,
    pub SRAM1__615: u32,
    pub SRAM2__615: u32,
    pub SRAM0__616: u32,
    pub SRAM1__616: u32,
    pub SRAM2__616: u32,
    pub SRAM0__617: u32,
    pub SRAM1__617: u32,
    pub SRAM2__617: u32,
    pub SRAM0__618: u32,
    pub SRAM1__618: u32,
    pub SRAM2__618: u32,
    pub SRAM0__619: u32,
    pub SRAM1__619: u32,
    pub SRAM2__619: u32,
    pub SRAM0__620: u32,
    pub SRAM1__620: u32,
    pub SRAM2__620: u32,
    pub SRAM0__621: u32,
    pub SRAM1__621: u32,
    pub SRAM2__621: u32,
    pub SRAM0__622: u32,
    pub SRAM1__622: u32,
    pub SRAM2__622: u32,
    pub SRAM0__623: u32,
    pub SRAM1__623: u32,
    pub SRAM2__623: u32,
    pub SRAM0__624: u32,
    pub SRAM1__624: u32,
    pub SRAM2__624: u32,
    pub SRAM0__625: u32,
    pub SRAM1__625: u32,
    pub SRAM2__625: u32,
    pub SRAM0__626: u32,
    pub SRAM1__626: u32,
    pub SRAM2__626: u32,
    pub SRAM0__627: u32,
    pub SRAM1__627: u32,
    pub SRAM2__627: u32,
    pub SRAM0__628: u32,
    pub SRAM1__628: u32,
    pub SRAM2__628: u32,
    pub SRAM0__629: u32,
    pub SRAM1__629: u32,
    pub SRAM2__629: u32,
    pub SRAM0__630: u32,
    pub SRAM1__630: u32,
    pub SRAM2__630: u32,
    pub SRAM0__631: u32,
    pub SRAM1__631: u32,
    pub SRAM2__631: u32,
    pub SRAM0__632: u32,
    pub SRAM1__632: u32,
    pub SRAM2__632: u32,
    pub SRAM0__633: u32,
    pub SRAM1__633: u32,
    pub SRAM2__633: u32,
    pub SRAM0__634: u32,
    pub SRAM1__634: u32,
    pub SRAM2__634: u32,
    pub SRAM0__635: u32,
    pub SRAM1__635: u32,
    pub SRAM2__635: u32,
    pub SRAM0__636: u32,
    pub SRAM1__636: u32,
    pub SRAM2__636: u32,
    pub SRAM0__637: u32,
    pub SRAM1__637: u32,
    pub SRAM2__637: u32,
    pub SRAM0__638: u32,
    pub SRAM1__638: u32,
    pub SRAM2__638: u32,
    pub SRAM0__639: u32,
    pub SRAM1__639: u32,
    pub SRAM2__639: u32,
    pub SRAM0__640: u32,
    pub SRAM1__640: u32,
    pub SRAM2__640: u32,
    pub SRAM0__641: u32,
    pub SRAM1__641: u32,
    pub SRAM2__641: u32,
    pub SRAM0__642: u32,
    pub SRAM1__642: u32,
    pub SRAM2__642: u32,
    pub SRAM0__643: u32,
    pub SRAM1__643: u32,
    pub SRAM2__643: u32,
    pub SRAM0__644: u32,
    pub SRAM1__644: u32,
    pub SRAM2__644: u32,
    pub SRAM0__645: u32,
    pub SRAM1__645: u32,
    pub SRAM2__645: u32,
    pub SRAM0__646: u32,
    pub SRAM1__646: u32,
    pub SRAM2__646: u32,
    pub SRAM0__647: u32,
    pub SRAM1__647: u32,
    pub SRAM2__647: u32,
    pub SRAM0__648: u32,
    pub SRAM1__648: u32,
    pub SRAM2__648: u32,
    pub SRAM0__649: u32,
    pub SRAM1__649: u32,
    pub SRAM2__649: u32,
    pub SRAM0__650: u32,
    pub SRAM1__650: u32,
    pub SRAM2__650: u32,
    pub SRAM0__651: u32,
    pub SRAM1__651: u32,
    pub SRAM2__651: u32,
    pub SRAM0__652: u32,
    pub SRAM1__652: u32,
    pub SRAM2__652: u32,
    pub SRAM0__653: u32,
    pub SRAM1__653: u32,
    pub SRAM2__653: u32,
    pub SRAM0__654: u32,
    pub SRAM1__654: u32,
    pub SRAM2__654: u32,
    pub SRAM0__655: u32,
    pub SRAM1__655: u32,
    pub SRAM2__655: u32,
    pub SRAM0__656: u32,
    pub SRAM1__656: u32,
    pub SRAM2__656: u32,
    pub SRAM0__657: u32,
    pub SRAM1__657: u32,
    pub SRAM2__657: u32,
    pub SRAM0__658: u32,
    pub SRAM1__658: u32,
    pub SRAM2__658: u32,
    pub SRAM0__659: u32,
    pub SRAM1__659: u32,
    pub SRAM2__659: u32,
    pub SRAM0__660: u32,
    pub SRAM1__660: u32,
    pub SRAM2__660: u32,
    pub SRAM0__661: u32,
    pub SRAM1__661: u32,
    pub SRAM2__661: u32,
    pub SRAM0__662: u32,
    pub SRAM1__662: u32,
    pub SRAM2__662: u32,
    pub SRAM0__663: u32,
    pub SRAM1__663: u32,
    pub SRAM2__663: u32,
    pub SRAM0__664: u32,
    pub SRAM1__664: u32,
    pub SRAM2__664: u32,
    pub SRAM0__665: u32,
    pub SRAM1__665: u32,
    pub SRAM2__665: u32,
    pub SRAM0__666: u32,
    pub SRAM1__666: u32,
    pub SRAM2__666: u32,
    pub SRAM0__667: u32,
    pub SRAM1__667: u32,
    pub SRAM2__667: u32,
    pub SRAM0__668: u32,
    pub SRAM1__668: u32,
    pub SRAM2__668: u32,
    pub SRAM0__669: u32,
    pub SRAM1__669: u32,
    pub SRAM2__669: u32,
    pub SRAM0__670: u32,
    pub SRAM1__670: u32,
    pub SRAM2__670: u32,
    pub SRAM0__671: u32,
    pub SRAM1__671: u32,
    pub SRAM2__671: u32,
    pub SRAM0__672: u32,
    pub SRAM1__672: u32,
    pub SRAM2__672: u32,
    pub SRAM0__673: u32,
    pub SRAM1__673: u32,
    pub SRAM2__673: u32,
    pub SRAM0__674: u32,
    pub SRAM1__674: u32,
    pub SRAM2__674: u32,
    pub SRAM0__675: u32,
    pub SRAM1__675: u32,
    pub SRAM2__675: u32,
    pub SRAM0__676: u32,
    pub SRAM1__676: u32,
    pub SRAM2__676: u32,
    pub SRAM0__677: u32,
    pub SRAM1__677: u32,
    pub SRAM2__677: u32,
    pub SRAM0__678: u32,
    pub SRAM1__678: u32,
    pub SRAM2__678: u32,
    pub SRAM0__679: u32,
    pub SRAM1__679: u32,
    pub SRAM2__679: u32,
    pub SRAM0__680: u32,
    pub SRAM1__680: u32,
    pub SRAM2__680: u32,
    pub SRAM0__681: u32,
    pub SRAM1__681: u32,
    pub SRAM2__681: u32,
    pub SRAM0__682: u32,
    pub SRAM1__682: u32,
    pub SRAM2__682: u32,
    pub SRAM0__683: u32,
    pub SRAM1__683: u32,
    pub SRAM2__683: u32,
    pub SRAM0__684: u32,
    pub SRAM1__684: u32,
    pub SRAM2__684: u32,
    pub SRAM0__685: u32,
    pub SRAM1__685: u32,
    pub SRAM2__685: u32,
    pub SRAM0__686: u32,
    pub SRAM1__686: u32,
    pub SRAM2__686: u32,
    pub SRAM0__687: u32,
    pub SRAM1__687: u32,
    pub SRAM2__687: u32,
    pub SRAM0__688: u32,
    pub SRAM1__688: u32,
    pub SRAM2__688: u32,
    pub SRAM0__689: u32,
    pub SRAM1__689: u32,
    pub SRAM2__689: u32,
    pub SRAM0__690: u32,
    pub SRAM1__690: u32,
    pub SRAM2__690: u32,
    pub SRAM0__691: u32,
    pub SRAM1__691: u32,
    pub SRAM2__691: u32,
    pub SRAM0__692: u32,
    pub SRAM1__692: u32,
    pub SRAM2__692: u32,
    pub SRAM0__693: u32,
    pub SRAM1__693: u32,
    pub SRAM2__693: u32,
    pub SRAM0__694: u32,
    pub SRAM1__694: u32,
    pub SRAM2__694: u32,
    pub SRAM0__695: u32,
    pub SRAM1__695: u32,
    pub SRAM2__695: u32,
    pub SRAM0__696: u32,
    pub SRAM1__696: u32,
    pub SRAM2__696: u32,
    pub SRAM0__697: u32,
    pub SRAM1__697: u32,
    pub SRAM2__697: u32,
    pub SRAM0__698: u32,
    pub SRAM1__698: u32,
    pub SRAM2__698: u32,
    pub SRAM0__699: u32,
    pub SRAM1__699: u32,
    pub SRAM2__699: u32,
    pub SRAM0__700: u32,
    pub SRAM1__700: u32,
    pub SRAM2__700: u32,
    pub SRAM0__701: u32,
    pub SRAM1__701: u32,
    pub SRAM2__701: u32,
    pub SRAM0__702: u32,
    pub SRAM1__702: u32,
    pub SRAM2__702: u32,
    pub SRAM0__703: u32,
    pub SRAM1__703: u32,
    pub SRAM2__703: u32,
    pub SRAM0__704: u32,
    pub SRAM1__704: u32,
    pub SRAM2__704: u32,
    pub SRAM0__705: u32,
    pub SRAM1__705: u32,
    pub SRAM2__705: u32,
    pub SRAM0__706: u32,
    pub SRAM1__706: u32,
    pub SRAM2__706: u32,
    pub SRAM0__707: u32,
    pub SRAM1__707: u32,
    pub SRAM2__707: u32,
    pub SRAM0__708: u32,
    pub SRAM1__708: u32,
    pub SRAM2__708: u32,
    pub SRAM0__709: u32,
    pub SRAM1__709: u32,
    pub SRAM2__709: u32,
    pub SRAM0__710: u32,
    pub SRAM1__710: u32,
    pub SRAM2__710: u32,
    pub SRAM0__711: u32,
    pub SRAM1__711: u32,
    pub SRAM2__711: u32,
    pub SRAM0__712: u32,
    pub SRAM1__712: u32,
    pub SRAM2__712: u32,
    pub SRAM0__713: u32,
    pub SRAM1__713: u32,
    pub SRAM2__713: u32,
    pub SRAM0__714: u32,
    pub SRAM1__714: u32,
    pub SRAM2__714: u32,
    pub SRAM0__715: u32,
    pub SRAM1__715: u32,
    pub SRAM2__715: u32,
    pub SRAM0__716: u32,
    pub SRAM1__716: u32,
    pub SRAM2__716: u32,
    pub SRAM0__717: u32,
    pub SRAM1__717: u32,
    pub SRAM2__717: u32,
    pub SRAM0__718: u32,
    pub SRAM1__718: u32,
    pub SRAM2__718: u32,
    pub SRAM0__719: u32,
    pub SRAM1__719: u32,
    pub SRAM2__719: u32,
    pub SRAM0__720: u32,
    pub SRAM1__720: u32,
    pub SRAM2__720: u32,
    pub SRAM0__721: u32,
    pub SRAM1__721: u32,
    pub SRAM2__721: u32,
    pub SRAM0__722: u32,
    pub SRAM1__722: u32,
    pub SRAM2__722: u32,
    pub SRAM0__723: u32,
    pub SRAM1__723: u32,
    pub SRAM2__723: u32,
    pub SRAM0__724: u32,
    pub SRAM1__724: u32,
    pub SRAM2__724: u32,
    pub SRAM0__725: u32,
    pub SRAM1__725: u32,
    pub SRAM2__725: u32,
    pub SRAM0__726: u32,
    pub SRAM1__726: u32,
    pub SRAM2__726: u32,
    pub SRAM0__727: u32,
    pub SRAM1__727: u32,
    pub SRAM2__727: u32,
    pub SRAM0__728: u32,
    pub SRAM1__728: u32,
    pub SRAM2__728: u32,
    pub SRAM0__729: u32,
    pub SRAM1__729: u32,
    pub SRAM2__729: u32,
    pub SRAM0__730: u32,
    pub SRAM1__730: u32,
    pub SRAM2__730: u32,
    pub SRAM0__731: u32,
    pub SRAM1__731: u32,
    pub SRAM2__731: u32,
    pub SRAM0__732: u32,
    pub SRAM1__732: u32,
    pub SRAM2__732: u32,
    pub SRAM0__733: u32,
    pub SRAM1__733: u32,
    pub SRAM2__733: u32,
    pub SRAM0__734: u32,
    pub SRAM1__734: u32,
    pub SRAM2__734: u32,
    pub SRAM0__735: u32,
    pub SRAM1__735: u32,
    pub SRAM2__735: u32,
    pub SRAM0__736: u32,
    pub SRAM1__736: u32,
    pub SRAM2__736: u32,
    pub SRAM0__737: u32,
    pub SRAM1__737: u32,
    pub SRAM2__737: u32,
    pub SRAM0__738: u32,
    pub SRAM1__738: u32,
    pub SRAM2__738: u32,
    pub SRAM0__739: u32,
    pub SRAM1__739: u32,
    pub SRAM2__739: u32,
    pub SRAM0__740: u32,
    pub SRAM1__740: u32,
    pub SRAM2__740: u32,
    pub SRAM0__741: u32,
    pub SRAM1__741: u32,
    pub SRAM2__741: u32,
    pub SRAM0__742: u32,
    pub SRAM1__742: u32,
    pub SRAM2__742: u32,
    pub SRAM0__743: u32,
    pub SRAM1__743: u32,
    pub SRAM2__743: u32,
    pub SRAM0__744: u32,
    pub SRAM1__744: u32,
    pub SRAM2__744: u32,
    pub SRAM0__745: u32,
    pub SRAM1__745: u32,
    pub SRAM2__745: u32,
    pub SRAM0__746: u32,
    pub SRAM1__746: u32,
    pub SRAM2__746: u32,
    pub SRAM0__747: u32,
    pub SRAM1__747: u32,
    pub SRAM2__747: u32,
    pub SRAM0__748: u32,
    pub SRAM1__748: u32,
    pub SRAM2__748: u32,
    pub SRAM0__749: u32,
    pub SRAM1__749: u32,
    pub SRAM2__749: u32,
    pub SRAM0__750: u32,
    pub SRAM1__750: u32,
    pub SRAM2__750: u32,
    pub SRAM0__751: u32,
    pub SRAM1__751: u32,
    pub SRAM2__751: u32,
    pub SRAM0__752: u32,
    pub SRAM1__752: u32,
    pub SRAM2__752: u32,
    pub SRAM0__753: u32,
    pub SRAM1__753: u32,
    pub SRAM2__753: u32,
    pub SRAM0__754: u32,
    pub SRAM1__754: u32,
    pub SRAM2__754: u32,
    pub SRAM0__755: u32,
    pub SRAM1__755: u32,
    pub SRAM2__755: u32,
    pub SRAM0__756: u32,
    pub SRAM1__756: u32,
    pub SRAM2__756: u32,
    pub SRAM0__757: u32,
    pub SRAM1__757: u32,
    pub SRAM2__757: u32,
    pub SRAM0__758: u32,
    pub SRAM1__758: u32,
    pub SRAM2__758: u32,
    pub SRAM0__759: u32,
    pub SRAM1__759: u32,
    pub SRAM2__759: u32,
    pub SRAM0__760: u32,
    pub SRAM1__760: u32,
    pub SRAM2__760: u32,
    pub SRAM0__761: u32,
    pub SRAM1__761: u32,
    pub SRAM2__761: u32,
    pub SRAM0__762: u32,
    pub SRAM1__762: u32,
    pub SRAM2__762: u32,
    pub SRAM0__763: u32,
    pub SRAM1__763: u32,
    pub SRAM2__763: u32,
    pub SRAM0__764: u32,
    pub SRAM1__764: u32,
    pub SRAM2__764: u32,
    pub SRAM0__765: u32,
    pub SRAM1__765: u32,
    pub SRAM2__765: u32,
    pub SRAM0__766: u32,
    pub SRAM1__766: u32,
    pub SRAM2__766: u32,
    pub SRAM0__767: u32,
    pub SRAM1__767: u32,
    pub SRAM2__767: u32,
    pub SRAM0__768: u32,
    pub SRAM1__768: u32,
    pub SRAM2__768: u32,
    pub SRAM0__769: u32,
    pub SRAM1__769: u32,
    pub SRAM2__769: u32,
    pub SRAM0__770: u32,
    pub SRAM1__770: u32,
    pub SRAM2__770: u32,
    pub SRAM0__771: u32,
    pub SRAM1__771: u32,
    pub SRAM2__771: u32,
    pub SRAM0__772: u32,
    pub SRAM1__772: u32,
    pub SRAM2__772: u32,
    pub SRAM0__773: u32,
    pub SRAM1__773: u32,
    pub SRAM2__773: u32,
    pub SRAM0__774: u32,
    pub SRAM1__774: u32,
    pub SRAM2__774: u32,
    pub SRAM0__775: u32,
    pub SRAM1__775: u32,
    pub SRAM2__775: u32,
    pub SRAM0__776: u32,
    pub SRAM1__776: u32,
    pub SRAM2__776: u32,
    pub SRAM0__777: u32,
    pub SRAM1__777: u32,
    pub SRAM2__777: u32,
    pub SRAM0__778: u32,
    pub SRAM1__778: u32,
    pub SRAM2__778: u32,
    pub SRAM0__779: u32,
    pub SRAM1__779: u32,
    pub SRAM2__779: u32,
    pub SRAM0__780: u32,
    pub SRAM1__780: u32,
    pub SRAM2__780: u32,
    pub SRAM0__781: u32,
    pub SRAM1__781: u32,
    pub SRAM2__781: u32,
    pub SRAM0__782: u32,
    pub SRAM1__782: u32,
    pub SRAM2__782: u32,
    pub SRAM0__783: u32,
    pub SRAM1__783: u32,
    pub SRAM2__783: u32,
    pub SRAM0__784: u32,
    pub SRAM1__784: u32,
    pub SRAM2__784: u32,
    pub SRAM0__785: u32,
    pub SRAM1__785: u32,
    pub SRAM2__785: u32,
    pub SRAM0__786: u32,
    pub SRAM1__786: u32,
    pub SRAM2__786: u32,
    pub SRAM0__787: u32,
    pub SRAM1__787: u32,
    pub SRAM2__787: u32,
    pub SRAM0__788: u32,
    pub SRAM1__788: u32,
    pub SRAM2__788: u32,
    pub SRAM0__789: u32,
    pub SRAM1__789: u32,
    pub SRAM2__789: u32,
    pub SRAM0__790: u32,
    pub SRAM1__790: u32,
    pub SRAM2__790: u32,
    pub SRAM0__791: u32,
    pub SRAM1__791: u32,
    pub SRAM2__791: u32,
    pub SRAM0__792: u32,
    pub SRAM1__792: u32,
    pub SRAM2__792: u32,
    pub SRAM0__793: u32,
    pub SRAM1__793: u32,
    pub SRAM2__793: u32,
    pub SRAM0__794: u32,
    pub SRAM1__794: u32,
    pub SRAM2__794: u32,
    pub SRAM0__795: u32,
    pub SRAM1__795: u32,
    pub SRAM2__795: u32,
    pub SRAM0__796: u32,
    pub SRAM1__796: u32,
    pub SRAM2__796: u32,
    pub SRAM0__797: u32,
    pub SRAM1__797: u32,
    pub SRAM2__797: u32,
    pub SRAM0__798: u32,
    pub SRAM1__798: u32,
    pub SRAM2__798: u32,
    pub SRAM0__799: u32,
    pub SRAM1__799: u32,
    pub SRAM2__799: u32,
    pub SRAM0__800: u32,
    pub SRAM1__800: u32,
    pub SRAM2__800: u32,
    pub SRAM0__801: u32,
    pub SRAM1__801: u32,
    pub SRAM2__801: u32,
    pub SRAM0__802: u32,
    pub SRAM1__802: u32,
    pub SRAM2__802: u32,
    pub SRAM0__803: u32,
    pub SRAM1__803: u32,
    pub SRAM2__803: u32,
    pub SRAM0__804: u32,
    pub SRAM1__804: u32,
    pub SRAM2__804: u32,
    pub SRAM0__805: u32,
    pub SRAM1__805: u32,
    pub SRAM2__805: u32,
    pub SRAM0__806: u32,
    pub SRAM1__806: u32,
    pub SRAM2__806: u32,
    pub SRAM0__807: u32,
    pub SRAM1__807: u32,
    pub SRAM2__807: u32,
    pub SRAM0__808: u32,
    pub SRAM1__808: u32,
    pub SRAM2__808: u32,
    pub SRAM0__809: u32,
    pub SRAM1__809: u32,
    pub SRAM2__809: u32,
    pub SRAM0__810: u32,
    pub SRAM1__810: u32,
    pub SRAM2__810: u32,
    pub SRAM0__811: u32,
    pub SRAM1__811: u32,
    pub SRAM2__811: u32,
    pub SRAM0__812: u32,
    pub SRAM1__812: u32,
    pub SRAM2__812: u32,
    pub SRAM0__813: u32,
    pub SRAM1__813: u32,
    pub SRAM2__813: u32,
    pub SRAM0__814: u32,
    pub SRAM1__814: u32,
    pub SRAM2__814: u32,
    pub SRAM0__815: u32,
    pub SRAM1__815: u32,
    pub SRAM2__815: u32,
    pub SRAM0__816: u32,
    pub SRAM1__816: u32,
    pub SRAM2__816: u32,
    pub SRAM0__817: u32,
    pub SRAM1__817: u32,
    pub SRAM2__817: u32,
    pub SRAM0__818: u32,
    pub SRAM1__818: u32,
    pub SRAM2__818: u32,
    pub SRAM0__819: u32,
    pub SRAM1__819: u32,
    pub SRAM2__819: u32,
    pub SRAM0__820: u32,
    pub SRAM1__820: u32,
    pub SRAM2__820: u32,
    pub SRAM0__821: u32,
    pub SRAM1__821: u32,
    pub SRAM2__821: u32,
    pub SRAM0__822: u32,
    pub SRAM1__822: u32,
    pub SRAM2__822: u32,
    pub SRAM0__823: u32,
    pub SRAM1__823: u32,
    pub SRAM2__823: u32,
    pub SRAM0__824: u32,
    pub SRAM1__824: u32,
    pub SRAM2__824: u32,
    pub SRAM0__825: u32,
    pub SRAM1__825: u32,
    pub SRAM2__825: u32,
    pub SRAM0__826: u32,
    pub SRAM1__826: u32,
    pub SRAM2__826: u32,
    pub SRAM0__827: u32,
    pub SRAM1__827: u32,
    pub SRAM2__827: u32,
    pub SRAM0__828: u32,
    pub SRAM1__828: u32,
    pub SRAM2__828: u32,
    pub SRAM0__829: u32,
    pub SRAM1__829: u32,
    pub SRAM2__829: u32,
    pub SRAM0__830: u32,
    pub SRAM1__830: u32,
    pub SRAM2__830: u32,
    pub SRAM0__831: u32,
    pub SRAM1__831: u32,
    pub SRAM2__831: u32,
    pub SRAM0__832: u32,
    pub SRAM1__832: u32,
    pub SRAM2__832: u32,
    pub SRAM0__833: u32,
    pub SRAM1__833: u32,
    pub SRAM2__833: u32,
    pub SRAM0__834: u32,
    pub SRAM1__834: u32,
    pub SRAM2__834: u32,
    pub SRAM0__835: u32,
    pub SRAM1__835: u32,
    pub SRAM2__835: u32,
    pub SRAM0__836: u32,
    pub SRAM1__836: u32,
    pub SRAM2__836: u32,
    pub SRAM0__837: u32,
    pub SRAM1__837: u32,
    pub SRAM2__837: u32,
    pub SRAM0__838: u32,
    pub SRAM1__838: u32,
    pub SRAM2__838: u32,
    pub SRAM0__839: u32,
    pub SRAM1__839: u32,
    pub SRAM2__839: u32,
    pub SRAM0__840: u32,
    pub SRAM1__840: u32,
    pub SRAM2__840: u32,
    pub SRAM0__841: u32,
    pub SRAM1__841: u32,
    pub SRAM2__841: u32,
    pub SRAM0__842: u32,
    pub SRAM1__842: u32,
    pub SRAM2__842: u32,
    pub SRAM0__843: u32,
    pub SRAM1__843: u32,
    pub SRAM2__843: u32,
    pub SRAM0__844: u32,
    pub SRAM1__844: u32,
    pub SRAM2__844: u32,
    pub SRAM0__845: u32,
    pub SRAM1__845: u32,
    pub SRAM2__845: u32,
    pub SRAM0__846: u32,
    pub SRAM1__846: u32,
    pub SRAM2__846: u32,
    pub SRAM0__847: u32,
    pub SRAM1__847: u32,
    pub SRAM2__847: u32,
    pub SRAM0__848: u32,
    pub SRAM1__848: u32,
    pub SRAM2__848: u32,
    pub SRAM0__849: u32,
    pub SRAM1__849: u32,
    pub SRAM2__849: u32,
    pub SRAM0__850: u32,
    pub SRAM1__850: u32,
    pub SRAM2__850: u32,
    pub SRAM0__851: u32,
    pub SRAM1__851: u32,
    pub SRAM2__851: u32,
    pub SRAM0__852: u32,
    pub SRAM1__852: u32,
    pub SRAM2__852: u32,
    pub SRAM0__853: u32,
    pub SRAM1__853: u32,
    pub SRAM2__853: u32,
    pub SRAM0__854: u32,
    pub SRAM1__854: u32,
    pub SRAM2__854: u32,
    pub SRAM0__855: u32,
    pub SRAM1__855: u32,
    pub SRAM2__855: u32,
    pub SRAM0__856: u32,
    pub SRAM1__856: u32,
    pub SRAM2__856: u32,
    pub SRAM0__857: u32,
    pub SRAM1__857: u32,
    pub SRAM2__857: u32,
    pub SRAM0__858: u32,
    pub SRAM1__858: u32,
    pub SRAM2__858: u32,
    pub SRAM0__859: u32,
    pub SRAM1__859: u32,
    pub SRAM2__859: u32,
    pub SRAM0__860: u32,
    pub SRAM1__860: u32,
    pub SRAM2__860: u32,
    pub SRAM0__861: u32,
    pub SRAM1__861: u32,
    pub SRAM2__861: u32,
    pub SRAM0__862: u32,
    pub SRAM1__862: u32,
    pub SRAM2__862: u32,
    pub SRAM0__863: u32,
    pub SRAM1__863: u32,
    pub SRAM2__863: u32,
    pub SRAM0__864: u32,
    pub SRAM1__864: u32,
    pub SRAM2__864: u32,
    pub SRAM0__865: u32,
    pub SRAM1__865: u32,
    pub SRAM2__865: u32,
    pub SRAM0__866: u32,
    pub SRAM1__866: u32,
    pub SRAM2__866: u32,
    pub SRAM0__867: u32,
    pub SRAM1__867: u32,
    pub SRAM2__867: u32,
    pub SRAM0__868: u32,
    pub SRAM1__868: u32,
    pub SRAM2__868: u32,
    pub SRAM0__869: u32,
    pub SRAM1__869: u32,
    pub SRAM2__869: u32,
    pub SRAM0__870: u32,
    pub SRAM1__870: u32,
    pub SRAM2__870: u32,
    pub SRAM0__871: u32,
    pub SRAM1__871: u32,
    pub SRAM2__871: u32,
    pub SRAM0__872: u32,
    pub SRAM1__872: u32,
    pub SRAM2__872: u32,
    pub SRAM0__873: u32,
    pub SRAM1__873: u32,
    pub SRAM2__873: u32,
    pub SRAM0__874: u32,
    pub SRAM1__874: u32,
    pub SRAM2__874: u32,
    pub SRAM0__875: u32,
    pub SRAM1__875: u32,
    pub SRAM2__875: u32,
    pub SRAM0__876: u32,
    pub SRAM1__876: u32,
    pub SRAM2__876: u32,
    pub SRAM0__877: u32,
    pub SRAM1__877: u32,
    pub SRAM2__877: u32,
    pub SRAM0__878: u32,
    pub SRAM1__878: u32,
    pub SRAM2__878: u32,
    pub SRAM0__879: u32,
    pub SRAM1__879: u32,
    pub SRAM2__879: u32,
    pub SRAM0__880: u32,
    pub SRAM1__880: u32,
    pub SRAM2__880: u32,
    pub SRAM0__881: u32,
    pub SRAM1__881: u32,
    pub SRAM2__881: u32,
    pub SRAM0__882: u32,
    pub SRAM1__882: u32,
    pub SRAM2__882: u32,
    pub SRAM0__883: u32,
    pub SRAM1__883: u32,
    pub SRAM2__883: u32,
    pub SRAM0__884: u32,
    pub SRAM1__884: u32,
    pub SRAM2__884: u32,
    pub SRAM0__885: u32,
    pub SRAM1__885: u32,
    pub SRAM2__885: u32,
    pub SRAM0__886: u32,
    pub SRAM1__886: u32,
    pub SRAM2__886: u32,
    pub SRAM0__887: u32,
    pub SRAM1__887: u32,
    pub SRAM2__887: u32,
    pub SRAM0__888: u32,
    pub SRAM1__888: u32,
    pub SRAM2__888: u32,
    pub SRAM0__889: u32,
    pub SRAM1__889: u32,
    pub SRAM2__889: u32,
    pub SRAM0__890: u32,
    pub SRAM1__890: u32,
    pub SRAM2__890: u32,
    pub SRAM0__891: u32,
    pub SRAM1__891: u32,
    pub SRAM2__891: u32,
    pub SRAM0__892: u32,
    pub SRAM1__892: u32,
    pub SRAM2__892: u32,
    pub SRAM0__893: u32,
    pub SRAM1__893: u32,
    pub SRAM2__893: u32,
    pub SRAM0__894: u32,
    pub SRAM1__894: u32,
    pub SRAM2__894: u32,
    pub SRAM0__895: u32,
    pub SRAM1__895: u32,
    pub SRAM2__895: u32,
    pub SRAM0__896: u32,
    pub SRAM1__896: u32,
    pub SRAM2__896: u32,
    pub SRAM0__897: u32,
    pub SRAM1__897: u32,
    pub SRAM2__897: u32,
    pub SRAM0__898: u32,
    pub SRAM1__898: u32,
    pub SRAM2__898: u32,
    pub SRAM0__899: u32,
    pub SRAM1__899: u32,
    pub SRAM2__899: u32,
    pub SRAM0__900: u32,
    pub SRAM1__900: u32,
    pub SRAM2__900: u32,
    pub SRAM0__901: u32,
    pub SRAM1__901: u32,
    pub SRAM2__901: u32,
    pub SRAM0__902: u32,
    pub SRAM1__902: u32,
    pub SRAM2__902: u32,
    pub SRAM0__903: u32,
    pub SRAM1__903: u32,
    pub SRAM2__903: u32,
    pub SRAM0__904: u32,
    pub SRAM1__904: u32,
    pub SRAM2__904: u32,
    pub SRAM0__905: u32,
    pub SRAM1__905: u32,
    pub SRAM2__905: u32,
    pub SRAM0__906: u32,
    pub SRAM1__906: u32,
    pub SRAM2__906: u32,
    pub SRAM0__907: u32,
    pub SRAM1__907: u32,
    pub SRAM2__907: u32,
    pub SRAM0__908: u32,
    pub SRAM1__908: u32,
    pub SRAM2__908: u32,
    pub SRAM0__909: u32,
    pub SRAM1__909: u32,
    pub SRAM2__909: u32,
    pub SRAM0__910: u32,
    pub SRAM1__910: u32,
    pub SRAM2__910: u32,
    pub SRAM0__911: u32,
    pub SRAM1__911: u32,
    pub SRAM2__911: u32,
    pub SRAM0__912: u32,
    pub SRAM1__912: u32,
    pub SRAM2__912: u32,
    pub SRAM0__913: u32,
    pub SRAM1__913: u32,
    pub SRAM2__913: u32,
    pub SRAM0__914: u32,
    pub SRAM1__914: u32,
    pub SRAM2__914: u32,
    pub SRAM0__915: u32,
    pub SRAM1__915: u32,
    pub SRAM2__915: u32,
    pub SRAM0__916: u32,
    pub SRAM1__916: u32,
    pub SRAM2__916: u32,
    pub SRAM0__917: u32,
    pub SRAM1__917: u32,
    pub SRAM2__917: u32,
    pub SRAM0__918: u32,
    pub SRAM1__918: u32,
    pub SRAM2__918: u32,
    pub SRAM0__919: u32,
    pub SRAM1__919: u32,
    pub SRAM2__919: u32,
    pub SRAM0__920: u32,
    pub SRAM1__920: u32,
    pub SRAM2__920: u32,
    pub SRAM0__921: u32,
    pub SRAM1__921: u32,
    pub SRAM2__921: u32,
    pub SRAM0__922: u32,
    pub SRAM1__922: u32,
    pub SRAM2__922: u32,
    pub SRAM0__923: u32,
    pub SRAM1__923: u32,
    pub SRAM2__923: u32,
    pub SRAM0__924: u32,
    pub SRAM1__924: u32,
    pub SRAM2__924: u32,
    pub SRAM0__925: u32,
    pub SRAM1__925: u32,
    pub SRAM2__925: u32,
    pub SRAM0__926: u32,
    pub SRAM1__926: u32,
    pub SRAM2__926: u32,
    pub SRAM0__927: u32,
    pub SRAM1__927: u32,
    pub SRAM2__927: u32,
    pub SRAM0__928: u32,
    pub SRAM1__928: u32,
    pub SRAM2__928: u32,
    pub SRAM0__929: u32,
    pub SRAM1__929: u32,
    pub SRAM2__929: u32,
    pub SRAM0__930: u32,
    pub SRAM1__930: u32,
    pub SRAM2__930: u32,
    pub SRAM0__931: u32,
    pub SRAM1__931: u32,
    pub SRAM2__931: u32,
    pub SRAM0__932: u32,
    pub SRAM1__932: u32,
    pub SRAM2__932: u32,
    pub SRAM0__933: u32,
    pub SRAM1__933: u32,
    pub SRAM2__933: u32,
    pub SRAM0__934: u32,
    pub SRAM1__934: u32,
    pub SRAM2__934: u32,
    pub SRAM0__935: u32,
    pub SRAM1__935: u32,
    pub SRAM2__935: u32,
    pub SRAM0__936: u32,
    pub SRAM1__936: u32,
    pub SRAM2__936: u32,
    pub SRAM0__937: u32,
    pub SRAM1__937: u32,
    pub SRAM2__937: u32,
    pub SRAM0__938: u32,
    pub SRAM1__938: u32,
    pub SRAM2__938: u32,
    pub SRAM0__939: u32,
    pub SRAM1__939: u32,
    pub SRAM2__939: u32,
    pub SRAM0__940: u32,
    pub SRAM1__940: u32,
    pub SRAM2__940: u32,
    pub SRAM0__941: u32,
    pub SRAM1__941: u32,
    pub SRAM2__941: u32,
    pub SRAM0__942: u32,
    pub SRAM1__942: u32,
    pub SRAM2__942: u32,
    pub SRAM0__943: u32,
    pub SRAM1__943: u32,
    pub SRAM2__943: u32,
    pub SRAM0__944: u32,
    pub SRAM1__944: u32,
    pub SRAM2__944: u32,
    pub SRAM0__945: u32,
    pub SRAM1__945: u32,
    pub SRAM2__945: u32,
    pub SRAM0__946: u32,
    pub SRAM1__946: u32,
    pub SRAM2__946: u32,
    pub SRAM0__947: u32,
    pub SRAM1__947: u32,
    pub SRAM2__947: u32,
    pub SRAM0__948: u32,
    pub SRAM1__948: u32,
    pub SRAM2__948: u32,
    pub SRAM0__949: u32,
    pub SRAM1__949: u32,
    pub SRAM2__949: u32,
    pub SRAM0__950: u32,
    pub SRAM1__950: u32,
    pub SRAM2__950: u32,
    pub SRAM0__951: u32,
    pub SRAM1__951: u32,
    pub SRAM2__951: u32,
    pub SRAM0__952: u32,
    pub SRAM1__952: u32,
    pub SRAM2__952: u32,
    pub SRAM0__953: u32,
    pub SRAM1__953: u32,
    pub SRAM2__953: u32,
    pub SRAM0__954: u32,
    pub SRAM1__954: u32,
    pub SRAM2__954: u32,
    pub SRAM0__955: u32,
    pub SRAM1__955: u32,
    pub SRAM2__955: u32,
    pub SRAM0__956: u32,
    pub SRAM1__956: u32,
    pub SRAM2__956: u32,
    pub SRAM0__957: u32,
    pub SRAM1__957: u32,
    pub SRAM2__957: u32,
    pub SRAM0__958: u32,
    pub SRAM1__958: u32,
    pub SRAM2__958: u32,
    pub SRAM0__959: u32,
    pub SRAM1__959: u32,
    pub SRAM2__959: u32,
    pub SRAM0__960: u32,
    pub SRAM1__960: u32,
    pub SRAM2__960: u32,
    pub SRAM0__961: u32,
    pub SRAM1__961: u32,
    pub SRAM2__961: u32,
    pub SRAM0__962: u32,
    pub SRAM1__962: u32,
    pub SRAM2__962: u32,
    pub SRAM0__963: u32,
    pub SRAM1__963: u32,
    pub SRAM2__963: u32,
    pub SRAM0__964: u32,
    pub SRAM1__964: u32,
    pub SRAM2__964: u32,
    pub SRAM0__965: u32,
    pub SRAM1__965: u32,
    pub SRAM2__965: u32,
    pub SRAM0__966: u32,
    pub SRAM1__966: u32,
    pub SRAM2__966: u32,
    pub SRAM0__967: u32,
    pub SRAM1__967: u32,
    pub SRAM2__967: u32,
    pub SRAM0__968: u32,
    pub SRAM1__968: u32,
    pub SRAM2__968: u32,
    pub SRAM0__969: u32,
    pub SRAM1__969: u32,
    pub SRAM2__969: u32,
    pub SRAM0__970: u32,
    pub SRAM1__970: u32,
    pub SRAM2__970: u32,
    pub SRAM0__971: u32,
    pub SRAM1__971: u32,
    pub SRAM2__971: u32,
    pub SRAM0__972: u32,
    pub SRAM1__972: u32,
    pub SRAM2__972: u32,
    pub SRAM0__973: u32,
    pub SRAM1__973: u32,
    pub SRAM2__973: u32,
    pub SRAM0__974: u32,
    pub SRAM1__974: u32,
    pub SRAM2__974: u32,
    pub SRAM0__975: u32,
    pub SRAM1__975: u32,
    pub SRAM2__975: u32,
    pub SRAM0__976: u32,
    pub SRAM1__976: u32,
    pub SRAM2__976: u32,
    pub SRAM0__977: u32,
    pub SRAM1__977: u32,
    pub SRAM2__977: u32,
    pub SRAM0__978: u32,
    pub SRAM1__978: u32,
    pub SRAM2__978: u32,
    pub SRAM0__979: u32,
    pub SRAM1__979: u32,
    pub SRAM2__979: u32,
    pub SRAM0__980: u32,
    pub SRAM1__980: u32,
    pub SRAM2__980: u32,
    pub SRAM0__981: u32,
    pub SRAM1__981: u32,
    pub SRAM2__981: u32,
    pub SRAM0__982: u32,
    pub SRAM1__982: u32,
    pub SRAM2__982: u32,
    pub SRAM0__983: u32,
    pub SRAM1__983: u32,
    pub SRAM2__983: u32,
    pub SRAM0__984: u32,
    pub SRAM1__984: u32,
    pub SRAM2__984: u32,
    pub SRAM0__985: u32,
    pub SRAM1__985: u32,
    pub SRAM2__985: u32,
    pub SRAM0__986: u32,
    pub SRAM1__986: u32,
    pub SRAM2__986: u32,
    pub SRAM0__987: u32,
    pub SRAM1__987: u32,
    pub SRAM2__987: u32,
    pub SRAM0__988: u32,
    pub SRAM1__988: u32,
    pub SRAM2__988: u32,
    pub SRAM0__989: u32,
    pub SRAM1__989: u32,
    pub SRAM2__989: u32,
    pub SRAM0__990: u32,
    pub SRAM1__990: u32,
    pub SRAM2__990: u32,
    pub SRAM0__991: u32,
    pub SRAM1__991: u32,
    pub SRAM2__991: u32,
    pub SRAM0__992: u32,
    pub SRAM1__992: u32,
    pub SRAM2__992: u32,
    pub SRAM0__993: u32,
    pub SRAM1__993: u32,
    pub SRAM2__993: u32,
    pub SRAM0__994: u32,
    pub SRAM1__994: u32,
    pub SRAM2__994: u32,
    pub SRAM0__995: u32,
    pub SRAM1__995: u32,
    pub SRAM2__995: u32,
    pub SRAM0__996: u32,
    pub SRAM1__996: u32,
    pub SRAM2__996: u32,
    pub SRAM0__997: u32,
    pub SRAM1__997: u32,
    pub SRAM2__997: u32,
    pub SRAM0__998: u32,
    pub SRAM1__998: u32,
    pub SRAM2__998: u32,
    pub SRAM0__999: u32,
    pub SRAM1__999: u32,
    pub SRAM2__999: u32,
    pub SRAM0__1000: u32,
    pub SRAM1__1000: u32,
    pub SRAM2__1000: u32,
    pub SRAM0__1001: u32,
    pub SRAM1__1001: u32,
    pub SRAM2__1001: u32,
    pub SRAM0__1002: u32,
    pub SRAM1__1002: u32,
    pub SRAM2__1002: u32,
    pub SRAM0__1003: u32,
    pub SRAM1__1003: u32,
    pub SRAM2__1003: u32,
    pub SRAM0__1004: u32,
    pub SRAM1__1004: u32,
    pub SRAM2__1004: u32,
    pub SRAM0__1005: u32,
    pub SRAM1__1005: u32,
    pub SRAM2__1005: u32,
    pub SRAM0__1006: u32,
    pub SRAM1__1006: u32,
    pub SRAM2__1006: u32,
    pub SRAM0__1007: u32,
    pub SRAM1__1007: u32,
    pub SRAM2__1007: u32,
    pub SRAM0__1008: u32,
    pub SRAM1__1008: u32,
    pub SRAM2__1008: u32,
    pub SRAM0__1009: u32,
    pub SRAM1__1009: u32,
    pub SRAM2__1009: u32,
    pub SRAM0__1010: u32,
    pub SRAM1__1010: u32,
    pub SRAM2__1010: u32,
    pub SRAM0__1011: u32,
    pub SRAM1__1011: u32,
    pub SRAM2__1011: u32,
    pub SRAM0__1012: u32,
    pub SRAM1__1012: u32,
    pub SRAM2__1012: u32,
    pub SRAM0__1013: u32,
    pub SRAM1__1013: u32,
    pub SRAM2__1013: u32,
    pub SRAM0__1014: u32,
    pub SRAM1__1014: u32,
    pub SRAM2__1014: u32,
    pub SRAM0__1015: u32,
    pub SRAM1__1015: u32,
    pub SRAM2__1015: u32,
    pub SRAM0__1016: u32,
    pub SRAM1__1016: u32,
    pub SRAM2__1016: u32,
    pub SRAM0__1017: u32,
    pub SRAM1__1017: u32,
    pub SRAM2__1017: u32,
    pub SRAM0__1018: u32,
    pub SRAM1__1018: u32,
    pub SRAM2__1018: u32,
    pub SRAM0__1019: u32,
    pub SRAM1__1019: u32,
    pub SRAM2__1019: u32,
    pub SRAM0__1020: u32,
    pub SRAM1__1020: u32,
    pub SRAM2__1020: u32,
    pub SRAM0__1021: u32,
    pub SRAM1__1021: u32,
    pub SRAM2__1021: u32,
    pub SRAM0__1022: u32,
    pub SRAM1__1022: u32,
    pub SRAM2__1022: u32,
    pub SRAM0__1023: u32,
    pub SRAM1__1023: u32,
    pub SRAM2__1023: u32,
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
