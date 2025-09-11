#[doc = "IOMUXC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPR0 General Purpose Register"]
    pub GPR0: u32,
    #[doc = "GPR1 General Purpose Register"]
    pub GPR1: u32,
    #[doc = "GPR2 General Purpose Register"]
    pub GPR2: u32,
    #[doc = "GPR3 General Purpose Register"]
    pub GPR3: u32,
}
#[doc = "GPR0 General Purpose Register"]
pub mod GPR0 {
    pub use crate::RO as access;
}
#[doc = "GPR1 General Purpose Register"]
pub mod GPR1 {
    pub use crate::RO as access;
}
#[doc = "GPR2 General Purpose Register"]
pub mod GPR2 {
    pub use crate::RO as access;
}
#[doc = "GPR3 General Purpose Register"]
pub mod GPR3 {
    pub use crate::RW as access;
    #[doc = "Set to enable LPSR mode."]
    pub mod LPSR_MODE_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DCDC captured status clear"]
    pub mod DCDC_STATUS_CAPT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "POR_B pad control"]
    pub mod POR_PULL_TYPE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DCDC_IN low voltage detect."]
    pub mod DCDC_IN_LOW_VOL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DCDC output over current alert"]
    pub mod DCDC_OVER_CUR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DCDC output over voltage alert"]
    pub mod DCDC_OVER_VOL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DCDC status OK"]
    pub mod DCDC_STS_DC_OK {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
