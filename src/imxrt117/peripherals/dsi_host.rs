#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DSI HOST
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// CFG_NUM_LANES
pub mod CFG_NUM_LANES {

    /// Sets the number of active lanes that are to be used for transmitting data.
    pub mod NUM_LANES {
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

            /// 0b00: 1 lane
            pub const NUM_LANES_0: u32 = 0b00;

            /// 0b01: 2 lanes
            pub const NUM_LANES_1: u32 = 0b01;
        }
    }
}

/// CFG_NONCONTINUOUS_CLK
pub mod CFG_NONCONTINUOUS_CLK {

    /// Sets the Host Controller into non-continuous MIPI clock mode. When in non-continuous clock mode, the high speed clock will transition into low power mode between transmissions.
    pub mod CLK_MODE {
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

            /// 0b0: Continuous high speed clock
            pub const CLK_MODE_0: u32 = 0b0;

            /// 0b1: Non-Continuous high speed clock
            pub const CLK_MODE_1: u32 = 0b1;
        }
    }
}

/// CFG_T_PRE
pub mod CFG_T_PRE {

    /// Sets the number of byte clock periods ('clk_byte' input) that the controller will wait after enabling the clock lane for HS operation before enabling the data lanes for HS operation. This setting represents the TCLK-PRE DPHY timing parameter. The minimum value for this port is 1.
    pub mod NUM_PERIODS {
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
}

/// CFG_T_POST
pub mod CFG_T_POST {
    pub use super::CFG_T_PRE::NUM_PERIODS;
}

/// CFG_TX_GAP
pub mod CFG_TX_GAP {
    pub use super::CFG_T_PRE::NUM_PERIODS;
}

/// CFG_AUTOINSERT_ETOP
pub mod CFG_AUTOINSERT_EOTP {

    /// Enables the Host Controller to automatically insert an EoTp short packet when switching from HS to LP mode.
    pub mod AUTOINSERT {
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

            /// 0b0: EoTp is not automatically inserted
            pub const NOT_AUTO: u32 = 0b0;

            /// 0b1: EoTp is automatically inserted
            pub const AUTO: u32 = 0b1;
        }
    }
}

/// CFG_EXTRA_CMDS_AFTER_ETOP
pub mod CFG_EXTRA_CMDS_AFTER_EOTP {

    /// Configures the DSI Host Controller to send extra End Of Transmission Packets after the end of a packet. The value is the number of extra EOTP packets sent.
    pub mod EXTRA_EOTP {
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
}

/// CFG_HTX_TO_COUNT
pub mod CFG_HTX_TO_COUNT {

    /// Sets the value of the DSI Host High Speed TX timeout count in clk_byte clock periods that once reached will initiate a timeout error and follow the recovery procedure documented in the DSI specification.
    pub mod COUNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CFG_LRX_H_TO_COUNT
pub mod CFG_LRX_H_TO_COUNT {
    pub use super::CFG_HTX_TO_COUNT::COUNT;
}

/// CFG_BTA_H_TO_COUNT
pub mod CFG_BTA_H_TO_COUNT {
    pub use super::CFG_HTX_TO_COUNT::COUNT;
}

/// CFG_TWAKEUP
pub mod CFG_TWAKEUP {

    /// DPHY Twakeup timing parameter. Sets the number of clk_esc clock periods to keep a clock or data lane in Mark-1 state after exiting ULPS. The MIPI DPHY spec requires a minimum of 1ms in Mark-1 state after leaving ULPS.
    pub mod NUM_PERIODS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (19 bits: 0x7ffff << 0)
        pub const mask: u32 = 0x7ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CFG_STATUS_OUT
pub mod CFG_STATUS_OUT {

    /// Status Register
    pub mod STATUS {
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

/// RX_ERROR_STATUS
pub mod RX_ERROR_STATUS {

    /// Status Register for Host receive error detection, ECC errors, CRC errors and for timeout indicators
    pub mod STATUS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
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
    /// CFG_NUM_LANES
    pub CFG_NUM_LANES: RWRegister<u32>,

    /// CFG_NONCONTINUOUS_CLK
    pub CFG_NONCONTINUOUS_CLK: RWRegister<u32>,

    /// CFG_T_PRE
    pub CFG_T_PRE: RWRegister<u32>,

    /// CFG_T_POST
    pub CFG_T_POST: RWRegister<u32>,

    /// CFG_TX_GAP
    pub CFG_TX_GAP: RWRegister<u32>,

    /// CFG_AUTOINSERT_ETOP
    pub CFG_AUTOINSERT_EOTP: RWRegister<u32>,

    /// CFG_EXTRA_CMDS_AFTER_ETOP
    pub CFG_EXTRA_CMDS_AFTER_EOTP: RWRegister<u32>,

    /// CFG_HTX_TO_COUNT
    pub CFG_HTX_TO_COUNT: RWRegister<u32>,

    /// CFG_LRX_H_TO_COUNT
    pub CFG_LRX_H_TO_COUNT: RWRegister<u32>,

    /// CFG_BTA_H_TO_COUNT
    pub CFG_BTA_H_TO_COUNT: RWRegister<u32>,

    /// CFG_TWAKEUP
    pub CFG_TWAKEUP: RWRegister<u32>,

    /// CFG_STATUS_OUT
    pub CFG_STATUS_OUT: RORegister<u32>,

    /// RX_ERROR_STATUS
    pub RX_ERROR_STATUS: RORegister<u32>,
}
pub struct ResetValues {
    pub CFG_NUM_LANES: u32,
    pub CFG_NONCONTINUOUS_CLK: u32,
    pub CFG_T_PRE: u32,
    pub CFG_T_POST: u32,
    pub CFG_TX_GAP: u32,
    pub CFG_AUTOINSERT_EOTP: u32,
    pub CFG_EXTRA_CMDS_AFTER_EOTP: u32,
    pub CFG_HTX_TO_COUNT: u32,
    pub CFG_LRX_H_TO_COUNT: u32,
    pub CFG_BTA_H_TO_COUNT: u32,
    pub CFG_TWAKEUP: u32,
    pub CFG_STATUS_OUT: u32,
    pub RX_ERROR_STATUS: u32,
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
