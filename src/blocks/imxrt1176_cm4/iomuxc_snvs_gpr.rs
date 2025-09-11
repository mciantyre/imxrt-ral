#[doc = "IOMUXC SNVS GPR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPR0 General Purpose Register"]
    pub GPR: [u32; 32usize],
    #[doc = "GPR32 General Purpose Register"]
    pub GPR32: u32,
    #[doc = "GPR33 General Purpose Register"]
    pub GPR33: u32,
    #[doc = "GPR34 General Purpose Register"]
    pub GPR34: u32,
    #[doc = "GPR35 General Purpose Register"]
    pub GPR35: u32,
    #[doc = "GPR36 General Purpose Register"]
    pub GPR36: u32,
    #[doc = "GPR37 General Purpose Register"]
    pub GPR37: u32,
}
#[doc = "GPR0 General Purpose Register"]
pub mod GPR {
    pub use crate::RW as access;
    #[doc = "General purpose bits"]
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
#[doc = "GPR32 General Purpose Register"]
pub mod GPR32 {
    pub use crate::RW as access;
    #[doc = "General purpose bits"]
    pub mod GPR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Lock the write to bit 15:0"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR33 General Purpose Register"]
pub mod GPR33 {
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
    #[doc = "SNVS LDO_SNVS_ANA bypass enable"]
    pub mod SNVS_BYPASS_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable bypass"]
            pub const NO: u32 = 0;
            #[doc = "Enable bypass"]
            pub const OVER: u32 = 0x01;
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
    #[doc = "32K OSC ok flag"]
    pub mod SNVS_XTAL_CLK_OK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "32K oscillator is NOT stable into normal operation"]
            pub const UNSTABLE: u32 = 0;
            #[doc = "32K oscillator is stable into normal operation"]
            pub const STABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR34 General Purpose Register"]
pub mod GPR34 {
    pub use crate::RW as access;
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write access is not blocked"]
            pub const OVER1: u32 = 0;
            #[doc = "Write access is blocked"]
            pub const NO1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS core voltage detect trim select"]
    pub mod SNVS_CORE_VOLT_DET_TRIM_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The trimming codes are selected from eFuse"]
            pub const OVER1: u32 = 0;
            #[doc = "The trimming codes of core voltage detectors used to change the voltage falling trip point are selected from SNVS_CORE_VOLT_DET_TRIM"]
            pub const NO1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS core voltage detect trim"]
    pub mod SNVS_CORE_VOLT_DET_TRIM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS clock detect trim select"]
    pub mod SNVS_CLK_DET_TRIM_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The trimming codes are selected from eFuse"]
            pub const OVER1: u32 = 0;
            #[doc = "The trimming codes of clock detector used to change the boundary frequencies are selected from SNVS_CLK_DET_TRIM"]
            pub const NO1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS clock detect trim bits"]
    pub mod SNVS_CLK_DET_TRIM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS clock detect offset of high boundary frequency"]
    pub mod SNVS_CLK_DET_OFFSET_HIGH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No change (Default)"]
            pub const OVER: u32 = 0;
            #[doc = "Add +5 to the Trim"]
            pub const NO: u32 = 0x01;
            #[doc = "Add +10 to the trim"]
            pub const OVER1: u32 = 0x02;
            #[doc = "Add -5 to the Trim"]
            pub const NO1: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS clock detect offset of low boundary frequency"]
    pub mod SNVS_CLK_DET_OFFSET_LOW {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No change (Default)"]
            pub const OVER: u32 = 0;
            #[doc = "Add +5 to the Trim"]
            pub const NO: u32 = 0x01;
            #[doc = "Add +10 to the trim"]
            pub const OVER1: u32 = 0x02;
            #[doc = "Add -5 to the Trim"]
            pub const NO1: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS OSC load capacitor trim select"]
    pub mod SNVS_CAP_TRIM_SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The trimming codes are selected from eFuse"]
            pub const OVER: u32 = 0;
            #[doc = "The trimming codes are used from SNVS_OSC_CAP_TRIM (osc32k's load capacitor)"]
            pub const NO: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS OSC load capacitor trim"]
    pub mod SNVS_OSC_CAP_TRIM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR35 General Purpose Register"]
pub mod GPR35 {
    pub use crate::RW as access;
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write access is not blocked"]
            pub const OVER1: u32 = 0;
            #[doc = "Write access is blocked"]
            pub const NO1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS voltage detect trim select"]
    pub mod SNVS_VOLT_DET_TRIM_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The trimming codes are selected from eFuse"]
            pub const OVER1: u32 = 0;
            #[doc = "The trimming codes of voltage detectors to change the voltage boundaries in battery voltage detecting are selected from SNVS_VOLT_DET_TRIM"]
            pub const NO1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS voltage detect trim"]
    pub mod SNVS_VOLT_DET_TRIM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS temperature detect trim select"]
    pub mod SNVS_TEMP_DET_TRIM_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The trimming codes are selected from eFuse"]
            pub const OVER1: u32 = 0;
            #[doc = "The trimming codes to define the temperature boundaries of temperature detector are selected from SNVS_TEMP_DET_TRIM"]
            pub const NO1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS temperature detect trim"]
    pub mod SNVS_TEMP_DET_TRIM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS temperature detect offset of high temperature boundary"]
    pub mod SNVS_TEMP_DET_OFFSET_HIGH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No change (Default)"]
            pub const OVER: u32 = 0;
            #[doc = "Add +5 to the Trim"]
            pub const NO: u32 = 0x01;
            #[doc = "Add +10 to the trim"]
            pub const OVER1: u32 = 0x02;
            #[doc = "Add -5 to the Trim"]
            pub const NO1: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS temperature detect offset of low temperature boundary"]
    pub mod SNVS_TEMP_DET_OFFSET_LOW {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No change (Default)"]
            pub const OVER: u32 = 0;
            #[doc = "Add +5 to the Trim"]
            pub const NO: u32 = 0x01;
            #[doc = "Add +10 to the trim"]
            pub const OVER1: u32 = 0x02;
            #[doc = "Add -5 to the Trim"]
            pub const NO1: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR36 General Purpose Register"]
pub mod GPR36 {
    pub use crate::RW as access;
    #[doc = "SNVS RAM isolation enable bit"]
    pub mod SNVSDIG_SNVS1P8_ISO_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable SRAM access (It should be cleared after LDO_SNVS_DIG and SNVS SRAM peripheral power is back)"]
            pub const DIS: u32 = 0;
            #[doc = "Enable the isolation to avoid extra leakage power before SNVS SRAM peripheral power or LDO_SNVS_DIG is switched off"]
            pub const EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS SRAM power-down enable bit"]
    pub mod SNVS_SRAM_SLEEP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enable SRAM access (It should be cleared after LDO_SNVS_DIG is enabled)"]
            pub const DIS: u32 = 0;
            #[doc = "SNVS SRAM can go in Shutdown/ Periphery Off Array On/ Periphery On Array Off mode. In addition, this bit ensures power-up without stuck-at /high DC current states and hence must be held to 1 during wake-up, so this bit is default high."]
            pub const EN: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS SRAM standby enable bit"]
    pub mod SNVS_SRAM_STDBY {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "SNVS SRAM does not enter low leakage state"]
            pub const NO: u32 = 0;
            #[doc = "SNVS SRAM enters low leakage state and large drivers are switched OFF"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS SRAM large switch control bit for peripheral"]
    pub mod SNVS_SRAM_PSWLARGEMP_FORCE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Switch on SNVS SRAM power for peripheral"]
            pub const NO: u32 = 0;
            #[doc = "Switch off SNVS SRAM power for peripheral (SRAM array power is not impacted, and data can be retained)"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS SRAM large switch control bit"]
    pub mod SNVS_SRAM_PSWLARGE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Switch on SNVS SRAM power for peripheral and array"]
            pub const NO: u32 = 0;
            #[doc = "Switch off SNVS SRAM power for peripheral and array"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS SRAM small switch control bit for peripheral"]
    pub mod SNVS_SRAM_PSWSMALLMP_FORCE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Switch on SNVS SRAM power for peripheral"]
            pub const NO: u32 = 0;
            #[doc = "Switch off SNVS SRAM power for peripheral (SRAM array power is not impacted, and data can be retained)"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS SRAM small switch control bit"]
    pub mod SNVS_SRAM_PSWSMALL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Switch on SNVS SRAM power for peripheral and array"]
            pub const NO: u32 = 0;
            #[doc = "Switch off SNVS SRAM power for peripheral and array"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "GPR37 General Purpose Register"]
pub mod GPR37 {
    pub use crate::RW as access;
    #[doc = "Lock the write to bit 31:1"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Write access is not blocked"]
            pub const OVER1: u32 = 0;
            #[doc = "Write access is blocked"]
            pub const NO1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS tamper detect pin pull enable bit"]
    pub mod SNVS_TAMPER_PUE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SNVS tamper detect pin pull selection bit"]
    pub mod SNVS_TAMPER_PUS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
