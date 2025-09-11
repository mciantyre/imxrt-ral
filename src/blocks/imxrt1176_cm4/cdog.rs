#[doc = "CDOG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control"]
    pub CONTROL: u32,
    #[doc = "Instruction Timer reload"]
    pub RELOAD: u32,
    #[doc = "Instruction Timer"]
    pub INSTRUCTION_TIMER: u32,
    #[doc = "Secure Counter"]
    pub SECURE_COUNTER: u32,
    #[doc = "Status 1"]
    pub STATUS: u32,
    #[doc = "Status 2"]
    pub STATUS2: u32,
    #[doc = "Flags"]
    pub FLAGS: u32,
    #[doc = "Persistent Data Storage"]
    pub PERSISTENT: u32,
    #[doc = "START Command"]
    pub START: u32,
    #[doc = "STOP Command"]
    pub STOP: u32,
    #[doc = "RESTART Command"]
    pub RESTART: u32,
    #[doc = "ADD Command"]
    pub ADD: u32,
    #[doc = "ADD1 Command"]
    pub ADD1: u32,
    #[doc = "ADD16 Command"]
    pub ADD16: u32,
    #[doc = "ADD256 Command"]
    pub ADD256: u32,
    #[doc = "SUB Command"]
    pub SUB: u32,
    #[doc = "SUB1 Command"]
    pub SUB1: u32,
    #[doc = "SUB16 Command"]
    pub SUB16: u32,
    #[doc = "SUB256 Command"]
    pub SUB256: u32,
}
#[doc = "Control"]
pub mod CONTROL {
    pub use crate::RW as access;
    #[doc = "Lock control"]
    pub mod LOCK_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Locked"]
            pub const LOCKED: u32 = 0x01;
            #[doc = "Unlocked"]
            pub const UNLOCKED: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "TIMEOUT fault control"]
    pub mod TIMEOUT_CTRL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 0x01;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 0x02;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 0x04;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MISCOMPARE fault control"]
    pub mod MISCOMPARE_CTRL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 0x01;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 0x02;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 0x04;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEQUENCE fault control"]
    pub mod SEQUENCE_CTRL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 0x01;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 0x02;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 0x04;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CONTROL fault control"]
    pub mod CONTROL_CTRL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 0x01;
            #[doc = "Disable reset"]
            pub const DISABLE_BOTH: u32 = 0x04;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "STATE fault control"]
    pub mod STATE_CTRL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 0x01;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 0x02;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 0x04;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADDRESS fault control"]
    pub mod ADDRESS_CTRL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 0x01;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 0x02;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 0x04;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IRQ pause control"]
    pub mod IRQ_PAUSE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Keep the timer running"]
            pub const RUN_TIMER: u32 = 0x01;
            #[doc = "Stop the timer"]
            pub const PAUSE_TIMER: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DEBUG_HALT control"]
    pub mod DEBUG_HALT_CTRL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Keep the timer running"]
            pub const RUN_TIMER: u32 = 0x01;
            #[doc = "Stop the timer"]
            pub const PAUSE_TIMER: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Instruction Timer reload"]
pub mod RELOAD {
    pub use crate::RW as access;
    #[doc = "Instruction Timer reload value"]
    pub mod RLOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Instruction Timer"]
pub mod INSTRUCTION_TIMER {
    pub use crate::RW as access;
    #[doc = "Current value of the Instruction Timer"]
    pub mod INSTIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Secure Counter"]
pub mod SECURE_COUNTER {
    pub use crate::WO as access;
    #[doc = "Secure Counter"]
    pub mod SECCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Status 1"]
pub mod STATUS {
    pub use crate::RO as access;
    #[doc = "Number of TIMEOUT faults since the last POR"]
    pub mod NUMTOF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of MISCOMPARE faults since the last POR"]
    pub mod NUMMISCOMPF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of SEQUENCE faults since the last POR"]
    pub mod NUMILSEQF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Current State"]
    pub mod CURST {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Status 2"]
pub mod STATUS2 {
    pub use crate::RO as access;
    #[doc = "Number of CONTROL faults since the last POR"]
    pub mod NUMCNTF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of STATE faults since the last POR"]
    pub mod NUMILLSTF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of ADDRESS faults since the last POR"]
    pub mod NUMILLA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Flags"]
pub mod FLAGS {
    pub use crate::RW as access;
    #[doc = "TIMEOUT fault flag"]
    pub mod TO_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "A TIMEOUT fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A TIMEOUT fault has occurred"]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MISCOMPARE fault flag"]
    pub mod MISCOM_FLAG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "A MISCOMPARE fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A MISCOMPARE fault has occurred"]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SEQUENCE fault flag"]
    pub mod SEQ_FLAG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "A SEQUENCE fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A SEQUENCE fault has occurred"]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "CONTROL fault flag"]
    pub mod CNT_FLAG {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "A CONTROL fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A CONTROL fault has occurred"]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "STATE fault flag"]
    pub mod STATE_FLAG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "A STATE fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A STATE fault has occurred"]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADDRESS fault flag"]
    pub mod ADDR_FLAG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "An ADDRESS fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "An ADDRESS fault has occurred"]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Power-on reset flag"]
    pub mod POR_FLAG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "A Power-on reset event has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A Power-on reset event has occurred"]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Persistent Data Storage"]
pub mod PERSISTENT {
    pub use crate::RW as access;
    #[doc = "Persistent Storage"]
    pub mod PERSIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "START Command"]
pub mod START {
    pub use crate::WO as access;
    #[doc = "Start command"]
    pub mod STRT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "STOP Command"]
pub mod STOP {
    pub use crate::WO as access;
    #[doc = "Stop command"]
    pub mod STP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "RESTART Command"]
pub mod RESTART {
    pub use crate::WO as access;
    #[doc = "Restart command"]
    pub mod RSTRT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ADD Command"]
pub mod ADD {
    pub use crate::WO as access;
    #[doc = "ADD Write Value"]
    pub mod AD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ADD1 Command"]
pub mod ADD1 {
    pub use crate::WO as access;
    #[doc = "ADD 1"]
    pub mod AD1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ADD16 Command"]
pub mod ADD16 {
    pub use crate::WO as access;
    #[doc = "ADD 16"]
    pub mod AD16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "ADD256 Command"]
pub mod ADD256 {
    pub use crate::WO as access;
    #[doc = "ADD 256"]
    pub mod AD256 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SUB Command"]
pub mod SUB {
    pub use crate::WO as access;
    #[doc = "Subtract Write Value"]
    pub mod S0B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SUB1 Command"]
pub mod SUB1 {
    pub use crate::WO as access;
    #[doc = "Subtract 1"]
    pub mod S1B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SUB16 Command"]
pub mod SUB16 {
    pub use crate::WO as access;
    #[doc = "Subtract 16"]
    pub mod SB16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SUB256 Command"]
pub mod SUB256 {
    pub use crate::WO as access;
    #[doc = "Subtract 256"]
    pub mod SB256 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
