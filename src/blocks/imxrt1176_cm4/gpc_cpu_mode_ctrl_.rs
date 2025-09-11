#[doc = "GPC_CPU"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "CM Authentication Control"]
    pub CM_AUTHEN_CTRL: u32,
    #[doc = "CM Interrupt Control"]
    pub CM_INT_CTRL: u32,
    #[doc = "Miscellaneous"]
    pub CM_MISC: u32,
    #[doc = "CPU mode control"]
    pub CM_MODE_CTRL: u32,
    #[doc = "CM CPU mode Status"]
    pub CM_MODE_STAT: u32,
    _reserved1: [u8; 0xe8],
    #[doc = "CM IRQ0~31 wakeup mask"]
    pub CM_IRQ_WAKEUP_MASK_0: u32,
    #[doc = "CM IRQ32~63 wakeup mask"]
    pub CM_IRQ_WAKEUP_MASK_1: u32,
    #[doc = "CM IRQ64~95 wakeup mask"]
    pub CM_IRQ_WAKEUP_MASK_2: u32,
    #[doc = "CM IRQ96~127 wakeup mask"]
    pub CM_IRQ_WAKEUP_MASK_3: u32,
    #[doc = "CM IRQ128~159 wakeup mask"]
    pub CM_IRQ_WAKEUP_MASK_4: u32,
    #[doc = "CM IRQ160~191 wakeup mask"]
    pub CM_IRQ_WAKEUP_MASK_5: u32,
    #[doc = "CM IRQ192~223 wakeup mask"]
    pub CM_IRQ_WAKEUP_MASK_6: u32,
    #[doc = "CM IRQ224~255 wakeup mask"]
    pub CM_IRQ_WAKEUP_MASK_7: u32,
    _reserved2: [u8; 0x20],
    #[doc = "CM non-irq wakeup mask"]
    pub CM_NON_IRQ_WAKEUP_MASK: u32,
    _reserved3: [u8; 0x0c],
    #[doc = "CM IRQ0~31 wakeup status"]
    pub CM_IRQ_WAKEUP_STAT_0: u32,
    #[doc = "CM IRQ32~63 wakeup status"]
    pub CM_IRQ_WAKEUP_STAT_1: u32,
    #[doc = "CM IRQ64~95 wakeup status"]
    pub CM_IRQ_WAKEUP_STAT_2: u32,
    #[doc = "CM IRQ96~127 wakeup status"]
    pub CM_IRQ_WAKEUP_STAT_3: u32,
    #[doc = "CM IRQ128~159 wakeup status"]
    pub CM_IRQ_WAKEUP_STAT_4: u32,
    #[doc = "CM IRQ160~191 wakeup status"]
    pub CM_IRQ_WAKEUP_STAT_5: u32,
    #[doc = "CM IRQ192~223 wakeup status"]
    pub CM_IRQ_WAKEUP_STAT_6: u32,
    #[doc = "CM IRQ224~255 wakeup status"]
    pub CM_IRQ_WAKEUP_STAT_7: u32,
    _reserved4: [u8; 0x20],
    #[doc = "CM non-irq wakeup status"]
    pub CM_NON_IRQ_WAKEUP_STAT: u32,
    _reserved5: [u8; 0x6c],
    #[doc = "CM sleep SSAR control"]
    pub CM_SLEEP_SSAR_CTRL: u32,
    _reserved6: [u8; 0x04],
    #[doc = "CM sleep LPCG control"]
    pub CM_SLEEP_LPCG_CTRL: u32,
    _reserved7: [u8; 0x04],
    #[doc = "CM sleep PLL control"]
    pub CM_SLEEP_PLL_CTRL: u32,
    _reserved8: [u8; 0x04],
    #[doc = "CM sleep isolation control"]
    pub CM_SLEEP_ISO_CTRL: u32,
    _reserved9: [u8; 0x04],
    #[doc = "CM sleep reset control"]
    pub CM_SLEEP_RESET_CTRL: u32,
    _reserved10: [u8; 0x04],
    #[doc = "CM sleep power control"]
    pub CM_SLEEP_POWER_CTRL: u32,
    _reserved11: [u8; 0x64],
    #[doc = "CM wakeup power control"]
    pub CM_WAKEUP_POWER_CTRL: u32,
    _reserved12: [u8; 0x04],
    #[doc = "CM wakeup reset control"]
    pub CM_WAKEUP_RESET_CTRL: u32,
    _reserved13: [u8; 0x04],
    #[doc = "CM wakeup isolation control"]
    pub CM_WAKEUP_ISO_CTRL: u32,
    _reserved14: [u8; 0x04],
    #[doc = "CM wakeup PLL control"]
    pub CM_WAKEUP_PLL_CTRL: u32,
    _reserved15: [u8; 0x04],
    #[doc = "CM wakeup LPCG control"]
    pub CM_WAKEUP_LPCG_CTRL: u32,
    _reserved16: [u8; 0x04],
    #[doc = "CM wakeup SSAR control"]
    pub CM_WAKEUP_SSAR_CTRL: u32,
    _reserved17: [u8; 0x44],
    #[doc = "CM Setpoint Control"]
    pub CM_SP_CTRL: u32,
    #[doc = "CM Setpoint Status"]
    pub CM_SP_STAT: u32,
    _reserved18: [u8; 0x08],
    #[doc = "CM Run Mode Setpoint Allowed"]
    pub CM_RUN_MODE_MAPPING: u32,
    #[doc = "CM Wait Mode Setpoint Allowed"]
    pub CM_WAIT_MODE_MAPPING: u32,
    #[doc = "CM Stop Mode Setpoint Allowed"]
    pub CM_STOP_MODE_MAPPING: u32,
    #[doc = "CM Suspend Mode Setpoint Allowed"]
    pub CM_SUSPEND_MODE_MAPPING: u32,
    #[doc = "CM Setpoint 0 Mapping"]
    pub CM_SP0_MAPPING: u32,
    #[doc = "CM Setpoint 1 Mapping"]
    pub CM_SP1_MAPPING: u32,
    #[doc = "CM Setpoint 2 Mapping"]
    pub CM_SP2_MAPPING: u32,
    #[doc = "CM Setpoint 3 Mapping"]
    pub CM_SP3_MAPPING: u32,
    #[doc = "CM Setpoint 4 Mapping"]
    pub CM_SP4_MAPPING: u32,
    #[doc = "CM Setpoint 5 Mapping"]
    pub CM_SP5_MAPPING: u32,
    #[doc = "CM Setpoint 6 Mapping"]
    pub CM_SP6_MAPPING: u32,
    #[doc = "CM Setpoint 7 Mapping"]
    pub CM_SP7_MAPPING: u32,
    #[doc = "CM Setpoint 8 Mapping"]
    pub CM_SP8_MAPPING: u32,
    #[doc = "CM Setpoint 9 Mapping"]
    pub CM_SP9_MAPPING: u32,
    #[doc = "CM Setpoint 10 Mapping"]
    pub CM_SP10_MAPPING: u32,
    #[doc = "CM Setpoint 11 Mapping"]
    pub CM_SP11_MAPPING: u32,
    #[doc = "CM Setpoint 12 Mapping"]
    pub CM_SP12_MAPPING: u32,
    #[doc = "CM Setpoint 13 Mapping"]
    pub CM_SP13_MAPPING: u32,
    #[doc = "CM Setpoint 14 Mapping"]
    pub CM_SP14_MAPPING: u32,
    #[doc = "CM Setpoint 15 Mapping"]
    pub CM_SP15_MAPPING: u32,
    _reserved19: [u8; 0x20],
    #[doc = "CM standby control"]
    pub CM_STBY_CTRL: u32,
}
#[doc = "CM Authentication Control"]
pub mod CM_AUTHEN_CTRL {
    pub use crate::RW as access;
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 0;
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
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allow only secure mode to access CPU mode control registers"]
            pub const B0: u32 = 0;
            #[doc = "Allow both secure and non-secure mode to access CPU mode control registers"]
            pub const B1: u32 = 0x01;
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
#[doc = "CM Interrupt Control"]
pub mod CM_INT_CTRL {
    pub use crate::RW as access;
    #[doc = "sp_req_not_allowed_for_sleep interrupt enable"]
    pub mod SP_REQ_NOT_ALLOWED_SLEEP_INT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt disable"]
            pub const B0: u32 = 0;
            #[doc = "Interrupt enable"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "sp_req_not_allowed_for_wakeup interrupt enable"]
    pub mod SP_REQ_NOT_ALLOWED_WAKEUP_INT_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt disable"]
            pub const B0: u32 = 0;
            #[doc = "Interrupt enable"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "sp_req_not_allowed_for_soft interrupt enable"]
    pub mod SP_REQ_NOT_ALLOWED_SOFT_INT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt disable"]
            pub const B0: u32 = 0;
            #[doc = "Interrupt enable"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "sp_req_not_allowed_for_sleep interrupt status and clear register"]
    pub mod SP_REQ_NOT_ALLOWED_SLEEP_INT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "sp_req_not_allowed_for_wakeup interrupt status and clear register"]
    pub mod SP_REQ_NOT_ALLOWED_WAKEUP_INT {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "sp_req_not_allowed_for_soft interrupt status and clear register"]
    pub mod SP_REQ_NOT_ALLOWED_SOFT_INT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous"]
pub mod CM_MISC {
    pub use crate::RW as access;
    #[doc = "Non-masked interrupt status"]
    pub mod NMI_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "NMI is not asserting"]
            pub const B0: u32 = 0;
            #[doc = "NMI is asserting"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow cpu_sleep_hold_req assert during CPU low power status"]
    pub mod SLEEP_HOLD_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable cpu_sleep_hold_req"]
            pub const B0: u32 = 0;
            #[doc = "Allow cpu_sleep_hold_req assert during CPU low power status"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Status of cpu_sleep_hold_ack_b"]
    pub mod SLEEP_HOLD_STAT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Master CPU"]
    pub mod MASTER_CPU {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CPU mode control"]
pub mod CM_MODE_CTRL {
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
pub mod CM_MODE_STAT {
    pub use crate::RO as access;
    #[doc = "Current CPU mode"]
    pub mod CPU_MODE_CURRENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "CPU is currently in RUN mode"]
            pub const RUN: u32 = 0;
            #[doc = "CPU is currently in WAIT mode"]
            pub const WAIT: u32 = 0x01;
            #[doc = "CPU is currently in STOP mode"]
            pub const STOP: u32 = 0x02;
            #[doc = "CPU is currently in SUSPEND mode"]
            pub const SUSPEND: u32 = 0x03;
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
            pub const RUN: u32 = 0;
            #[doc = "CPU was previously in WAIT mode"]
            pub const WAIT: u32 = 0x01;
            #[doc = "CPU was previously in STOP mode"]
            pub const STOP: u32 = 0x02;
            #[doc = "CPU was previously in SUSPEND mode"]
            pub const SUSPEND: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM IRQ0~31 wakeup mask"]
pub mod CM_IRQ_WAKEUP_MASK_0 {
    pub use crate::RW as access;
    #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
    pub mod IRQ_WAKEUP_MASK_0_31 {
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
pub mod CM_IRQ_WAKEUP_MASK_1 {
    pub use crate::RW as access;
    #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
    pub mod IRQ_WAKEUP_MASK_32_63 {
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
pub mod CM_IRQ_WAKEUP_MASK_2 {
    pub use crate::RW as access;
    #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
    pub mod IRQ_WAKEUP_MASK_64_95 {
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
pub mod CM_IRQ_WAKEUP_MASK_3 {
    pub use crate::RW as access;
    #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
    pub mod IRQ_WAKEUP_MASK_96_127 {
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
pub mod CM_IRQ_WAKEUP_MASK_4 {
    pub use crate::RW as access;
    #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
    pub mod IRQ_WAKEUP_MASK_128_159 {
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
pub mod CM_IRQ_WAKEUP_MASK_5 {
    pub use crate::RW as access;
    #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
    pub mod IRQ_WAKEUP_MASK_160_191 {
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
pub mod CM_IRQ_WAKEUP_MASK_6 {
    pub use crate::RW as access;
    #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
    pub mod IRQ_WAKEUP_MASK_192_223 {
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
pub mod CM_IRQ_WAKEUP_MASK_7 {
    pub use crate::RW as access;
    #[doc = "\"1\" means the IRQ cannot wakeup CPU platform"]
    pub mod IRQ_WAKEUP_MASK_224_255 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM non-irq wakeup mask"]
pub mod CM_NON_IRQ_WAKEUP_MASK {
    pub use crate::RW as access;
    #[doc = "There are 256 interrupts and 1 event as a wakeup source for GPC. This field masks the 1 event wakeup source."]
    pub mod EVENT_WAKEUP_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The event cannot wakeup CPU platform"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
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
pub mod CM_IRQ_WAKEUP_STAT_0 {
    pub use crate::RO as access;
    #[doc = "IRQ status"]
    pub mod IRQ_WAKEUP_STAT_0_31 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "None"]
            pub const B0: u32 = 0;
            #[doc = "Valid"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM IRQ32~63 wakeup status"]
pub mod CM_IRQ_WAKEUP_STAT_1 {
    pub use crate::RO as access;
    #[doc = "IRQ status"]
    pub mod IRQ_WAKEUP_STAT_32_63 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "None"]
            pub const B0: u32 = 0;
            #[doc = "Valid"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM IRQ64~95 wakeup status"]
pub mod CM_IRQ_WAKEUP_STAT_2 {
    pub use crate::RO as access;
    #[doc = "IRQ status"]
    pub mod IRQ_WAKEUP_STAT_64_95 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "None"]
            pub const B0: u32 = 0;
            #[doc = "Valid"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM IRQ96~127 wakeup status"]
pub mod CM_IRQ_WAKEUP_STAT_3 {
    pub use crate::RO as access;
    #[doc = "IRQ status"]
    pub mod IRQ_WAKEUP_STAT_96_127 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "None"]
            pub const B0: u32 = 0;
            #[doc = "Valid"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM IRQ128~159 wakeup status"]
pub mod CM_IRQ_WAKEUP_STAT_4 {
    pub use crate::RO as access;
    #[doc = "IRQ status"]
    pub mod IRQ_WAKEUP_STAT_128_159 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "None"]
            pub const B0: u32 = 0;
            #[doc = "Valid"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM IRQ160~191 wakeup status"]
pub mod CM_IRQ_WAKEUP_STAT_5 {
    pub use crate::RO as access;
    #[doc = "IRQ status"]
    pub mod IRQ_WAKEUP_STAT_160_191 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "None"]
            pub const B0: u32 = 0;
            #[doc = "Valid"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM IRQ192~223 wakeup status"]
pub mod CM_IRQ_WAKEUP_STAT_6 {
    pub use crate::RO as access;
    #[doc = "IRQ status"]
    pub mod IRQ_WAKEUP_STAT_192_223 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "None"]
            pub const B0: u32 = 0;
            #[doc = "Valid"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM IRQ224~255 wakeup status"]
pub mod CM_IRQ_WAKEUP_STAT_7 {
    pub use crate::RO as access;
    #[doc = "IRQ status"]
    pub mod IRQ_WAKEUP_MASK_224_255 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "None"]
            pub const B0: u32 = 0;
            #[doc = "Valid"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM non-irq wakeup status"]
pub mod CM_NON_IRQ_WAKEUP_STAT {
    pub use crate::RO as access;
    #[doc = "Event wakeup status"]
    pub mod EVENT_WAKEUP_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt is asserting (pending)"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Debug wakeup status"]
    pub mod DEBUG_WAKEUP_STAT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM sleep SSAR control"]
pub mod CM_SLEEP_SSAR_CTRL {
    pub use crate::RW as access;
    #[doc = "Step count, useage is depending on CNT_MODE."]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM sleep LPCG control"]
pub mod CM_SLEEP_LPCG_CTRL {
    pub use crate::RW as access;
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM sleep PLL control"]
pub mod CM_SLEEP_PLL_CTRL {
    pub use crate::RW as access;
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM sleep isolation control"]
pub mod CM_SLEEP_ISO_CTRL {
    pub use crate::RW as access;
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM sleep reset control"]
pub mod CM_SLEEP_RESET_CTRL {
    pub use crate::RW as access;
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM sleep power control"]
pub mod CM_SLEEP_POWER_CTRL {
    pub use crate::RW as access;
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM wakeup power control"]
pub mod CM_WAKEUP_POWER_CTRL {
    pub use crate::RW as access;
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM wakeup reset control"]
pub mod CM_WAKEUP_RESET_CTRL {
    pub use crate::RW as access;
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM wakeup isolation control"]
pub mod CM_WAKEUP_ISO_CTRL {
    pub use crate::RW as access;
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM wakeup PLL control"]
pub mod CM_WAKEUP_PLL_CTRL {
    pub use crate::RW as access;
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM wakeup LPCG control"]
pub mod CM_WAKEUP_LPCG_CTRL {
    pub use crate::RW as access;
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM wakeup SSAR control"]
pub mod CM_WAKEUP_SSAR_CTRL {
    pub use crate::RW as access;
    #[doc = "Step count, useage is depending on CNT_MODE"]
    pub mod STEP_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count mode"]
    pub mod CNT_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Counter disable mode: not use step counter, step completes once receiving step_done"]
            pub const B0: u32 = 0;
            #[doc = "Counter delay mode: delay after receiving step_done, delay cycle number is STEP_CNT"]
            pub const B1: u32 = 0x01;
            #[doc = "Ignore step_done response, the counter starts to count once step begins, when counter reaches STEP_CNT value, the step completes"]
            pub const B2: u32 = 0x02;
            #[doc = "Time out mode, the counter starts to count once step begins, the step completes when either step_done received or counting to STEP_CNT value"]
            pub const B3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable this step"]
    pub mod DISABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint Control"]
pub mod CM_SP_CTRL {
    pub use crate::RW as access;
    #[doc = "Request a Setpoint transition when this bit is set"]
    pub mod CPU_SP_RUN_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The Setpoint that CPU want the system to transit to when CPU_SP_RUN_EN is set"]
    pub mod CPU_SP_RUN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 means enable Setpoint transition on next CPU platform sleep sequence"]
    pub mod CPU_SP_SLEEP_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The Setpoint that CPU want the system to transit to on next CPU platform sleep sequence"]
    pub mod CPU_SP_SLEEP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 means enable Setpoint transition on next CPU platform wakeup sequence"]
    pub mod CPU_SP_WAKEUP_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The Setpoint that CPU want the system to transit to on next CPU platform wakeup sequence"]
    pub mod CPU_SP_WAKEUP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select the Setpoint transiton on the next CPU platform wakeup sequence"]
    pub mod CPU_SP_WAKEUP_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Request SP transition to CPU_SP_WAKEUP"]
            pub const B0: u32 = 0;
            #[doc = "Request SP transition to the Setpoint when the sleep event happens, which is captured in CPU_SP_PREVIOUS"]
            pub const B1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint Status"]
pub mod CM_SP_STAT {
    pub use crate::RO as access;
    #[doc = "The current Setpoint of the system"]
    pub mod CPU_SP_CURRENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The previous Setpoint of the system"]
    pub mod CPU_SP_PREVIOUS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The requested Setpoint from the CPU platform"]
    pub mod CPU_SP_TARGET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Run Mode Setpoint Allowed"]
pub mod CM_RUN_MODE_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines which Setpoint is allowed when CPU enters RUN mode. Each bit stands for 1 Setpoint, locked by LOCK_CFG field"]
    pub mod CPU_RUN_MODE_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Wait Mode Setpoint Allowed"]
pub mod CM_WAIT_MODE_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines which Setpoint is allowed when CPU enters WAIT mode. Each bit stands for 1 Setpoint, locked by LOCK_CFG"]
    pub mod CPU_WAIT_MODE_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Stop Mode Setpoint Allowed"]
pub mod CM_STOP_MODE_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines which Setpoint is allowed when CPU enters STOP mode. Each bit stands for 1 Setpoint, locked by LOCK_CFG"]
    pub mod CPU_STOP_MODE_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Suspend Mode Setpoint Allowed"]
pub mod CM_SUSPEND_MODE_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines which Setpoint is allowed when CPU enters SUSPEND mode. Each bit stands for 1 Setpoint, locked by LOCK_CFG"]
    pub mod CPU_SUSPEND_MODE_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 0 Mapping"]
pub mod CM_SP0_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP0 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP0_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 1 Mapping"]
pub mod CM_SP1_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP1 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP1_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 2 Mapping"]
pub mod CM_SP2_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP2 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP2_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 3 Mapping"]
pub mod CM_SP3_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP3 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP3_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 4 Mapping"]
pub mod CM_SP4_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP4 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP4_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 5 Mapping"]
pub mod CM_SP5_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP5 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP5_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 6 Mapping"]
pub mod CM_SP6_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP6 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP6_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 7 Mapping"]
pub mod CM_SP7_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP7 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP7_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 8 Mapping"]
pub mod CM_SP8_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP8 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP8_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 9 Mapping"]
pub mod CM_SP9_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP9 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP9_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 10 Mapping"]
pub mod CM_SP10_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP10 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP10_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 11 Mapping"]
pub mod CM_SP11_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP11 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP11_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 12 Mapping"]
pub mod CM_SP12_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP12 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP12_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 13 Mapping"]
pub mod CM_SP13_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP13 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP13_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 14 Mapping"]
pub mod CM_SP14_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP14 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP14_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM Setpoint 15 Mapping"]
pub mod CM_SP15_MAPPING {
    pub use crate::RW as access;
    #[doc = "Defines when SP15 is set as the CPU_SP_TARGET, which SP is allowed, locked by LOCK_CFG field"]
    pub mod CPU_SP15_MAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "CM standby control"]
pub mod CM_STBY_CTRL {
    pub use crate::RW as access;
    #[doc = "0x1: Request the chip into standby mode when CPU entering WAIT mode, locked by LOCK_CFG field."]
    pub mod STBY_WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0x1: Request the chip into standby mode when CPU entering STOP mode, locked by LOCK_CFG field."]
    pub mod STBY_STOP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0x1: Request the chip into standby mode when CPU entering SUSPEND mode, locked by LOCK_CFG field."]
    pub mod STBY_SUSPEND {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicate the CPU is busy entering standby mode."]
    pub mod STBY_SLEEP_BUSY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicate the CPU is busy exiting standby mode."]
    pub mod STBY_WAKEUP_BUSY {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
