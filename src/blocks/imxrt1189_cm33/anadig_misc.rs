#[doc = "RT1180_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4800],
    #[doc = "Chip Silicon Version Register"]
    pub MISC_DIFPROG: u32,
}
#[doc = "Chip Silicon Version Register"]
pub mod MISC_DIFPROG {
    pub use crate::RO as access;
    #[doc = "Chip ID"]
    pub mod CHIPID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
