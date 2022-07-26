#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Nested Vectored Interrupt Controller
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// Interrupt Set Enable Register n
pub mod NVICISER0 {

    /// Interrupt set enable bits
    pub mod SETENA {
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

/// Interrupt Set Enable Register n
pub mod NVICISER1 {
    pub use super::NVICISER0::SETENA;
}

/// Interrupt Set Enable Register n
pub mod NVICISER2 {
    pub use super::NVICISER0::SETENA;
}

/// Interrupt Set Enable Register n
pub mod NVICISER3 {
    pub use super::NVICISER0::SETENA;
}

/// Interrupt Clear Enable Register n
pub mod NVICICER0 {

    /// Interrupt clear-enable bits
    pub mod CLRENA {
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

/// Interrupt Clear Enable Register n
pub mod NVICICER1 {
    pub use super::NVICICER0::CLRENA;
}

/// Interrupt Clear Enable Register n
pub mod NVICICER2 {
    pub use super::NVICICER0::CLRENA;
}

/// Interrupt Clear Enable Register n
pub mod NVICICER3 {
    pub use super::NVICICER0::CLRENA;
}

/// Interrupt Set Pending Register n
pub mod NVICISPR0 {

    /// Interrupt set-pending bits
    pub mod SETPEND {
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

/// Interrupt Set Pending Register n
pub mod NVICISPR1 {
    pub use super::NVICISPR0::SETPEND;
}

/// Interrupt Set Pending Register n
pub mod NVICISPR2 {
    pub use super::NVICISPR0::SETPEND;
}

/// Interrupt Set Pending Register n
pub mod NVICISPR3 {
    pub use super::NVICISPR0::SETPEND;
}

/// Interrupt Clear Pending Register n
pub mod NVICICPR0 {

    /// Interrupt clear-pending bits
    pub mod CLRPEND {
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

/// Interrupt Clear Pending Register n
pub mod NVICICPR1 {
    pub use super::NVICICPR0::CLRPEND;
}

/// Interrupt Clear Pending Register n
pub mod NVICICPR2 {
    pub use super::NVICICPR0::CLRPEND;
}

/// Interrupt Clear Pending Register n
pub mod NVICICPR3 {
    pub use super::NVICICPR0::CLRPEND;
}

/// Interrupt Active bit Register n
pub mod NVICIABR0 {

    /// Interrupt active flags
    pub mod ACTIVE {
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

/// Interrupt Active bit Register n
pub mod NVICIABR1 {
    pub use super::NVICIABR0::ACTIVE;
}

/// Interrupt Active bit Register n
pub mod NVICIABR2 {
    pub use super::NVICIABR0::ACTIVE;
}

/// Interrupt Active bit Register n
pub mod NVICIABR3 {
    pub use super::NVICIABR0::ACTIVE;
}

/// Interrupt Priority Register 0
pub mod NVICIP0 {

    /// Priority of interrupt 0
    pub mod PRI0 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 1
pub mod NVICIP1 {

    /// Priority of interrupt 1
    pub mod PRI1 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 2
pub mod NVICIP2 {

    /// Priority of interrupt 2
    pub mod PRI2 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 3
pub mod NVICIP3 {

    /// Priority of interrupt 3
    pub mod PRI3 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 4
pub mod NVICIP4 {

    /// Priority of interrupt 4
    pub mod PRI4 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 5
pub mod NVICIP5 {

    /// Priority of interrupt 5
    pub mod PRI5 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 6
pub mod NVICIP6 {

    /// Priority of interrupt 6
    pub mod PRI6 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 7
pub mod NVICIP7 {

    /// Priority of interrupt 7
    pub mod PRI7 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 8
pub mod NVICIP8 {

    /// Priority of interrupt 8
    pub mod PRI8 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 9
pub mod NVICIP9 {

    /// Priority of interrupt 9
    pub mod PRI9 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 10
pub mod NVICIP10 {

    /// Priority of interrupt 10
    pub mod PRI10 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 11
pub mod NVICIP11 {

    /// Priority of interrupt 11
    pub mod PRI11 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 12
pub mod NVICIP12 {

    /// Priority of interrupt 12
    pub mod PRI12 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 13
pub mod NVICIP13 {

    /// Priority of interrupt 13
    pub mod PRI13 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 14
pub mod NVICIP14 {

    /// Priority of interrupt 14
    pub mod PRI14 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 15
pub mod NVICIP15 {

    /// Priority of interrupt 15
    pub mod PRI15 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 16
pub mod NVICIP16 {

    /// Priority of interrupt 16
    pub mod PRI16 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 17
pub mod NVICIP17 {

    /// Priority of interrupt 17
    pub mod PRI17 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 18
pub mod NVICIP18 {

    /// Priority of interrupt 18
    pub mod PRI18 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 19
pub mod NVICIP19 {

    /// Priority of interrupt 19
    pub mod PRI19 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 20
pub mod NVICIP20 {

    /// Priority of interrupt 20
    pub mod PRI20 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 21
pub mod NVICIP21 {

    /// Priority of interrupt 21
    pub mod PRI21 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 22
pub mod NVICIP22 {

    /// Priority of interrupt 22
    pub mod PRI22 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 23
pub mod NVICIP23 {

    /// Priority of interrupt 23
    pub mod PRI23 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 24
pub mod NVICIP24 {

    /// Priority of interrupt 24
    pub mod PRI24 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 25
pub mod NVICIP25 {

    /// Priority of interrupt 25
    pub mod PRI25 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 26
pub mod NVICIP26 {

    /// Priority of interrupt 26
    pub mod PRI26 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 27
pub mod NVICIP27 {

    /// Priority of interrupt 27
    pub mod PRI27 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 28
pub mod NVICIP28 {

    /// Priority of interrupt 28
    pub mod PRI28 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 29
pub mod NVICIP29 {

    /// Priority of interrupt 29
    pub mod PRI29 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 30
pub mod NVICIP30 {

    /// Priority of interrupt 30
    pub mod PRI30 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 31
pub mod NVICIP31 {

    /// Priority of interrupt 31
    pub mod PRI31 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 32
pub mod NVICIP32 {

    /// Priority of interrupt 32
    pub mod PRI32 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 33
pub mod NVICIP33 {

    /// Priority of interrupt 33
    pub mod PRI33 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 34
pub mod NVICIP34 {

    /// Priority of interrupt 34
    pub mod PRI34 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 35
pub mod NVICIP35 {

    /// Priority of interrupt 35
    pub mod PRI35 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 36
pub mod NVICIP36 {

    /// Priority of interrupt 36
    pub mod PRI36 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 37
pub mod NVICIP37 {

    /// Priority of interrupt 37
    pub mod PRI37 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 38
pub mod NVICIP38 {

    /// Priority of interrupt 38
    pub mod PRI38 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 39
pub mod NVICIP39 {

    /// Priority of interrupt 39
    pub mod PRI39 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 40
pub mod NVICIP40 {

    /// Priority of interrupt 40
    pub mod PRI40 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 41
pub mod NVICIP41 {

    /// Priority of interrupt 41
    pub mod PRI41 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 42
pub mod NVICIP42 {

    /// Priority of interrupt 42
    pub mod PRI42 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 43
pub mod NVICIP43 {

    /// Priority of interrupt 43
    pub mod PRI43 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 44
pub mod NVICIP44 {

    /// Priority of interrupt 44
    pub mod PRI44 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 45
pub mod NVICIP45 {

    /// Priority of interrupt 45
    pub mod PRI45 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 46
pub mod NVICIP46 {

    /// Priority of interrupt 46
    pub mod PRI46 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 47
pub mod NVICIP47 {

    /// Priority of interrupt 47
    pub mod PRI47 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 48
pub mod NVICIP48 {

    /// Priority of interrupt 48
    pub mod PRI48 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 49
pub mod NVICIP49 {

    /// Priority of interrupt 49
    pub mod PRI49 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 50
pub mod NVICIP50 {

    /// Priority of interrupt 50
    pub mod PRI50 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 51
pub mod NVICIP51 {

    /// Priority of interrupt 51
    pub mod PRI51 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 52
pub mod NVICIP52 {

    /// Priority of interrupt 52
    pub mod PRI52 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 53
pub mod NVICIP53 {

    /// Priority of interrupt 53
    pub mod PRI53 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 54
pub mod NVICIP54 {

    /// Priority of interrupt 54
    pub mod PRI54 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 55
pub mod NVICIP55 {

    /// Priority of interrupt 55
    pub mod PRI55 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 56
pub mod NVICIP56 {

    /// Priority of interrupt 56
    pub mod PRI56 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 57
pub mod NVICIP57 {

    /// Priority of interrupt 57
    pub mod PRI57 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 58
pub mod NVICIP58 {

    /// Priority of interrupt 58
    pub mod PRI58 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 59
pub mod NVICIP59 {

    /// Priority of interrupt 59
    pub mod PRI59 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 60
pub mod NVICIP60 {

    /// Priority of interrupt 60
    pub mod PRI60 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 61
pub mod NVICIP61 {

    /// Priority of interrupt 61
    pub mod PRI61 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 62
pub mod NVICIP62 {

    /// Priority of interrupt 62
    pub mod PRI62 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 63
pub mod NVICIP63 {

    /// Priority of interrupt 63
    pub mod PRI63 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 64
pub mod NVICIP64 {

    /// Priority of interrupt 64
    pub mod PRI64 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 65
pub mod NVICIP65 {

    /// Priority of interrupt 65
    pub mod PRI65 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 66
pub mod NVICIP66 {

    /// Priority of interrupt 66
    pub mod PRI66 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 67
pub mod NVICIP67 {

    /// Priority of interrupt 67
    pub mod PRI67 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 68
pub mod NVICIP68 {

    /// Priority of interrupt 68
    pub mod PRI68 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 69
pub mod NVICIP69 {

    /// Priority of interrupt 69
    pub mod PRI69 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 70
pub mod NVICIP70 {

    /// Priority of interrupt 70
    pub mod PRI70 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 71
pub mod NVICIP71 {

    /// Priority of interrupt 71
    pub mod PRI71 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 72
pub mod NVICIP72 {

    /// Priority of interrupt 72
    pub mod PRI72 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 73
pub mod NVICIP73 {

    /// Priority of interrupt 73
    pub mod PRI73 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 74
pub mod NVICIP74 {

    /// Priority of interrupt 74
    pub mod PRI74 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 75
pub mod NVICIP75 {

    /// Priority of interrupt 75
    pub mod PRI75 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 76
pub mod NVICIP76 {

    /// Priority of interrupt 76
    pub mod PRI76 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 77
pub mod NVICIP77 {

    /// Priority of interrupt 77
    pub mod PRI77 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 78
pub mod NVICIP78 {

    /// Priority of interrupt 78
    pub mod PRI78 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 79
pub mod NVICIP79 {

    /// Priority of interrupt 79
    pub mod PRI79 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 80
pub mod NVICIP80 {

    /// Priority of interrupt 80
    pub mod PRI80 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 81
pub mod NVICIP81 {

    /// Priority of interrupt 81
    pub mod PRI81 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 82
pub mod NVICIP82 {

    /// Priority of interrupt 82
    pub mod PRI82 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 83
pub mod NVICIP83 {

    /// Priority of interrupt 83
    pub mod PRI83 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 84
pub mod NVICIP84 {

    /// Priority of interrupt 84
    pub mod PRI84 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 85
pub mod NVICIP85 {

    /// Priority of interrupt 85
    pub mod PRI85 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 86
pub mod NVICIP86 {

    /// Priority of interrupt 86
    pub mod PRI86 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 87
pub mod NVICIP87 {

    /// Priority of interrupt 87
    pub mod PRI87 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 88
pub mod NVICIP88 {

    /// Priority of interrupt 88
    pub mod PRI88 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 89
pub mod NVICIP89 {

    /// Priority of interrupt 89
    pub mod PRI89 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 90
pub mod NVICIP90 {

    /// Priority of interrupt 90
    pub mod PRI90 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 91
pub mod NVICIP91 {

    /// Priority of interrupt 91
    pub mod PRI91 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 92
pub mod NVICIP92 {

    /// Priority of interrupt 92
    pub mod PRI92 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 93
pub mod NVICIP93 {

    /// Priority of interrupt 93
    pub mod PRI93 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 94
pub mod NVICIP94 {

    /// Priority of interrupt 94
    pub mod PRI94 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 95
pub mod NVICIP95 {

    /// Priority of interrupt 95
    pub mod PRI95 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 96
pub mod NVICIP96 {

    /// Priority of interrupt 96
    pub mod PRI96 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 97
pub mod NVICIP97 {

    /// Priority of interrupt 97
    pub mod PRI97 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 98
pub mod NVICIP98 {

    /// Priority of interrupt 98
    pub mod PRI98 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 99
pub mod NVICIP99 {

    /// Priority of interrupt 99
    pub mod PRI99 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 100
pub mod NVICIP100 {

    /// Priority of interrupt 100
    pub mod PRI100 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 101
pub mod NVICIP101 {

    /// Priority of interrupt 101
    pub mod PRI101 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 102
pub mod NVICIP102 {

    /// Priority of interrupt 102
    pub mod PRI102 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 103
pub mod NVICIP103 {

    /// Priority of interrupt 103
    pub mod PRI103 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 104
pub mod NVICIP104 {

    /// Priority of interrupt 104
    pub mod PRI104 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 105
pub mod NVICIP105 {

    /// Priority of interrupt 105
    pub mod PRI105 {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Software Trigger Interrupt Register
pub mod NVICSTIR {

    /// Interrupt ID of the interrupt to trigger, in the range 0-239. For example, a value of 0x03 specifies interrupt IRQ3.
    pub mod INTID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
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
    /// Interrupt Set Enable Register n
    pub NVICISER0: RWRegister<u32>,

    /// Interrupt Set Enable Register n
    pub NVICISER1: RWRegister<u32>,

    /// Interrupt Set Enable Register n
    pub NVICISER2: RWRegister<u32>,

    /// Interrupt Set Enable Register n
    pub NVICISER3: RWRegister<u32>,

    _reserved1: [u32; 28],

    /// Interrupt Clear Enable Register n
    pub NVICICER0: RWRegister<u32>,

    /// Interrupt Clear Enable Register n
    pub NVICICER1: RWRegister<u32>,

    /// Interrupt Clear Enable Register n
    pub NVICICER2: RWRegister<u32>,

    /// Interrupt Clear Enable Register n
    pub NVICICER3: RWRegister<u32>,

    _reserved2: [u32; 28],

    /// Interrupt Set Pending Register n
    pub NVICISPR0: RWRegister<u32>,

    /// Interrupt Set Pending Register n
    pub NVICISPR1: RWRegister<u32>,

    /// Interrupt Set Pending Register n
    pub NVICISPR2: RWRegister<u32>,

    /// Interrupt Set Pending Register n
    pub NVICISPR3: RWRegister<u32>,

    _reserved3: [u32; 28],

    /// Interrupt Clear Pending Register n
    pub NVICICPR0: RWRegister<u32>,

    /// Interrupt Clear Pending Register n
    pub NVICICPR1: RWRegister<u32>,

    /// Interrupt Clear Pending Register n
    pub NVICICPR2: RWRegister<u32>,

    /// Interrupt Clear Pending Register n
    pub NVICICPR3: RWRegister<u32>,

    _reserved4: [u32; 28],

    /// Interrupt Active bit Register n
    pub NVICIABR0: RWRegister<u32>,

    /// Interrupt Active bit Register n
    pub NVICIABR1: RWRegister<u32>,

    /// Interrupt Active bit Register n
    pub NVICIABR2: RWRegister<u32>,

    /// Interrupt Active bit Register n
    pub NVICIABR3: RWRegister<u32>,

    _reserved5: [u32; 60],

    /// Interrupt Priority Register 0
    pub NVICIP0: RWRegister<u8>,

    /// Interrupt Priority Register 1
    pub NVICIP1: RWRegister<u8>,

    /// Interrupt Priority Register 2
    pub NVICIP2: RWRegister<u8>,

    /// Interrupt Priority Register 3
    pub NVICIP3: RWRegister<u8>,

    /// Interrupt Priority Register 4
    pub NVICIP4: RWRegister<u8>,

    /// Interrupt Priority Register 5
    pub NVICIP5: RWRegister<u8>,

    /// Interrupt Priority Register 6
    pub NVICIP6: RWRegister<u8>,

    /// Interrupt Priority Register 7
    pub NVICIP7: RWRegister<u8>,

    /// Interrupt Priority Register 8
    pub NVICIP8: RWRegister<u8>,

    /// Interrupt Priority Register 9
    pub NVICIP9: RWRegister<u8>,

    /// Interrupt Priority Register 10
    pub NVICIP10: RWRegister<u8>,

    /// Interrupt Priority Register 11
    pub NVICIP11: RWRegister<u8>,

    /// Interrupt Priority Register 12
    pub NVICIP12: RWRegister<u8>,

    /// Interrupt Priority Register 13
    pub NVICIP13: RWRegister<u8>,

    /// Interrupt Priority Register 14
    pub NVICIP14: RWRegister<u8>,

    /// Interrupt Priority Register 15
    pub NVICIP15: RWRegister<u8>,

    /// Interrupt Priority Register 16
    pub NVICIP16: RWRegister<u8>,

    /// Interrupt Priority Register 17
    pub NVICIP17: RWRegister<u8>,

    /// Interrupt Priority Register 18
    pub NVICIP18: RWRegister<u8>,

    /// Interrupt Priority Register 19
    pub NVICIP19: RWRegister<u8>,

    /// Interrupt Priority Register 20
    pub NVICIP20: RWRegister<u8>,

    /// Interrupt Priority Register 21
    pub NVICIP21: RWRegister<u8>,

    /// Interrupt Priority Register 22
    pub NVICIP22: RWRegister<u8>,

    /// Interrupt Priority Register 23
    pub NVICIP23: RWRegister<u8>,

    /// Interrupt Priority Register 24
    pub NVICIP24: RWRegister<u8>,

    /// Interrupt Priority Register 25
    pub NVICIP25: RWRegister<u8>,

    /// Interrupt Priority Register 26
    pub NVICIP26: RWRegister<u8>,

    /// Interrupt Priority Register 27
    pub NVICIP27: RWRegister<u8>,

    /// Interrupt Priority Register 28
    pub NVICIP28: RWRegister<u8>,

    /// Interrupt Priority Register 29
    pub NVICIP29: RWRegister<u8>,

    /// Interrupt Priority Register 30
    pub NVICIP30: RWRegister<u8>,

    /// Interrupt Priority Register 31
    pub NVICIP31: RWRegister<u8>,

    /// Interrupt Priority Register 32
    pub NVICIP32: RWRegister<u8>,

    /// Interrupt Priority Register 33
    pub NVICIP33: RWRegister<u8>,

    /// Interrupt Priority Register 34
    pub NVICIP34: RWRegister<u8>,

    /// Interrupt Priority Register 35
    pub NVICIP35: RWRegister<u8>,

    /// Interrupt Priority Register 36
    pub NVICIP36: RWRegister<u8>,

    /// Interrupt Priority Register 37
    pub NVICIP37: RWRegister<u8>,

    /// Interrupt Priority Register 38
    pub NVICIP38: RWRegister<u8>,

    /// Interrupt Priority Register 39
    pub NVICIP39: RWRegister<u8>,

    /// Interrupt Priority Register 40
    pub NVICIP40: RWRegister<u8>,

    /// Interrupt Priority Register 41
    pub NVICIP41: RWRegister<u8>,

    /// Interrupt Priority Register 42
    pub NVICIP42: RWRegister<u8>,

    /// Interrupt Priority Register 43
    pub NVICIP43: RWRegister<u8>,

    /// Interrupt Priority Register 44
    pub NVICIP44: RWRegister<u8>,

    /// Interrupt Priority Register 45
    pub NVICIP45: RWRegister<u8>,

    /// Interrupt Priority Register 46
    pub NVICIP46: RWRegister<u8>,

    /// Interrupt Priority Register 47
    pub NVICIP47: RWRegister<u8>,

    /// Interrupt Priority Register 48
    pub NVICIP48: RWRegister<u8>,

    /// Interrupt Priority Register 49
    pub NVICIP49: RWRegister<u8>,

    /// Interrupt Priority Register 50
    pub NVICIP50: RWRegister<u8>,

    /// Interrupt Priority Register 51
    pub NVICIP51: RWRegister<u8>,

    /// Interrupt Priority Register 52
    pub NVICIP52: RWRegister<u8>,

    /// Interrupt Priority Register 53
    pub NVICIP53: RWRegister<u8>,

    /// Interrupt Priority Register 54
    pub NVICIP54: RWRegister<u8>,

    /// Interrupt Priority Register 55
    pub NVICIP55: RWRegister<u8>,

    /// Interrupt Priority Register 56
    pub NVICIP56: RWRegister<u8>,

    /// Interrupt Priority Register 57
    pub NVICIP57: RWRegister<u8>,

    /// Interrupt Priority Register 58
    pub NVICIP58: RWRegister<u8>,

    /// Interrupt Priority Register 59
    pub NVICIP59: RWRegister<u8>,

    /// Interrupt Priority Register 60
    pub NVICIP60: RWRegister<u8>,

    /// Interrupt Priority Register 61
    pub NVICIP61: RWRegister<u8>,

    /// Interrupt Priority Register 62
    pub NVICIP62: RWRegister<u8>,

    /// Interrupt Priority Register 63
    pub NVICIP63: RWRegister<u8>,

    /// Interrupt Priority Register 64
    pub NVICIP64: RWRegister<u8>,

    /// Interrupt Priority Register 65
    pub NVICIP65: RWRegister<u8>,

    /// Interrupt Priority Register 66
    pub NVICIP66: RWRegister<u8>,

    /// Interrupt Priority Register 67
    pub NVICIP67: RWRegister<u8>,

    /// Interrupt Priority Register 68
    pub NVICIP68: RWRegister<u8>,

    /// Interrupt Priority Register 69
    pub NVICIP69: RWRegister<u8>,

    /// Interrupt Priority Register 70
    pub NVICIP70: RWRegister<u8>,

    /// Interrupt Priority Register 71
    pub NVICIP71: RWRegister<u8>,

    /// Interrupt Priority Register 72
    pub NVICIP72: RWRegister<u8>,

    /// Interrupt Priority Register 73
    pub NVICIP73: RWRegister<u8>,

    /// Interrupt Priority Register 74
    pub NVICIP74: RWRegister<u8>,

    /// Interrupt Priority Register 75
    pub NVICIP75: RWRegister<u8>,

    /// Interrupt Priority Register 76
    pub NVICIP76: RWRegister<u8>,

    /// Interrupt Priority Register 77
    pub NVICIP77: RWRegister<u8>,

    /// Interrupt Priority Register 78
    pub NVICIP78: RWRegister<u8>,

    /// Interrupt Priority Register 79
    pub NVICIP79: RWRegister<u8>,

    /// Interrupt Priority Register 80
    pub NVICIP80: RWRegister<u8>,

    /// Interrupt Priority Register 81
    pub NVICIP81: RWRegister<u8>,

    /// Interrupt Priority Register 82
    pub NVICIP82: RWRegister<u8>,

    /// Interrupt Priority Register 83
    pub NVICIP83: RWRegister<u8>,

    /// Interrupt Priority Register 84
    pub NVICIP84: RWRegister<u8>,

    /// Interrupt Priority Register 85
    pub NVICIP85: RWRegister<u8>,

    /// Interrupt Priority Register 86
    pub NVICIP86: RWRegister<u8>,

    /// Interrupt Priority Register 87
    pub NVICIP87: RWRegister<u8>,

    /// Interrupt Priority Register 88
    pub NVICIP88: RWRegister<u8>,

    /// Interrupt Priority Register 89
    pub NVICIP89: RWRegister<u8>,

    /// Interrupt Priority Register 90
    pub NVICIP90: RWRegister<u8>,

    /// Interrupt Priority Register 91
    pub NVICIP91: RWRegister<u8>,

    /// Interrupt Priority Register 92
    pub NVICIP92: RWRegister<u8>,

    /// Interrupt Priority Register 93
    pub NVICIP93: RWRegister<u8>,

    /// Interrupt Priority Register 94
    pub NVICIP94: RWRegister<u8>,

    /// Interrupt Priority Register 95
    pub NVICIP95: RWRegister<u8>,

    /// Interrupt Priority Register 96
    pub NVICIP96: RWRegister<u8>,

    /// Interrupt Priority Register 97
    pub NVICIP97: RWRegister<u8>,

    /// Interrupt Priority Register 98
    pub NVICIP98: RWRegister<u8>,

    /// Interrupt Priority Register 99
    pub NVICIP99: RWRegister<u8>,

    /// Interrupt Priority Register 100
    pub NVICIP100: RWRegister<u8>,

    /// Interrupt Priority Register 101
    pub NVICIP101: RWRegister<u8>,

    /// Interrupt Priority Register 102
    pub NVICIP102: RWRegister<u8>,

    /// Interrupt Priority Register 103
    pub NVICIP103: RWRegister<u8>,

    /// Interrupt Priority Register 104
    pub NVICIP104: RWRegister<u8>,

    /// Interrupt Priority Register 105
    pub NVICIP105: RWRegister<u8>,

    _reserved6: [u16; 1355],

    /// Software Trigger Interrupt Register
    pub NVICSTIR: RWRegister<u32>,
}
pub struct ResetValues {
    pub NVICISER0: u32,
    pub NVICISER1: u32,
    pub NVICISER2: u32,
    pub NVICISER3: u32,
    pub NVICICER0: u32,
    pub NVICICER1: u32,
    pub NVICICER2: u32,
    pub NVICICER3: u32,
    pub NVICISPR0: u32,
    pub NVICISPR1: u32,
    pub NVICISPR2: u32,
    pub NVICISPR3: u32,
    pub NVICICPR0: u32,
    pub NVICICPR1: u32,
    pub NVICICPR2: u32,
    pub NVICICPR3: u32,
    pub NVICIABR0: u32,
    pub NVICIABR1: u32,
    pub NVICIABR2: u32,
    pub NVICIABR3: u32,
    pub NVICIP0: u8,
    pub NVICIP1: u8,
    pub NVICIP2: u8,
    pub NVICIP3: u8,
    pub NVICIP4: u8,
    pub NVICIP5: u8,
    pub NVICIP6: u8,
    pub NVICIP7: u8,
    pub NVICIP8: u8,
    pub NVICIP9: u8,
    pub NVICIP10: u8,
    pub NVICIP11: u8,
    pub NVICIP12: u8,
    pub NVICIP13: u8,
    pub NVICIP14: u8,
    pub NVICIP15: u8,
    pub NVICIP16: u8,
    pub NVICIP17: u8,
    pub NVICIP18: u8,
    pub NVICIP19: u8,
    pub NVICIP20: u8,
    pub NVICIP21: u8,
    pub NVICIP22: u8,
    pub NVICIP23: u8,
    pub NVICIP24: u8,
    pub NVICIP25: u8,
    pub NVICIP26: u8,
    pub NVICIP27: u8,
    pub NVICIP28: u8,
    pub NVICIP29: u8,
    pub NVICIP30: u8,
    pub NVICIP31: u8,
    pub NVICIP32: u8,
    pub NVICIP33: u8,
    pub NVICIP34: u8,
    pub NVICIP35: u8,
    pub NVICIP36: u8,
    pub NVICIP37: u8,
    pub NVICIP38: u8,
    pub NVICIP39: u8,
    pub NVICIP40: u8,
    pub NVICIP41: u8,
    pub NVICIP42: u8,
    pub NVICIP43: u8,
    pub NVICIP44: u8,
    pub NVICIP45: u8,
    pub NVICIP46: u8,
    pub NVICIP47: u8,
    pub NVICIP48: u8,
    pub NVICIP49: u8,
    pub NVICIP50: u8,
    pub NVICIP51: u8,
    pub NVICIP52: u8,
    pub NVICIP53: u8,
    pub NVICIP54: u8,
    pub NVICIP55: u8,
    pub NVICIP56: u8,
    pub NVICIP57: u8,
    pub NVICIP58: u8,
    pub NVICIP59: u8,
    pub NVICIP60: u8,
    pub NVICIP61: u8,
    pub NVICIP62: u8,
    pub NVICIP63: u8,
    pub NVICIP64: u8,
    pub NVICIP65: u8,
    pub NVICIP66: u8,
    pub NVICIP67: u8,
    pub NVICIP68: u8,
    pub NVICIP69: u8,
    pub NVICIP70: u8,
    pub NVICIP71: u8,
    pub NVICIP72: u8,
    pub NVICIP73: u8,
    pub NVICIP74: u8,
    pub NVICIP75: u8,
    pub NVICIP76: u8,
    pub NVICIP77: u8,
    pub NVICIP78: u8,
    pub NVICIP79: u8,
    pub NVICIP80: u8,
    pub NVICIP81: u8,
    pub NVICIP82: u8,
    pub NVICIP83: u8,
    pub NVICIP84: u8,
    pub NVICIP85: u8,
    pub NVICIP86: u8,
    pub NVICIP87: u8,
    pub NVICIP88: u8,
    pub NVICIP89: u8,
    pub NVICIP90: u8,
    pub NVICIP91: u8,
    pub NVICIP92: u8,
    pub NVICIP93: u8,
    pub NVICIP94: u8,
    pub NVICIP95: u8,
    pub NVICIP96: u8,
    pub NVICIP97: u8,
    pub NVICIP98: u8,
    pub NVICIP99: u8,
    pub NVICIP100: u8,
    pub NVICIP101: u8,
    pub NVICIP102: u8,
    pub NVICIP103: u8,
    pub NVICIP104: u8,
    pub NVICIP105: u8,
    pub NVICSTIR: u32,
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
