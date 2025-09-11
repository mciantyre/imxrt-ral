#[doc = "FlexCAN wrapper"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x09e0],
    #[doc = "Glitch Filter Width Register"]
    pub GFWR: u32,
}
#[doc = "Glitch Filter Width Register"]
pub mod GFWR {
    pub use crate::RW as access;
    #[doc = "Glitch Filter Width"]
    pub mod GFWR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
