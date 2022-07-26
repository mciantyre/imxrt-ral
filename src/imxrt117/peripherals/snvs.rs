#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SNVS
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister, WORegister};

/// SNVS_HP Lock Register
pub mod HPLR {

    /// Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR
    pub mod ZMK_WSL {
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

            /// 0b0: Write access is allowed
            pub const WRITE_ALLOWED: u32 = 0b0;

            /// 0b1: Write access is not allowed
            pub const WRITE_NOT_ALLOWED: u32 = 0b1;
        }
    }

    /// Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR
    pub mod ZMK_RSL {
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

            /// 0b0: Read access is allowed (only in software Programming mode)
            pub const READ_ALLOWED: u32 = 0b0;

            /// 0b1: Read access is not allowed
            pub const READ_NOT_ALLOWED: u32 = 0b1;
        }
    }

    /// Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits
    pub mod SRTC_SL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WSL::RW;
    }

    /// LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)
    pub mod LPCALB_SL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WSL::RW;
    }

    /// Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit
    pub mod MC_SL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write access (increment) is allowed
            pub const WRITE_ALLOWED: u32 = 0b0;

            /// 0b1: Write access (increment) is not allowed
            pub const WRITE_NOT_ALLOWED: u32 = 0b1;
        }
    }

    /// General Purpose Register Soft Lock When set, prevents any writes to the GPR
    pub mod GPR_SL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WSL::RW;
    }

    /// LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR
    pub mod LPSVCR_SL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write access is allowed
            pub const WRITE_ACCESS_ALLOWED: u32 = 0b0;

            /// 0b1: Write access is not allowed
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0b1;
        }
    }

    /// LP Tamper Glitch Filter Configuration Register Soft Lock When set, prevents any writes to the LPTGFCR
    pub mod LPTGFCR_SL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSVCR_SL::RW;
    }

    /// LP Security Events Configuration Register Soft Lock When set, prevents any writes to the LPSECR
    pub mod LPSECR_SL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSVCR_SL::RW;
    }

    /// Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR
    pub mod MKS_SL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSVCR_SL::RW;
    }

    /// HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR
    pub mod HPSVCR_L {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSVCR_SL::RW;
    }

    /// HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR
    pub mod HPSICR_L {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSVCR_SL::RW;
    }

    /// High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR
    pub mod HAC_L {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LPSVCR_SL::RW;
    }

    /// Active Tamper 1 Soft Lock When set, prevents any writes to the Active Tamper 1 registers
    pub mod AT1_SL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write access is allowed.
            pub const WRITE_ACCESS_ALLOWED: u32 = 0b0;

            /// 0b1: Write access is not allowed.
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0b1;
        }
    }

    /// Active Tamper 2 Soft Lock When set, prevents any writes to the Active Tamper 2 registers
    pub mod AT2_SL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AT1_SL::RW;
    }

    /// Active Tamper 3 Soft Lock When set, prevents any writes to the Active Tamper 3 registers
    pub mod AT3_SL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AT1_SL::RW;
    }

    /// Active Tamper 4 Soft Lock When set, prevents any writes to the Active Tamper 4 registers
    pub mod AT4_SL {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AT1_SL::RW;
    }

    /// Active Tamper 5 Soft Lock When set, prevents any writes to the Active Tamper 5 registers
    pub mod AT5_SL {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AT1_SL::RW;
    }
}

/// SNVS_HP Command Register
pub mod HPCOMR {

    /// SSM State Transition Transition state of the system security monitor
    pub mod SSM_ST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state
    pub mod SSM_ST_DIS {
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

            /// 0b0: Secure to Trusted State transition is enabled
            pub const ENABLED: u32 = 0b0;

            /// 0b1: Secure to Trusted State transition is disabled
            pub const DISABLED: u32 = 0b1;
        }
    }

    /// SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state
    pub mod SSM_SFNS_DIS {
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

            /// 0b0: Soft Fail to Non-Secure State transition is enabled
            pub const ENABLED: u32 = 0b0;

            /// 0b1: Soft Fail to Non-Secure State transition is disabled
            pub const DISABLED: u32 = 0b1;
        }
    }

    /// LP Software Reset When set to 1, most registers in the SNVS_LP section are reset, but the following registers are not reset by an LP software reset: Monotonic Counter Secure Real Time Counter Time Alarm Register This bit cannot be set when the LP_SWR_DIS bit is set
    pub mod LP_SWR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Action
            pub const NO_ACTION: u32 = 0b0;

            /// 0b1: Reset LP section
            pub const RESET: u32 = 0b1;
        }
    }

    /// LP Software Reset Disable When set, disables the LP software reset
    pub mod LP_SWR_DIS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LP software reset is enabled
            pub const ENABLED: u32 = 0b0;

            /// 0b1: LP software reset is disabled
            pub const DISABLED: u32 = 0b1;
        }
    }

    /// Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation
    pub mod SW_SV {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation
    pub mod SW_FSV {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LP Software Security Violation When set, SNVS_LP treats this bit as a security violation
    pub mod SW_LPSV {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism
    pub mod PROG_ZMK {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Action
            pub const NO_ACTION: u32 = 0b0;

            /// 0b1: Activate hardware key programming mechanism
            pub const PROGRAM_KEY: u32 = 0b1;
        }
    }

    /// Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default
    pub mod MKS_EN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OTP master key is selected as an SNVS master key
            pub const SELECT_OTP: u32 = 0b0;

            /// 0b1: SNVS master key is selected according to the setting of the MASTER_KEY_SEL field of LPMKCR
            pub const SELECT_PER_LPMKCR: u32 = 0b1;
        }
    }

    /// High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state
    pub mod HAC_EN {
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

            /// 0b0: High Assurance Counter is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: High Assurance Counter is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register
    pub mod HAC_LOAD {
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

            /// 0b0: No Action
            pub const NO_ACTION: u32 = 0b0;

            /// 0b1: Load the HAC
            pub const LOAD_HAC: u32 = 0b1;
        }
    }

    /// High Assurance Counter Clear When set, it clears the High Assurance Counter Register
    pub mod HAC_CLEAR {
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

            /// 0b0: No Action
            pub const NO_ACTION: u32 = 0b0;

            /// 0b1: Clear the HAC
            pub const CLEAR_HAC: u32 = 0b1;
        }
    }

    /// High Assurance Counter Stop This bit can be set only when SSM is in soft fail state
    pub mod HAC_STOP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only
    pub mod NPSWA_EN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_HP Control Register
pub mod HPCR {

    /// HP Real Time Counter Enable
    pub mod RTC_EN {
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

            /// 0b0: RTC is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: RTC is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter
    pub mod HPTA_EN {
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

            /// 0b0: HP Time Alarm Interrupt is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: HP Time Alarm Interrupt is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Disable periodic interrupt in the functional interrupt
    pub mod DIS_PI {
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

            /// 0b0: Periodic interrupt will trigger a functional interrupt
            pub const ENABLED: u32 = 0b0;

            /// 0b1: Disable periodic interrupt in the function interrupt
            pub const DISABLED: u32 = 0b1;
        }
    }

    /// HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled
    pub mod PI_EN {
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

            /// 0b0: HP Periodic Interrupt is disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: HP Periodic Interrupt is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Periodic Interrupt Frequency Defines frequency of the periodic interrupt
    pub mod PI_FREQ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: - bit 0 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_0: u32 = 0b0000;

            /// 0b0001: - bit 1 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_1: u32 = 0b0001;

            /// 0b0010: - bit 2 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_2: u32 = 0b0010;

            /// 0b0011: - bit 3 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_3: u32 = 0b0011;

            /// 0b0100: - bit 4 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_4: u32 = 0b0100;

            /// 0b0101: - bit 5 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_5: u32 = 0b0101;

            /// 0b0110: - bit 6 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_6: u32 = 0b0110;

            /// 0b0111: - bit 7 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_7: u32 = 0b0111;

            /// 0b1000: - bit 8 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_8: u32 = 0b1000;

            /// 0b1001: - bit 9 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_9: u32 = 0b1001;

            /// 0b1010: - bit 10 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_10: u32 = 0b1010;

            /// 0b1011: - bit 11 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_11: u32 = 0b1011;

            /// 0b1100: - bit 12 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_12: u32 = 0b1100;

            /// 0b1101: - bit 13 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_13: u32 = 0b1101;

            /// 0b1110: - bit 14 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_14: u32 = 0b1110;

            /// 0b1111: - bit 15 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const USE_BIT_1r5: u32 = 0b1111;
        }
    }

    /// HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled.
    pub mod HPCALB_EN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: HP Timer calibration disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: HP Timer calibration enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// HP Calibration Value Defines signed calibration value for the HP Real Time Counter
    pub mod HPCALB_VAL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: +0 counts per each 32768 ticks of the counter
            pub const ADD_0_PER_32768_TICKS: u32 = 0b00000;

            /// 0b00001: +1 counts per each 32768 ticks of the counter
            pub const ADD_1_PER_32768_TICKS: u32 = 0b00001;

            /// 0b00010: +2 counts per each 32768 ticks of the counter
            pub const ADD_2_PER_32768_TICKS: u32 = 0b00010;

            /// 0b01111: +15 counts per each 32768 ticks of the counter
            pub const ADD_15_PER_32768_TICKS: u32 = 0b01111;

            /// 0b10000: -16 counts per each 32768 ticks of the counter
            pub const SUB_16_PER_32768_TICKS: u32 = 0b10000;

            /// 0b10001: -15 counts per each 32768 ticks of the counter
            pub const SUB_15_PER_32768_TICKS: u32 = 0b10001;

            /// 0b11110: -2 counts per each 32768 ticks of the counter
            pub const SUB_2_PER_32768_TICKS: u32 = 0b11110;

            /// 0b11111: -1 counts per each 32768 ticks of the counter
            pub const SUB_1_PER_32768_TICKS: u32 = 0b11111;
        }
    }

    /// HP Time Synchronize
    pub mod HP_TS {
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

            /// 0b0: No Action
            pub const NO_ACTION: u32 = 0b0;

            /// 0b1: Synchronize the HP Time Counter to the LP Time Counter
            pub const SYNC_TIME: u32 = 0b1;
        }
    }

    /// Button Configuration
    pub mod BTN_CONFIG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Button interrupt mask
    pub mod BTN_MASK {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_HP Security Interrupt Control Register
pub mod HPSICR {

    /// CAAM Security Violation Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the CAAM Security Violation security violation
    pub mod CAAM_EN {
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

            /// 0b0: CAAM Security Violation Interrupt is Disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: CAAM Security Violation Interrupt is Enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// JTAG Active Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the JTAG Active security violation
    pub mod JTAGC_EN {
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

            /// 0b0: JTAG Active Interrupt is Disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: JTAG Active Interrupt is Enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Watchdog 2 Reset Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Watchdog 2 Reset security violation
    pub mod WDOG2_EN {
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

            /// 0b0: Watchdog 2 Reset Interrupt is Disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Watchdog 2 Reset Interrupt is Enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Internal Boot Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Internal Boot security violation
    pub mod SRC_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Internal Boot Interrupt is Disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Internal Boot Interrupt is Enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// OCOTP attack error Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the OCOTP attack error security violation
    pub mod OCOTP_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OCOTP attack error Interrupt is Disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: OCOTP attack error Interrupt is Enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section
    pub mod LPSVI_EN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LP Security Violation Interrupt is Disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: LP Security Violation Interrupt is Enabled
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SNVS_HP Security Violation Control Register
pub mod HPSVCR {

    /// CAAM Security Violation Security Violation Configuration This field configures the CAAM Security Violation Security Violation Input
    pub mod CAAM_CFG {
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

            /// 0b0: CAAM Security Violation is a non-fatal violation
            pub const NON_FATAL: u32 = 0b0;

            /// 0b1: CAAM Security Violation is a fatal violation
            pub const FATAL: u32 = 0b1;
        }
    }

    /// JTAG Active Security Violation Configuration This field configures the JTAG Active Security Violation Input
    pub mod JTAGC_CFG {
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

            /// 0b0: JTAG Active is a non-fatal violation
            pub const NON_FATAL: u32 = 0b0;

            /// 0b1: JTAG Active is a fatal violation
            pub const FATAL: u32 = 0b1;
        }
    }

    /// Watchdog 2 Reset Security Violation Configuration This field configures the Watchdog 2 Reset Security Violation Input
    pub mod WDOG2_CFG {
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

            /// 0b0: Watchdog 2 Reset is a non-fatal violation
            pub const NON_FATAL: u32 = 0b0;

            /// 0b1: Watchdog 2 Reset is a fatal violation
            pub const FATAL: u32 = 0b1;
        }
    }

    /// Internal Boot Security Violation Configuration This field configures the Internal Boot Security Violation Input
    pub mod SRC_CFG {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Internal Boot is a non-fatal violation
            pub const NON_FATAL: u32 = 0b0;

            /// 0b1: Internal Boot is a fatal violation
            pub const FATAL: u32 = 0b1;
        }
    }

    /// OCOTP attack error Security Violation Configuration This field configures the OCOTP attack error Security Violation Input
    pub mod OCOTP_CFG {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: OCOTP attack error is disabled
            pub const DISABLED: u32 = 0b00;

            /// 0b01: OCOTP attack error is a non-fatal violation
            pub const NON_FATAL: u32 = 0b01;

            /// 0b00: OCOTP attack error is a fatal violation
            pub const FATAL: u32 = 0b00;
        }
    }

    /// LP Security Violation Configuration This field configures the LP security violation source.
    pub mod LPSV_CFG {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: LP security violation is disabled
            pub const DISABLED: u32 = 0b00;

            /// 0b01: LP security violation is a non-fatal violation
            pub const NON_FATAL: u32 = 0b01;

            /// 0b00: LP security violation is a fatal violation
            pub const FATAL: u32 = 0b00;
        }
    }
}

/// SNVS_HP Status Register
pub mod HPSR {

    /// HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared.
    pub mod HPTA {
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

            /// 0b0: No time alarm interrupt occurred.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: A time alarm interrupt occurred.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared.
    pub mod PI {
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

            /// 0b0: No periodic interrupt occurred.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: A periodic interrupt occurred.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Low Power Disable If 1, the low power section has been disabled by means of an input signal to SNVS
    pub mod LPDIS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Button Value of the BTN input
    pub mod BTN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Button Interrupt Signal ipi_snvs_btn_int_b was asserted.
    pub mod BI {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System Security Monitor State This field contains the encoded state of the SSM's state machine
    pub mod SSM_STATE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Init
            pub const INIT: u32 = 0b0000;

            /// 0b0001: Hard Fail
            pub const HARD_FAIL: u32 = 0b0001;

            /// 0b0011: Soft Fail
            pub const SOFT_FAIL: u32 = 0b0011;

            /// 0b1000: Init Intermediate (transition state between Init and Check - SSM stays in this state only one clock cycle)
            pub const INTERMEDIATE: u32 = 0b1000;

            /// 0b1001: Check
            pub const CHECK: u32 = 0b1001;

            /// 0b1011: Non-Secure
            pub const NON_SECURE: u32 = 0b1011;

            /// 0b1101: Trusted
            pub const TRUSTED: u32 = 0b1101;

            /// 0b1111: Secure
            pub const SECURE: u32 = 0b1111;
        }
    }

    /// System Security Configuration This field reflects the three security configuration inputs to SNVS
    pub mod SYS_SECURITY_CFG {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Fab Configuration - the default configuration of newly fabricated chips
            pub const FAB_CONFIG: u32 = 0b000;

            /// 0b001: Open Configuration - the configuration after NXP-programmable fuses have been blown
            pub const OPEN_CONFIG: u32 = 0b001;

            /// 0b011: Closed Configuration - the configuration after OEM-programmable fuses have been blown
            pub const CLOSED_CONFIG: u32 = 0b011;

            /// 0b111: Field Return Configuration - the configuration of chips that are returned to NXP for analysis
            pub const FIELD_RETURN_CONFIG: u32 = 0b111;
        }
    }

    /// System Secure Boot If SYS_SECURE_BOOT is 1, the chip boots from internal ROM
    pub mod SYS_SECURE_BOOT {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// One Time Programmable Master Key is Equal to Zero
    pub mod OTPMK_ZERO {
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

            /// 0b0: The OTPMK is not zero.
            pub const OTPMK_NOT_ZERO: u32 = 0b0;

            /// 0b1: The OTPMK is zero.
            pub const OTPMK_IS_ZERO: u32 = 0b1;
        }
    }

    /// Zeroizable Master Key is Equal to Zero
    pub mod ZMK_ZERO {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The ZMK is not zero.
            pub const ZMK_NOT_ZERO: u32 = 0b0;

            /// 0b1: The ZMK is zero.
            pub const ZMK_IS_ZERO: u32 = 0b1;
        }
    }
}

/// SNVS_HP Security Violation Status Register
pub mod HPSVSR {

    /// CAAM Security Violation security violation was detected.
    pub mod CAAM {
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

            /// 0b0: No CAAM Security Violation security violation was detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: CAAM Security Violation security violation was detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// JTAG Active security violation was detected.
    pub mod JTAGC {
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

            /// 0b0: No JTAG Active security violation was detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: JTAG Active security violation was detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Watchdog 2 Reset security violation was detected.
    pub mod WDOG2 {
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

            /// 0b0: No Watchdog 2 Reset security violation was detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: Watchdog 2 Reset security violation was detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Internal Boot security violation was detected.
    pub mod SRC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Internal Boot security violation was detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: Internal Boot security violation was detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// OCOTP attack error security violation was detected.
    pub mod OCOTP {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No OCOTP attack error security violation was detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: OCOTP attack error security violation was detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Software Security Violation This bit is a read-only copy of the SW_SV bit in the HP Command Register
    pub mod SW_SV {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software Fatal Security Violation This bit is a read-only copy of the SW_FSV bit in the HP Command Register
    pub mod SW_FSV {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LP Software Security Violation This bit is a read-only copy of the SW_LPSV bit in the HP Command Register
    pub mod SW_LPSV {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Zeroizable Master Key Syndrome The ZMK syndrome indicates the single-bit error location and parity for the ZMK register
    pub mod ZMK_SYNDROME {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (9 bits: 0x1ff << 16)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data
    pub mod ZMK_ECC_FAIL {
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

            /// 0b0: ZMK ECC Failure was not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: ZMK ECC Failure was detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// LP Security Violation A security volation was detected in the SNVS low power section
    pub mod LP_SEC_VIO {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_HP High Assurance Counter IV Register
pub mod HPHACIVR {

    /// High Assurance Counter Initial Value This register is used to set the starting count value to the high assurance counter
    pub mod HAC_COUNTER_IV {
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

/// SNVS_HP High Assurance Counter Register
pub mod HPHACR {

    /// High Assurance Counter When the HAC_EN bit is set and the SSM is in the soft fail state, this counter starts to count down with the system clock
    pub mod HAC_COUNTER {
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

/// SNVS_HP Real Time Counter MSB Register
pub mod HPRTCMR {

    /// HP Real Time Counter The most-significant 15 bits of the RTC
    pub mod RTC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (15 bits: 0x7fff << 0)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_HP Real Time Counter LSB Register
pub mod HPRTCLR {

    /// HP Real Time Counter least-significant 32 bits
    pub mod RTC {
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

/// SNVS_HP Time Alarm MSB Register
pub mod HPTAMR {

    /// HP Time Alarm, most-significant 15 bits
    pub mod HPTA_MS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (15 bits: 0x7fff << 0)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_HP Time Alarm LSB Register
pub mod HPTALR {

    /// HP Time Alarm, 32 least-significant bits
    pub mod HPTA_LS {
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

/// SNVS_LP Lock Register
pub mod LPLR {

    /// Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR
    pub mod ZMK_WHL {
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

            /// 0b0: Write access is allowed.
            pub const WRITE_ACCESS_ALLOWED: u32 = 0b0;

            /// 0b1: Write access is not allowed.
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0b1;
        }
    }

    /// Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR
    pub mod ZMK_RHL {
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

            /// 0b0: Read access is allowed (only in software programming mode).
            pub const READ_ACCESS_ALLOWED: u32 = 0b0;

            /// 0b1: Read access is not allowed.
            pub const READ_ACCESS_NOT_ALLOWED: u32 = 0b1;
        }
    }

    /// Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits
    pub mod SRTC_HL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WHL::RW;
    }

    /// LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)
    pub mod LPCALB_HL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WHL::RW;
    }

    /// Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit
    pub mod MC_HL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write access (increment) is allowed.
            pub const WRITE_ACCESS_ALLOWED: u32 = 0b0;

            /// 0b1: Write access (increment) is not allowed.
            pub const WRITE_ACCESS_NOT_ALLOWED: u32 = 0b1;
        }
    }

    /// General Purpose Register Hard Lock When set, prevents any writes to the GPR
    pub mod GPR_HL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WHL::RW;
    }

    /// LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR
    pub mod LPSVCR_HL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WHL::RW;
    }

    /// LP Tamper Glitch Filter Configuration Register Hard Lock When set, prevents any writes to the LPTGFCR
    pub mod LPTGFCR_HL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WHL::RW;
    }

    /// LP Security Events Configuration Register Hard Lock When set, prevents any writes to the LPSECR
    pub mod LPSECR_HL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WHL::RW;
    }

    /// Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register
    pub mod MKS_HL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WHL::RW;
    }

    /// Active Tamper 1 Hard Lock When set, prevents any writes to the Active Tamper 1 registers
    pub mod AT1_HL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WHL::RW;
    }

    /// Active Tamper 2 Hard Lock When set, prevents any writes to the Active Tamper 2 registers
    pub mod AT2_HL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WHL::RW;
    }

    /// Active Tamper 3 Hard Lock When set, prevents any writes to the Active Tamper 3 registers
    pub mod AT3_HL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WHL::RW;
    }

    /// Active Tamper 4 Hard Lock When set, prevents any writes to the Active Tamper 4 registers
    pub mod AT4_HL {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WHL::RW;
    }

    /// Active Tamper 5 Hard Lock When set, prevents any writes to the Active Tamper 5 registers
    pub mod AT5_HL {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ZMK_WHL::RW;
    }
}

/// SNVS_LP Control Register
pub mod LPCR {

    /// Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational
    pub mod SRTC_ENV {
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

            /// 0b0: SRTC is disabled or invalid.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: SRTC is enabled and valid.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter
    pub mod LPTA_EN {
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

            /// 0b0: LP time alarm interrupt is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: LP time alarm interrupt is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)
    pub mod MC_ENV {
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

            /// 0b0: MC is disabled or invalid.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: MC is enabled and valid.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (tamper event, MC rollover, SRTC rollover, or time alarm )
    pub mod LPWUI_EN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)
    pub mod SRTC_INV_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SRTC stays valid in the case of security violation (other than a software violation (HPSVSR\[SW_LPSV\] = 1 or HPCOMR\[SW_LPSV\] = 1)).
            pub const KEEP_VALID: u32 = 0b0;

            /// 0b1: SRTC is invalidated in the case of security violation.
            pub const INVALIDATE: u32 = 0b1;
        }
    }

    /// Dumb PMIC Enabled When set, software can control the system power
    pub mod DP_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Smart PMIC enabled.
            pub const SMART_PMIC_ENABLED: u32 = 0b0;

            /// 0b1: Dumb PMIC enabled.
            pub const DUMB_PMIC_ENABLED: u32 = 0b1;
        }
    }

    /// Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power
    pub mod TOP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Leave system power on.
            pub const KEEP_ON: u32 = 0b0;

            /// 0b1: Turn off system power.
            pub const TURN_OFF: u32 = 0b1;
        }
    }

    /// Digital Low-Voltage Event Enable By default the detection of a low-voltage event does not cause the pmic_en_b signal to be asserted
    pub mod LVD_EN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LP Calibration Enable When set, enables the SRTC calibration mechanism
    pub mod LPCALB_EN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SRTC Time calibration is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: SRTC Time calibration is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// LP Calibration Value Defines signed calibration value for SRTC
    pub mod LPCALB_VAL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: +0 counts per each 32768 ticks of the counter clock
            pub const ADD_0_PER_32768_TICKS: u32 = 0b00000;

            /// 0b00001: +1 counts per each 32768 ticks of the counter clock
            pub const ADD_1_PER_32768_TICKS: u32 = 0b00001;

            /// 0b00010: +2 counts per each 32768 ticks of the counter clock
            pub const ADD_2_PER_32768_TICKS: u32 = 0b00010;

            /// 0b01111: +15 counts per each 32768 ticks of the counter clock
            pub const ADD_15_PER_32768_TICKS: u32 = 0b01111;

            /// 0b10000: -16 counts per each 32768 ticks of the counter clock
            pub const SUB_16_PER_32768_TICKS: u32 = 0b10000;

            /// 0b10001: -15 counts per each 32768 ticks of the counter clock
            pub const SUB_15_PER_32768_TICKS: u32 = 0b10001;

            /// 0b11110: -2 counts per each 32768 ticks of the counter clock
            pub const SUB_2_PER_32768_TICKS: u32 = 0b11110;

            /// 0b11111: -1 counts per each 32768 ticks of the counter clock
            pub const SUB_1_PER_32768_TICKS: u32 = 0b11111;
        }
    }

    /// This field configures the button press time out values for the PMIC Logic
    pub mod BTN_PRESS_TIME {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field configures the amount of debounce time for the BTN input signal
    pub mod DEBOUNCE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power
    pub mod ON_TIME {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en
    pub mod PK_EN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override
    pub mod PK_OVERRIDE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// General Purpose Registers Zeroization Disable
    pub mod GPR_Z_DIS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_LP Master Key Control Register
pub mod LPMKCR {

    /// Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR
    pub mod MASTER_KEY_SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Select one time programmable master key.
            pub const SELECT_OTPMK: u32 = 0b00;

            /// 0b10: Select zeroizable master key when MKS_EN bit is set .
            pub const SELECT_ZMK: u32 = 0b10;

            /// 0b11: Select combined master key when MKS_EN bit is set .
            pub const SELECT_COMBO: u32 = 0b11;
        }
    }

    /// Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it
    pub mod ZMK_HWP {
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

            /// 0b0: ZMK is in the software programming mode.
            pub const SW_PROG_MODE: u32 = 0b0;

            /// 0b1: ZMK is in the hardware programming mode.
            pub const HW_PROG_MODE: u32 = 0b1;
        }
    }

    /// Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules
    pub mod ZMK_VAL {
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

            /// 0b0: ZMK is not valid.
            pub const INVALID: u32 = 0b0;

            /// 0b1: ZMK is valid.
            pub const VALID: u32 = 0b1;
        }
    }

    /// Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register
    pub mod ZMK_ECC_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ZMK ECC check is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: ZMK ECC check is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Zeroizable Master Key Error Correcting Code Value This field is automatically calculated and set when one is written into ZMK_ECC_EN bit of this register
    pub mod ZMK_ECC_VALUE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (9 bits: 0x1ff << 7)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_LP Security Violation Control Register
pub mod LPSVCR {

    /// CAAM Security Violation Enable This bit enables CAAM Security Violation Input
    pub mod CAAM_EN {
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

            /// 0b0: CAAM Security Violation is disabled in the LP domain.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: CAAM Security Violation is enabled in the LP domain.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// JTAG Active Enable This bit enables JTAG Active Input
    pub mod JTAGC_EN {
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

            /// 0b0: JTAG Active is disabled in the LP domain.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: JTAG Active is enabled in the LP domain.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Watchdog 2 Reset Enable This bit enables Watchdog 2 Reset Input
    pub mod WDOG2_EN {
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

            /// 0b0: Watchdog 2 Reset is disabled in the LP domain.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Watchdog 2 Reset is enabled in the LP domain.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Internal Boot Enable This bit enables Internal Boot Input
    pub mod SRC_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Internal Boot is disabled in the LP domain.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Internal Boot is enabled in the LP domain.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// OCOTP attack error Enable This bit enables OCOTP attack error Input
    pub mod OCOTP_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OCOTP attack error is disabled in the LP domain.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: OCOTP attack error is enabled in the LP domain.
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SNVS_LP Tamper Glitch Filters Configuration Register
pub mod LPTGFCR {

    /// Wire-Mesh Tamper Glitch Filter Configures the length of the digital glitch filter for the wire-mesh tamper 1 and 2 pins between 1 and 63 SRTC clock cycles
    pub mod WMTGF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wire-Mesh Tamper Glitch Filter Enable When set, enables the wire-mesh tamper glitch filter
    pub mod WMTGF_EN {
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

            /// 0b0: Wire-mesh tamper glitch filter is bypassed.
            pub const BYPASSED: u32 = 0b0;

            /// 0b1: Wire-mesh tamper glitch filter is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tamper Glitch Filter 1 Configures the length of the digital glitch filter for the external tamper 1 pin between 128 and 32640 SRTC clock cycles
    pub mod ETGF1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper Glitch Filter 1 Enable When set, enables the external tamper glitch filter 1.
    pub mod ETGF1_EN {
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

            /// 0b0: External tamper glitch filter 1 is bypassed.
            pub const BYPASSED: u32 = 0b0;

            /// 0b1: External tamper glitch filter 1 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tamper Glitch Filter 2 Configures the length of the digital glitch filter for the external tamper 2 pin between 128 and 32640 SRTC clock cycles
    pub mod ETGF2 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper Glitch Filter 2 Enable When set, enables the external tamper glitch filter 2.
    pub mod ETGF2_EN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper glitch filter 2 is bypassed.
            pub const BYPASSED: u32 = 0b0;

            /// 0b1: External tamper glitch filter 2 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SNVS_LP Tamper Detect Configuration Register
pub mod LPTDCR {

    /// SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation.
    pub mod SRTCR_EN {
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

            /// 0b0: SRTC rollover is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: SRTC rollover is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// MC Rollover Enable When set, an MC Rollover event generates an LP security violation.
    pub mod MCR_EN {
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

            /// 0b0: MC rollover is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: MC rollover is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Clock Tamper Enable When set, a clock monitor tamper generates an LP security violation.
    pub mod CT_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clock tamper is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Clock tamper is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Temperature Tamper Enable When set, a temperature monitor tamper generates an LP security violation
    pub mod TT_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Temperature tamper is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Temperature tamper is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Voltage Tamper Enable Voltage Tamper Enable should be enabled 500 us after setting SCSC_SOSC_CTR \[VOLT_TEMP_TAMPER_EN\]
    pub mod VT_EN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Voltage tamper is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Voltage tamper is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Wire-Mesh Tampering 1 Enable When set, wire-mesh tampering 1 detection generates an LP security violation
    pub mod WMT1_EN {
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

            /// 0b0: Wire-mesh tamper 1 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Wire-mesh tamper 1 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Wire-Mesh Tampering 2 Enable When set, wire-mesh tampering 2 detection generates an LP security violation
    pub mod WMT2_EN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Wire-mesh tamper 2 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Wire-mesh tamper 2 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tampering 1 Enable When set, external tampering 1 detection generates an LP security violation
    pub mod ET1_EN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper 1 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: External tamper 1 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tampering 2 Enable When set, external tampering 2 detection generates an LP security violation
    pub mod ET2_EN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper 2 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: External tamper 2 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tampering 1 Polarity This bit is used to determine the polarity of external tamper 1.
    pub mod ET1P {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper 1 is active low.
            pub const ACTIVE_LOW: u32 = 0b0;

            /// 0b1: External tamper 1 is active high.
            pub const ACTIVE_HIGH: u32 = 0b1;
        }
    }

    /// External Tampering 2 Polarity This bit is used to determine the polarity of external tamper 2.
    pub mod ET2P {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper 2 is active low.
            pub const ACTIVE_LOW: u32 = 0b0;

            /// 0b1: External tamper 2 is active high.
            pub const ACTIVE_HIGH: u32 = 0b1;
        }
    }

    /// System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)
    pub mod PFD_OBSERV {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS
    pub mod POR_OBSERV {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low Temp Detect Configuration These configuration bits are wired as an output of the module.
    pub mod LTDC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// High Temperature Detect Configuration These configuration bits are wired as an output of the module
    pub mod HTDC {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Voltage Reference Configuration These configuration bits are wired as an output of the module.
    pub mod VRC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted
    pub mod OSCB {
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

            /// 0b0: Normal SRTC clock oscillator not bypassed.
            pub const NOT_BYPASSED: u32 = 0b0;

            /// 0b1: Normal SRTC clock oscillator bypassed. Alternate clock can drive the SRTC clock source.
            pub const BYPASSED: u32 = 0b1;
        }
    }
}

/// SNVS_LP Status Register
pub mod LPSR {

    /// LP Time Alarm
    pub mod LPTA {
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

            /// 0b0: No time alarm interrupt occurred.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: A time alarm interrupt occurred.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Secure Real Time Counter Rollover
    pub mod SRTCR {
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

            /// 0b0: SRTC has not reached its maximum value.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: SRTC has reached its maximum value.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Monotonic Counter Rollover
    pub mod MCR {
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

            /// 0b0: MC has not reached its maximum value.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: MC has reached its maximum value.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Digital Low Voltage Event Detected
    pub mod LVD {
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

            /// 0b0: No low voltage event detected.
            pub const NOLOWVOLT: u32 = 0b0;

            /// 0b1: Low voltage event is detected.
            pub const LOWVOLTDETECTED: u32 = 0b1;
        }
    }

    /// Clock Tampering Detected
    pub mod CTD {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No clock tamper.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: Clock tamper is detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Temperature Tamper Detected
    pub mod TTD {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No temperature tamper.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: Temperature tamper is detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Voltage Tampering Detected
    pub mod VTD {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Voltage tampering not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: Voltage tampering detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Wire-Mesh Tampering 1 Detected
    pub mod WMT1D {
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

            /// 0b0: Wire-mesh tampering 1 not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: Wire-mesh tampering 1 detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Wire-Mesh Tampering 2 Detected
    pub mod WMT2D {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Wire-mesh tampering 2 not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: Wire-mesh tampering 2 detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// External Tampering 1 Detected
    pub mod ET1D {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tampering 1 not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: External tampering 1 detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// External Tampering 2 Detected
    pub mod ET2D {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tampering 2 not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: External tampering 2 detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports
    pub mod ESVD {
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

            /// 0b0: No external security violation.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: External security violation is detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Emergency Off This bit is set when a power off is requested.
    pub mod EO {
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

            /// 0b0: Emergency off was not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: Emergency off was detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time
    pub mod SPOF {
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

            /// 0b0: Set Power Off was not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: Set Power Off was detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state
    pub mod LPNS {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LP section was not programmed in the non-secure state.
            pub const NOT_PRGRMD_IN_NON_SECURE_STATE: u32 = 0b0;

            /// 0b1: LP section was programmed in the non-secure state.
            pub const WAS_PRGRMD_IN_NON_SECURE_STATE: u32 = 0b1;
        }
    }

    /// LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state
    pub mod LPS {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LP section was not programmed in secure or trusted state.
            pub const NOT_PRGRMD_IN_SECURE_OR_TRUSTED_STATE: u32 = 0b0;

            /// 0b1: LP section was programmed in secure or trusted state.
            pub const WAS_PRGRMD_IN_SECURE_OR_TRUSTED_STATE: u32 = 0b1;
        }
    }
}

/// SNVS_LP Secure Real Time Counter MSB Register
pub mod LPSRTCMR {

    /// LP Secure Real Time Counter The most-significant 15 bits of the SRTC
    pub mod SRTC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (15 bits: 0x7fff << 0)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_LP Secure Real Time Counter LSB Register
pub mod LPSRTCLR {

    /// LP Secure Real Time Counter least-significant 32 bits This register can be programmed only when SRTC is not active and not locked, meaning the SRTC_ENV, SRTC_SL, and SRTC_HL bits are not set
    pub mod SRTC {
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

/// SNVS_LP Time Alarm Register
pub mod LPTAR {

    /// LP Time Alarm This register can be programmed only when the LP time alarm is disabled (LPTA_EN bit is not set)
    pub mod LPTA {
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

/// SNVS_LP Secure Monotonic Counter MSB Register
pub mod LPSMCMR {

    /// Monotonic Counter most-significant 16 Bits Note that writing to this register does not change the value of this field to the value that was written
    pub mod MON_COUNTER {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Monotonic Counter Era Bits These bits are inputs to the module and typically connect to fuses
    pub mod MC_ERA_BITS {
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

/// SNVS_LP Secure Monotonic Counter LSB Register
pub mod LPSMCLR {

    /// Monotonic Counter bits Note that writing to this register does not change the value of this field to the value that was written
    pub mod MON_COUNTER {
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

/// SNVS_LP Digital Low-Voltage Detector Register
pub mod LPLVDR {

    /// Low-Voltage Detector Value
    pub mod LVD {
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

/// SNVS_LP General Purpose Register 0 (legacy alias)
pub mod LPGPR0_legacy_alias {

    /// General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed.
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

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR0 {

    /// Zeroizable Master Key Each of these registers contains 32 bits of the 256-bit ZMK value
    pub mod ZMK {
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

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR1 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR2 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR3 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR4 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR5 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR6 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR7 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR_alias0 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR_alias1 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR_alias2 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR_alias3 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP Tamper Detectors Config 2 Register
pub mod LPTDC2R {

    /// External Tampering 3 Enable When set, external tampering 3 detection generates an LP security violation
    pub mod ET3_EN {
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

            /// 0b0: External tamper 3 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: External tamper 3 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tampering 4 Enable When set, external tampering 4 detection generates an LP security violation
    pub mod ET4_EN {
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

            /// 0b0: External tamper 4 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: External tamper 4 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tampering 5 Enable When set, external tampering 5 detection generates an LP security violation
    pub mod ET5_EN {
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

            /// 0b0: External tamper 5 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: External tamper 5 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tampering 6 Enable When set, external tampering 6 detection generates an LP security violation
    pub mod ET6_EN {
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

            /// 0b0: External tamper 6 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: External tamper 6 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tampering 7 Enable When set, external tampering 7 detection generates an LP security violation
    pub mod ET7_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper 7 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: External tamper 7 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tampering 8 Enable When set, external tampering 8 detection generates an LP security violation
    pub mod ET8_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper 8 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: External tamper 8 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tampering 9 Enable When set, external tampering 9 detection generates an LP security violation
    pub mod ET9_EN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper 9 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: External tamper 9 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tampering 10 Enable When set, external tampering 10 detection generates an LP security violation
    pub mod ET10_EN {
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

            /// 0b0: External tamper 10 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: External tamper 10 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tampering 3 Polarity This bit is used to determine the polarity of external tamper 3.
    pub mod ET3P {
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

            /// 0b0: External tamper 3 active low.
            pub const ACTIVE_LOW: u32 = 0b0;

            /// 0b1: External tamper 3 active high.
            pub const ACTIVE_HIGH: u32 = 0b1;
        }
    }

    /// External Tampering 4 Polarity This bit is used to determine the polarity of external tamper 4.
    pub mod ET4P {
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

            /// 0b0: External tamper 4 is active low.
            pub const ACTIVE_LOW: u32 = 0b0;

            /// 0b1: External tamper 4 is active high.
            pub const ACTIVE_HIGH: u32 = 0b1;
        }
    }

    /// External Tampering 5 Polarity This bit is used to determine the polarity of external tamper 5.
    pub mod ET5P {
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

            /// 0b0: External tamper 5 is active low.
            pub const ACTIVE_LOW: u32 = 0b0;

            /// 0b1: External tamper 5 is active high.
            pub const ACTIVE_HIGH: u32 = 0b1;
        }
    }

    /// External Tampering 6 Polarity This bit is used to determine the polarity of external tamper 6.
    pub mod ET6P {
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

            /// 0b0: External tamper 6 is active low.
            pub const ACTIVE_LOW: u32 = 0b0;

            /// 0b1: External tamper 6 is active high.
            pub const ACTIVE_HIGH: u32 = 0b1;
        }
    }

    /// External Tampering 7 Polarity This bit is used to determine the polarity of external tamper 7.
    pub mod ET7P {
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

            /// 0b0: External tamper 7 is active low.
            pub const ACTIVE_LOW: u32 = 0b0;

            /// 0b1: External tamper 7 is active high.
            pub const ACTIVE_HIGH: u32 = 0b1;
        }
    }

    /// External Tampering 8 Polarity This bit is used to determine the polarity of external tamper 8.
    pub mod ET8P {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper 8 is active low.
            pub const ACTIVE_LOW: u32 = 0b0;

            /// 0b1: External tamper 8 is active high.
            pub const ACTIVE_HIGH: u32 = 0b1;
        }
    }

    /// External Tampering 9 Polarity This bit is used to determine the polarity of external tamper 9.
    pub mod ET9P {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper 9 is active low.
            pub const ACTIVE_LOW: u32 = 0b0;

            /// 0b1: External tamper 9 is active high.
            pub const ACTIVE_HIGH: u32 = 0b1;
        }
    }

    /// External Tampering 10 Polarity This bit is used to determine the polarity of external tamper 10.
    pub mod ET10P {
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

            /// 0b0: External tamper 10 is active low.
            pub const ACTIVE_LOW: u32 = 0b0;

            /// 0b1: External tamper 10 is active high.
            pub const ACTIVE_HIGH: u32 = 0b1;
        }
    }
}

/// SNVS_LP Tamper Detectors Status Register
pub mod LPTDSR {

    /// External Tampering 3 Detected
    pub mod ET3D {
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

            /// 0b0: External tamper 3 is not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: External tamper 3 is detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// External Tampering 4 Detected
    pub mod ET4D {
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

            /// 0b0: External tamper 4 is not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: External tamper 4 is detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// External Tampering 5 Detected
    pub mod ET5D {
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

            /// 0b0: External tamper 5 is not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: External tamper 5 is detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// External Tampering 6 Detected
    pub mod ET6D {
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

            /// 0b0: External tamper 6 is not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: External tamper 6 is detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// External Tampering 7 Detected
    pub mod ET7D {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper 7 is not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: External tamper 7 is detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// External Tampering 8 Detected
    pub mod ET8D {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper 8 is not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: External tamper 8 is detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// External Tampering 9 Enable When set, external tampering 9 detection generates an LP security violation
    pub mod ET9D {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper 9 is not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: External tamper 9 is detected.
            pub const REPORTED: u32 = 0b1;
        }
    }

    /// External Tampering 10 Detected
    pub mod ET10D {
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

            /// 0b0: External tamper 10 is not detected.
            pub const NOREPORT: u32 = 0b0;

            /// 0b1: External tamper 10 is detected.
            pub const REPORTED: u32 = 0b1;
        }
    }
}

/// SNVS_LP Tamper Glitch Filter 1 Configuration Register
pub mod LPTGF1CR {

    /// External Tamper Glitch Filter 3 Configures the length of the digital glitch filter for the external tamper 3 pin between 128 and 32640 SRTC clock cycles
    pub mod ETGF3 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper Glitch Filter 3 Enable When set, enables the external tamper glitch filter 3.
    pub mod ETGF3_EN {
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

            /// 0b0: External tamper glitch filter 3 is bypassed.
            pub const BYPASSED: u32 = 0b0;

            /// 0b1: External tamper glitch filter 3 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tamper Glitch Filter 4 Configures the length of the digital glitch filter for the external tamper 4 pin between 128 and 32640 SRTC clock cycles
    pub mod ETGF4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper Glitch Filter 4 Enable When set, enables the external tamper glitch filter 4.
    pub mod ETGF4_EN {
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

            /// 0b0: External tamper glitch filter 4 is bypassed.
            pub const BYPASSED: u32 = 0b0;

            /// 0b1: External tamper glitch filter 4 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tamper Glitch Filter 5 Configures the length of the digital glitch filter for the external tamper 5 pin between 128 and 32640 SRTC clock cycles
    pub mod ETGF5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper Glitch Filter 5 Enable When set, enables the external tamper glitch filter 5.
    pub mod ETGF5_EN {
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

            /// 0b0: External tamper glitch filter 5 is bypassed.
            pub const BYPASSED: u32 = 0b0;

            /// 0b1: External tamper glitch filter 5 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tamper Glitch Filter 6 Configures the length of the digital glitch filter for the external tamper 6 pin between 128 and 32640 SRTC clock cycles
    pub mod ETGF6 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper Glitch Filter 6 Enable When set, enables the external tamper glitch filter 6.
    pub mod ETGF6_EN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper glitch filter 6 is bypassed.
            pub const BYPASSED: u32 = 0b0;

            /// 0b1: External tamper glitch filter 6 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SNVS_LP Tamper Glitch Filter 2 Configuration Register
pub mod LPTGF2CR {

    /// External Tamper Glitch Filter 7 Configures the length of the digital glitch filter for the external tamper 7 pin between 128 and 32640 SRTC clock cycles
    pub mod ETGF7 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper Glitch Filter 7 Enable When set, enables the external tamper glitch filter 7.
    pub mod ETGF7_EN {
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

            /// 0b0: External tamper glitch filter 7 is bypassed.
            pub const BYPASSED: u32 = 0b0;

            /// 0b1: External tamper glitch filter 7 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tamper Glitch Filter 8 Configures the length of the digital glitch filter for the external tamper 8 pin between 128 and 32640 SRTC clock cycles
    pub mod ETGF8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper Glitch Filter 8 Enable When set, enables the external tamper glitch filter 8.
    pub mod ETGF8_EN {
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

            /// 0b0: External tamper glitch filter 8 is bypassed.
            pub const BYPASSED: u32 = 0b0;

            /// 0b1: External tamper glitch filter 8 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tamper Glitch Filter 9 Configures the length of the digital glitch filter for the external tamper 9 pin between 128 and 32640 SRTC clock cycles
    pub mod ETGF9 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper Glitch Filter 9 Enable When set, enables the external tamper glitch filter 9.
    pub mod ETGF9_EN {
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

            /// 0b0: External tamper glitch filter 9 is bypassed.
            pub const BYPASSED: u32 = 0b0;

            /// 0b1: External tamper glitch filter 9 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// External Tamper Glitch Filter 10 Configures the length of the digital glitch filter for the external tamper 10 pin between 128 and 32640 SRTC clock cycles
    pub mod ETGF10 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper Glitch Filter 10 Enable When set, enables the external tamper glitch filter 10.
    pub mod ETGF10_EN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External tamper glitch filter 10 is bypassed.
            pub const BYPASSED: u32 = 0b0;

            /// 0b1: External tamper glitch filter 10 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SNVS_LP Active Tamper 1 Configuration Register
pub mod LPAT1CR {

    /// Active Tamper 1 Initial Seed Default Seed is 1111h.
    pub mod Seed {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Active Tamper 1 Polynomial Default Polynomial is 8400h.
    pub mod Polynomial {
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

/// SNVS_LP Active Tamper 2 Configuration Register
pub mod LPAT2CR {
    pub use super::LPAT1CR::Polynomial;
    pub use super::LPAT1CR::Seed;
}

/// SNVS_LP Active Tamper 3 Configuration Register
pub mod LPAT3CR {
    pub use super::LPAT1CR::Polynomial;
    pub use super::LPAT1CR::Seed;
}

/// SNVS_LP Active Tamper 4 Configuration Register
pub mod LPAT4CR {
    pub use super::LPAT1CR::Polynomial;
    pub use super::LPAT1CR::Seed;
}

/// SNVS_LP Active Tamper 5 Configuration Register
pub mod LPAT5CR {
    pub use super::LPAT1CR::Polynomial;
    pub use super::LPAT1CR::Seed;
}

/// SNVS_LP Active Tamper Control Register
pub mod LPATCTLR {

    /// Active Tamper 1 Enable When set, enables the Active Tamper 1 LFSR.
    pub mod AT1_EN {
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

            /// 0b0: Active Tamper 1 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Active Tamper 1 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Active Tamper 2 Enable When set, enables the Active Tamper 2 LFSR.
    pub mod AT2_EN {
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

            /// 0b0: Active Tamper 2 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Active Tamper 2 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Active Tamper 3 Enable When set, enables the Active Tamper 3 LFSR.
    pub mod AT3_EN {
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

            /// 0b0: Active Tamper 3 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Active Tamper 3 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Active Tamper 4 Enable When set, enables the Active Tamper 4 LFSR.
    pub mod AT4_EN {
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

            /// 0b0: Active Tamper 4 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Active Tamper 4 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Active Tamper 5 Enable When set, enables the Active Tamper 5 LFSR.
    pub mod AT5_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Active Tamper 5 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Active Tamper 5 is enabled.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Active Tamper 1 Pad Out Enable When set, enables the Active Tamper 1 external pad.
    pub mod AT1_PAD_EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AT1_EN::RW;
    }

    /// Active Tamper 2 Pad Out Enable When set, enables the Active Tamper 2 external pad.
    pub mod AT2_PAD_EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AT2_EN::RW;
    }

    /// Active Tamper 3 Pad Out Enable When set, enables the Active Tamper 3 external pad.
    pub mod AT3_PAD_EN {
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

            /// 0b0: Active Tamper 3 is disabled.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Active Tamper 3 is enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Active Tamper 4 Pad Out Enable When set, enables the Active Tamper 4 external pad.
    pub mod AT4_PAD_EN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AT4_EN::RW;
    }

    /// Active Tamper 5 Pad Out Enable When set, enables the Active Tamper 5 external pad.
    pub mod AT5_PAD_EN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AT5_EN::RW;
    }
}

/// SNVS_LP Active Tamper Clock Control Register
pub mod LPATCLKR {

    /// Active Tamper 1 Clock Control 00: 16hz 01: 8hz 10: 4hz 11: 2hz
    pub mod AT1_CLK_CTL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Active Tamper 2 Clock Control 00: 16hz 01: 8hz 10: 4hz 11: 2hz
    pub mod AT2_CLK_CTL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Active Tamper 3 Clock Control 00: 16hz 01: 8hz 10: 4hz 11: 2hz
    pub mod AT3_CLK_CTL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Active Tamper 4 Clock Control 00: 16hz 01: 8hz 10: 4hz 11: 2hz
    pub mod AT4_CLK_CTL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Active Tamper 5 Clock Control 00: 16hz 01: 8hz 10: 4hz 11: 2hz
    pub mod AT5_CLK_CTL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_LP Active Tamper Routing Control 1 Register
pub mod LPATRC1R {

    /// External Tamper 1 Routing Control Any undefined selection will be routed to passive
    pub mod ET1RCTL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper 2 Routing Control Any undefined selection will be routed to passive
    pub mod ET2RCTL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper 3 Routing Control Any undefined selection will be routed to passive
    pub mod ET3RCTL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper 4 Routing Control Any undefined selection will be routed to passive
    pub mod ET4RCTL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper 5 Routing Control Any undefined selection will be routed to passive
    pub mod ET5RCTL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper 6 Routing Control Any undefined selection will be routed to passive
    pub mod ET6RCTL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper 7 Routing Control Any undefined selection will be routed to passive
    pub mod ET7RCTL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper 8 Routing Control Any undefined selection will be routed to passive
    pub mod ET8RCTL {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (3 bits: 0b111 << 28)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_LP Active Tamper Routing Control 2 Register
pub mod LPATRC2R {

    /// External Tamper 9 Routing Control Any undefined selection will be routed to passive
    pub mod ET9RCTL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External Tamper 10 Routing Control Any undefined selection will be routed to passive
    pub mod ET10RCTL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR0 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR1 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR2 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR3 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_HP Version ID Register 1
pub mod HPVIDR1 {

    /// SNVS block minor version number
    pub mod MINOR_REV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNVS block major version number
    pub mod MAJOR_REV {
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

    /// SNVS block ID
    pub mod IP_ID {
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

/// SNVS_HP Version ID Register 2
pub mod HPVIDR2 {

    /// SNVS ECO Revision The engineering change order revision number for this release of SNVS.
    pub mod ECO_REV {
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

    /// IP Era 00h - Era 1 or 2 03h - Era 3 04h - Era 4 05h - Era 5 06h - Era 6
    pub mod IP_ERA {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
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
    /// SNVS_HP Lock Register
    pub HPLR: RWRegister<u32>,

    /// SNVS_HP Command Register
    pub HPCOMR: RWRegister<u32>,

    /// SNVS_HP Control Register
    pub HPCR: RWRegister<u32>,

    /// SNVS_HP Security Interrupt Control Register
    pub HPSICR: RWRegister<u32>,

    /// SNVS_HP Security Violation Control Register
    pub HPSVCR: RWRegister<u32>,

    /// SNVS_HP Status Register
    pub HPSR: RWRegister<u32>,

    /// SNVS_HP Security Violation Status Register
    pub HPSVSR: RWRegister<u32>,

    /// SNVS_HP High Assurance Counter IV Register
    pub HPHACIVR: RWRegister<u32>,

    /// SNVS_HP High Assurance Counter Register
    pub HPHACR: RORegister<u32>,

    /// SNVS_HP Real Time Counter MSB Register
    pub HPRTCMR: RWRegister<u32>,

    /// SNVS_HP Real Time Counter LSB Register
    pub HPRTCLR: RWRegister<u32>,

    /// SNVS_HP Time Alarm MSB Register
    pub HPTAMR: RWRegister<u32>,

    /// SNVS_HP Time Alarm LSB Register
    pub HPTALR: RWRegister<u32>,

    /// SNVS_LP Lock Register
    pub LPLR: RWRegister<u32>,

    /// SNVS_LP Control Register
    pub LPCR: RWRegister<u32>,

    /// SNVS_LP Master Key Control Register
    pub LPMKCR: RWRegister<u32>,

    /// SNVS_LP Security Violation Control Register
    pub LPSVCR: RWRegister<u32>,

    /// SNVS_LP Tamper Glitch Filters Configuration Register
    pub LPTGFCR: RWRegister<u32>,

    /// SNVS_LP Tamper Detect Configuration Register
    pub LPTDCR: RWRegister<u32>,

    /// SNVS_LP Status Register
    pub LPSR: RWRegister<u32>,

    /// SNVS_LP Secure Real Time Counter MSB Register
    pub LPSRTCMR: RWRegister<u32>,

    /// SNVS_LP Secure Real Time Counter LSB Register
    pub LPSRTCLR: RWRegister<u32>,

    /// SNVS_LP Time Alarm Register
    pub LPTAR: RWRegister<u32>,

    /// SNVS_LP Secure Monotonic Counter MSB Register
    pub LPSMCMR: RWRegister<u32>,

    /// SNVS_LP Secure Monotonic Counter LSB Register
    pub LPSMCLR: RWRegister<u32>,

    /// SNVS_LP Digital Low-Voltage Detector Register
    pub LPLVDR: RWRegister<u32>,

    /// SNVS_LP General Purpose Register 0 (legacy alias)
    pub LPGPR0_legacy_alias: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR0: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR1: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR2: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR3: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR4: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR5: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR6: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR7: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR_alias0: RWRegister<u32>,

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR_alias1: RWRegister<u32>,

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR_alias2: RWRegister<u32>,

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR_alias3: RWRegister<u32>,

    /// SNVS_LP Tamper Detectors Config 2 Register
    pub LPTDC2R: RWRegister<u32>,

    /// SNVS_LP Tamper Detectors Status Register
    pub LPTDSR: RWRegister<u32>,

    /// SNVS_LP Tamper Glitch Filter 1 Configuration Register
    pub LPTGF1CR: RWRegister<u32>,

    /// SNVS_LP Tamper Glitch Filter 2 Configuration Register
    pub LPTGF2CR: RWRegister<u32>,

    _reserved2: [u32; 4],

    /// SNVS_LP Active Tamper 1 Configuration Register
    pub LPAT1CR: WORegister<u32>,

    /// SNVS_LP Active Tamper 2 Configuration Register
    pub LPAT2CR: WORegister<u32>,

    /// SNVS_LP Active Tamper 3 Configuration Register
    pub LPAT3CR: WORegister<u32>,

    /// SNVS_LP Active Tamper 4 Configuration Register
    pub LPAT4CR: WORegister<u32>,

    /// SNVS_LP Active Tamper 5 Configuration Register
    pub LPAT5CR: WORegister<u32>,

    _reserved3: [u32; 3],

    /// SNVS_LP Active Tamper Control Register
    pub LPATCTLR: RWRegister<u32>,

    /// SNVS_LP Active Tamper Clock Control Register
    pub LPATCLKR: RWRegister<u32>,

    /// SNVS_LP Active Tamper Routing Control 1 Register
    pub LPATRC1R: RWRegister<u32>,

    /// SNVS_LP Active Tamper Routing Control 2 Register
    pub LPATRC2R: RWRegister<u32>,

    _reserved4: [u32; 4],

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR0: RWRegister<u32>,

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR1: RWRegister<u32>,

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR2: RWRegister<u32>,

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR3: RWRegister<u32>,

    _reserved5: [u32; 698],

    /// SNVS_HP Version ID Register 1
    pub HPVIDR1: RORegister<u32>,

    /// SNVS_HP Version ID Register 2
    pub HPVIDR2: RORegister<u32>,
}
pub struct ResetValues {
    pub HPLR: u32,
    pub HPCOMR: u32,
    pub HPCR: u32,
    pub HPSICR: u32,
    pub HPSVCR: u32,
    pub HPSR: u32,
    pub HPSVSR: u32,
    pub HPHACIVR: u32,
    pub HPHACR: u32,
    pub HPRTCMR: u32,
    pub HPRTCLR: u32,
    pub HPTAMR: u32,
    pub HPTALR: u32,
    pub LPLR: u32,
    pub LPCR: u32,
    pub LPMKCR: u32,
    pub LPSVCR: u32,
    pub LPTGFCR: u32,
    pub LPTDCR: u32,
    pub LPSR: u32,
    pub LPSRTCMR: u32,
    pub LPSRTCLR: u32,
    pub LPTAR: u32,
    pub LPSMCMR: u32,
    pub LPSMCLR: u32,
    pub LPLVDR: u32,
    pub LPGPR0_legacy_alias: u32,
    pub LPZMKR0: u32,
    pub LPZMKR1: u32,
    pub LPZMKR2: u32,
    pub LPZMKR3: u32,
    pub LPZMKR4: u32,
    pub LPZMKR5: u32,
    pub LPZMKR6: u32,
    pub LPZMKR7: u32,
    pub LPGPR_alias0: u32,
    pub LPGPR_alias1: u32,
    pub LPGPR_alias2: u32,
    pub LPGPR_alias3: u32,
    pub LPTDC2R: u32,
    pub LPTDSR: u32,
    pub LPTGF1CR: u32,
    pub LPTGF2CR: u32,
    pub LPAT1CR: u32,
    pub LPAT2CR: u32,
    pub LPAT3CR: u32,
    pub LPAT4CR: u32,
    pub LPAT5CR: u32,
    pub LPATCTLR: u32,
    pub LPATCLKR: u32,
    pub LPATRC1R: u32,
    pub LPATRC2R: u32,
    pub LPGPR0: u32,
    pub LPGPR1: u32,
    pub LPGPR2: u32,
    pub LPGPR3: u32,
    pub HPVIDR1: u32,
    pub HPVIDR2: u32,
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
