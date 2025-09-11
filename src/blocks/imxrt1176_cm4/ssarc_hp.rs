#[doc = "SRAM Registers"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "no description available"]
    pub DESC: [DESC::RegisterBlock; 1024usize],
}
pub mod DESC {
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Description Address Register"]
        pub SRAM0_: u32,
        #[doc = "Description Data Register"]
        pub SRAM1_: u32,
        #[doc = "Description Control Register"]
        pub SRAM2_: u32,
        _reserved0: [u8; 0x04],
    }
    #[doc = "Description Address Register"]
    pub mod SRAM0_ {
        pub use crate::RW as access;
        #[doc = "Address field"]
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
    #[doc = "Description Data Register"]
    pub mod SRAM1_ {
        pub use crate::RW as access;
        #[doc = "Data field"]
        pub mod DATA {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Description Control Register"]
    pub mod SRAM2_ {
        pub use crate::RW as access;
        #[doc = "Type field"]
        pub mod TYPE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "SR"]
                pub const SR: u32 = 0;
                #[doc = "WO"]
                pub const WO: u32 = 0x01;
                #[doc = "RMW_OR"]
                pub const RMW_OR: u32 = 0x02;
                #[doc = "RMW_AND"]
                pub const RMW_AND: u32 = 0x03;
                #[doc = "DELAY"]
                pub const DELAY: u32 = 0x04;
                #[doc = "POLLING_0"]
                pub const POLLING_0: u32 = 0x05;
                #[doc = "POLLING_1"]
                pub const POLLING_1: u32 = 0x06;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Save Enable"]
        pub mod SV_EN {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Do not use this descriptor in the save operation"]
                pub const SV_EN_0: u32 = 0;
                #[doc = "Use this descriptor in the save operation"]
                pub const SV_EN_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Restore Enable"]
        pub mod RT_EN {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Do not use this descriptor for the restore operation"]
                pub const RT_EN_0: u32 = 0;
                #[doc = "Use this descriptor for the restore operation"]
                pub const RT_EN_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Size field"]
        pub mod SIZE {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0x03 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "8-bit"]
                pub const SIZE_0: u32 = 0;
                #[doc = "16-bit"]
                pub const SIZE_1: u32 = 0x01;
                #[doc = "32-bit"]
                pub const SIZE_2: u32 = 0x02;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
}
