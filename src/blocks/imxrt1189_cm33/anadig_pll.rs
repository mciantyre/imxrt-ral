#[doc = "RT1180_ANADIG_REGISTER"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4000],
    #[doc = "ARM_PLL_CTRL_REGISTER"]
    pub ARM_PLL_CTRL: u32,
    _reserved1: [u8; 0x0c],
    #[doc = "SYS_PLL3_CTRL_REGISTER"]
    pub SYS_PLL3_CTRL: u32,
    _reserved2: [u8; 0x0c],
    #[doc = "SYS_PLL3_UPDATE_REGISTER"]
    pub SYS_PLL3_UPDATE: u32,
    _reserved3: [u8; 0x0c],
    #[doc = "SYS_PLL3_PFD_REGISTER"]
    pub SYS_PLL3_PFD: u32,
    _reserved4: [u8; 0x0c],
    #[doc = "SYS_PLL2_CTRL_REGISTER"]
    pub SYS_PLL2_CTRL: u32,
    _reserved5: [u8; 0x0c],
    #[doc = "SYS_PLL2_UPDATE_REGISTER"]
    pub SYS_PLL2_UPDATE: u32,
    _reserved6: [u8; 0x0c],
    #[doc = "SYS_PLL2_SS_REGISTER"]
    pub SYS_PLL2_SS: u32,
    _reserved7: [u8; 0x0c],
    #[doc = "SYS_PLL2_PFD_REGISTER"]
    pub SYS_PLL2_PFD: u32,
    _reserved8: [u8; 0x0c],
    #[doc = "SYS_PLL2_MFN_REGISTER"]
    pub SYS_PLL2_MFN: u32,
    _reserved9: [u8; 0x0c],
    #[doc = "SYS_PLL2_MFI_REGISTER"]
    pub SYS_PLL2_MFI: u32,
    _reserved10: [u8; 0x0c],
    #[doc = "SYS_PLL2_MFD_REGISTER"]
    pub SYS_PLL2_MFD: u32,
    _reserved11: [u8; 0x5c],
    #[doc = "SYS_PLL1_CTRL_REGISTER"]
    pub SYS_PLL1_CTRL: u32,
    _reserved12: [u8; 0xfc],
    #[doc = "PLL_AUDIO_CTRL_REGISTER"]
    pub PLL_AUDIO_CTRL: u32,
}
#[doc = "ARM_PLL_CTRL_REGISTER"]
pub mod ARM_PLL_CTRL {
    pub use crate::RW as access;
    #[doc = "DIV_SELECT"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PLL Start up initialization"]
    pub mod HOLD_RING_OFF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "Initialize PLL start up"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers up the PLL."]
    pub mod POWERUP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Power down the PLL"]
            pub const PDOWN: u32 = 0;
            #[doc = "Power Up the PLL"]
            pub const PUP: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the clock output."]
    pub mod ENABLE_CLK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable the clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable the clock"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "POST_DIV_SEL"]
    pub mod POST_DIV_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Divide by 2"]
            pub const DIV2: u32 = 0;
            #[doc = "Divide by 4"]
            pub const DIV4: u32 = 0x01;
            #[doc = "Divide by 8"]
            pub const DIV8: u32 = 0x02;
            #[doc = "Divide by 1"]
            pub const DIV1: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the pll."]
    pub mod BYPASS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Function mode"]
            pub const FUNC: u32 = 0;
            #[doc = "Bypass Mode"]
            pub const BYPASS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ARM_PLL_STABLE"]
    pub mod ARM_PLL_STABLE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ARM PLL is not stable"]
            pub const DISABLE: u32 = 0;
            #[doc = "ARM PLL is stable"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ARM_PLL_GATE"]
    pub mod ARM_PLL_GATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Clock is not gated"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock is gated"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "pll_arm_control_mode"]
    pub mod ARM_PLL_CONTROL_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SYS_PLL3_CTRL_REGISTER"]
pub mod SYS_PLL3_CTRL {
    pub use crate::RW as access;
    #[doc = "DIV_SELECT"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "div_select=130x1: div_select=150x2"]
            pub const DIV2_1: u32 = 0;
            #[doc = "div_select=160x3: div_select=200x4"]
            pub const DIV4_1: u32 = 0x01;
            #[doc = "div_select=220x5: div_select=250x6"]
            pub const DIV8_1: u32 = 0x02;
            #[doc = "div_select=300x7: div_select=240"]
            pub const DIV1_1: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS PLL3 DIV2 gate"]
    pub mod SYS_PLL3_DIV2 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Internal PLL Regulator"]
    pub mod PLL_REG_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PLL Start up initialization"]
    pub mod HOLD_RING_OFF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "Initialize PLL start up"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the clock output."]
    pub mod ENABLE_CLK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable the clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable the clock"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BYPASS"]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Function mode"]
            pub const FUNC: u32 = 0;
            #[doc = "Bypass Mode"]
            pub const BYPASS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Power up the PLL"]
    pub mod POWERUP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Power down the PLL"]
            pub const PDOWN: u32 = 0;
            #[doc = "Power Up the PLL"]
            pub const PUP: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL3_DIV2_CONTROL_MODE"]
    pub mod SYS_PLL3_DIV2_CONTROL_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL3_STABLE"]
    pub mod SYS_PLL3_STABLE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not Stable"]
            pub const NS: u32 = 0;
            #[doc = "Stable"]
            pub const STABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL3_GATE"]
    pub mod SYS_PLL3_GATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Clock is not gated"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock is gated"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL3_control_mode"]
    pub mod SYS_PLL3_CONTROL_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SYS_PLL3_UPDATE_REGISTER"]
pub mod SYS_PLL3_UPDATE {
    pub use crate::RW as access;
    #[doc = "PFD0_OVERRIDE"]
    pub mod PFD0_UPDATE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD1_OVERRIDE"]
    pub mod PFD1_UPDATE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD2_OVERRIDE"]
    pub mod PFD2_UPDATE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD3_UPDATE"]
    pub mod PFD3_UPDATE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "pfd0_control_mode"]
    pub mod PFD0_CONTROL_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "pfd1_control_mode"]
    pub mod PFD1_CONTROL_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "pdf2_control_mode"]
    pub mod PFD2_CONTROL_MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "pfd3_control_mode"]
    pub mod PFD3_CONTROL_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SYS_PLL3_PFD_REGISTER"]
pub mod SYS_PLL3_PFD {
    pub use crate::RW as access;
    #[doc = "PFD0_FRAC"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD0_STABLE"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD0_DIV1_CLKGATE"]
    pub mod PFD0_DIV1_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ref_pfd0 fractional divider clock is enabled"]
            pub const ON: u32 = 0;
            #[doc = "Fractional divider clock (reference ref_pfd0) is off (power savings"]
            pub const OFF: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD1_FRAC"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD1_STABLE"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD1_DIV1_CLKGATE"]
    pub mod PFD1_DIV1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ref_pfd1 fractional divider clock is enabled"]
            pub const ON: u32 = 0;
            #[doc = "Fractional divider clock (reference ref_pfd1) is off (power savings)"]
            pub const OFF: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD2_FRAC"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD2_STABLE"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD2_DIV1_CLKGATE"]
    pub mod PFD2_DIV1_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ref_pfd2 fractional divider clock is enabled"]
            pub const ON: u32 = 0;
            #[doc = "Fractional divider clock (reference ref_pfd2) is off (power savings)"]
            pub const OFF: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD3_FRAC"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD3_STABLE"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD3_DIV1_CLKGATE"]
    pub mod PFD3_DIV1_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ref_pfd3 fractional divider clock is enabled"]
            pub const ON: u32 = 0;
            #[doc = "Fractional divider clock (reference ref_pfd3) is off (power savings)"]
            pub const OFF: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SYS_PLL2_CTRL_REGISTER"]
pub mod SYS_PLL2_CTRL {
    pub use crate::RW as access;
    #[doc = "Enable Internal PLL Regulator"]
    pub mod PLL_REG_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PLL Start up initialization"]
    pub mod HOLD_RING_OFF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "Initialize PLL start up"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the clock output."]
    pub mod ENABLE_CLK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable the clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable the clock"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the pll."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Function mode"]
            pub const FUNC: u32 = 0;
            #[doc = "Bypass Mode"]
            pub const BYPASS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DITHER_ENABLE"]
    pub mod DITHER_ENABLE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable Dither"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable Dither"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD_OFFSET_EN"]
    pub mod PFD_OFFSET_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PLL_DDR_OVERRIDE"]
    pub mod PLL_DDR_OVERRIDE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Power up the PLL"]
    pub mod POWERUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Power down the PLL"]
            pub const PDOWN: u32 = 0;
            #[doc = "Power Up the PLL"]
            pub const PUP: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL2_STABLE"]
    pub mod SYS_PLL2_STABLE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL2_GATE"]
    pub mod SYS_PLL2_GATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Clock is not gated"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock is gated"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL2_control_mode"]
    pub mod SYS_PLL2_CONTROL_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SYS_PLL2_UPDATE_REGISTER"]
pub mod SYS_PLL2_UPDATE {
    pub use crate::RW as access;
    #[doc = "PFD0_UPDATE"]
    pub mod PFD0_UPDATE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD1_UPDATE"]
    pub mod PFD1_UPDATE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD2_UPDATE"]
    pub mod PFD2_UPDATE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD3_UPDATE"]
    pub mod PFD3_UPDATE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "pfd0_control_mode"]
    pub mod PFD0_CONTROL_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "pfd1_control_mode"]
    pub mod PFD1_CONTROL_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "pfd2_control_mode"]
    pub mod PFD2_CONTROL_MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "pfd3_control_mode"]
    pub mod PFD3_CONTROL_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SYS_PLL2_SS_REGISTER"]
pub mod SYS_PLL2_SS {
    pub use crate::RW as access;
    #[doc = "STEP"]
    pub mod STEP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ENABLE"]
    pub mod ENABLE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable Spread Spectrum"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable Spread Spectrum"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "STOP"]
    pub mod STOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SYS_PLL2_PFD_REGISTER"]
pub mod SYS_PLL2_PFD {
    pub use crate::RW as access;
    #[doc = "PFD0_FRAC"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD0_STABLE"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD0_CLKGATE"]
    pub mod PFD0_DIV1_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PFD0 fractional divider clock is enabled."]
            pub const ENABLE: u32 = 0;
            #[doc = "Fractional divider clock (reference PFD0) is off (power savings)"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD1_FRAC"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD1_STABLE"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD1_CLKGATE"]
    pub mod PFD1_DIV1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PFD1 fractional divider clock is enabled."]
            pub const ENABLE: u32 = 0;
            #[doc = "Fractional divider clock (reference PFD1) is off (power savings)"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD2_FRAC"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD2_STABLE"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD2_CLKGATE"]
    pub mod PFD2_DIV1_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PFD2 fractional divider clock is enabled."]
            pub const ENABLE: u32 = 0;
            #[doc = "Fractional divider clock (reference PFD2) is off (power savings)"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD3_FRAC"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD3_STABLE"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PFD3_CLKGATE"]
    pub mod PFD3_DIV1_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PFD3 fractional divider clock is enabled."]
            pub const ENABLE: u32 = 0;
            #[doc = "Fractional divider clock (reference PFD3) is off (power savings)"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SYS_PLL2_MFN_REGISTER"]
pub mod SYS_PLL2_MFN {
    pub use crate::RW as access;
    #[doc = "MFN"]
    pub mod MFN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SYS_PLL2_MFI_REGISTER"]
pub mod SYS_PLL2_MFI {
    pub use crate::RW as access;
    #[doc = "MFI"]
    pub mod MFI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SYS_PLL2_MFD_REGISTER"]
pub mod SYS_PLL2_MFD {
    pub use crate::RW as access;
    #[doc = "Denominator"]
    pub mod MFD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SYS_PLL1_CTRL_REGISTER"]
pub mod SYS_PLL1_CTRL {
    pub use crate::RW as access;
    #[doc = "ENABLE_CLK"]
    pub mod ENABLE_CLK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL1_GATE"]
    pub mod SYS_PLL1_GATE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No gate"]
            pub const NOGATE: u32 = 0;
            #[doc = "Gate the output"]
            pub const GATED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL1_DIV2"]
    pub mod SYS_PLL1_DIV2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL1_DIV5"]
    pub mod SYS_PLL1_DIV5 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL1_DIV5_CONTROL_MODE"]
    pub mod SYS_PLL1_DIV5_CONTROL_MODE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL1_DIV2_CONTROL_MODE"]
    pub mod SYS_PLL1_DIV2_CONTROL_MODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL1_STABLE"]
    pub mod SYS_PLL1_STABLE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not Stable"]
            pub const NS: u32 = 0;
            #[doc = "Stable"]
            pub const STABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SYS_PLL1_CONTROL_MODE"]
    pub mod SYS_PLL1_CONTROL_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PLL_AUDIO_CTRL_REGISTER"]
pub mod PLL_AUDIO_CTRL {
    pub use crate::RW as access;
    #[doc = "ENABLE_CLK"]
    pub mod ENABLE_CLK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PLL_AUDIO_GATE"]
    pub mod PLL_AUDIO_GATE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No gate"]
            pub const NOGATE: u32 = 0;
            #[doc = "Gate the output"]
            pub const GATED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PLL_AUDIO_STABLE"]
    pub mod PLL_AUDIO_STABLE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "pll_audio_control_mode"]
    pub mod PLL_AUDIO_CONTROL_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software Mode (Default)"]
            pub const SW: u32 = 0;
            #[doc = "GPC Mode"]
            pub const GPC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
