#[doc = "CCM_OBS"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Clock root section."]
    pub OBSERVE: [OBSERVE::RegisterBlock; 6usize],
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
        pub OBSERVE_STATUS0: u32,
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
        _reserved2: [u8; 0x34],
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
                pub const RAW_0: u32 = 0;
                #[doc = "Select raw signal."]
                pub const RAW_1: u32 = 0x01;
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
                pub const INV_0: u32 = 0;
                #[doc = "Invert clock phase before measurement or send to IO."]
                pub const INV_1: u32 = 0x01;
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
                #[doc = "No reset"]
                pub const RESET_0: u32 = 0;
                #[doc = "Reset observe divider"]
                pub const RESET_1: u32 = 0x01;
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
        #[doc = "Turn off"]
        pub mod OFF {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "observe slice is on"]
                pub const OFF_0: u32 = 0;
                #[doc = "observe slice is off"]
                pub const OFF_1: u32 = 0x01;
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
        #[doc = "Invert"]
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
        #[doc = "Invert"]
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
        #[doc = "Invert"]
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
    pub mod OBSERVE_STATUS0 {
        pub use crate::RO as access;
        #[doc = "Select value"]
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
                #[doc = "Divided signal is selected"]
                pub const RAW_0: u32 = 0;
                #[doc = "Raw signal is selected"]
                pub const RAW_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Polarity of the observe target"]
        pub mod INV {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Polarity is not inverted"]
                pub const INV_0: u32 = 0;
                #[doc = "Polarity of the observe target is inverted"]
                pub const INV_1: u32 = 0x01;
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
                pub const RESET_0: u32 = 0;
                #[doc = "Observe divider is in reset state"]
                pub const RESET_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Divide value status. The clock will be divided by DIVIDE + 1."]
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
                pub const OFF_0: u32 = 0;
                #[doc = "observe slice is off"]
                pub const OFF_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Observe access control"]
    pub mod OBSERVE_AUTHEN {
        pub use crate::RW as access;
        #[doc = "User access"]
        pub mod TZ_USER {
            pub const offset: u32 = 0;
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
        #[doc = "Non-secure access"]
        pub mod TZ_NS {
            pub const offset: u32 = 1;
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
        #[doc = "Lock truszone setting"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 4;
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
        #[doc = "White list"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x0f << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "No domain can change."]
                pub const WHITE_LIST_0: u32 = 0;
                #[doc = "Domain 0 can change."]
                pub const WHITE_LIST_1: u32 = 0x01;
                #[doc = "Domain 1 can change."]
                pub const WHITE_LIST_2: u32 = 0x02;
                #[doc = "Domain 0 and domain 1 can change."]
                pub const WHITE_LIST_3: u32 = 0x03;
                #[doc = "Domain 2 can change."]
                pub const WHITE_LIST_4: u32 = 0x04;
                #[doc = "All domain can change."]
                pub const WHITE_LIST_15: u32 = 0x0f;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "White list is not locked."]
                pub const LOCK_LIST_0: u32 = 0;
                #[doc = "White list is locked."]
                pub const LOCK_LIST_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Low power and access control by domain"]
        pub mod DOMAIN_MODE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Clock does not work in domain mode."]
                pub const DOMAIN_MODE_0: u32 = 0;
                #[doc = "Clock works in domain mode."]
                pub const DOMAIN_MODE_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock low power and access mode"]
        pub mod LOCK_MODE {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "MODE is not locked."]
                pub const LOCK_MODE_0: u32 = 0;
                #[doc = "MODE is locked."]
                pub const LOCK_MODE_1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Observe access control"]
    pub mod OBSERVE_AUTHEN_SET {
        pub use crate::RW as access;
        #[doc = "User access"]
        pub mod TZ_USER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access"]
        pub mod TZ_NS {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock truszone setting"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "White list"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x0f << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Low power and access control by domain"]
        pub mod DOMAIN_MODE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock low power and access mode"]
        pub mod LOCK_MODE {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x01 << offset;
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
        #[doc = "User access"]
        pub mod TZ_USER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access"]
        pub mod TZ_NS {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock truszone setting"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "White list"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x0f << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Low power and access control by domain"]
        pub mod DOMAIN_MODE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock low power and access mode"]
        pub mod LOCK_MODE {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x01 << offset;
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
        #[doc = "User access"]
        pub mod TZ_USER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Non-secure access"]
        pub mod TZ_NS {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock truszone setting"]
        pub mod LOCK_TZ {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "White list"]
        pub mod WHITE_LIST {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x0f << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock white list"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Low power and access control by domain"]
        pub mod DOMAIN_MODE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock low power and access mode"]
        pub mod LOCK_MODE {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x01 << offset;
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
}
