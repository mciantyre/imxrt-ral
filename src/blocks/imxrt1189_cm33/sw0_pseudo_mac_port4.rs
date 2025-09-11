#[doc = "Pseudo MAC port"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Port pseudo MAC status register"]
    pub PPMSR: u32,
    _reserved0: [u8; 0x7c],
    #[doc = "Port pseudo MAC receive octets counter"]
    pub PPMROCR: [u32; 2usize],
    #[doc = "Port pseudo MAC receive unicast frame counter register"]
    pub PPMRUFCR: [u32; 2usize],
    #[doc = "Port pseudo MAC receive multicast frame counter register"]
    pub PPMRMFCR: [u32; 2usize],
    #[doc = "Port pseudo MAC receive broadcast frame counter register"]
    pub PPMRBFCR: [u32; 2usize],
    _reserved1: [u8; 0x20],
    #[doc = "Port pseudo MAC transmit octets counter"]
    pub PPMTOCR: [u32; 2usize],
    #[doc = "Port pseudo MAC transmit unicast frame counter register"]
    pub PPMTUFCR: [u32; 2usize],
    #[doc = "Port pseudo MAC transmit multicast frame counter register"]
    pub PPMTMFCR: [u32; 2usize],
    #[doc = "Port pseudo MAC transmit broadcast frame counter register"]
    pub PPMTBFCR: [u32; 2usize],
}
#[doc = "Port pseudo MAC status register"]
pub mod PPMSR {
    pub use crate::RO as access;
    #[doc = "Local link end's state 0 - Link is down 1 - Link is up The operational state is always \"Link is up\" for the pseudo link"]
    pub mod LSTATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Remote link end's state 0 - Link is down 1 - Link is up The operational state is always \"Link is up\" for the pseudo link"]
    pub mod RSTATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port pseudo MAC receive octets counter"]
pub mod PPMROCR {
    pub use crate::RO as access;
    #[doc = "Incremented for each octet received (on the link) (that is, Ethernet header, payload, pad and FCS)."]
    pub mod ROCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port pseudo MAC receive unicast frame counter register"]
pub mod PPMRUFCR {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame received (on the link) in which bit 0 of the destination address was 0"]
    pub mod RUCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port pseudo MAC receive multicast frame counter register"]
pub mod PPMRMFCR {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame received (on the link) in which bit 0 of the destination address was 1 but not the broadcast address (all bits set to 1)"]
    pub mod RMCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port pseudo MAC receive broadcast frame counter register"]
pub mod PPMRBFCR {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame received (on the link) in which all bits of the destination address were 1"]
    pub mod RBCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port pseudo MAC transmit octets counter"]
pub mod PPMTOCR {
    pub use crate::RO as access;
    #[doc = "Incremented for each octet transmitted (on the link) (that is, Ethernet header, payload, pad and FCS)"]
    pub mod TOCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port pseudo MAC transmit unicast frame counter register"]
pub mod PPMTUFCR {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame transmitted (on the link) in which bit 0 of the destination address was 0"]
    pub mod TUCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port pseudo MAC transmit multicast frame counter register"]
pub mod PPMTMFCR {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame transmitted (on the link) in which bit 0 of the destination address was 1 but not the broadcast address (all bits set to 1)"]
    pub mod TMCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port pseudo MAC transmit broadcast frame counter register"]
pub mod PPMTBFCR {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame transmitted (on the link) in which all bits of the destination address were 1"]
    pub mod TBCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
