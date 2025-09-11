#[doc = "IPS Domain"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "The slot assignments are given below:"]
    pub SLOT_CTRL: [SLOTCTRL::RegisterBlock; 38usize],
}
pub mod SLOTCTRL {
    #[doc = "The slot assignments are given below:"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Slot Control Register"]
        pub SLOT_CTRL: u32,
        _reserved0: [u8; 0x0c],
    }
    #[doc = "Slot Control Register"]
    pub mod SLOT_CTRL {
        pub use crate::RW as access;
        #[doc = "Domain ID of the slot to be locked"]
        pub mod LOCKED_DOMAIN_ID {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x0f << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock domain ID of this slot"]
        pub mod DOMAIN_LOCK {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Do not lock the domain ID"]
                pub const UNLOCK: u32 = 0;
                #[doc = "Lock the domain ID"]
                pub const LOCK: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Allow non-secure write access to this domain control register or domain register"]
        pub mod ALLOW_NONSECURE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Do not allow non-secure write access"]
                pub const PREVENT: u32 = 0;
                #[doc = "Allow non-secure write access"]
                pub const ALLOW: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Allow user write access to this domain control register or domain register"]
        pub mod ALLOW_USER {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Do not allow user write access"]
                pub const PREVENT: u32 = 0;
                #[doc = "Allow user write access"]
                pub const ALLOW: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock control of this slot"]
        pub mod LOCK_CONTROL {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Do not lock the control register of this slot"]
                pub const UNLOCK: u32 = 0;
                #[doc = "Lock the control register of this slot"]
                pub const LOCK: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
}
