#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! XRDC2
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Module Control Register
pub mod MCR {

    /// Global Valid MDAC
    pub mod GVLDM {
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

            /// 0b0: MDACs are disabled.
            pub const disabled: u32 = 0b0;

            /// 0b1: MDACs are enabled.
            pub const enabled: u32 = 0b1;
        }
    }

    /// Global Valid Access Control
    pub mod GVLDC {
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

            /// 0b0: Access controls are disabled, XRDC2 allows all transactions.
            pub const disabled: u32 = 0b0;

            /// 0b1: Access controls are enabled.
            pub const enabled: u32 = 0b1;
        }
    }

    /// Global Configuration Lock
    pub mod GCL {
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

            /// 0b00: Lock disabled, registers can be written by any domain.
            pub const disabled_00: u32 = 0b00;

            /// 0b01: Lock disabled until the next reset, registers can be written by any domain.
            pub const disabled_01: u32 = 0b01;

            /// 0b10: Lock enabled, only the global configuration lock owner (SR\[GCLO\]) can write to registers.
            pub const enabled_10: u32 = 0b10;

            /// 0b11: Lock enabled, all registers are read only until the next reset.
            pub const enabled_11: u32 = 0b11;
        }
    }
}

/// Status Register
pub mod SR {

    /// Domain Identifier Number
    pub mod DIN {
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

    /// Hardware Revision Level
    pub mod HRL {
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

    /// Global Configuration Lock Owner
    pub mod GCLO {
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

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_0 {

    /// Domain "x" access control policy
    pub mod D0ACP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D1ACP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D2ACP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D3ACP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D4ACP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D5ACP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (3 bits: 0b111 << 15)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D6ACP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D7ACP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (3 bits: 0b111 << 21)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Exclusive Access Lock Owner
    pub mod EALO {
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
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_0 {

    /// Domain "x" access control policy
    pub mod D8ACP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D9ACP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D10ACP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D11ACP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D12ACP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D13ACP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (3 bits: 0b111 << 15)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D14ACP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Domain "x" access control policy
    pub mod D15ACP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (3 bits: 0b111 << 21)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Exclusive Access Lock
    pub mod EAL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Lock disabled.
            pub const disabled_00: u32 = 0b00;

            /// 0b01: Lock disabled until next reset.
            pub const disabled_01: u32 = 0b01;

            /// 0b10: Lock enabled, lock state = available.
            pub const enabled_10: u32 = 0b10;

            /// 0b11: Lock enabled, lock state = not available.
            pub const enabled_11: u32 = 0b11;
        }
    }

    /// Descriptor Lock
    pub mod DL2 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Lock disabled, descriptor registers can be written.
            pub const disabled_00: u32 = 0b00;

            /// 0b01: Lock disabled until the next reset, descriptor registers can be written.
            pub const disabled_01: u32 = 0b01;

            /// 0b10: Lock enabled, only domain "x" can only update the DxACP field; no other fields can be written.
            pub const enabled_10: u32 = 0b10;

            /// 0b11: Lock enabled, descriptor registers are read-only until the next reset.
            pub const enabled_11: u32 = 0b11;
        }
    }

    /// Valid
    pub mod VLD {
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

            /// 0b0: The MSAC assignment is invalid.
            pub const invalid: u32 = 0b0;

            /// 0b1: The MSAC assignment is valid.
            pub const valid: u32 = 0b1;
        }
    }
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_1 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_1 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_2 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_2 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_3 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_3 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_4 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_4 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_5 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_5 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_6 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_6 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_7 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_7 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_8 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_8 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_9 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_9 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_10 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_10 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_11 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_11 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_12 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_12 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_13 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_13 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_14 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_14 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_15 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_15 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_16 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_16 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_17 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_17 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_18 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_18 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_19 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_19 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_20 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_20 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_21 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_21 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_22 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_22 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_23 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_23 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_24 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_24 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_25 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_25 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_26 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_26 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_27 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_27 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_28 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_28 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_29 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_29 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_30 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_30 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_31 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_31 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_32 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_32 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_33 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_33 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_34 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_34 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_35 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_35 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_36 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_36 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_37 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_37 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_38 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_38 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_39 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_39 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_40 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_40 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_41 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_41 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_42 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_42 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_43 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_43 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_44 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_44 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_45 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_45 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_46 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_46 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_47 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_47 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_48 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_48 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_49 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_49 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_50 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_50 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_51 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_51 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_52 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_52 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_53 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_53 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_54 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_54 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_55 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_55 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_56 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_56 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_57 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_57 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_58 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_58 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_59 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_59 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_60 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_60 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_61 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_61 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_62 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_62 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_63 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_63 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_64 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_64 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_65 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_65 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_66 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_66 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_67 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_67 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_68 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_68 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_69 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_69 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_70 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_70 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_71 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_71 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_72 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_72 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_73 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_73 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_74 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_74 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_75 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_75 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_76 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_76 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_77 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_77 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_78 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_78 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_79 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_79 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_80 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_80 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_81 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_81 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_82 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_82 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_83 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_83 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_84 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_84 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_85 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_85 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_86 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_86 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_87 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_87 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_88 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_88 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_89 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_89 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_90 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_90 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_91 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_91 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_92 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_92 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_93 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_93 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_94 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_94 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_95 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_95 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_96 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_96 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_97 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_97 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_98 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_98 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_99 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_99 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_100 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_100 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_101 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_101 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_102 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_102 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_103 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_103 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_104 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_104 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_105 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_105 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_106 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_106 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_107 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_107 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_108 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_108 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_109 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_109 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_110 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_110 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_111 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_111 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_112 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_112 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_113 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_113 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_114 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_114 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_115 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_115 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_116 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_116 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_117 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_117 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_118 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_118 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_119 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_119 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_120 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_120 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_121 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_121 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_122 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_122 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_123 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_123 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_124 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_124 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_125 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_125 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_126 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_126 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W0_127 {
    pub use super::MSC_MSAC_W0_0::D0ACP;
    pub use super::MSC_MSAC_W0_0::D1ACP;
    pub use super::MSC_MSAC_W0_0::D2ACP;
    pub use super::MSC_MSAC_W0_0::D3ACP;
    pub use super::MSC_MSAC_W0_0::D4ACP;
    pub use super::MSC_MSAC_W0_0::D5ACP;
    pub use super::MSC_MSAC_W0_0::D6ACP;
    pub use super::MSC_MSAC_W0_0::D7ACP;
    pub use super::MSC_MSAC_W0_0::EALO;
}

/// Memory Slot Access Control
pub mod MSC_MSAC_W1_127 {
    pub use super::MSC_MSAC_W1_0::D10ACP;
    pub use super::MSC_MSAC_W1_0::D11ACP;
    pub use super::MSC_MSAC_W1_0::D12ACP;
    pub use super::MSC_MSAC_W1_0::D13ACP;
    pub use super::MSC_MSAC_W1_0::D14ACP;
    pub use super::MSC_MSAC_W1_0::D15ACP;
    pub use super::MSC_MSAC_W1_0::D8ACP;
    pub use super::MSC_MSAC_W1_0::D9ACP;
    pub use super::MSC_MSAC_W1_0::DL2;
    pub use super::MSC_MSAC_W1_0::EAL;
    pub use super::MSC_MSAC_W1_0::VLD;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Module Control Register
    pub MCR: RWRegister<u32>,

    /// Status Register
    pub SR: RORegister<u32>,

    _reserved1: [u32; 1022],

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_0: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_0: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_1: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_1: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_2: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_2: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_3: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_3: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_4: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_4: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_5: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_5: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_6: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_6: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_7: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_7: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_8: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_8: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_9: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_9: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_10: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_10: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_11: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_11: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_12: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_12: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_13: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_13: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_14: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_14: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_15: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_15: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_16: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_16: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_17: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_17: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_18: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_18: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_19: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_19: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_20: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_20: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_21: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_21: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_22: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_22: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_23: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_23: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_24: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_24: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_25: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_25: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_26: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_26: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_27: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_27: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_28: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_28: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_29: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_29: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_30: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_30: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_31: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_31: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_32: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_32: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_33: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_33: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_34: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_34: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_35: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_35: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_36: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_36: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_37: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_37: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_38: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_38: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_39: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_39: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_40: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_40: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_41: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_41: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_42: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_42: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_43: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_43: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_44: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_44: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_45: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_45: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_46: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_46: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_47: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_47: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_48: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_48: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_49: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_49: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_50: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_50: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_51: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_51: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_52: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_52: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_53: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_53: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_54: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_54: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_55: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_55: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_56: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_56: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_57: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_57: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_58: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_58: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_59: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_59: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_60: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_60: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_61: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_61: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_62: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_62: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_63: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_63: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_64: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_64: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_65: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_65: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_66: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_66: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_67: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_67: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_68: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_68: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_69: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_69: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_70: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_70: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_71: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_71: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_72: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_72: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_73: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_73: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_74: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_74: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_75: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_75: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_76: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_76: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_77: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_77: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_78: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_78: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_79: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_79: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_80: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_80: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_81: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_81: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_82: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_82: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_83: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_83: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_84: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_84: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_85: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_85: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_86: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_86: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_87: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_87: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_88: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_88: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_89: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_89: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_90: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_90: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_91: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_91: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_92: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_92: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_93: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_93: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_94: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_94: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_95: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_95: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_96: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_96: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_97: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_97: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_98: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_98: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_99: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_99: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_100: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_100: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_101: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_101: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_102: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_102: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_103: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_103: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_104: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_104: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_105: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_105: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_106: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_106: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_107: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_107: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_108: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_108: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_109: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_109: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_110: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_110: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_111: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_111: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_112: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_112: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_113: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_113: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_114: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_114: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_115: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_115: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_116: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_116: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_117: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_117: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_118: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_118: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_119: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_119: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_120: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_120: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_121: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_121: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_122: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_122: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_123: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_123: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_124: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_124: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_125: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_125: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_126: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_126: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W0_127: RWRegister<u32>,

    /// Memory Slot Access Control
    pub MSC_MSAC_W1_127: RWRegister<u32>,
}
pub struct ResetValues {
    pub MCR: u32,
    pub SR: u32,
    pub MSC_MSAC_W0_0: u32,
    pub MSC_MSAC_W1_0: u32,
    pub MSC_MSAC_W0_1: u32,
    pub MSC_MSAC_W1_1: u32,
    pub MSC_MSAC_W0_2: u32,
    pub MSC_MSAC_W1_2: u32,
    pub MSC_MSAC_W0_3: u32,
    pub MSC_MSAC_W1_3: u32,
    pub MSC_MSAC_W0_4: u32,
    pub MSC_MSAC_W1_4: u32,
    pub MSC_MSAC_W0_5: u32,
    pub MSC_MSAC_W1_5: u32,
    pub MSC_MSAC_W0_6: u32,
    pub MSC_MSAC_W1_6: u32,
    pub MSC_MSAC_W0_7: u32,
    pub MSC_MSAC_W1_7: u32,
    pub MSC_MSAC_W0_8: u32,
    pub MSC_MSAC_W1_8: u32,
    pub MSC_MSAC_W0_9: u32,
    pub MSC_MSAC_W1_9: u32,
    pub MSC_MSAC_W0_10: u32,
    pub MSC_MSAC_W1_10: u32,
    pub MSC_MSAC_W0_11: u32,
    pub MSC_MSAC_W1_11: u32,
    pub MSC_MSAC_W0_12: u32,
    pub MSC_MSAC_W1_12: u32,
    pub MSC_MSAC_W0_13: u32,
    pub MSC_MSAC_W1_13: u32,
    pub MSC_MSAC_W0_14: u32,
    pub MSC_MSAC_W1_14: u32,
    pub MSC_MSAC_W0_15: u32,
    pub MSC_MSAC_W1_15: u32,
    pub MSC_MSAC_W0_16: u32,
    pub MSC_MSAC_W1_16: u32,
    pub MSC_MSAC_W0_17: u32,
    pub MSC_MSAC_W1_17: u32,
    pub MSC_MSAC_W0_18: u32,
    pub MSC_MSAC_W1_18: u32,
    pub MSC_MSAC_W0_19: u32,
    pub MSC_MSAC_W1_19: u32,
    pub MSC_MSAC_W0_20: u32,
    pub MSC_MSAC_W1_20: u32,
    pub MSC_MSAC_W0_21: u32,
    pub MSC_MSAC_W1_21: u32,
    pub MSC_MSAC_W0_22: u32,
    pub MSC_MSAC_W1_22: u32,
    pub MSC_MSAC_W0_23: u32,
    pub MSC_MSAC_W1_23: u32,
    pub MSC_MSAC_W0_24: u32,
    pub MSC_MSAC_W1_24: u32,
    pub MSC_MSAC_W0_25: u32,
    pub MSC_MSAC_W1_25: u32,
    pub MSC_MSAC_W0_26: u32,
    pub MSC_MSAC_W1_26: u32,
    pub MSC_MSAC_W0_27: u32,
    pub MSC_MSAC_W1_27: u32,
    pub MSC_MSAC_W0_28: u32,
    pub MSC_MSAC_W1_28: u32,
    pub MSC_MSAC_W0_29: u32,
    pub MSC_MSAC_W1_29: u32,
    pub MSC_MSAC_W0_30: u32,
    pub MSC_MSAC_W1_30: u32,
    pub MSC_MSAC_W0_31: u32,
    pub MSC_MSAC_W1_31: u32,
    pub MSC_MSAC_W0_32: u32,
    pub MSC_MSAC_W1_32: u32,
    pub MSC_MSAC_W0_33: u32,
    pub MSC_MSAC_W1_33: u32,
    pub MSC_MSAC_W0_34: u32,
    pub MSC_MSAC_W1_34: u32,
    pub MSC_MSAC_W0_35: u32,
    pub MSC_MSAC_W1_35: u32,
    pub MSC_MSAC_W0_36: u32,
    pub MSC_MSAC_W1_36: u32,
    pub MSC_MSAC_W0_37: u32,
    pub MSC_MSAC_W1_37: u32,
    pub MSC_MSAC_W0_38: u32,
    pub MSC_MSAC_W1_38: u32,
    pub MSC_MSAC_W0_39: u32,
    pub MSC_MSAC_W1_39: u32,
    pub MSC_MSAC_W0_40: u32,
    pub MSC_MSAC_W1_40: u32,
    pub MSC_MSAC_W0_41: u32,
    pub MSC_MSAC_W1_41: u32,
    pub MSC_MSAC_W0_42: u32,
    pub MSC_MSAC_W1_42: u32,
    pub MSC_MSAC_W0_43: u32,
    pub MSC_MSAC_W1_43: u32,
    pub MSC_MSAC_W0_44: u32,
    pub MSC_MSAC_W1_44: u32,
    pub MSC_MSAC_W0_45: u32,
    pub MSC_MSAC_W1_45: u32,
    pub MSC_MSAC_W0_46: u32,
    pub MSC_MSAC_W1_46: u32,
    pub MSC_MSAC_W0_47: u32,
    pub MSC_MSAC_W1_47: u32,
    pub MSC_MSAC_W0_48: u32,
    pub MSC_MSAC_W1_48: u32,
    pub MSC_MSAC_W0_49: u32,
    pub MSC_MSAC_W1_49: u32,
    pub MSC_MSAC_W0_50: u32,
    pub MSC_MSAC_W1_50: u32,
    pub MSC_MSAC_W0_51: u32,
    pub MSC_MSAC_W1_51: u32,
    pub MSC_MSAC_W0_52: u32,
    pub MSC_MSAC_W1_52: u32,
    pub MSC_MSAC_W0_53: u32,
    pub MSC_MSAC_W1_53: u32,
    pub MSC_MSAC_W0_54: u32,
    pub MSC_MSAC_W1_54: u32,
    pub MSC_MSAC_W0_55: u32,
    pub MSC_MSAC_W1_55: u32,
    pub MSC_MSAC_W0_56: u32,
    pub MSC_MSAC_W1_56: u32,
    pub MSC_MSAC_W0_57: u32,
    pub MSC_MSAC_W1_57: u32,
    pub MSC_MSAC_W0_58: u32,
    pub MSC_MSAC_W1_58: u32,
    pub MSC_MSAC_W0_59: u32,
    pub MSC_MSAC_W1_59: u32,
    pub MSC_MSAC_W0_60: u32,
    pub MSC_MSAC_W1_60: u32,
    pub MSC_MSAC_W0_61: u32,
    pub MSC_MSAC_W1_61: u32,
    pub MSC_MSAC_W0_62: u32,
    pub MSC_MSAC_W1_62: u32,
    pub MSC_MSAC_W0_63: u32,
    pub MSC_MSAC_W1_63: u32,
    pub MSC_MSAC_W0_64: u32,
    pub MSC_MSAC_W1_64: u32,
    pub MSC_MSAC_W0_65: u32,
    pub MSC_MSAC_W1_65: u32,
    pub MSC_MSAC_W0_66: u32,
    pub MSC_MSAC_W1_66: u32,
    pub MSC_MSAC_W0_67: u32,
    pub MSC_MSAC_W1_67: u32,
    pub MSC_MSAC_W0_68: u32,
    pub MSC_MSAC_W1_68: u32,
    pub MSC_MSAC_W0_69: u32,
    pub MSC_MSAC_W1_69: u32,
    pub MSC_MSAC_W0_70: u32,
    pub MSC_MSAC_W1_70: u32,
    pub MSC_MSAC_W0_71: u32,
    pub MSC_MSAC_W1_71: u32,
    pub MSC_MSAC_W0_72: u32,
    pub MSC_MSAC_W1_72: u32,
    pub MSC_MSAC_W0_73: u32,
    pub MSC_MSAC_W1_73: u32,
    pub MSC_MSAC_W0_74: u32,
    pub MSC_MSAC_W1_74: u32,
    pub MSC_MSAC_W0_75: u32,
    pub MSC_MSAC_W1_75: u32,
    pub MSC_MSAC_W0_76: u32,
    pub MSC_MSAC_W1_76: u32,
    pub MSC_MSAC_W0_77: u32,
    pub MSC_MSAC_W1_77: u32,
    pub MSC_MSAC_W0_78: u32,
    pub MSC_MSAC_W1_78: u32,
    pub MSC_MSAC_W0_79: u32,
    pub MSC_MSAC_W1_79: u32,
    pub MSC_MSAC_W0_80: u32,
    pub MSC_MSAC_W1_80: u32,
    pub MSC_MSAC_W0_81: u32,
    pub MSC_MSAC_W1_81: u32,
    pub MSC_MSAC_W0_82: u32,
    pub MSC_MSAC_W1_82: u32,
    pub MSC_MSAC_W0_83: u32,
    pub MSC_MSAC_W1_83: u32,
    pub MSC_MSAC_W0_84: u32,
    pub MSC_MSAC_W1_84: u32,
    pub MSC_MSAC_W0_85: u32,
    pub MSC_MSAC_W1_85: u32,
    pub MSC_MSAC_W0_86: u32,
    pub MSC_MSAC_W1_86: u32,
    pub MSC_MSAC_W0_87: u32,
    pub MSC_MSAC_W1_87: u32,
    pub MSC_MSAC_W0_88: u32,
    pub MSC_MSAC_W1_88: u32,
    pub MSC_MSAC_W0_89: u32,
    pub MSC_MSAC_W1_89: u32,
    pub MSC_MSAC_W0_90: u32,
    pub MSC_MSAC_W1_90: u32,
    pub MSC_MSAC_W0_91: u32,
    pub MSC_MSAC_W1_91: u32,
    pub MSC_MSAC_W0_92: u32,
    pub MSC_MSAC_W1_92: u32,
    pub MSC_MSAC_W0_93: u32,
    pub MSC_MSAC_W1_93: u32,
    pub MSC_MSAC_W0_94: u32,
    pub MSC_MSAC_W1_94: u32,
    pub MSC_MSAC_W0_95: u32,
    pub MSC_MSAC_W1_95: u32,
    pub MSC_MSAC_W0_96: u32,
    pub MSC_MSAC_W1_96: u32,
    pub MSC_MSAC_W0_97: u32,
    pub MSC_MSAC_W1_97: u32,
    pub MSC_MSAC_W0_98: u32,
    pub MSC_MSAC_W1_98: u32,
    pub MSC_MSAC_W0_99: u32,
    pub MSC_MSAC_W1_99: u32,
    pub MSC_MSAC_W0_100: u32,
    pub MSC_MSAC_W1_100: u32,
    pub MSC_MSAC_W0_101: u32,
    pub MSC_MSAC_W1_101: u32,
    pub MSC_MSAC_W0_102: u32,
    pub MSC_MSAC_W1_102: u32,
    pub MSC_MSAC_W0_103: u32,
    pub MSC_MSAC_W1_103: u32,
    pub MSC_MSAC_W0_104: u32,
    pub MSC_MSAC_W1_104: u32,
    pub MSC_MSAC_W0_105: u32,
    pub MSC_MSAC_W1_105: u32,
    pub MSC_MSAC_W0_106: u32,
    pub MSC_MSAC_W1_106: u32,
    pub MSC_MSAC_W0_107: u32,
    pub MSC_MSAC_W1_107: u32,
    pub MSC_MSAC_W0_108: u32,
    pub MSC_MSAC_W1_108: u32,
    pub MSC_MSAC_W0_109: u32,
    pub MSC_MSAC_W1_109: u32,
    pub MSC_MSAC_W0_110: u32,
    pub MSC_MSAC_W1_110: u32,
    pub MSC_MSAC_W0_111: u32,
    pub MSC_MSAC_W1_111: u32,
    pub MSC_MSAC_W0_112: u32,
    pub MSC_MSAC_W1_112: u32,
    pub MSC_MSAC_W0_113: u32,
    pub MSC_MSAC_W1_113: u32,
    pub MSC_MSAC_W0_114: u32,
    pub MSC_MSAC_W1_114: u32,
    pub MSC_MSAC_W0_115: u32,
    pub MSC_MSAC_W1_115: u32,
    pub MSC_MSAC_W0_116: u32,
    pub MSC_MSAC_W1_116: u32,
    pub MSC_MSAC_W0_117: u32,
    pub MSC_MSAC_W1_117: u32,
    pub MSC_MSAC_W0_118: u32,
    pub MSC_MSAC_W1_118: u32,
    pub MSC_MSAC_W0_119: u32,
    pub MSC_MSAC_W1_119: u32,
    pub MSC_MSAC_W0_120: u32,
    pub MSC_MSAC_W1_120: u32,
    pub MSC_MSAC_W0_121: u32,
    pub MSC_MSAC_W1_121: u32,
    pub MSC_MSAC_W0_122: u32,
    pub MSC_MSAC_W1_122: u32,
    pub MSC_MSAC_W0_123: u32,
    pub MSC_MSAC_W1_123: u32,
    pub MSC_MSAC_W0_124: u32,
    pub MSC_MSAC_W1_124: u32,
    pub MSC_MSAC_W0_125: u32,
    pub MSC_MSAC_W1_125: u32,
    pub MSC_MSAC_W0_126: u32,
    pub MSC_MSAC_W1_126: u32,
    pub MSC_MSAC_W0_127: u32,
    pub MSC_MSAC_W1_127: u32,
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
