#[doc = "PGMC_PPC"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "PPC Authentication Control"]
    pub PPC_AUTHEN_CTRL: u32,
    _reserved1: [u8; 0x08],
    #[doc = "PPC Mode"]
    pub PPC_MODE: u32,
    #[doc = "PPC standby CPU mode control"]
    pub PPC_STBY_CM_CTRL: u32,
    #[doc = "PPC standby Setpoint control"]
    pub PPC_STBY_SP_CTRL: u32,
}
#[doc = "PPC Authentication Control"]
pub mod PPC_AUTHEN_CTRL {
    pub use crate::RW as access;
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock NONSECURE and USER"]
    pub mod LOCK_SETTING {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Configuration lock"]
    pub mod LOCK_CFG {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PPC Mode"]
pub mod PPC_MODE {
    pub use crate::RW as access;
    #[doc = "Control mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod CTRL_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not affected by any low power mode"]
            pub const CTRL_MODE_0: u32 = 0;
            #[doc = "Controlled by CPU power mode of the domain"]
            pub const CTRL_MODE_1: u32 = 0x01;
            #[doc = "Controlled by Setpoint and system standby"]
            pub const CTRL_MODE_2: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain assignment of the BPC"]
    pub mod DOMAIN_ASSIGN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Domain 0"]
            pub const D0: u32 = 0;
            #[doc = "Domain 1"]
            pub const D1: u32 = 0x01;
            #[doc = "Domain 2"]
            pub const D2: u32 = 0x02;
            #[doc = "Domain 3"]
            pub const D3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PPC standby CPU mode control"]
pub mod PPC_STBY_CM_CTRL {
    pub use crate::RW as access;
    #[doc = "PMIC Standby on when domain enters WAIT mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod STBY_ON_AT_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PMIC Standby on when domain enters STOP mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod STBY_ON_AT_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PMIC Standby on when domain enters SUSPEND mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod STBY_ON_AT_SUSPEND {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software PMIC standby on trigger"]
    pub mod STBY_ON_SOFT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software PMIC standby off trigger"]
    pub mod STBY_OFF_SOFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PPC standby Setpoint control"]
pub mod PPC_STBY_SP_CTRL {
    pub use crate::RW as access;
    #[doc = "PMIC standby on when system enters Setpoint number. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod STBY_ON_AT_SP_ACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PMIC standby on when system enters Setpoint number and system is in standby mode. This field is locked by AUTHEN_CTRL\\[LOCK_CFG\\] field."]
    pub mod STBY_ON_AT_SP_SLEEP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
