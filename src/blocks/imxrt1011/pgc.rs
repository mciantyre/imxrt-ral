#[doc = "PGC"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0220],
    #[doc = "PGC Mega Control Register"]
    pub MEGA_CTRL: u32,
    #[doc = "PGC Mega Power Up Sequence Control Register"]
    pub MEGA_PUPSCR: u32,
    #[doc = "PGC Mega Pull Down Sequence Control Register"]
    pub MEGA_PDNSCR: u32,
    #[doc = "PGC Mega Power Gating Controller Status Register"]
    pub MEGA_SR: u32,
    _reserved1: [u8; 0x70],
    #[doc = "PGC CPU Control Register"]
    pub CPU_CTRL: u32,
    #[doc = "PGC CPU Power Up Sequence Control Register"]
    pub CPU_PUPSCR: u32,
    #[doc = "PGC CPU Pull Down Sequence Control Register"]
    pub CPU_PDNSCR: u32,
    #[doc = "PGC CPU Power Gating Controller Status Register"]
    pub CPU_SR: u32,
}
#[doc = "PGC Mega Control Register"]
pub mod MEGA_CTRL {
    pub use crate::RW as access;
    #[doc = "Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    pub mod PCR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not switch off power even if pdn_req is asserted."]
            pub const PCR_0: u32 = 0;
            #[doc = "Switch off power when pdn_req is asserted."]
            pub const PCR_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PGC Mega Power Up Sequence Control Register"]
pub mod MEGA_PUPSCR {
    pub use crate::RW as access;
    #[doc = "After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b)"]
    pub mod SW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "After asserting power toggle on/off signal (switch_b), the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    pub mod SW2ISO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PGC Mega Pull Down Sequence Control Register"]
pub mod MEGA_PDNSCR {
    pub use crate::RW as access;
    #[doc = "After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    pub mod ISO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b)"]
    pub mod ISO2SW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PGC Mega Power Gating Controller Status Register"]
pub mod MEGA_SR {
    pub use crate::RW as access;
    #[doc = "Power status"]
    pub mod PSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The target subsystem was not powered down for the previous power-down request."]
            pub const PSR_0: u32 = 0;
            #[doc = "The target subsystem was powered down for the previous power-down request."]
            pub const PSR_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PGC CPU Control Register"]
pub mod CPU_CTRL {
    pub use crate::RW as access;
    #[doc = "Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    pub mod PCR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Do not switch off power even if pdn_req is asserted."]
            pub const PCR_0: u32 = 0;
            #[doc = "Switch off power when pdn_req is asserted."]
            pub const PCR_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PGC CPU Power Up Sequence Control Register"]
pub mod CPU_PUPSCR {
    pub use crate::RW as access;
    #[doc = "There are two different silicon revisions: 1"]
    pub mod SW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "There are two different silicon revisions: 1"]
    pub mod SW2ISO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PGC CPU Pull Down Sequence Control Register"]
pub mod CPU_PDNSCR {
    pub use crate::RW as access;
    #[doc = "After a power-down request (pdn_req assertion), the PGC waits a number of 32k clocks equal to the value of ISO before asserting isolation"]
    pub mod ISO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "After asserting isolation, the PGC waits a number of 32k clocks equal to the value of ISO2SW before negating"]
    pub mod ISO2SW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PGC CPU Power Gating Controller Status Register"]
pub mod CPU_SR {
    pub use crate::RW as access;
    #[doc = "Power status"]
    pub mod PSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The target subsystem was not powered down for the previous power-down request."]
            pub const PSR_0: u32 = 0;
            #[doc = "The target subsystem was powered down for the previous power-down request."]
            pub const PSR_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
