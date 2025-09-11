#[doc = "PGMC_BPC"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "BPC Authentication Control"]
    pub BPC_AUTHEN_CTRL: u32,
    _reserved1: [u8; 0x08],
    #[doc = "BPC Mode"]
    pub BPC_MODE: u32,
    #[doc = "BPC power control"]
    pub BPC_POWER_CTRL: u32,
    _reserved2: [u8; 0x14],
    #[doc = "BPC flag"]
    pub BPC_FLAG: u32,
    _reserved3: [u8; 0x10],
    #[doc = "BPC SSAR save control"]
    pub BPC_SSAR_SAVE_CTRL: u32,
    #[doc = "BPC SSAR restore control"]
    pub BPC_SSAR_RESTORE_CTRL: u32,
}
#[doc = "BPC Authentication Control"]
pub mod BPC_AUTHEN_CTRL {
    pub use crate::RW as access;
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allow only privilege mode to access basic power control registers"]
            pub const USER_0: u32 = 0;
            #[doc = "Allow both privilege and user mode to access basic power control registers"]
            pub const USER_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allow only secure mode to access basic power control registers"]
            pub const NONSECURE_0: u32 = 0;
            #[doc = "Allow both secure and non-secure mode to access basic power control registers"]
            pub const NONSECURE_1: u32 = 0x01;
        }
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
#[doc = "BPC Mode"]
pub mod BPC_MODE {
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
            #[doc = "Controlled by Setpoint"]
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
#[doc = "BPC power control"]
pub mod BPC_POWER_CTRL {
    pub use crate::RW as access;
    #[doc = "0x1: Power off when domain enters WAIT mode"]
    pub mod PWR_OFF_AT_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0x1: Power off when domain enters STOP mode"]
    pub mod PWR_OFF_AT_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0x1: Power off when domain enters SUSPEND mode"]
    pub mod PWR_OFF_AT_SUSPEND {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software isolation on trigger"]
    pub mod ISO_ON_SOFT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software power off trigger"]
    pub mod PSW_OFF_SOFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software power on trigger"]
    pub mod PSW_ON_SOFT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software isolation off trigger"]
    pub mod ISO_OFF_SOFT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Power off when system enters Setpoint number"]
    pub mod PWR_OFF_AT_SP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "BPC flag"]
pub mod BPC_FLAG {
    pub use crate::RW as access;
    #[doc = "set to 1 after power switch off, cleared by writing 1"]
    pub mod PDN_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "BPC SSAR save control"]
pub mod BPC_SSAR_SAVE_CTRL {
    pub use crate::RW as access;
    #[doc = "Save data at RUN mode, software writting 0x1 to trigger SSARC to execute save process"]
    pub mod SAVE_AT_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save data when domain enters WAIT mode"]
    pub mod SAVE_AT_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save data when domain enters STOP mode"]
    pub mod SAVE_AT_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save data when domain enters SUSPEND mode"]
    pub mod SAVE_AT_SUSPEND {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Save data when system enters a Setpoint."]
    pub mod SAVE_AT_SP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "BPC SSAR restore control"]
pub mod BPC_SSAR_RESTORE_CTRL {
    pub use crate::RW as access;
    #[doc = "Restore data at RUN mode"]
    pub mod RESTORE_AT_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Restore data when system enters a Setpoint."]
    pub mod RESTORE_AT_SP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
