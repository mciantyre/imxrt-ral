#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CDOG
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister, WORegister};

/// Control
pub mod CONTROL {

    /// Lock control
    pub mod LOCK_CTRL {
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

            /// 0b01: Locked
            pub const LOCKED: u32 = 0b01;

            /// 0b10: Unlocked
            pub const UNLOCKED: u32 = 0b10;
        }
    }

    /// TIMEOUT fault control
    pub mod TIMEOUT_CTRL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (3 bits: 0b111 << 2)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b001: Enable reset
            pub const ENABLE_RESET: u32 = 0b001;

            /// 0b010: Enable interrupt
            pub const ENABLE_INTERRUPT: u32 = 0b010;

            /// 0b100: Disable both reset and interrupt
            pub const DISABLE_BOTH: u32 = 0b100;
        }
    }

    /// MISCOMPARE fault control
    pub mod MISCOMPARE_CTRL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (3 bits: 0b111 << 5)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEOUT_CTRL::RW;
    }

    /// SEQUENCE fault control
    pub mod SEQUENCE_CTRL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEOUT_CTRL::RW;
    }

    /// CONTROL fault control
    pub mod CONTROL_CTRL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (3 bits: 0b111 << 11)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b001: Enable reset
            pub const ENABLE_RESET: u32 = 0b001;

            /// 0b100: Disable reset
            pub const DISABLE_BOTH: u32 = 0b100;
        }
    }

    /// STATE fault control
    pub mod STATE_CTRL {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (3 bits: 0b111 << 14)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEOUT_CTRL::RW;
    }

    /// ADDRESS fault control
    pub mod ADDRESS_CTRL {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (3 bits: 0b111 << 17)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TIMEOUT_CTRL::RW;
    }

    /// IRQ pause control
    pub mod IRQ_PAUSE {
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

            /// 0b01: Keep the timer running
            pub const RUN_TIMER: u32 = 0b01;

            /// 0b10: Stop the timer
            pub const PAUSE_TIMER: u32 = 0b10;
        }
    }

    /// DEBUG_HALT control
    pub mod DEBUG_HALT_CTRL {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IRQ_PAUSE::RW;
    }
}

/// Instruction Timer reload
pub mod RELOAD {

    /// Instruction Timer reload value
    pub mod RLOAD {
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

/// Instruction Timer
pub mod INSTRUCTION_TIMER {

    /// Current value of the Instruction Timer
    pub mod INSTIM {
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

/// Secure Counter
pub mod SECURE_COUNTER {

    /// Secure Counter
    pub mod SECCNT {
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

/// Status 1
pub mod STATUS {

    /// Number of TIMEOUT faults since the last POR
    pub mod NUMTOF {
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

    /// Number of MISCOMPARE faults since the last POR
    pub mod NUMMISCOMPF {
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

    /// Number of SEQUENCE faults since the last POR
    pub mod NUMILSEQF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Current State
    pub mod CURST {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Status 2
pub mod STATUS2 {

    /// Number of CONTROL faults since the last POR
    pub mod NUMCNTF {
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

    /// Number of STATE faults since the last POR
    pub mod NUMILLSTF {
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

    /// Number of ADDRESS faults since the last POR
    pub mod NUMILLA {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flags
pub mod FLAGS {

    /// TIMEOUT fault flag
    pub mod TO_FLAG {
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

            /// 0b0: A TIMEOUT fault has not occurred
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: A TIMEOUT fault has occurred
            pub const FLAG: u32 = 0b1;
        }
    }

    /// MISCOMPARE fault flag
    pub mod MISCOM_FLAG {
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

            /// 0b0: A MISCOMPARE fault has not occurred
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: A MISCOMPARE fault has occurred
            pub const FLAG: u32 = 0b1;
        }
    }

    /// SEQUENCE fault flag
    pub mod SEQ_FLAG {
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

            /// 0b0: A SEQUENCE fault has not occurred
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: A SEQUENCE fault has occurred
            pub const FLAG: u32 = 0b1;
        }
    }

    /// CONTROL fault flag
    pub mod CNT_FLAG {
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

            /// 0b0: A CONTROL fault has not occurred
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: A CONTROL fault has occurred
            pub const FLAG: u32 = 0b1;
        }
    }

    /// STATE fault flag
    pub mod STATE_FLAG {
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

            /// 0b0: A STATE fault has not occurred
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: A STATE fault has occurred
            pub const FLAG: u32 = 0b1;
        }
    }

    /// ADDRESS fault flag
    pub mod ADDR_FLAG {
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

            /// 0b0: An ADDRESS fault has not occurred
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: An ADDRESS fault has occurred
            pub const FLAG: u32 = 0b1;
        }
    }

    /// Power-on reset flag
    pub mod POR_FLAG {
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

            /// 0b0: A Power-on reset event has not occurred
            pub const NO_FLAG: u32 = 0b0;

            /// 0b1: A Power-on reset event has occurred
            pub const FLAG: u32 = 0b1;
        }
    }
}

/// Persistent Data Storage
pub mod PERSISTENT {

    /// Persistent Storage
    pub mod PERSIS {
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

/// START Command
pub mod START {

    /// Start command
    pub mod STRT {
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

/// STOP Command
pub mod STOP {

    /// Stop command
    pub mod STP {
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

/// RESTART Command
pub mod RESTART {

    /// Restart command
    pub mod RSTRT {
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

/// ADD Command
pub mod ADD {

    /// ADD Write Value
    pub mod AD {
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

/// ADD1 Command
pub mod ADD1 {

    /// ADD 1
    pub mod AD1 {
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

/// ADD16 Command
pub mod ADD16 {

    /// ADD 16
    pub mod AD16 {
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

/// ADD256 Command
pub mod ADD256 {

    /// ADD 256
    pub mod AD256 {
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

/// SUB Command
pub mod SUB {

    /// Subtract Write Value
    pub mod S0B {
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

/// SUB1 Command
pub mod SUB1 {

    /// Subtract 1
    pub mod S1B {
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

/// SUB16 Command
pub mod SUB16 {

    /// Subtract 16
    pub mod SB16 {
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

/// SUB256 Command
pub mod SUB256 {

    /// Subtract 256
    pub mod SB256 {
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
    /// Control
    pub CONTROL: RWRegister<u32>,

    /// Instruction Timer reload
    pub RELOAD: RWRegister<u32>,

    /// Instruction Timer
    pub INSTRUCTION_TIMER: RWRegister<u32>,

    /// Secure Counter
    pub SECURE_COUNTER: WORegister<u32>,

    /// Status 1
    pub STATUS: RORegister<u32>,

    /// Status 2
    pub STATUS2: RORegister<u32>,

    /// Flags
    pub FLAGS: RWRegister<u32>,

    /// Persistent Data Storage
    pub PERSISTENT: RWRegister<u32>,

    /// START Command
    pub START: WORegister<u32>,

    /// STOP Command
    pub STOP: WORegister<u32>,

    /// RESTART Command
    pub RESTART: WORegister<u32>,

    /// ADD Command
    pub ADD: WORegister<u32>,

    /// ADD1 Command
    pub ADD1: WORegister<u32>,

    /// ADD16 Command
    pub ADD16: WORegister<u32>,

    /// ADD256 Command
    pub ADD256: WORegister<u32>,

    /// SUB Command
    pub SUB: WORegister<u32>,

    /// SUB1 Command
    pub SUB1: WORegister<u32>,

    /// SUB16 Command
    pub SUB16: WORegister<u32>,

    /// SUB256 Command
    pub SUB256: WORegister<u32>,
}
pub struct ResetValues {
    pub CONTROL: u32,
    pub RELOAD: u32,
    pub INSTRUCTION_TIMER: u32,
    pub SECURE_COUNTER: u32,
    pub STATUS: u32,
    pub STATUS2: u32,
    pub FLAGS: u32,
    pub PERSISTENT: u32,
    pub START: u32,
    pub STOP: u32,
    pub RESTART: u32,
    pub ADD: u32,
    pub ADD1: u32,
    pub ADD16: u32,
    pub ADD256: u32,
    pub SUB: u32,
    pub SUB1: u32,
    pub SUB16: u32,
    pub SUB256: u32,
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
