#[doc = "MX6RT_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0540],
    #[doc = "PMU_LDO_SNVS_DIG_REGISTER"]
    pub PMU_LDO_SNVS_DIG: u32,
}
#[doc = "PMU_LDO_SNVS_DIG_REGISTER"]
pub mod PMU_LDO_SNVS_DIG {
    pub use crate::RW as access;
    #[doc = "REG_LP_EN"]
    pub mod REG_LP_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "test_override"]
    pub mod TEST_OVERRIDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "REG_EN"]
    pub mod REG_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
