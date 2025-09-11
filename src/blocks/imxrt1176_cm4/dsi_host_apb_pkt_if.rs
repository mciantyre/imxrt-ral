#[doc = "DSI HOST APB PKT Interface"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "TX_PAYLOAD"]
    pub TX_PAYLOAD: u32,
    #[doc = "PKT_CONTROL"]
    pub PKT_CONTROL: u32,
    #[doc = "SEND_PACKET"]
    pub SEND_PACKET: u32,
    #[doc = "PKT_STATUS"]
    pub PKT_STATUS: u32,
    #[doc = "PKT_FIFO_WR_LEVEL"]
    pub PKT_FIFO_WR_LEVEL: u32,
    #[doc = "PKT_FIFO_RD_LEVEL"]
    pub PKT_FIFO_RD_LEVEL: u32,
    #[doc = "PKT_RX_PAYLOAD"]
    pub PKT_RX_PAYLOAD: u32,
    #[doc = "PKT_RX_PKT_HEADER"]
    pub PKT_RX_PKT_HEADER: u32,
    #[doc = "IRQ_STATUS"]
    pub IRQ_STATUS: u32,
    #[doc = "IRQ_STATUS2"]
    pub IRQ_STATUS2: u32,
    #[doc = "IRQ_MASK"]
    pub IRQ_MASK: u32,
    #[doc = "IRQ_MASK2"]
    pub IRQ_MASK2: u32,
}
#[doc = "TX_PAYLOAD"]
pub mod TX_PAYLOAD {
    pub use crate::RW as access;
    #[doc = "Tx Payload data write register. Write to this register loads the payload FIFO with 32 bit values."]
    pub mod PAYLOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PKT_CONTROL"]
pub mod PKT_CONTROL {
    pub use crate::RW as access;
    #[doc = "Tx packet control"]
    pub mod CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SEND_PACKET"]
pub mod SEND_PACKET {
    pub use crate::RW as access;
    #[doc = "Tx send packet, writing to this register causes the packet described in dsi_host_pkt_control to be sent."]
    pub mod TX_SEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Packet not sent"]
            pub const TX_SEND_0: u32 = 0;
            #[doc = "Packet is sent"]
            pub const TX_SEND_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PKT_STATUS"]
pub mod PKT_STATUS {
    pub use crate::RO as access;
    #[doc = "Status of APB to packet interface."]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PKT_FIFO_WR_LEVEL"]
pub mod PKT_FIFO_WR_LEVEL {
    pub use crate::RO as access;
    #[doc = "Write level of APB to pkt interface FIFO"]
    pub mod WR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PKT_FIFO_RD_LEVEL"]
pub mod PKT_FIFO_RD_LEVEL {
    pub use crate::RO as access;
    #[doc = "Read level of APB to pkt interface FIFO"]
    pub mod RD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PKT_RX_PAYLOAD"]
pub mod PKT_RX_PAYLOAD {
    pub use crate::RO as access;
    #[doc = "APB to pkt interface Rx payload read"]
    pub mod PAYLOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PKT_RX_PKT_HEADER"]
pub mod PKT_RX_PKT_HEADER {
    pub use crate::RO as access;
    #[doc = "APB to pkt interface Rx packet header"]
    pub mod HEADER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ_STATUS"]
pub mod IRQ_STATUS {
    pub use crate::RO as access;
    #[doc = "Status of APB to packet interface."]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ_STATUS2"]
pub mod IRQ_STATUS2 {
    pub use crate::RO as access;
    #[doc = "Status of APB to packet interface part 2, read part 2 first then dsi_host_irq_status. Reading dsi_host_irq_status will clear both status and status2."]
    pub mod STATUS2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ_MASK"]
pub mod IRQ_MASK {
    pub use crate::RW as access;
    #[doc = "IRQ Mask"]
    pub mod MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ_MASK2"]
pub mod IRQ_MASK2 {
    pub use crate::RW as access;
    #[doc = "IRQ mask 2"]
    pub mod MASK2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
