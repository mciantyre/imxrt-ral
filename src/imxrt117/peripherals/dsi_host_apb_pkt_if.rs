#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DSI HOST APB PKT Interface
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// TX_PAYLOAD
pub mod TX_PAYLOAD {

    /// Tx Payload data write register. Write to this register loads the payload FIFO with 32 bit values.
    pub mod PAYLOAD {
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

/// PKT_CONTROL
pub mod PKT_CONTROL {

    /// Tx packet control
    pub mod CTRL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (27 bits: 0x7ffffff << 0)
        pub const mask: u32 = 0x7ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SEND_PACKET
pub mod SEND_PACKET {

    /// Tx send packet, writing to this register causes the packet described in dsi_host_pkt_control to be sent.
    pub mod TX_SEND {
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

            /// 0b0: Packet not sent
            pub const TX_SEND_0: u32 = 0b0;

            /// 0b1: Packet is sent
            pub const TX_SEND_1: u32 = 0b1;
        }
    }
}

/// PKT_STATUS
pub mod PKT_STATUS {

    /// Status of APB to packet interface.
    pub mod STATUS {
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

/// PKT_FIFO_WR_LEVEL
pub mod PKT_FIFO_WR_LEVEL {

    /// Write level of APB to pkt interface FIFO
    pub mod WR {
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

/// PKT_FIFO_RD_LEVEL
pub mod PKT_FIFO_RD_LEVEL {

    /// Read level of APB to pkt interface FIFO
    pub mod RD {
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

/// PKT_RX_PAYLOAD
pub mod PKT_RX_PAYLOAD {

    /// APB to pkt interface Rx payload read
    pub mod PAYLOAD {
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

/// PKT_RX_PKT_HEADER
pub mod PKT_RX_PKT_HEADER {

    /// APB to pkt interface Rx packet header
    pub mod HEADER {
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

/// IRQ_STATUS
pub mod IRQ_STATUS {

    /// Status of APB to packet interface.
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

/// IRQ_STATUS2
pub mod IRQ_STATUS2 {

    /// Status of APB to packet interface part 2, read part 2 first then dsi_host_irq_status. Reading dsi_host_irq_status will clear both status and status2.
    pub mod STATUS2 {
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
}

/// IRQ_MASK
pub mod IRQ_MASK {

    /// IRQ Mask
    pub mod MASK {
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

/// IRQ_MASK2
pub mod IRQ_MASK2 {

    /// IRQ mask 2
    pub mod MASK2 {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// TX_PAYLOAD
    pub TX_PAYLOAD: RWRegister<u32>,

    /// PKT_CONTROL
    pub PKT_CONTROL: RWRegister<u32>,

    /// SEND_PACKET
    pub SEND_PACKET: RWRegister<u32>,

    /// PKT_STATUS
    pub PKT_STATUS: RORegister<u32>,

    /// PKT_FIFO_WR_LEVEL
    pub PKT_FIFO_WR_LEVEL: RORegister<u32>,

    /// PKT_FIFO_RD_LEVEL
    pub PKT_FIFO_RD_LEVEL: RORegister<u32>,

    /// PKT_RX_PAYLOAD
    pub PKT_RX_PAYLOAD: RORegister<u32>,

    /// PKT_RX_PKT_HEADER
    pub PKT_RX_PKT_HEADER: RORegister<u32>,

    /// IRQ_STATUS
    pub IRQ_STATUS: RORegister<u32>,

    /// IRQ_STATUS2
    pub IRQ_STATUS2: RORegister<u32>,

    /// IRQ_MASK
    pub IRQ_MASK: RWRegister<u32>,

    /// IRQ_MASK2
    pub IRQ_MASK2: RWRegister<u32>,
}
pub struct ResetValues {
    pub TX_PAYLOAD: u32,
    pub PKT_CONTROL: u32,
    pub SEND_PACKET: u32,
    pub PKT_STATUS: u32,
    pub PKT_FIFO_WR_LEVEL: u32,
    pub PKT_FIFO_RD_LEVEL: u32,
    pub PKT_RX_PAYLOAD: u32,
    pub PKT_RX_PKT_HEADER: u32,
    pub IRQ_STATUS: u32,
    pub IRQ_STATUS2: u32,
    pub IRQ_MASK: u32,
    pub IRQ_MASK2: u32,
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
