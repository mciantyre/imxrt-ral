#[doc = "ERM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "ERM Configuration Register 0"]
    pub CR0: u32,
    _reserved0: [u8; 0x0c],
    #[doc = "ERM Status Register 0"]
    pub SR0: u32,
    _reserved1: [u8; 0xf4],
    #[doc = "ERM Memory 0 Correctable Error Count Register"]
    pub CORR_ERR_CNT0: u32,
    _reserved2: [u8; 0x0c],
    #[doc = "ERM Memory 1 Correctable Error Count Register"]
    pub CORR_ERR_CNT1: u32,
    _reserved3: [u8; 0x0c],
    #[doc = "ERM Memory 2 Correctable Error Count Register"]
    pub CORR_ERR_CNT2: u32,
    _reserved4: [u8; 0x0c],
    #[doc = "ERM Memory 3 Correctable Error Count Register"]
    pub CORR_ERR_CNT3: u32,
}
#[doc = "ERM Configuration Register 0"]
pub mod CR0 {
    pub use crate::RW as access;
    #[doc = "ENCIE3"]
    pub mod ENCIE3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt notification of Memory 3 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 3 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ESCIE3"]
    pub mod ESCIE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt notification of Memory 3 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 3 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ENCIE2"]
    pub mod ENCIE2 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt notification of Memory 2 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 2 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ESCIE2"]
    pub mod ESCIE2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt notification of Memory 2 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 2 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ENCIE1"]
    pub mod ENCIE1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt notification of Memory 1 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 1 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ESCIE1"]
    pub mod ESCIE1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt notification of Memory 1 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 1 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ENCIE0"]
    pub mod ENCIE0 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt notification of Memory 0 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 0 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ESCIE0"]
    pub mod ESCIE0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt notification of Memory 0 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 0 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ERM Status Register 0"]
pub mod SR0 {
    pub use crate::RW as access;
    #[doc = "NCE3"]
    pub mod NCE3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No non-correctable error event on Memory 3 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 3 detected."]
            pub const ERROR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SBC3"]
    pub mod SBC3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No single-bit correction event on Memory 3 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 3 detected."]
            pub const EVENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NCE2"]
    pub mod NCE2 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No non-correctable error event on Memory 2 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 2 detected."]
            pub const ERROR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SBC2"]
    pub mod SBC2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No single-bit correction event on Memory 2 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 2 detected."]
            pub const EVENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NCE1"]
    pub mod NCE1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No non-correctable error event on Memory 1 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 1 detected."]
            pub const ERROR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SBC1"]
    pub mod SBC1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No single-bit correction event on Memory 1 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 1 detected."]
            pub const EVENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "NCE0"]
    pub mod NCE0 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No non-correctable error event on Memory 0 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 0 detected."]
            pub const ERROR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SBC0"]
    pub mod SBC0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No single-bit correction event on Memory 0 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 0 detected."]
            pub const EVENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ERM Memory 0 Correctable Error Count Register"]
pub mod CORR_ERR_CNT0 {
    pub use crate::RW as access;
    #[doc = "Memory n Correctable Error Count"]
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
#[doc = "ERM Memory 1 Correctable Error Count Register"]
pub mod CORR_ERR_CNT1 {
    pub use crate::RW as access;
    #[doc = "Memory n Correctable Error Count"]
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
#[doc = "ERM Memory 2 Correctable Error Count Register"]
pub mod CORR_ERR_CNT2 {
    pub use crate::RW as access;
    #[doc = "Memory n Correctable Error Count"]
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
#[doc = "ERM Memory 3 Correctable Error Count Register"]
pub mod CORR_ERR_CNT3 {
    pub use crate::RW as access;
    #[doc = "Memory n Correctable Error Count"]
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
