#[doc = "IEE_APC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "End address of IEE region (n)"]
    pub REGION0_TOP_ADDR: u32,
    #[doc = "Start address of IEE region (n)"]
    pub REGION0_BOT_ADDR: u32,
    #[doc = "Region enable for region (n)"]
    pub REGION0_ENA: u32,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION0_ACC_CTL: u32,
    #[doc = "End address of IEE region (n)"]
    pub REGION1_TOP_ADDR: u32,
    #[doc = "Start address of IEE region (n)"]
    pub REGION1_BOT_ADDR: u32,
    #[doc = "Region enable for region (n)"]
    pub REGION1_ENA: u32,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION1_ACC_CTL: u32,
    #[doc = "End address of IEE region (n)"]
    pub REGION2_TOP_ADDR: u32,
    #[doc = "Start address of IEE region (n)"]
    pub REGION2_BOT_ADDR: u32,
    #[doc = "Region enable for region (n)"]
    pub REGION2_ENA: u32,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION2_ACC_CTL: u32,
    #[doc = "End address of IEE region (n)"]
    pub REGION3_TOP_ADDR: u32,
    #[doc = "Start address of IEE region (n)"]
    pub REGION3_BOT_ADDR: u32,
    #[doc = "Region enable for region (n)"]
    pub REGION3_ENA: u32,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION3_ACC_CTL: u32,
    #[doc = "End address of IEE region (n)"]
    pub REGION4_TOP_ADDR: u32,
    #[doc = "Start address of IEE region (n)"]
    pub REGION4_BOT_ADDR: u32,
    #[doc = "Region enable for region (n)"]
    pub REGION4_ENA: u32,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION4_ACC_CTL: u32,
    #[doc = "End address of IEE region (n)"]
    pub REGION5_TOP_ADDR: u32,
    #[doc = "Start address of IEE region (n)"]
    pub REGION5_BOT_ADDR: u32,
    #[doc = "Region enable for region (n)"]
    pub REGION5_ENA: u32,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION5_ACC_CTL: u32,
    #[doc = "End address of IEE region (n)"]
    pub REGION6_TOP_ADDR: u32,
    #[doc = "Start address of IEE region (n)"]
    pub REGION6_BOT_ADDR: u32,
    #[doc = "Region enable for region (n)"]
    pub REGION6_ENA: u32,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION6_ACC_CTL: u32,
    #[doc = "End address of IEE region (n)"]
    pub REGION7_TOP_ADDR: u32,
    #[doc = "Start address of IEE region (n)"]
    pub REGION7_BOT_ADDR: u32,
    #[doc = "Region enable for region (n)"]
    pub REGION7_ENA: u32,
    #[doc = "Access control for IEE APC registers of region (n)"]
    pub REGION7_ACC_CTL: u32,
}
#[doc = "End address of IEE region (n)"]
pub mod REGION0_TOP_ADDR {
    pub use crate::RW as access;
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION0_BOT_ADDR {
    pub use crate::RW as access;
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION0_ENA {
    pub use crate::RW as access;
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION0_ACC_CTL {
    pub use crate::RW as access;
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION1_TOP_ADDR {
    pub use crate::RW as access;
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION1_BOT_ADDR {
    pub use crate::RW as access;
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION1_ENA {
    pub use crate::RW as access;
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION1_ACC_CTL {
    pub use crate::RW as access;
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION2_TOP_ADDR {
    pub use crate::RW as access;
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION2_BOT_ADDR {
    pub use crate::RW as access;
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION2_ENA {
    pub use crate::RW as access;
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION2_ACC_CTL {
    pub use crate::RW as access;
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION3_TOP_ADDR {
    pub use crate::RW as access;
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION3_BOT_ADDR {
    pub use crate::RW as access;
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION3_ENA {
    pub use crate::RW as access;
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION3_ACC_CTL {
    pub use crate::RW as access;
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION4_TOP_ADDR {
    pub use crate::RW as access;
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION4_BOT_ADDR {
    pub use crate::RW as access;
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION4_ENA {
    pub use crate::RW as access;
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION4_ACC_CTL {
    pub use crate::RW as access;
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION5_TOP_ADDR {
    pub use crate::RW as access;
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION5_BOT_ADDR {
    pub use crate::RW as access;
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION5_ENA {
    pub use crate::RW as access;
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION5_ACC_CTL {
    pub use crate::RW as access;
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION6_TOP_ADDR {
    pub use crate::RW as access;
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION6_BOT_ADDR {
    pub use crate::RW as access;
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION6_ENA {
    pub use crate::RW as access;
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION6_ACC_CTL {
    pub use crate::RW as access;
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "End address of IEE region (n)"]
pub mod REGION7_TOP_ADDR {
    pub use crate::RW as access;
    #[doc = "End address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod TOP_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Start address of IEE region (n)"]
pub mod REGION7_BOT_ADDR {
    pub use crate::RW as access;
    #[doc = "Start address\\[31:6\\] of IEE region. The lower 6 address bits of IEE region is always 0."]
    pub mod BOT_ADDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Region enable for region (n)"]
pub mod REGION7_ENA {
    pub use crate::RW as access;
    #[doc = "Enable this region"]
    pub mod ENCRYPT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This region is not enabled for IEE routing even hit"]
            pub const DISABLE: u32 = 0;
            #[doc = "This region is enabled for IEE routing once hit"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Access control for IEE APC registers of region (n)"]
pub mod REGION7_ACC_CTL {
    pub use crate::RW as access;
    #[doc = "Allowed domain ID"]
    pub mod ALLOW_DID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the lower half word"]
    pub mod LOCK_L {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Lower half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Lower half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow nonsecure mode access"]
    pub mod ALLOW_NS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only secure access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Secure and nonsecure access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod ALLOW_USER {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Only privilege access to this region's registers is allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "User and privilege access to this region's registers is allowed"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock bit for the higher half word"]
    pub mod LOCK_H {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Higher half word is not locked"]
            pub const UNLOCK: u32 = 0;
            #[doc = "Higher half word is locked"]
            pub const LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
