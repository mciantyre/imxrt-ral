#[doc = "M7 Systick module"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SysTick Control and Status Register"]
    pub SYST_CSR: u32,
    #[doc = "SysTick Reload Value Register"]
    pub SYST_RVR: u32,
    #[doc = "SysTick Current Value Register"]
    pub SYST_CVR: u32,
    #[doc = "SysTick Calibration Value Register"]
    pub SYST_CALIB: u32,
}
#[doc = "SysTick Control and Status Register"]
pub mod SYST_CSR {
    pub use crate::RW as access;
    #[doc = "Enable/disable systick counter"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "counter disabled"]
            pub const COUNTER_DISABLED: u32 = 0;
            #[doc = "counter enabled"]
            pub const COUNTER_ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Systick interrupt."]
    pub mod TICKINT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "counting down to 0 does not assert the SysTick exception request"]
            pub const INTERRUPT_DISABLED: u32 = 0;
            #[doc = "counting down to 0 asserts the SysTick exception request"]
            pub const INTERRUPT_ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Clock source selection."]
    pub mod CLKSOURCE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "external clock"]
            pub const EXTERNAL_CLOCK: u32 = 0;
            #[doc = "processor clock"]
            pub const PROCESSOR_CLOCK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Returns 1 if timer counted to 0 since the last read of this register."]
    pub mod COUNTFLAG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SysTick Reload Value Register"]
pub mod SYST_RVR {
    pub use crate::RW as access;
    #[doc = "Value to load into the SysTick Current Value Register when the counter reaches 0"]
    pub mod RELOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SysTick Current Value Register"]
pub mod SYST_CVR {
    pub use crate::RW as access;
    #[doc = "Reads current counter value at the time the register is accessed. Any write to the register clears the register to 0."]
    pub mod CURRENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SysTick Calibration Value Register"]
pub mod SYST_CALIB {
    pub use crate::RO as access;
    #[doc = "Reload value to use for 10ms timing"]
    pub mod TENMS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates whether the TENMS value is exact"]
    pub mod SKEW {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "10ms calibration value is exact"]
            pub const EXACT_VALUE: u32 = 0;
            #[doc = "10ms calibration value is inexact, because of the clock frequency"]
            pub const INEXACT_VALUE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates whether the device provides an alternative reference clock"]
    pub mod NOREF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The alternative reference clock is provided"]
            pub const CLOCK_PROVIDED: u32 = 0;
            #[doc = "The alternative reference clock is not provided"]
            pub const CLOCK_DISABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
