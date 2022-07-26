#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SEMA42
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// Gate Register
pub mod GATE0 {

    /// Gate Finite State Machine.
    pub mod GTFSM {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: The gate is unlocked (free).
            pub const GTFSM_0: u8 = 0b0000;

            /// 0b0001: The gate has been locked by processor with master_index = 0.
            pub const GTFSM_1: u8 = 0b0001;

            /// 0b0010: The gate has been locked by processor with master_index = 1.
            pub const GTFSM_2: u8 = 0b0010;

            /// 0b0011: The gate has been locked by processor with master_index = 2.
            pub const GTFSM_3: u8 = 0b0011;

            /// 0b0100: The gate has been locked by processor with master_index = 3.
            pub const GTFSM_4: u8 = 0b0100;

            /// 0b0101: The gate has been locked by processor with master_index = 4.
            pub const GTFSM_5: u8 = 0b0101;

            /// 0b0110: The gate has been locked by processor with master_index = 5.
            pub const GTFSM_6: u8 = 0b0110;

            /// 0b0111: The gate has been locked by processor with master_index = 6.
            pub const GTFSM_7: u8 = 0b0111;

            /// 0b1000: The gate has been locked by processor with master_index = 7.
            pub const GTFSM_8: u8 = 0b1000;

            /// 0b1001: The gate has been locked by processor with master_index = 8.
            pub const GTFSM_9: u8 = 0b1001;

            /// 0b1010: The gate has been locked by processor with master_index = 9.
            pub const GTFSM_10: u8 = 0b1010;

            /// 0b1011: The gate has been locked by processor with master_index = 10.
            pub const GTFSM_11: u8 = 0b1011;

            /// 0b1100: The gate has been locked by processor with master_index = 11.
            pub const GTFSM_12: u8 = 0b1100;

            /// 0b1101: The gate has been locked by processor with master_index = 12.
            pub const GTFSM_13: u8 = 0b1101;

            /// 0b1110: The gate has been locked by processor with master_index = 13.
            pub const GTFSM_14: u8 = 0b1110;

            /// 0b1111: The gate has been locked by processor with master_index = 14.
            pub const GTFSM_15: u8 = 0b1111;
        }
    }

    /// Read-only bits. They indicate which domain had currently locked the gate.
    pub mod LDOM {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u8 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: The gate is locked by domain 0. (True if the field GTFSM does not equal to 0000.)
            pub const LDOM_0: u8 = 0b00;

            /// 0b01: The gate has been locked by domain 1.
            pub const LDOM_1: u8 = 0b01;
        }
    }
}

/// Gate Register
pub mod GATE1 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE2 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE3 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE4 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE5 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE6 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE7 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE8 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE9 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE10 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE11 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE12 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE13 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE14 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE15 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE16 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE17 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE18 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE19 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE20 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE21 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE22 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE23 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE24 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE25 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE26 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE27 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE28 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE29 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE30 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE31 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE32 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE33 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE34 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE35 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE36 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE37 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE38 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE39 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE40 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE41 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE42 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE43 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE44 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE45 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE46 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE47 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE48 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE49 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE50 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE51 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE52 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE53 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE54 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE55 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE56 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE57 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE58 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE59 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE60 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE61 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE62 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// Gate Register
pub mod GATE63 {
    pub use super::GATE0::GTFSM;
    pub use super::GATE0::LDOM;
}

/// RSTGT_R and RSTGT_W
/// RSTGT_R: Reset Gate Read
/// RSTGT_W: Reset Gate Write
pub mod RSTGT_ {

    /// Reset Gate Bus Master
    pub mod RSTGMS {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reset Gate Finite State Machine
    pub mod RSTGSM {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Idle, waiting for the first data pattern write.
            pub const RSTGSM_0: u16 = 0b00;

            /// 0b01: Waiting for the second data pattern write.
            pub const RSTGSM_1: u16 = 0b01;

            /// 0b10: The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state. The "01" state persists for only one clock cycle. Software will never be able to observe this state.
            pub const RSTGSM_2: u16 = 0b10;

            /// 0b11: This state encoding is never used and therefore reserved.
            pub const RSTGSM_3: u16 = 0b11;
        }
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

    /// Reset Gate Data Pattern
    pub mod RSTGDP {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// Gate Register
    pub GATE0: RWRegister<u8>,

    /// Gate Register
    pub GATE1: RWRegister<u8>,

    /// Gate Register
    pub GATE2: RWRegister<u8>,

    /// Gate Register
    pub GATE3: RWRegister<u8>,

    /// Gate Register
    pub GATE4: RWRegister<u8>,

    /// Gate Register
    pub GATE5: RWRegister<u8>,

    /// Gate Register
    pub GATE6: RWRegister<u8>,

    /// Gate Register
    pub GATE7: RWRegister<u8>,

    /// Gate Register
    pub GATE8: RWRegister<u8>,

    /// Gate Register
    pub GATE9: RWRegister<u8>,

    /// Gate Register
    pub GATE10: RWRegister<u8>,

    /// Gate Register
    pub GATE11: RWRegister<u8>,

    /// Gate Register
    pub GATE12: RWRegister<u8>,

    /// Gate Register
    pub GATE13: RWRegister<u8>,

    /// Gate Register
    pub GATE14: RWRegister<u8>,

    /// Gate Register
    pub GATE15: RWRegister<u8>,

    /// Gate Register
    pub GATE16: RWRegister<u8>,

    /// Gate Register
    pub GATE17: RWRegister<u8>,

    /// Gate Register
    pub GATE18: RWRegister<u8>,

    /// Gate Register
    pub GATE19: RWRegister<u8>,

    /// Gate Register
    pub GATE20: RWRegister<u8>,

    /// Gate Register
    pub GATE21: RWRegister<u8>,

    /// Gate Register
    pub GATE22: RWRegister<u8>,

    /// Gate Register
    pub GATE23: RWRegister<u8>,

    /// Gate Register
    pub GATE24: RWRegister<u8>,

    /// Gate Register
    pub GATE25: RWRegister<u8>,

    /// Gate Register
    pub GATE26: RWRegister<u8>,

    /// Gate Register
    pub GATE27: RWRegister<u8>,

    /// Gate Register
    pub GATE28: RWRegister<u8>,

    /// Gate Register
    pub GATE29: RWRegister<u8>,

    /// Gate Register
    pub GATE30: RWRegister<u8>,

    /// Gate Register
    pub GATE31: RWRegister<u8>,

    /// Gate Register
    pub GATE32: RWRegister<u8>,

    /// Gate Register
    pub GATE33: RWRegister<u8>,

    /// Gate Register
    pub GATE34: RWRegister<u8>,

    /// Gate Register
    pub GATE35: RWRegister<u8>,

    /// Gate Register
    pub GATE36: RWRegister<u8>,

    /// Gate Register
    pub GATE37: RWRegister<u8>,

    /// Gate Register
    pub GATE38: RWRegister<u8>,

    /// Gate Register
    pub GATE39: RWRegister<u8>,

    /// Gate Register
    pub GATE40: RWRegister<u8>,

    /// Gate Register
    pub GATE41: RWRegister<u8>,

    /// Gate Register
    pub GATE42: RWRegister<u8>,

    /// Gate Register
    pub GATE43: RWRegister<u8>,

    /// Gate Register
    pub GATE44: RWRegister<u8>,

    /// Gate Register
    pub GATE45: RWRegister<u8>,

    /// Gate Register
    pub GATE46: RWRegister<u8>,

    /// Gate Register
    pub GATE47: RWRegister<u8>,

    /// Gate Register
    pub GATE48: RWRegister<u8>,

    /// Gate Register
    pub GATE49: RWRegister<u8>,

    /// Gate Register
    pub GATE50: RWRegister<u8>,

    /// Gate Register
    pub GATE51: RWRegister<u8>,

    /// Gate Register
    pub GATE52: RWRegister<u8>,

    /// Gate Register
    pub GATE53: RWRegister<u8>,

    /// Gate Register
    pub GATE54: RWRegister<u8>,

    /// Gate Register
    pub GATE55: RWRegister<u8>,

    /// Gate Register
    pub GATE56: RWRegister<u8>,

    /// Gate Register
    pub GATE57: RWRegister<u8>,

    /// Gate Register
    pub GATE58: RWRegister<u8>,

    /// Gate Register
    pub GATE59: RWRegister<u8>,

    /// Gate Register
    pub GATE60: RWRegister<u8>,

    /// Gate Register
    pub GATE61: RWRegister<u8>,

    /// Gate Register
    pub GATE62: RWRegister<u8>,

    /// Gate Register
    pub GATE63: RWRegister<u8>,

    _reserved1: [u16; 1],

    /// RSTGT_R and RSTGT_W
    /// RSTGT_R: Reset Gate Read
    /// RSTGT_W: Reset Gate Write
    pub RSTGT_: RWRegister<u16>,
}
pub struct ResetValues {
    pub GATE0: u8,
    pub GATE1: u8,
    pub GATE2: u8,
    pub GATE3: u8,
    pub GATE4: u8,
    pub GATE5: u8,
    pub GATE6: u8,
    pub GATE7: u8,
    pub GATE8: u8,
    pub GATE9: u8,
    pub GATE10: u8,
    pub GATE11: u8,
    pub GATE12: u8,
    pub GATE13: u8,
    pub GATE14: u8,
    pub GATE15: u8,
    pub GATE16: u8,
    pub GATE17: u8,
    pub GATE18: u8,
    pub GATE19: u8,
    pub GATE20: u8,
    pub GATE21: u8,
    pub GATE22: u8,
    pub GATE23: u8,
    pub GATE24: u8,
    pub GATE25: u8,
    pub GATE26: u8,
    pub GATE27: u8,
    pub GATE28: u8,
    pub GATE29: u8,
    pub GATE30: u8,
    pub GATE31: u8,
    pub GATE32: u8,
    pub GATE33: u8,
    pub GATE34: u8,
    pub GATE35: u8,
    pub GATE36: u8,
    pub GATE37: u8,
    pub GATE38: u8,
    pub GATE39: u8,
    pub GATE40: u8,
    pub GATE41: u8,
    pub GATE42: u8,
    pub GATE43: u8,
    pub GATE44: u8,
    pub GATE45: u8,
    pub GATE46: u8,
    pub GATE47: u8,
    pub GATE48: u8,
    pub GATE49: u8,
    pub GATE50: u8,
    pub GATE51: u8,
    pub GATE52: u8,
    pub GATE53: u8,
    pub GATE54: u8,
    pub GATE55: u8,
    pub GATE56: u8,
    pub GATE57: u8,
    pub GATE58: u8,
    pub GATE59: u8,
    pub GATE60: u8,
    pub GATE61: u8,
    pub GATE62: u8,
    pub GATE63: u8,
    pub RSTGT_: u16,
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
