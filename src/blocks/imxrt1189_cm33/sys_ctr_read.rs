#[doc = "SYS_CTR_READ"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "Counter Count Value Low"]
    pub CNTCV0: u32,
    #[doc = "Counter Count Value High"]
    pub CNTCV1: u32,
    _reserved1: [u8; 0x0fc0],
    #[doc = "Counter ID"]
    pub CNTID0: u32,
}
#[doc = "Counter Count Value Low"]
pub mod CNTCV0 {
    pub use crate::RO as access;
    #[doc = "Counter Count Value Bits \\[31:0\\]"]
    pub mod CNTCV0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Counter Count Value High"]
pub mod CNTCV1 {
    pub use crate::RO as access;
    #[doc = "Counter Count Value Bits \\[55:32\\]"]
    pub mod CNTCV1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Counter ID"]
pub mod CNTID0 {
    pub use crate::RO as access;
    #[doc = "Counter Identification"]
    pub mod CNTID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
