#[doc = "CCM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Clock root section."]
    pub CLOCK_ROOT: [CLOCKROOT::RegisterBlock; 74usize],
    _reserved0: [u8; 0x1f00],
    #[doc = "Clock root section."]
    pub OBSERVE: [OBSERVE::RegisterBlock; 2usize],
    _reserved1: [u8; 0x0300],
    #[doc = "General Purpose Register"]
    pub GPR_SHARED0: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED0_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED0_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED0_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED0_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED0_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED0_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED0_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED1: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED1_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED1_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED1_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED1_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED1_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED1_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED1_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED2: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED2_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED2_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED2_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED2_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED2_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED2_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED2_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED3: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED3_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED3_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED3_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED3_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED3_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED3_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED3_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED4: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED4_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED4_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED4_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED4_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED4_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED4_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED4_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED5: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED5_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED5_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED5_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED5_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED5_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED5_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED5_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED6: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED6_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED6_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED6_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED6_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED6_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED6_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED6_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED7: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED7_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED7_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED7_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED7_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED7_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED7_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED7_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED8: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED8_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED8_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED8_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED8_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED8_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED8_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED8_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED9: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED9_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED9_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED9_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED9_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED9_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED9_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED9_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED10: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED10_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED10_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED10_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED10_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED10_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED10_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED10_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED11: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED11_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED11_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED11_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED11_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED11_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED11_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED11_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED12: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED12_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED12_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED12_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED12_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED12_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED12_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED12_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED13: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED13_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED13_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED13_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED13_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED13_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED13_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED13_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED14: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED14_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED14_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED14_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED14_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED14_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED14_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED14_AUTHEN_TOG: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED15: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED15_SET: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED15_CLR: u32,
    #[doc = "General Purpose Register"]
    pub GPR_SHARED15_TOG: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED15_AUTHEN: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED15_AUTHEN_SET: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED15_AUTHEN_CLR: u32,
    #[doc = "GPR access control"]
    pub GPR_SHARED15_AUTHEN_TOG: u32,
    #[doc = "General purpose status register for CM33"]
    pub GPR_SHARED_STATUS0: u32,
    #[doc = "General purpose status register for CM33"]
    pub GPR_SHARED_STATUS1: u32,
    #[doc = "General purpose status register for CM33"]
    pub GPR_SHARED_STATUS2: u32,
    #[doc = "General purpose status register for CM33"]
    pub GPR_SHARED_STATUS3: u32,
    #[doc = "General status register for CM7"]
    pub GPR_SHARED_STATUS4: u32,
    #[doc = "General purpose status register for CM7"]
    pub GPR_SHARED_STATUS5: u32,
    #[doc = "General status register for CM7"]
    pub GPR_SHARED_STATUS6: u32,
    #[doc = "General purpose status register for CM7"]
    pub GPR_SHARED_STATUS7: u32,
    _reserved2: [u8; 0x01e0],
    #[doc = "General purpose register section."]
    pub GPR_PRIVATE: [GPRPRIVATE::RegisterBlock; 4usize],
    _reserved3: [u8; 0x0380],
    #[doc = "Clock source section."]
    pub OSCPLL: [OSCPLL::RegisterBlock; 25usize],
    _reserved4: [u8; 0x29c0],
    #[doc = "LPCG section."]
    pub LPCG: [LPCG::RegisterBlock; 149usize],
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED0 {
    pub use crate::RW as access;
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED0_SET {
    pub use crate::RW as access;
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED0_CLR {
    pub use crate::RW as access;
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED0_TOG {
    pub use crate::RW as access;
}
#[doc = "GPR access control"]
pub mod GPR_SHARED0_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED0_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED0_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED0_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED1 {
    pub use crate::RW as access;
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED1_SET {
    pub use crate::RW as access;
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED1_CLR {
    pub use crate::RW as access;
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED1_TOG {
    pub use crate::RW as access;
}
#[doc = "GPR access control"]
pub mod GPR_SHARED1_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED1_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED1_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED1_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED2 {
    pub use crate::RW as access;
    #[doc = "m33_mask_cm7"]
    pub mod M33_MASK_CM7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_cm33"]
    pub mod M33_MASK_CM33 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_edma3"]
    pub mod M33_MASK_EDMA3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_edma4"]
    pub mod M33_MASK_EDMA4 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_netc"]
    pub mod M33_MASK_NETC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sim_aon"]
    pub mod M33_MASK_SIM_AON {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_adc1"]
    pub mod M33_MASK_ADC1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_adc2"]
    pub mod M33_MASK_ADC2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexspi1"]
    pub mod M33_MASK_FLEXSPI1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexspi2"]
    pub mod M33_MASK_FLEXSPI2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_trdc"]
    pub mod M33_MASK_TRDC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_semc"]
    pub mod M33_MASK_SEMC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_iee"]
    pub mod M33_MASK_IEE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio1"]
    pub mod M33_MASK_GPIO1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio2"]
    pub mod M33_MASK_GPIO2 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio3"]
    pub mod M33_MASK_GPIO3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio4"]
    pub mod M33_MASK_GPIO4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio5"]
    pub mod M33_MASK_GPIO5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio6"]
    pub mod M33_MASK_GPIO6 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexio1"]
    pub mod M33_MASK_FLEXIO1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexio2"]
    pub mod M33_MASK_FLEXIO2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpit1"]
    pub mod M33_MASK_LPIT1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpit2"]
    pub mod M33_MASK_LPIT2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpit3"]
    pub mod M33_MASK_LPIT3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm1"]
    pub mod M33_MASK_TPM1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm2"]
    pub mod M33_MASK_TPM2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm3"]
    pub mod M33_MASK_TPM3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm4"]
    pub mod M33_MASK_TPM4 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm5"]
    pub mod M33_MASK_TPM5 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED2_SET {
    pub use crate::RW as access;
    #[doc = "m33_mask_cm7"]
    pub mod M33_MASK_CM7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_cm33"]
    pub mod M33_MASK_CM33 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_edma3"]
    pub mod M33_MASK_EDMA3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_edma4"]
    pub mod M33_MASK_EDMA4 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_netc"]
    pub mod M33_MASK_NETC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sim_aon"]
    pub mod M33_MASK_SIM_AON {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_adc1"]
    pub mod M33_MASK_ADC1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_adc2"]
    pub mod M33_MASK_ADC2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexspi1"]
    pub mod M33_MASK_FLEXSPI1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexspi2"]
    pub mod M33_MASK_FLEXSPI2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_trdc"]
    pub mod M33_MASK_TRDC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_semc"]
    pub mod M33_MASK_SEMC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_iee"]
    pub mod M33_MASK_IEE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio1"]
    pub mod M33_MASK_GPIO1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio2"]
    pub mod M33_MASK_GPIO2 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio3"]
    pub mod M33_MASK_GPIO3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio4"]
    pub mod M33_MASK_GPIO4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio5"]
    pub mod M33_MASK_GPIO5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio6"]
    pub mod M33_MASK_GPIO6 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexio1"]
    pub mod M33_MASK_FLEXIO1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexio2"]
    pub mod M33_MASK_FLEXIO2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpit1"]
    pub mod M33_MASK_LPIT1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpit2"]
    pub mod M33_MASK_LPIT2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpit3"]
    pub mod M33_MASK_LPIT3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm1"]
    pub mod M33_MASK_TPM1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm2"]
    pub mod M33_MASK_TPM2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm3"]
    pub mod M33_MASK_TPM3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm4"]
    pub mod M33_MASK_TPM4 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm5"]
    pub mod M33_MASK_TPM5 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED2_CLR {
    pub use crate::RW as access;
    #[doc = "m33_mask_cm7"]
    pub mod M33_MASK_CM7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_cm33"]
    pub mod M33_MASK_CM33 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_edma3"]
    pub mod M33_MASK_EDMA3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_edma4"]
    pub mod M33_MASK_EDMA4 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_netc"]
    pub mod M33_MASK_NETC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sim_aon"]
    pub mod M33_MASK_SIM_AON {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_adc1"]
    pub mod M33_MASK_ADC1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_adc2"]
    pub mod M33_MASK_ADC2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexspi1"]
    pub mod M33_MASK_FLEXSPI1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexspi2"]
    pub mod M33_MASK_FLEXSPI2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_trdc"]
    pub mod M33_MASK_TRDC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_semc"]
    pub mod M33_MASK_SEMC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_iee"]
    pub mod M33_MASK_IEE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio1"]
    pub mod M33_MASK_GPIO1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio2"]
    pub mod M33_MASK_GPIO2 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio3"]
    pub mod M33_MASK_GPIO3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio4"]
    pub mod M33_MASK_GPIO4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio5"]
    pub mod M33_MASK_GPIO5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio6"]
    pub mod M33_MASK_GPIO6 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexio1"]
    pub mod M33_MASK_FLEXIO1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexio2"]
    pub mod M33_MASK_FLEXIO2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpit1"]
    pub mod M33_MASK_LPIT1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpit2"]
    pub mod M33_MASK_LPIT2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpit3"]
    pub mod M33_MASK_LPIT3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm1"]
    pub mod M33_MASK_TPM1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm2"]
    pub mod M33_MASK_TPM2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm3"]
    pub mod M33_MASK_TPM3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm4"]
    pub mod M33_MASK_TPM4 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm5"]
    pub mod M33_MASK_TPM5 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED2_TOG {
    pub use crate::RW as access;
    #[doc = "m33_mask_cm7"]
    pub mod M33_MASK_CM7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_cm33"]
    pub mod M33_MASK_CM33 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_edma3"]
    pub mod M33_MASK_EDMA3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_edma4"]
    pub mod M33_MASK_EDMA4 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_netc"]
    pub mod M33_MASK_NETC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sim_aon"]
    pub mod M33_MASK_SIM_AON {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_adc1"]
    pub mod M33_MASK_ADC1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_adc2"]
    pub mod M33_MASK_ADC2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexspi1"]
    pub mod M33_MASK_FLEXSPI1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexspi2"]
    pub mod M33_MASK_FLEXSPI2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_trdc"]
    pub mod M33_MASK_TRDC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_semc"]
    pub mod M33_MASK_SEMC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_iee"]
    pub mod M33_MASK_IEE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio1"]
    pub mod M33_MASK_GPIO1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio2"]
    pub mod M33_MASK_GPIO2 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio3"]
    pub mod M33_MASK_GPIO3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio4"]
    pub mod M33_MASK_GPIO4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio5"]
    pub mod M33_MASK_GPIO5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpio6"]
    pub mod M33_MASK_GPIO6 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexio1"]
    pub mod M33_MASK_FLEXIO1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_flexio2"]
    pub mod M33_MASK_FLEXIO2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpit1"]
    pub mod M33_MASK_LPIT1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpit2"]
    pub mod M33_MASK_LPIT2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpit3"]
    pub mod M33_MASK_LPIT3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm1"]
    pub mod M33_MASK_TPM1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm2"]
    pub mod M33_MASK_TPM2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm3"]
    pub mod M33_MASK_TPM3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm4"]
    pub mod M33_MASK_TPM4 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_tpm5"]
    pub mod M33_MASK_TPM5 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED2_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED2_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED2_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED2_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED3 {
    pub use crate::RW as access;
    #[doc = "m33_mask_tpm6"]
    pub mod M33_MASK_TPM6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpt1"]
    pub mod M33_MASK_GPT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpt2"]
    pub mod M33_MASK_GPT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_can1"]
    pub mod M33_MASK_CAN1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_can2"]
    pub mod M33_MASK_CAN2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_can3"]
    pub mod M33_MASK_CAN3 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart1"]
    pub mod M33_MASK_LPUART1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart2"]
    pub mod M33_MASK_LPUART2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart3"]
    pub mod M33_MASK_LPUART3 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart4"]
    pub mod M33_MASK_LPUART4 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart5"]
    pub mod M33_MASK_LPUART5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart6"]
    pub mod M33_MASK_LPUART6 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart7"]
    pub mod M33_MASK_LPUART7 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart8"]
    pub mod M33_MASK_LPUART8 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart9"]
    pub mod M33_MASK_LPUART9 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart10"]
    pub mod M33_MASK_LPUART10 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart11"]
    pub mod M33_MASK_LPUART11 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart12"]
    pub mod M33_MASK_LPUART12 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c1"]
    pub mod M33_MASK_LPI2C1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c2"]
    pub mod M33_MASK_LPI2C2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c3"]
    pub mod M33_MASK_LPI2C3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c4"]
    pub mod M33_MASK_LPI2C4 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c5"]
    pub mod M33_MASK_LPI2C5 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c6"]
    pub mod M33_MASK_LPI2C6 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi1"]
    pub mod M33_MASK_LPSPI1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi2"]
    pub mod M33_MASK_LPSPI2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi3"]
    pub mod M33_MASK_LPSPI3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi4"]
    pub mod M33_MASK_LPSPI4 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi5"]
    pub mod M33_MASK_LPSPI5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi6"]
    pub mod M33_MASK_LPSPI6 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sinc1"]
    pub mod M33_MASK_SINC1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sinc2"]
    pub mod M33_MASK_SINC2 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED3_SET {
    pub use crate::RW as access;
    #[doc = "m33_mask_tpm6"]
    pub mod M33_MASK_TPM6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpt1"]
    pub mod M33_MASK_GPT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpt2"]
    pub mod M33_MASK_GPT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_can1"]
    pub mod M33_MASK_CAN1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_can2"]
    pub mod M33_MASK_CAN2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_can3"]
    pub mod M33_MASK_CAN3 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart1"]
    pub mod M33_MASK_LPUART1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart2"]
    pub mod M33_MASK_LPUART2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart3"]
    pub mod M33_MASK_LPUART3 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart4"]
    pub mod M33_MASK_LPUART4 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart5"]
    pub mod M33_MASK_LPUART5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart6"]
    pub mod M33_MASK_LPUART6 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart7"]
    pub mod M33_MASK_LPUART7 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart8"]
    pub mod M33_MASK_LPUART8 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart9"]
    pub mod M33_MASK_LPUART9 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart10"]
    pub mod M33_MASK_LPUART10 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart11"]
    pub mod M33_MASK_LPUART11 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart12"]
    pub mod M33_MASK_LPUART12 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c1"]
    pub mod M33_MASK_LPI2C1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c2"]
    pub mod M33_MASK_LPI2C2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c3"]
    pub mod M33_MASK_LPI2C3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c4"]
    pub mod M33_MASK_LPI2C4 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c5"]
    pub mod M33_MASK_LPI2C5 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c6"]
    pub mod M33_MASK_LPI2C6 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi1"]
    pub mod M33_MASK_LPSPI1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi2"]
    pub mod M33_MASK_LPSPI2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi3"]
    pub mod M33_MASK_LPSPI3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi4"]
    pub mod M33_MASK_LPSPI4 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi5"]
    pub mod M33_MASK_LPSPI5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi6"]
    pub mod M33_MASK_LPSPI6 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sinc1"]
    pub mod M33_MASK_SINC1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sinc2"]
    pub mod M33_MASK_SINC2 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED3_CLR {
    pub use crate::RW as access;
    #[doc = "m33_mask_tpm6"]
    pub mod M33_MASK_TPM6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpt1"]
    pub mod M33_MASK_GPT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpt2"]
    pub mod M33_MASK_GPT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_can1"]
    pub mod M33_MASK_CAN1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_can2"]
    pub mod M33_MASK_CAN2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_can3"]
    pub mod M33_MASK_CAN3 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart1"]
    pub mod M33_MASK_LPUART1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart2"]
    pub mod M33_MASK_LPUART2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart3"]
    pub mod M33_MASK_LPUART3 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart4"]
    pub mod M33_MASK_LPUART4 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart5"]
    pub mod M33_MASK_LPUART5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart6"]
    pub mod M33_MASK_LPUART6 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart7"]
    pub mod M33_MASK_LPUART7 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart8"]
    pub mod M33_MASK_LPUART8 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart9"]
    pub mod M33_MASK_LPUART9 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart10"]
    pub mod M33_MASK_LPUART10 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart11"]
    pub mod M33_MASK_LPUART11 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart12"]
    pub mod M33_MASK_LPUART12 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c1"]
    pub mod M33_MASK_LPI2C1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c2"]
    pub mod M33_MASK_LPI2C2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c3"]
    pub mod M33_MASK_LPI2C3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c4"]
    pub mod M33_MASK_LPI2C4 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c5"]
    pub mod M33_MASK_LPI2C5 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c6"]
    pub mod M33_MASK_LPI2C6 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi1"]
    pub mod M33_MASK_LPSPI1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi2"]
    pub mod M33_MASK_LPSPI2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi3"]
    pub mod M33_MASK_LPSPI3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi4"]
    pub mod M33_MASK_LPSPI4 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi5"]
    pub mod M33_MASK_LPSPI5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi6"]
    pub mod M33_MASK_LPSPI6 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sinc1"]
    pub mod M33_MASK_SINC1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sinc2"]
    pub mod M33_MASK_SINC2 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED3_TOG {
    pub use crate::RW as access;
    #[doc = "m33_mask_tpm6"]
    pub mod M33_MASK_TPM6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpt1"]
    pub mod M33_MASK_GPT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_gpt2"]
    pub mod M33_MASK_GPT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_can1"]
    pub mod M33_MASK_CAN1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_can2"]
    pub mod M33_MASK_CAN2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_can3"]
    pub mod M33_MASK_CAN3 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart1"]
    pub mod M33_MASK_LPUART1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart2"]
    pub mod M33_MASK_LPUART2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart3"]
    pub mod M33_MASK_LPUART3 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart4"]
    pub mod M33_MASK_LPUART4 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart5"]
    pub mod M33_MASK_LPUART5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart6"]
    pub mod M33_MASK_LPUART6 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart7"]
    pub mod M33_MASK_LPUART7 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart8"]
    pub mod M33_MASK_LPUART8 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart9"]
    pub mod M33_MASK_LPUART9 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart10"]
    pub mod M33_MASK_LPUART10 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart11"]
    pub mod M33_MASK_LPUART11 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpuart12"]
    pub mod M33_MASK_LPUART12 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c1"]
    pub mod M33_MASK_LPI2C1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c2"]
    pub mod M33_MASK_LPI2C2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c3"]
    pub mod M33_MASK_LPI2C3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c4"]
    pub mod M33_MASK_LPI2C4 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c5"]
    pub mod M33_MASK_LPI2C5 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpi2c6"]
    pub mod M33_MASK_LPI2C6 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi1"]
    pub mod M33_MASK_LPSPI1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi2"]
    pub mod M33_MASK_LPSPI2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi3"]
    pub mod M33_MASK_LPSPI3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi4"]
    pub mod M33_MASK_LPSPI4 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi5"]
    pub mod M33_MASK_LPSPI5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_lpspi6"]
    pub mod M33_MASK_LPSPI6 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sinc1"]
    pub mod M33_MASK_SINC1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sinc2"]
    pub mod M33_MASK_SINC2 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED3_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED3_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED3_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED3_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED4 {
    pub use crate::RW as access;
    #[doc = "m33_mask_sinc3"]
    pub mod M33_MASK_SINC3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai1"]
    pub mod M33_MASK_SAI1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai2"]
    pub mod M33_MASK_SAI2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai3"]
    pub mod M33_MASK_SAI3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai4"]
    pub mod M33_MASK_SAI4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_mic"]
    pub mod M33_MASK_MIC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED4_SET {
    pub use crate::RW as access;
    #[doc = "m33_mask_sinc3"]
    pub mod M33_MASK_SINC3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai1"]
    pub mod M33_MASK_SAI1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai2"]
    pub mod M33_MASK_SAI2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai3"]
    pub mod M33_MASK_SAI3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai4"]
    pub mod M33_MASK_SAI4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_mic"]
    pub mod M33_MASK_MIC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED4_CLR {
    pub use crate::RW as access;
    #[doc = "m33_mask_sinc3"]
    pub mod M33_MASK_SINC3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai1"]
    pub mod M33_MASK_SAI1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai2"]
    pub mod M33_MASK_SAI2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai3"]
    pub mod M33_MASK_SAI3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai4"]
    pub mod M33_MASK_SAI4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_mic"]
    pub mod M33_MASK_MIC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED4_TOG {
    pub use crate::RW as access;
    #[doc = "m33_mask_sinc3"]
    pub mod M33_MASK_SINC3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai1"]
    pub mod M33_MASK_SAI1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai2"]
    pub mod M33_MASK_SAI2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai3"]
    pub mod M33_MASK_SAI3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_sai4"]
    pub mod M33_MASK_SAI4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mask_mic"]
    pub mod M33_MASK_MIC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED4_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED4_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED4_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED4_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED5 {
    pub use crate::RW as access;
    #[doc = "m7_mask_cm7"]
    pub mod M7_MASK_CM7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_cm33"]
    pub mod M7_MASK_CM33 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_edma3"]
    pub mod M7_MASK_EDMA3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_edma4"]
    pub mod M7_MASK_EDMA4 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_netc"]
    pub mod M7_MASK_NETC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sim_aon"]
    pub mod M7_MASK_SIM_AON {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_adc1"]
    pub mod M7_MASK_ADC1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_adc2"]
    pub mod M7_MASK_ADC2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexspi1"]
    pub mod M7_MASK_FLEXSPI1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexspi2"]
    pub mod M7_MASK_FLEXSPI2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_trdc"]
    pub mod M7_MASK_TRDC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_semc"]
    pub mod M7_MASK_SEMC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_iee"]
    pub mod M7_MASK_IEE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio1"]
    pub mod M7_MASK_GPIO1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio2"]
    pub mod M7_MASK_GPIO2 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio3"]
    pub mod M7_MASK_GPIO3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio4"]
    pub mod M7_MASK_GPIO4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio5"]
    pub mod M7_MASK_GPIO5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio6"]
    pub mod M7_MASK_GPIO6 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexio1"]
    pub mod M7_MASK_FLEXIO1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexio2"]
    pub mod M7_MASK_FLEXIO2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpit1"]
    pub mod M7_MASK_LPIT1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpit2"]
    pub mod M7_MASK_LPIT2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpit3"]
    pub mod M7_MASK_LPIT3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm1"]
    pub mod M7_MASK_TPM1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm2"]
    pub mod M7_MASK_TPM2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm3"]
    pub mod M7_MASK_TPM3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm4"]
    pub mod M7_MASK_TPM4 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm5"]
    pub mod M7_MASK_TPM5 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED5_SET {
    pub use crate::RW as access;
    #[doc = "m7_mask_cm7"]
    pub mod M7_MASK_CM7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_cm33"]
    pub mod M7_MASK_CM33 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_edma3"]
    pub mod M7_MASK_EDMA3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_edma4"]
    pub mod M7_MASK_EDMA4 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_netc"]
    pub mod M7_MASK_NETC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sim_aon"]
    pub mod M7_MASK_SIM_AON {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_adc1"]
    pub mod M7_MASK_ADC1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_adc2"]
    pub mod M7_MASK_ADC2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexspi1"]
    pub mod M7_MASK_FLEXSPI1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexspi2"]
    pub mod M7_MASK_FLEXSPI2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_trdc"]
    pub mod M7_MASK_TRDC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_semc"]
    pub mod M7_MASK_SEMC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_iee"]
    pub mod M7_MASK_IEE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio1"]
    pub mod M7_MASK_GPIO1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio2"]
    pub mod M7_MASK_GPIO2 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio3"]
    pub mod M7_MASK_GPIO3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio4"]
    pub mod M7_MASK_GPIO4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio5"]
    pub mod M7_MASK_GPIO5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio6"]
    pub mod M7_MASK_GPIO6 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexio1"]
    pub mod M7_MASK_FLEXIO1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexio2"]
    pub mod M7_MASK_FLEXIO2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpit1"]
    pub mod M7_MASK_LPIT1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpit2"]
    pub mod M7_MASK_LPIT2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpit3"]
    pub mod M7_MASK_LPIT3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm1"]
    pub mod M7_MASK_TPM1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm2"]
    pub mod M7_MASK_TPM2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm3"]
    pub mod M7_MASK_TPM3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm4"]
    pub mod M7_MASK_TPM4 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm5"]
    pub mod M7_MASK_TPM5 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED5_CLR {
    pub use crate::RW as access;
    #[doc = "m7_mask_cm7"]
    pub mod M7_MASK_CM7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_cm33"]
    pub mod M7_MASK_CM33 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_edma3"]
    pub mod M7_MASK_EDMA3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_edma4"]
    pub mod M7_MASK_EDMA4 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_netc"]
    pub mod M7_MASK_NETC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sim_aon"]
    pub mod M7_MASK_SIM_AON {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_adc1"]
    pub mod M7_MASK_ADC1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_adc2"]
    pub mod M7_MASK_ADC2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexspi1"]
    pub mod M7_MASK_FLEXSPI1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexspi2"]
    pub mod M7_MASK_FLEXSPI2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_trdc"]
    pub mod M7_MASK_TRDC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_semc"]
    pub mod M7_MASK_SEMC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_iee"]
    pub mod M7_MASK_IEE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio1"]
    pub mod M7_MASK_GPIO1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio2"]
    pub mod M7_MASK_GPIO2 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio3"]
    pub mod M7_MASK_GPIO3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio4"]
    pub mod M7_MASK_GPIO4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio5"]
    pub mod M7_MASK_GPIO5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio6"]
    pub mod M7_MASK_GPIO6 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexio1"]
    pub mod M7_MASK_FLEXIO1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexio2"]
    pub mod M7_MASK_FLEXIO2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpit1"]
    pub mod M7_MASK_LPIT1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpit2"]
    pub mod M7_MASK_LPIT2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpit3"]
    pub mod M7_MASK_LPIT3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm1"]
    pub mod M7_MASK_TPM1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm2"]
    pub mod M7_MASK_TPM2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm3"]
    pub mod M7_MASK_TPM3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm4"]
    pub mod M7_MASK_TPM4 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm5"]
    pub mod M7_MASK_TPM5 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED5_TOG {
    pub use crate::RW as access;
    #[doc = "m7_mask_cm7"]
    pub mod M7_MASK_CM7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_cm33"]
    pub mod M7_MASK_CM33 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_edma3"]
    pub mod M7_MASK_EDMA3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_edma4"]
    pub mod M7_MASK_EDMA4 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_netc"]
    pub mod M7_MASK_NETC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sim_aon"]
    pub mod M7_MASK_SIM_AON {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_adc1"]
    pub mod M7_MASK_ADC1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_adc2"]
    pub mod M7_MASK_ADC2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexspi1"]
    pub mod M7_MASK_FLEXSPI1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexspi2"]
    pub mod M7_MASK_FLEXSPI2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_trdc"]
    pub mod M7_MASK_TRDC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_semc"]
    pub mod M7_MASK_SEMC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_iee"]
    pub mod M7_MASK_IEE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio1"]
    pub mod M7_MASK_GPIO1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio2"]
    pub mod M7_MASK_GPIO2 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio3"]
    pub mod M7_MASK_GPIO3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio4"]
    pub mod M7_MASK_GPIO4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio5"]
    pub mod M7_MASK_GPIO5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpio6"]
    pub mod M7_MASK_GPIO6 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexio1"]
    pub mod M7_MASK_FLEXIO1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_flexio2"]
    pub mod M7_MASK_FLEXIO2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpit1"]
    pub mod M7_MASK_LPIT1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpit2"]
    pub mod M7_MASK_LPIT2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpit3"]
    pub mod M7_MASK_LPIT3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm1"]
    pub mod M7_MASK_TPM1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm2"]
    pub mod M7_MASK_TPM2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm3"]
    pub mod M7_MASK_TPM3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm4"]
    pub mod M7_MASK_TPM4 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_tpm5"]
    pub mod M7_MASK_TPM5 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED5_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED5_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED5_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED5_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED6 {
    pub use crate::RW as access;
    #[doc = "m7_mask_tpm6"]
    pub mod M7_MASK_TPM6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpt1"]
    pub mod M7_MASK_GPT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpt2"]
    pub mod M7_MASK_GPT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_can1"]
    pub mod M7_MASK_CAN1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_can2"]
    pub mod M7_MASK_CAN2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_can3"]
    pub mod M7_MASK_CAN3 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart1"]
    pub mod M7_MASK_LPUART1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart2"]
    pub mod M7_MASK_LPUART2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart3"]
    pub mod M7_MASK_LPUART3 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart4"]
    pub mod M7_MASK_LPUART4 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart5"]
    pub mod M7_MASK_LPUART5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart6"]
    pub mod M7_MASK_LPUART6 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart7"]
    pub mod M7_MASK_LPUART7 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart8"]
    pub mod M7_MASK_LPUART8 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart9"]
    pub mod M7_MASK_LPUART9 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart10"]
    pub mod M7_MASK_LPUART10 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart11"]
    pub mod M7_MASK_LPUART11 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart12"]
    pub mod M7_MASK_LPUART12 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c1"]
    pub mod M7_MASK_LPI2C1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c2"]
    pub mod M7_MASK_LPI2C2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c3"]
    pub mod M7_MASK_LPI2C3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c4"]
    pub mod M7_MASK_LPI2C4 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c5"]
    pub mod M7_MASK_LPI2C5 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c6"]
    pub mod M7_MASK_LPI2C6 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi1"]
    pub mod M7_MASK_LPSPI1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi2"]
    pub mod M7_MASK_LPSPI2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi3"]
    pub mod M7_MASK_LPSPI3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi4"]
    pub mod M7_MASK_LPSPI4 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi5"]
    pub mod M7_MASK_LPSPI5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi6"]
    pub mod M7_MASK_LPSPI6 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sinc1"]
    pub mod M7_MASK_SINC1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sinc2"]
    pub mod M7_MASK_SINC2 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED6_SET {
    pub use crate::RW as access;
    #[doc = "m7_mask_tpm6"]
    pub mod M7_MASK_TPM6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpt1"]
    pub mod M7_MASK_GPT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpt2"]
    pub mod M7_MASK_GPT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_can1"]
    pub mod M7_MASK_CAN1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_can2"]
    pub mod M7_MASK_CAN2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_can3"]
    pub mod M7_MASK_CAN3 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart1"]
    pub mod M7_MASK_LPUART1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart2"]
    pub mod M7_MASK_LPUART2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart3"]
    pub mod M7_MASK_LPUART3 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart4"]
    pub mod M7_MASK_LPUART4 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart5"]
    pub mod M7_MASK_LPUART5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart6"]
    pub mod M7_MASK_LPUART6 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart7"]
    pub mod M7_MASK_LPUART7 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart8"]
    pub mod M7_MASK_LPUART8 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart9"]
    pub mod M7_MASK_LPUART9 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart10"]
    pub mod M7_MASK_LPUART10 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart11"]
    pub mod M7_MASK_LPUART11 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart12"]
    pub mod M7_MASK_LPUART12 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c1"]
    pub mod M7_MASK_LPI2C1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c2"]
    pub mod M7_MASK_LPI2C2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c3"]
    pub mod M7_MASK_LPI2C3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c4"]
    pub mod M7_MASK_LPI2C4 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c5"]
    pub mod M7_MASK_LPI2C5 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c6"]
    pub mod M7_MASK_LPI2C6 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi1"]
    pub mod M7_MASK_LPSPI1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi2"]
    pub mod M7_MASK_LPSPI2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi3"]
    pub mod M7_MASK_LPSPI3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi4"]
    pub mod M7_MASK_LPSPI4 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi5"]
    pub mod M7_MASK_LPSPI5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi6"]
    pub mod M7_MASK_LPSPI6 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sinc1"]
    pub mod M7_MASK_SINC1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sinc2"]
    pub mod M7_MASK_SINC2 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED6_CLR {
    pub use crate::RW as access;
    #[doc = "m7_mask_tpm6"]
    pub mod M7_MASK_TPM6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpt1"]
    pub mod M7_MASK_GPT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpt2"]
    pub mod M7_MASK_GPT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_can1"]
    pub mod M7_MASK_CAN1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_can2"]
    pub mod M7_MASK_CAN2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_can3"]
    pub mod M7_MASK_CAN3 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart1"]
    pub mod M7_MASK_LPUART1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart2"]
    pub mod M7_MASK_LPUART2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart3"]
    pub mod M7_MASK_LPUART3 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart4"]
    pub mod M7_MASK_LPUART4 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart5"]
    pub mod M7_MASK_LPUART5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart6"]
    pub mod M7_MASK_LPUART6 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart7"]
    pub mod M7_MASK_LPUART7 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart8"]
    pub mod M7_MASK_LPUART8 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart9"]
    pub mod M7_MASK_LPUART9 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart10"]
    pub mod M7_MASK_LPUART10 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart11"]
    pub mod M7_MASK_LPUART11 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart12"]
    pub mod M7_MASK_LPUART12 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c1"]
    pub mod M7_MASK_LPI2C1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c2"]
    pub mod M7_MASK_LPI2C2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c3"]
    pub mod M7_MASK_LPI2C3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c4"]
    pub mod M7_MASK_LPI2C4 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c5"]
    pub mod M7_MASK_LPI2C5 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c6"]
    pub mod M7_MASK_LPI2C6 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi1"]
    pub mod M7_MASK_LPSPI1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi2"]
    pub mod M7_MASK_LPSPI2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi3"]
    pub mod M7_MASK_LPSPI3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi4"]
    pub mod M7_MASK_LPSPI4 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi5"]
    pub mod M7_MASK_LPSPI5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi6"]
    pub mod M7_MASK_LPSPI6 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sinc1"]
    pub mod M7_MASK_SINC1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sinc2"]
    pub mod M7_MASK_SINC2 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED6_TOG {
    pub use crate::RW as access;
    #[doc = "m7_mask_tpm6"]
    pub mod M7_MASK_TPM6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpt1"]
    pub mod M7_MASK_GPT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_gpt2"]
    pub mod M7_MASK_GPT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_can1"]
    pub mod M7_MASK_CAN1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_can2"]
    pub mod M7_MASK_CAN2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_can3"]
    pub mod M7_MASK_CAN3 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart1"]
    pub mod M7_MASK_LPUART1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart2"]
    pub mod M7_MASK_LPUART2 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart3"]
    pub mod M7_MASK_LPUART3 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart4"]
    pub mod M7_MASK_LPUART4 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart5"]
    pub mod M7_MASK_LPUART5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart6"]
    pub mod M7_MASK_LPUART6 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart7"]
    pub mod M7_MASK_LPUART7 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart8"]
    pub mod M7_MASK_LPUART8 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart9"]
    pub mod M7_MASK_LPUART9 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart10"]
    pub mod M7_MASK_LPUART10 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart11"]
    pub mod M7_MASK_LPUART11 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpuart12"]
    pub mod M7_MASK_LPUART12 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c1"]
    pub mod M7_MASK_LPI2C1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c2"]
    pub mod M7_MASK_LPI2C2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c3"]
    pub mod M7_MASK_LPI2C3 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c4"]
    pub mod M7_MASK_LPI2C4 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c5"]
    pub mod M7_MASK_LPI2C5 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpi2c6"]
    pub mod M7_MASK_LPI2C6 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi1"]
    pub mod M7_MASK_LPSPI1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi2"]
    pub mod M7_MASK_LPSPI2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi3"]
    pub mod M7_MASK_LPSPI3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi4"]
    pub mod M7_MASK_LPSPI4 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi5"]
    pub mod M7_MASK_LPSPI5 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_lpspi6"]
    pub mod M7_MASK_LPSPI6 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sinc1"]
    pub mod M7_MASK_SINC1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sinc2"]
    pub mod M7_MASK_SINC2 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED6_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED6_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED6_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED6_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED7 {
    pub use crate::RW as access;
    #[doc = "m7_mask_sinc3"]
    pub mod M7_MASK_SINC3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai1"]
    pub mod M7_MASK_SAI1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai2"]
    pub mod M7_MASK_SAI2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai3"]
    pub mod M7_MASK_SAI3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai4"]
    pub mod M7_MASK_SAI4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_mic"]
    pub mod M7_MASK_MIC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED7_SET {
    pub use crate::RW as access;
    #[doc = "m7_mask_sinc3"]
    pub mod M7_MASK_SINC3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai1"]
    pub mod M7_MASK_SAI1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai2"]
    pub mod M7_MASK_SAI2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai3"]
    pub mod M7_MASK_SAI3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai4"]
    pub mod M7_MASK_SAI4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_mic"]
    pub mod M7_MASK_MIC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED7_CLR {
    pub use crate::RW as access;
    #[doc = "m7_mask_sinc3"]
    pub mod M7_MASK_SINC3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai1"]
    pub mod M7_MASK_SAI1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai2"]
    pub mod M7_MASK_SAI2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai3"]
    pub mod M7_MASK_SAI3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai4"]
    pub mod M7_MASK_SAI4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_mic"]
    pub mod M7_MASK_MIC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED7_TOG {
    pub use crate::RW as access;
    #[doc = "m7_mask_sinc3"]
    pub mod M7_MASK_SINC3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai1"]
    pub mod M7_MASK_SAI1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai2"]
    pub mod M7_MASK_SAI2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai3"]
    pub mod M7_MASK_SAI3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_sai4"]
    pub mod M7_MASK_SAI4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mask_mic"]
    pub mod M7_MASK_MIC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED7_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED7_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED7_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED7_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED8 {
    pub use crate::RW as access;
    #[doc = "m33_cm7_ipg_stop"]
    pub mod M33_CM7_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_cm33_ipg_stop"]
    pub mod M33_CM33_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_edma3_ipg_stop"]
    pub mod M33_EDMA3_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_edma4_ipg_stop"]
    pub mod M33_EDMA4_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_netc_ipg_stop"]
    pub mod M33_NETC_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sim_aon_ipg_stop"]
    pub mod M33_SIM_AON_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_adc1_ipg_stop"]
    pub mod M33_ADC1_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_adc2_ipg_stop"]
    pub mod M33_ADC2_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi1_ipg_stop"]
    pub mod M33_FLEXSPI1_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi2_ipg_stop"]
    pub mod M33_FLEXSPI2_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_trdc_ipg_stop"]
    pub mod M33_TRDC_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_semc_ipg_stop"]
    pub mod M33_SEMC_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_iee_ipg_stop"]
    pub mod M33_IEE_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio1_ipg_stop"]
    pub mod M33_GPIO1_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio2_ipg_stop"]
    pub mod M33_GPIO2_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio3_ipg_stop"]
    pub mod M33_GPIO3_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio4_ipg_stop"]
    pub mod M33_GPIO4_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio5_ipg_stop"]
    pub mod M33_GPIO5_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio6_ipg_stop"]
    pub mod M33_GPIO6_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio1_ipg_stop"]
    pub mod M33_FLEXIO1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio2_ipg_stop"]
    pub mod M33_FLEXIO2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can1_ipg_stop"]
    pub mod M33_CAN1_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can2_ipg_stop"]
    pub mod M33_CAN2_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can3_ipg_stop"]
    pub mod M33_CAN3_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart1_ipg_stop"]
    pub mod M33_LPUART1_IPG_STOP {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart2_ipg_stop"]
    pub mod M33_LPUART2_IPG_STOP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart3_ipg_stop"]
    pub mod M33_LPUART3_IPG_STOP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart4_ipg_stop"]
    pub mod M33_LPUART4_IPG_STOP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart5_ipg_stop"]
    pub mod M33_LPUART5_IPG_STOP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED8_SET {
    pub use crate::RW as access;
    #[doc = "m33_cm7_ipg_stop"]
    pub mod M33_CM7_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_cm33_ipg_stop"]
    pub mod M33_CM33_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_edma3_ipg_stop"]
    pub mod M33_EDMA3_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_edma4_ipg_stop"]
    pub mod M33_EDMA4_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_netc_ipg_stop"]
    pub mod M33_NETC_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sim_aon_ipg_stop"]
    pub mod M33_SIM_AON_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_adc1_ipg_stop"]
    pub mod M33_ADC1_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_adc2_ipg_stop"]
    pub mod M33_ADC2_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi1_ipg_stop"]
    pub mod M33_FLEXSPI1_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi2_ipg_stop"]
    pub mod M33_FLEXSPI2_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_trdc_ipg_stop"]
    pub mod M33_TRDC_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_semc_ipg_stop"]
    pub mod M33_SEMC_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_iee_ipg_stop"]
    pub mod M33_IEE_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio1_ipg_stop"]
    pub mod M33_GPIO1_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio2_ipg_stop"]
    pub mod M33_GPIO2_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio3_ipg_stop"]
    pub mod M33_GPIO3_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio4_ipg_stop"]
    pub mod M33_GPIO4_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio5_ipg_stop"]
    pub mod M33_GPIO5_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio6_ipg_stop"]
    pub mod M33_GPIO6_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio1_ipg_stop"]
    pub mod M33_FLEXIO1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio2_ipg_stop"]
    pub mod M33_FLEXIO2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can1_ipg_stop"]
    pub mod M33_CAN1_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can2_ipg_stop"]
    pub mod M33_CAN2_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can3_ipg_stop"]
    pub mod M33_CAN3_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart1_ipg_stop"]
    pub mod M33_LPUART1_IPG_STOP {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart2_ipg_stop"]
    pub mod M33_LPUART2_IPG_STOP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart3_ipg_stop"]
    pub mod M33_LPUART3_IPG_STOP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart4_ipg_stop"]
    pub mod M33_LPUART4_IPG_STOP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart5_ipg_stop"]
    pub mod M33_LPUART5_IPG_STOP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED8_CLR {
    pub use crate::RW as access;
    #[doc = "m33_cm7_ipg_stop"]
    pub mod M33_CM7_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_cm33_ipg_stop"]
    pub mod M33_CM33_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_edma3_ipg_stop"]
    pub mod M33_EDMA3_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_edma4_ipg_stop"]
    pub mod M33_EDMA4_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_netc_ipg_stop"]
    pub mod M33_NETC_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sim_aon_ipg_stop"]
    pub mod M33_SIM_AON_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_adc1_ipg_stop"]
    pub mod M33_ADC1_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_adc2_ipg_stop"]
    pub mod M33_ADC2_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi1_ipg_stop"]
    pub mod M33_FLEXSPI1_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi2_ipg_stop"]
    pub mod M33_FLEXSPI2_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_trdc_ipg_stop"]
    pub mod M33_TRDC_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_semc_ipg_stop"]
    pub mod M33_SEMC_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_iee_ipg_stop"]
    pub mod M33_IEE_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio1_ipg_stop"]
    pub mod M33_GPIO1_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio2_ipg_stop"]
    pub mod M33_GPIO2_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio3_ipg_stop"]
    pub mod M33_GPIO3_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio4_ipg_stop"]
    pub mod M33_GPIO4_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio5_ipg_stop"]
    pub mod M33_GPIO5_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio6_ipg_stop"]
    pub mod M33_GPIO6_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio1_ipg_stop"]
    pub mod M33_FLEXIO1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio2_ipg_stop"]
    pub mod M33_FLEXIO2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can1_ipg_stop"]
    pub mod M33_CAN1_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can2_ipg_stop"]
    pub mod M33_CAN2_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can3_ipg_stop"]
    pub mod M33_CAN3_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart1_ipg_stop"]
    pub mod M33_LPUART1_IPG_STOP {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart2_ipg_stop"]
    pub mod M33_LPUART2_IPG_STOP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart3_ipg_stop"]
    pub mod M33_LPUART3_IPG_STOP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart4_ipg_stop"]
    pub mod M33_LPUART4_IPG_STOP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart5_ipg_stop"]
    pub mod M33_LPUART5_IPG_STOP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED8_TOG {
    pub use crate::RW as access;
    #[doc = "m33_cm7_ipg_stop"]
    pub mod M33_CM7_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_cm33_ipg_stop"]
    pub mod M33_CM33_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_edma3_ipg_stop"]
    pub mod M33_EDMA3_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_edma4_ipg_stop"]
    pub mod M33_EDMA4_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_netc_ipg_stop"]
    pub mod M33_NETC_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sim_aon_ipg_stop"]
    pub mod M33_SIM_AON_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_adc1_ipg_stop"]
    pub mod M33_ADC1_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_adc2_ipg_stop"]
    pub mod M33_ADC2_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi1_ipg_stop"]
    pub mod M33_FLEXSPI1_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi2_ipg_stop"]
    pub mod M33_FLEXSPI2_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_trdc_ipg_stop"]
    pub mod M33_TRDC_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_semc_ipg_stop"]
    pub mod M33_SEMC_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_iee_ipg_stop"]
    pub mod M33_IEE_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio1_ipg_stop"]
    pub mod M33_GPIO1_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio2_ipg_stop"]
    pub mod M33_GPIO2_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio3_ipg_stop"]
    pub mod M33_GPIO3_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio4_ipg_stop"]
    pub mod M33_GPIO4_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio5_ipg_stop"]
    pub mod M33_GPIO5_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpio6_ipg_stop"]
    pub mod M33_GPIO6_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio1_ipg_stop"]
    pub mod M33_FLEXIO1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio2_ipg_stop"]
    pub mod M33_FLEXIO2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can1_ipg_stop"]
    pub mod M33_CAN1_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can2_ipg_stop"]
    pub mod M33_CAN2_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can3_ipg_stop"]
    pub mod M33_CAN3_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart1_ipg_stop"]
    pub mod M33_LPUART1_IPG_STOP {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart2_ipg_stop"]
    pub mod M33_LPUART2_IPG_STOP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart3_ipg_stop"]
    pub mod M33_LPUART3_IPG_STOP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart4_ipg_stop"]
    pub mod M33_LPUART4_IPG_STOP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart5_ipg_stop"]
    pub mod M33_LPUART5_IPG_STOP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED8_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED8_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED8_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED8_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED9 {
    pub use crate::RW as access;
    #[doc = "m33_lpuart6_ipg_stop"]
    pub mod M33_LPUART6_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart7_ipg_stop"]
    pub mod M33_LPUART7_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart8_ipg_stop"]
    pub mod M33_LPUART8_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart9_ipg_stop"]
    pub mod M33_LPUART9_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart10_ipg_stop"]
    pub mod M33_LPUART10_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart11_ipg_stop"]
    pub mod M33_LPUART11_IPG_STOP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart12_ipg_stop"]
    pub mod M33_LPUART12_IPG_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c1_ipg_stop"]
    pub mod M33_LPI2C1_IPG_STOP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c2_ipg_stop"]
    pub mod M33_LPI2C2_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c3_ipg_stop"]
    pub mod M33_LPI2C3_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c4_ipg_stop"]
    pub mod M33_LPI2C4_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c5_ipg_stop"]
    pub mod M33_LPI2C5_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c6_ipg_stop"]
    pub mod M33_LPI2C6_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi1_ipg_stop"]
    pub mod M33_LPSPI1_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi2_ipg_stop"]
    pub mod M33_LPSPI2_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi3_ipg_stop"]
    pub mod M33_LPSPI3_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi4_ipg_stop"]
    pub mod M33_LPSPI4_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi5_ipg_stop"]
    pub mod M33_LPSPI5_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi6_ipg_stop"]
    pub mod M33_LPSPI6_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc1_ipg_stop"]
    pub mod M33_SINC1_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc2_ipg_stop"]
    pub mod M33_SINC2_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc3_ipg_stop"]
    pub mod M33_SINC3_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai1_ipg_stop"]
    pub mod M33_SAI1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai2_ipg_stop"]
    pub mod M33_SAI2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai3_ipg_stop"]
    pub mod M33_SAI3_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai4_ipg_stop"]
    pub mod M33_SAI4_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mic_ipg_stop"]
    pub mod M33_MIC_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED9_SET {
    pub use crate::RW as access;
    #[doc = "m33_lpuart6_ipg_stop"]
    pub mod M33_LPUART6_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart7_ipg_stop"]
    pub mod M33_LPUART7_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart8_ipg_stop"]
    pub mod M33_LPUART8_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart9_ipg_stop"]
    pub mod M33_LPUART9_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart10_ipg_stop"]
    pub mod M33_LPUART10_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart11_ipg_stop"]
    pub mod M33_LPUART11_IPG_STOP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart12_ipg_stop"]
    pub mod M33_LPUART12_IPG_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c1_ipg_stop"]
    pub mod M33_LPI2C1_IPG_STOP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c2_ipg_stop"]
    pub mod M33_LPI2C2_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c3_ipg_stop"]
    pub mod M33_LPI2C3_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c4_ipg_stop"]
    pub mod M33_LPI2C4_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c5_ipg_stop"]
    pub mod M33_LPI2C5_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c6_ipg_stop"]
    pub mod M33_LPI2C6_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi1_ipg_stop"]
    pub mod M33_LPSPI1_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi2_ipg_stop"]
    pub mod M33_LPSPI2_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi3_ipg_stop"]
    pub mod M33_LPSPI3_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi4_ipg_stop"]
    pub mod M33_LPSPI4_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi5_ipg_stop"]
    pub mod M33_LPSPI5_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi6_ipg_stop"]
    pub mod M33_LPSPI6_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc1_ipg_stop"]
    pub mod M33_SINC1_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc2_ipg_stop"]
    pub mod M33_SINC2_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc3_ipg_stop"]
    pub mod M33_SINC3_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai1_ipg_stop"]
    pub mod M33_SAI1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai2_ipg_stop"]
    pub mod M33_SAI2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai3_ipg_stop"]
    pub mod M33_SAI3_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai4_ipg_stop"]
    pub mod M33_SAI4_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mic_ipg_stop"]
    pub mod M33_MIC_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED9_CLR {
    pub use crate::RW as access;
    #[doc = "m33_lpuart6_ipg_stop"]
    pub mod M33_LPUART6_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart7_ipg_stop"]
    pub mod M33_LPUART7_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart8_ipg_stop"]
    pub mod M33_LPUART8_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart9_ipg_stop"]
    pub mod M33_LPUART9_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart10_ipg_stop"]
    pub mod M33_LPUART10_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart11_ipg_stop"]
    pub mod M33_LPUART11_IPG_STOP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart12_ipg_stop"]
    pub mod M33_LPUART12_IPG_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c1_ipg_stop"]
    pub mod M33_LPI2C1_IPG_STOP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c2_ipg_stop"]
    pub mod M33_LPI2C2_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c3_ipg_stop"]
    pub mod M33_LPI2C3_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c4_ipg_stop"]
    pub mod M33_LPI2C4_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c5_ipg_stop"]
    pub mod M33_LPI2C5_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c6_ipg_stop"]
    pub mod M33_LPI2C6_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi1_ipg_stop"]
    pub mod M33_LPSPI1_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi2_ipg_stop"]
    pub mod M33_LPSPI2_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi3_ipg_stop"]
    pub mod M33_LPSPI3_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi4_ipg_stop"]
    pub mod M33_LPSPI4_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi5_ipg_stop"]
    pub mod M33_LPSPI5_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi6_ipg_stop"]
    pub mod M33_LPSPI6_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc1_ipg_stop"]
    pub mod M33_SINC1_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc2_ipg_stop"]
    pub mod M33_SINC2_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc3_ipg_stop"]
    pub mod M33_SINC3_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai1_ipg_stop"]
    pub mod M33_SAI1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai2_ipg_stop"]
    pub mod M33_SAI2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai3_ipg_stop"]
    pub mod M33_SAI3_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai4_ipg_stop"]
    pub mod M33_SAI4_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mic_ipg_stop"]
    pub mod M33_MIC_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED9_TOG {
    pub use crate::RW as access;
    #[doc = "m33_lpuart6_ipg_stop"]
    pub mod M33_LPUART6_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart7_ipg_stop"]
    pub mod M33_LPUART7_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart8_ipg_stop"]
    pub mod M33_LPUART8_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart9_ipg_stop"]
    pub mod M33_LPUART9_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart10_ipg_stop"]
    pub mod M33_LPUART10_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart11_ipg_stop"]
    pub mod M33_LPUART11_IPG_STOP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart12_ipg_stop"]
    pub mod M33_LPUART12_IPG_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c1_ipg_stop"]
    pub mod M33_LPI2C1_IPG_STOP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c2_ipg_stop"]
    pub mod M33_LPI2C2_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c3_ipg_stop"]
    pub mod M33_LPI2C3_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c4_ipg_stop"]
    pub mod M33_LPI2C4_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c5_ipg_stop"]
    pub mod M33_LPI2C5_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c6_ipg_stop"]
    pub mod M33_LPI2C6_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi1_ipg_stop"]
    pub mod M33_LPSPI1_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi2_ipg_stop"]
    pub mod M33_LPSPI2_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi3_ipg_stop"]
    pub mod M33_LPSPI3_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi4_ipg_stop"]
    pub mod M33_LPSPI4_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi5_ipg_stop"]
    pub mod M33_LPSPI5_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi6_ipg_stop"]
    pub mod M33_LPSPI6_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc1_ipg_stop"]
    pub mod M33_SINC1_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc2_ipg_stop"]
    pub mod M33_SINC2_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc3_ipg_stop"]
    pub mod M33_SINC3_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai1_ipg_stop"]
    pub mod M33_SAI1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai2_ipg_stop"]
    pub mod M33_SAI2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai3_ipg_stop"]
    pub mod M33_SAI3_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sai4_ipg_stop"]
    pub mod M33_SAI4_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mic_ipg_stop"]
    pub mod M33_MIC_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED9_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED9_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED9_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED9_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED10 {
    pub use crate::RW as access;
    #[doc = "m33_adc1_ipg_doze"]
    pub mod M33_ADC1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_adc2_ipg_doze"]
    pub mod M33_ADC2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi1_ipg_doze"]
    pub mod M33_FLEXSPI1_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi2_ipg_doze"]
    pub mod M33_FLEXSPI2_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio1_ipg_doze"]
    pub mod M33_FLEXIO1_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio2_ipg_doze"]
    pub mod M33_FLEXIO2_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpit1_ipg_doze"]
    pub mod M33_LPIT1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpit2_ipg_doze"]
    pub mod M33_LPIT2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpit3_ipg_doze"]
    pub mod M33_LPIT3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm1_ipg_doze"]
    pub mod M33_TPM1_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm2_ipg_doze"]
    pub mod M33_TPM2_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm3_ipg_doze"]
    pub mod M33_TPM3_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm4_ipg_doze"]
    pub mod M33_TPM4_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm5_ipg_doze"]
    pub mod M33_TPM5_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm6_ipg_doze"]
    pub mod M33_TPM6_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpt1_ipg_doze"]
    pub mod M33_GPT1_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpt2_ipg_doze"]
    pub mod M33_GPT2_IPG_DOZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can1_ipg_doze"]
    pub mod M33_CAN1_IPG_DOZE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can2_ipg_doze"]
    pub mod M33_CAN2_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can3_ipg_doze"]
    pub mod M33_CAN3_IPG_DOZE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart1_ipg_doze"]
    pub mod M33_LPUART1_IPG_DOZE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart2_ipg_doze"]
    pub mod M33_LPUART2_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart3_ipg_doze"]
    pub mod M33_LPUART3_IPG_DOZE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart4_ipg_doze"]
    pub mod M33_LPUART4_IPG_DOZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart5_ipg_doze"]
    pub mod M33_LPUART5_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart6_ipg_doze"]
    pub mod M33_LPUART6_IPG_DOZE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart7_ipg_doze"]
    pub mod M33_LPUART7_IPG_DOZE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart8_ipg_doze"]
    pub mod M33_LPUART8_IPG_DOZE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart9_ipg_doze"]
    pub mod M33_LPUART9_IPG_DOZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart10_ipg_doze"]
    pub mod M33_LPUART10_IPG_DOZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart11_ipg_doze"]
    pub mod M33_LPUART11_IPG_DOZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart12_ipg_doze"]
    pub mod M33_LPUART12_IPG_DOZE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED10_SET {
    pub use crate::RW as access;
    #[doc = "m33_adc1_ipg_doze"]
    pub mod M33_ADC1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_adc2_ipg_doze"]
    pub mod M33_ADC2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi1_ipg_doze"]
    pub mod M33_FLEXSPI1_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi2_ipg_doze"]
    pub mod M33_FLEXSPI2_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio1_ipg_doze"]
    pub mod M33_FLEXIO1_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio2_ipg_doze"]
    pub mod M33_FLEXIO2_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpit1_ipg_doze"]
    pub mod M33_LPIT1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpit2_ipg_doze"]
    pub mod M33_LPIT2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpit3_ipg_doze"]
    pub mod M33_LPIT3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm1_ipg_doze"]
    pub mod M33_TPM1_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm2_ipg_doze"]
    pub mod M33_TPM2_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm3_ipg_doze"]
    pub mod M33_TPM3_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm4_ipg_doze"]
    pub mod M33_TPM4_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm5_ipg_doze"]
    pub mod M33_TPM5_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm6_ipg_doze"]
    pub mod M33_TPM6_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpt1_ipg_doze"]
    pub mod M33_GPT1_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpt2_ipg_doze"]
    pub mod M33_GPT2_IPG_DOZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can1_ipg_doze"]
    pub mod M33_CAN1_IPG_DOZE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can2_ipg_doze"]
    pub mod M33_CAN2_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can3_ipg_doze"]
    pub mod M33_CAN3_IPG_DOZE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart1_ipg_doze"]
    pub mod M33_LPUART1_IPG_DOZE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart2_ipg_doze"]
    pub mod M33_LPUART2_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart3_ipg_doze"]
    pub mod M33_LPUART3_IPG_DOZE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart4_ipg_doze"]
    pub mod M33_LPUART4_IPG_DOZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart5_ipg_doze"]
    pub mod M33_LPUART5_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart6_ipg_doze"]
    pub mod M33_LPUART6_IPG_DOZE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart7_ipg_doze"]
    pub mod M33_LPUART7_IPG_DOZE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart8_ipg_doze"]
    pub mod M33_LPUART8_IPG_DOZE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart9_ipg_doze"]
    pub mod M33_LPUART9_IPG_DOZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart10_ipg_doze"]
    pub mod M33_LPUART10_IPG_DOZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart11_ipg_doze"]
    pub mod M33_LPUART11_IPG_DOZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart12_ipg_doze"]
    pub mod M33_LPUART12_IPG_DOZE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED10_CLR {
    pub use crate::RW as access;
    #[doc = "m33_adc1_ipg_doze"]
    pub mod M33_ADC1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_adc2_ipg_doze"]
    pub mod M33_ADC2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi1_ipg_doze"]
    pub mod M33_FLEXSPI1_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi2_ipg_doze"]
    pub mod M33_FLEXSPI2_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio1_ipg_doze"]
    pub mod M33_FLEXIO1_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio2_ipg_doze"]
    pub mod M33_FLEXIO2_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpit1_ipg_doze"]
    pub mod M33_LPIT1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpit2_ipg_doze"]
    pub mod M33_LPIT2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpit3_ipg_doze"]
    pub mod M33_LPIT3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm1_ipg_doze"]
    pub mod M33_TPM1_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm2_ipg_doze"]
    pub mod M33_TPM2_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm3_ipg_doze"]
    pub mod M33_TPM3_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm4_ipg_doze"]
    pub mod M33_TPM4_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm5_ipg_doze"]
    pub mod M33_TPM5_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm6_ipg_doze"]
    pub mod M33_TPM6_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpt1_ipg_doze"]
    pub mod M33_GPT1_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpt2_ipg_doze"]
    pub mod M33_GPT2_IPG_DOZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can1_ipg_doze"]
    pub mod M33_CAN1_IPG_DOZE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can2_ipg_doze"]
    pub mod M33_CAN2_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can3_ipg_doze"]
    pub mod M33_CAN3_IPG_DOZE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart1_ipg_doze"]
    pub mod M33_LPUART1_IPG_DOZE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart2_ipg_doze"]
    pub mod M33_LPUART2_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart3_ipg_doze"]
    pub mod M33_LPUART3_IPG_DOZE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart4_ipg_doze"]
    pub mod M33_LPUART4_IPG_DOZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart5_ipg_doze"]
    pub mod M33_LPUART5_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart6_ipg_doze"]
    pub mod M33_LPUART6_IPG_DOZE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart7_ipg_doze"]
    pub mod M33_LPUART7_IPG_DOZE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart8_ipg_doze"]
    pub mod M33_LPUART8_IPG_DOZE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart9_ipg_doze"]
    pub mod M33_LPUART9_IPG_DOZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart10_ipg_doze"]
    pub mod M33_LPUART10_IPG_DOZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart11_ipg_doze"]
    pub mod M33_LPUART11_IPG_DOZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart12_ipg_doze"]
    pub mod M33_LPUART12_IPG_DOZE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED10_TOG {
    pub use crate::RW as access;
    #[doc = "m33_adc1_ipg_doze"]
    pub mod M33_ADC1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_adc2_ipg_doze"]
    pub mod M33_ADC2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi1_ipg_doze"]
    pub mod M33_FLEXSPI1_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexspi2_ipg_doze"]
    pub mod M33_FLEXSPI2_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio1_ipg_doze"]
    pub mod M33_FLEXIO1_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_flexio2_ipg_doze"]
    pub mod M33_FLEXIO2_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpit1_ipg_doze"]
    pub mod M33_LPIT1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpit2_ipg_doze"]
    pub mod M33_LPIT2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpit3_ipg_doze"]
    pub mod M33_LPIT3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm1_ipg_doze"]
    pub mod M33_TPM1_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm2_ipg_doze"]
    pub mod M33_TPM2_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm3_ipg_doze"]
    pub mod M33_TPM3_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm4_ipg_doze"]
    pub mod M33_TPM4_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm5_ipg_doze"]
    pub mod M33_TPM5_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_tpm6_ipg_doze"]
    pub mod M33_TPM6_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpt1_ipg_doze"]
    pub mod M33_GPT1_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_gpt2_ipg_doze"]
    pub mod M33_GPT2_IPG_DOZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can1_ipg_doze"]
    pub mod M33_CAN1_IPG_DOZE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can2_ipg_doze"]
    pub mod M33_CAN2_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_can3_ipg_doze"]
    pub mod M33_CAN3_IPG_DOZE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart1_ipg_doze"]
    pub mod M33_LPUART1_IPG_DOZE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart2_ipg_doze"]
    pub mod M33_LPUART2_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart3_ipg_doze"]
    pub mod M33_LPUART3_IPG_DOZE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart4_ipg_doze"]
    pub mod M33_LPUART4_IPG_DOZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart5_ipg_doze"]
    pub mod M33_LPUART5_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart6_ipg_doze"]
    pub mod M33_LPUART6_IPG_DOZE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart7_ipg_doze"]
    pub mod M33_LPUART7_IPG_DOZE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart8_ipg_doze"]
    pub mod M33_LPUART8_IPG_DOZE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart9_ipg_doze"]
    pub mod M33_LPUART9_IPG_DOZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart10_ipg_doze"]
    pub mod M33_LPUART10_IPG_DOZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart11_ipg_doze"]
    pub mod M33_LPUART11_IPG_DOZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpuart12_ipg_doze"]
    pub mod M33_LPUART12_IPG_DOZE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED10_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED10_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED10_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED10_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED11 {
    pub use crate::RW as access;
    #[doc = "m33_lpi2c1_ipg_doze"]
    pub mod M33_LPI2C1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c2_ipg_doze"]
    pub mod M33_LPI2C2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c3_ipg_doze"]
    pub mod M33_LPI2C3_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c4_ipg_doze"]
    pub mod M33_LPI2C4_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c5_ipg_doze"]
    pub mod M33_LPI2C5_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c6_ipg_doze"]
    pub mod M33_LPI2C6_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi1_ipg_doze"]
    pub mod M33_LPSPI1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi2_ipg_doze"]
    pub mod M33_LPSPI2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi3_ipg_doze"]
    pub mod M33_LPSPI3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi4_ipg_doze"]
    pub mod M33_LPSPI4_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi5_ipg_doze"]
    pub mod M33_LPSPI5_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi6_ipg_doze"]
    pub mod M33_LPSPI6_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc1_ipg_doze"]
    pub mod M33_SINC1_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc2_ipg_doze"]
    pub mod M33_SINC2_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc3_ipg_doze"]
    pub mod M33_SINC3_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mic_ipg_doze"]
    pub mod M33_MIC_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED11_SET {
    pub use crate::RW as access;
    #[doc = "m33_lpi2c1_ipg_doze"]
    pub mod M33_LPI2C1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c2_ipg_doze"]
    pub mod M33_LPI2C2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c3_ipg_doze"]
    pub mod M33_LPI2C3_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c4_ipg_doze"]
    pub mod M33_LPI2C4_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c5_ipg_doze"]
    pub mod M33_LPI2C5_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c6_ipg_doze"]
    pub mod M33_LPI2C6_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi1_ipg_doze"]
    pub mod M33_LPSPI1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi2_ipg_doze"]
    pub mod M33_LPSPI2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi3_ipg_doze"]
    pub mod M33_LPSPI3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi4_ipg_doze"]
    pub mod M33_LPSPI4_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi5_ipg_doze"]
    pub mod M33_LPSPI5_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi6_ipg_doze"]
    pub mod M33_LPSPI6_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc1_ipg_doze"]
    pub mod M33_SINC1_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc2_ipg_doze"]
    pub mod M33_SINC2_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc3_ipg_doze"]
    pub mod M33_SINC3_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mic_ipg_doze"]
    pub mod M33_MIC_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED11_CLR {
    pub use crate::RW as access;
    #[doc = "m33_lpi2c1_ipg_doze"]
    pub mod M33_LPI2C1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c2_ipg_doze"]
    pub mod M33_LPI2C2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c3_ipg_doze"]
    pub mod M33_LPI2C3_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c4_ipg_doze"]
    pub mod M33_LPI2C4_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c5_ipg_doze"]
    pub mod M33_LPI2C5_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c6_ipg_doze"]
    pub mod M33_LPI2C6_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi1_ipg_doze"]
    pub mod M33_LPSPI1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi2_ipg_doze"]
    pub mod M33_LPSPI2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi3_ipg_doze"]
    pub mod M33_LPSPI3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi4_ipg_doze"]
    pub mod M33_LPSPI4_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi5_ipg_doze"]
    pub mod M33_LPSPI5_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi6_ipg_doze"]
    pub mod M33_LPSPI6_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc1_ipg_doze"]
    pub mod M33_SINC1_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc2_ipg_doze"]
    pub mod M33_SINC2_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc3_ipg_doze"]
    pub mod M33_SINC3_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mic_ipg_doze"]
    pub mod M33_MIC_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED11_TOG {
    pub use crate::RW as access;
    #[doc = "m33_lpi2c1_ipg_doze"]
    pub mod M33_LPI2C1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c2_ipg_doze"]
    pub mod M33_LPI2C2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c3_ipg_doze"]
    pub mod M33_LPI2C3_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c4_ipg_doze"]
    pub mod M33_LPI2C4_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c5_ipg_doze"]
    pub mod M33_LPI2C5_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpi2c6_ipg_doze"]
    pub mod M33_LPI2C6_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi1_ipg_doze"]
    pub mod M33_LPSPI1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi2_ipg_doze"]
    pub mod M33_LPSPI2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi3_ipg_doze"]
    pub mod M33_LPSPI3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi4_ipg_doze"]
    pub mod M33_LPSPI4_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi5_ipg_doze"]
    pub mod M33_LPSPI5_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_lpspi6_ipg_doze"]
    pub mod M33_LPSPI6_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc1_ipg_doze"]
    pub mod M33_SINC1_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc2_ipg_doze"]
    pub mod M33_SINC2_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_sinc3_ipg_doze"]
    pub mod M33_SINC3_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m33_mic_ipg_doze"]
    pub mod M33_MIC_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED11_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED11_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED11_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED11_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED12 {
    pub use crate::RW as access;
    #[doc = "m7_cm7_ipg_stop"]
    pub mod M7_CM7_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_cm33_ipg_stop"]
    pub mod M7_CM33_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_edma3_ipg_stop"]
    pub mod M7_EDMA3_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_edma4_ipg_stop"]
    pub mod M7_EDMA4_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_netc_ipg_stop"]
    pub mod M7_NETC_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sim_aon_ipg_stop"]
    pub mod M7_SIM_AON_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_adc1_ipg_stop"]
    pub mod M7_ADC1_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_adc2_ipg_stop"]
    pub mod M7_ADC2_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi1_ipg_stop"]
    pub mod M7_FLEXSPI1_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi2_ipg_stop"]
    pub mod M7_FLEXSPI2_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_trdc_ipg_stop"]
    pub mod M7_TRDC_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_semc_ipg_stop"]
    pub mod M7_SEMC_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_iee_ipg_stop"]
    pub mod M7_IEE_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio1_ipg_stop"]
    pub mod M7_GPIO1_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio2_ipg_stop"]
    pub mod M7_GPIO2_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio3_ipg_stop"]
    pub mod M7_GPIO3_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio4_ipg_stop"]
    pub mod M7_GPIO4_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio5_ipg_stop"]
    pub mod M7_GPIO5_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio6_ipg_stop"]
    pub mod M7_GPIO6_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio1_ipg_stop"]
    pub mod M7_FLEXIO1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio2_ipg_stop"]
    pub mod M7_FLEXIO2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can1_ipg_stop"]
    pub mod M7_CAN1_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can2_ipg_stop"]
    pub mod M7_CAN2_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can3_ipg_stop"]
    pub mod M7_CAN3_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart1_ipg_stop"]
    pub mod M7_LPUART1_IPG_STOP {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart2_ipg_stop"]
    pub mod M7_LPUART2_IPG_STOP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart3_ipg_stop"]
    pub mod M7_LPUART3_IPG_STOP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart4_ipg_stop"]
    pub mod M7_LPUART4_IPG_STOP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart5_ipg_stop"]
    pub mod M7_LPUART5_IPG_STOP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED12_SET {
    pub use crate::RW as access;
    #[doc = "m7_cm7_ipg_stop"]
    pub mod M7_CM7_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_cm33_ipg_stop"]
    pub mod M7_CM33_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_edma3_ipg_stop"]
    pub mod M7_EDMA3_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_edma4_ipg_stop"]
    pub mod M7_EDMA4_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_netc_ipg_stop"]
    pub mod M7_NETC_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sim_aon_ipg_stop"]
    pub mod M7_SIM_AON_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_adc1_ipg_stop"]
    pub mod M7_ADC1_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_adc2_ipg_stop"]
    pub mod M7_ADC2_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi1_ipg_stop"]
    pub mod M7_FLEXSPI1_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi2_ipg_stop"]
    pub mod M7_FLEXSPI2_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_trdc_ipg_stop"]
    pub mod M7_TRDC_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_semc_ipg_stop"]
    pub mod M7_SEMC_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_iee_ipg_stop"]
    pub mod M7_IEE_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio1_ipg_stop"]
    pub mod M7_GPIO1_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio2_ipg_stop"]
    pub mod M7_GPIO2_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio3_ipg_stop"]
    pub mod M7_GPIO3_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio4_ipg_stop"]
    pub mod M7_GPIO4_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio5_ipg_stop"]
    pub mod M7_GPIO5_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio6_ipg_stop"]
    pub mod M7_GPIO6_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio1_ipg_stop"]
    pub mod M7_FLEXIO1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio2_ipg_stop"]
    pub mod M7_FLEXIO2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can1_ipg_stop"]
    pub mod M7_CAN1_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can2_ipg_stop"]
    pub mod M7_CAN2_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can3_ipg_stop"]
    pub mod M7_CAN3_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart1_ipg_stop"]
    pub mod M7_LPUART1_IPG_STOP {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart2_ipg_stop"]
    pub mod M7_LPUART2_IPG_STOP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart3_ipg_stop"]
    pub mod M7_LPUART3_IPG_STOP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart4_ipg_stop"]
    pub mod M7_LPUART4_IPG_STOP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart5_ipg_stop"]
    pub mod M7_LPUART5_IPG_STOP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED12_CLR {
    pub use crate::RW as access;
    #[doc = "m7_cm7_ipg_stop"]
    pub mod M7_CM7_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_cm33_ipg_stop"]
    pub mod M7_CM33_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_edma3_ipg_stop"]
    pub mod M7_EDMA3_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_edma4_ipg_stop"]
    pub mod M7_EDMA4_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_netc_ipg_stop"]
    pub mod M7_NETC_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sim_aon_ipg_stop"]
    pub mod M7_SIM_AON_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_adc1_ipg_stop"]
    pub mod M7_ADC1_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_adc2_ipg_stop"]
    pub mod M7_ADC2_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi1_ipg_stop"]
    pub mod M7_FLEXSPI1_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi2_ipg_stop"]
    pub mod M7_FLEXSPI2_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_trdc_ipg_stop"]
    pub mod M7_TRDC_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_semc_ipg_stop"]
    pub mod M7_SEMC_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_iee_ipg_stop"]
    pub mod M7_IEE_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio1_ipg_stop"]
    pub mod M7_GPIO1_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio2_ipg_stop"]
    pub mod M7_GPIO2_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio3_ipg_stop"]
    pub mod M7_GPIO3_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio4_ipg_stop"]
    pub mod M7_GPIO4_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio5_ipg_stop"]
    pub mod M7_GPIO5_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio6_ipg_stop"]
    pub mod M7_GPIO6_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio1_ipg_stop"]
    pub mod M7_FLEXIO1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio2_ipg_stop"]
    pub mod M7_FLEXIO2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can1_ipg_stop"]
    pub mod M7_CAN1_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can2_ipg_stop"]
    pub mod M7_CAN2_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can3_ipg_stop"]
    pub mod M7_CAN3_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart1_ipg_stop"]
    pub mod M7_LPUART1_IPG_STOP {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart2_ipg_stop"]
    pub mod M7_LPUART2_IPG_STOP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart3_ipg_stop"]
    pub mod M7_LPUART3_IPG_STOP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart4_ipg_stop"]
    pub mod M7_LPUART4_IPG_STOP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart5_ipg_stop"]
    pub mod M7_LPUART5_IPG_STOP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED12_TOG {
    pub use crate::RW as access;
    #[doc = "m7_cm7_ipg_stop"]
    pub mod M7_CM7_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_cm33_ipg_stop"]
    pub mod M7_CM33_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_edma3_ipg_stop"]
    pub mod M7_EDMA3_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_edma4_ipg_stop"]
    pub mod M7_EDMA4_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_netc_ipg_stop"]
    pub mod M7_NETC_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sim_aon_ipg_stop"]
    pub mod M7_SIM_AON_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_adc1_ipg_stop"]
    pub mod M7_ADC1_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_adc2_ipg_stop"]
    pub mod M7_ADC2_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi1_ipg_stop"]
    pub mod M7_FLEXSPI1_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi2_ipg_stop"]
    pub mod M7_FLEXSPI2_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_trdc_ipg_stop"]
    pub mod M7_TRDC_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_semc_ipg_stop"]
    pub mod M7_SEMC_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_iee_ipg_stop"]
    pub mod M7_IEE_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio1_ipg_stop"]
    pub mod M7_GPIO1_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio2_ipg_stop"]
    pub mod M7_GPIO2_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio3_ipg_stop"]
    pub mod M7_GPIO3_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio4_ipg_stop"]
    pub mod M7_GPIO4_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio5_ipg_stop"]
    pub mod M7_GPIO5_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpio6_ipg_stop"]
    pub mod M7_GPIO6_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio1_ipg_stop"]
    pub mod M7_FLEXIO1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio2_ipg_stop"]
    pub mod M7_FLEXIO2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can1_ipg_stop"]
    pub mod M7_CAN1_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can2_ipg_stop"]
    pub mod M7_CAN2_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can3_ipg_stop"]
    pub mod M7_CAN3_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart1_ipg_stop"]
    pub mod M7_LPUART1_IPG_STOP {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart2_ipg_stop"]
    pub mod M7_LPUART2_IPG_STOP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart3_ipg_stop"]
    pub mod M7_LPUART3_IPG_STOP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart4_ipg_stop"]
    pub mod M7_LPUART4_IPG_STOP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart5_ipg_stop"]
    pub mod M7_LPUART5_IPG_STOP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED12_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED12_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED12_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED12_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED13 {
    pub use crate::RW as access;
    #[doc = "m7_lpuart6_ipg_stop"]
    pub mod M7_LPUART6_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart7_ipg_stop"]
    pub mod M7_LPUART7_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart8_ipg_stop"]
    pub mod M7_LPUART8_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart9_ipg_stop"]
    pub mod M7_LPUART9_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart10_ipg_stop"]
    pub mod M7_LPUART10_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart11_ipg_stop"]
    pub mod M7_LPUART11_IPG_STOP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart12_ipg_stop"]
    pub mod M7_LPUART12_IPG_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c1_ipg_stop"]
    pub mod M7_LPI2C1_IPG_STOP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c2_ipg_stop"]
    pub mod M7_LPI2C2_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c3_ipg_stop"]
    pub mod M7_LPI2C3_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c4_ipg_stop"]
    pub mod M7_LPI2C4_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c5_ipg_stop"]
    pub mod M7_LPI2C5_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c6_ipg_stop"]
    pub mod M7_LPI2C6_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi1_ipg_stop"]
    pub mod M7_LPSPI1_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi2_ipg_stop"]
    pub mod M7_LPSPI2_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi3_ipg_stop"]
    pub mod M7_LPSPI3_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi4_ipg_stop"]
    pub mod M7_LPSPI4_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi5_ipg_stop"]
    pub mod M7_LPSPI5_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi6_ipg_stop"]
    pub mod M7_LPSPI6_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc1_ipg_stop"]
    pub mod M7_SINC1_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc2_ipg_stop"]
    pub mod M7_SINC2_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc3_ipg_stop"]
    pub mod M7_SINC3_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai1_ipg_stop"]
    pub mod M7_SAI1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai2_ipg_stop"]
    pub mod M7_SAI2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai3_ipg_stop"]
    pub mod M7_SAI3_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai4_ipg_stop"]
    pub mod M7_SAI4_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mic_ipg_stop"]
    pub mod M7_MIC_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED13_SET {
    pub use crate::RW as access;
    #[doc = "m7_lpuart6_ipg_stop"]
    pub mod M7_LPUART6_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart7_ipg_stop"]
    pub mod M7_LPUART7_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart8_ipg_stop"]
    pub mod M7_LPUART8_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart9_ipg_stop"]
    pub mod M7_LPUART9_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart10_ipg_stop"]
    pub mod M7_LPUART10_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart11_ipg_stop"]
    pub mod M7_LPUART11_IPG_STOP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart12_ipg_stop"]
    pub mod M7_LPUART12_IPG_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c1_ipg_stop"]
    pub mod M7_LPI2C1_IPG_STOP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c2_ipg_stop"]
    pub mod M7_LPI2C2_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c3_ipg_stop"]
    pub mod M7_LPI2C3_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c4_ipg_stop"]
    pub mod M7_LPI2C4_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c5_ipg_stop"]
    pub mod M7_LPI2C5_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c6_ipg_stop"]
    pub mod M7_LPI2C6_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi1_ipg_stop"]
    pub mod M7_LPSPI1_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi2_ipg_stop"]
    pub mod M7_LPSPI2_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi3_ipg_stop"]
    pub mod M7_LPSPI3_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi4_ipg_stop"]
    pub mod M7_LPSPI4_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi5_ipg_stop"]
    pub mod M7_LPSPI5_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi6_ipg_stop"]
    pub mod M7_LPSPI6_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc1_ipg_stop"]
    pub mod M7_SINC1_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc2_ipg_stop"]
    pub mod M7_SINC2_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc3_ipg_stop"]
    pub mod M7_SINC3_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai1_ipg_stop"]
    pub mod M7_SAI1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai2_ipg_stop"]
    pub mod M7_SAI2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai3_ipg_stop"]
    pub mod M7_SAI3_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai4_ipg_stop"]
    pub mod M7_SAI4_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mic_ipg_stop"]
    pub mod M7_MIC_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED13_CLR {
    pub use crate::RW as access;
    #[doc = "m7_lpuart6_ipg_stop"]
    pub mod M7_LPUART6_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart7_ipg_stop"]
    pub mod M7_LPUART7_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart8_ipg_stop"]
    pub mod M7_LPUART8_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart9_ipg_stop"]
    pub mod M7_LPUART9_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart10_ipg_stop"]
    pub mod M7_LPUART10_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart11_ipg_stop"]
    pub mod M7_LPUART11_IPG_STOP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart12_ipg_stop"]
    pub mod M7_LPUART12_IPG_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c1_ipg_stop"]
    pub mod M7_LPI2C1_IPG_STOP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c2_ipg_stop"]
    pub mod M7_LPI2C2_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c3_ipg_stop"]
    pub mod M7_LPI2C3_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c4_ipg_stop"]
    pub mod M7_LPI2C4_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c5_ipg_stop"]
    pub mod M7_LPI2C5_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c6_ipg_stop"]
    pub mod M7_LPI2C6_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi1_ipg_stop"]
    pub mod M7_LPSPI1_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi2_ipg_stop"]
    pub mod M7_LPSPI2_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi3_ipg_stop"]
    pub mod M7_LPSPI3_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi4_ipg_stop"]
    pub mod M7_LPSPI4_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi5_ipg_stop"]
    pub mod M7_LPSPI5_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi6_ipg_stop"]
    pub mod M7_LPSPI6_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc1_ipg_stop"]
    pub mod M7_SINC1_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc2_ipg_stop"]
    pub mod M7_SINC2_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc3_ipg_stop"]
    pub mod M7_SINC3_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai1_ipg_stop"]
    pub mod M7_SAI1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai2_ipg_stop"]
    pub mod M7_SAI2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai3_ipg_stop"]
    pub mod M7_SAI3_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai4_ipg_stop"]
    pub mod M7_SAI4_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mic_ipg_stop"]
    pub mod M7_MIC_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED13_TOG {
    pub use crate::RW as access;
    #[doc = "m7_lpuart6_ipg_stop"]
    pub mod M7_LPUART6_IPG_STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart7_ipg_stop"]
    pub mod M7_LPUART7_IPG_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart8_ipg_stop"]
    pub mod M7_LPUART8_IPG_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart9_ipg_stop"]
    pub mod M7_LPUART9_IPG_STOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart10_ipg_stop"]
    pub mod M7_LPUART10_IPG_STOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart11_ipg_stop"]
    pub mod M7_LPUART11_IPG_STOP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart12_ipg_stop"]
    pub mod M7_LPUART12_IPG_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c1_ipg_stop"]
    pub mod M7_LPI2C1_IPG_STOP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c2_ipg_stop"]
    pub mod M7_LPI2C2_IPG_STOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c3_ipg_stop"]
    pub mod M7_LPI2C3_IPG_STOP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c4_ipg_stop"]
    pub mod M7_LPI2C4_IPG_STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c5_ipg_stop"]
    pub mod M7_LPI2C5_IPG_STOP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c6_ipg_stop"]
    pub mod M7_LPI2C6_IPG_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi1_ipg_stop"]
    pub mod M7_LPSPI1_IPG_STOP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi2_ipg_stop"]
    pub mod M7_LPSPI2_IPG_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi3_ipg_stop"]
    pub mod M7_LPSPI3_IPG_STOP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi4_ipg_stop"]
    pub mod M7_LPSPI4_IPG_STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi5_ipg_stop"]
    pub mod M7_LPSPI5_IPG_STOP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi6_ipg_stop"]
    pub mod M7_LPSPI6_IPG_STOP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc1_ipg_stop"]
    pub mod M7_SINC1_IPG_STOP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc2_ipg_stop"]
    pub mod M7_SINC2_IPG_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc3_ipg_stop"]
    pub mod M7_SINC3_IPG_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai1_ipg_stop"]
    pub mod M7_SAI1_IPG_STOP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai2_ipg_stop"]
    pub mod M7_SAI2_IPG_STOP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai3_ipg_stop"]
    pub mod M7_SAI3_IPG_STOP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sai4_ipg_stop"]
    pub mod M7_SAI4_IPG_STOP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mic_ipg_stop"]
    pub mod M7_MIC_IPG_STOP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED13_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED13_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED13_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED13_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED14 {
    pub use crate::RW as access;
    #[doc = "m7_adc1_ipg_doze"]
    pub mod M7_ADC1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_adc2_ipg_doze"]
    pub mod M7_ADC2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi1_ipg_doze"]
    pub mod M7_FLEXSPI1_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi2_ipg_doze"]
    pub mod M7_FLEXSPI2_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio1_ipg_doze"]
    pub mod M7_FLEXIO1_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio2_ipg_doze"]
    pub mod M7_FLEXIO2_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpit1_ipg_doze"]
    pub mod M7_LPIT1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpit2_ipg_doze"]
    pub mod M7_LPIT2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpit3_ipg_doze"]
    pub mod M7_LPIT3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm1_ipg_doze"]
    pub mod M7_TPM1_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm2_ipg_doze"]
    pub mod M7_TPM2_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm3_ipg_doze"]
    pub mod M7_TPM3_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm4_ipg_doze"]
    pub mod M7_TPM4_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm5_ipg_doze"]
    pub mod M7_TPM5_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm6_ipg_doze"]
    pub mod M7_TPM6_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpt1_ipg_doze"]
    pub mod M7_GPT1_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpt2_ipg_doze"]
    pub mod M7_GPT2_IPG_DOZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can1_ipg_doze"]
    pub mod M7_CAN1_IPG_DOZE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can2_ipg_doze"]
    pub mod M7_CAN2_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can3_ipg_doze"]
    pub mod M7_CAN3_IPG_DOZE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart1_ipg_doze"]
    pub mod M7_LPUART1_IPG_DOZE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart2_ipg_doze"]
    pub mod M7_LPUART2_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart3_ipg_doze"]
    pub mod M7_LPUART3_IPG_DOZE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart4_ipg_doze"]
    pub mod M7_LPUART4_IPG_DOZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart5_ipg_doze"]
    pub mod M7_LPUART5_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart6_ipg_doze"]
    pub mod M7_LPUART6_IPG_DOZE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart7_ipg_doze"]
    pub mod M7_LPUART7_IPG_DOZE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart8_ipg_doze"]
    pub mod M7_LPUART8_IPG_DOZE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart9_ipg_doze"]
    pub mod M7_LPUART9_IPG_DOZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart10_ipg_doze"]
    pub mod M7_LPUART10_IPG_DOZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart11_ipg_doze"]
    pub mod M7_LPUART11_IPG_DOZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart12_ipg_doze"]
    pub mod M7_LPUART12_IPG_DOZE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED14_SET {
    pub use crate::RW as access;
    #[doc = "m7_adc1_ipg_doze"]
    pub mod M7_ADC1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_adc2_ipg_doze"]
    pub mod M7_ADC2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi1_ipg_doze"]
    pub mod M7_FLEXSPI1_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi2_ipg_doze"]
    pub mod M7_FLEXSPI2_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio1_ipg_doze"]
    pub mod M7_FLEXIO1_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio2_ipg_doze"]
    pub mod M7_FLEXIO2_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpit1_ipg_doze"]
    pub mod M7_LPIT1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpit2_ipg_doze"]
    pub mod M7_LPIT2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpit3_ipg_doze"]
    pub mod M7_LPIT3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm1_ipg_doze"]
    pub mod M7_TPM1_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm2_ipg_doze"]
    pub mod M7_TPM2_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm3_ipg_doze"]
    pub mod M7_TPM3_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm4_ipg_doze"]
    pub mod M7_TPM4_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm5_ipg_doze"]
    pub mod M7_TPM5_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm6_ipg_doze"]
    pub mod M7_TPM6_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpt1_ipg_doze"]
    pub mod M7_GPT1_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpt2_ipg_doze"]
    pub mod M7_GPT2_IPG_DOZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can1_ipg_doze"]
    pub mod M7_CAN1_IPG_DOZE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can2_ipg_doze"]
    pub mod M7_CAN2_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can3_ipg_doze"]
    pub mod M7_CAN3_IPG_DOZE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart1_ipg_doze"]
    pub mod M7_LPUART1_IPG_DOZE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart2_ipg_doze"]
    pub mod M7_LPUART2_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart3_ipg_doze"]
    pub mod M7_LPUART3_IPG_DOZE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart4_ipg_doze"]
    pub mod M7_LPUART4_IPG_DOZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart5_ipg_doze"]
    pub mod M7_LPUART5_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart6_ipg_doze"]
    pub mod M7_LPUART6_IPG_DOZE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart7_ipg_doze"]
    pub mod M7_LPUART7_IPG_DOZE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart8_ipg_doze"]
    pub mod M7_LPUART8_IPG_DOZE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart9_ipg_doze"]
    pub mod M7_LPUART9_IPG_DOZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart10_ipg_doze"]
    pub mod M7_LPUART10_IPG_DOZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart11_ipg_doze"]
    pub mod M7_LPUART11_IPG_DOZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart12_ipg_doze"]
    pub mod M7_LPUART12_IPG_DOZE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED14_CLR {
    pub use crate::RW as access;
    #[doc = "m7_adc1_ipg_doze"]
    pub mod M7_ADC1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_adc2_ipg_doze"]
    pub mod M7_ADC2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi1_ipg_doze"]
    pub mod M7_FLEXSPI1_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi2_ipg_doze"]
    pub mod M7_FLEXSPI2_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio1_ipg_doze"]
    pub mod M7_FLEXIO1_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio2_ipg_doze"]
    pub mod M7_FLEXIO2_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpit1_ipg_doze"]
    pub mod M7_LPIT1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpit2_ipg_doze"]
    pub mod M7_LPIT2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpit3_ipg_doze"]
    pub mod M7_LPIT3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm1_ipg_doze"]
    pub mod M7_TPM1_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm2_ipg_doze"]
    pub mod M7_TPM2_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm3_ipg_doze"]
    pub mod M7_TPM3_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm4_ipg_doze"]
    pub mod M7_TPM4_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm5_ipg_doze"]
    pub mod M7_TPM5_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm6_ipg_doze"]
    pub mod M7_TPM6_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpt1_ipg_doze"]
    pub mod M7_GPT1_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpt2_ipg_doze"]
    pub mod M7_GPT2_IPG_DOZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can1_ipg_doze"]
    pub mod M7_CAN1_IPG_DOZE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can2_ipg_doze"]
    pub mod M7_CAN2_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can3_ipg_doze"]
    pub mod M7_CAN3_IPG_DOZE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart1_ipg_doze"]
    pub mod M7_LPUART1_IPG_DOZE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart2_ipg_doze"]
    pub mod M7_LPUART2_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart3_ipg_doze"]
    pub mod M7_LPUART3_IPG_DOZE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart4_ipg_doze"]
    pub mod M7_LPUART4_IPG_DOZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart5_ipg_doze"]
    pub mod M7_LPUART5_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart6_ipg_doze"]
    pub mod M7_LPUART6_IPG_DOZE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart7_ipg_doze"]
    pub mod M7_LPUART7_IPG_DOZE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart8_ipg_doze"]
    pub mod M7_LPUART8_IPG_DOZE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart9_ipg_doze"]
    pub mod M7_LPUART9_IPG_DOZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart10_ipg_doze"]
    pub mod M7_LPUART10_IPG_DOZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart11_ipg_doze"]
    pub mod M7_LPUART11_IPG_DOZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart12_ipg_doze"]
    pub mod M7_LPUART12_IPG_DOZE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED14_TOG {
    pub use crate::RW as access;
    #[doc = "m7_adc1_ipg_doze"]
    pub mod M7_ADC1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_adc2_ipg_doze"]
    pub mod M7_ADC2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi1_ipg_doze"]
    pub mod M7_FLEXSPI1_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexspi2_ipg_doze"]
    pub mod M7_FLEXSPI2_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio1_ipg_doze"]
    pub mod M7_FLEXIO1_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_flexio2_ipg_doze"]
    pub mod M7_FLEXIO2_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpit1_ipg_doze"]
    pub mod M7_LPIT1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpit2_ipg_doze"]
    pub mod M7_LPIT2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpit3_ipg_doze"]
    pub mod M7_LPIT3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm1_ipg_doze"]
    pub mod M7_TPM1_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm2_ipg_doze"]
    pub mod M7_TPM2_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm3_ipg_doze"]
    pub mod M7_TPM3_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm4_ipg_doze"]
    pub mod M7_TPM4_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm5_ipg_doze"]
    pub mod M7_TPM5_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_tpm6_ipg_doze"]
    pub mod M7_TPM6_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpt1_ipg_doze"]
    pub mod M7_GPT1_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_gpt2_ipg_doze"]
    pub mod M7_GPT2_IPG_DOZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can1_ipg_doze"]
    pub mod M7_CAN1_IPG_DOZE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can2_ipg_doze"]
    pub mod M7_CAN2_IPG_DOZE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_can3_ipg_doze"]
    pub mod M7_CAN3_IPG_DOZE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart1_ipg_doze"]
    pub mod M7_LPUART1_IPG_DOZE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart2_ipg_doze"]
    pub mod M7_LPUART2_IPG_DOZE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart3_ipg_doze"]
    pub mod M7_LPUART3_IPG_DOZE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart4_ipg_doze"]
    pub mod M7_LPUART4_IPG_DOZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart5_ipg_doze"]
    pub mod M7_LPUART5_IPG_DOZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart6_ipg_doze"]
    pub mod M7_LPUART6_IPG_DOZE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart7_ipg_doze"]
    pub mod M7_LPUART7_IPG_DOZE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart8_ipg_doze"]
    pub mod M7_LPUART8_IPG_DOZE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart9_ipg_doze"]
    pub mod M7_LPUART9_IPG_DOZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart10_ipg_doze"]
    pub mod M7_LPUART10_IPG_DOZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart11_ipg_doze"]
    pub mod M7_LPUART11_IPG_DOZE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpuart12_ipg_doze"]
    pub mod M7_LPUART12_IPG_DOZE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED14_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED14_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED14_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED14_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED15 {
    pub use crate::RW as access;
    #[doc = "m7_lpi2c1_ipg_doze"]
    pub mod M7_LPI2C1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c2_ipg_doze"]
    pub mod M7_LPI2C2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c3_ipg_doze"]
    pub mod M7_LPI2C3_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c4_ipg_doze"]
    pub mod M7_LPI2C4_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c5_ipg_doze"]
    pub mod M7_LPI2C5_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c6_ipg_doze"]
    pub mod M7_LPI2C6_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi1_ipg_doze"]
    pub mod M7_LPSPI1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi2_ipg_doze"]
    pub mod M7_LPSPI2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi3_ipg_doze"]
    pub mod M7_LPSPI3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi4_ipg_doze"]
    pub mod M7_LPSPI4_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi5_ipg_doze"]
    pub mod M7_LPSPI5_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi6_ipg_doze"]
    pub mod M7_LPSPI6_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc1_ipg_doze"]
    pub mod M7_SINC1_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc2_ipg_doze"]
    pub mod M7_SINC2_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc3_ipg_doze"]
    pub mod M7_SINC3_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mic_ipg_doze"]
    pub mod M7_MIC_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED15_SET {
    pub use crate::RW as access;
    #[doc = "m7_lpi2c1_ipg_doze"]
    pub mod M7_LPI2C1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c2_ipg_doze"]
    pub mod M7_LPI2C2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c3_ipg_doze"]
    pub mod M7_LPI2C3_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c4_ipg_doze"]
    pub mod M7_LPI2C4_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c5_ipg_doze"]
    pub mod M7_LPI2C5_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c6_ipg_doze"]
    pub mod M7_LPI2C6_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi1_ipg_doze"]
    pub mod M7_LPSPI1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi2_ipg_doze"]
    pub mod M7_LPSPI2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi3_ipg_doze"]
    pub mod M7_LPSPI3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi4_ipg_doze"]
    pub mod M7_LPSPI4_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi5_ipg_doze"]
    pub mod M7_LPSPI5_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi6_ipg_doze"]
    pub mod M7_LPSPI6_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc1_ipg_doze"]
    pub mod M7_SINC1_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc2_ipg_doze"]
    pub mod M7_SINC2_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc3_ipg_doze"]
    pub mod M7_SINC3_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mic_ipg_doze"]
    pub mod M7_MIC_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED15_CLR {
    pub use crate::RW as access;
    #[doc = "m7_lpi2c1_ipg_doze"]
    pub mod M7_LPI2C1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c2_ipg_doze"]
    pub mod M7_LPI2C2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c3_ipg_doze"]
    pub mod M7_LPI2C3_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c4_ipg_doze"]
    pub mod M7_LPI2C4_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c5_ipg_doze"]
    pub mod M7_LPI2C5_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c6_ipg_doze"]
    pub mod M7_LPI2C6_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi1_ipg_doze"]
    pub mod M7_LPSPI1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi2_ipg_doze"]
    pub mod M7_LPSPI2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi3_ipg_doze"]
    pub mod M7_LPSPI3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi4_ipg_doze"]
    pub mod M7_LPSPI4_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi5_ipg_doze"]
    pub mod M7_LPSPI5_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi6_ipg_doze"]
    pub mod M7_LPSPI6_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc1_ipg_doze"]
    pub mod M7_SINC1_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc2_ipg_doze"]
    pub mod M7_SINC2_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc3_ipg_doze"]
    pub mod M7_SINC3_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mic_ipg_doze"]
    pub mod M7_MIC_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General Purpose Register"]
pub mod GPR_SHARED15_TOG {
    pub use crate::RW as access;
    #[doc = "m7_lpi2c1_ipg_doze"]
    pub mod M7_LPI2C1_IPG_DOZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c2_ipg_doze"]
    pub mod M7_LPI2C2_IPG_DOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c3_ipg_doze"]
    pub mod M7_LPI2C3_IPG_DOZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c4_ipg_doze"]
    pub mod M7_LPI2C4_IPG_DOZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c5_ipg_doze"]
    pub mod M7_LPI2C5_IPG_DOZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpi2c6_ipg_doze"]
    pub mod M7_LPI2C6_IPG_DOZE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi1_ipg_doze"]
    pub mod M7_LPSPI1_IPG_DOZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi2_ipg_doze"]
    pub mod M7_LPSPI2_IPG_DOZE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi3_ipg_doze"]
    pub mod M7_LPSPI3_IPG_DOZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi4_ipg_doze"]
    pub mod M7_LPSPI4_IPG_DOZE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi5_ipg_doze"]
    pub mod M7_LPSPI5_IPG_DOZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_lpspi6_ipg_doze"]
    pub mod M7_LPSPI6_IPG_DOZE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc1_ipg_doze"]
    pub mod M7_SINC1_IPG_DOZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc2_ipg_doze"]
    pub mod M7_SINC2_IPG_DOZE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_sinc3_ipg_doze"]
    pub mod M7_SINC3_IPG_DOZE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "m7_mic_ipg_doze"]
    pub mod M7_MIC_IPG_DOZE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED15_AUTHEN {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Registers of shared GPR slice cannot be changed in user mode."]
            pub const USR_MODE_NO: u32 = 0;
            #[doc = "Registers of shared GPR slice can be changed in user mode."]
            pub const USR_MODE_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Cannot be changed in Non-secure mode."]
            pub const NONSEC_NO: u32 = 0;
            #[doc = "Can be changed in Non-secure mode."]
            pub const NONSEC_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TrustZone settings is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "TrustZone settings is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Whitelist is not locked."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Whitelist is locked."]
            pub const LOCKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED15_AUTHEN_SET {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED15_AUTHEN_CLR {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR access control"]
pub mod GPR_SHARED15_AUTHEN_TOG {
    pub use crate::RW as access;
    #[doc = "User access permission"]
    pub mod TZ_USER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-secure access permission"]
    pub mod TZ_NS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock TrustZone settings"]
    pub mod LOCK_TZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock white list"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Whitelist settings"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General purpose status register for CM33"]
pub mod GPR_SHARED_STATUS0 {
    pub use crate::RO as access;
    #[doc = "Acknowledge indicators for low power handshake"]
    pub mod GPR_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General purpose status register for CM33"]
pub mod GPR_SHARED_STATUS1 {
    pub use crate::RO as access;
    #[doc = "Acknowledge indicators for low power handshake"]
    pub mod GPR_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General purpose status register for CM33"]
pub mod GPR_SHARED_STATUS2 {
    pub use crate::RO as access;
    #[doc = "Acknowledge indicators for low power handshake"]
    pub mod GPR_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General purpose status register for CM33"]
pub mod GPR_SHARED_STATUS3 {
    pub use crate::RO as access;
    #[doc = "Acknowledge indicators for low power handshake"]
    pub mod GPR_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General status register for CM7"]
pub mod GPR_SHARED_STATUS4 {
    pub use crate::RO as access;
    #[doc = "Acknowledge indicators for low power handshake"]
    pub mod GPR_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General purpose status register for CM7"]
pub mod GPR_SHARED_STATUS5 {
    pub use crate::RO as access;
    #[doc = "Acknowledge indicators for low power handshake"]
    pub mod GPR_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General status register for CM7"]
pub mod GPR_SHARED_STATUS6 {
    pub use crate::RO as access;
    #[doc = "Acknowledge indicators for low power handshake"]
    pub mod GPR_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General purpose status register for CM7"]
pub mod GPR_SHARED_STATUS7 {
    pub use crate::RO as access;
    #[doc = "Acknowledge indicators for low power handshake"]
    pub mod GPR_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
pub mod CLOCKROOT {
    #[doc = "Clock root section."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Clock Root Control Register"]
        pub CLOCK_ROOT_CONTROL: u32,
        #[doc = "Clock Root Control Register"]
        pub CLOCK_ROOT_CONTROL_SET: u32,
        #[doc = "Clock Root Control Register"]
        pub CLOCK_ROOT_CONTROL_CLR: u32,
        #[doc = "Clock Root Control Register"]
        pub CLOCK_ROOT_CONTROL_TOG: u32,
        _reserved0: [u8; 0x10],
        #[doc = "Clock root working status"]
        pub CLOCK_ROOT_STATUS0: u32,
        _reserved1: [u8; 0x0c],
        #[doc = "Clock root access control"]
        pub CLOCK_ROOT_AUTHEN: u32,
        _reserved2: [u8; 0x4c],
    }
    #[doc = "Clock Root Control Register"]
    pub mod CLOCK_ROOT_CONTROL {
        pub use crate::RW as access;
        #[doc = "Clock division fraction."]
        pub mod DIV {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock multiplexer."]
        pub mod MUX {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x03 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Select clock source 0"]
                pub const SOURCE0: u32 = 0;
                #[doc = "Select clock source 1"]
                pub const SOURCE1: u32 = 0x01;
                #[doc = "Select clock source 2"]
                pub const SOURCE2: u32 = 0x02;
                #[doc = "Select clock source 3"]
                pub const SOURCE3: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Shutdown clock root."]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Turn on clock"]
                pub const ON: u32 = 0;
                #[doc = "Turn off clock"]
                pub const OFF: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Clock Root Control Register"]
    pub mod CLOCK_ROOT_CONTROL_SET {
        pub use crate::RW as access;
        #[doc = "Clock division fraction."]
        pub mod DIV {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock multiplexer."]
        pub mod MUX {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Shutdown clock root."]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Clock Root Control Register"]
    pub mod CLOCK_ROOT_CONTROL_CLR {
        pub use crate::RW as access;
        #[doc = "Clock division fraction."]
        pub mod DIV {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock multiplexer."]
        pub mod MUX {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Shutdown clock root."]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Clock Root Control Register"]
    pub mod CLOCK_ROOT_CONTROL_TOG {
        pub use crate::RW as access;
        #[doc = "Clock division fraction."]
        pub mod DIV {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock multiplexer."]
        pub mod MUX {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Shutdown clock root."]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Clock root working status"]
    pub mod CLOCK_ROOT_STATUS0 {
        pub use crate::RO as access;
        #[doc = "Current clock root DIV setting"]
        pub mod DIV {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Current clock root MUX setting"]
        pub mod MUX {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Current clock root OFF setting"]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock is running"]
                pub const OFF_0: u32 = 0;
                #[doc = "Clock is disabled/off"]
                pub const OFF_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Internal updating in generation logic Indication for clock generation logic is applying new setting."]
        pub mod SLICE_BUSY {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock generation logic is not busy"]
                pub const NOT_BUSY: u32 = 0;
                #[doc = "Clock generation logic is applying the new setting"]
                pub const BUSY: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Indication for clock root internal logic is updating. This status is a combination of UPDATE_FORWARD and SLICE_BUSY."]
        pub mod CHANGING {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Status is not updating currently"]
                pub const NOT_UPDATING: u32 = 0;
                #[doc = "Clock generation logic is currently updating"]
                pub const UPDATING: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Clock root access control"]
    pub mod CLOCK_ROOT_AUTHEN {
        pub use crate::RW as access;
        #[doc = "User access permission"]
        pub mod TZ_USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock cannot be changed in user mode"]
                pub const TZ_USER_0: u32 = 0;
                #[doc = "Clock can be changed in user mode"]
                pub const TZ_USER_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access permission"]
        pub mod TZ_NS {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Cannot be changed in Non-secure mode"]
                pub const TZ_NS_0: u32 = 0;
                #[doc = "Can be changed in Non-secure mode"]
                pub const TZ_NS_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock TrustZone settings"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Trustzone setting is not locked"]
                pub const LOCK_TZ_0: u32 = 0;
                #[doc = "Trustzone setting is locked"]
                pub const LOCK_TZ_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Whitelist is not locked"]
                pub const LOCK_LIST_0: u32 = 0;
                #[doc = "Whitelist is locked"]
                pub const LOCK_LIST_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Whitelist settings"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
}
pub mod OBSERVE {
    #[doc = "Clock root section."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Observe control"]
        pub OBSERVE_CONTROL: u32,
        #[doc = "Observe control"]
        pub OBSERVE_CONTROL_SET: u32,
        #[doc = "Observe control"]
        pub OBSERVE_CONTROL_CLR: u32,
        #[doc = "Observe control"]
        pub OBSERVE_CONTROL_TOG: u32,
        _reserved0: [u8; 0x10],
        #[doc = "Observe status"]
        pub OBSERVE_STATUS: u32,
        _reserved1: [u8; 0x0c],
        #[doc = "Observe access control"]
        pub OBSERVE_AUTHEN: u32,
        #[doc = "Observe access control"]
        pub OBSERVE_AUTHEN_SET: u32,
        #[doc = "Observe access control"]
        pub OBSERVE_AUTHEN_CLR: u32,
        #[doc = "Observe access control"]
        pub OBSERVE_AUTHEN_TOG: u32,
        #[doc = "Current frequency detected"]
        pub OBSERVE_FREQUENCY_CURRENT: u32,
        #[doc = "Minimum frequency detected"]
        pub OBSERVE_FREQUENCY_MIN: u32,
        #[doc = "Maximum frequency detected"]
        pub OBSERVE_FREQUENCY_MAX: u32,
        _reserved2: [u8; 0x04],
        #[doc = "Current period time detected"]
        pub OBSERVE_PERIOD_CURRENT: u32,
        #[doc = "Minimum period time detected"]
        pub OBSERVE_PERIOD_MIN: u32,
        #[doc = "Maximum period time detected"]
        pub OBSERVE_PERIOD_MAX: u32,
        _reserved3: [u8; 0x04],
        #[doc = "Current high level time detected"]
        pub OBSERVE_HIGH_CURRENT: u32,
        #[doc = "Minimum high level time detected"]
        pub OBSERVE_HIGH_MIN: u32,
        #[doc = "Maximum high level time detected"]
        pub OBSERVE_HIGH_MAX: u32,
        _reserved4: [u8; 0x04],
        #[doc = "Current high level time detected"]
        pub OBSERVE_LOW_CURRENT: u32,
        #[doc = "Minimum high level time detected"]
        pub OBSERVE_LOW_MIN: u32,
        #[doc = "Maximum high level time detected"]
        pub OBSERVE_LOW_MAX: u32,
        _reserved5: [u8; 0x04],
    }
    #[doc = "Observe control"]
    pub mod OBSERVE_CONTROL {
        pub use crate::RW as access;
        #[doc = "Observe signal selector"]
        pub mod SELECT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01ff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Observe raw signal"]
        pub mod RAW {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Select divided signal."]
                pub const SEL_DIV: u32 = 0;
                #[doc = "Select raw signal."]
                pub const SEL_RAW: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Invert input signal phase."]
        pub mod INV {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock phase remain same."]
                pub const NO_INVERT_PHASE: u32 = 0;
                #[doc = "Invert clock phase before measurement or send to IO."]
                pub const INVERT_PHASE: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Reset observe divider"]
        pub mod RESET {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Reset deasserts"]
                pub const RST_DEASSERT: u32 = 0;
                #[doc = "Reset asserts"]
                pub const RST_ASSERT: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Division factor of the divider for observed signal"]
        pub mod DIVIDE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Turn off"]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "observe slice is on"]
                pub const OBS_SL_ON: u32 = 0;
                #[doc = "observe slice is off"]
                pub const OBS_SL_OFF: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Observe control"]
    pub mod OBSERVE_CONTROL_SET {
        pub use crate::RW as access;
        #[doc = "Observe signal selector"]
        pub mod SELECT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01ff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Observe raw signal"]
        pub mod RAW {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Invert input signal phase."]
        pub mod INV {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Reset observe divider"]
        pub mod RESET {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Division factor of the divider for observed signal"]
        pub mod DIVIDE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Turn off"]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Observe control"]
    pub mod OBSERVE_CONTROL_CLR {
        pub use crate::RW as access;
        #[doc = "Observe signal selector"]
        pub mod SELECT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01ff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Observe raw signal"]
        pub mod RAW {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Invert input signal phase."]
        pub mod INV {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Reset observe divider"]
        pub mod RESET {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Division factor of the divider for observed signal"]
        pub mod DIVIDE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Turn off"]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Observe control"]
    pub mod OBSERVE_CONTROL_TOG {
        pub use crate::RW as access;
        #[doc = "Observe signal selector"]
        pub mod SELECT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01ff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Observe raw signal"]
        pub mod RAW {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Invert input signal phase."]
        pub mod INV {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Reset observe divider"]
        pub mod RESET {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Division factor of the divider for observed signal"]
        pub mod DIVIDE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Turn off"]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Observe status"]
    pub mod OBSERVE_STATUS {
        pub use crate::RO as access;
        #[doc = "Observe signal selector"]
        pub mod SELECT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01ff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Observe raw signal"]
        pub mod RAW {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Select divided signal."]
                pub const SEL_DIV: u32 = 0;
                #[doc = "Select raw signal."]
                pub const SEL_RAW: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Invert"]
        pub mod INV {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock phase remain same."]
                pub const NO_INVERT: u32 = 0;
                #[doc = "Invert clock phase before measurement or send to IO."]
                pub const INVERT: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Reset state"]
        pub mod RESET {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Observe divider is not in reset state"]
                pub const DIV_NOT_RST: u32 = 0;
                #[doc = "Observe divider is in reset state"]
                pub const DIV_RST: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Divider for observe signal"]
        pub mod DIVIDE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Turn off slice"]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "observe slice is on"]
                pub const ON: u32 = 0;
                #[doc = "observe slice is off"]
                pub const OFF: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "frequency measurement done flag"]
        pub mod FREQ_MEASURE_DONE {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Frequency measurement is on-going or not started"]
                pub const FREQ_MEAS_ON: u32 = 0;
                #[doc = "Frequency measurement is done."]
                pub const FREQ_MEAS_DONE: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Busy"]
        pub mod BUSY {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Current observe is not busy"]
                pub const NON_BUSY: u32 = 0;
                #[doc = "Current observe is busy"]
                pub const BUSY: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Update Forward"]
        pub mod UPDATED_FORWARD {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Busy"]
        pub mod CHANGING {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Observe access control"]
    pub mod OBSERVE_AUTHEN {
        pub use crate::RW as access;
        #[doc = "User access permission"]
        pub mod TZ_USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Observe slice settings cannot be changed in user mode."]
                pub const USR_MODE_NO: u32 = 0;
                #[doc = "Observe slice settings can be changed in user mode."]
                pub const USR_MODE_YES: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access permission"]
        pub mod TZ_NS {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Cannot be changed in non-secure mode."]
                pub const NONSEC_NO: u32 = 0;
                #[doc = "Can be changed in non-secure mode."]
                pub const NONSEC_YES: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock TrustZone setting"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "TrustZone settings is not locked."]
                pub const UNLOCKED: u32 = 0;
                #[doc = "TrustZone settings is locked."]
                pub const LOCKED: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "White list is not locked."]
                pub const UNLOCKED: u32 = 0;
                #[doc = "White list is locked."]
                pub const LOCKED: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Whitelist settings"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "No domain can change."]
                pub const NO_CHANGE: u32 = 0;
                #[doc = "Domain 0 can change."]
                pub const DOM0_CHANGE: u32 = 0x01;
                #[doc = "Domain 1 can change."]
                pub const DOM1_CHANGE: u32 = 0x02;
                #[doc = "Domain 0 and domain 1 can change."]
                pub const DOM01_CHANGE: u32 = 0x03;
                #[doc = "Domain 2 can change."]
                pub const DOM2_CHANGE: u32 = 0x04;
                #[doc = "All domain can change."]
                pub const ALL_CHANGE: u32 = 0x0f;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Observe access control"]
    pub mod OBSERVE_AUTHEN_SET {
        pub use crate::RW as access;
        #[doc = "User access permission"]
        pub mod TZ_USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access permission"]
        pub mod TZ_NS {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock TrustZone setting"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Whitelist settings"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Observe access control"]
    pub mod OBSERVE_AUTHEN_CLR {
        pub use crate::RW as access;
        #[doc = "User access permission"]
        pub mod TZ_USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access permission"]
        pub mod TZ_NS {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock TrustZone setting"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Whitelist settings"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Observe access control"]
    pub mod OBSERVE_AUTHEN_TOG {
        pub use crate::RW as access;
        #[doc = "User access permission"]
        pub mod TZ_USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access permission"]
        pub mod TZ_NS {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock TrustZone setting"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Whitelist settings"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Current frequency detected"]
    pub mod OBSERVE_FREQUENCY_CURRENT {
        pub use crate::RO as access;
        #[doc = "Frequency"]
        pub mod FREQUENCY {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Minimum frequency detected"]
    pub mod OBSERVE_FREQUENCY_MIN {
        pub use crate::RO as access;
        #[doc = "Frequency"]
        pub mod FREQUENCY {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Maximum frequency detected"]
    pub mod OBSERVE_FREQUENCY_MAX {
        pub use crate::RO as access;
        #[doc = "Frequency"]
        pub mod FREQUENCY {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Current period time detected"]
    pub mod OBSERVE_PERIOD_CURRENT {
        pub use crate::RO as access;
        #[doc = "Period time"]
        pub mod PERIOD {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Minimum period time detected"]
    pub mod OBSERVE_PERIOD_MIN {
        pub use crate::RO as access;
        #[doc = "Period time"]
        pub mod PERIOD {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Maximum period time detected"]
    pub mod OBSERVE_PERIOD_MAX {
        pub use crate::RO as access;
        #[doc = "Period time"]
        pub mod PERIOD {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Current high level time detected"]
    pub mod OBSERVE_HIGH_CURRENT {
        pub use crate::RO as access;
        #[doc = "High level time"]
        pub mod HIGH {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Minimum high level time detected"]
    pub mod OBSERVE_HIGH_MIN {
        pub use crate::RO as access;
        #[doc = "High level time"]
        pub mod HIGH {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Maximum high level time detected"]
    pub mod OBSERVE_HIGH_MAX {
        pub use crate::RO as access;
        #[doc = "High level time"]
        pub mod HIGH {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Current high level time detected"]
    pub mod OBSERVE_LOW_CURRENT {
        pub use crate::RO as access;
        #[doc = "High level time"]
        pub mod LOW {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Minimum high level time detected"]
    pub mod OBSERVE_LOW_MIN {
        pub use crate::RO as access;
        #[doc = "High level time"]
        pub mod LOW {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Maximum high level time detected"]
    pub mod OBSERVE_LOW_MAX {
        pub use crate::RO as access;
        #[doc = "High level time"]
        pub mod LOW {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
}
pub mod GPRPRIVATE {
    #[doc = "General purpose register section."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "General purpose register"]
        pub GPR_PRIVATE: u32,
        #[doc = "General purpose register"]
        pub GPR_PRIVATE_SET: u32,
        #[doc = "General purpose register"]
        pub GPR_PRIVATE_CLR: u32,
        #[doc = "General purpose register"]
        pub GPR_PRIVATE_TOG: u32,
        #[doc = "GPR access control"]
        pub GPR_PRIVATE_AUTHEN: u32,
        #[doc = "GPR access control"]
        pub GPR_PRIVATE_AUTHEN_SET: u32,
        #[doc = "GPR access control"]
        pub GPR_PRIVATE_AUTHEN_CLR: u32,
        #[doc = "GPR access control"]
        pub GPR_PRIVATE_AUTHEN_TOG: u32,
    }
    #[doc = "General purpose register"]
    pub mod GPR_PRIVATE {
        pub use crate::RW as access;
        #[doc = "GP register"]
        pub mod GPR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "General purpose register"]
    pub mod GPR_PRIVATE_SET {
        pub use crate::RW as access;
        #[doc = "GP register"]
        pub mod GPR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "General purpose register"]
    pub mod GPR_PRIVATE_CLR {
        pub use crate::RW as access;
        #[doc = "GP register"]
        pub mod GPR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "General purpose register"]
    pub mod GPR_PRIVATE_TOG {
        pub use crate::RW as access;
        #[doc = "GP register"]
        pub mod GPR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "GPR access control"]
    pub mod GPR_PRIVATE_AUTHEN {
        pub use crate::RW as access;
        #[doc = "User access permission"]
        pub mod TZ_USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Registers of private GPR cannot be changed in user mode."]
                pub const USR_MODE_NO: u32 = 0;
                #[doc = "Registers of private GPR can be changed in user mode."]
                pub const USR_MODE_YES: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access permission"]
        pub mod TZ_NS {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Cannot be changed in Non-secure mode."]
                pub const NONSEC_NO: u32 = 0;
                #[doc = "Can be changed in Non-secure mode."]
                pub const NONSEC_YES: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock TrustZone settings"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "TrustZone settings is not locked."]
                pub const UNLOCKED: u32 = 0;
                #[doc = "TrustZone settings is locked."]
                pub const LOCKED: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Whitelist is not locked."]
                pub const UNLOCKED: u32 = 0;
                #[doc = "Whitelist is locked."]
                pub const LOCKED: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Whitelist settings"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "GPR access control"]
    pub mod GPR_PRIVATE_AUTHEN_SET {
        pub use crate::RW as access;
        #[doc = "User access permission"]
        pub mod TZ_USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access permission"]
        pub mod TZ_NS {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock TrustZone settings"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Whitelist settings"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "GPR access control"]
    pub mod GPR_PRIVATE_AUTHEN_CLR {
        pub use crate::RW as access;
        #[doc = "User access permission"]
        pub mod TZ_USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access permission"]
        pub mod TZ_NS {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock TrustZone settings"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Whitelist settings"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "GPR access control"]
    pub mod GPR_PRIVATE_AUTHEN_TOG {
        pub use crate::RW as access;
        #[doc = "User access permission"]
        pub mod TZ_USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access permission"]
        pub mod TZ_NS {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock TrustZone settings"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Whitelist settings"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
}
pub mod OSCPLL {
    #[doc = "Clock source section."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Clock source direct control"]
        pub OSCPLL_DIRECT: u32,
        _reserved0: [u8; 0x0c],
        #[doc = "Clock source low power mode setting"]
        pub OSCPLL_LPM0: u32,
        #[doc = "clock source low power mode setting"]
        pub OSCPLL_LPM1: u32,
        _reserved1: [u8; 0x04],
        #[doc = "LPM setting of current CPU domain"]
        pub OSCPLL_LPM_CUR: u32,
        #[doc = "Clock source working status"]
        pub OSCPLL_STATUS0: u32,
        #[doc = "Clock source domain status"]
        pub OSCPLL_STATUS1: u32,
        _reserved2: [u8; 0x08],
        #[doc = "Clock Source access control"]
        pub OSCPLL_AUTHEN: u32,
        _reserved3: [u8; 0x0c],
    }
    #[doc = "Clock source direct control"]
    pub mod OSCPLL_DIRECT {
        pub use crate::RW as access;
        #[doc = "turn on clock source"]
        pub mod ON {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "OSCPLL is OFF"]
                pub const ON_0: u32 = 0;
                #[doc = "OSCPLL is ON"]
                pub const ON_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Clock source low power mode setting"]
    pub mod OSCPLL_LPM0 {
        pub use crate::RW as access;
        #[doc = "Clock Source LPM in DOMAIN0"]
        pub mod LPM_SETTING_D0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN1"]
        pub mod LPM_SETTING_D1 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN2"]
        pub mod LPM_SETTING_D2 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN3"]
        pub mod LPM_SETTING_D3 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN4"]
        pub mod LPM_SETTING_D4 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN5"]
        pub mod LPM_SETTING_D5 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN6"]
        pub mod LPM_SETTING_D6 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN7"]
        pub mod LPM_SETTING_D7 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "clock source low power mode setting"]
    pub mod OSCPLL_LPM1 {
        pub use crate::RW as access;
        #[doc = "Clock Source LPM in DOMAIN8"]
        pub mod LPM_SETTING_D8 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN9"]
        pub mod LPM_SETTING_D9 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN10"]
        pub mod LPM_SETTING_D10 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN11"]
        pub mod LPM_SETTING_D11 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN12"]
        pub mod LPM_SETTING_D12 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN13"]
        pub mod LPM_SETTING_D13 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN14"]
        pub mod LPM_SETTING_D14 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN15"]
        pub mod LPM_SETTING_D15 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "LPM setting of current CPU domain"]
    pub mod OSCPLL_LPM_CUR {
        pub use crate::RW as access;
        #[doc = "LPM settings value for current CPU domain that is reading this register."]
        pub mod LPM_SETTING_CUR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Clock source working status"]
    pub mod OSCPLL_STATUS0 {
        pub use crate::RO as access;
        #[doc = "Clock source current state"]
        pub mod ON {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock source is OFF"]
                pub const ON_0: u32 = 0;
                #[doc = "Clock source is ON"]
                pub const ON_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock source active"]
        pub mod STATUS_EARLY {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock source is not active"]
                pub const DIS: u32 = 0;
                #[doc = "Clock source is active"]
                pub const ENABLE: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock source ready"]
        pub mod STATUS_LATE {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock source is not ready to use"]
                pub const DIS: u32 = 0;
                #[doc = "Clock source is ready to use"]
                pub const ENABLE: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "This Clock Source is being used or not."]
        pub mod IN_USE {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock source is not being used by clock roots"]
                pub const DIS: u32 = 0;
                #[doc = "Clock source is being used by clock roots"]
                pub const ENABLE: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Clock source domain status"]
    pub mod OSCPLL_STATUS1 {
        pub use crate::RO as access;
        #[doc = "Domain active"]
        pub mod DOMAIN_ACTIVE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Domain enable"]
        pub mod DOMAIN_ENABLE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Clock Source access control"]
    pub mod OSCPLL_AUTHEN {
        pub use crate::RW as access;
        #[doc = "CPULPM mode enable"]
        pub mod CPULPM_MODE {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Disable CPULPM mode."]
                pub const DISABLE: u32 = 0;
                #[doc = "Enable CPULPM mode."]
                pub const ENABLE: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Auto mode enable"]
        pub mod AUTO_CTRL {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Disable Auto mode"]
                pub const DISABLE: u32 = 0;
                #[doc = "Enable Auto mode"]
                pub const ENABLE: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "User access permission"]
        pub mod TZ_USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock cannot be changed in user mode."]
                pub const TZ_USER_0: u32 = 0;
                #[doc = "Clock can be changed in user mode."]
                pub const TZ_USER_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access permission"]
        pub mod TZ_NS {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Cannot be changed in Non-secure mode."]
                pub const TZ_NS_0: u32 = 0;
                #[doc = "Can be changed in Non-secure mode."]
                pub const TZ_NS_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock TrustZone settings"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Trustzone setting is not locked."]
                pub const LOCK_TZ_0: u32 = 0;
                #[doc = "Trustzone setting is locked."]
                pub const LOCK_TZ_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Whitelist is not locked."]
                pub const LOCK_LIST_0: u32 = 0;
                #[doc = "Whitelist is locked."]
                pub const LOCK_LIST_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Whitelist"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
}
pub mod LPCG {
    #[doc = "LPCG section."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "LPCG direct control"]
        pub LPCG_DIRECT: u32,
        _reserved0: [u8; 0x0c],
        #[doc = "Clock source low power mode setting"]
        pub LPCG_LPM0: u32,
        #[doc = "clock source low power mode setting"]
        pub LPCG_LPM1: u32,
        _reserved1: [u8; 0x04],
        #[doc = "LPM setting of current CPU domain"]
        pub LPCG_LPM_CUR: u32,
        #[doc = "LPCG working status"]
        pub LPCG_STATUS0: u32,
        #[doc = "LPCG domain status"]
        pub LPCG_STATUS1: u32,
        _reserved2: [u8; 0x08],
        #[doc = "LPCG access control"]
        pub LPCG_AUTHEN: u32,
        _reserved3: [u8; 0x0c],
    }
    #[doc = "LPCG direct control"]
    pub mod LPCG_DIRECT {
        pub use crate::RW as access;
        #[doc = "Turn on LPCG"]
        pub mod ON {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "LPCG is OFF."]
                pub const OFF: u32 = 0;
                #[doc = "LPCG is ON."]
                pub const ON: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock off handshake timeout enable"]
        pub mod CLKOFF_ACK_TIMEOUT_EN {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "disable"]
                pub const DISABLE: u32 = 0;
                #[doc = "enable"]
                pub const ENABLE: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Clock source low power mode setting"]
    pub mod LPCG_LPM0 {
        pub use crate::RW as access;
        #[doc = "Clock Source LPM in DOMAIN0"]
        pub mod LPM_SETTING_D0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN1"]
        pub mod LPM_SETTING_D1 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN2"]
        pub mod LPM_SETTING_D2 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN3"]
        pub mod LPM_SETTING_D3 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN4"]
        pub mod LPM_SETTING_D4 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN5"]
        pub mod LPM_SETTING_D5 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN6"]
        pub mod LPM_SETTING_D6 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN7"]
        pub mod LPM_SETTING_D7 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "clock source low power mode setting"]
    pub mod LPCG_LPM1 {
        pub use crate::RW as access;
        #[doc = "Clock Source LPM in DOMAIN8"]
        pub mod LPM_SETTING_D8 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN9"]
        pub mod LPM_SETTING_D9 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN10"]
        pub mod LPM_SETTING_D10 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN11"]
        pub mod LPM_SETTING_D11 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN12"]
        pub mod LPM_SETTING_D12 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN13"]
        pub mod LPM_SETTING_D13 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN14"]
        pub mod LPM_SETTING_D14 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Clock Source LPM in DOMAIN15"]
        pub mod LPM_SETTING_D15 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "LPM setting of current CPU domain"]
    pub mod LPCG_LPM_CUR {
        pub use crate::RW as access;
        #[doc = "LPM settings value for current CPU domain that is reading this register."]
        pub mod LPM_SETTING_CUR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock Source will be OFF in any CPU mode."]
                pub const CLKSRC_OFF_ALL: u32 = 0;
                #[doc = "Clock Source will be ON in RUN mode, OFF in WAIT/STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUN: u32 = 0x01;
                #[doc = "Clock Source will be ON in RUN/WAIT mode, OFF in STOP/SUSPEND mode."]
                pub const CLKSRC_ONRUNWAIT: u32 = 0x02;
                #[doc = "Clock Source will be ON in RUN/WAIT/STOP mode, OFF in SUSPEND mode."]
                pub const CLKSRC_ONRUNWAITSTOP: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "LPCG working status"]
    pub mod LPCG_STATUS0 {
        pub use crate::RO as access;
        #[doc = "LPCG work status"]
        pub mod ON {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "LPCG is OFF."]
                pub const OFF: u32 = 0;
                #[doc = "LPCG is ON."]
                pub const ON: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "LPCG domain status"]
    pub mod LPCG_STATUS1 {
        pub use crate::RO as access;
        #[doc = "Domain active"]
        pub mod DOMAIN_ACTIVE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Domain enable"]
        pub mod DOMAIN_ENABLE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "LPCG access control"]
    pub mod LPCG_AUTHEN {
        pub use crate::RW as access;
        #[doc = "CPULPM mode enable"]
        pub mod CPULPM_MODE {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Disable CPULPM mode, this LPCG is in Direct Control mode."]
                pub const DISABLE: u32 = 0;
                #[doc = "Enable CPULPM mode, this LPCG is in CPULPM mode."]
                pub const ENABLE: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "CPULPM mode enable"]
        pub mod ACK_MODE {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Disable ACK mode."]
                pub const DISABLE_ACK: u32 = 0;
                #[doc = "Enable ACK mode."]
                pub const ENABLE_ACK: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "User access permission"]
        pub mod TZ_USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "LPCG settings cannot be changed in user mode."]
                pub const USR_MODE_NO: u32 = 0;
                #[doc = "LPCG settings can be changed in user mode."]
                pub const USR_MODE_YES: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access permission"]
        pub mod TZ_NS {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Cannot be changed in Non-secure mode."]
                pub const NONSEC_NO: u32 = 0;
                #[doc = "Can be changed in Non-secure mode."]
                pub const NONSEC_YES: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock TrustZone settings"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "TrustZone settings is not locked."]
                pub const UNLOCKED: u32 = 0;
                #[doc = "TrustZone settings is locked."]
                pub const LOCKED: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Whitelist is not locked."]
                pub const UNLOCKED: u32 = 0;
                #[doc = "Whitelist is locked."]
                pub const LOCKED: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Whitelist"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
}
