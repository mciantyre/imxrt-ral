#[doc = "WDOG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Watchdog Control Register"]
    pub WCR: u16,
    #[doc = "Watchdog Service Register"]
    pub WSR: u16,
    #[doc = "Watchdog Reset Status Register"]
    pub WRSR: u16,
    #[doc = "Watchdog Interrupt Control Register"]
    pub WICR: u16,
    #[doc = "Watchdog Miscellaneous Control Register"]
    pub WMCR: u16,
}
#[doc = "Watchdog Control Register"]
pub mod WCR {
    pub use crate::RW as access;
    #[doc = "WDZST"]
    pub mod WDZST {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Continue timer operation (Default)."]
            pub const WDZST_0: u16 = 0;
            #[doc = "Suspend the watchdog timer."]
            pub const WDZST_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WDBG"]
    pub mod WDBG {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Continue WDOG timer operation (Default)."]
            pub const WDBG_0: u16 = 0;
            #[doc = "Suspend the watchdog timer."]
            pub const WDBG_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WDE"]
    pub mod WDE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable the Watchdog (Default)."]
            pub const WDE_0: u16 = 0;
            #[doc = "Enable the Watchdog."]
            pub const WDE_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WDT"]
    pub mod WDT {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect on WDOG_B (Default)."]
            pub const WDT_0: u16 = 0;
            #[doc = "Assert WDOG_B upon a Watchdog Time-out event."]
            pub const WDT_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SRS"]
    pub mod SRS {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Assert system reset signal."]
            pub const SRS_0: u16 = 0;
            #[doc = "No effect on the system (Default)."]
            pub const SRS_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WDA"]
    pub mod WDA {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Assert WDOG_B output."]
            pub const WDA_0: u16 = 0;
            #[doc = "No effect on system (Default)."]
            pub const WDA_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "software reset extension, an option way to generate software reset"]
    pub mod SRE {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "using original way to generate software reset (default)"]
            pub const SRE_0: u16 = 0;
            #[doc = "using new way to generate software reset."]
            pub const SRE_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WDW"]
    pub mod WDW {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Continue WDOG timer operation (Default)."]
            pub const WDW_0: u16 = 0;
            #[doc = "Suspend WDOG timer operation."]
            pub const WDW_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WT"]
    pub mod WT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "- 0.5 Seconds (Default)."]
            pub const WT_0: u16 = 0;
            #[doc = "- 1.0 Seconds."]
            pub const WT_1: u16 = 0x01;
            #[doc = "- 1.5 Seconds."]
            pub const WT_2: u16 = 0x02;
            #[doc = "- 2.0 Seconds."]
            pub const WT_3: u16 = 0x03;
            #[doc = "- 128 Seconds."]
            pub const WT_255: u16 = 0xff;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Watchdog Service Register"]
pub mod WSR {
    pub use crate::RW as access;
    #[doc = "WSR"]
    pub mod WSR {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
            pub const WSR_21845: u16 = 0x5555;
            #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
            pub const WSR_43690: u16 = 0xaaaa;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Watchdog Reset Status Register"]
pub mod WRSR {
    pub use crate::RO as access;
    #[doc = "SFTW"]
    pub mod SFTW {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not the result of a software reset."]
            pub const SFTW_0: u16 = 0;
            #[doc = "Reset is the result of a software reset."]
            pub const SFTW_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "TOUT"]
    pub mod TOUT {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not the result of a WDOG timeout."]
            pub const TOUT_0: u16 = 0;
            #[doc = "Reset is the result of a WDOG timeout."]
            pub const TOUT_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "POR"]
    pub mod POR {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset is not the result of a power on reset."]
            pub const POR_0: u16 = 0;
            #[doc = "Reset is the result of a power on reset."]
            pub const POR_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Watchdog Interrupt Control Register"]
pub mod WICR {
    pub use crate::RW as access;
    #[doc = "WICT"]
    pub mod WICT {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "WICT\\[7:0\\] = Time duration between interrupt and time-out is 0 seconds."]
            pub const WICT_0: u16 = 0;
            #[doc = "WICT\\[7:0\\] = Time duration between interrupt and time-out is 0.5 seconds."]
            pub const WICT_1: u16 = 0x01;
            #[doc = "WICT\\[7:0\\] = Time duration between interrupt and time-out is 2 seconds (Default)."]
            pub const WICT_4: u16 = 0x04;
            #[doc = "WICT\\[7:0\\] = Time duration between interrupt and time-out is 127.5 seconds."]
            pub const WICT_255: u16 = 0xff;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WTIS"]
    pub mod WTIS {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No interrupt has occurred (Default)."]
            pub const WTIS_0: u16 = 0;
            #[doc = "Interrupt has occurred"]
            pub const WTIS_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WIE"]
    pub mod WIE {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable Interrupt (Default)."]
            pub const WIE_0: u16 = 0;
            #[doc = "Enable Interrupt."]
            pub const WIE_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Watchdog Miscellaneous Control Register"]
pub mod WMCR {
    pub use crate::RW as access;
    #[doc = "PDE"]
    pub mod PDE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Power Down Counter of WDOG is disabled."]
            pub const PDE_0: u16 = 0;
            #[doc = "Power Down Counter of WDOG is enabled (Default)."]
            pub const PDE_1: u16 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
