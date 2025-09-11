#[doc = "SSARC Registers"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Descriptor Control0 0 Register"]
    pub DESC_CTRL0_0: u32,
    #[doc = "Descriptor Control1 0 Register"]
    pub DESC_CTRL1_0: u32,
    #[doc = "Descriptor Address Up 0 Register"]
    pub DESC_ADDR_UP_0: u32,
    #[doc = "Descriptor Address Down 0 Register"]
    pub DESC_ADDR_DOWN_0: u32,
    _reserved0: [u8; 0x10],
    #[doc = "Descriptor Control0 1 Register"]
    pub DESC_CTRL0_1: u32,
    #[doc = "Descriptor Control1 1 Register"]
    pub DESC_CTRL1_1: u32,
    #[doc = "Descriptor Address Up 1 Register"]
    pub DESC_ADDR_UP_1: u32,
    #[doc = "Descriptor Address Down 1 Register"]
    pub DESC_ADDR_DOWN_1: u32,
    _reserved1: [u8; 0x10],
    #[doc = "Descriptor Control0 2 Register"]
    pub DESC_CTRL0_2: u32,
    #[doc = "Descriptor Control1 2 Register"]
    pub DESC_CTRL1_2: u32,
    #[doc = "Descriptor Address Up 2 Register"]
    pub DESC_ADDR_UP_2: u32,
    #[doc = "Descriptor Address Down 2 Register"]
    pub DESC_ADDR_DOWN_2: u32,
    _reserved2: [u8; 0x10],
    #[doc = "Descriptor Control0 3 Register"]
    pub DESC_CTRL0_3: u32,
    #[doc = "Descriptor Control1 3 Register"]
    pub DESC_CTRL1_3: u32,
    #[doc = "Descriptor Address Up 3 Register"]
    pub DESC_ADDR_UP_3: u32,
    #[doc = "Descriptor Address Down 3 Register"]
    pub DESC_ADDR_DOWN_3: u32,
    _reserved3: [u8; 0x10],
    #[doc = "Descriptor Control0 4 Register"]
    pub DESC_CTRL0_4: u32,
    #[doc = "Descriptor Control1 4 Register"]
    pub DESC_CTRL1_4: u32,
    #[doc = "Descriptor Address Up 4 Register"]
    pub DESC_ADDR_UP_4: u32,
    #[doc = "Descriptor Address Down 4 Register"]
    pub DESC_ADDR_DOWN_4: u32,
    _reserved4: [u8; 0x10],
    #[doc = "Descriptor Control0 5 Register"]
    pub DESC_CTRL0_5: u32,
    #[doc = "Descriptor Control1 5 Register"]
    pub DESC_CTRL1_5: u32,
    #[doc = "Descriptor Address Up 5 Register"]
    pub DESC_ADDR_UP_5: u32,
    #[doc = "Descriptor Address Down 5 Register"]
    pub DESC_ADDR_DOWN_5: u32,
    _reserved5: [u8; 0x10],
    #[doc = "Descriptor Control0 6 Register"]
    pub DESC_CTRL0_6: u32,
    #[doc = "Descriptor Control1 6 Register"]
    pub DESC_CTRL1_6: u32,
    #[doc = "Descriptor Address Up 6 Register"]
    pub DESC_ADDR_UP_6: u32,
    #[doc = "Descriptor Address Down 6 Register"]
    pub DESC_ADDR_DOWN_6: u32,
    _reserved6: [u8; 0x10],
    #[doc = "Descriptor Control0 7 Register"]
    pub DESC_CTRL0_7: u32,
    #[doc = "Descriptor Control1 7 Register"]
    pub DESC_CTRL1_7: u32,
    #[doc = "Descriptor Address Up 7 Register"]
    pub DESC_ADDR_UP_7: u32,
    #[doc = "Descriptor Address Down 7 Register"]
    pub DESC_ADDR_DOWN_7: u32,
    _reserved7: [u8; 0x10],
    #[doc = "Descriptor Control0 8 Register"]
    pub DESC_CTRL0_8: u32,
    #[doc = "Descriptor Control1 8 Register"]
    pub DESC_CTRL1_8: u32,
    #[doc = "Descriptor Address Up 8 Register"]
    pub DESC_ADDR_UP_8: u32,
    #[doc = "Descriptor Address Down 8 Register"]
    pub DESC_ADDR_DOWN_8: u32,
    _reserved8: [u8; 0x10],
    #[doc = "Descriptor Control0 9 Register"]
    pub DESC_CTRL0_9: u32,
    #[doc = "Descriptor Control1 9 Register"]
    pub DESC_CTRL1_9: u32,
    #[doc = "Descriptor Address Up 9 Register"]
    pub DESC_ADDR_UP_9: u32,
    #[doc = "Descriptor Address Down 9 Register"]
    pub DESC_ADDR_DOWN_9: u32,
    _reserved9: [u8; 0x10],
    #[doc = "Descriptor Control0 10 Register"]
    pub DESC_CTRL0_10: u32,
    #[doc = "Descriptor Control1 10 Register"]
    pub DESC_CTRL1_10: u32,
    #[doc = "Descriptor Address Up 10 Register"]
    pub DESC_ADDR_UP_10: u32,
    #[doc = "Descriptor Address Down 10 Register"]
    pub DESC_ADDR_DOWN_10: u32,
    _reserved10: [u8; 0x10],
    #[doc = "Descriptor Control0 11 Register"]
    pub DESC_CTRL0_11: u32,
    #[doc = "Descriptor Control1 11 Register"]
    pub DESC_CTRL1_11: u32,
    #[doc = "Descriptor Address Up 11 Register"]
    pub DESC_ADDR_UP_11: u32,
    #[doc = "Descriptor Address Down 11 Register"]
    pub DESC_ADDR_DOWN_11: u32,
    _reserved11: [u8; 0x10],
    #[doc = "Descriptor Control0 12 Register"]
    pub DESC_CTRL0_12: u32,
    #[doc = "Descriptor Control1 12 Register"]
    pub DESC_CTRL1_12: u32,
    #[doc = "Descriptor Address Up 12 Register"]
    pub DESC_ADDR_UP_12: u32,
    #[doc = "Descriptor Address Down 12 Register"]
    pub DESC_ADDR_DOWN_12: u32,
    _reserved12: [u8; 0x10],
    #[doc = "Descriptor Control0 13 Register"]
    pub DESC_CTRL0_13: u32,
    #[doc = "Descriptor Control1 13 Register"]
    pub DESC_CTRL1_13: u32,
    #[doc = "Descriptor Address Up 13 Register"]
    pub DESC_ADDR_UP_13: u32,
    #[doc = "Descriptor Address Down 13 Register"]
    pub DESC_ADDR_DOWN_13: u32,
    _reserved13: [u8; 0x10],
    #[doc = "Descriptor Control0 14 Register"]
    pub DESC_CTRL0_14: u32,
    #[doc = "Descriptor Control1 14 Register"]
    pub DESC_CTRL1_14: u32,
    #[doc = "Descriptor Address Up 14 Register"]
    pub DESC_ADDR_UP_14: u32,
    #[doc = "Descriptor Address Down 14 Register"]
    pub DESC_ADDR_DOWN_14: u32,
    _reserved14: [u8; 0x10],
    #[doc = "Descriptor Control0 15 Register"]
    pub DESC_CTRL0_15: u32,
    #[doc = "Descriptor Control1 15 Register"]
    pub DESC_CTRL1_15: u32,
    #[doc = "Descriptor Address Up 15 Register"]
    pub DESC_ADDR_UP_15: u32,
    #[doc = "Descriptor Address Down 15 Register"]
    pub DESC_ADDR_DOWN_15: u32,
    _reserved15: [u8; 0x10],
    #[doc = "Control Register"]
    pub CTRL: u32,
    #[doc = "Interrupt Status Register"]
    pub INT_STATUS: u32,
    _reserved16: [u8; 0x04],
    #[doc = "HP Timeout Register"]
    pub HP_TIMEOUT: u32,
    _reserved17: [u8; 0x0c],
    #[doc = "Hardware Request Pending Register"]
    pub HW_GROUP_PENDING: u32,
    #[doc = "Software Request Pending Register"]
    pub SW_GROUP_PENDING: u32,
}
#[doc = "Descriptor Control0 0 Register"]
pub mod DESC_CTRL0_0 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 0 Register"]
pub mod DESC_CTRL1_0 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 0 Register"]
pub mod DESC_ADDR_UP_0 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 0 Register"]
pub mod DESC_ADDR_DOWN_0 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 1 Register"]
pub mod DESC_CTRL0_1 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 1 Register"]
pub mod DESC_CTRL1_1 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 1 Register"]
pub mod DESC_ADDR_UP_1 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 1 Register"]
pub mod DESC_ADDR_DOWN_1 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 2 Register"]
pub mod DESC_CTRL0_2 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 2 Register"]
pub mod DESC_CTRL1_2 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 2 Register"]
pub mod DESC_ADDR_UP_2 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 2 Register"]
pub mod DESC_ADDR_DOWN_2 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 3 Register"]
pub mod DESC_CTRL0_3 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 3 Register"]
pub mod DESC_CTRL1_3 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 3 Register"]
pub mod DESC_ADDR_UP_3 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 3 Register"]
pub mod DESC_ADDR_DOWN_3 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 4 Register"]
pub mod DESC_CTRL0_4 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 4 Register"]
pub mod DESC_CTRL1_4 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 4 Register"]
pub mod DESC_ADDR_UP_4 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 4 Register"]
pub mod DESC_ADDR_DOWN_4 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 5 Register"]
pub mod DESC_CTRL0_5 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 5 Register"]
pub mod DESC_CTRL1_5 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 5 Register"]
pub mod DESC_ADDR_UP_5 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 5 Register"]
pub mod DESC_ADDR_DOWN_5 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 6 Register"]
pub mod DESC_CTRL0_6 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 6 Register"]
pub mod DESC_CTRL1_6 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 6 Register"]
pub mod DESC_ADDR_UP_6 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 6 Register"]
pub mod DESC_ADDR_DOWN_6 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 7 Register"]
pub mod DESC_CTRL0_7 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 7 Register"]
pub mod DESC_CTRL1_7 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 7 Register"]
pub mod DESC_ADDR_UP_7 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 7 Register"]
pub mod DESC_ADDR_DOWN_7 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 8 Register"]
pub mod DESC_CTRL0_8 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 8 Register"]
pub mod DESC_CTRL1_8 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 8 Register"]
pub mod DESC_ADDR_UP_8 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 8 Register"]
pub mod DESC_ADDR_DOWN_8 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 9 Register"]
pub mod DESC_CTRL0_9 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 9 Register"]
pub mod DESC_CTRL1_9 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 9 Register"]
pub mod DESC_ADDR_UP_9 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 9 Register"]
pub mod DESC_ADDR_DOWN_9 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 10 Register"]
pub mod DESC_CTRL0_10 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 10 Register"]
pub mod DESC_CTRL1_10 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 10 Register"]
pub mod DESC_ADDR_UP_10 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 10 Register"]
pub mod DESC_ADDR_DOWN_10 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 11 Register"]
pub mod DESC_CTRL0_11 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 11 Register"]
pub mod DESC_CTRL1_11 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 11 Register"]
pub mod DESC_ADDR_UP_11 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 11 Register"]
pub mod DESC_ADDR_DOWN_11 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 12 Register"]
pub mod DESC_CTRL0_12 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 12 Register"]
pub mod DESC_CTRL1_12 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 12 Register"]
pub mod DESC_ADDR_UP_12 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 12 Register"]
pub mod DESC_ADDR_DOWN_12 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 13 Register"]
pub mod DESC_CTRL0_13 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 13 Register"]
pub mod DESC_CTRL1_13 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 13 Register"]
pub mod DESC_ADDR_UP_13 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 13 Register"]
pub mod DESC_ADDR_DOWN_13 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 14 Register"]
pub mod DESC_CTRL0_14 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 14 Register"]
pub mod DESC_CTRL1_14 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 14 Register"]
pub mod DESC_ADDR_UP_14 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 14 Register"]
pub mod DESC_ADDR_DOWN_14 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control0 15 Register"]
pub mod DESC_CTRL0_15 {
    pub use crate::RW as access;
    #[doc = "Start index"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "End index"]
    pub mod END {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Order"]
    pub mod SV_ORDER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const SV_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const SV_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore order"]
    pub mod RT_ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Descriptors within the group are processed from start to end"]
            pub const RT_START_END: u32 = 0;
            #[doc = "Descriptors within the group are processed from end to start"]
            pub const RT_END_START: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Control1 15 Register"]
pub mod DESC_CTRL1_15 {
    pub use crate::RW as access;
    #[doc = "Software trigger save"]
    pub mod SW_TRIG_SV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software save request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software save operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger restore"]
    pub mod SW_TRIG_RT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software restore request/software restore request complete"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request a software restore operation/software restore operation in progress"]
            pub const REQ_YES: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field describes the mapping (0-7) to external request signals from different domains"]
    pub mod POWER_DOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC_BPC0"]
            pub const DOMAIN0: u32 = 0;
            #[doc = "PGMC_BPC1"]
            pub const DOMAIN1: u32 = 0x01;
            #[doc = "PGMC_BPC2"]
            pub const DOMAIN2: u32 = 0x02;
            #[doc = "PGMC_BPC3"]
            pub const DOMAIN3: u32 = 0x03;
            #[doc = "PGMC_BPC4"]
            pub const DOMAIN4: u32 = 0x04;
            #[doc = "PGMC_BPC5"]
            pub const DOMAIN5: u32 = 0x05;
            #[doc = "PGMC_BPC6"]
            pub const DOMAIN6: u32 = 0x06;
            #[doc = "PGMC_BPC7"]
            pub const DOMAIN7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Enable"]
    pub mod GP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group disabled"]
            pub const GP_DIS: u32 = 0;
            #[doc = "Group enabled"]
            pub const GP_EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save Priority"]
    pub mod SV_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore Priority"]
    pub mod RT_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU Domain"]
    pub mod CPUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read Lock"]
    pub mod RL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (read access allowed)"]
            pub const R_UNLOCK: u32 = 0;
            #[doc = "Group is locked (read access not allowed)"]
            pub const R_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Lock"]
    pub mod WL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Group is unlocked (write access allowed)"]
            pub const W_UNLOCK: u32 = 0;
            #[doc = "Group is locked (write access not allowed)"]
            pub const W_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain lock"]
    pub mod DL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Unlock"]
            pub const D_UNLOCK: u32 = 0;
            #[doc = "Lock"]
            pub const D_LOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Up 15 Register"]
pub mod DESC_ADDR_UP_15 {
    pub use crate::RW as access;
    #[doc = "Address field (High)"]
    pub mod ADDR_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Address Down 15 Register"]
pub mod DESC_ADDR_DOWN_15 {
    pub use crate::RW as access;
    #[doc = "Address field (Low)"]
    pub mod ADDR_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Control Register"]
pub mod CTRL {
    pub use crate::RW as access;
    #[doc = "Save/Restore request disable"]
    pub mod DIS_HW_REQ {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PGMC save/restore requests enabled"]
            pub const ENABLE_PGMC: u32 = 0;
            #[doc = "PGMC save/restore requests disabled"]
            pub const DIS_PGMC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software reset"]
    pub mod SW_RESET {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Interrupt Status Register"]
pub mod INT_STATUS {
    pub use crate::RW as access;
    #[doc = "Error Index"]
    pub mod ERR_INDEX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AHB Bus response field"]
    pub mod AHB_RESP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Group Conflict field"]
    pub mod GROUP_CONFLICT {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No group conflict error"]
            pub const GRP_CONFLICT_ERR_NO: u32 = 0;
            #[doc = "A group conflict error has occurred"]
            pub const GRP_CONFLICT_ERR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timeout field"]
    pub mod TIMEOUT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No timeout event"]
            pub const ERR_INDEX_ERR_NO: u32 = 0;
            #[doc = "A timeout event has occurred"]
            pub const ERR_INDEX_ERR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software Request Done"]
    pub mod SW_REQ_DONE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No software triggered requests or software triggered request still in progress"]
            pub const SW_REQ_ERR_A: u32 = 0;
            #[doc = "Atleast one software triggered has been complete"]
            pub const SW_REQ_ERR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AHB Error field"]
    pub mod AHB_ERR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No AHB error"]
            pub const AHB_ERRNO: u32 = 0;
            #[doc = "An AHB error has occurred"]
            pub const AHB_ERR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address Error field"]
    pub mod ADDR_ERR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No address error"]
            pub const ADDERR_ERRNO: u32 = 0;
            #[doc = "An address error has occurred"]
            pub const ADDERR_ERR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "HP Timeout Register"]
pub mod HP_TIMEOUT {
    pub use crate::RW as access;
    #[doc = "Time out value"]
    pub mod TIMEOUT_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Hardware Request Pending Register"]
pub mod HW_GROUP_PENDING {
    pub use crate::RO as access;
    #[doc = "This field indicates which groups are pending for save from hardware request"]
    pub mod HW_SAVE_PENDING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field indicates which groups are pending for restore from hardware request"]
    pub mod HW_RESTORE_PENDING {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Software Request Pending Register"]
pub mod SW_GROUP_PENDING {
    pub use crate::RO as access;
    #[doc = "This field indicates which groups are pending for save from software request"]
    pub mod SW_SAVE_PENDING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field indicates which groups are pending for restore from software request"]
    pub mod SW_RESTORE_PENDING {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
