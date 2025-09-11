#[doc = "PIT"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "PIT Module Control Register"]
    pub MCR: u32,
    _reserved0: [u8; 0xdc],
    #[doc = "PIT Upper Lifetime Timer Register"]
    pub LTMR64H: u32,
    #[doc = "PIT Lower Lifetime Timer Register"]
    pub LTMR64L: u32,
    _reserved1: [u8; 0x18],
    #[doc = "no description available"]
    pub TIMER: [TIMER::RegisterBlock; 4usize],
}
#[doc = "PIT Module Control Register"]
pub mod MCR {
    pub use crate::RW as access;
    #[doc = "Freeze"]
    pub mod FRZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Timers continue to run in Debug mode."]
            pub const FRZ_0: u32 = 0;
            #[doc = "Timers are stopped in Debug mode."]
            pub const FRZ_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Module Disable for PIT"]
    pub mod MDIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Clock for standard PIT timers is enabled."]
            pub const MDIS_0: u32 = 0;
            #[doc = "Clock for standard PIT timers is disabled."]
            pub const MDIS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PIT Upper Lifetime Timer Register"]
pub mod LTMR64H {
    pub use crate::RO as access;
    #[doc = "Life Timer value"]
    pub mod LTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PIT Lower Lifetime Timer Register"]
pub mod LTMR64L {
    pub use crate::RO as access;
    #[doc = "Life Timer value"]
    pub mod LTL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
pub mod TIMER {
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Timer Load Value Register"]
        pub LDVAL: u32,
        #[doc = "Current Timer Value Register"]
        pub CVAL: u32,
        #[doc = "Timer Control Register"]
        pub TCTRL: u32,
        #[doc = "Timer Flag Register"]
        pub TFLG: u32,
    }
    #[doc = "Timer Load Value Register"]
    pub mod LDVAL {
        pub use crate::RW as access;
        #[doc = "Timer Start Value"]
        pub mod TSV {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Current Timer Value Register"]
    pub mod CVAL {
        pub use crate::RO as access;
        #[doc = "Current Timer Value"]
        pub mod TVL {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Timer Control Register"]
    pub mod TCTRL {
        pub use crate::RW as access;
        #[doc = "Timer Enable"]
        pub mod TEN {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Timer n is disabled."]
                pub const TEN_0: u32 = 0;
                #[doc = "Timer n is enabled."]
                pub const TEN_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Timer Interrupt Enable"]
        pub mod TIE {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Interrupt requests from Timer n are disabled."]
                pub const TIE_0: u32 = 0;
                #[doc = "Interrupt is requested whenever TIF is set."]
                pub const TIE_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Chain Mode"]
        pub mod CHN {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Timer is not chained."]
                pub const CHN_0: u32 = 0;
                #[doc = "Timer is chained to a previous timer. For example, for channel 2, if this field is set, Timer 2 is chained to Timer 1."]
                pub const CHN_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Timer Flag Register"]
    pub mod TFLG {
        pub use crate::RW as access;
        #[doc = "Timer Interrupt Flag"]
        pub mod TIF {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Timeout has not yet occurred."]
                pub const TIF_0: u32 = 0;
                #[doc = "Timeout has occurred."]
                pub const TIF_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
}
