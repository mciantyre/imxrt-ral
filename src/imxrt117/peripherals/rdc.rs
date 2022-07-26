#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! RDC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Version Information
pub mod VIR {

    /// Number of Domains
    pub mod NDID {
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

    /// Number of Masters
    pub mod NMSTR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (8 bits: 0xff << 4)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of Peripherals
    pub mod NPER {
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

    /// Number of Memory Regions
    pub mod NRGN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (8 bits: 0xff << 20)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Status
pub mod STAT {

    /// Domain ID
    pub mod DID {
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

    /// Power Domain Status
    pub mod PDS {
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

            /// 0b0: Power Down Domain is OFF
            pub const PDS_0: u32 = 0b0;

            /// 0b1: Power Down Domain is ON
            pub const PDS_1: u32 = 0b1;
        }
    }
}

/// Interrupt and Control
pub mod INTCTRL {

    /// Restoration Complete Interrupt
    pub mod RCI_EN {
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

            /// 0b0: Interrupt Disabled
            pub const RCI_EN_0: u32 = 0b0;

            /// 0b1: Interrupt Enabled
            pub const RCI_EN_1: u32 = 0b1;
        }
    }
}

/// Interrupt Status
pub mod INTSTAT {

    /// Interrupt Status
    pub mod INT {
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

            /// 0b0: No Interrupt Pending
            pub const INT_0: u32 = 0b0;

            /// 0b1: Interrupt Pending
            pub const INT_1: u32 = 0b1;
        }
    }
}

/// Master Domain Assignment
pub mod MDA0 {

    /// Domain ID
    pub mod DID {
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

            /// 0b00: Master assigned to Processing Domain 0
            pub const DID_0: u32 = 0b00;

            /// 0b01: Master assigned to Processing Domain 1
            pub const DID_1: u32 = 0b01;
        }
    }

    /// Assignment Lock
    pub mod LCK {
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

            /// 0b0: Not Locked
            pub const LCK_0: u32 = 0b0;

            /// 0b1: Locked
            pub const LCK_1: u32 = 0b1;
        }
    }
}

/// Master Domain Assignment
pub mod MDA1 {
    pub use super::MDA0::DID;
    pub use super::MDA0::LCK;
}

/// Master Domain Assignment
pub mod MDA2 {
    pub use super::MDA0::DID;
    pub use super::MDA0::LCK;
}

/// Master Domain Assignment
pub mod MDA3 {
    pub use super::MDA0::DID;
    pub use super::MDA0::LCK;
}

/// Master Domain Assignment
pub mod MDA4 {
    pub use super::MDA0::DID;
    pub use super::MDA0::LCK;
}

/// Master Domain Assignment
pub mod MDA5 {
    pub use super::MDA0::DID;
    pub use super::MDA0::LCK;
}

/// Master Domain Assignment
pub mod MDA6 {
    pub use super::MDA0::DID;
    pub use super::MDA0::LCK;
}

/// Master Domain Assignment
pub mod MDA7 {
    pub use super::MDA0::DID;
    pub use super::MDA0::LCK;
}

/// Master Domain Assignment
pub mod MDA8 {
    pub use super::MDA0::DID;
    pub use super::MDA0::LCK;
}

/// Master Domain Assignment
pub mod MDA9 {
    pub use super::MDA0::DID;
    pub use super::MDA0::LCK;
}

/// Master Domain Assignment
pub mod MDA10 {
    pub use super::MDA0::DID;
    pub use super::MDA0::LCK;
}

/// Master Domain Assignment
pub mod MDA11 {
    pub use super::MDA0::DID;
    pub use super::MDA0::LCK;
}

/// Peripheral Domain Access Permissions
pub mod PDAP0 {

    /// Domain 0 Write Access
    pub mod D0W {
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

            /// 0b0: No Write Access
            pub const D0W_0: u32 = 0b0;

            /// 0b1: Write Access Allowed
            pub const D0W_1: u32 = 0b1;
        }
    }

    /// Domain 0 Read Access
    pub mod D0R {
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

            /// 0b0: No Read Access
            pub const D0R_0: u32 = 0b0;

            /// 0b1: Read Access Allowed
            pub const D0R_1: u32 = 0b1;
        }
    }

    /// Domain 1 Write Access
    pub mod D1W {
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

            /// 0b0: No Write Access
            pub const D1W_0: u32 = 0b0;

            /// 0b1: Write Access Allowed
            pub const D1W_1: u32 = 0b1;
        }
    }

    /// Domain 1 Read Access
    pub mod D1R {
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

            /// 0b0: No Read Access
            pub const D1R_0: u32 = 0b0;

            /// 0b1: Read Access Allowed
            pub const D1R_1: u32 = 0b1;
        }
    }

    /// Semaphore Required
    pub mod SREQ {
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

            /// 0b0: Semaphores have no effect
            pub const SREQ_0: u32 = 0b0;

            /// 0b1: Semaphores are enforced
            pub const SREQ_1: u32 = 0b1;
        }
    }

    /// Peripheral Permissions Lock
    pub mod LCK {
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

            /// 0b0: Not Locked
            pub const LCK_0: u32 = 0b0;

            /// 0b1: Locked
            pub const LCK_1: u32 = 0b1;
        }
    }
}

/// Peripheral Domain Access Permissions
pub mod PDAP1 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP2 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP3 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP4 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP5 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP6 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP7 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP8 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP9 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP10 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP11 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP12 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP13 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP14 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP15 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP16 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP17 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP18 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP19 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP20 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP21 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP22 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP23 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP24 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP25 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP26 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP27 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP28 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP29 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP30 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP31 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP32 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP33 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP34 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP35 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP36 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP37 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP38 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP39 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP40 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP41 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP42 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP43 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP44 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP45 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP46 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP47 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP48 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP49 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP50 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP51 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP52 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP53 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP54 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP55 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP56 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP57 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP58 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP59 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP60 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP61 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP62 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP63 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP64 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP65 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP66 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP67 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP68 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP69 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP70 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP71 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP72 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP73 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP74 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP75 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP76 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP77 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP78 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP79 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP80 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP81 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP82 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP83 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP84 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP85 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP86 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP87 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP88 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP89 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP90 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP91 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP92 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP93 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP94 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP95 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP96 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP97 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP98 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP99 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP100 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP101 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP102 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP103 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP104 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP105 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP106 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP107 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP108 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP109 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP110 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP111 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP112 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP113 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP114 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP115 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP116 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP117 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP118 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP119 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP120 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP121 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP122 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP123 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP124 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP125 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP126 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Peripheral Domain Access Permissions
pub mod PDAP127 {
    pub use super::PDAP0::D0R;
    pub use super::PDAP0::D0W;
    pub use super::PDAP0::D1R;
    pub use super::PDAP0::D1W;
    pub use super::PDAP0::LCK;
    pub use super::PDAP0::SREQ;
}

/// Memory Region Start Address
pub mod MRSA0 {

    /// Start address for memory region
    pub mod SADR {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (25 bits: 0x1ffffff << 7)
        pub const mask: u32 = 0x1ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Memory Region Start Address
pub mod MRSA1 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA2 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA3 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA4 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA5 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA6 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA7 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA8 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA9 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA10 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA11 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA12 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA13 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA14 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA15 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA16 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA17 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA18 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA19 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA20 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA21 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA22 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA23 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA24 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA25 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA26 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA27 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA28 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA29 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA30 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA31 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA32 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA33 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA34 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA35 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA36 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA37 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA38 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA39 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA40 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA41 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA42 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA43 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA44 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA45 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA46 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA47 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA48 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA49 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA50 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA51 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA52 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA53 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA54 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA55 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA56 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA57 {
    pub use super::MRSA0::SADR;
}

/// Memory Region Start Address
pub mod MRSA58 {
    pub use super::MRSA0::SADR;
}

/// Memory Region End Address
pub mod MREA0 {

    /// Upper bound for memory region
    pub mod EADR {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (25 bits: 0x1ffffff << 7)
        pub const mask: u32 = 0x1ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Memory Region End Address
pub mod MREA1 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA2 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA3 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA4 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA5 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA6 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA7 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA8 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA9 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA10 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA11 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA12 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA13 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA14 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA15 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA16 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA17 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA18 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA19 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA20 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA21 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA22 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA23 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA24 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA25 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA26 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA27 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA28 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA29 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA30 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA31 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA32 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA33 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA34 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA35 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA36 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA37 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA38 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA39 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA40 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA41 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA42 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA43 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA44 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA45 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA46 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA47 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA48 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA49 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA50 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA51 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA52 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA53 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA54 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA55 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA56 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA57 {
    pub use super::MREA0::EADR;
}

/// Memory Region End Address
pub mod MREA58 {
    pub use super::MREA0::EADR;
}

/// Memory Region Control
pub mod MRC0 {

    /// Domain 0 Write Access to Region
    pub mod D0W {
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

            /// 0b0: Processing Domain 0 does not have Write access to the memory region
            pub const D0W_0: u32 = 0b0;

            /// 0b1: Processing Domain 0 has Write access to the memory region
            pub const D0W_1: u32 = 0b1;
        }
    }

    /// Domain 0 Read Access to Region
    pub mod D0R {
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

            /// 0b0: Processing Domain 0 does not have Read access to the memory region
            pub const D0R_0: u32 = 0b0;

            /// 0b1: Processing Domain 0 has Read access to the memory region
            pub const D0R_1: u32 = 0b1;
        }
    }

    /// Domain 1 Write Access to Region
    pub mod D1W {
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

            /// 0b0: Processing Domain 1 does not have Write access to the memory region
            pub const D1W_0: u32 = 0b0;

            /// 0b1: Processing Domain 1 has Write access to the memory region
            pub const D1W_1: u32 = 0b1;
        }
    }

    /// Domain 1 Read Access to Region
    pub mod D1R {
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

            /// 0b0: Processing Domain 1 does not have Read access to the memory region
            pub const D1R_0: u32 = 0b0;

            /// 0b1: Processing Domain 1 has Read access to the memory region
            pub const D1R_1: u32 = 0b1;
        }
    }

    /// Region Enable
    pub mod ENA {
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

            /// 0b0: Memory region is not defined or restricted.
            pub const ENA_0: u32 = 0b0;

            /// 0b1: Memory boundaries, domain permissions and controls are in effect.
            pub const ENA_1: u32 = 0b1;
        }
    }

    /// Region Lock
    pub mod LCK {
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

            /// 0b0: No Lock. All fields in this register may be modified.
            pub const LCK_0: u32 = 0b0;

            /// 0b1: Locked. No fields in this register may be modified except ENA, which may be set but not cleared.
            pub const LCK_1: u32 = 0b1;
        }
    }
}

/// Memory Region Control
pub mod MRC1 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC2 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC3 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC4 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC5 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC6 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC7 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC8 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC9 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC10 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC11 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC12 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC13 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC14 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC15 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC16 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC17 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC18 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC19 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC20 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC21 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC22 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC23 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC24 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC25 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC26 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC27 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC28 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC29 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC30 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC31 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC32 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC33 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC34 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC35 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC36 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC37 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC38 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC39 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC40 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC41 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC42 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC43 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC44 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC45 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC46 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC47 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC48 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC49 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC50 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC51 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC52 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC53 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC54 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC55 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC56 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC57 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Control
pub mod MRC58 {
    pub use super::MRC0::D0R;
    pub use super::MRC0::D0W;
    pub use super::MRC0::D1R;
    pub use super::MRC0::D1W;
    pub use super::MRC0::ENA;
    pub use super::MRC0::LCK;
}

/// Memory Region Violation Status
pub mod MRVS0 {

    /// Violating Domain ID
    pub mod VDID {
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

            /// 0b00: Processing Domain 0
            pub const VDID_0: u32 = 0b00;

            /// 0b01: Processing Domain 1
            pub const VDID_1: u32 = 0b01;
        }
    }

    /// Access Denied
    pub mod AD {
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

    /// Violating Address
    pub mod VADR {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (27 bits: 0x7ffffff << 5)
        pub const mask: u32 = 0x7ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Memory Region Violation Status
pub mod MRVS1 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS2 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS3 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS4 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS5 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS6 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS7 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS8 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS9 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS10 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS11 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS12 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS13 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS14 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS15 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS16 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS17 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS18 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS19 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS20 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS21 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS22 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS23 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS24 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS25 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS26 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS27 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS28 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS29 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS30 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS31 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS32 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS33 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS34 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS35 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS36 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS37 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS38 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS39 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS40 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS41 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS42 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS43 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS44 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS45 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS46 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS47 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS48 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS49 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS50 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS51 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS52 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS53 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS54 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS55 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS56 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS57 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}

/// Memory Region Violation Status
pub mod MRVS58 {
    pub use super::MRVS0::AD;
    pub use super::MRVS0::VADR;
    pub use super::MRVS0::VDID;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Version Information
    pub VIR: RORegister<u32>,

    _reserved1: [u32; 8],

    /// Status
    pub STAT: RWRegister<u32>,

    /// Interrupt and Control
    pub INTCTRL: RWRegister<u32>,

    /// Interrupt Status
    pub INTSTAT: RWRegister<u32>,

    _reserved2: [u32; 116],

    /// Master Domain Assignment
    pub MDA0: RWRegister<u32>,

    /// Master Domain Assignment
    pub MDA1: RWRegister<u32>,

    /// Master Domain Assignment
    pub MDA2: RWRegister<u32>,

    /// Master Domain Assignment
    pub MDA3: RWRegister<u32>,

    /// Master Domain Assignment
    pub MDA4: RWRegister<u32>,

    /// Master Domain Assignment
    pub MDA5: RWRegister<u32>,

    /// Master Domain Assignment
    pub MDA6: RWRegister<u32>,

    /// Master Domain Assignment
    pub MDA7: RWRegister<u32>,

    /// Master Domain Assignment
    pub MDA8: RWRegister<u32>,

    /// Master Domain Assignment
    pub MDA9: RWRegister<u32>,

    /// Master Domain Assignment
    pub MDA10: RWRegister<u32>,

    /// Master Domain Assignment
    pub MDA11: RWRegister<u32>,

    _reserved3: [u32; 116],

    /// Peripheral Domain Access Permissions
    pub PDAP0: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP1: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP2: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP3: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP4: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP5: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP6: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP7: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP8: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP9: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP10: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP11: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP12: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP13: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP14: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP15: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP16: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP17: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP18: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP19: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP20: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP21: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP22: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP23: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP24: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP25: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP26: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP27: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP28: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP29: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP30: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP31: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP32: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP33: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP34: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP35: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP36: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP37: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP38: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP39: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP40: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP41: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP42: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP43: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP44: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP45: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP46: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP47: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP48: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP49: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP50: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP51: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP52: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP53: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP54: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP55: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP56: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP57: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP58: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP59: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP60: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP61: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP62: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP63: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP64: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP65: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP66: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP67: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP68: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP69: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP70: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP71: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP72: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP73: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP74: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP75: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP76: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP77: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP78: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP79: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP80: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP81: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP82: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP83: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP84: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP85: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP86: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP87: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP88: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP89: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP90: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP91: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP92: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP93: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP94: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP95: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP96: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP97: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP98: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP99: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP100: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP101: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP102: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP103: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP104: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP105: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP106: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP107: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP108: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP109: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP110: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP111: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP112: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP113: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP114: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP115: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP116: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP117: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP118: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP119: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP120: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP121: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP122: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP123: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP124: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP125: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP126: RWRegister<u32>,

    /// Peripheral Domain Access Permissions
    pub PDAP127: RWRegister<u32>,

    _reserved4: [u32; 128],

    /// Memory Region Start Address
    pub MRSA0: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA0: RWRegister<u32>,

    /// Memory Region Control
    pub MRC0: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS0: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA1: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA1: RWRegister<u32>,

    /// Memory Region Control
    pub MRC1: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS1: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA2: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA2: RWRegister<u32>,

    /// Memory Region Control
    pub MRC2: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS2: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA3: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA3: RWRegister<u32>,

    /// Memory Region Control
    pub MRC3: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS3: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA4: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA4: RWRegister<u32>,

    /// Memory Region Control
    pub MRC4: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS4: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA5: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA5: RWRegister<u32>,

    /// Memory Region Control
    pub MRC5: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS5: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA6: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA6: RWRegister<u32>,

    /// Memory Region Control
    pub MRC6: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS6: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA7: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA7: RWRegister<u32>,

    /// Memory Region Control
    pub MRC7: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS7: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA8: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA8: RWRegister<u32>,

    /// Memory Region Control
    pub MRC8: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS8: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA9: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA9: RWRegister<u32>,

    /// Memory Region Control
    pub MRC9: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS9: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA10: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA10: RWRegister<u32>,

    /// Memory Region Control
    pub MRC10: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS10: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA11: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA11: RWRegister<u32>,

    /// Memory Region Control
    pub MRC11: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS11: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA12: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA12: RWRegister<u32>,

    /// Memory Region Control
    pub MRC12: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS12: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA13: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA13: RWRegister<u32>,

    /// Memory Region Control
    pub MRC13: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS13: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA14: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA14: RWRegister<u32>,

    /// Memory Region Control
    pub MRC14: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS14: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA15: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA15: RWRegister<u32>,

    /// Memory Region Control
    pub MRC15: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS15: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA16: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA16: RWRegister<u32>,

    /// Memory Region Control
    pub MRC16: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS16: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA17: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA17: RWRegister<u32>,

    /// Memory Region Control
    pub MRC17: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS17: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA18: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA18: RWRegister<u32>,

    /// Memory Region Control
    pub MRC18: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS18: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA19: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA19: RWRegister<u32>,

    /// Memory Region Control
    pub MRC19: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS19: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA20: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA20: RWRegister<u32>,

    /// Memory Region Control
    pub MRC20: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS20: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA21: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA21: RWRegister<u32>,

    /// Memory Region Control
    pub MRC21: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS21: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA22: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA22: RWRegister<u32>,

    /// Memory Region Control
    pub MRC22: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS22: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA23: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA23: RWRegister<u32>,

    /// Memory Region Control
    pub MRC23: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS23: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA24: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA24: RWRegister<u32>,

    /// Memory Region Control
    pub MRC24: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS24: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA25: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA25: RWRegister<u32>,

    /// Memory Region Control
    pub MRC25: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS25: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA26: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA26: RWRegister<u32>,

    /// Memory Region Control
    pub MRC26: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS26: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA27: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA27: RWRegister<u32>,

    /// Memory Region Control
    pub MRC27: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS27: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA28: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA28: RWRegister<u32>,

    /// Memory Region Control
    pub MRC28: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS28: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA29: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA29: RWRegister<u32>,

    /// Memory Region Control
    pub MRC29: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS29: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA30: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA30: RWRegister<u32>,

    /// Memory Region Control
    pub MRC30: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS30: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA31: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA31: RWRegister<u32>,

    /// Memory Region Control
    pub MRC31: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS31: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA32: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA32: RWRegister<u32>,

    /// Memory Region Control
    pub MRC32: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS32: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA33: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA33: RWRegister<u32>,

    /// Memory Region Control
    pub MRC33: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS33: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA34: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA34: RWRegister<u32>,

    /// Memory Region Control
    pub MRC34: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS34: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA35: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA35: RWRegister<u32>,

    /// Memory Region Control
    pub MRC35: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS35: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA36: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA36: RWRegister<u32>,

    /// Memory Region Control
    pub MRC36: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS36: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA37: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA37: RWRegister<u32>,

    /// Memory Region Control
    pub MRC37: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS37: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA38: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA38: RWRegister<u32>,

    /// Memory Region Control
    pub MRC38: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS38: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA39: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA39: RWRegister<u32>,

    /// Memory Region Control
    pub MRC39: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS39: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA40: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA40: RWRegister<u32>,

    /// Memory Region Control
    pub MRC40: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS40: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA41: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA41: RWRegister<u32>,

    /// Memory Region Control
    pub MRC41: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS41: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA42: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA42: RWRegister<u32>,

    /// Memory Region Control
    pub MRC42: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS42: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA43: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA43: RWRegister<u32>,

    /// Memory Region Control
    pub MRC43: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS43: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA44: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA44: RWRegister<u32>,

    /// Memory Region Control
    pub MRC44: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS44: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA45: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA45: RWRegister<u32>,

    /// Memory Region Control
    pub MRC45: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS45: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA46: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA46: RWRegister<u32>,

    /// Memory Region Control
    pub MRC46: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS46: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA47: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA47: RWRegister<u32>,

    /// Memory Region Control
    pub MRC47: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS47: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA48: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA48: RWRegister<u32>,

    /// Memory Region Control
    pub MRC48: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS48: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA49: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA49: RWRegister<u32>,

    /// Memory Region Control
    pub MRC49: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS49: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA50: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA50: RWRegister<u32>,

    /// Memory Region Control
    pub MRC50: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS50: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA51: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA51: RWRegister<u32>,

    /// Memory Region Control
    pub MRC51: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS51: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA52: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA52: RWRegister<u32>,

    /// Memory Region Control
    pub MRC52: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS52: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA53: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA53: RWRegister<u32>,

    /// Memory Region Control
    pub MRC53: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS53: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA54: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA54: RWRegister<u32>,

    /// Memory Region Control
    pub MRC54: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS54: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA55: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA55: RWRegister<u32>,

    /// Memory Region Control
    pub MRC55: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS55: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA56: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA56: RWRegister<u32>,

    /// Memory Region Control
    pub MRC56: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS56: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA57: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA57: RWRegister<u32>,

    /// Memory Region Control
    pub MRC57: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS57: RWRegister<u32>,

    /// Memory Region Start Address
    pub MRSA58: RWRegister<u32>,

    /// Memory Region End Address
    pub MREA58: RWRegister<u32>,

    /// Memory Region Control
    pub MRC58: RWRegister<u32>,

    /// Memory Region Violation Status
    pub MRVS58: RWRegister<u32>,
}
pub struct ResetValues {
    pub VIR: u32,
    pub STAT: u32,
    pub INTCTRL: u32,
    pub INTSTAT: u32,
    pub MDA0: u32,
    pub MDA1: u32,
    pub MDA2: u32,
    pub MDA3: u32,
    pub MDA4: u32,
    pub MDA5: u32,
    pub MDA6: u32,
    pub MDA7: u32,
    pub MDA8: u32,
    pub MDA9: u32,
    pub MDA10: u32,
    pub MDA11: u32,
    pub PDAP0: u32,
    pub PDAP1: u32,
    pub PDAP2: u32,
    pub PDAP3: u32,
    pub PDAP4: u32,
    pub PDAP5: u32,
    pub PDAP6: u32,
    pub PDAP7: u32,
    pub PDAP8: u32,
    pub PDAP9: u32,
    pub PDAP10: u32,
    pub PDAP11: u32,
    pub PDAP12: u32,
    pub PDAP13: u32,
    pub PDAP14: u32,
    pub PDAP15: u32,
    pub PDAP16: u32,
    pub PDAP17: u32,
    pub PDAP18: u32,
    pub PDAP19: u32,
    pub PDAP20: u32,
    pub PDAP21: u32,
    pub PDAP22: u32,
    pub PDAP23: u32,
    pub PDAP24: u32,
    pub PDAP25: u32,
    pub PDAP26: u32,
    pub PDAP27: u32,
    pub PDAP28: u32,
    pub PDAP29: u32,
    pub PDAP30: u32,
    pub PDAP31: u32,
    pub PDAP32: u32,
    pub PDAP33: u32,
    pub PDAP34: u32,
    pub PDAP35: u32,
    pub PDAP36: u32,
    pub PDAP37: u32,
    pub PDAP38: u32,
    pub PDAP39: u32,
    pub PDAP40: u32,
    pub PDAP41: u32,
    pub PDAP42: u32,
    pub PDAP43: u32,
    pub PDAP44: u32,
    pub PDAP45: u32,
    pub PDAP46: u32,
    pub PDAP47: u32,
    pub PDAP48: u32,
    pub PDAP49: u32,
    pub PDAP50: u32,
    pub PDAP51: u32,
    pub PDAP52: u32,
    pub PDAP53: u32,
    pub PDAP54: u32,
    pub PDAP55: u32,
    pub PDAP56: u32,
    pub PDAP57: u32,
    pub PDAP58: u32,
    pub PDAP59: u32,
    pub PDAP60: u32,
    pub PDAP61: u32,
    pub PDAP62: u32,
    pub PDAP63: u32,
    pub PDAP64: u32,
    pub PDAP65: u32,
    pub PDAP66: u32,
    pub PDAP67: u32,
    pub PDAP68: u32,
    pub PDAP69: u32,
    pub PDAP70: u32,
    pub PDAP71: u32,
    pub PDAP72: u32,
    pub PDAP73: u32,
    pub PDAP74: u32,
    pub PDAP75: u32,
    pub PDAP76: u32,
    pub PDAP77: u32,
    pub PDAP78: u32,
    pub PDAP79: u32,
    pub PDAP80: u32,
    pub PDAP81: u32,
    pub PDAP82: u32,
    pub PDAP83: u32,
    pub PDAP84: u32,
    pub PDAP85: u32,
    pub PDAP86: u32,
    pub PDAP87: u32,
    pub PDAP88: u32,
    pub PDAP89: u32,
    pub PDAP90: u32,
    pub PDAP91: u32,
    pub PDAP92: u32,
    pub PDAP93: u32,
    pub PDAP94: u32,
    pub PDAP95: u32,
    pub PDAP96: u32,
    pub PDAP97: u32,
    pub PDAP98: u32,
    pub PDAP99: u32,
    pub PDAP100: u32,
    pub PDAP101: u32,
    pub PDAP102: u32,
    pub PDAP103: u32,
    pub PDAP104: u32,
    pub PDAP105: u32,
    pub PDAP106: u32,
    pub PDAP107: u32,
    pub PDAP108: u32,
    pub PDAP109: u32,
    pub PDAP110: u32,
    pub PDAP111: u32,
    pub PDAP112: u32,
    pub PDAP113: u32,
    pub PDAP114: u32,
    pub PDAP115: u32,
    pub PDAP116: u32,
    pub PDAP117: u32,
    pub PDAP118: u32,
    pub PDAP119: u32,
    pub PDAP120: u32,
    pub PDAP121: u32,
    pub PDAP122: u32,
    pub PDAP123: u32,
    pub PDAP124: u32,
    pub PDAP125: u32,
    pub PDAP126: u32,
    pub PDAP127: u32,
    pub MRSA0: u32,
    pub MREA0: u32,
    pub MRC0: u32,
    pub MRVS0: u32,
    pub MRSA1: u32,
    pub MREA1: u32,
    pub MRC1: u32,
    pub MRVS1: u32,
    pub MRSA2: u32,
    pub MREA2: u32,
    pub MRC2: u32,
    pub MRVS2: u32,
    pub MRSA3: u32,
    pub MREA3: u32,
    pub MRC3: u32,
    pub MRVS3: u32,
    pub MRSA4: u32,
    pub MREA4: u32,
    pub MRC4: u32,
    pub MRVS4: u32,
    pub MRSA5: u32,
    pub MREA5: u32,
    pub MRC5: u32,
    pub MRVS5: u32,
    pub MRSA6: u32,
    pub MREA6: u32,
    pub MRC6: u32,
    pub MRVS6: u32,
    pub MRSA7: u32,
    pub MREA7: u32,
    pub MRC7: u32,
    pub MRVS7: u32,
    pub MRSA8: u32,
    pub MREA8: u32,
    pub MRC8: u32,
    pub MRVS8: u32,
    pub MRSA9: u32,
    pub MREA9: u32,
    pub MRC9: u32,
    pub MRVS9: u32,
    pub MRSA10: u32,
    pub MREA10: u32,
    pub MRC10: u32,
    pub MRVS10: u32,
    pub MRSA11: u32,
    pub MREA11: u32,
    pub MRC11: u32,
    pub MRVS11: u32,
    pub MRSA12: u32,
    pub MREA12: u32,
    pub MRC12: u32,
    pub MRVS12: u32,
    pub MRSA13: u32,
    pub MREA13: u32,
    pub MRC13: u32,
    pub MRVS13: u32,
    pub MRSA14: u32,
    pub MREA14: u32,
    pub MRC14: u32,
    pub MRVS14: u32,
    pub MRSA15: u32,
    pub MREA15: u32,
    pub MRC15: u32,
    pub MRVS15: u32,
    pub MRSA16: u32,
    pub MREA16: u32,
    pub MRC16: u32,
    pub MRVS16: u32,
    pub MRSA17: u32,
    pub MREA17: u32,
    pub MRC17: u32,
    pub MRVS17: u32,
    pub MRSA18: u32,
    pub MREA18: u32,
    pub MRC18: u32,
    pub MRVS18: u32,
    pub MRSA19: u32,
    pub MREA19: u32,
    pub MRC19: u32,
    pub MRVS19: u32,
    pub MRSA20: u32,
    pub MREA20: u32,
    pub MRC20: u32,
    pub MRVS20: u32,
    pub MRSA21: u32,
    pub MREA21: u32,
    pub MRC21: u32,
    pub MRVS21: u32,
    pub MRSA22: u32,
    pub MREA22: u32,
    pub MRC22: u32,
    pub MRVS22: u32,
    pub MRSA23: u32,
    pub MREA23: u32,
    pub MRC23: u32,
    pub MRVS23: u32,
    pub MRSA24: u32,
    pub MREA24: u32,
    pub MRC24: u32,
    pub MRVS24: u32,
    pub MRSA25: u32,
    pub MREA25: u32,
    pub MRC25: u32,
    pub MRVS25: u32,
    pub MRSA26: u32,
    pub MREA26: u32,
    pub MRC26: u32,
    pub MRVS26: u32,
    pub MRSA27: u32,
    pub MREA27: u32,
    pub MRC27: u32,
    pub MRVS27: u32,
    pub MRSA28: u32,
    pub MREA28: u32,
    pub MRC28: u32,
    pub MRVS28: u32,
    pub MRSA29: u32,
    pub MREA29: u32,
    pub MRC29: u32,
    pub MRVS29: u32,
    pub MRSA30: u32,
    pub MREA30: u32,
    pub MRC30: u32,
    pub MRVS30: u32,
    pub MRSA31: u32,
    pub MREA31: u32,
    pub MRC31: u32,
    pub MRVS31: u32,
    pub MRSA32: u32,
    pub MREA32: u32,
    pub MRC32: u32,
    pub MRVS32: u32,
    pub MRSA33: u32,
    pub MREA33: u32,
    pub MRC33: u32,
    pub MRVS33: u32,
    pub MRSA34: u32,
    pub MREA34: u32,
    pub MRC34: u32,
    pub MRVS34: u32,
    pub MRSA35: u32,
    pub MREA35: u32,
    pub MRC35: u32,
    pub MRVS35: u32,
    pub MRSA36: u32,
    pub MREA36: u32,
    pub MRC36: u32,
    pub MRVS36: u32,
    pub MRSA37: u32,
    pub MREA37: u32,
    pub MRC37: u32,
    pub MRVS37: u32,
    pub MRSA38: u32,
    pub MREA38: u32,
    pub MRC38: u32,
    pub MRVS38: u32,
    pub MRSA39: u32,
    pub MREA39: u32,
    pub MRC39: u32,
    pub MRVS39: u32,
    pub MRSA40: u32,
    pub MREA40: u32,
    pub MRC40: u32,
    pub MRVS40: u32,
    pub MRSA41: u32,
    pub MREA41: u32,
    pub MRC41: u32,
    pub MRVS41: u32,
    pub MRSA42: u32,
    pub MREA42: u32,
    pub MRC42: u32,
    pub MRVS42: u32,
    pub MRSA43: u32,
    pub MREA43: u32,
    pub MRC43: u32,
    pub MRVS43: u32,
    pub MRSA44: u32,
    pub MREA44: u32,
    pub MRC44: u32,
    pub MRVS44: u32,
    pub MRSA45: u32,
    pub MREA45: u32,
    pub MRC45: u32,
    pub MRVS45: u32,
    pub MRSA46: u32,
    pub MREA46: u32,
    pub MRC46: u32,
    pub MRVS46: u32,
    pub MRSA47: u32,
    pub MREA47: u32,
    pub MRC47: u32,
    pub MRVS47: u32,
    pub MRSA48: u32,
    pub MREA48: u32,
    pub MRC48: u32,
    pub MRVS48: u32,
    pub MRSA49: u32,
    pub MREA49: u32,
    pub MRC49: u32,
    pub MRVS49: u32,
    pub MRSA50: u32,
    pub MREA50: u32,
    pub MRC50: u32,
    pub MRVS50: u32,
    pub MRSA51: u32,
    pub MREA51: u32,
    pub MRC51: u32,
    pub MRVS51: u32,
    pub MRSA52: u32,
    pub MREA52: u32,
    pub MRC52: u32,
    pub MRVS52: u32,
    pub MRSA53: u32,
    pub MREA53: u32,
    pub MRC53: u32,
    pub MRVS53: u32,
    pub MRSA54: u32,
    pub MREA54: u32,
    pub MRC54: u32,
    pub MRVS54: u32,
    pub MRSA55: u32,
    pub MREA55: u32,
    pub MRC55: u32,
    pub MRVS55: u32,
    pub MRSA56: u32,
    pub MREA56: u32,
    pub MRC56: u32,
    pub MRVS56: u32,
    pub MRSA57: u32,
    pub MREA57: u32,
    pub MRC57: u32,
    pub MRVS57: u32,
    pub MRSA58: u32,
    pub MREA58: u32,
    pub MRC58: u32,
    pub MRVS58: u32,
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
