#[doc = "GPC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPC Interface control register"]
    pub CNTR: u32,
    _reserved0: [u8; 0x04],
    #[doc = "IRQ masking register 1"]
    pub IMR1: u32,
    #[doc = "IRQ masking register 2"]
    pub IMR2: u32,
    #[doc = "IRQ masking register 3"]
    pub IMR3: u32,
    #[doc = "IRQ masking register 4"]
    pub IMR4: u32,
    #[doc = "IRQ status resister 1"]
    pub ISR1: u32,
    #[doc = "IRQ status resister 2"]
    pub ISR2: u32,
    #[doc = "IRQ status resister 3"]
    pub ISR3: u32,
    #[doc = "IRQ status resister 4"]
    pub ISR4: u32,
    _reserved1: [u8; 0x0c],
    #[doc = "IRQ masking register 5"]
    pub IMR5: u32,
    #[doc = "IRQ status resister 5"]
    pub ISR5: u32,
}
#[doc = "GPC Interface control register"]
pub mod CNTR {
    pub use crate::RW as access;
    #[doc = "MEGA domain power down request"]
    pub mod MEGA_PDN_REQ {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Request"]
            pub const MEGA_PDN_REQ_0: u32 = 0;
            #[doc = "Request power down sequence"]
            pub const MEGA_PDN_REQ_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MEGA domain power up request"]
    pub mod MEGA_PUP_REQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Request"]
            pub const MEGA_PUP_REQ_0: u32 = 0;
            #[doc = "Request power up sequence"]
            pub const MEGA_PUP_REQ_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FlexRAM PDRAM0 Power Gate Enable"]
    pub mod PDRAM0_PGE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FlexRAM PDRAM0 domain will keep power even if the CPU core is powered down."]
            pub const PDRAM0_PGE_0: u32 = 0;
            #[doc = "FlexRAM PDRAM0 domain will be powered down when the CPU core is powered down.."]
            pub const PDRAM0_PGE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ masking register 1"]
pub mod IMR1 {
    pub use crate::RW as access;
    #[doc = "IRQ\\[31:0\\] masking bits: 1-irq masked, 0-irq is not masked"]
    pub mod IMR1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ masking register 2"]
pub mod IMR2 {
    pub use crate::RW as access;
    #[doc = "IRQ\\[63:32\\] masking bits: 1-irq masked, 0-irq is not masked"]
    pub mod IMR2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ masking register 3"]
pub mod IMR3 {
    pub use crate::RW as access;
    #[doc = "IRQ\\[95:64\\] masking bits: 1-irq masked, 0-irq is not masked"]
    pub mod IMR3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ masking register 4"]
pub mod IMR4 {
    pub use crate::RW as access;
    #[doc = "IRQ\\[127:96\\] masking bits: 1-irq masked, 0-irq is not masked"]
    pub mod IMR4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ status resister 1"]
pub mod ISR1 {
    pub use crate::RO as access;
    #[doc = "IRQ\\[31:0\\] status, read only"]
    pub mod ISR1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ status resister 2"]
pub mod ISR2 {
    pub use crate::RO as access;
    #[doc = "IRQ\\[63:32\\] status, read only"]
    pub mod ISR2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ status resister 3"]
pub mod ISR3 {
    pub use crate::RO as access;
    #[doc = "IRQ\\[95:64\\] status, read only"]
    pub mod ISR3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ status resister 4"]
pub mod ISR4 {
    pub use crate::RO as access;
    #[doc = "IRQ\\[127:96\\] status, read only"]
    pub mod ISR4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ masking register 5"]
pub mod IMR5 {
    pub use crate::RW as access;
    #[doc = "IRQ\\[159:128\\] masking bits: 1-irq masked, 0-irq is not masked"]
    pub mod IMR5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IRQ status resister 5"]
pub mod ISR5 {
    pub use crate::RO as access;
    #[doc = "IRQ\\[159:128\\] status, read only"]
    pub mod ISR4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
