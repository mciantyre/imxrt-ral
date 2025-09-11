#[doc = "GPIO"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPIO data register"]
    pub DR: u32,
    #[doc = "GPIO direction register"]
    pub GDIR: u32,
    #[doc = "GPIO pad status register"]
    pub PSR: u32,
    #[doc = "GPIO interrupt configuration register1"]
    pub ICR1: u32,
    #[doc = "GPIO interrupt configuration register2"]
    pub ICR2: u32,
    #[doc = "GPIO interrupt mask register"]
    pub IMR: u32,
    #[doc = "GPIO interrupt status register"]
    pub ISR: u32,
    #[doc = "GPIO edge select register"]
    pub EDGE_SEL: u32,
    _reserved0: [u8; 0x64],
    #[doc = "GPIO data register SET"]
    pub DR_SET: u32,
    #[doc = "GPIO data register CLEAR"]
    pub DR_CLEAR: u32,
    #[doc = "GPIO data register TOGGLE"]
    pub DR_TOGGLE: u32,
}
#[doc = "GPIO data register"]
pub mod DR {
    pub use crate::RW as access;
    #[doc = "DR"]
    pub mod DR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPIO direction register"]
pub mod GDIR {
    pub use crate::RW as access;
    #[doc = "GDIR"]
    pub mod GDIR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPIO pad status register"]
pub mod PSR {
    pub use crate::RO as access;
    #[doc = "PSR"]
    pub mod PSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPIO interrupt configuration register1"]
pub mod ICR1 {
    pub use crate::RW as access;
    #[doc = "ICR0"]
    pub mod ICR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR1"]
    pub mod ICR1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR2"]
    pub mod ICR2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR3"]
    pub mod ICR3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR4"]
    pub mod ICR4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR5"]
    pub mod ICR5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR6"]
    pub mod ICR6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR7"]
    pub mod ICR7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR8"]
    pub mod ICR8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR9"]
    pub mod ICR9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR10"]
    pub mod ICR10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR11"]
    pub mod ICR11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR12"]
    pub mod ICR12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR13"]
    pub mod ICR13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR14"]
    pub mod ICR14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR15"]
    pub mod ICR15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPIO interrupt configuration register2"]
pub mod ICR2 {
    pub use crate::RW as access;
    #[doc = "ICR16"]
    pub mod ICR16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR17"]
    pub mod ICR17 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR18"]
    pub mod ICR18 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR19"]
    pub mod ICR19 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR20"]
    pub mod ICR20 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR21"]
    pub mod ICR21 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR22"]
    pub mod ICR22 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR23"]
    pub mod ICR23 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR24"]
    pub mod ICR24 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR25"]
    pub mod ICR25 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR26"]
    pub mod ICR26 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR27"]
    pub mod ICR27 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR28"]
    pub mod ICR28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR29"]
    pub mod ICR29 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR30"]
    pub mod ICR30 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ICR31"]
    pub mod ICR31 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt n is low-level sensitive."]
            pub const LOW_LEVEL: u32 = 0;
            #[doc = "Interrupt n is high-level sensitive."]
            pub const HIGH_LEVEL: u32 = 0x01;
            #[doc = "Interrupt n is rising-edge sensitive."]
            pub const RISING_EDGE: u32 = 0x02;
            #[doc = "Interrupt n is falling-edge sensitive."]
            pub const FALLING_EDGE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPIO interrupt mask register"]
pub mod IMR {
    pub use crate::RW as access;
    #[doc = "IMR"]
    pub mod IMR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPIO interrupt status register"]
pub mod ISR {
    pub use crate::RW as access;
    #[doc = "ISR"]
    pub mod ISR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPIO edge select register"]
pub mod EDGE_SEL {
    pub use crate::RW as access;
    #[doc = "GPIO_EDGE_SEL"]
    pub mod GPIO_EDGE_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPIO data register SET"]
pub mod DR_SET {
    pub use crate::WO as access;
    #[doc = "DR_SET"]
    pub mod DR_SET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPIO data register CLEAR"]
pub mod DR_CLEAR {
    pub use crate::WO as access;
    #[doc = "DR_CLEAR"]
    pub mod DR_CLEAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPIO data register TOGGLE"]
pub mod DR_TOGGLE {
    pub use crate::WO as access;
    #[doc = "DR_TOGGLE"]
    pub mod DR_TOGGLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
