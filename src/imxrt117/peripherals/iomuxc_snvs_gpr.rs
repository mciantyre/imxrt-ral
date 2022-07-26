#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC SNVS GPR
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// GPR0 General Purpose Register
pub mod GPR0 {

    /// General purpose bits
    pub mod GPR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR0 General Purpose Register
pub mod GPR1 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR2 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR3 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR4 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR5 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR6 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR7 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR8 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR9 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR10 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR11 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR12 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR13 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR14 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR15 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR16 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR17 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR18 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR19 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR20 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR21 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR22 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR23 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR24 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR25 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR26 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR27 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR28 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR29 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR30 {
    pub use super::GPR0::GPR;
}

/// GPR0 General Purpose Register
pub mod GPR31 {
    pub use super::GPR0::GPR;
}

/// GPR32 General Purpose Register
pub mod GPR32 {

    /// General purpose bits
    pub mod GPR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (15 bits: 0x7fff << 1)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lock the write to bit 15:0
    pub mod LOCK {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR33 General Purpose Register
pub mod GPR33 {

    /// DCDC captured status clear
    pub mod DCDC_STATUS_CAPT_CLR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No change
            pub const OVER: u32 = 0b0;

            /// 0b1: Clear the 3 bits of DCDC captured status: DCDC_OVER_VOL, DCDC_OVER_CUR, and DCDC_IN_LOW_VOL
            pub const NO: u32 = 0b1;
        }
    }

    /// SNVS LDO_SNVS_ANA bypass enable
    pub mod SNVS_BYPASS_EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable bypass
            pub const NO: u32 = 0b0;

            /// 0b1: Enable bypass
            pub const OVER: u32 = 0b1;
        }
    }

    /// DCDC_IN low voltage detect
    pub mod DCDC_IN_LOW_VOL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Voltage on DCDC_IN is higher than 2.6V
            pub const NO: u32 = 0b0;

            /// 0b1: Voltage on DCDC_IN is lower than 2.6V
            pub const OVER: u32 = 0b1;
        }
    }

    /// DCDC output over current alert
    pub mod DCDC_OVER_CUR {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Overcurrent on DCDC output
            pub const NO: u32 = 0b0;

            /// 0b1: Overcurrent on DCDC output
            pub const OVER: u32 = 0b1;
        }
    }

    /// DCDC output over voltage alert
    pub mod DCDC_OVER_VOL {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Overvoltage on DCDC VDDLP0 or VDDLP8 output
            pub const NO: u32 = 0b0;

            /// 0b1: Overvoltage on DCDC VDDLP0 or VDDLP8 output
            pub const OVERVOLTAGE: u32 = 0b1;
        }
    }

    /// DCDC status OK
    pub mod DCDC_STS_DC_OK {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DCDC is settling
            pub const DISABLE: u32 = 0b0;

            /// 0b1: DCDC already settled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// 32K OSC ok flag
    pub mod SNVS_XTAL_CLK_OK {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 32K oscillator is NOT stable into normal operation
            pub const UNSTABLE: u32 = 0b0;

            /// 0b1: 32K oscillator is stable into normal operation
            pub const STABLE: u32 = 0b1;
        }
    }
}

/// GPR34 General Purpose Register
pub mod GPR34 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write access is not blocked
            pub const OVER1: u32 = 0b0;

            /// 0b1: Write access is blocked
            pub const NO1: u32 = 0b1;
        }
    }

    /// SNVS core voltage detect trim select
    pub mod SNVS_CORE_VOLT_DET_TRIM_SEL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The trimming codes are selected from eFuse
            pub const OVER1: u32 = 0b0;

            /// 0b1: The trimming codes of core voltage detectors used to change the voltage falling trip point are selected from SNVS_CORE_VOLT_DET_TRIM
            pub const NO1: u32 = 0b1;
        }
    }

    /// SNVS core voltage detect trim
    pub mod SNVS_CORE_VOLT_DET_TRIM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNVS clock detect trim select
    pub mod SNVS_CLK_DET_TRIM_SEL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The trimming codes are selected from eFuse
            pub const OVER1: u32 = 0b0;

            /// 0b1: The trimming codes of clock detector used to change the boundary frequencies are selected from SNVS_CLK_DET_TRIM
            pub const NO1: u32 = 0b1;
        }
    }

    /// SNVS clock detect trim bits
    pub mod SNVS_CLK_DET_TRIM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNVS clock detect offset of high boundary frequency
    pub mod SNVS_CLK_DET_OFFSET_HIGH {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No change (Default)
            pub const OVER: u32 = 0b00;

            /// 0b01: Add +5 to the Trim
            pub const NO: u32 = 0b01;

            /// 0b10: Add +10 to the trim
            pub const OVER1: u32 = 0b10;

            /// 0b11: Add -5 to the Trim
            pub const NO1: u32 = 0b11;
        }
    }

    /// SNVS clock detect offset of low boundary frequency
    pub mod SNVS_CLK_DET_OFFSET_LOW {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SNVS_CLK_DET_OFFSET_HIGH::RW;
    }

    /// SNVS OSC load capacitor trim select
    pub mod SNVS_CAP_TRIM_SEL {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The trimming codes are selected from eFuse
            pub const OVER: u32 = 0b0;

            /// 0b1: The trimming codes are used from SNVS_OSC_CAP_TRIM (osc32k's load capacitor)
            pub const NO: u32 = 0b1;
        }
    }

    /// SNVS OSC load capacitor trim
    pub mod SNVS_OSC_CAP_TRIM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR35 General Purpose Register
pub mod GPR35 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write access is not blocked
            pub const OVER1: u32 = 0b0;

            /// 0b1: Write access is blocked
            pub const NO1: u32 = 0b1;
        }
    }

    /// SNVS voltage detect trim select
    pub mod SNVS_VOLT_DET_TRIM_SEL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The trimming codes are selected from eFuse
            pub const OVER1: u32 = 0b0;

            /// 0b1: The trimming codes of voltage detectors to change the voltage boundaries in battery voltage detecting are selected from SNVS_VOLT_DET_TRIM
            pub const NO1: u32 = 0b1;
        }
    }

    /// SNVS voltage detect trim
    pub mod SNVS_VOLT_DET_TRIM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (8 bits: 0xff << 4)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNVS temperature detect trim select
    pub mod SNVS_TEMP_DET_TRIM_SEL {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The trimming codes are selected from eFuse
            pub const OVER1: u32 = 0b0;

            /// 0b1: The trimming codes to define the temperature boundaries of temperature detector are selected from SNVS_TEMP_DET_TRIM
            pub const NO1: u32 = 0b1;
        }
    }

    /// SNVS temperature detect trim
    pub mod SNVS_TEMP_DET_TRIM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNVS temperature detect offset of high temperature boundary
    pub mod SNVS_TEMP_DET_OFFSET_HIGH {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No change (Default)
            pub const OVER: u32 = 0b00;

            /// 0b01: Add +5 to the Trim
            pub const NO: u32 = 0b01;

            /// 0b10: Add +10 to the trim
            pub const OVER1: u32 = 0b10;

            /// 0b11: Add -5 to the Trim
            pub const NO1: u32 = 0b11;
        }
    }

    /// SNVS temperature detect offset of low temperature boundary
    pub mod SNVS_TEMP_DET_OFFSET_LOW {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SNVS_TEMP_DET_OFFSET_HIGH::RW;
    }
}

/// GPR36 General Purpose Register
pub mod GPR36 {

    /// SNVS RAM isolation enable bit
    pub mod SNVSDIG_SNVS1P8_ISO_EN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enable SRAM access (It should be cleared after LDO_SNVS_DIG and SNVS SRAM peripheral power is back)
            pub const DIS: u32 = 0b0;

            /// 0b1: Enable the isolation to avoid extra leakage power before SNVS SRAM peripheral power or LDO_SNVS_DIG is switched off
            pub const EN: u32 = 0b1;
        }
    }

    /// SNVS SRAM power-down enable bit
    pub mod SNVS_SRAM_SLEEP {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enable SRAM access (It should be cleared after LDO_SNVS_DIG is enabled)
            pub const DIS: u32 = 0b0;

            /// 0b1: SNVS SRAM can go in Shutdown/ Periphery Off Array On/ Periphery On Array Off mode. In addition, this bit ensures power-up without stuck-at /high DC current states and hence must be held to 1 during wake-up, so this bit is default high.
            pub const EN: u32 = 0b1;
        }
    }

    /// SNVS SRAM standby enable bit
    pub mod SNVS_SRAM_STDBY {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SNVS SRAM does not enter low leakage state
            pub const No: u32 = 0b0;

            /// 0b1: SNVS SRAM enters low leakage state and large drivers are switched OFF
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// SNVS SRAM large switch control bit for peripheral
    pub mod SNVS_SRAM_PSWLARGEMP_FORCE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Switch on SNVS SRAM power for peripheral
            pub const No: u32 = 0b0;

            /// 0b1: Switch off SNVS SRAM power for peripheral (SRAM array power is not impacted, and data can be retained)
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// SNVS SRAM large switch control bit
    pub mod SNVS_SRAM_PSWLARGE {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Switch on SNVS SRAM power for peripheral and array
            pub const No: u32 = 0b0;

            /// 0b1: Switch off SNVS SRAM power for peripheral and array
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// SNVS SRAM small switch control bit for peripheral
    pub mod SNVS_SRAM_PSWSMALLMP_FORCE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SNVS_SRAM_PSWLARGEMP_FORCE::RW;
    }

    /// SNVS SRAM small switch control bit
    pub mod SNVS_SRAM_PSWSMALL {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SNVS_SRAM_PSWLARGE::RW;
    }
}

/// GPR37 General Purpose Register
pub mod GPR37 {

    /// Lock the write to bit 31:1
    pub mod LOCK {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write access is not blocked
            pub const OVER1: u32 = 0b0;

            /// 0b1: Write access is blocked
            pub const NO1: u32 = 0b1;
        }
    }

    /// SNVS tamper detect pin pull enable bit
    pub mod SNVS_TAMPER_PUE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (10 bits: 0x3ff << 1)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNVS tamper detect pin pull selection bit
    pub mod SNVS_TAMPER_PUS {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (10 bits: 0x3ff << 11)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// GPR0 General Purpose Register
    pub GPR0: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR1: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR2: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR3: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR4: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR5: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR6: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR7: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR8: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR9: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR10: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR11: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR12: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR13: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR14: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR15: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR16: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR17: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR18: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR19: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR20: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR21: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR22: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR23: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR24: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR25: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR26: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR27: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR28: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR29: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR30: RWRegister<u32>,

    /// GPR0 General Purpose Register
    pub GPR31: RWRegister<u32>,

    /// GPR32 General Purpose Register
    pub GPR32: RWRegister<u32>,

    /// GPR33 General Purpose Register
    pub GPR33: RWRegister<u32>,

    /// GPR34 General Purpose Register
    pub GPR34: RWRegister<u32>,

    /// GPR35 General Purpose Register
    pub GPR35: RWRegister<u32>,

    /// GPR36 General Purpose Register
    pub GPR36: RWRegister<u32>,

    /// GPR37 General Purpose Register
    pub GPR37: RWRegister<u32>,
}
pub struct ResetValues {
    pub GPR0: u32,
    pub GPR1: u32,
    pub GPR2: u32,
    pub GPR3: u32,
    pub GPR4: u32,
    pub GPR5: u32,
    pub GPR6: u32,
    pub GPR7: u32,
    pub GPR8: u32,
    pub GPR9: u32,
    pub GPR10: u32,
    pub GPR11: u32,
    pub GPR12: u32,
    pub GPR13: u32,
    pub GPR14: u32,
    pub GPR15: u32,
    pub GPR16: u32,
    pub GPR17: u32,
    pub GPR18: u32,
    pub GPR19: u32,
    pub GPR20: u32,
    pub GPR21: u32,
    pub GPR22: u32,
    pub GPR23: u32,
    pub GPR24: u32,
    pub GPR25: u32,
    pub GPR26: u32,
    pub GPR27: u32,
    pub GPR28: u32,
    pub GPR29: u32,
    pub GPR30: u32,
    pub GPR31: u32,
    pub GPR32: u32,
    pub GPR33: u32,
    pub GPR34: u32,
    pub GPR35: u32,
    pub GPR36: u32,
    pub GPR37: u32,
}
pub struct Instance<const N: u8> {
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) addr: u32,
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) intrs: &'static [crate::Interrupt],
}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> ::core::ops::Deref for Instance<N> {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

#[cfg(not(feature = "nosync"))]
unsafe impl<const N: u8> Send for Instance<N> {}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> Instance<N> {
    /// Return the interrupt signals associated with this
    /// peripheral instance
    ///
    /// Collection may be empty if there is no interrupt signal
    /// associated with the peripheral. There's no guarantee for
    /// interrupt signal ordering in the collection.
    #[inline(always)]
    pub const fn interrupts<'a>(&'a self) -> &'a [crate::Interrupt] {
        self.intrs
    }
}
