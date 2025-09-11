#[doc = "S3MUA"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID Register"]
    pub VER: u32,
    #[doc = "Parameter Register"]
    pub PAR: u32,
    #[doc = "Unused Register 0"]
    pub UNUSED0: u32,
    #[doc = "Status Register"]
    pub SR: u32,
    _reserved0: [u8; 0x0110],
    #[doc = "Transmit Control Register"]
    pub TCR: u32,
    #[doc = "Transmit Status Register"]
    pub TSR: u32,
    #[doc = "Receive Control Register"]
    pub RCR: u32,
    #[doc = "Receive Status Register"]
    pub RSR: u32,
    _reserved1: [u8; 0xcc],
    #[doc = "Unused Register 1"]
    pub UNUSED1: u32,
    #[doc = "Transmit Register"]
    pub TR: [u32; 8usize],
    _reserved2: [u8; 0x60],
    #[doc = "Receive Register"]
    pub RR: [u32; 4usize],
}
#[doc = "Version ID Register"]
pub mod VER {
    pub use crate::RO as access;
    #[doc = "Feature Set Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard features are implemented."]
            pub const STANDARD: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Minor Version Number (0x00 )"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Major Version Number (0x01 )"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Parameter Register"]
pub mod PAR {
    pub use crate::RO as access;
    #[doc = "Number of Transmit (TRn) registers (8)"]
    pub mod TR_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of Receive (RRn) registers (4)"]
    pub mod RR_NUM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Unused Register 0"]
pub mod UNUSED0 {
    pub use crate::RO as access;
}
#[doc = "Status Register"]
pub mod SR {
    pub use crate::RO as access;
    #[doc = "Transmit Empty Pending"]
    pub mod TEP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive Full Pending Flag"]
    pub mod RFP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No data is ready to be read. All RSR\\[RFn\\] bits are clear."]
            pub const CLEAR: u32 = 0;
            #[doc = "Data is ready to be read. One or more RSR\\[RFn\\] bits are set."]
            pub const SET: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Control Register"]
pub mod TCR {
    pub use crate::RW as access;
    #[doc = "Transmit Register n Empty Interrupt Enable"]
    pub mod TEIEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Status Register"]
pub mod TSR {
    pub use crate::RO as access;
    #[doc = "Transmit Register n Empty"]
    pub mod TEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Control Register"]
pub mod RCR {
    pub use crate::RW as access;
    #[doc = "Receive Register n Full Interrupt Enable"]
    pub mod RFIEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Status Register"]
pub mod RSR {
    pub use crate::RO as access;
    #[doc = "Receive Register n Full"]
    pub mod RFN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Unused Register 1"]
pub mod UNUSED1 {
    pub use crate::RW as access;
    #[doc = "Unused 16-bit Register"]
    pub mod DATA16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Register"]
pub mod TR {
    pub use crate::RW as access;
    #[doc = "Transmit Data"]
    pub mod TR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Register"]
pub mod RR {
    pub use crate::RO as access;
    #[doc = "Receive Data"]
    pub mod RR_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
