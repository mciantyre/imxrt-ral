#[doc = "Timer"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module ID Register"]
    pub TMR_ID: u32,
    _reserved0: [u8; 0x04],
    #[doc = "Timer Capability Register"]
    pub TMR_CAPR: u32,
    _reserved1: [u8; 0x14],
    #[doc = "Timer free running time low register"]
    pub TMR_FRT_L: u32,
    #[doc = "Timer free running time high register"]
    pub TMR_FRT_H: u32,
    #[doc = "Timer synchronous time low register"]
    pub TMR_SRT_L: u32,
    #[doc = "Timer synchronous time high register."]
    pub TMR_SRT_H: u32,
    #[doc = "Default ns timer counter low register"]
    pub TMR_DEF_CNT_L: u32,
    #[doc = "Default ns timer counter high register"]
    pub TMR_DEF_CNT_H: u32,
    _reserved2: [u8; 0x48],
    #[doc = "Timer Control Register"]
    pub TMR_CTRL: u32,
    #[doc = "Timer Event Register"]
    pub TMR_TEVENT: u32,
    #[doc = "Timer event mask register"]
    pub TMR_TEMASK: u32,
    _reserved3: [u8; 0x08],
    #[doc = "Timer status register"]
    pub TMR_STAT: u32,
    #[doc = "Timer counter low register"]
    pub TMR_CNT_L: u32,
    #[doc = "Timer counter high register"]
    pub TMR_CNT_H: u32,
    #[doc = "Timer addend register"]
    pub TMR_ADD: u32,
    #[doc = "Timer accumulator register"]
    pub TMR_ACC: u32,
    #[doc = "Timer prescale register"]
    pub TMR_PRSC: u32,
    #[doc = "Extended timer control register"]
    pub TMR_ECTRL: u32,
    #[doc = "Timer offset low register"]
    pub TMROFF_L: u32,
    #[doc = "Timer offset high register"]
    pub TMROFF_H: u32,
    #[doc = "Alarm 1 time comparator low register"]
    pub TMR_ALARM1_L: u32,
    #[doc = "Alarm 1 time comparator high register"]
    pub TMR_ALARM1_H: u32,
    #[doc = "Alarm 2 time comparator low register"]
    pub TMR_ALARM2_L: u32,
    #[doc = "Alarm 2 time comparator high register"]
    pub TMR_ALARM2_H: u32,
    _reserved4: [u8; 0x04],
    #[doc = "Timer Alarm Control Register"]
    pub TMR_ALARM_CTRL: u32,
    #[doc = "Timer i fixed interval period register"]
    pub TMR_FIPER: [u32; 3usize],
    #[doc = "Timer FIPER Control Register"]
    pub TMR_FIPER_CTRL: u32,
    #[doc = "External trigger stamp register"]
    pub TMR_ETTS1_L: u32,
    #[doc = "External trigger stamp register"]
    pub TMR_ETTS1_H: u32,
    #[doc = "External trigger stamp register"]
    pub TMR_ETTS2_L: u32,
    #[doc = "External trigger stamp register"]
    pub TMR_ETTS2_H: u32,
    #[doc = "Timer current time low register"]
    pub TMR_CUR_TIME_L: u32,
    #[doc = "Timer current time high register"]
    pub TMR_CUR_TIME_H: u32,
    #[doc = "Timer parameter register"]
    pub TMR_PARAM: u32,
}
#[doc = "Module ID Register"]
pub mod TMR_ID {
    pub use crate::RO as access;
    #[doc = "Minor revision"]
    pub mod REV_MN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Major revision"]
    pub mod REV_MJ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer IP ID"]
    pub mod TMR_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Capability Register"]
pub mod TMR_CAPR {
    pub use crate::RO as access;
    #[doc = "IEEE 1722 support 0 Not supported 1 Supported"]
    pub mod IEEE_1722 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enhanced 1588 nanosecond (ns) Timer Adjustment supported"]
    pub mod ECADJ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Support 802.1AS-2020 by providing both a ns and free-running clock to the Ethernet MACs."]
    pub mod IEEE_8021AS_REV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of MSI-x Vectors supported"]
    pub mod NUM_MSIX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer free running time low register"]
pub mod TMR_FRT_L {
    pub use crate::RO as access;
    #[doc = "Read-only copy of the free running time (lower 32b)"]
    pub mod TMR_FRT_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer free running time high register"]
pub mod TMR_FRT_H {
    pub use crate::RO as access;
    #[doc = "Read-only copy of the free running time (upper 32b)"]
    pub mod TMR_FRT_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer synchronous time low register"]
pub mod TMR_SRT_L {
    pub use crate::RO as access;
    #[doc = "Read-only copy of the synchronous time (lower 32b)"]
    pub mod TMR_SRT_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer synchronous time high register."]
pub mod TMR_SRT_H {
    pub use crate::RO as access;
    #[doc = "Read-only copy of the synchronous time (upper 32b)"]
    pub mod TMR_SRT_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Default ns timer counter low register"]
pub mod TMR_DEF_CNT_L {
    pub use crate::RO as access;
    #[doc = "Read-only copy of the Default ns time counter (lower 32b)"]
    pub mod TMR_DEF_CNT_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Default ns timer counter high register"]
pub mod TMR_DEF_CNT_H {
    pub use crate::RO as access;
    #[doc = "Read-only copy of the Default ns time counter (upper 32b)"]
    pub mod TMR_DEF_CNT_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Control Register"]
pub mod TMR_CTRL {
    pub use crate::RW as access;
    #[doc = "1588 timer reference clock source select"]
    pub mod CK_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1588 timer enable"]
    pub mod TE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External oscillator input clock phase"]
    pub mod CIPH {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Generated clock (TMR_GCLK) output phase."]
    pub mod COPH {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 1 edge polarity"]
    pub mod ETEP1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 2 edge polarity"]
    pub mod ETEP2 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mode bit to allow atomic writes to TCLK_PERIOD and TMR_ADD"]
    pub mod COMP_MODE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1588 timer reference clock period"]
    pub mod TCLK_PERIOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Fiper2 pulse loop back mode enabled"]
    pub mod PP2L {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Fiper1 pulse loop back mode enabled"]
    pub mod PP1L {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIPER start indication"]
    pub mod FS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "shadow Register disable"]
    pub mod SHADOW_DIS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alarm2 output polarity"]
    pub mod ALM2P {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alarm1 output polarity"]
    pub mod ALM1P {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Event Register"]
pub mod TMR_TEVENT {
    pub use crate::RW as access;
    #[doc = "Periodic pulse event 3 enable"]
    pub mod PP3EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Periodic pulse event 2 enable"]
    pub mod PP2EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Periodic pulse event 1 enable"]
    pub mod PP1EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer ALM2 event enable"]
    pub mod ALM1EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer ALM1 event enable"]
    pub mod ALM2EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 1 timestamp FIFO Threshold Level Hit"]
    pub mod ETS1_THREN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 2 timestamp FIFO Threshold Level Hit"]
    pub mod ETS2_THREN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 1 new timestamp sample event available"]
    pub mod ETS1EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 2 new timestamp sample event available"]
    pub mod ETS2EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 1 timestamp FIFO Overflow event occurred"]
    pub mod ETS1_OVEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 2 timestamp FIFO Overflow event occurred"]
    pub mod ETS2_OVEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer event mask register"]
pub mod TMR_TEMASK {
    pub use crate::RW as access;
    #[doc = "Periodic pulse event 3 enable"]
    pub mod PP3EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Periodic pulse event 2 enable"]
    pub mod PP2EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Periodic pulse event 1 enable"]
    pub mod PP1EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer ALM2 event enable"]
    pub mod ALM1EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer ALM1 event enable"]
    pub mod ALM2EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 1 timestamp FIFO Threshold Level Hit event enable"]
    pub mod ETS1_THREN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 2 timestamp FIFO Threshold Level Hit event enable"]
    pub mod ETS2_THREN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 1 new timestamp sample event available interrupt enable"]
    pub mod ETS1EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 2 new timestamp sample event available interrupt enable"]
    pub mod ETS2EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 1 timestamp FIFO Overflow event interrupt enabled"]
    pub mod ETS1_OVEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 2 timestamp FIFO Overflow event interrupt enabled"]
    pub mod ETS2_OVEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer status register"]
pub mod TMR_STAT {
    pub use crate::RO as access;
    #[doc = "External trigger 1 Valid time-stamp 0 all valid external trigger time-stamps have been read 1 external trigger has an unread valid time-stamp value"]
    pub mod ETS1_VLD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External trigger 2 Valid time-stamp 0 all valid external trigger time-stamps have been read 1 external trigger has an unread valid time-stamp value"]
    pub mod ETS2_VLD {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Reference Clock Detected"]
    pub mod RCD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reference Clock has not been detected as active. Registers in timer clock domain are not allowed to be accessed; reads return 0, writes are ignored."]
            pub const REF_CLK_NOT_ACTIVE: u32 = 0;
            #[doc = "Reference Clock has been detected as active. Registers in timer clock domain are allowed to be accessed."]
            pub const REF_CLK_ACTIVE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer counter low register"]
pub mod TMR_CNT_L {
    pub use crate::RW as access;
    #[doc = "Timer counter register."]
    pub mod TMR_CNT_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer counter high register"]
pub mod TMR_CNT_H {
    pub use crate::RW as access;
    #[doc = "Timer counter register."]
    pub mod TMR_CNT_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer addend register"]
pub mod TMR_ADD {
    pub use crate::RW as access;
    #[doc = "Timer addend."]
    pub mod ADDEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer accumulator register"]
pub mod TMR_ACC {
    pub use crate::RO as access;
    #[doc = "32-bit timer accumulator register"]
    pub mod TMR_ACC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer prescale register"]
pub mod TMR_PRSC {
    pub use crate::RW as access;
    #[doc = "Output clock division prescale factor."]
    pub mod PRSC_OCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Extended timer control register"]
pub mod TMR_ECTRL {
    pub use crate::RW as access;
    #[doc = "External trigger FIFO threshold."]
    pub mod ETFF_THR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer offset low register"]
pub mod TMROFF_L {
    pub use crate::RW as access;
    #[doc = "Offset value of the clock counter."]
    pub mod TMROFF_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer offset high register"]
pub mod TMROFF_H {
    pub use crate::RW as access;
    #[doc = "Offset value of the clock counter."]
    pub mod TMROFF_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Alarm 1 time comparator low register"]
pub mod TMR_ALARM1_L {
    pub use crate::RW as access;
    #[doc = "Alarm time comparator register."]
    pub mod ALARM_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Alarm 1 time comparator high register"]
pub mod TMR_ALARM1_H {
    pub use crate::RW as access;
    #[doc = "Alarm time comparator register."]
    pub mod ALARM_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Alarm 2 time comparator low register"]
pub mod TMR_ALARM2_L {
    pub use crate::RW as access;
    #[doc = "Alarm time comparator register."]
    pub mod ALARM_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Alarm 2 time comparator high register"]
pub mod TMR_ALARM2_H {
    pub use crate::RW as access;
    #[doc = "Alarm time comparator register."]
    pub mod ALARM_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Alarm Control Register"]
pub mod TMR_ALARM_CTRL {
    pub use crate::RW as access;
    #[doc = "ALARM 1 pulse width selector"]
    pub mod ALARM1_PW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alarm1 pulse generation time"]
    pub mod PG1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ALARM 2 pulse width selector"]
    pub mod ALARM2_PW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alarm2 pulse generation time"]
    pub mod PG2 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer i fixed interval period register"]
pub mod TMR_FIPER {
    pub use crate::RW as access;
    #[doc = "Fixed Interval Pulse Period"]
    pub mod FIPER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer FIPER Control Register"]
pub mod TMR_FIPER_CTRL {
    pub use crate::RW as access;
    #[doc = "FIPER 1 pulse width selection"]
    pub mod FIPER1_PW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIPER1 pulse generation select"]
    pub mod PG1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIPER1 disable"]
    pub mod FIPER1_DIS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIPER 2 pulse width selection"]
    pub mod FIPER2_PW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIPER2 pulse generation time"]
    pub mod PG2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIPER2 disable"]
    pub mod FIPER2_DIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIPER 3 pulse width selection"]
    pub mod FIPER3_PW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIPER3 pulse generation time"]
    pub mod PG3 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIPER3 disable"]
    pub mod FIPER3_DIS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "External trigger stamp register"]
pub mod TMR_ETTS1_L {
    pub use crate::RO as access;
    #[doc = "Time stamp field at the programmable edge of the external trigger"]
    pub mod ETTS_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "External trigger stamp register"]
pub mod TMR_ETTS1_H {
    pub use crate::RO as access;
    #[doc = "Time stamp field at the programmable edge of the external trigger"]
    pub mod ETTS_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "External trigger stamp register"]
pub mod TMR_ETTS2_L {
    pub use crate::RO as access;
    #[doc = "Time stamp field at the programmable edge of the external trigger"]
    pub mod ETTS_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "External trigger stamp register"]
pub mod TMR_ETTS2_H {
    pub use crate::RO as access;
    #[doc = "Time stamp field at the programmable edge of the external trigger"]
    pub mod ETTS_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer current time low register"]
pub mod TMR_CUR_TIME_L {
    pub use crate::RO as access;
    #[doc = "Read-only copy of current time (lower 32b)"]
    pub mod TMR_CUR_TIME_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer current time high register"]
pub mod TMR_CUR_TIME_H {
    pub use crate::RO as access;
    #[doc = "Read-only copy of current time (upper 32b)"]
    pub mod TMR_CUR_TIME_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer parameter register"]
pub mod TMR_PARAM {
    pub use crate::RW as access;
    #[doc = "Timer synchronization"]
    pub mod SYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "User specific parameter values"]
    pub mod PARAM_VAL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
