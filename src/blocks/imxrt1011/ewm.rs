#[doc = "EWM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CTRL: u8,
    #[doc = "Service Register"]
    pub SERV: u8,
    #[doc = "Compare Low Register"]
    pub CMPL: u8,
    #[doc = "Compare High Register"]
    pub CMPH: u8,
    #[doc = "Clock Control Register"]
    pub CLKCTRL: u8,
    #[doc = "Clock Prescaler Register"]
    pub CLKPRESCALER: u8,
}
#[doc = "Control Register"]
pub mod CTRL {
    pub use crate::RW as access;
    #[doc = "EWM enable."]
    pub mod EWMEN {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EWM_in's Assertion State Select."]
    pub mod ASSIN {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Input Enable."]
    pub mod INEN {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Interrupt Enable."]
    pub mod INTEN {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Service Register"]
pub mod SERV {
    pub use crate::RW as access;
    #[doc = "SERVICE"]
    pub mod SERVICE {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Compare Low Register"]
pub mod CMPL {
    pub use crate::RW as access;
    #[doc = "COMPAREL"]
    pub mod COMPAREL {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Compare High Register"]
pub mod CMPH {
    pub use crate::RW as access;
    #[doc = "COMPAREH"]
    pub mod COMPAREH {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Clock Control Register"]
pub mod CLKCTRL {
    pub use crate::RW as access;
    #[doc = "CLKSEL"]
    pub mod CLKSEL {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Clock Prescaler Register"]
pub mod CLKPRESCALER {
    pub use crate::RW as access;
    #[doc = "CLK_DIV"]
    pub mod CLK_DIV {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
