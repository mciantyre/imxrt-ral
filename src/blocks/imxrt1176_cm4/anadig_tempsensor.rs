#[doc = "MX6RT_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "Tempsensor Register"]
    pub TEMPSENSOR: u32,
    _reserved1: [u8; 0x2c],
    #[doc = "TEMPSNS_OTP_TRIM_VALUE_REGISTER"]
    pub TEMPSNS_OTP_TRIM_VALUE: u32,
}
#[doc = "Tempsensor Register"]
pub mod TEMPSENSOR {
    pub use crate::RW as access;
    #[doc = "AI toggle"]
    pub mod TEMPSNS_AI_TOGGLE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AI Busy monitor"]
    pub mod TEMPSNS_AI_BUSY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "TEMPSNS_OTP_TRIM_VALUE_REGISTER"]
pub mod TEMPSNS_OTP_TRIM_VALUE {
    pub use crate::RO as access;
    #[doc = "Temperature Value at 25C"]
    pub mod TEMPSNS_TEMP_VAL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
