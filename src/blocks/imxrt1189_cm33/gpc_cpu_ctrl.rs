#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Array of registers: _CM_AUTHEN_CTRL, _CM_MISC, _CM_MODE_CTRL, _CM_MODE_STAT, _CM_IRQ_WAKEUP_MASK_0, _CM_IRQ_WAKEUP_MASK_1, _CM_IRQ_WAKEUP_MASK_2, _CM_IRQ_WAKEUP_MASK_3, _CM_IRQ_WAKEUP_MASK_4, _CM_IRQ_WAKEUP_MASK_5, _CM_IRQ_WAKEUP_MASK_6, _CM_IRQ_WAKEUP_MASK_7, _CM_NON_IRQ_WAKEUP_MASK, _CM_IRQ_WAKEUP_STAT_0, _CM_IRQ_WAKEUP_STAT_1, _CM_IRQ_WAKEUP_STAT_2, _CM_IRQ_WAKEUP_STAT_3, _CM_IRQ_WAKEUP_STAT_4, _CM_IRQ_WAKEUP_STAT_5, _CM_IRQ_WAKEUP_STAT_6, _CM_IRQ_WAKEUP_STAT_7, _CM_NON_IRQ_WAKEUP_STAT, _CM_SLEEP_SSAR_CTRL, _CM_SLEEP_LPCG_CTRL, _CM_SLEEP_PLL_CTRL, _CM_SLEEP_ISO_CTRL, _CM_SLEEP_RESET_CTRL, _CM_SLEEP_POWER_CTRL, _CM_WAKEUP_POWER_CTRL, _CM_WAKEUP_RESET_CTRL, _CM_WAKEUP_ISO_CTRL, _CM_WAKEUP_PLL_CTRL, _CM_WAKEUP_LPCG_CTRL, _CM_WAKEUP_SSAR_CTRL, _CM_SYS_SLEEP_CTRL"]
    pub AUTHEN: [AUTHEN::RegisterBlock; 2usize],
}
pub mod AUTHEN {
    #[doc = "Array of registers: _CM_AUTHEN_CTRL, _CM_MISC, _CM_MODE_CTRL, _CM_MODE_STAT, _CM_IRQ_WAKEUP_MASK_0, _CM_IRQ_WAKEUP_MASK_1, _CM_IRQ_WAKEUP_MASK_2, _CM_IRQ_WAKEUP_MASK_3, _CM_IRQ_WAKEUP_MASK_4, _CM_IRQ_WAKEUP_MASK_5, _CM_IRQ_WAKEUP_MASK_6, _CM_IRQ_WAKEUP_MASK_7, _CM_NON_IRQ_WAKEUP_MASK, _CM_IRQ_WAKEUP_STAT_0, _CM_IRQ_WAKEUP_STAT_1, _CM_IRQ_WAKEUP_STAT_2, _CM_IRQ_WAKEUP_STAT_3, _CM_IRQ_WAKEUP_STAT_4, _CM_IRQ_WAKEUP_STAT_5, _CM_IRQ_WAKEUP_STAT_6, _CM_IRQ_WAKEUP_STAT_7, _CM_NON_IRQ_WAKEUP_STAT, _CM_SLEEP_SSAR_CTRL, _CM_SLEEP_LPCG_CTRL, _CM_SLEEP_PLL_CTRL, _CM_SLEEP_ISO_CTRL, _CM_SLEEP_RESET_CTRL, _CM_SLEEP_POWER_CTRL, _CM_WAKEUP_POWER_CTRL, _CM_WAKEUP_RESET_CTRL, _CM_WAKEUP_ISO_CTRL, _CM_WAKEUP_PLL_CTRL, _CM_WAKEUP_LPCG_CTRL, _CM_WAKEUP_SSAR_CTRL, _CM_SYS_SLEEP_CTRL"]
    #[repr(C)]
    pub struct RegisterBlock {
        _reserved0: [u8; 0x04],
        #[doc = "CM Authentication Control"]
        pub _CM_AUTHEN_CTRL: u32,
        _reserved1: [u8; 0x04],
        #[doc = "Miscellaneous"]
        pub _CM_MISC: u32,
        #[doc = "CPU mode control"]
        pub _CM_MODE_CTRL: u32,
        #[doc = "CM CPU mode Status"]
        pub _CM_MODE_STAT: u32,
        _reserved2: [u8; 0xe8],
        #[doc = "CM IRQ0~31 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_0: u32,
        #[doc = "CM IRQ32~63 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_1: u32,
        #[doc = "CM IRQ64~95 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_2: u32,
        #[doc = "CM IRQ96~127 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_3: u32,
        #[doc = "CM IRQ128~159 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_4: u32,
        #[doc = "CM IRQ160~191 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_5: u32,
        #[doc = "CM IRQ192~223 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_6: u32,
        #[doc = "CM IRQ224~255 wakeup mask"]
        pub _CM_IRQ_WAKEUP_MASK_7: u32,
        _reserved3: [u8; 0x20],
        #[doc = "CM non-IRQ wakeup mask"]
        pub _CM_NON_IRQ_WAKEUP_MASK: u32,
        _reserved4: [u8; 0x0c],
        #[doc = "CM IRQ0~31 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_0: u32,
        #[doc = "CM IRQ32~63 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_1: u32,
        #[doc = "CM IRQ64~95 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_2: u32,
        #[doc = "CM IRQ96~127 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_3: u32,
        #[doc = "CM IRQ128~159 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_4: u32,
        #[doc = "CM IRQ160~191 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_5: u32,
        #[doc = "CM IRQ192~223 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_6: u32,
        #[doc = "CM IRQ224~255 wakeup status"]
        pub _CM_IRQ_WAKEUP_STAT_7: u32,
        _reserved5: [u8; 0x20],
        #[doc = "CM non-irq wakeup status"]
        pub _CM_NON_IRQ_WAKEUP_STAT: u32,
        _reserved6: [u8; 0x6c],
        #[doc = "CM sleep SSAR control"]
        pub _CM_SLEEP_SSAR_CTRL: u32,
        _reserved7: [u8; 0x04],
        #[doc = "CM sleep LPCG control"]
        pub _CM_SLEEP_LPCG_CTRL: u32,
        _reserved8: [u8; 0x04],
        #[doc = "CM sleep PLL control"]
        pub _CM_SLEEP_PLL_CTRL: u32,
        _reserved9: [u8; 0x04],
        #[doc = "CM sleep isolation control"]
        pub _CM_SLEEP_ISO_CTRL: u32,
        _reserved10: [u8; 0x04],
        #[doc = "CM sleep reset control"]
        pub _CM_SLEEP_RESET_CTRL: u32,
        _reserved11: [u8; 0x04],
        #[doc = "CM sleep power control"]
        pub _CM_SLEEP_POWER_CTRL: u32,
        _reserved12: [u8; 0x64],
        #[doc = "CM wakeup power control"]
        pub _CM_WAKEUP_POWER_CTRL: u32,
        _reserved13: [u8; 0x04],
        #[doc = "CM wakeup reset control"]
        pub _CM_WAKEUP_RESET_CTRL: u32,
        _reserved14: [u8; 0x04],
        #[doc = "CM wakeup isolation control"]
        pub _CM_WAKEUP_ISO_CTRL: u32,
        _reserved15: [u8; 0x04],
        #[doc = "CM wakeup PLL control"]
        pub _CM_WAKEUP_PLL_CTRL: u32,
        _reserved16: [u8; 0x04],
        #[doc = "CM wakeup LPCG control"]
        pub _CM_WAKEUP_LPCG_CTRL: u32,
        _reserved17: [u8; 0x0c],
        #[doc = "CM wakeup SSAR control"]
        pub _CM_WAKEUP_SSAR_CTRL: u32,
        _reserved18: [u8; 0xbc],
        #[doc = "CM system sleep control"]
        pub _CM_SYS_SLEEP_CTRL: u32,
        _reserved19: [u8; 0x047c],
    }
    #[doc = "CM Authentication Control"]
    pub mod _CM_AUTHEN_CTRL {
        pub use crate::RW as access;
        #[doc = "Configuration lock"]
        pub mod LOCK_CFG {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "The value of low power configuration fields are not locked."]
                pub const B0: u32 = 0;
                #[doc = "The value of low power configuration fields are locked. It locks the CPUx_CM registers which are marked as \"Locked by LOCK_CFG field\" in the function field."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Allow user mode access"]
        pub mod USER {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Allow only privilege mode to access CPU mode control registers"]
                pub const B0: u32 = 0;
                #[doc = "Allow both privilege and user mode to access CPU mode control registers"]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Allow non-secure mode access"]
        pub mod NONSECURE {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Allow only secure mode to access CPU mode control"]
                pub const B0: u32 = 0;
                #[doc = "Allow both secure and non-secure mode to access CPU mode control registers"]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Lock NONSECURE and USER"]
        pub mod LOCK_SETTING {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "NONSECURE and USER fields are not locked"]
                pub const B0: u32 = 0;
                #[doc = "NONSECURE and USER fields are locked"]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "White list lock"]
        pub mod LOCK_LIST {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "WHITE_LIST is not locked"]
                pub const B0: u32 = 0;
                #[doc = "WHITE_LIST is locked"]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Domain ID white list"]
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
    #[doc = "Miscellaneous"]
    pub mod _CM_MISC {
        pub use crate::RW as access;
        #[doc = "Non-masked interrupt status"]
        pub mod NMI_STAT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "NMI is not asserted"]
                pub const B0: u32 = 0;
                #[doc = "NMI is asserted"]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Allow cpu_sleep_hold_req to assert during CPU low power status"]
        pub mod SLEEP_HOLD_EN {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Disable cpu_sleep_hold_req"]
                pub const B0: u32 = 0;
                #[doc = "Allow cpu_sleep_hold_req to assert during CPU low power status"]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "CPU sleep hold status"]
        pub mod SLEEP_HOLD_STAT {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "CPU sleep hold is acknowledged"]
                pub const B0: u32 = 0;
                #[doc = "CPU is not in sleep hold"]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CPU mode control"]
    pub mod _CM_MODE_CTRL {
        pub use crate::RW as access;
        #[doc = "The CPU mode the CPU platform should transit to on next sleep event"]
        pub mod CPU_MODE_TARGET {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x03 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Stay in RUN mode"]
                pub const B0: u32 = 0;
                #[doc = "Transit to WAIT mode"]
                pub const B1: u32 = 0x01;
                #[doc = "Transit to STOP mode"]
                pub const B2: u32 = 0x02;
                #[doc = "Transit to SUSPEND mode"]
                pub const B3: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "WFE assertion can be sleep event"]
        pub mod WFE_EN {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "WFE assertion can not trigger low power"]
                pub const B0: u32 = 0;
                #[doc = "WFE assertion can trigger low power"]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM CPU mode Status"]
    pub mod _CM_MODE_STAT {
        pub use crate::RO as access;
        #[doc = "Current CPU mode"]
        pub mod CPU_MODE_CURRENT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x03 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "CPU is currently in RUN mode"]
                pub const B0: u32 = 0;
                #[doc = "CPU is currently in WAIT mode"]
                pub const B1: u32 = 0x01;
                #[doc = "CPU is currently in STOP mode"]
                pub const B2: u32 = 0x02;
                #[doc = "CPU is currently in SUSPEND mode"]
                pub const B3: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Previous CPU mode"]
        pub mod CPU_MODE_PREVIOUS {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x03 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "CPU was previously in RUN mode"]
                pub const B0: u32 = 0;
                #[doc = "CPU was previously in WAIT mode"]
                pub const B1: u32 = 0x01;
                #[doc = "CPU was previously in STOP mode"]
                pub const B2: u32 = 0x02;
                #[doc = "CPU was previously in SUSPEND mode"]
                pub const B3: u32 = 0x03;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ0~31 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_0 {
        pub use crate::RW as access;
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_0_31 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ32~63 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_1 {
        pub use crate::RW as access;
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_32_63 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ64~95 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_2 {
        pub use crate::RW as access;
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_64_95 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ96~127 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_3 {
        pub use crate::RW as access;
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_96_127 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ128~159 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_4 {
        pub use crate::RW as access;
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_128_159 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ160~191 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_5 {
        pub use crate::RW as access;
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_160_191 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ192~223 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_6 {
        pub use crate::RW as access;
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_192_223 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ224~255 wakeup mask"]
    pub mod _CM_IRQ_WAKEUP_MASK_7 {
        pub use crate::RW as access;
        #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
        pub mod MASK_224_255 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM non-IRQ wakeup mask"]
    pub mod _CM_NON_IRQ_WAKEUP_MASK {
        pub use crate::RW as access;
        #[doc = "\"1\" means the debug_wakeup_request cannot wakeup CPU platform"]
        pub mod DEBUG_WAKEUP_MASK {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ0~31 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_0 {
        pub use crate::RO as access;
        #[doc = "IRQ status"]
        pub mod STAT_0_31 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ32~63 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_1 {
        pub use crate::RO as access;
        #[doc = "IRQ status"]
        pub mod STAT_32_63 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ64~95 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_2 {
        pub use crate::RO as access;
        #[doc = "IRQ status"]
        pub mod STAT_64_95 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ96~127 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_3 {
        pub use crate::RO as access;
        #[doc = "IRQ status"]
        pub mod STAT_96_127 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ128~159 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_4 {
        pub use crate::RO as access;
        #[doc = "IRQ status"]
        pub mod STAT_128_159 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ160~191 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_5 {
        pub use crate::RO as access;
        #[doc = "IRQ status"]
        pub mod STAT_160_191 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ192~223 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_6 {
        pub use crate::RO as access;
        #[doc = "IRQ status"]
        pub mod STAT_192_223 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM IRQ224~255 wakeup status"]
    pub mod _CM_IRQ_WAKEUP_STAT_7 {
        pub use crate::RO as access;
        #[doc = "IRQ status"]
        pub mod STAT_224_255 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM non-irq wakeup status"]
    pub mod _CM_NON_IRQ_WAKEUP_STAT {
        pub use crate::RO as access;
        #[doc = "Debug wakeup status"]
        pub mod DEBUG_WAKEUP_STAT {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "No debug wakeup is requested"]
                pub const B0: u32 = 0;
                #[doc = "Debug wakeup is requested"]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM sleep SSAR control"]
    pub mod _CM_SLEEP_SSAR_CTRL {
        pub use crate::RW as access;
        #[doc = "Disable this step"]
        pub mod DISABLE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "This step is enabled."]
                pub const B0: u32 = 0;
                #[doc = "This step is disabled. GPC will skip this step and not send any request."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM sleep LPCG control"]
    pub mod _CM_SLEEP_LPCG_CTRL {
        pub use crate::RW as access;
        #[doc = "Disable this step"]
        pub mod DISABLE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "This step is enabled."]
                pub const B0: u32 = 0;
                #[doc = "This step is disabled. GPC will skip this step and not send any request."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM sleep PLL control"]
    pub mod _CM_SLEEP_PLL_CTRL {
        pub use crate::RW as access;
        #[doc = "Disable this step"]
        pub mod DISABLE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "This step is enabled."]
                pub const B0: u32 = 0;
                #[doc = "This step is disabled. GPC will skip this step and not send any request."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM sleep isolation control"]
    pub mod _CM_SLEEP_ISO_CTRL {
        pub use crate::RW as access;
        #[doc = "Disable this step"]
        pub mod DISABLE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "This step is enabled."]
                pub const B0: u32 = 0;
                #[doc = "This step is disabled. GPC will skip this step and not send any request."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM sleep reset control"]
    pub mod _CM_SLEEP_RESET_CTRL {
        pub use crate::RW as access;
        #[doc = "Disable this step"]
        pub mod DISABLE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "This step is enabled."]
                pub const B0: u32 = 0;
                #[doc = "This step is disabled. GPC will skip this step and not send any request."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM sleep power control"]
    pub mod _CM_SLEEP_POWER_CTRL {
        pub use crate::RW as access;
        #[doc = "Disable this step"]
        pub mod DISABLE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "This step is enabled."]
                pub const B0: u32 = 0;
                #[doc = "This step is disabled. GPC will skip this step and not send any request."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM wakeup power control"]
    pub mod _CM_WAKEUP_POWER_CTRL {
        pub use crate::RW as access;
        #[doc = "Disable this step"]
        pub mod DISABLE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "This step is enabled."]
                pub const B0: u32 = 0;
                #[doc = "This step is disabled. GPC will skip this step and not send any request."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM wakeup reset control"]
    pub mod _CM_WAKEUP_RESET_CTRL {
        pub use crate::RW as access;
        #[doc = "Disable this step"]
        pub mod DISABLE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "This step is enabled."]
                pub const B0: u32 = 0;
                #[doc = "This step is disabled. GPC will skip this step and not send any request."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM wakeup isolation control"]
    pub mod _CM_WAKEUP_ISO_CTRL {
        pub use crate::RW as access;
        #[doc = "Disable this step"]
        pub mod DISABLE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "This step is enabled."]
                pub const B0: u32 = 0;
                #[doc = "This step is disabled. GPC will skip this step and not send any request."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM wakeup PLL control"]
    pub mod _CM_WAKEUP_PLL_CTRL {
        pub use crate::RW as access;
        #[doc = "Disable this step"]
        pub mod DISABLE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "This step is enabled."]
                pub const B0: u32 = 0;
                #[doc = "This step is disabled. GPC will skip this step and not send any request."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM wakeup LPCG control"]
    pub mod _CM_WAKEUP_LPCG_CTRL {
        pub use crate::RW as access;
        #[doc = "Disable this step"]
        pub mod DISABLE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "This step is enabled."]
                pub const B0: u32 = 0;
                #[doc = "This step is disabled. GPC will skip this step and not send any request."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM wakeup SSAR control"]
    pub mod _CM_WAKEUP_SSAR_CTRL {
        pub use crate::RW as access;
        #[doc = "Disable this step"]
        pub mod DISABLE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "This step is enabled."]
                pub const B0: u32 = 0;
                #[doc = "This step is disabled. GPC will skip this step and not send any request."]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "CM system sleep control"]
    pub mod _CM_SYS_SLEEP_CTRL {
        pub use crate::RW as access;
        #[doc = "Request system sleep when CPU is in WAIT mode"]
        pub mod SS_WAIT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Do not request system sleep when CPU is in WAIT mode"]
                pub const B0: u32 = 0;
                #[doc = "Request system sleep when CPU is in WAIT mode"]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Request system sleep when CPU is in STOP mode"]
        pub mod SS_STOP {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Do not request system sleep when CPU is in STOP mode"]
                pub const B0: u32 = 0;
                #[doc = "Request system sleep when CPU is in STOP mode"]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Request system sleep when CPU is in SUSPEND mode"]
        pub mod SS_SUSPEND {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Do not request system sleep when CPU is in SUSPEND mode"]
                pub const B0: u32 = 0;
                #[doc = "Request system sleep when CPU is in SUSPEND mode"]
                pub const B1: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
}
