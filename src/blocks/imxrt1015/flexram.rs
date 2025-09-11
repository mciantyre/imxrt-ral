#[doc = "FLEXRAM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "TCM CRTL Register"]
    pub TCM_CTRL: u32,
    _reserved0: [u8; 0x0c],
    #[doc = "Interrupt Status Register"]
    pub INT_STATUS: u32,
    #[doc = "Interrupt Status Enable Register"]
    pub INT_STAT_EN: u32,
    #[doc = "Interrupt Enable Register"]
    pub INT_SIG_EN: u32,
}
#[doc = "TCM CRTL Register"]
pub mod TCM_CTRL {
    pub use crate::RW as access;
    #[doc = "TCM Write Wait Mode Enable"]
    pub mod TCM_WWAIT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle."]
            pub const TCM_WWAIT_EN_0: u32 = 0;
            #[doc = "TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles."]
            pub const TCM_WWAIT_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "TCM Read Wait Mode Enable"]
    pub mod TCM_RWAIT_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle."]
            pub const TCM_RWAIT_EN_0: u32 = 0;
            #[doc = "TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles."]
            pub const TCM_RWAIT_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Force RAM Clock Always On"]
    pub mod FORCE_CLK_ON {
        pub const offset: u32 = 2;
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
    #[doc = "ITCM Access Error Status"]
    pub mod ITCM_ERR_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ITCM access error does not happen"]
            pub const ITCM_ERR_STATUS_0: u32 = 0;
            #[doc = "ITCM access error happens."]
            pub const ITCM_ERR_STATUS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DTCM Access Error Status"]
    pub mod DTCM_ERR_STATUS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DTCM access error does not happen"]
            pub const DTCM_ERR_STATUS_0: u32 = 0;
            #[doc = "DTCM access error happens."]
            pub const DTCM_ERR_STATUS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OCRAM Access Error Status"]
    pub mod OCRAM_ERR_STATUS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "OCRAM access error does not happen"]
            pub const OCRAM_ERR_STATUS_0: u32 = 0;
            #[doc = "OCRAM access error happens."]
            pub const OCRAM_ERR_STATUS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Interrupt Status Enable Register"]
pub mod INT_STAT_EN {
    pub use crate::RW as access;
    #[doc = "ITCM Access Error Status Enable"]
    pub mod ITCM_ERR_STAT_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Masked"]
            pub const ITCM_ERR_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ITCM_ERR_STAT_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DTCM Access Error Status Enable"]
    pub mod DTCM_ERR_STAT_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Masked"]
            pub const DTCM_ERR_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DTCM_ERR_STAT_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OCRAM Access Error Status Enable"]
    pub mod OCRAM_ERR_STAT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Masked"]
            pub const OCRAM_ERR_STAT_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const OCRAM_ERR_STAT_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Interrupt Enable Register"]
pub mod INT_SIG_EN {
    pub use crate::RW as access;
    #[doc = "ITCM Access Error Interrupt Enable"]
    pub mod ITCM_ERR_SIG_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Masked"]
            pub const ITCM_ERR_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const ITCM_ERR_SIG_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DTCM Access Error Interrupt Enable"]
    pub mod DTCM_ERR_SIG_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Masked"]
            pub const DTCM_ERR_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const DTCM_ERR_SIG_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OCRAM Access Error Interrupt Enable"]
    pub mod OCRAM_ERR_SIG_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Masked"]
            pub const OCRAM_ERR_SIG_EN_0: u32 = 0;
            #[doc = "Enabled"]
            pub const OCRAM_ERR_SIG_EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
