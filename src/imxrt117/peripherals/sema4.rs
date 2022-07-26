#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IPS_Semaphores
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Semaphores Gate n Register
pub mod Gate0 {

    /// Gate Finite State Machine.
    pub mod GTFSM {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u8 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: The gate is unlocked (free).
            pub const GTFSM_0: u8 = 0b00;

            /// 0b01: The gate has been locked by processor 0.
            pub const GTFSM_1: u8 = 0b01;

            /// 0b10: The gate has been locked by processor 1.
            pub const GTFSM_2: u8 = 0b10;

            /// 0b11: This state encoding is never used and therefore reserved. Attempted writes of 0x03 are treated as "no operation" and do not affect the gate state machine.
            pub const GTFSM_3: u8 = 0b11;
        }
    }
}

/// Semaphores Gate n Register
pub mod Gate1 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate2 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate3 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate4 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate5 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate6 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate7 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate8 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate9 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate10 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate11 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate12 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate13 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate14 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Gate n Register
pub mod Gate15 {
    pub use super::Gate0::GTFSM;
}

/// Semaphores Processor n IRQ Notification Enable
pub mod CP0INE {

    /// Interrupt Request Notification Enable 7. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 7.
    pub mod INE7 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE7_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE7_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 6. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 6.
    pub mod INE6 {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE6_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE6_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 5. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 5.
    pub mod INE5 {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE5_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE5_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 4. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 4.
    pub mod INE4 {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE4_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE4_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 3
    pub mod INE3 {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE3_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE3_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 2
    pub mod INE2 {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE2_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE2_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 1
    pub mod INE1 {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE1_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE1_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 0
    pub mod INE0 {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE0_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE0_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 15. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 15.
    pub mod INE15 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE15_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE15_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 14. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 14.
    pub mod INE14 {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE14_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE14_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 13. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 13.
    pub mod INE13 {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE13_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE13_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 12. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 12.
    pub mod INE12 {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE12_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE12_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 11. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 11.
    pub mod INE11 {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE11_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE11_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 10. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 10.
    pub mod INE10 {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE10_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE10_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 9. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 9.
    pub mod INE9 {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE9_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE9_1: u16 = 0b1;
        }
    }

    /// Interrupt Request Notification Enable 8. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 8.
    pub mod INE8 {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The generation of the notification interrupt is disabled.
            pub const INE8_0: u16 = 0b0;

            /// 0b1: The generation of the notification interrupt is enabled.
            pub const INE8_1: u16 = 0b1;
        }
    }
}

/// Semaphores Processor n IRQ Notification Enable
pub mod CP1INE {
    pub use super::CP0INE::INE0;
    pub use super::CP0INE::INE1;
    pub use super::CP0INE::INE10;
    pub use super::CP0INE::INE11;
    pub use super::CP0INE::INE12;
    pub use super::CP0INE::INE13;
    pub use super::CP0INE::INE14;
    pub use super::CP0INE::INE15;
    pub use super::CP0INE::INE2;
    pub use super::CP0INE::INE3;
    pub use super::CP0INE::INE4;
    pub use super::CP0INE::INE5;
    pub use super::CP0INE::INE6;
    pub use super::CP0INE::INE7;
    pub use super::CP0INE::INE8;
    pub use super::CP0INE::INE9;
}

/// Semaphores Processor n IRQ Notification
pub mod CP0NTF {

    /// Gate 7 Notification
    pub mod GN7 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 6 Notification
    pub mod GN6 {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 5 Notification
    pub mod GN5 {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 4 Notification
    pub mod GN4 {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 3 Notification
    pub mod GN3 {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 2 Notification
    pub mod GN2 {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 1 Notification
    pub mod GN1 {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 0 Notification
    pub mod GN0 {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 15 Notification
    pub mod GN15 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 14 Notification
    pub mod GN14 {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 13 Notification
    pub mod GN13 {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 12 Notification
    pub mod GN12 {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 11 Notification
    pub mod GN11 {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 10 Notification
    pub mod GN10 {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 9 Notification
    pub mod GN9 {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gate 8 Notification
    pub mod GN8 {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Semaphores Processor n IRQ Notification
pub mod CP1NTF {
    pub use super::CP0NTF::GN0;
    pub use super::CP0NTF::GN1;
    pub use super::CP0NTF::GN10;
    pub use super::CP0NTF::GN11;
    pub use super::CP0NTF::GN12;
    pub use super::CP0NTF::GN13;
    pub use super::CP0NTF::GN14;
    pub use super::CP0NTF::GN15;
    pub use super::CP0NTF::GN2;
    pub use super::CP0NTF::GN3;
    pub use super::CP0NTF::GN4;
    pub use super::CP0NTF::GN5;
    pub use super::CP0NTF::GN6;
    pub use super::CP0NTF::GN7;
    pub use super::CP0NTF::GN8;
    pub use super::CP0NTF::GN9;
}

/// Semaphores (Secure) Reset Gate n
pub mod RSTGT {

    /// This field contains sub-fields that vary depending on whether it is being read or written
    pub mod RSTGSM_RSTGMS_RSTGDP {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reset Gate Number
    pub mod RSTGTN {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Semaphores (Secure) Reset IRQ Notification
pub mod RSTNTF {

    /// This field contains sub-fields that vary depending on whether it is being read or written
    pub mod RSTNSM_RSTNMS_RSTNDP {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reset Notification Number
    pub mod RSTNTN {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u16 = 0xff << offset;
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
    /// Semaphores Gate n Register
    pub Gate0: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate1: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate2: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate3: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate4: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate5: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate6: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate7: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate8: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate9: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate10: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate11: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate12: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate13: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate14: RWRegister<u8>,

    /// Semaphores Gate n Register
    pub Gate15: RWRegister<u8>,

    _reserved1: [u32; 12],

    /// Semaphores Processor n IRQ Notification Enable
    pub CP0INE: RWRegister<u16>,

    _reserved2: [u16; 3],

    /// Semaphores Processor n IRQ Notification Enable
    pub CP1INE: RWRegister<u16>,

    _reserved3: [u16; 27],

    /// Semaphores Processor n IRQ Notification
    pub CP0NTF: RORegister<u16>,

    _reserved4: [u16; 3],

    /// Semaphores Processor n IRQ Notification
    pub CP1NTF: RORegister<u16>,

    _reserved5: [u16; 59],

    /// Semaphores (Secure) Reset Gate n
    pub RSTGT: RWRegister<u16>,

    _reserved6: [u16; 1],

    /// Semaphores (Secure) Reset IRQ Notification
    pub RSTNTF: RWRegister<u16>,
}
pub struct ResetValues {
    pub Gate0: u8,
    pub Gate1: u8,
    pub Gate2: u8,
    pub Gate3: u8,
    pub Gate4: u8,
    pub Gate5: u8,
    pub Gate6: u8,
    pub Gate7: u8,
    pub Gate8: u8,
    pub Gate9: u8,
    pub Gate10: u8,
    pub Gate11: u8,
    pub Gate12: u8,
    pub Gate13: u8,
    pub Gate14: u8,
    pub Gate15: u8,
    pub CP0INE: u16,
    pub CP1INE: u16,
    pub CP0NTF: u16,
    pub CP1NTF: u16,
    pub RSTGT: u16,
    pub RSTNTF: u16,
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
