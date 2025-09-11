#[doc = "CCM_ANALOG"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    pub PLL_USB1: u32,
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    pub PLL_USB1_SET: u32,
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    pub PLL_USB1_CLR: u32,
    #[doc = "Analog USB1 480MHz PLL Control Register"]
    pub PLL_USB1_TOG: u32,
    _reserved1: [u8; 0x10],
    #[doc = "Analog System PLL Control Register"]
    pub PLL_SYS: u32,
    #[doc = "Analog System PLL Control Register"]
    pub PLL_SYS_SET: u32,
    #[doc = "Analog System PLL Control Register"]
    pub PLL_SYS_CLR: u32,
    #[doc = "Analog System PLL Control Register"]
    pub PLL_SYS_TOG: u32,
    #[doc = "528MHz System PLL Spread Spectrum Register"]
    pub PLL_SYS_SS: u32,
    _reserved2: [u8; 0x0c],
    #[doc = "Numerator of 528MHz System PLL Fractional Loop Divider Register"]
    pub PLL_SYS_NUM: u32,
    _reserved3: [u8; 0x0c],
    #[doc = "Denominator of 528MHz System PLL Fractional Loop Divider Register"]
    pub PLL_SYS_DENOM: u32,
    _reserved4: [u8; 0x0c],
    #[doc = "Analog Audio PLL control Register"]
    pub PLL_AUDIO: u32,
    #[doc = "Analog Audio PLL control Register"]
    pub PLL_AUDIO_SET: u32,
    #[doc = "Analog Audio PLL control Register"]
    pub PLL_AUDIO_CLR: u32,
    #[doc = "Analog Audio PLL control Register"]
    pub PLL_AUDIO_TOG: u32,
    #[doc = "Numerator of Audio PLL Fractional Loop Divider Register"]
    pub PLL_AUDIO_NUM: u32,
    _reserved5: [u8; 0x0c],
    #[doc = "Denominator of Audio PLL Fractional Loop Divider Register"]
    pub PLL_AUDIO_DENOM: u32,
    _reserved6: [u8; 0x4c],
    #[doc = "Analog ENET PLL Control Register"]
    pub PLL_ENET: u32,
    #[doc = "Analog ENET PLL Control Register"]
    pub PLL_ENET_SET: u32,
    #[doc = "Analog ENET PLL Control Register"]
    pub PLL_ENET_CLR: u32,
    #[doc = "Analog ENET PLL Control Register"]
    pub PLL_ENET_TOG: u32,
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub PFD_480: u32,
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub PFD_480_SET: u32,
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub PFD_480_CLR: u32,
    #[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub PFD_480_TOG: u32,
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub PFD_528: u32,
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub PFD_528_SET: u32,
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub PFD_528_CLR: u32,
    #[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub PFD_528_TOG: u32,
    _reserved7: [u8; 0x40],
    #[doc = "Miscellaneous Register 0"]
    pub MISC0: u32,
    #[doc = "Miscellaneous Register 0"]
    pub MISC0_SET: u32,
    #[doc = "Miscellaneous Register 0"]
    pub MISC0_CLR: u32,
    #[doc = "Miscellaneous Register 0"]
    pub MISC0_TOG: u32,
    #[doc = "Miscellaneous Register 1"]
    pub MISC1: u32,
    #[doc = "Miscellaneous Register 1"]
    pub MISC1_SET: u32,
    #[doc = "Miscellaneous Register 1"]
    pub MISC1_CLR: u32,
    #[doc = "Miscellaneous Register 1"]
    pub MISC1_TOG: u32,
    #[doc = "Miscellaneous Register 2"]
    pub MISC2: u32,
    #[doc = "Miscellaneous Register 2"]
    pub MISC2_SET: u32,
    #[doc = "Miscellaneous Register 2"]
    pub MISC2_CLR: u32,
    #[doc = "Miscellaneous Register 2"]
    pub MISC2_TOG: u32,
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod PLL_USB1 {
    pub use crate::RW as access;
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    pub mod EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PLL outputs for USBPHYn off."]
            pub const EN_USB_CLKS_0: u32 = 0;
            #[doc = "PLL outputs for USBPHYn on."]
            pub const EN_USB_CLKS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    pub mod POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the PLL clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod PLL_USB1_SET {
    pub use crate::RW as access;
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    pub mod EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PLL outputs for USBPHYn off."]
            pub const EN_USB_CLKS_0: u32 = 0;
            #[doc = "PLL outputs for USBPHYn on."]
            pub const EN_USB_CLKS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    pub mod POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the PLL clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod PLL_USB1_CLR {
    pub use crate::RW as access;
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    pub mod EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PLL outputs for USBPHYn off."]
            pub const EN_USB_CLKS_0: u32 = 0;
            #[doc = "PLL outputs for USBPHYn on."]
            pub const EN_USB_CLKS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    pub mod POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the PLL clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod PLL_USB1_TOG {
    pub use crate::RW as access;
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    pub mod EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PLL outputs for USBPHYn off."]
            pub const EN_USB_CLKS_0: u32 = 0;
            #[doc = "PLL outputs for USBPHYn on."]
            pub const EN_USB_CLKS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    pub mod POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the PLL clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog System PLL Control Register"]
pub mod PLL_SYS {
    pub use crate::RW as access;
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog System PLL Control Register"]
pub mod PLL_SYS_SET {
    pub use crate::RW as access;
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog System PLL Control Register"]
pub mod PLL_SYS_CLR {
    pub use crate::RW as access;
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog System PLL Control Register"]
pub mod PLL_SYS_TOG {
    pub use crate::RW as access;
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "528MHz System PLL Spread Spectrum Register"]
pub mod PLL_SYS_SS {
    pub use crate::RW as access;
    #[doc = "Frequency change step = step/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
    pub mod STEP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable bit"]
    pub mod ENABLE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Spread spectrum modulation disabled"]
            pub const ENABLE_0: u32 = 0;
            #[doc = "Soread spectrum modulation enabled"]
            pub const ENABLE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frequency change = stop/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
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
#[doc = "Numerator of 528MHz System PLL Fractional Loop Divider Register"]
pub mod PLL_SYS_NUM {
    pub use crate::RW as access;
    #[doc = "30 bit numerator (A) of fractional loop divider (signed integer)."]
    pub mod A {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Denominator of 528MHz System PLL Fractional Loop Divider Register"]
pub mod PLL_SYS_DENOM {
    pub use crate::RW as access;
    #[doc = "30 bit denominator (B) of fractional loop divider (unsigned integer)."]
    pub mod B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Audio PLL control Register"]
pub mod PLL_AUDIO {
    pub use crate::RW as access;
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    pub mod POST_DIV_SELECT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Divide by 4."]
            pub const POST_DIV_SELECT_0: u32 = 0;
            #[doc = "Divide by 2."]
            pub const POST_DIV_SELECT_1: u32 = 0x01;
            #[doc = "Divide by 1."]
            pub const POST_DIV_SELECT_2: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Audio PLL control Register"]
pub mod PLL_AUDIO_SET {
    pub use crate::RW as access;
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    pub mod POST_DIV_SELECT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Divide by 4."]
            pub const POST_DIV_SELECT_0: u32 = 0;
            #[doc = "Divide by 2."]
            pub const POST_DIV_SELECT_1: u32 = 0x01;
            #[doc = "Divide by 1."]
            pub const POST_DIV_SELECT_2: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Audio PLL control Register"]
pub mod PLL_AUDIO_CLR {
    pub use crate::RW as access;
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    pub mod POST_DIV_SELECT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Divide by 4."]
            pub const POST_DIV_SELECT_0: u32 = 0;
            #[doc = "Divide by 2."]
            pub const POST_DIV_SELECT_1: u32 = 0x01;
            #[doc = "Divide by 1."]
            pub const POST_DIV_SELECT_2: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog Audio PLL control Register"]
pub mod PLL_AUDIO_TOG {
    pub use crate::RW as access;
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable PLL output"]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    pub mod POST_DIV_SELECT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Divide by 4."]
            pub const POST_DIV_SELECT_0: u32 = 0;
            #[doc = "Divide by 2."]
            pub const POST_DIV_SELECT_1: u32 = 0x01;
            #[doc = "Divide by 1."]
            pub const POST_DIV_SELECT_2: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Numerator of Audio PLL Fractional Loop Divider Register"]
pub mod PLL_AUDIO_NUM {
    pub use crate::RW as access;
    #[doc = "30 bit numerator of fractional loop divider."]
    pub mod A {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Denominator of Audio PLL Fractional Loop Divider Register"]
pub mod PLL_AUDIO_DENOM {
    pub use crate::RW as access;
    #[doc = "30 bit denominator of fractional loop divider."]
    pub mod B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog ENET PLL Control Register"]
pub mod PLL_ENET {
    pub use crate::RW as access;
    #[doc = "Controls the frequency of the ethernet reference clock"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the ethernet clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    pub mod ENET_25M_REF_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the PLL providing ENET 500 MHz reference clock"]
    pub mod ENET_500M_REF_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog ENET PLL Control Register"]
pub mod PLL_ENET_SET {
    pub use crate::RW as access;
    #[doc = "Controls the frequency of the ethernet reference clock"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the ethernet clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    pub mod ENET_25M_REF_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the PLL providing ENET 500 MHz reference clock"]
    pub mod ENET_500M_REF_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog ENET PLL Control Register"]
pub mod PLL_ENET_CLR {
    pub use crate::RW as access;
    #[doc = "Controls the frequency of the ethernet reference clock"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the ethernet clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    pub mod ENET_25M_REF_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the PLL providing ENET 500 MHz reference clock"]
    pub mod ENET_500M_REF_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Analog ENET PLL Control Register"]
pub mod PLL_ENET_TOG {
    pub use crate::RW as access;
    #[doc = "Controls the frequency of the ethernet reference clock"]
    pub mod DIV_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Powers down the PLL."]
    pub mod POWERDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the ethernet clock output."]
    pub mod ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Determines the bypass source."]
    pub mod BYPASS_CLK_SRC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select the 24MHz oscillator as source."]
            pub const REF_CLK_24M: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bypass the PLL."]
    pub mod BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    pub mod ENET_25M_REF_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable the PLL providing ENET 500 MHz reference clock"]
    pub mod ENET_500M_REF_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod PFD_480 {
    pub use crate::RW as access;
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod PFD_480_SET {
    pub use crate::RW as access;
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod PFD_480_CLR {
    pub use crate::RW as access;
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod PFD_480_TOG {
    pub use crate::RW as access;
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod PFD_528 {
    pub use crate::RW as access;
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod PFD_528_SET {
    pub use crate::RW as access;
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod PFD_528_CLR {
    pub use crate::RW as access;
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod PFD_528_TOG {
    pub use crate::RW as access;
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD1_FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD1_STABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD1_CLKGATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD2_FRAC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD2_STABLE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD2_CLKGATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field controls the fractional divide value"]
    pub mod PFD3_FRAC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    pub mod PFD3_STABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Clock Gate"]
    pub mod PFD3_CLKGATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous Register 0"]
pub mod MISC0 {
    pub use crate::RW as access;
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Uses coarse bias currents for startup"]
            pub const REFTOP_SELFBIASOFF_0: u32 = 0;
            #[doc = "Uses bandgap-based bias currents for best performance."]
            pub const REFTOP_SELFBIASOFF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Nominal VBG"]
            pub const REFTOP_VBGADJ_0: u32 = 0;
            #[doc = "VBG+0.78%"]
            pub const REFTOP_VBGADJ_1: u32 = 0x01;
            #[doc = "VBG+1.56%"]
            pub const REFTOP_VBGADJ_2: u32 = 0x02;
            #[doc = "VBG+2.34%"]
            pub const REFTOP_VBGADJ_3: u32 = 0x03;
            #[doc = "VBG-0.78%"]
            pub const REFTOP_VBGADJ_4: u32 = 0x04;
            #[doc = "VBG-1.56%"]
            pub const REFTOP_VBGADJ_5: u32 = 0x05;
            #[doc = "VBG-2.34%"]
            pub const REFTOP_VBGADJ_6: u32 = 0x06;
            #[doc = "VBG-3.12%"]
            pub const REFTOP_VBGADJ_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Configure the analog behavior in stop mode."]
    pub mod STOP_MODE_CONFIG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "All analog except RTC powered down on stop mode assertion."]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
            pub const STOP_MODE_CONFIG_1: u32 = 0x01;
            #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    pub mod DISCON_HIGH_SNVS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Turn on the switch"]
            pub const DISCON_HIGH_SNVS_0: u32 = 0;
            #[doc = "Turn off the switch"]
            pub const DISCON_HIGH_SNVS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    pub mod OSC_I {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0;
            #[doc = "Decrease current by 12.5%"]
            pub const MINUS_12_5_PERCENT: u32 = 0x01;
            #[doc = "Decrease current by 25.0%"]
            pub const MINUS_25_PERCENT: u32 = 0x02;
            #[doc = "Decrease current by 37.5%"]
            pub const MINUS_37_5_PERCENT: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    pub mod CLKGATE_CTRL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
            pub const ALLOW_AUTO_GATE: u32 = 0;
            #[doc = "Prevent the logic from ever gating off the clock."]
            pub const NO_AUTO_GATE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    pub mod CLKGATE_DELAY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "0.5ms"]
            pub const CLKGATE_DELAY_0: u32 = 0;
            #[doc = "1.0ms"]
            pub const CLKGATE_DELAY_1: u32 = 0x01;
            #[doc = "2.0ms"]
            pub const CLKGATE_DELAY_2: u32 = 0x02;
            #[doc = "3.0ms"]
            pub const CLKGATE_DELAY_3: u32 = 0x03;
            #[doc = "4.0ms"]
            pub const CLKGATE_DELAY_4: u32 = 0x04;
            #[doc = "5.0ms"]
            pub const CLKGATE_DELAY_5: u32 = 0x05;
            #[doc = "6.0ms"]
            pub const CLKGATE_DELAY_6: u32 = 0x06;
            #[doc = "7.0ms"]
            pub const CLKGATE_DELAY_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    pub mod RTC_XTAL_SOURCE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal ring oscillator"]
            pub const RTC_XTAL_SOURCE_0: u32 = 0;
            #[doc = "RTC_XTAL"]
            pub const RTC_XTAL_SOURCE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous Register 0"]
pub mod MISC0_SET {
    pub use crate::RW as access;
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Uses coarse bias currents for startup"]
            pub const REFTOP_SELFBIASOFF_0: u32 = 0;
            #[doc = "Uses bandgap-based bias currents for best performance."]
            pub const REFTOP_SELFBIASOFF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Nominal VBG"]
            pub const REFTOP_VBGADJ_0: u32 = 0;
            #[doc = "VBG+0.78%"]
            pub const REFTOP_VBGADJ_1: u32 = 0x01;
            #[doc = "VBG+1.56%"]
            pub const REFTOP_VBGADJ_2: u32 = 0x02;
            #[doc = "VBG+2.34%"]
            pub const REFTOP_VBGADJ_3: u32 = 0x03;
            #[doc = "VBG-0.78%"]
            pub const REFTOP_VBGADJ_4: u32 = 0x04;
            #[doc = "VBG-1.56%"]
            pub const REFTOP_VBGADJ_5: u32 = 0x05;
            #[doc = "VBG-2.34%"]
            pub const REFTOP_VBGADJ_6: u32 = 0x06;
            #[doc = "VBG-3.12%"]
            pub const REFTOP_VBGADJ_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Configure the analog behavior in stop mode."]
    pub mod STOP_MODE_CONFIG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "All analog except RTC powered down on stop mode assertion."]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
            pub const STOP_MODE_CONFIG_1: u32 = 0x01;
            #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    pub mod DISCON_HIGH_SNVS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Turn on the switch"]
            pub const DISCON_HIGH_SNVS_0: u32 = 0;
            #[doc = "Turn off the switch"]
            pub const DISCON_HIGH_SNVS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    pub mod OSC_I {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0;
            #[doc = "Decrease current by 12.5%"]
            pub const MINUS_12_5_PERCENT: u32 = 0x01;
            #[doc = "Decrease current by 25.0%"]
            pub const MINUS_25_PERCENT: u32 = 0x02;
            #[doc = "Decrease current by 37.5%"]
            pub const MINUS_37_5_PERCENT: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    pub mod CLKGATE_CTRL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
            pub const ALLOW_AUTO_GATE: u32 = 0;
            #[doc = "Prevent the logic from ever gating off the clock."]
            pub const NO_AUTO_GATE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    pub mod CLKGATE_DELAY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "0.5ms"]
            pub const CLKGATE_DELAY_0: u32 = 0;
            #[doc = "1.0ms"]
            pub const CLKGATE_DELAY_1: u32 = 0x01;
            #[doc = "2.0ms"]
            pub const CLKGATE_DELAY_2: u32 = 0x02;
            #[doc = "3.0ms"]
            pub const CLKGATE_DELAY_3: u32 = 0x03;
            #[doc = "4.0ms"]
            pub const CLKGATE_DELAY_4: u32 = 0x04;
            #[doc = "5.0ms"]
            pub const CLKGATE_DELAY_5: u32 = 0x05;
            #[doc = "6.0ms"]
            pub const CLKGATE_DELAY_6: u32 = 0x06;
            #[doc = "7.0ms"]
            pub const CLKGATE_DELAY_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    pub mod RTC_XTAL_SOURCE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal ring oscillator"]
            pub const RTC_XTAL_SOURCE_0: u32 = 0;
            #[doc = "RTC_XTAL"]
            pub const RTC_XTAL_SOURCE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous Register 0"]
pub mod MISC0_CLR {
    pub use crate::RW as access;
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Uses coarse bias currents for startup"]
            pub const REFTOP_SELFBIASOFF_0: u32 = 0;
            #[doc = "Uses bandgap-based bias currents for best performance."]
            pub const REFTOP_SELFBIASOFF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Nominal VBG"]
            pub const REFTOP_VBGADJ_0: u32 = 0;
            #[doc = "VBG+0.78%"]
            pub const REFTOP_VBGADJ_1: u32 = 0x01;
            #[doc = "VBG+1.56%"]
            pub const REFTOP_VBGADJ_2: u32 = 0x02;
            #[doc = "VBG+2.34%"]
            pub const REFTOP_VBGADJ_3: u32 = 0x03;
            #[doc = "VBG-0.78%"]
            pub const REFTOP_VBGADJ_4: u32 = 0x04;
            #[doc = "VBG-1.56%"]
            pub const REFTOP_VBGADJ_5: u32 = 0x05;
            #[doc = "VBG-2.34%"]
            pub const REFTOP_VBGADJ_6: u32 = 0x06;
            #[doc = "VBG-3.12%"]
            pub const REFTOP_VBGADJ_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Configure the analog behavior in stop mode."]
    pub mod STOP_MODE_CONFIG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "All analog except RTC powered down on stop mode assertion."]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
            pub const STOP_MODE_CONFIG_1: u32 = 0x01;
            #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    pub mod DISCON_HIGH_SNVS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Turn on the switch"]
            pub const DISCON_HIGH_SNVS_0: u32 = 0;
            #[doc = "Turn off the switch"]
            pub const DISCON_HIGH_SNVS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    pub mod OSC_I {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0;
            #[doc = "Decrease current by 12.5%"]
            pub const MINUS_12_5_PERCENT: u32 = 0x01;
            #[doc = "Decrease current by 25.0%"]
            pub const MINUS_25_PERCENT: u32 = 0x02;
            #[doc = "Decrease current by 37.5%"]
            pub const MINUS_37_5_PERCENT: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    pub mod CLKGATE_CTRL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
            pub const ALLOW_AUTO_GATE: u32 = 0;
            #[doc = "Prevent the logic from ever gating off the clock."]
            pub const NO_AUTO_GATE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    pub mod CLKGATE_DELAY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "0.5ms"]
            pub const CLKGATE_DELAY_0: u32 = 0;
            #[doc = "1.0ms"]
            pub const CLKGATE_DELAY_1: u32 = 0x01;
            #[doc = "2.0ms"]
            pub const CLKGATE_DELAY_2: u32 = 0x02;
            #[doc = "3.0ms"]
            pub const CLKGATE_DELAY_3: u32 = 0x03;
            #[doc = "4.0ms"]
            pub const CLKGATE_DELAY_4: u32 = 0x04;
            #[doc = "5.0ms"]
            pub const CLKGATE_DELAY_5: u32 = 0x05;
            #[doc = "6.0ms"]
            pub const CLKGATE_DELAY_6: u32 = 0x06;
            #[doc = "7.0ms"]
            pub const CLKGATE_DELAY_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    pub mod RTC_XTAL_SOURCE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal ring oscillator"]
            pub const RTC_XTAL_SOURCE_0: u32 = 0;
            #[doc = "RTC_XTAL"]
            pub const RTC_XTAL_SOURCE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous Register 0"]
pub mod MISC0_TOG {
    pub use crate::RW as access;
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    pub mod REFTOP_PWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    pub mod REFTOP_SELFBIASOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Uses coarse bias currents for startup"]
            pub const REFTOP_SELFBIASOFF_0: u32 = 0;
            #[doc = "Uses bandgap-based bias currents for best performance."]
            pub const REFTOP_SELFBIASOFF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REFTOP_VBGADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Nominal VBG"]
            pub const REFTOP_VBGADJ_0: u32 = 0;
            #[doc = "VBG+0.78%"]
            pub const REFTOP_VBGADJ_1: u32 = 0x01;
            #[doc = "VBG+1.56%"]
            pub const REFTOP_VBGADJ_2: u32 = 0x02;
            #[doc = "VBG+2.34%"]
            pub const REFTOP_VBGADJ_3: u32 = 0x03;
            #[doc = "VBG-0.78%"]
            pub const REFTOP_VBGADJ_4: u32 = 0x04;
            #[doc = "VBG-1.56%"]
            pub const REFTOP_VBGADJ_5: u32 = 0x05;
            #[doc = "VBG-2.34%"]
            pub const REFTOP_VBGADJ_6: u32 = 0x06;
            #[doc = "VBG-3.12%"]
            pub const REFTOP_VBGADJ_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    pub mod REFTOP_VBGUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Configure the analog behavior in stop mode."]
    pub mod STOP_MODE_CONFIG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "All analog except RTC powered down on stop mode assertion."]
            pub const STOP_MODE_CONFIG_0: u32 = 0;
            #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
            pub const STOP_MODE_CONFIG_1: u32 = 0x01;
            #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_2: u32 = 0x02;
            #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
            pub const STOP_MODE_CONFIG_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    pub mod DISCON_HIGH_SNVS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Turn on the switch"]
            pub const DISCON_HIGH_SNVS_0: u32 = 0;
            #[doc = "Turn off the switch"]
            pub const DISCON_HIGH_SNVS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    pub mod OSC_I {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Nominal"]
            pub const NOMINAL: u32 = 0;
            #[doc = "Decrease current by 12.5%"]
            pub const MINUS_12_5_PERCENT: u32 = 0x01;
            #[doc = "Decrease current by 25.0%"]
            pub const MINUS_25_PERCENT: u32 = 0x02;
            #[doc = "Decrease current by 37.5%"]
            pub const MINUS_37_5_PERCENT: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    pub mod OSC_XTALOK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    pub mod CLKGATE_CTRL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
            pub const ALLOW_AUTO_GATE: u32 = 0;
            #[doc = "Prevent the logic from ever gating off the clock."]
            pub const NO_AUTO_GATE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    pub mod CLKGATE_DELAY {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "0.5ms"]
            pub const CLKGATE_DELAY_0: u32 = 0;
            #[doc = "1.0ms"]
            pub const CLKGATE_DELAY_1: u32 = 0x01;
            #[doc = "2.0ms"]
            pub const CLKGATE_DELAY_2: u32 = 0x02;
            #[doc = "3.0ms"]
            pub const CLKGATE_DELAY_3: u32 = 0x03;
            #[doc = "4.0ms"]
            pub const CLKGATE_DELAY_4: u32 = 0x04;
            #[doc = "5.0ms"]
            pub const CLKGATE_DELAY_5: u32 = 0x05;
            #[doc = "6.0ms"]
            pub const CLKGATE_DELAY_6: u32 = 0x06;
            #[doc = "7.0ms"]
            pub const CLKGATE_DELAY_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    pub mod RTC_XTAL_SOURCE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal ring oscillator"]
            pub const RTC_XTAL_SOURCE_0: u32 = 0;
            #[doc = "RTC_XTAL"]
            pub const RTC_XTAL_SOURCE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    pub mod XTAL_24M_PWD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous Register 1"]
pub mod MISC1 {
    pub use crate::RW as access;
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    pub mod PFD_480_AUTOGATE_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    pub mod PFD_528_AUTOGATE_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    pub mod IRQ_TEMPPANIC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    pub mod IRQ_TEMPLOW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    pub mod IRQ_TEMPHIGH {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    pub mod IRQ_ANA_BO {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    pub mod IRQ_DIG_BO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous Register 1"]
pub mod MISC1_SET {
    pub use crate::RW as access;
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    pub mod PFD_480_AUTOGATE_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    pub mod PFD_528_AUTOGATE_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    pub mod IRQ_TEMPPANIC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    pub mod IRQ_TEMPLOW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    pub mod IRQ_TEMPHIGH {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    pub mod IRQ_ANA_BO {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    pub mod IRQ_DIG_BO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous Register 1"]
pub mod MISC1_CLR {
    pub use crate::RW as access;
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    pub mod PFD_480_AUTOGATE_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    pub mod PFD_528_AUTOGATE_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    pub mod IRQ_TEMPPANIC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    pub mod IRQ_TEMPLOW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    pub mod IRQ_TEMPHIGH {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    pub mod IRQ_ANA_BO {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    pub mod IRQ_DIG_BO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous Register 1"]
pub mod MISC1_TOG {
    pub use crate::RW as access;
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    pub mod PFD_480_AUTOGATE_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    pub mod PFD_528_AUTOGATE_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    pub mod IRQ_TEMPPANIC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    pub mod IRQ_TEMPLOW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    pub mod IRQ_TEMPHIGH {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    pub mod IRQ_ANA_BO {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    pub mod IRQ_DIG_BO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous Register 2"]
pub mod MISC2 {
    pub use crate::RW as access;
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    pub mod REG0_BO_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG0_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG0_BO_OFFSET_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_BO_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG0_BO_STATUS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_ENABLE_BO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ARM supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_OK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    pub mod PLL3_DISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
            pub const PLL3_DISABLE_0: u32 = 0;
            #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
            pub const PLL3_DISABLE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG1_BO_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG1_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG1_BO_OFFSET_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_BO_STATUS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG1_BO_STATUS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_ENABLE_BO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_OK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_LSB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_LSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_LSB_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG2_BO_OFFSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG2_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG2_BO_OFFSET_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_BO_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_ENABLE_BO {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    pub mod REG2_OK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_MSB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_MSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_MSB_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_STEP_TIME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_STEP_TIME {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_STEP_TIME {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Post-divider for video"]
    pub mod VIDEO_DIV {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "divide by 1 (Default)"]
            pub const VIDEO_DIV_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const VIDEO_DIV_1: u32 = 0x01;
            #[doc = "divide by 1"]
            pub const VIDEO_DIV_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const VIDEO_DIV_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous Register 2"]
pub mod MISC2_SET {
    pub use crate::RW as access;
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    pub mod REG0_BO_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG0_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG0_BO_OFFSET_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_BO_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG0_BO_STATUS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_ENABLE_BO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ARM supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_OK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    pub mod PLL3_DISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
            pub const PLL3_DISABLE_0: u32 = 0;
            #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
            pub const PLL3_DISABLE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG1_BO_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG1_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG1_BO_OFFSET_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_BO_STATUS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG1_BO_STATUS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_ENABLE_BO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_OK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_LSB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_LSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_LSB_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG2_BO_OFFSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG2_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG2_BO_OFFSET_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_BO_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_ENABLE_BO {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    pub mod REG2_OK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_MSB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_MSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_MSB_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_STEP_TIME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_STEP_TIME {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_STEP_TIME {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Post-divider for video"]
    pub mod VIDEO_DIV {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "divide by 1 (Default)"]
            pub const VIDEO_DIV_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const VIDEO_DIV_1: u32 = 0x01;
            #[doc = "divide by 1"]
            pub const VIDEO_DIV_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const VIDEO_DIV_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous Register 2"]
pub mod MISC2_CLR {
    pub use crate::RW as access;
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    pub mod REG0_BO_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG0_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG0_BO_OFFSET_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_BO_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG0_BO_STATUS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_ENABLE_BO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ARM supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_OK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    pub mod PLL3_DISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
            pub const PLL3_DISABLE_0: u32 = 0;
            #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
            pub const PLL3_DISABLE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG1_BO_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG1_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG1_BO_OFFSET_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_BO_STATUS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG1_BO_STATUS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_ENABLE_BO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_OK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_LSB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_LSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_LSB_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG2_BO_OFFSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG2_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG2_BO_OFFSET_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_BO_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_ENABLE_BO {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    pub mod REG2_OK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_MSB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_MSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_MSB_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_STEP_TIME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_STEP_TIME {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_STEP_TIME {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Post-divider for video"]
    pub mod VIDEO_DIV {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "divide by 1 (Default)"]
            pub const VIDEO_DIV_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const VIDEO_DIV_1: u32 = 0x01;
            #[doc = "divide by 1"]
            pub const VIDEO_DIV_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const VIDEO_DIV_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous Register 2"]
pub mod MISC2_TOG {
    pub use crate::RW as access;
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    pub mod REG0_BO_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG0_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG0_BO_OFFSET_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_BO_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG0_BO_STATUS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_ENABLE_BO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ARM supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_OK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    pub mod PLL3_DISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
            pub const PLL3_DISABLE_0: u32 = 0;
            #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
            pub const PLL3_DISABLE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG1_BO_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG1_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG1_BO_OFFSET_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_BO_STATUS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout, supply is below target minus brownout offset."]
            pub const REG1_BO_STATUS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_ENABLE_BO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_OK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_LSB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_LSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_LSB_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    pub mod REG2_BO_OFFSET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Brownout offset = 0.100V"]
            pub const REG2_BO_OFFSET_4: u32 = 0x04;
            #[doc = "Brownout offset = 0.175V"]
            pub const REG2_BO_OFFSET_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_BO_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_ENABLE_BO {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    pub mod REG2_OK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    pub mod AUDIO_DIV_MSB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "divide by 1 (Default)"]
            pub const AUDIO_DIV_MSB_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const AUDIO_DIV_MSB_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG0_STEP_TIME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG1_STEP_TIME {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    pub mod REG2_STEP_TIME {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const _64_CLOCKS: u32 = 0;
            #[doc = "128"]
            pub const _128_CLOCKS: u32 = 0x01;
            #[doc = "256"]
            pub const _256_CLOCKS: u32 = 0x02;
            #[doc = "512"]
            pub const _512_CLOCKS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Post-divider for video"]
    pub mod VIDEO_DIV {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "divide by 1 (Default)"]
            pub const VIDEO_DIV_0: u32 = 0;
            #[doc = "divide by 2"]
            pub const VIDEO_DIV_1: u32 = 0x01;
            #[doc = "divide by 1"]
            pub const VIDEO_DIV_2: u32 = 0x02;
            #[doc = "divide by 4"]
            pub const VIDEO_DIV_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
