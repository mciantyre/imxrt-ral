#[doc = "SRC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SRC Control Register"]
    pub SCR: u32,
    #[doc = "SRC Reset Mode Register"]
    pub SRMR: u32,
    #[doc = "SRC Boot Mode Register 1"]
    pub SBMR1: u32,
    #[doc = "SRC Boot Mode Register 2"]
    pub SBMR2: u32,
    #[doc = "SRC Reset Status Register"]
    pub SRSR: u32,
    #[doc = "SRC General Purpose Register"]
    pub GPR: [u32; 20usize],
    _reserved0: [u8; 0x019c],
    #[doc = "Slice Authentication Register"]
    pub AUTHEN_MEGA: u32,
    #[doc = "Slice Control Register"]
    pub CTRL_MEGA: u32,
    #[doc = "Slice Setpoint Config Register"]
    pub SETPOINT_MEGA: u32,
    #[doc = "Slice Domain Config Register"]
    pub DOMAIN_MEGA: u32,
    #[doc = "Slice Status Register"]
    pub STAT_MEGA: u32,
    _reserved1: [u8; 0x0c],
    #[doc = "Slice Authentication Register"]
    pub AUTHEN_DISPLAY: u32,
    #[doc = "Slice Control Register"]
    pub CTRL_DISPLAY: u32,
    #[doc = "Slice Setpoint Config Register"]
    pub SETPOINT_DISPLAY: u32,
    #[doc = "Slice Domain Config Register"]
    pub DOMAIN_DISPLAY: u32,
    #[doc = "Slice Status Register"]
    pub STAT_DISPLAY: u32,
    _reserved2: [u8; 0x0c],
    #[doc = "Slice Authentication Register"]
    pub AUTHEN_WAKEUP: u32,
    #[doc = "Slice Control Register"]
    pub CTRL_WAKEUP: u32,
    #[doc = "Slice Setpoint Config Register"]
    pub SETPOINT_WAKEUP: u32,
    #[doc = "Slice Domain Config Register"]
    pub DOMAIN_WAKEUP: u32,
    #[doc = "Slice Status Register"]
    pub STAT_WAKEUP: u32,
    _reserved3: [u8; 0x2c],
    #[doc = "Slice Authentication Register"]
    pub AUTHEN_M4CORE: u32,
    #[doc = "Slice Control Register"]
    pub CTRL_M4CORE: u32,
    #[doc = "Slice Setpoint Config Register"]
    pub SETPOINT_M4CORE: u32,
    #[doc = "Slice Domain Config Register"]
    pub DOMAIN_M4CORE: u32,
    #[doc = "Slice Status Register"]
    pub STAT_M4CORE: u32,
    _reserved4: [u8; 0x0c],
    #[doc = "Slice Authentication Register"]
    pub AUTHEN_M7CORE: u32,
    #[doc = "Slice Control Register"]
    pub CTRL_M7CORE: u32,
    #[doc = "Slice Setpoint Config Register"]
    pub SETPOINT_M7CORE: u32,
    #[doc = "Slice Domain Config Register"]
    pub DOMAIN_M7CORE: u32,
    #[doc = "Slice Status Register"]
    pub STAT_M7CORE: u32,
    _reserved5: [u8; 0x0c],
    #[doc = "Slice Authentication Register"]
    pub AUTHEN_M4DEBUG: u32,
    #[doc = "Slice Control Register"]
    pub CTRL_M4DEBUG: u32,
    #[doc = "Slice Setpoint Config Register"]
    pub SETPOINT_M4DEBUG: u32,
    #[doc = "Slice Domain Config Register"]
    pub DOMAIN_M4DEBUG: u32,
    #[doc = "Slice Status Register"]
    pub STAT_M4DEBUG: u32,
    _reserved6: [u8; 0x0c],
    #[doc = "Slice Authentication Register"]
    pub AUTHEN_M7DEBUG: u32,
    #[doc = "Slice Control Register"]
    pub CTRL_M7DEBUG: u32,
    #[doc = "Slice Setpoint Config Register"]
    pub SETPOINT_M7DEBUG: u32,
    #[doc = "Slice Domain Config Register"]
    pub DOMAIN_M7DEBUG: u32,
    #[doc = "Slice Status Register"]
    pub STAT_M7DEBUG: u32,
    _reserved7: [u8; 0x0c],
    #[doc = "Slice Authentication Register"]
    pub AUTHEN_USBPHY1: u32,
    #[doc = "Slice Control Register"]
    pub CTRL_USBPHY1: u32,
    #[doc = "Slice Setpoint Config Register"]
    pub SETPOINT_USBPHY1: u32,
    #[doc = "Slice Domain Config Register"]
    pub DOMAIN_USBPHY1: u32,
    #[doc = "Slice Status Register"]
    pub STAT_USBPHY1: u32,
    _reserved8: [u8; 0x0c],
    #[doc = "Slice Authentication Register"]
    pub AUTHEN_USBPHY2: u32,
    #[doc = "Slice Control Register"]
    pub CTRL_USBPHY2: u32,
    #[doc = "Slice Setpoint Config Register"]
    pub SETPOINT_USBPHY2: u32,
    #[doc = "Slice Domain Config Register"]
    pub DOMAIN_USBPHY2: u32,
    #[doc = "Slice Status Register"]
    pub STAT_USBPHY2: u32,
}
#[doc = "SRC Control Register"]
pub mod SCR {
    pub use crate::RW as access;
    #[doc = "cm4 core reset will be held until boot core write this bit to 1 to release it."]
    pub mod BT_RELEASE_M4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "cm4 core reset is asserted"]
            pub const BT_RELEASE_M4_0: u32 = 0;
            #[doc = "cm4 core reset is released"]
            pub const BT_RELEASE_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "cm7 core reset will be held until boot core write this bit to 1 to release it."]
    pub mod BT_RELEASE_M7 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "cm7 core reset is asserted"]
            pub const BT_RELEASE_M7_0: u32 = 0;
            #[doc = "cm7 core reset is released"]
            pub const BT_RELEASE_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SRC Reset Mode Register"]
pub mod SRMR {
    pub use crate::RW as access;
    #[doc = "Wdog reset mode configuration"]
    pub mod WDOG_RESET_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "reset system"]
            pub const WDOG_RESET_MODE_0: u32 = 0;
            #[doc = "do not reset anything"]
            pub const WDOG_RESET_MODE_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Wdog3 reset mode configuration"]
    pub mod WDOG3_RESET_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "reset system"]
            pub const WDOG3_RESET_MODE_0: u32 = 0;
            #[doc = "do not reset anything"]
            pub const WDOG3_RESET_MODE_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Wdog4 reset mode configuration"]
    pub mod WDOG4_RESET_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "reset system"]
            pub const WDOG4_RESET_MODE_0: u32 = 0;
            #[doc = "do not reset anything"]
            pub const WDOG4_RESET_MODE_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "M4 core lockup reset mode configuration"]
    pub mod M4LOCKUP_RESET_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "reset system"]
            pub const M4LOCKUP_RESET_MODE_0: u32 = 0;
            #[doc = "do not reset anything"]
            pub const M4LOCKUP_RESET_MODE_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "M7 core lockup reset mode configuration"]
    pub mod M7LOCKUP_RESET_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "reset system"]
            pub const M7LOCKUP_RESET_MODE_0: u32 = 0;
            #[doc = "do not reset anything"]
            pub const M7LOCKUP_RESET_MODE_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "M4 request reset configuration"]
    pub mod M4REQ_RESET_MODE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "reset system"]
            pub const M4REQ_RESET_MODE_0: u32 = 0;
            #[doc = "do not reset anything"]
            pub const M4REQ_RESET_MODE_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "M7 request reset configuration"]
    pub mod M7REQ_RESET_MODE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "reset system"]
            pub const M7REQ_RESET_MODE_0: u32 = 0;
            #[doc = "do not reset anything"]
            pub const M7REQ_RESET_MODE_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Tempsense reset mode configuration"]
    pub mod TEMPSENSE_RESET_MODE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "reset system"]
            pub const TEMPSENSE_RESET_MODE_0: u32 = 0;
            #[doc = "do not reset anything"]
            pub const TEMPSENSE_RESET_MODE_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CSU reset mode configuration"]
    pub mod CSU_RESET_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "reset system"]
            pub const CSU_RESET_MODE_0: u32 = 0;
            #[doc = "do not reset anything"]
            pub const CSU_RESET_MODE_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Jtag SW reset mode configuration"]
    pub mod JTAGSW_RESET_MODE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "reset system"]
            pub const JTAGSW_RESET_MODE_0: u32 = 0;
            #[doc = "do not reset anything"]
            pub const JTAGSW_RESET_MODE_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Jtag SW reset mode configuration"]
    pub mod OVERVOLT_RESET_MODE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "reset system"]
            pub const OVERVOLT_RESET_MODE_0: u32 = 0;
            #[doc = "do not reset anything"]
            pub const OVERVOLT_RESET_MODE_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SRC Boot Mode Register 1"]
pub mod SBMR1 {
    pub use crate::RO as access;
    #[doc = "Refer to fusemap."]
    pub mod BOOT_CFG1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Refer to fusemap."]
    pub mod BOOT_CFG2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Refer to fusemap."]
    pub mod BOOT_CFG3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Refer to fusemap."]
    pub mod BOOT_CFG4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SRC Boot Mode Register 2"]
pub mod SBMR2 {
    pub use crate::RO as access;
    #[doc = "SECONFIG\\[1\\] shows the state of the SECONFIG\\[1\\] fuse"]
    pub mod SEC_CONFIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BT_FUSE_SEL shows the state of the BT_FUSE_SEL fuse"]
    pub mod BT_FUSE_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BMOD\\[1:0\\] shows the latched state of the BOOT_MODE1 and BOOT_MODE0 signals on the rising edge of POR_B"]
    pub mod BMOD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SRC Reset Status Register"]
pub mod SRSR {
    pub use crate::RW as access;
    #[doc = "Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
    pub mod IPP_RESET_B_M7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of ipp_reset_b pin."]
            pub const IPP_RESET_B_M7_0: u32 = 0;
            #[doc = "Reset is a result of ipp_reset_b pin."]
            pub const IPP_RESET_B_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates whether reset was the result of m7 reset request"]
    pub mod M7_REQUEST_M7 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of m7 reset request."]
            pub const M7_REQUEST_M7_0: u32 = 0;
            #[doc = "Reset is a result of m7 reset request."]
            pub const M7_REQUEST_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates a reset has been caused by M7 CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core"]
    pub mod M7_LOCKUP_M7 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the mentioned case."]
            pub const M7_LOCKUP_M7_0: u32 = 0;
            #[doc = "Reset is a result of the mentioned case."]
            pub const M7_LOCKUP_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates whether the reset was the result of the csu_reset_b input."]
    pub mod CSU_RESET_B_M7 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the csu_reset_b event."]
            pub const CSU_RESET_B_M7_0: u32 = 0;
            #[doc = "Reset is a result of the csu_reset_b event."]
            pub const CSU_RESET_B_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
    pub mod IPP_USER_RESET_B_M7 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the ipp_user_reset_b qualified as COLD reset event."]
            pub const IPP_USER_RESET_B_M7_0: u32 = 0;
            #[doc = "Reset is a result of the ipp_user_reset_b qualified as COLD reset event."]
            pub const IPP_USER_RESET_B_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IC Watchdog Time-out reset"]
    pub mod WDOG_RST_B_M7 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the watchdog time-out event."]
            pub const WDOG_RST_B_M7_0: u32 = 0;
            #[doc = "Reset is a result of the watchdog time-out event."]
            pub const WDOG_RST_B_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
    pub mod JTAG_RST_B_M7 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of HIGH-Z reset from JTAG."]
            pub const JTAG_RST_B_M7_0: u32 = 0;
            #[doc = "Reset is a result of HIGH-Z reset from JTAG."]
            pub const JTAG_RST_B_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "JTAG software reset. Indicates whether the reset was the result of software reset from JTAG."]
    pub mod JTAG_SW_RST_M7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of software reset from JTAG."]
            pub const JTAG_SW_RST_M7_0: u32 = 0;
            #[doc = "Reset is a result of software reset from JTAG."]
            pub const JTAG_SW_RST_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IC Watchdog3 Time-out reset"]
    pub mod WDOG3_RST_B_M7 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the watchdog3 time-out event."]
            pub const WDOG3_RST_B_M7_0: u32 = 0;
            #[doc = "Reset is a result of the watchdog3 time-out event."]
            pub const WDOG3_RST_B_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IC Watchdog4 Time-out reset"]
    pub mod WDOG4_RST_B_M7 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the watchdog4 time-out event."]
            pub const WDOG4_RST_B_M7_0: u32 = 0;
            #[doc = "Reset is a result of the watchdog4 time-out event."]
            pub const WDOG4_RST_B_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Temper Sensor software reset"]
    pub mod TEMPSENSE_RST_B_M7 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of software reset from Temperature Sensor."]
            pub const TEMPSENSE_RST_B_M7_0: u32 = 0;
            #[doc = "Reset is a result of software reset from Temperature Sensor."]
            pub const TEMPSENSE_RST_B_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates whether reset was the result of m4 reset request."]
    pub mod M4_REQUEST_M7 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of m4 reset request."]
            pub const M4_REQUEST_M7_0: u32 = 0;
            #[doc = "Reset is a result of m4 reset request."]
            pub const M4_REQUEST_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates a reset has been caused by M4 CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core"]
    pub mod M4_LOCKUP_M7 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the mentioned case."]
            pub const M4_LOCKUP_M7_0: u32 = 0;
            #[doc = "Reset is a result of the mentioned case."]
            pub const M4_LOCKUP_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates a reset has been caused by power suppy voltage over the highest permitted level."]
    pub mod OVERVOLT_RST_M7 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the mentioned case."]
            pub const OVERVOLT_RST_M7_0: u32 = 0;
            #[doc = "Reset is a result of the mentioned case."]
            pub const OVERVOLT_RST_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates a reset has been caused by CDOG reset."]
    pub mod CDOG_RST_M7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the mentioned case."]
            pub const CDOG_RST_M7_0: u32 = 0;
            #[doc = "Reset is a result of the mentioned case."]
            pub const CDOG_RST_M7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
    pub mod IPP_RESET_B_M4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of ipp_reset_b pin."]
            pub const IPP_RESET_B_M4_0: u32 = 0;
            #[doc = "Reset is a result of ipp_reset_b pin."]
            pub const IPP_RESET_B_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates whether reset was the result of m4 reset request"]
    pub mod M4_REQUEST_M4 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of m4 reset request."]
            pub const M4_REQUEST_M4_0: u32 = 0;
            #[doc = "Reset is a result of m4 reset request."]
            pub const M4_REQUEST_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates a reset has been caused by M4 CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core"]
    pub mod M4_LOCKUP_M4 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the mentioned case."]
            pub const M4_LOCKUP_M4_0: u32 = 0;
            #[doc = "Reset is a result of the mentioned case."]
            pub const M4_LOCKUP_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates whether the reset was the result of the csu_reset_b input."]
    pub mod CSU_RESET_B_M4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the csu_reset_b event."]
            pub const CSU_RESET_B_M4_0: u32 = 0;
            #[doc = "Reset is a result of the csu_reset_b event."]
            pub const CSU_RESET_B_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
    pub mod IPP_USER_RESET_B_M4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the ipp_user_reset_b qualified as COLD reset event."]
            pub const IPP_USER_RESET_B_M4_0: u32 = 0;
            #[doc = "Reset is a result of the ipp_user_reset_b qualified as COLD reset event."]
            pub const IPP_USER_RESET_B_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IC Watchdog Time-out reset"]
    pub mod WDOG_RST_B_M4 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the watchdog time-out event."]
            pub const WDOG_RST_B_M4_0: u32 = 0;
            #[doc = "Reset is a result of the watchdog time-out event."]
            pub const WDOG_RST_B_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
    pub mod JTAG_RST_B_M4 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of HIGH-Z reset from JTAG."]
            pub const JTAG_RST_B_M4_0: u32 = 0;
            #[doc = "Reset is a result of HIGH-Z reset from JTAG."]
            pub const JTAG_RST_B_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "JTAG software reset. Indicates whether the reset was the result of software reset from JTAG."]
    pub mod JTAG_SW_RST_M4 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of software reset from JTAG."]
            pub const JTAG_SW_RST_M4_0: u32 = 0;
            #[doc = "Reset is a result of software reset from JTAG."]
            pub const JTAG_SW_RST_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IC Watchdog3 Time-out reset"]
    pub mod WDOG3_RST_B_M4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the watchdog3 time-out event."]
            pub const WDOG3_RST_B_M4_0: u32 = 0;
            #[doc = "Reset is a result of the watchdog3 time-out event."]
            pub const WDOG3_RST_B_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IC Watchdog4 Time-out reset"]
    pub mod WDOG4_RST_B_M4 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the watchdog4 time-out event."]
            pub const WDOG4_RST_B_M4_0: u32 = 0;
            #[doc = "Reset is a result of the watchdog4 time-out event."]
            pub const WDOG4_RST_B_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Temper Sensor software reset"]
    pub mod TEMPSENSE_RST_B_M4 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of software reset from Temperature Sensor."]
            pub const TEMPSENSE_RST_B_M4_0: u32 = 0;
            #[doc = "Reset is a result of software reset from Temperature Sensor."]
            pub const TEMPSENSE_RST_B_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates whether reset was the result of m7 reset request."]
    pub mod M7_REQUEST_M4 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of m7 reset request."]
            pub const M7_REQUEST_M4_0: u32 = 0;
            #[doc = "Reset is a result of m7 reset request."]
            pub const M7_REQUEST_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates a reset has been caused by M7 CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core"]
    pub mod M7_LOCKUP_M4 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the mentioned case."]
            pub const M7_LOCKUP_M4_0: u32 = 0;
            #[doc = "Reset is a result of the mentioned case."]
            pub const M7_LOCKUP_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates a reset has been caused by power suppy voltage over the highest permitted level."]
    pub mod OVERVOLT_RST_M4 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the mentioned case."]
            pub const OVERVOLT_RST_M4_0: u32 = 0;
            #[doc = "Reset is a result of the mentioned case."]
            pub const OVERVOLT_RST_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates a reset has been caused by CDOG reset."]
    pub mod CDOG_RST_M4 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not a result of the mentioned case."]
            pub const CDOG_RST_M4_0: u32 = 0;
            #[doc = "Reset is a result of the mentioned case."]
            pub const CDOG_RST_M4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SRC General Purpose Register"]
pub mod GPR {
    pub use crate::RW as access;
    #[doc = "General Purpose Register."]
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
#[doc = "Slice Authentication Register"]
pub mod AUTHEN_MEGA {
    pub use crate::RW as access;
    #[doc = "Control whether reset slice is in domain mode"]
    pub mod DOMAIN_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by CPU power mode transition"]
            pub const DOMAIN_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by CPU power mode transition. Do not set this bit and SETPOINT_MODE at the same time."]
            pub const DOMAIN_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control whether reset slice is in Setpoint mode"]
    pub mod SETPOINT_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by Setpoint transition"]
            pub const SETPOINT_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by Setpoint transition. Do not set this bit and DOMAIN_MODE at the same time."]
            pub const SETPOINT_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain/Setpoint mode lock"]
    pub mod LOCK_MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "when this bitfield set to 1, reset of slice would be subject to corresponding core status transition"]
    pub mod ASSIGN_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Assign list lock"]
    pub mod LOCK_ASSIGN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock NONSECURE and USER"]
    pub mod LOCK_SETTING {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Control Register"]
pub mod CTRL_MEGA {
    pub use crate::RW as access;
    #[doc = "This is a self clearing bit"]
    pub mod SW_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "do not assert slice software reset"]
            pub const SW_RESET_0: u32 = 0;
            #[doc = "assert slice software reset"]
            pub const SW_RESET_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Setpoint Config Register"]
pub mod SETPOINT_MEGA {
    pub use crate::RW as access;
    #[doc = "SETPOINT0"]
    pub mod SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT0_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT1"]
    pub mod SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT1_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT2"]
    pub mod SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT2_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT3"]
    pub mod SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT3_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT4"]
    pub mod SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT4_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT5"]
    pub mod SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT5_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT5_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT6"]
    pub mod SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT6_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT6_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT7"]
    pub mod SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT7_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT8"]
    pub mod SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT8_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT8_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT9"]
    pub mod SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT9_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT9_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT10"]
    pub mod SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT10_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT10_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT11"]
    pub mod SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT11_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT11_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT12"]
    pub mod SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT12_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT12_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT13"]
    pub mod SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT13_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT13_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT14"]
    pub mod SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT14_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT14_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT15"]
    pub mod SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT15_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT15_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Domain Config Register"]
pub mod DOMAIN_MEGA {
    pub use crate::RW as access;
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU0_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU0_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU0_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU0_SUSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU1_RUN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU1_WAIT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU1_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU1_SUSP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Status Register"]
pub mod STAT_MEGA {
    pub use crate::RW as access;
    #[doc = "This is a Read Only bit. It indicate if the reset is in process."]
    pub mod UNDER_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is finished"]
            pub const UNDER_RST_0: u32 = 0;
            #[doc = "the reset is in process"]
            pub const UNDER_RST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by the power mode transfer."]
    pub mod RST_BY_HW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by the power mode transfer"]
            pub const RST_BY_HW_0: u32 = 0;
            #[doc = "the reset is caused by the power mode transfer"]
            pub const RST_BY_HW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by setting SW_RESET bit."]
    pub mod RST_BY_SW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by software setting"]
            pub const RST_BY_SW_0: u32 = 0;
            #[doc = "the reset is caused by software setting"]
            pub const RST_BY_SW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Authentication Register"]
pub mod AUTHEN_DISPLAY {
    pub use crate::RW as access;
    #[doc = "Control whether reset slice is in domain mode"]
    pub mod DOMAIN_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by CPU power mode transition"]
            pub const DOMAIN_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by CPU power mode transition. Do not set this bit and SETPOINT_MODE at the same time."]
            pub const DOMAIN_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control whether reset slice is in Setpoint mode"]
    pub mod SETPOINT_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by Setpoint transition"]
            pub const SETPOINT_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by Setpoint transition. Do not set this bit and DOMAIN_MODE at the same time."]
            pub const SETPOINT_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain/Setpoint mode lock"]
    pub mod LOCK_MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "when this bitfield set to 1, reset of slice would be subject to corresponding core status transition"]
    pub mod ASSIGN_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Assign list lock"]
    pub mod LOCK_ASSIGN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock NONSECURE and USER"]
    pub mod LOCK_SETTING {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Control Register"]
pub mod CTRL_DISPLAY {
    pub use crate::RW as access;
    #[doc = "This is a self clearing bit"]
    pub mod SW_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "do not assert slice software reset"]
            pub const SW_RESET_0: u32 = 0;
            #[doc = "assert slice software reset"]
            pub const SW_RESET_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Setpoint Config Register"]
pub mod SETPOINT_DISPLAY {
    pub use crate::RW as access;
    #[doc = "SETPOINT0"]
    pub mod SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT0_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT1"]
    pub mod SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT1_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT2"]
    pub mod SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT2_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT3"]
    pub mod SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT3_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT4"]
    pub mod SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT4_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT5"]
    pub mod SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT5_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT5_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT6"]
    pub mod SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT6_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT6_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT7"]
    pub mod SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT7_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT8"]
    pub mod SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT8_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT8_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT9"]
    pub mod SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT9_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT9_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT10"]
    pub mod SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT10_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT10_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT11"]
    pub mod SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT11_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT11_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT12"]
    pub mod SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT12_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT12_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT13"]
    pub mod SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT13_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT13_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT14"]
    pub mod SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT14_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT14_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT15"]
    pub mod SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT15_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT15_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Domain Config Register"]
pub mod DOMAIN_DISPLAY {
    pub use crate::RW as access;
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU0_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU0_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU0_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU0_SUSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU1_RUN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU1_WAIT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU1_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU1_SUSP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Status Register"]
pub mod STAT_DISPLAY {
    pub use crate::RW as access;
    #[doc = "This is a Read Only bit. It indicate if the reset is in process."]
    pub mod UNDER_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is finished"]
            pub const UNDER_RST_0: u32 = 0;
            #[doc = "the reset is in process"]
            pub const UNDER_RST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by the power mode transfer."]
    pub mod RST_BY_HW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by the power mode transfer"]
            pub const RST_BY_HW_0: u32 = 0;
            #[doc = "the reset is caused by the power mode transfer"]
            pub const RST_BY_HW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by setting SW_RESET bit."]
    pub mod RST_BY_SW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by software setting"]
            pub const RST_BY_SW_0: u32 = 0;
            #[doc = "the reset is caused by software setting"]
            pub const RST_BY_SW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Authentication Register"]
pub mod AUTHEN_WAKEUP {
    pub use crate::RW as access;
    #[doc = "Control whether reset slice is in domain mode"]
    pub mod DOMAIN_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by CPU power mode transition"]
            pub const DOMAIN_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by CPU power mode transition. Do not set this bit and SETPOINT_MODE at the same time."]
            pub const DOMAIN_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control whether reset slice is in Setpoint mode"]
    pub mod SETPOINT_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by Setpoint transition"]
            pub const SETPOINT_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by Setpoint transition. Do not set this bit and DOMAIN_MODE at the same time."]
            pub const SETPOINT_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain/Setpoint mode lock"]
    pub mod LOCK_MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "when this bitfield set to 1, reset of slice would be subject to corresponding core status transition"]
    pub mod ASSIGN_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Assign list lock"]
    pub mod LOCK_ASSIGN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock NONSECURE and USER"]
    pub mod LOCK_SETTING {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Control Register"]
pub mod CTRL_WAKEUP {
    pub use crate::RW as access;
    #[doc = "This is a self clearing bit"]
    pub mod SW_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "do not assert slice software reset"]
            pub const SW_RESET_0: u32 = 0;
            #[doc = "assert slice software reset"]
            pub const SW_RESET_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Setpoint Config Register"]
pub mod SETPOINT_WAKEUP {
    pub use crate::RW as access;
    #[doc = "SETPOINT0"]
    pub mod SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT0_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT1"]
    pub mod SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT1_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT2"]
    pub mod SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT2_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT3"]
    pub mod SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT3_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT4"]
    pub mod SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT4_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT5"]
    pub mod SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT5_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT5_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT6"]
    pub mod SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT6_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT6_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT7"]
    pub mod SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT7_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT8"]
    pub mod SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT8_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT8_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT9"]
    pub mod SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT9_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT9_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT10"]
    pub mod SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT10_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT10_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT11"]
    pub mod SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT11_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT11_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT12"]
    pub mod SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT12_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT12_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT13"]
    pub mod SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT13_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT13_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT14"]
    pub mod SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT14_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT14_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT15"]
    pub mod SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT15_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT15_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Domain Config Register"]
pub mod DOMAIN_WAKEUP {
    pub use crate::RW as access;
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU0_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU0_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU0_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU0_SUSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU1_RUN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU1_WAIT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU1_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU1_SUSP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Status Register"]
pub mod STAT_WAKEUP {
    pub use crate::RW as access;
    #[doc = "This is a Read Only bit. It indicate if the reset is in process."]
    pub mod UNDER_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is finished"]
            pub const UNDER_RST_0: u32 = 0;
            #[doc = "the reset is in process"]
            pub const UNDER_RST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by the power mode transfer."]
    pub mod RST_BY_HW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by the power mode transfer"]
            pub const RST_BY_HW_0: u32 = 0;
            #[doc = "the reset is caused by the power mode transfer"]
            pub const RST_BY_HW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by setting SW_RESET bit."]
    pub mod RST_BY_SW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by software setting"]
            pub const RST_BY_SW_0: u32 = 0;
            #[doc = "the reset is caused by software setting"]
            pub const RST_BY_SW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Authentication Register"]
pub mod AUTHEN_M4CORE {
    pub use crate::RW as access;
    #[doc = "Control whether reset slice is in domain mode"]
    pub mod DOMAIN_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by CPU power mode transition"]
            pub const DOMAIN_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by CPU power mode transition. Do not set this bit and SETPOINT_MODE at the same time."]
            pub const DOMAIN_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control whether reset slice is in Setpoint mode"]
    pub mod SETPOINT_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by Setpoint transition"]
            pub const SETPOINT_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by Setpoint transition. Do not set this bit and DOMAIN_MODE at the same time."]
            pub const SETPOINT_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain/Setpoint mode lock"]
    pub mod LOCK_MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "when this bitfield set to 1, reset of slice would be subject to corresponding core status transition"]
    pub mod ASSIGN_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Assign list lock"]
    pub mod LOCK_ASSIGN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock NONSECURE and USER"]
    pub mod LOCK_SETTING {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Control Register"]
pub mod CTRL_M4CORE {
    pub use crate::RW as access;
    #[doc = "This is a self clearing bit"]
    pub mod SW_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "do not assert slice software reset"]
            pub const SW_RESET_0: u32 = 0;
            #[doc = "assert slice software reset"]
            pub const SW_RESET_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Setpoint Config Register"]
pub mod SETPOINT_M4CORE {
    pub use crate::RW as access;
    #[doc = "SETPOINT0"]
    pub mod SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT0_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT1"]
    pub mod SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT1_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT2"]
    pub mod SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT2_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT3"]
    pub mod SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT3_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT4"]
    pub mod SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT4_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT5"]
    pub mod SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT5_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT5_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT6"]
    pub mod SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT6_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT6_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT7"]
    pub mod SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT7_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT8"]
    pub mod SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT8_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT8_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT9"]
    pub mod SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT9_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT9_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT10"]
    pub mod SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT10_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT10_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT11"]
    pub mod SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT11_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT11_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT12"]
    pub mod SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT12_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT12_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT13"]
    pub mod SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT13_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT13_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT14"]
    pub mod SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT14_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT14_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT15"]
    pub mod SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT15_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT15_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Domain Config Register"]
pub mod DOMAIN_M4CORE {
    pub use crate::RW as access;
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU0_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU0_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU0_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU0_SUSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU1_RUN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU1_WAIT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU1_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU1_SUSP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Status Register"]
pub mod STAT_M4CORE {
    pub use crate::RW as access;
    #[doc = "This is a Read Only bit. It indicate if the reset is in process."]
    pub mod UNDER_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is finished"]
            pub const UNDER_RST_0: u32 = 0;
            #[doc = "the reset is in process"]
            pub const UNDER_RST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by the power mode transfer."]
    pub mod RST_BY_HW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by the power mode transfer"]
            pub const RST_BY_HW_0: u32 = 0;
            #[doc = "the reset is caused by the power mode transfer"]
            pub const RST_BY_HW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by setting SW_RESET bit."]
    pub mod RST_BY_SW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by software setting"]
            pub const RST_BY_SW_0: u32 = 0;
            #[doc = "the reset is caused by software setting"]
            pub const RST_BY_SW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Authentication Register"]
pub mod AUTHEN_M7CORE {
    pub use crate::RW as access;
    #[doc = "Control whether reset slice is in domain mode"]
    pub mod DOMAIN_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by CPU power mode transition"]
            pub const DOMAIN_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by CPU power mode transition. Do not set this bit and SETPOINT_MODE at the same time."]
            pub const DOMAIN_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control whether reset slice is in Setpoint mode"]
    pub mod SETPOINT_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by Setpoint transition"]
            pub const SETPOINT_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by Setpoint transition. Do not set this bit and DOMAIN_MODE at the same time."]
            pub const SETPOINT_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain/Setpoint mode lock"]
    pub mod LOCK_MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "when this bitfield set to 1, reset of slice would be subject to corresponding core status transition"]
    pub mod ASSIGN_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Assign list lock"]
    pub mod LOCK_ASSIGN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock NONSECURE and USER"]
    pub mod LOCK_SETTING {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Control Register"]
pub mod CTRL_M7CORE {
    pub use crate::RW as access;
    #[doc = "This is a self clearing bit"]
    pub mod SW_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "do not assert slice software reset"]
            pub const SW_RESET_0: u32 = 0;
            #[doc = "assert slice software reset"]
            pub const SW_RESET_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Setpoint Config Register"]
pub mod SETPOINT_M7CORE {
    pub use crate::RW as access;
    #[doc = "SETPOINT0"]
    pub mod SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT0_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT1"]
    pub mod SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT1_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT2"]
    pub mod SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT2_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT3"]
    pub mod SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT3_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT4"]
    pub mod SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT4_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT5"]
    pub mod SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT5_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT5_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT6"]
    pub mod SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT6_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT6_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT7"]
    pub mod SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT7_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT8"]
    pub mod SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT8_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT8_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT9"]
    pub mod SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT9_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT9_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT10"]
    pub mod SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT10_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT10_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT11"]
    pub mod SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT11_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT11_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT12"]
    pub mod SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT12_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT12_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT13"]
    pub mod SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT13_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT13_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT14"]
    pub mod SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT14_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT14_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT15"]
    pub mod SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT15_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT15_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Domain Config Register"]
pub mod DOMAIN_M7CORE {
    pub use crate::RW as access;
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU0_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU0_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU0_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU0_SUSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU1_RUN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU1_WAIT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU1_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU1_SUSP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Status Register"]
pub mod STAT_M7CORE {
    pub use crate::RW as access;
    #[doc = "This is a Read Only bit. It indicate if the reset is in process."]
    pub mod UNDER_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is finished"]
            pub const UNDER_RST_0: u32 = 0;
            #[doc = "the reset is in process"]
            pub const UNDER_RST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by the power mode transfer."]
    pub mod RST_BY_HW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by the power mode transfer"]
            pub const RST_BY_HW_0: u32 = 0;
            #[doc = "the reset is caused by the power mode transfer"]
            pub const RST_BY_HW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by setting SW_RESET bit."]
    pub mod RST_BY_SW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by software setting"]
            pub const RST_BY_SW_0: u32 = 0;
            #[doc = "the reset is caused by software setting"]
            pub const RST_BY_SW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Authentication Register"]
pub mod AUTHEN_M4DEBUG {
    pub use crate::RW as access;
    #[doc = "Control whether reset slice is in domain mode"]
    pub mod DOMAIN_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by CPU power mode transition"]
            pub const DOMAIN_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by CPU power mode transition. Do not set this bit and SETPOINT_MODE at the same time."]
            pub const DOMAIN_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control whether reset slice is in Setpoint mode"]
    pub mod SETPOINT_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by Setpoint transition"]
            pub const SETPOINT_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by Setpoint transition. Do not set this bit and DOMAIN_MODE at the same time."]
            pub const SETPOINT_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain/Setpoint mode lock"]
    pub mod LOCK_MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "when this bitfield set to 1, reset of slice would be subject to corresponding core status transition"]
    pub mod ASSIGN_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Assign list lock"]
    pub mod LOCK_ASSIGN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock NONSECURE and USER"]
    pub mod LOCK_SETTING {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Control Register"]
pub mod CTRL_M4DEBUG {
    pub use crate::RW as access;
    #[doc = "This is a self clearing bit"]
    pub mod SW_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "do not assert slice software reset"]
            pub const SW_RESET_0: u32 = 0;
            #[doc = "assert slice software reset"]
            pub const SW_RESET_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Setpoint Config Register"]
pub mod SETPOINT_M4DEBUG {
    pub use crate::RW as access;
    #[doc = "SETPOINT0"]
    pub mod SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT0_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT1"]
    pub mod SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT1_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT2"]
    pub mod SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT2_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT3"]
    pub mod SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT3_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT4"]
    pub mod SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT4_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT5"]
    pub mod SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT5_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT5_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT6"]
    pub mod SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT6_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT6_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT7"]
    pub mod SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT7_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT8"]
    pub mod SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT8_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT8_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT9"]
    pub mod SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT9_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT9_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT10"]
    pub mod SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT10_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT10_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT11"]
    pub mod SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT11_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT11_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT12"]
    pub mod SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT12_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT12_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT13"]
    pub mod SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT13_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT13_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT14"]
    pub mod SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT14_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT14_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT15"]
    pub mod SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT15_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT15_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Domain Config Register"]
pub mod DOMAIN_M4DEBUG {
    pub use crate::RW as access;
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU0_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU0_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU0_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU0_SUSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU1_RUN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU1_WAIT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU1_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU1_SUSP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Status Register"]
pub mod STAT_M4DEBUG {
    pub use crate::RW as access;
    #[doc = "This is a Read Only bit. It indicate if the reset is in process."]
    pub mod UNDER_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is finished"]
            pub const UNDER_RST_0: u32 = 0;
            #[doc = "the reset is in process"]
            pub const UNDER_RST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by the power mode transfer."]
    pub mod RST_BY_HW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by the power mode transfer"]
            pub const RST_BY_HW_0: u32 = 0;
            #[doc = "the reset is caused by the power mode transfer"]
            pub const RST_BY_HW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by setting SW_RESET bit."]
    pub mod RST_BY_SW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by software setting"]
            pub const RST_BY_SW_0: u32 = 0;
            #[doc = "the reset is caused by software setting"]
            pub const RST_BY_SW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Authentication Register"]
pub mod AUTHEN_M7DEBUG {
    pub use crate::RW as access;
    #[doc = "Control whether reset slice is in domain mode"]
    pub mod DOMAIN_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by CPU power mode transition"]
            pub const DOMAIN_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by CPU power mode transition. Do not set this bit and SETPOINT_MODE at the same time."]
            pub const DOMAIN_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control whether reset slice is in Setpoint mode"]
    pub mod SETPOINT_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by Setpoint transition"]
            pub const SETPOINT_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by Setpoint transition. Do not set this bit and DOMAIN_MODE at the same time."]
            pub const SETPOINT_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain/Setpoint mode lock"]
    pub mod LOCK_MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "when this bitfield set to 1, reset of slice would be subject to corresponding core status transition"]
    pub mod ASSIGN_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Assign list lock"]
    pub mod LOCK_ASSIGN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock NONSECURE and USER"]
    pub mod LOCK_SETTING {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Control Register"]
pub mod CTRL_M7DEBUG {
    pub use crate::RW as access;
    #[doc = "This is a self clearing bit"]
    pub mod SW_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "do not assert slice software reset"]
            pub const SW_RESET_0: u32 = 0;
            #[doc = "assert slice software reset"]
            pub const SW_RESET_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Setpoint Config Register"]
pub mod SETPOINT_M7DEBUG {
    pub use crate::RW as access;
    #[doc = "SETPOINT0"]
    pub mod SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT0_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT1"]
    pub mod SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT1_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT2"]
    pub mod SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT2_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT3"]
    pub mod SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT3_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT4"]
    pub mod SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT4_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT5"]
    pub mod SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT5_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT5_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT6"]
    pub mod SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT6_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT6_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT7"]
    pub mod SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT7_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT8"]
    pub mod SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT8_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT8_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT9"]
    pub mod SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT9_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT9_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT10"]
    pub mod SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT10_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT10_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT11"]
    pub mod SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT11_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT11_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT12"]
    pub mod SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT12_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT12_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT13"]
    pub mod SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT13_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT13_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT14"]
    pub mod SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT14_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT14_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT15"]
    pub mod SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT15_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT15_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Domain Config Register"]
pub mod DOMAIN_M7DEBUG {
    pub use crate::RW as access;
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU0_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU0_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU0_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU0_SUSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU1_RUN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU1_WAIT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU1_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU1_SUSP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Status Register"]
pub mod STAT_M7DEBUG {
    pub use crate::RW as access;
    #[doc = "This is a Read Only bit. It indicate if the reset is in process."]
    pub mod UNDER_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is finished"]
            pub const UNDER_RST_0: u32 = 0;
            #[doc = "the reset is in process"]
            pub const UNDER_RST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by the power mode transfer."]
    pub mod RST_BY_HW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by the power mode transfer"]
            pub const RST_BY_HW_0: u32 = 0;
            #[doc = "the reset is caused by the power mode transfer"]
            pub const RST_BY_HW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by setting SW_RESET bit."]
    pub mod RST_BY_SW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by software setting"]
            pub const RST_BY_SW_0: u32 = 0;
            #[doc = "the reset is caused by software setting"]
            pub const RST_BY_SW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Authentication Register"]
pub mod AUTHEN_USBPHY1 {
    pub use crate::RW as access;
    #[doc = "Control whether reset slice is in domain mode"]
    pub mod DOMAIN_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by CPU power mode transition"]
            pub const DOMAIN_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by CPU power mode transition. Do not set this bit and SETPOINT_MODE at the same time."]
            pub const DOMAIN_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control whether reset slice is in Setpoint mode"]
    pub mod SETPOINT_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by Setpoint transition"]
            pub const SETPOINT_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by Setpoint transition. Do not set this bit and DOMAIN_MODE at the same time."]
            pub const SETPOINT_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain/Setpoint mode lock"]
    pub mod LOCK_MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "when this bitfield set to 1, reset of slice would be subject to corresponding core status transition"]
    pub mod ASSIGN_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Assign list lock"]
    pub mod LOCK_ASSIGN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock NONSECURE and USER"]
    pub mod LOCK_SETTING {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Control Register"]
pub mod CTRL_USBPHY1 {
    pub use crate::RW as access;
    #[doc = "This is a self clearing bit"]
    pub mod SW_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "do not assert slice software reset"]
            pub const SW_RESET_0: u32 = 0;
            #[doc = "assert slice software reset"]
            pub const SW_RESET_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Setpoint Config Register"]
pub mod SETPOINT_USBPHY1 {
    pub use crate::RW as access;
    #[doc = "SETPOINT0"]
    pub mod SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT0_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT1"]
    pub mod SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT1_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT2"]
    pub mod SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT2_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT3"]
    pub mod SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT3_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT4"]
    pub mod SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT4_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT5"]
    pub mod SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT5_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT5_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT6"]
    pub mod SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT6_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT6_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT7"]
    pub mod SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT7_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT8"]
    pub mod SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT8_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT8_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT9"]
    pub mod SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT9_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT9_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT10"]
    pub mod SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT10_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT10_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT11"]
    pub mod SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT11_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT11_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT12"]
    pub mod SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT12_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT12_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT13"]
    pub mod SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT13_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT13_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT14"]
    pub mod SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT14_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT14_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT15"]
    pub mod SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT15_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT15_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Domain Config Register"]
pub mod DOMAIN_USBPHY1 {
    pub use crate::RW as access;
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU0_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU0_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU0_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU0_SUSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU1_RUN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU1_WAIT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU1_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU1_SUSP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Status Register"]
pub mod STAT_USBPHY1 {
    pub use crate::RW as access;
    #[doc = "This is a Read Only bit. It indicate if the reset is in process."]
    pub mod UNDER_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is finished"]
            pub const UNDER_RST_0: u32 = 0;
            #[doc = "the reset is in process"]
            pub const UNDER_RST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by the power mode transfer."]
    pub mod RST_BY_HW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by the power mode transfer"]
            pub const RST_BY_HW_0: u32 = 0;
            #[doc = "the reset is caused by the power mode transfer"]
            pub const RST_BY_HW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by setting SW_RESET bit."]
    pub mod RST_BY_SW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by software setting"]
            pub const RST_BY_SW_0: u32 = 0;
            #[doc = "the reset is caused by software setting"]
            pub const RST_BY_SW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Authentication Register"]
pub mod AUTHEN_USBPHY2 {
    pub use crate::RW as access;
    #[doc = "Control whether reset slice is in domain mode"]
    pub mod DOMAIN_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by CPU power mode transition"]
            pub const DOMAIN_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by CPU power mode transition. Do not set this bit and SETPOINT_MODE at the same time."]
            pub const DOMAIN_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control whether reset slice is in Setpoint mode"]
    pub mod SETPOINT_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "slice hardware reset will NOT be triggered by Setpoint transition"]
            pub const SETPOINT_MODE_0: u32 = 0;
            #[doc = "slice hardware reset will be triggered by Setpoint transition. Do not set this bit and DOMAIN_MODE at the same time."]
            pub const SETPOINT_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain/Setpoint mode lock"]
    pub mod LOCK_MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "when this bitfield set to 1, reset of slice would be subject to corresponding core status transition"]
    pub mod ASSIGN_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Assign list lock"]
    pub mod LOCK_ASSIGN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Domain ID white list"]
    pub mod WHITE_LIST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "White list lock"]
    pub mod LOCK_LIST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow user mode access"]
    pub mod USER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Allow non-secure mode access"]
    pub mod NONSECURE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock NONSECURE and USER"]
    pub mod LOCK_SETTING {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Control Register"]
pub mod CTRL_USBPHY2 {
    pub use crate::RW as access;
    #[doc = "This is a self clearing bit"]
    pub mod SW_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "do not assert slice software reset"]
            pub const SW_RESET_0: u32 = 0;
            #[doc = "assert slice software reset"]
            pub const SW_RESET_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Setpoint Config Register"]
pub mod SETPOINT_USBPHY2 {
    pub use crate::RW as access;
    #[doc = "SETPOINT0"]
    pub mod SETPOINT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT0_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT1"]
    pub mod SETPOINT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT1_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT2"]
    pub mod SETPOINT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT2_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT3"]
    pub mod SETPOINT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT3_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT4"]
    pub mod SETPOINT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT4_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT4_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT5"]
    pub mod SETPOINT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT5_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT5_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT6"]
    pub mod SETPOINT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT6_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT6_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT7"]
    pub mod SETPOINT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT7_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT8"]
    pub mod SETPOINT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT8_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT8_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT9"]
    pub mod SETPOINT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT9_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT9_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT10"]
    pub mod SETPOINT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT10_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT10_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT11"]
    pub mod SETPOINT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT11_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT11_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT12"]
    pub mod SETPOINT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT12_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT12_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT13"]
    pub mod SETPOINT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT13_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT13_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT14"]
    pub mod SETPOINT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT14_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT14_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SETPOINT15"]
    pub mod SETPOINT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when system in Setpoint n"]
            pub const SETPOINT15_0: u32 = 0;
            #[doc = "Slice reset will be asserted when system in Setpoint n"]
            pub const SETPOINT15_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Domain Config Register"]
pub mod DOMAIN_USBPHY2 {
    pub use crate::RW as access;
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU0_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in RUN mode"]
            pub const CPU0_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU0_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in WAIT mode"]
            pub const CPU0_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU0_STOP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in STOP mode"]
            pub const CPU0_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU0_SUSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU0 in SUSPEND mode"]
            pub const CPU0_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for RUN"]
    pub mod CPU1_RUN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in RUN mode"]
            pub const CPU1_RUN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for WAIT"]
    pub mod CPU1_WAIT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in WAIT mode"]
            pub const CPU1_WAIT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for STOP"]
    pub mod CPU1_STOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in STOP mode"]
            pub const CPU1_STOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CPU mode setting for SUSPEND"]
    pub mod CPU1_SUSP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Slice reset will be de-asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_0: u32 = 0;
            #[doc = "Slice reset will be asserted when CPU1 in SUSPEND mode"]
            pub const CPU1_SUSP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Slice Status Register"]
pub mod STAT_USBPHY2 {
    pub use crate::RW as access;
    #[doc = "This is a Read Only bit. It indicate if the reset is in process."]
    pub mod UNDER_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is finished"]
            pub const UNDER_RST_0: u32 = 0;
            #[doc = "the reset is in process"]
            pub const UNDER_RST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by the power mode transfer."]
    pub mod RST_BY_HW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by the power mode transfer"]
            pub const RST_BY_HW_0: u32 = 0;
            #[doc = "the reset is caused by the power mode transfer"]
            pub const RST_BY_HW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit indicate if the reset is caused by setting SW_RESET bit."]
    pub mod RST_BY_SW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "the reset is not caused by software setting"]
            pub const RST_BY_SW_0: u32 = 0;
            #[doc = "the reset is caused by software setting"]
            pub const RST_BY_SW_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
