#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! no description available
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Lane Configuration Register
pub mod CFG_NUM_LANES {

    /// This field is used to set the number of active lanes for receiving data.
    pub mod CFG_NUM_LANES {
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

            /// 0b00: 1 Lane
            pub const CFG_NUM_LANES_0: u32 = 0b00;

            /// 0b01: 2 Lane
            pub const CFG_NUM_LANES_1: u32 = 0b01;
        }
    }
}

/// Disable Data Lane Register
pub mod CFG_DISABLE_DATA_LANES {

    /// This field is used to disable data lanes.
    pub mod CFG_DISABLE_DATA_LANES {
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
}

/// ECC and CRC Error Status Register
pub mod BIT_ERR {

    /// This field shows the error status of ECC and CRC
    pub mod BIT_ERR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IRQ Status Register
pub mod IRQ_STATUS {

    /// This field shows the IRQ status
    pub mod IRQ_STATUS {
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

/// IRQ Mask Setting Register
pub mod IRQ_MASK {

    /// This field shows the IRQ Mask setting
    pub mod IRQ_MASK {
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

/// Ultra Low Power State (ULPS) Status Register
pub mod ULPS_STATUS {

    /// This field shows the status of Rx D-PHY ULPS state
    pub mod STATUS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ERRSot HS Status Register
pub mod PPI_ERRSOT_HS {

    /// This field indicates PPI ErrSotHS captured status from D-PHY
    pub mod STATUS {
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
}

/// ErrSotSync HS Status Register
pub mod PPI_ERRSOTSYNC_HS {
    pub use super::PPI_ERRSOT_HS::STATUS;
}

/// ErrEsc Status Register
pub mod PPI_ERRESC {
    pub use super::PPI_ERRSOT_HS::STATUS;
}

/// ErrSyncEsc Status Register
pub mod PPI_ERRSYNCESC {
    pub use super::PPI_ERRSOT_HS::STATUS;
}

/// ErrControl Status Register
pub mod PPI_ERRCONTROL {
    pub use super::PPI_ERRSOT_HS::STATUS;
}

/// Disable Payload 0 Register
pub mod CFG_DISABLE_PAYLOAD_0 {

    /// Null
    pub mod DIS_PAYLOAD_NULL {
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

    /// Blank
    pub mod DIS_PAYLOAD_BLANK {
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

    /// Embedded
    pub mod DIS_PAYLOAD_EMBEDDED {
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

    /// Legacy YUV 420 8 bit
    pub mod DIS_PAYLOAD_YUV420 {
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

    /// YUV422 8 bit
    pub mod DIS_PAYLOAD_YUV422_8BIT {
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

    /// RGB444
    pub mod DIS_PAYLOAD_RGB444 {
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

    /// RGB555
    pub mod DIS_PAYLOAD_RGB555 {
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

    /// RGB565
    pub mod DIS_PAYLOAD_RGB565 {
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

    /// RGB666
    pub mod DIS_PAYLOAD_RGB666 {
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

    /// RGB888
    pub mod DIS_PAYLOAD_RGB888 {
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
}

/// Disable Payload 1 Register
pub mod CFG_DISABLE_PAYLOAD_1 {

    /// User defined type 0x31
    pub mod DIS_PAYLOAD_UDEF_30 {
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

    /// User defined type 0x32
    pub mod DIS_PAYLOAD_UDEF_31 {
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

    /// User defined type 0x33
    pub mod DIS_PAYLOAD_UDEF_32 {
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

    /// User defined type 0x34
    pub mod DIS_PAYLOAD_UDEF_33 {
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

    /// User defined type 0x35
    pub mod DIS_PAYLOAD_UDEF_34 {
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

    /// User defined type 0x35
    pub mod DIS_PAYLOAD_UDEF_35 {
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

    /// User defined type 0x36
    pub mod DIS_PAYLOAD_UDEF_36 {
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

    /// User defined type 0x37
    pub mod DIS_PAYLOAD_UDEF_37 {
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

    /// Unsupported Data Types
    pub mod DIS_PAYLOAD_UNSUPPORTED {
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
}

/// Ignore Virtual Channel Register
pub mod CFG_IGNORE_VC {

    /// When set, this input causes the interface to ignore the Virtual Channel (VC) field in received packets
    pub mod IGNORE_VC {
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
}

/// Virtual Channel value Register
pub mod CFG_VID_VC {

    /// This bit field sets the Virtual Channel value the interface must match in an incoming packet for it to accept the packet
    pub mod VID_VC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// FIFO Send Level Configuration Register
pub mod CFG_VID_P_FIFO_SEND_LEVEL {

    /// FIFO Send Level field
    pub mod SEND_LEVEL {
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
}

/// VSYNC Configuration Register
pub mod CFG_VID_VSYNC {

    /// Width of VSYNC
    pub mod WIDTH {
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

/// Start of HSYNC Delay control Register
pub mod CFG_VID_HSYNC_FP {

    /// Delay control for beginning of HSYNC pulse
    pub mod DELAY_CTL {
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

/// HSYNC Configuration Register
pub mod CFG_VID_HSYNC {
    pub use super::CFG_VID_VSYNC::WIDTH;
}

/// End of HSYNC Delay Control Register
pub mod CFG_VID_HSYNC_BP {
    pub use super::CFG_VID_HSYNC_FP::DELAY_CTL;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 64],

    /// Lane Configuration Register
    pub CFG_NUM_LANES: RWRegister<u32>,

    /// Disable Data Lane Register
    pub CFG_DISABLE_DATA_LANES: RWRegister<u32>,

    /// ECC and CRC Error Status Register
    pub BIT_ERR: RORegister<u32>,

    /// IRQ Status Register
    pub IRQ_STATUS: RORegister<u32>,

    /// IRQ Mask Setting Register
    pub IRQ_MASK: RWRegister<u32>,

    /// Ultra Low Power State (ULPS) Status Register
    pub ULPS_STATUS: RORegister<u32>,

    /// ERRSot HS Status Register
    pub PPI_ERRSOT_HS: RORegister<u32>,

    /// ErrSotSync HS Status Register
    pub PPI_ERRSOTSYNC_HS: RORegister<u32>,

    /// ErrEsc Status Register
    pub PPI_ERRESC: RORegister<u32>,

    /// ErrSyncEsc Status Register
    pub PPI_ERRSYNCESC: RORegister<u32>,

    /// ErrControl Status Register
    pub PPI_ERRCONTROL: RORegister<u32>,

    /// Disable Payload 0 Register
    pub CFG_DISABLE_PAYLOAD_0: RWRegister<u32>,

    /// Disable Payload 1 Register
    pub CFG_DISABLE_PAYLOAD_1: RWRegister<u32>,

    _reserved2: [u32; 19],

    /// Ignore Virtual Channel Register
    pub CFG_IGNORE_VC: RWRegister<u32>,

    /// Virtual Channel value Register
    pub CFG_VID_VC: RWRegister<u32>,

    /// FIFO Send Level Configuration Register
    pub CFG_VID_P_FIFO_SEND_LEVEL: RWRegister<u32>,

    /// VSYNC Configuration Register
    pub CFG_VID_VSYNC: RWRegister<u32>,

    /// Start of HSYNC Delay control Register
    pub CFG_VID_HSYNC_FP: RWRegister<u32>,

    /// HSYNC Configuration Register
    pub CFG_VID_HSYNC: RWRegister<u32>,

    /// End of HSYNC Delay Control Register
    pub CFG_VID_HSYNC_BP: RWRegister<u32>,
}
pub struct ResetValues {
    pub CFG_NUM_LANES: u32,
    pub CFG_DISABLE_DATA_LANES: u32,
    pub BIT_ERR: u32,
    pub IRQ_STATUS: u32,
    pub IRQ_MASK: u32,
    pub ULPS_STATUS: u32,
    pub PPI_ERRSOT_HS: u32,
    pub PPI_ERRSOTSYNC_HS: u32,
    pub PPI_ERRESC: u32,
    pub PPI_ERRSYNCESC: u32,
    pub PPI_ERRCONTROL: u32,
    pub CFG_DISABLE_PAYLOAD_0: u32,
    pub CFG_DISABLE_PAYLOAD_1: u32,
    pub CFG_IGNORE_VC: u32,
    pub CFG_VID_VC: u32,
    pub CFG_VID_P_FIFO_SEND_LEVEL: u32,
    pub CFG_VID_VSYNC: u32,
    pub CFG_VID_HSYNC_FP: u32,
    pub CFG_VID_HSYNC: u32,
    pub CFG_VID_HSYNC_BP: u32,
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
