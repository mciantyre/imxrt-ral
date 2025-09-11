#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Analog Control Register CTRL0"]
    pub CTRL0: u32,
    #[doc = "Analog Control Register CTRL0"]
    pub CTRL0_SET: u32,
    #[doc = "Analog Control Register CTRL0"]
    pub CTRL0_CLR: u32,
    #[doc = "Analog Control Register CTRL0"]
    pub CTRL0_TOG: u32,
    _reserved0: [u8; 0x40],
    #[doc = "Analog Status Register STAT0"]
    pub STAT0: u32,
    #[doc = "Analog Status Register STAT0"]
    pub STAT0_SET: u32,
    #[doc = "Analog Status Register STAT0"]
    pub STAT0_CLR: u32,
    #[doc = "Analog Status Register STAT0"]
    pub STAT0_TOG: u32,
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0 {
    pub use crate::RW as access;
    #[doc = "LinrReg master enable"]
    pub mod LINREG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LinReg power-up load disable"]
    pub mod LINREG_PWRUPLOAD_DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal pull-down enabled"]
            pub const LINREG_PWRUPLOAD_DIS_0: u32 = 0;
            #[doc = "Internal pull-down disabled"]
            pub const LINREG_PWRUPLOAD_DIS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LinReg current-limit enable"]
    pub mod LINREG_ILIMIT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LinReg output voltage target setting"]
    pub mod LINREG_OUTPUT_TRG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Isolation control for attached PHY load"]
    pub mod LINREG_PHY_ISO_B {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0_SET {
    pub use crate::RW as access;
    #[doc = "LinrReg master enable"]
    pub mod LINREG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LinReg power-up load disable"]
    pub mod LINREG_PWRUPLOAD_DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LinReg current-limit enable"]
    pub mod LINREG_ILIMIT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LinReg output voltage target setting"]
    pub mod LINREG_OUTPUT_TRG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Isolation control for attached PHY load"]
    pub mod LINREG_PHY_ISO_B {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0_CLR {
    pub use crate::RW as access;
    #[doc = "LinrReg master enable"]
    pub mod LINREG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LinReg power-up load disable"]
    pub mod LINREG_PWRUPLOAD_DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LinReg current-limit enable"]
    pub mod LINREG_ILIMIT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LinReg output voltage target setting"]
    pub mod LINREG_OUTPUT_TRG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Isolation control for attached PHY load"]
    pub mod LINREG_PHY_ISO_B {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Control Register CTRL0"]
pub mod CTRL0_TOG {
    pub use crate::RW as access;
    #[doc = "LinrReg master enable"]
    pub mod LINREG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LinReg power-up load disable"]
    pub mod LINREG_PWRUPLOAD_DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LinReg current-limit enable"]
    pub mod LINREG_ILIMIT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LinReg output voltage target setting"]
    pub mod LINREG_OUTPUT_TRG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Isolation control for attached PHY load"]
    pub mod LINREG_PHY_ISO_B {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0 {
    pub use crate::RO as access;
    #[doc = "LinReg Status Bits"]
    pub mod LINREG_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0_SET {
    pub use crate::RO as access;
    #[doc = "LinReg Status Bits"]
    pub mod LINREG_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0_CLR {
    pub use crate::RO as access;
    #[doc = "LinReg Status Bits"]
    pub mod LINREG_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Status Register STAT0"]
pub mod STAT0_TOG {
    pub use crate::RO as access;
    #[doc = "LinReg Status Bits"]
    pub mod LINREG_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
