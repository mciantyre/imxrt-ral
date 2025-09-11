#[doc = "SYS_CTR_CONTROL"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Counter Control"]
    pub CNTCR: u32,
    #[doc = "Counter Status"]
    pub CNTSR: u32,
    #[doc = "Counter Count Value Low"]
    pub CNTCV0: u32,
    #[doc = "Counter Count Value High"]
    pub CNTCV1: u32,
    _reserved0: [u8; 0x10],
    #[doc = "Frequency-Modes Table 0"]
    pub CNTFID0: u32,
    #[doc = "Frequency-Modes Table 1"]
    pub CNTFID1: u32,
    #[doc = "Frequency-Modes Table 2"]
    pub CNTFID2: u32,
    _reserved1: [u8; 0x94],
    #[doc = "Counter Control 2"]
    pub CNTCR2: u32,
    _reserved2: [u8; 0x0f0c],
    #[doc = "Counter ID"]
    pub CNTID0: u32,
}
#[doc = "Counter Control"]
pub mod CNTCR {
    pub use crate::RW as access;
    #[doc = "Enable Counting"]
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
    #[doc = "Enable Debug Halt"]
    pub mod HDBG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Ignored"]
            pub const DISABLED: u32 = 0;
            #[doc = "Causes SYS_CTR to halt"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frequency Change Request, ID 0"]
    pub mod FCR0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No change"]
            pub const DISABLED: u32 = 0;
            #[doc = "Base frequency"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frequency Change Request, ID 1"]
    pub mod FCR1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No change"]
            pub const DISABLED: u32 = 0;
            #[doc = "Base frequency"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Counter Status"]
pub mod CNTSR {
    pub use crate::RO as access;
    #[doc = "Debug Halt"]
    pub mod DBGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Did not halt"]
            pub const DISABLED: u32 = 0;
            #[doc = "Halted"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frequency Change Acknowledge, ID 0"]
    pub mod FCA0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not selected"]
            pub const DISABLED: u32 = 0;
            #[doc = "Selected"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frequency Change Acknowledge, ID 1"]
    pub mod FCA1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not selected"]
            pub const DISABLED: u32 = 0;
            #[doc = "Selected"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Counter Count Value Low"]
pub mod CNTCV0 {
    pub use crate::RW as access;
    #[doc = "Counter Count Value Bits \\[31:0\\]"]
    pub mod CNTCV0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Counter Count Value High"]
pub mod CNTCV1 {
    pub use crate::RW as access;
    #[doc = "Counter Count Value Bits \\[55:32\\]"]
    pub mod CNTCV1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frequency-Modes Table 0"]
pub mod CNTFID0 {
    pub use crate::RO as access;
    #[doc = "Counter Frequency ID 0"]
    pub mod CNTFID0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frequency-Modes Table 1"]
pub mod CNTFID1 {
    pub use crate::RO as access;
    #[doc = "Counter Frequency ID 1"]
    pub mod CNTFID1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frequency-Modes Table 2"]
pub mod CNTFID2 {
    pub use crate::RO as access;
    #[doc = "Counter Frequency ID 2"]
    pub mod CNTFID2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Counter Control 2"]
pub mod CNTCR2 {
    pub use crate::RW as access;
    #[doc = "Hardware Frequency Change Enable"]
    pub mod HWFC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect"]
            pub const DISABLED: u32 = 0;
            #[doc = "Same as performed via software"]
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
