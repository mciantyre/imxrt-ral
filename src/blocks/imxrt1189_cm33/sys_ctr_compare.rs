#[doc = "SYS_CTR_COMPARE"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    #[doc = "Compare Count Value Low"]
    pub CMPCVL0: u32,
    #[doc = "Compare Count Value High"]
    pub CMPCVH0: u32,
    _reserved1: [u8; 0x04],
    #[doc = "Compare Control"]
    pub CMPCR0: u32,
    _reserved2: [u8; 0xf0],
    #[doc = "Compare Count Value Low"]
    pub CMPCVL1: u32,
    #[doc = "Compare Count Value High"]
    pub CMPCVH1: u32,
    _reserved3: [u8; 0x04],
    #[doc = "Compare Control"]
    pub CMPCR1: u32,
    _reserved4: [u8; 0x0ea0],
    #[doc = "Counter ID"]
    pub CNTID0: u32,
}
#[doc = "Compare Count Value Low"]
pub mod CMPCVL0 {
    pub use crate::RW as access;
    #[doc = "Compare Count Value Bits \\[31:0\\]"]
    pub mod CMPCV0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Compare Count Value High"]
pub mod CMPCVH0 {
    pub use crate::RW as access;
    #[doc = "Compare Count Value Bits \\[55:32\\]"]
    pub mod CMPCV1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Compare Control"]
pub mod CMPCR0 {
    pub use crate::RW as access;
    #[doc = "Compare Enable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Interrupt Request Mask"]
    pub mod IMASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not masked"]
            pub const DISABLED: u32 = 0;
            #[doc = "Masked"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Compare Interrupt Status"]
    pub mod ISTAT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Either less than the compare value or compare is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Greater than or equal to the compare value and compare is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Compare Count Value Low"]
pub mod CMPCVL1 {
    pub use crate::RW as access;
    #[doc = "Compare Count Value Bits \\[31:0\\]"]
    pub mod CMPCV0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Compare Count Value High"]
pub mod CMPCVH1 {
    pub use crate::RW as access;
    #[doc = "Compare Count Value Bits \\[55:32\\]"]
    pub mod CMPCV1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Compare Control"]
pub mod CMPCR1 {
    pub use crate::RW as access;
    #[doc = "Compare Enable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Interrupt Request Mask"]
    pub mod IMASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not masked"]
            pub const DISABLED: u32 = 0;
            #[doc = "Masked"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Compare Interrupt Status"]
    pub mod ISTAT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Either less than the compare value or compare is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Greater than or equal to the compare value and compare is enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Counter ID"]
pub mod CNTID0 {
    pub use crate::RO as access;
    #[doc = "Counter Identification"]
    pub mod CNTID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
