#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIO
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister, WORegister};

/// GPIO data register
pub mod DR {

    /// DR data bits
    pub mod DR {
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

/// GPIO direction register
pub mod GDIR {

    /// GPIO direction bits
    pub mod GDIR {
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

/// GPIO pad status register
pub mod PSR {

    /// GPIO pad status bits
    pub mod PSR {
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

/// GPIO interrupt configuration register1
pub mod ICR1 {

    /// Interrupt configuration field for GPIO interrupt 0
    pub mod ICR0 {
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

            /// 0b00: Interrupt 0 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 0 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 0 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 0 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 1
    pub mod ICR1 {
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

            /// 0b00: Interrupt 1 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 1 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 1 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 1 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 2
    pub mod ICR2 {
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

            /// 0b00: Interrupt 2 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 2 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 2 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 2 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 3
    pub mod ICR3 {
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

            /// 0b00: Interrupt 3 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 3 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 3 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 3 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 4
    pub mod ICR4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 4 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 4 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 4 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 4 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 5
    pub mod ICR5 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 5 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 5 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 5 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 5 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 6
    pub mod ICR6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 6 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 6 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 6 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 6 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 7
    pub mod ICR7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 7 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 7 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 7 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 7 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 8
    pub mod ICR8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 8 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 8 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 8 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 8 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 9
    pub mod ICR9 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 9 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 9 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 9 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 9 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 10
    pub mod ICR10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 10 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 10 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 10 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 10 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 11
    pub mod ICR11 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 11 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 11 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 11 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 11 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 12
    pub mod ICR12 {
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

            /// 0b00: Interrupt 12 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 12 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 12 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 12 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 13
    pub mod ICR13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 13 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 13 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 13 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 13 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 14
    pub mod ICR14 {
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

            /// 0b00: Interrupt 14 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 14 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 14 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 14 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 15
    pub mod ICR15 {
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

            /// 0b00: Interrupt 15 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 15 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 15 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 15 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }
}

/// GPIO interrupt configuration register2
pub mod ICR2 {

    /// Interrupt configuration field for GPIO interrupt 16
    pub mod ICR16 {
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

            /// 0b00: Interrupt 16 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 16 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 16 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 16 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 17
    pub mod ICR17 {
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

            /// 0b00: Interrupt 17 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 17 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 17 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 17 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 18
    pub mod ICR18 {
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

            /// 0b00: Interrupt 18 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 18 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 18 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 18 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 19
    pub mod ICR19 {
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

            /// 0b00: Interrupt 19 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 19 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 19 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 19 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 20
    pub mod ICR20 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 20 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 20 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 20 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 20 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 21
    pub mod ICR21 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 21 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 21 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 21 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 21 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 22
    pub mod ICR22 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 22 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 22 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 22 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 22 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 23
    pub mod ICR23 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 23 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 23 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 23 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 23 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 24
    pub mod ICR24 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 24 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 24 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 24 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 24 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 25
    pub mod ICR25 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 25 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 25 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 25 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 25 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 26
    pub mod ICR26 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 26 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 26 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 26 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 26 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 27
    pub mod ICR27 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 27 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 27 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 27 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 27 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 28
    pub mod ICR28 {
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

            /// 0b00: Interrupt 28 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 28 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 28 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 28 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 29
    pub mod ICR29 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Interrupt 29 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 29 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 29 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 29 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 30
    pub mod ICR30 {
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

            /// 0b00: Interrupt 30 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 30 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 30 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 30 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }

    /// Interrupt configuration field for GPIO interrupt 31
    pub mod ICR31 {
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

            /// 0b00: Interrupt 31 is low-level sensitive.
            pub const LOW_LEVEL: u32 = 0b00;

            /// 0b01: Interrupt 31 is high-level sensitive.
            pub const HIGH_LEVEL: u32 = 0b01;

            /// 0b10: Interrupt 31 is rising-edge sensitive.
            pub const RISING_EDGE: u32 = 0b10;

            /// 0b11: Interrupt 31 is falling-edge sensitive.
            pub const FALLING_EDGE: u32 = 0b11;
        }
    }
}

/// GPIO interrupt mask register
pub mod IMR {

    /// Interrupt Mask bits
    pub mod IMR {
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

/// GPIO interrupt status register
pub mod ISR {

    /// Interrupt status bits
    pub mod ISR {
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

/// GPIO edge select register
pub mod EDGE_SEL {

    /// Edge select
    pub mod GPIO_EDGE_SEL {
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

/// GPIO data register SET
pub mod DR_SET {

    /// Set
    pub mod DR_SET {
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

/// GPIO data register CLEAR
pub mod DR_CLEAR {

    /// Clear
    pub mod DR_CLEAR {
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

/// GPIO data register TOGGLE
pub mod DR_TOGGLE {

    /// Toggle
    pub mod DR_TOGGLE {
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
    /// GPIO data register
    pub DR: RWRegister<u32>,

    /// GPIO direction register
    pub GDIR: RWRegister<u32>,

    /// GPIO pad status register
    pub PSR: RORegister<u32>,

    /// GPIO interrupt configuration register1
    pub ICR1: RWRegister<u32>,

    /// GPIO interrupt configuration register2
    pub ICR2: RWRegister<u32>,

    /// GPIO interrupt mask register
    pub IMR: RWRegister<u32>,

    /// GPIO interrupt status register
    pub ISR: RWRegister<u32>,

    /// GPIO edge select register
    pub EDGE_SEL: RWRegister<u32>,

    _reserved1: [u32; 25],

    /// GPIO data register SET
    pub DR_SET: WORegister<u32>,

    /// GPIO data register CLEAR
    pub DR_CLEAR: WORegister<u32>,

    /// GPIO data register TOGGLE
    pub DR_TOGGLE: WORegister<u32>,
}
pub struct ResetValues {
    pub DR: u32,
    pub GDIR: u32,
    pub PSR: u32,
    pub ICR1: u32,
    pub ICR2: u32,
    pub IMR: u32,
    pub ISR: u32,
    pub EDGE_SEL: u32,
    pub DR_SET: u32,
    pub DR_CLEAR: u32,
    pub DR_TOGGLE: u32,
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
