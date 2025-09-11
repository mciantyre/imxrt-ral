#[doc = "XBAR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Crossbar Select Register"]
    pub SEL0: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL1: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL2: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL3: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL4: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL5: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL6: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL7: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL8: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL9: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL10: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL11: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL12: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL13: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL14: u16,
    #[doc = "Crossbar Select Register"]
    pub SEL15: u16,
}
#[doc = "Crossbar Select Register"]
pub mod SEL0 {
    pub use crate::RW as access;
    #[doc = "SEL0"]
    pub mod SEL0 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL1"]
    pub mod SEL1 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL1 {
    pub use crate::RW as access;
    #[doc = "SEL2"]
    pub mod SEL2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL3"]
    pub mod SEL3 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL2 {
    pub use crate::RW as access;
    #[doc = "SEL4"]
    pub mod SEL4 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL5"]
    pub mod SEL5 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL3 {
    pub use crate::RW as access;
    #[doc = "SEL6"]
    pub mod SEL6 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL7"]
    pub mod SEL7 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL4 {
    pub use crate::RW as access;
    #[doc = "SEL8"]
    pub mod SEL8 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL9"]
    pub mod SEL9 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL5 {
    pub use crate::RW as access;
    #[doc = "SEL10"]
    pub mod SEL10 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL11"]
    pub mod SEL11 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL6 {
    pub use crate::RW as access;
    #[doc = "SEL12"]
    pub mod SEL12 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL13"]
    pub mod SEL13 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL7 {
    pub use crate::RW as access;
    #[doc = "SEL14"]
    pub mod SEL14 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL15"]
    pub mod SEL15 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL8 {
    pub use crate::RW as access;
    #[doc = "SEL16"]
    pub mod SEL16 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL17"]
    pub mod SEL17 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL9 {
    pub use crate::RW as access;
    #[doc = "SEL18"]
    pub mod SEL18 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL19"]
    pub mod SEL19 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL10 {
    pub use crate::RW as access;
    #[doc = "SEL20"]
    pub mod SEL20 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL21"]
    pub mod SEL21 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL11 {
    pub use crate::RW as access;
    #[doc = "SEL22"]
    pub mod SEL22 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL23"]
    pub mod SEL23 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL12 {
    pub use crate::RW as access;
    #[doc = "SEL24"]
    pub mod SEL24 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL25"]
    pub mod SEL25 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL13 {
    pub use crate::RW as access;
    #[doc = "SEL26"]
    pub mod SEL26 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL27"]
    pub mod SEL27 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL14 {
    pub use crate::RW as access;
    #[doc = "SEL28"]
    pub mod SEL28 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL29"]
    pub mod SEL29 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Crossbar Select Register"]
pub mod SEL15 {
    pub use crate::RW as access;
    #[doc = "SEL30"]
    pub mod SEL30 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEL31"]
    pub mod SEL31 {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
