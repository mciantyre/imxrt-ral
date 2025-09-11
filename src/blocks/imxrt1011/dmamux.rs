#[doc = "DMAMUX"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Channel 0 Configuration Register"]
    pub CHCFG: [u32; 16usize],
}
#[doc = "Channel 0 Configuration Register"]
pub mod CHCFG {
    pub use crate::RW as access;
    #[doc = "DMA Channel Source (Slot Number)"]
    pub mod SOURCE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DMA Channel Always Enable"]
    pub mod A_ON {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DMA Channel Always ON function is disabled"]
            pub const A_ON_0: u32 = 0;
            #[doc = "DMA Channel Always ON function is enabled"]
            pub const A_ON_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DMA Channel Trigger Enable"]
    pub mod TRIG {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
            pub const TRIG_0: u32 = 0;
            #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMA_CH_MUX is in Periodic Trigger mode."]
            pub const TRIG_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DMA Mux Channel Enable"]
    pub mod ENBL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DMA Mux channel is disabled"]
            pub const ENBL_0: u32 = 0;
            #[doc = "DMA Mux channel is enabled"]
            pub const ENBL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
