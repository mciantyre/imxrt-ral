#[doc = "NETC privileged"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "NETC reset register"]
    pub NETCRR: u32,
    #[doc = "NETC status register"]
    pub NETCSR: u32,
    _reserved1: [u8; 0x0100],
    #[doc = "Memory Error Injection Config Register"]
    pub MEICR: u32,
    _reserved2: [u8; 0x0bf4],
    #[doc = "Correctable memory error configuration register"]
    pub CMECR: u32,
    #[doc = "Correctable memory error status register"]
    pub CMESR: u32,
    _reserved3: [u8; 0x04],
    #[doc = "Correctable memory error count register"]
    pub CMECTR: u32,
    _reserved4: [u8; 0x20],
    #[doc = "Uncorrectable non-fatal memory error configuration register"]
    pub UNMECR: u32,
    #[doc = "Uncorrectable non-fatal memory error status register 0"]
    pub UNMESR0: u32,
    #[doc = "Uncorrectable non-fatal memory error status register 1"]
    pub UNMESR1: u32,
    #[doc = "Uncorrectable non-fatal memory error count register"]
    pub UNMECTR: u32,
    #[doc = "Uncorrectable fatal memory error configuration register"]
    pub UFMECR: u32,
    #[doc = "Uncorrectable fatal memory error status register 0"]
    pub UFMESR0: u32,
    #[doc = "Uncorrectable fatal memory error status register 1"]
    pub UFMESR1: u32,
}
#[doc = "NETC reset register"]
pub mod NETCRR {
    pub use crate::RW as access;
    #[doc = "Soft reset"]
    pub mod SR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "NETC status register"]
pub mod NETCSR {
    pub use crate::RO as access;
    #[doc = "Error"]
    pub mod ERROR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates NETC's global operational state"]
    pub mod STATE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Memory Error Injection Config Register"]
pub mod MEICR {
    pub use crate::RW as access;
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Armed"]
    pub mod ARM {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable"]
    pub mod EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Correctable memory error configuration register"]
pub mod CMECR {
    pub use crate::RW as access;
    #[doc = "Threshold"]
    pub mod THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Correctable memory error status register"]
pub mod CMESR {
    pub use crate::RW as access;
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Single-bit ECC error"]
    pub mod SBEE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Correctable memory error count register"]
pub mod CMECTR {
    pub use crate::RO as access;
    #[doc = "Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Uncorrectable non-fatal memory error configuration register"]
pub mod UNMECR {
    pub use crate::RW as access;
    #[doc = "Threshold"]
    pub mod THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Report disable"]
    pub mod RD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Uncorrectable non-fatal memory error status register 0"]
pub mod UNMESR0 {
    pub use crate::RW as access;
    #[doc = "Syndrome"]
    pub mod SYNDROME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Multi-bit ECC error"]
    pub mod MBEE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Uncorrectable non-fatal memory error status register 1"]
pub mod UNMESR1 {
    pub use crate::RO as access;
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Uncorrectable non-fatal memory error count register"]
pub mod UNMECTR {
    pub use crate::RO as access;
    #[doc = "Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Uncorrectable fatal memory error configuration register"]
pub mod UFMECR {
    pub use crate::RW as access;
    #[doc = "Report disable"]
    pub mod RD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Uncorrectable fatal memory error status register 0"]
pub mod UFMESR0 {
    pub use crate::RW as access;
    #[doc = "Syndrome"]
    pub mod SYNDROME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Multiple"]
    pub mod M {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Multi-bit ECC error"]
    pub mod MBEE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Uncorrectable fatal memory error status register 1"]
pub mod UFMESR1 {
    pub use crate::RO as access;
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
