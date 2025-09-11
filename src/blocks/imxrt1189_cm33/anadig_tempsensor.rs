#[doc = "RT1180_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4530],
    #[doc = "TEMPSNS_OTP_TRIM_VALUE_REGISTER"]
    pub TEMPSNS_OTP_TRIM_VALUE: u32,
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
