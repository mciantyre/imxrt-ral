#[doc = "Crossbar Switch"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Crossbar B Select Register 0"]
    pub SEL0: u16,
    #[doc = "Crossbar B Select Register 1"]
    pub SEL1: u16,
    #[doc = "Crossbar B Select Register 2"]
    pub SEL2: u16,
    #[doc = "Crossbar B Select Register 3"]
    pub SEL3: u16,
    #[doc = "Crossbar B Select Register 4"]
    pub SEL4: u16,
    #[doc = "Crossbar B Select Register 5"]
    pub SEL5: u16,
    #[doc = "Crossbar B Select Register 6"]
    pub SEL6: u16,
    #[doc = "Crossbar B Select Register 7"]
    pub SEL7: u16,
}
#[doc = "Crossbar B Select Register 0"]
pub mod SEL0 {
    pub use crate::RW as access;
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT0 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT1 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL1 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar B Select Register 1"]
pub mod SEL1 {
    pub use crate::RW as access;
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT2 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT3 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL3 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar B Select Register 2"]
pub mod SEL2 {
    pub use crate::RW as access;
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT4 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL4 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT5 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL5 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar B Select Register 3"]
pub mod SEL3 {
    pub use crate::RW as access;
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT6 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL6 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT7 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL7 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar B Select Register 4"]
pub mod SEL4 {
    pub use crate::RW as access;
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT8 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL8 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT9 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL9 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar B Select Register 5"]
pub mod SEL5 {
    pub use crate::RW as access;
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT10 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL10 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT11 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL11 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar B Select Register 6"]
pub mod SEL6 {
    pub use crate::RW as access;
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT12 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL12 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT13 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL13 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar B Select Register 7"]
pub mod SEL7 {
    pub use crate::RW as access;
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT14 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL14 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Input (XBARB_INn) to be muxed to XBARB_OUT15 (refer to Functional Description section for input/output assignment)"]
    pub mod SEL15 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
