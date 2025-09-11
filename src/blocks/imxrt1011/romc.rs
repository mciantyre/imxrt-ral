#[doc = "ROMC"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xd4],
    #[doc = "ROMC Data Registers"]
    pub ROMPATCHD: [u32; 8usize],
    #[doc = "ROMC Control Register"]
    pub ROMPATCHCNTL: u32,
    #[doc = "ROMC Enable Register High"]
    pub ROMPATCHENH: u32,
    #[doc = "ROMC Enable Register Low"]
    pub ROMPATCHENL: u32,
    #[doc = "ROMC Address Registers"]
    pub ROMPATCHA: [u32; 16usize],
    _reserved1: [u8; 0xc8],
    #[doc = "ROMC Status Register"]
    pub ROMPATCHSR: u32,
}
#[doc = "ROMC Data Registers"]
pub mod ROMPATCHD {
    pub use crate::RW as access;
    #[doc = "Data Fix Registers - Stores the data used for 1-word data fix operations"]
    pub mod DATAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ROMC Control Register"]
pub mod ROMPATCHCNTL {
    pub use crate::RW as access;
    #[doc = "Data Fix Enable - Controls the use of the first 8 address comparators for 1-word data fix or for code patch routine"]
    pub mod DATAFIX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Address comparator triggers a opcode patch"]
            pub const DATAFIX_0: u32 = 0;
            #[doc = "Address comparator triggers a data fix"]
            pub const DATAFIX_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROMC Disable -- This bit, when set, disables all ROMC operations"]
    pub mod DIS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Does not affect any ROMC functions (default)"]
            pub const DIS_0: u32 = 0;
            #[doc = "Disable all ROMC functions: data fixing, and opcode patching"]
            pub const DIS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ROMC Enable Register High"]
pub mod ROMPATCHENH {
    pub use crate::RO as access;
}
#[doc = "ROMC Enable Register Low"]
pub mod ROMPATCHENL {
    pub use crate::RW as access;
    #[doc = "Enable Address Comparator - This bit enables the corresponding address comparator to trigger an event"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Address comparator disabled"]
            pub const ENABLE_0: u32 = 0;
            #[doc = "Address comparator enabled, ROMC will trigger a opcode patch or data fix event upon matching of the associated address"]
            pub const ENABLE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ROMC Address Registers"]
pub mod ROMPATCHA {
    pub use crate::RW as access;
    #[doc = "THUMB Comparator Select - Indicates that this address will trigger a THUMB opcode patch or an Arm opcode patch"]
    pub mod THUMBX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Arm patch"]
            pub const THUMBX_0: u32 = 0;
            #[doc = "THUMB patch (ignore if data fix)"]
            pub const THUMBX_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address Comparator Registers - Indicates the memory address to be watched"]
    pub mod ADDRX {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x003f_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ROMC Status Register"]
pub mod ROMPATCHSR {
    pub use crate::RW as access;
    #[doc = "ROMC Source Number - Binary encoding of the number of the address comparator which has an address match in the most recent patch event on ROMC AHB"]
    pub mod SOURCE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Address Comparator 0 matched"]
            pub const SOURCE_0: u32 = 0;
            #[doc = "Address Comparator 1 matched"]
            pub const SOURCE_1: u32 = 0x01;
            #[doc = "Address Comparator 15 matched"]
            pub const SOURCE_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ROMC AHB Multiple Address Comparator matches Indicator - Indicates that multiple address comparator matches occurred"]
    pub mod SW {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "no event or comparator collisions"]
            pub const SW_0: u32 = 0;
            #[doc = "a collision has occurred"]
            pub const SW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
