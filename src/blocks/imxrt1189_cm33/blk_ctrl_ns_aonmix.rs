#[doc = "Block Control Non-Secure AON Domain"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPC CORE SLEEP Request Select"]
    pub GPC_CFG: u32,
    _reserved0: [u8; 0x04],
    #[doc = "IPG Debug mask"]
    pub IPG_DEBUG: u32,
    _reserved1: [u8; 0x10],
    #[doc = "SSI Master: AXI Async Bridge from AXIM to NIC400. Low power mode control."]
    pub SSI: u32,
    #[doc = "SAI1 MCLK control register"]
    pub SAI1_MCLK_CTRL: u32,
    #[doc = "DCDC status register"]
    pub DCDC_STATUS: u32,
    #[doc = "Fuse access disable register"]
    pub FUSE_ACC_DIS: u32,
    #[doc = "M33 NMI interrupt clear register"]
    pub M33_NMI_CLR: u32,
    #[doc = "I3C1 async wakeup control register"]
    pub I3C1_ASYNC_WAKEUP_CTRL: u32,
    #[doc = "Miscellaneous control register of IO"]
    pub MISC_IO_CTRL: u32,
}
#[doc = "GPC CORE SLEEP Request Select"]
pub mod GPC_CFG {
    pub use crate::RW as access;
    #[doc = "M33 SLEEP Request Select"]
    pub mod M33_SLEEP_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select SLEEPING as request source"]
            pub const SLEEPING: u32 = 0;
            #[doc = "Select SLEEPDEEP as request source"]
            pub const SLEEPDEEP: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "M7 SLEEP Request Select"]
    pub mod M7_SLEEP_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select SLEEPING as request source"]
            pub const SLEEPING: u32 = 0;
            #[doc = "Select SLEEPDEEP as request source"]
            pub const SLEEPDEEP: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IPG Debug mask"]
pub mod IPG_DEBUG {
    pub use crate::RW as access;
    #[doc = "Mask bit for CAN1 debug halted mode with M33 core"]
    pub mod M33_CAN1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "CAN1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "CAN1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for EDMA3 debug halted mode with M33 core"]
    pub mod M33_EDMA3 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "EDMA3 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "EDMA3 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for LPI2C1 debug halted mode with M33 core"]
    pub mod M33_LPI2C1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPI2C1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPI2C1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for LPI2C2 debug halted mode with M33 core"]
    pub mod M33_LPI2C2 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPI2C2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPI2C2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for LPIT1 debug halted mode with M33 core"]
    pub mod M33_LPIT1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPIT1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPIT1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for LPSPI1 debug halted mode with M33 core"]
    pub mod M33_LPSPI1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPSPI1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPSPI1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for LPSPI2 debug halted mode with M33 core"]
    pub mod M33_LPSPI2 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPSPI2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPSPI2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for LPTMR1 debug halted mode with M33 core"]
    pub mod M33_LPTMR1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPTMR1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "LPTMR1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for SAI1 debug halted mode with M33 core"]
    pub mod M33_SAI1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "SAI1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "SAI1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for TPM1 debug halted mode with M33 core"]
    pub mod M33_TPM1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TPM1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "TPM1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for TPM2 debug halted mode with M33 core"]
    pub mod M33_TPM2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TPM2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "TPM2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for WDOG1 debug halted mode with M33 core"]
    pub mod M33_WDOG1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "WDOG1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "WDOG1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for WDOG2 debug halted mode with M33 core"]
    pub mod M33_WDOG2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "WDOG2 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "WDOG2 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for GPT1 debug halted mode with M33 core"]
    pub mod M33_GPT1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "GPT1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "GPT1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for CAN3 debug halted mode with M33 core"]
    pub mod M33_CAN3 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "CAN3 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "CAN3 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for I3C1 debug halted mode with M33 core"]
    pub mod M33_I3C1 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "I3C1 does not enter debug halted mode with CM33"]
            pub const MASK: u32 = 0;
            #[doc = "I3C1 enters debug halted mode when CM33 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for CAN1 debug halted mode with M7 core"]
    pub mod M7_CAN1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "CAN1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "CAN1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for EDMA3 debug halted mode with M7 core"]
    pub mod M7_EDMA3 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "EDMA3 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "EDMA3 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for LPI2C1 debug halted mode with M7 core"]
    pub mod M7_LPI2C1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPI2C1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPI2C1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for LPI2C2 debug halted mode with M7 core"]
    pub mod M7_LPI2C2 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPI2C2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPI2C2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for LPIT1 debug halted mode with M7 core"]
    pub mod M7_LPIT1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPIT1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPIT1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for LPSPI1 debug halted mode with M7 core"]
    pub mod M7_LPSPI1 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPSPI1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPSPI1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for LPSPI2 debug halted mode with M7 core"]
    pub mod M7_LPSPI2 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPSPI2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPSPI2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for LPTMR1 debug halted mode with M7 core"]
    pub mod M7_LPTMR1 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPTMR1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "LPTMR1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for SAI1 debug halted mode with M7 core"]
    pub mod M7_SAI1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "SAI1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "SAI1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for TPM1 debug halted mode with M7 core"]
    pub mod M7_TPM1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TPM1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "TPM1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for TPM2 debug halted mode with M7 core"]
    pub mod M7_TPM2 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TPM2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "TPM2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for WDOG1 debug halted mode with M7 core"]
    pub mod M7_WDOG1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "WDOG1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "WDOG1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for WDOG2 debug halted mode with M7 core"]
    pub mod M7_WDOG2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "WDOG2 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "WDOG2 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for GPT1 debug halted mode with M7 core"]
    pub mod M7_GPT1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "GPT1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "GPT1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for CAN3 debug halted mode with M7 core"]
    pub mod M7_CAN3 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "CAN3 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "CAN3 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mask bit for I3C1 debug halted mode with M7 core"]
    pub mod M7_I3C1 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "I3C1 does not enter debug halted mode with CM7"]
            pub const MASK: u32 = 0;
            #[doc = "I3C1 enters debug halted mode when CM7 is debug halted"]
            pub const UNMASK: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SSI Master: AXI Async Bridge from AXIM to NIC400. Low power mode control."]
pub mod SSI {
    pub use crate::RW as access;
    #[doc = "AON Domain SSI master pause mode"]
    pub mod PAUSE_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "AON Domain SSI master is not in pause mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "AON Domain SSI master is in pause mode"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "AON Domain SSI master blackhole mode"]
    pub mod BLKHOLE_MODE_B {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "AON Domain SSI master will enter into blackhole mode"]
            pub const ENABLE: u32 = 0;
            #[doc = "AON Domain SSI master will exit from blackhole mode"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "SAI1 MCLK control register"]
pub mod SAI1_MCLK_CTRL {
    pub use crate::RW as access;
    #[doc = "SAI1_MCLK IO direction control. IOMUX need select SAI1 MCLK function"]
    pub mod SAI1_MCLK_DIR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "SAI1_MCLK is input signal"]
            pub const INPUT: u32 = 0;
            #[doc = "SAI1_MCLK is output signal"]
            pub const OUTPUT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DCDC status register"]
pub mod DCDC_STATUS {
    pub use crate::RW as access;
    #[doc = "DCDC captured status clear"]
    pub mod DCDC_STATUS_CAPT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No change"]
            pub const OVER: u32 = 0;
            #[doc = "Clear the 3 bits of DCDC captured status: DCDC_OVER_VOL, DCDC_OVER_CUR, and DCDC_IN_LOW_VOL"]
            pub const NO: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DCDC_IN low voltage detect"]
    pub mod DCDC_IN_LOW_VOL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Voltage on DCDC_IN is higher than 2.6V"]
            pub const NO: u32 = 0;
            #[doc = "Voltage on DCDC_IN is lower than 2.6V"]
            pub const OVER: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DCDC output over current alert"]
    pub mod DCDC_OVER_CUR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Overcurrent on DCDC output"]
            pub const NO: u32 = 0;
            #[doc = "Overcurrent on DCDC output"]
            pub const OVER: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DCDC output over voltage alert"]
    pub mod DCDC_OVER_VOL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No Overvoltage on DCDC VDDLP0 or VDDLP8 output"]
            pub const NO: u32 = 0;
            #[doc = "Overvoltage on DCDC VDDLP0 or VDDLP8 output"]
            pub const OVERVOLTAGE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DCDC status OK"]
    pub mod DCDC_STS_DC_OK {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DCDC is settling"]
            pub const DISABLE: u32 = 0;
            #[doc = "DCDC already settled"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Fuse access disable register"]
pub mod FUSE_ACC_DIS {
    pub use crate::RO as access;
    #[doc = "Fuse read disable flag"]
    pub mod OSCCA_FUSE_READ_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Read is allowed"]
            pub const ENABLE: u32 = 0;
            #[doc = "Read is not allowed"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Fuse calibrate flag"]
    pub mod OCOTP_CALIBRATED {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "OCOTP is not calibrated"]
            pub const ENABLE: u32 = 0;
            #[doc = "OCOTP is calibrated"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "OCOTP busy flag"]
    pub mod OCOTP_BUSY {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "OCOTP is not busy"]
            pub const ENABLE: u32 = 0;
            #[doc = "OCOTP is busy"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "M33 NMI interrupt clear register"]
pub mod M33_NMI_CLR {
    pub use crate::RW as access;
    #[doc = "Clear CM33 NMI holding register"]
    pub mod M33_NMI_CLEAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "I3C1 async wakeup control register"]
pub mod I3C1_ASYNC_WAKEUP_CTRL {
    pub use crate::RW as access;
    #[doc = "Async wakeup interrupt clear"]
    pub mod IRQ_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Async wakeup interrupt status"]
    pub mod IRQ_STATUS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt not asserted"]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt asserted"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Miscellaneous control register of IO"]
pub mod MISC_IO_CTRL {
    pub use crate::RW as access;
    #[doc = "Disable I3C on-chip strong pull for I3C1"]
    pub mod I3C_ON_CHIP_STRONG_PULL_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "On-chip strong pull is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "On-chip strong pull is disabled"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "GPIO_AON IO bank supply voltage range selection"]
    pub mod GPIO_AON_HIGH_RANGE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "GPIO_AON IO bank supply voltage range selection"]
    pub mod GPIO_AON_LOW_RANGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
