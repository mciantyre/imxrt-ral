#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DCDC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::RWRegister;

/// DCDC Control Register 0
pub mod CTRL0 {

    /// DCDC Enable
    pub mod ENABLE {
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

            /// 0b0: Disable (Bypass)
            pub const disable: u32 = 0b0;

            /// 0b1: Enable
            pub const enable: u32 = 0b1;
        }
    }

    /// Enable the DCDC_DIG switching converter output
    pub mod DIG_EN {
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

            /// 0b1: Enable
            pub const enable: u32 = 0b1;
        }
    }

    /// DCDC standby mode enable
    pub mod STBY_EN {
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

            /// 0b1: Enter into standby mode
            pub const enable: u32 = 0b1;
        }
    }

    /// DCDC low-power (LP) mode enable DCDC can't start up directly into LP mode
    pub mod LP_MODE_EN {
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

            /// 0b1: Enter into low-power mode
            pub const enable: u32 = 0b1;
        }
    }

    /// DCDC low-power mode enable by GPC standby request
    pub mod STBY_LP_MODE_EN {
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

            /// 0b0: Disable DCDC entry into low-power mode from a GPC standby request
            pub const disable: u32 = 0b0;

            /// 0b1: Enable DCDC to enter into low-power mode from a GPC standby request
            pub const enable: u32 = 0b1;
        }
    }

    /// Enable internal count for DCDC_OK timeout
    pub mod ENABLE_DCDC_CNT {
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

            /// 0b0: Wait DCDC_OK for ACK
            pub const wait: u32 = 0b0;

            /// 0b1: Enable internal count for DCDC_OK timeout
            pub const enable_count: u32 = 0b1;
        }
    }

    /// Hold trim input
    pub mod TRIM_HOLD {
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

            /// 0b0: Sample trim input
            pub const sample: u32 = 0b0;

            /// 0b1: Hold trim input
            pub const hold: u32 = 0b1;
        }
    }

    /// DEBUG_BITS\[11:0\]
    pub mod DEBUG_BITS {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (12 bits: 0xfff << 19)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control mode
    pub mod CONTROL_MODE {
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

            /// 0b0: Software control mode
            pub const swctrl: u32 = 0b0;

            /// 0b1: Hardware control mode (controlled by GPC Setpoints)
            pub const gpc: u32 = 0b1;
        }
    }
}

/// DCDC Control Register 1
pub mod CTRL1 {

    /// Target value of VDD1P8 in buck mode, 25mV each step from 0x00 to 0x1F:
    pub mod VDD1P8CTRL_TRG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 1.5V
            pub const v1p5: u32 = 0b00000;

            /// 0b01100: 1.8V
            pub const v1p8: u32 = 0b01100;

            /// 0b11111: 2.275V
            pub const v2p275: u32 = 0b11111;
        }
    }

    /// Target value of VDD1P0 in buck mode, 25mV each step from 0x00 to 0x1F:
    pub mod VDD1P0CTRL_TRG {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 0.6V
            pub const v0p6: u32 = 0b00000;

            /// 0b10000: 1.0V
            pub const v1p0: u32 = 0b10000;

            /// 0b11111: 1.375V
            pub const v1p375: u32 = 0b11111;
        }
    }

    /// Target value of VDD1P8 in standby mode, 25mV each step from 0x00 to 0x1F:
    pub mod VDD1P8CTRL_STBY_TRG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 1.525V
            pub const v1p525: u32 = 0b00000;

            /// 0b01011: 1.8V
            pub const v1p8: u32 = 0b01011;

            /// 0b11111: 2.3V
            pub const v2p4: u32 = 0b11111;
        }
    }

    /// Target value of VDD1P0 in standby mode, 25mV each step from 0x00 to 0x1F:
    pub mod VDD1P0CTRL_STBY_TRG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 0.625V
            pub const v0p625: u32 = 0b00000;

            /// 0b01111: 1.0V
            pub const v1p0: u32 = 0b01111;

            /// 0b11111: 1.4V
            pub const v1p4: u32 = 0b11111;
        }
    }
}

/// DCDC Register 0
pub mod REG0 {

    /// Power Down Zero Cross Detection
    pub mod PWD_ZCD {
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

            /// 0b0: Zero cross detetion function powered up
            pub const powered_up: u32 = 0b0;

            /// 0b1: Zero cross detetion function powered down
            pub const powered_down: u32 = 0b1;
        }
    }

    /// Disable Auto Clock Switch
    pub mod DISABLE_AUTO_CLK_SWITCH {
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

            /// 0b0: If DISABLE_AUTO_CLK_SWITCH is set to 0 and 24M xtal is OK, the clock source will switch from internal ring oscillator to 24M xtal automatically
            pub const xtal_clk: u32 = 0b0;

            /// 0b1: If DISABLE_AUTO_CLK_SWITCH is set to 1, SEL_CLK will determine which clock source the DCDC uses
            pub const sel_clk: u32 = 0b1;
        }
    }

    /// Select Clock
    pub mod SEL_CLK {
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

            /// 0b0: DCDC uses internal ring oscillator
            pub const int_rng_osc: u32 = 0b0;

            /// 0b1: DCDC uses 24M xtal
            pub const xtal_24M: u32 = 0b1;
        }
    }

    /// Power down internal ring oscillator
    pub mod PWD_OSC_INT {
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

            /// 0b0: Internal ring oscillator powered up
            pub const powered_up: u32 = 0b0;

            /// 0b1: Internal ring oscillator powered down
            pub const powered_down: u32 = 0b1;
        }
    }

    /// Power down signal of the current detector
    pub mod PWD_CUR_SNS_CMP {
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

            /// 0b0: Current Detector powered up
            pub const powered_up: u32 = 0b0;

            /// 0b1: Current Detector powered down
            pub const powered_down: u32 = 0b1;
        }
    }

    /// Current Sense (detector) Threshold
    pub mod CUR_SNS_THRSH {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (3 bits: 0b111 << 5)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power down overcurrent detection comparator
    pub mod PWD_OVERCUR_DET {
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

            /// 0b0: Overcurrent detection comparator is enabled
            pub const enabled: u32 = 0b0;

            /// 0b1: Overcurrent detection comparator is disabled
            pub const disabled: u32 = 0b1;
        }
    }

    /// Set to "1" to power down the low voltage detection comparator
    pub mod PWD_CMP_DCDC_IN_DET {
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

            /// 0b0: Low voltage detection comparator is enabled
            pub const enabled: u32 = 0b0;

            /// 0b1: Low voltage detection comparator is disabled
            pub const disabled: u32 = 0b1;
        }
    }

    /// Power Down High Voltage Detection for VDD1P8
    pub mod PWD_HIGH_VDD1P8_DET {
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

            /// 0b0: Overvoltage detection comparator for the VDD1P8 output is enabled
            pub const enabled: u32 = 0b0;

            /// 0b1: Overvoltage detection comparator for the VDD1P8 output is disabled
            pub const disabled: u32 = 0b1;
        }
    }

    /// Power Down High Voltage Detection for VDD1P0
    pub mod PWD_HIGH_VDD1P0_DET {
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

            /// 0b0: Overvoltage detection comparator for the VDD1P0 output is enabled
            pub const enabled: u32 = 0b0;

            /// 0b1: Overvoltage detection comparator for the VDD1P0 output is disabled
            pub const disabled: u32 = 0b1;
        }
    }

    /// Low Power High Hysteric Value
    pub mod LP_HIGH_HYS {
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

            /// 0b0: Adjust hysteretic value in low power to 12.5mV
            pub const lp_12p5mV: u32 = 0b0;

            /// 0b1: Adjust hysteretic value in low power to 25mV
            pub const lp_25mV: u32 = 0b1;
        }
    }

    /// power down the out-of-range detection comparator
    pub mod PWD_CMP_OFFSET {
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

            /// 0b0: Out-of-range comparator powered up
            pub const powered_up: u32 = 0b0;

            /// 0b1: Out-of-range comparator powered down
            pub const powered_down: u32 = 0b1;
        }
    }

    /// Disable xtalok detection circuit
    pub mod XTALOK_DISABLE {
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

            /// 0b0: Enable xtalok detection circuit
            pub const enabled: u32 = 0b0;

            /// 0b1: Disable xtalok detection circuit and always outputs OK signal "1"
            pub const disabled: u32 = 0b1;
        }
    }

    /// 24M XTAL OK
    pub mod XTAL_24M_OK {
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

            /// 0b0: DCDC uses internal ring oscillator
            pub const int_rng_osc: u32 = 0b0;

            /// 0b1: DCDC uses xtal 24M
            pub const xtal_24M: u32 = 0b1;
        }
    }

    /// DCDC Output OK
    pub mod STS_DC_OK {
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

            /// 0b0: DCDC is settling
            pub const not_settled: u32 = 0b0;

            /// 0b1: DCDC already settled
            pub const settled: u32 = 0b1;
        }
    }
}

/// DCDC Register 1
pub mod REG1 {

    /// DM Control
    pub mod DM_CTRL {
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

            /// 0b0: No change to ripple when the discontinuous current is present in DCM.
            pub const DM_CTRL_0: u32 = 0b0;

            /// 0b1: Improves ripple when the inductor current goes to zero in DCM.
            pub const DM_CTRL_1: u32 = 0b1;
        }
    }

    /// Load Resistor Enable
    pub mod RLOAD_REG_EN_LPSR {
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

            /// 0b0: Disconnect load resistor
            pub const loadR_disconnect: u32 = 0b0;

            /// 0b1: Connect load resistor
            pub const loadR_connect: u32 = 0b1;
        }
    }

    /// Trim Bandgap Voltage
    pub mod VBG_TRIM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 0.452V
            pub const minvolt: u32 = 0b00000;

            /// 0b10000: 0.5V
            pub const default: u32 = 0b10000;

            /// 0b11111: 0.545V
            pub const maxvolt: u32 = 0b11111;
        }
    }

    /// Low Power Comparator Current Bias
    pub mod LP_CMP_ISRC_SEL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 50nA
            pub const sel0: u32 = 0b00;

            /// 0b01: 100nA
            pub const sel1: u32 = 0b01;

            /// 0b10: 200nA
            pub const sel2: u32 = 0b10;

            /// 0b11: 400nA
            pub const sel3: u32 = 0b11;
        }
    }

    /// Increase Threshold Detection
    pub mod LOOPCTRL_CM_HST_THRESH {
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

    /// Increase Threshold Detection
    pub mod LOOPCTRL_DF_HST_THRESH {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable hysteresis in switching converter common mode analog comparators
    pub mod LOOPCTRL_EN_CM_HYST {
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

            /// 0b0: Disable hysteresis in switching converter common mode analog comparators
            pub const disable: u32 = 0b0;

            /// 0b1: Enable hysteresis in switching converter common mode analog comparators
            pub const enable: u32 = 0b1;
        }
    }

    /// Enable hysteresis in switching converter differential mode analog comparators
    pub mod LOOPCTRL_EN_DF_HYST {
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

            /// 0b0: Disable hysteresis in switching converter differential mode analog comparators
            pub const disable: u32 = 0b0;

            /// 0b1: Enable hysteresis in switching converter differential mode analog comparators
            pub const enable: u32 = 0b1;
        }
    }
}

/// DCDC Register 2
pub mod REG2 {

    /// Ratio of integral control parameter to proportional control parameter in the switching DCDC converter, and can be used to optimize efficiency and loop response
    pub mod LOOPCTRL_DC_C {
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

    /// Magnitude of proportional control parameter in the switching DCDC converter control loop.
    pub mod LOOPCTRL_DC_R {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (4 bits: 0b1111 << 2)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Two's complement feed forward step in duty cycle in the switching DCDC converter
    pub mod LOOPCTRL_DC_FF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable RC Scale
    pub mod LOOPCTRL_EN_RCSCALE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Increase the threshold detection for RC scale circuit.
    pub mod LOOPCTRL_RCSCALE_THRSH {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Invert the sign of the hysteresis in DCDC analog comparators.
    pub mod LOOPCTRL_HYST_SIGN {
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

    /// This bit enables the DCDC to improve efficiency and minimize ripple using the information from the BATT_VAL field
    pub mod BATTMONITOR_EN_BATADJ {
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

    /// Software should be configured to place the battery voltage in this register measured with an 8-mV LSB resolution through the ADC
    pub mod BATTMONITOR_BATT_VAL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (10 bits: 0x3ff << 16)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DCM Set Control
    pub mod DCM_SET_CTRL {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set high to enable supply stepping to change only after the differential control loop has toggled as well
    pub mod LOOPCTRL_TOGGLE_DIF {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCDC Register 3
pub mod REG3 {

    /// signal "1" when the voltage on DCDC_IN is lower than 2.6V
    pub mod IN_BROWNOUT {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: DCDC_IN is lower than 2.6V
            pub const brownout: u32 = 0b1;
        }
    }

    /// signal "1" when overvoltage on the VDD1P8 output happens
    pub mod OVERVOLT_VDD1P8_DET_OUT {
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

            /// 0b1: VDD1P8 Overvoltage
            pub const overvoltage_1p8: u32 = 0b1;
        }
    }

    /// signal "1" when overvoltage on the VDD1P0 output happens
    pub mod OVERVOLT_VDD1P0_DET_OUT {
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

            /// 0b1: VDD1P0 Overvoltage
            pub const overvoltage_1p0: u32 = 0b1;
        }
    }

    /// signal "1" when overcurrent happens.
    pub mod OVERCUR_DETECT_OUT {
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

            /// 0b1: Overcurrent
            pub const overcurrent_signal: u32 = 0b1;
        }
    }

    /// no description available
    pub mod ENABLE_FF {
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

            /// 0b1: Enable feed-forward (FF) function that can speed up transient settling.
            pub const enable_ff: u32 = 0b1;
        }
    }

    /// Disable Pulse Skip
    pub mod DISABLE_PULSE_SKIP {
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

            /// 0b0: Stop charging if the duty cycle is lower than what is set by NEGLIMIT_IN
            pub const stopcharge: u32 = 0b0;
        }
    }

    /// no description available
    pub mod DISABLE_IDLE_SKIP {
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

            /// 0b0: Enable the idle skip function. The DCDC will be idle when out-of-range comparator detects the output voltage is higher than the target by 25mV. This function requires the out-of-range comparator to be enabled (PWD_CMP_OFFSET=0).
            pub const enable: u32 = 0b0;
        }
    }

    /// no description available
    pub mod DOUBLE_IBIAS_CMP_LP_LPSR {
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

            /// 0b1: Double the bias current of the comparator for low-voltage detector in LP (low-power) mode
            pub const doublebias: u32 = 0b1;
        }
    }

    /// Select the feedback point of the internal regulator
    pub mod REG_FBK_SEL {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set DCDC clock to half freqeuncy for continuous mode.
    pub mod MINPWR_DC_HALFCLK {
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

            /// 0b0: DCDC clock remains at full frequency for continuous mode
            pub const fullfreq: u32 = 0b0;

            /// 0b1: DCDC clock set to half frequency for continuous mode
            pub const halffreq: u32 = 0b1;
        }
    }

    /// Use half switch FET
    pub mod MINPWR_HALF_FETS {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Miscellaneous Delay Timing
    pub mod MISC_DELAY_TIMING {
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

    /// Disable Step for VDD1P0
    pub mod VDD1P0CTRL_DISABLE_STEP {
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

            /// 0b0: Enable stepping for VDD1P0
            pub const enable: u32 = 0b0;

            /// 0b1: Disable stepping for VDD1P0
            pub const disable: u32 = 0b1;
        }
    }

    /// Disable Step for VDD1P8
    pub mod VDD1P8CTRL_DISABLE_STEP {
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

            /// 0b0: Enable stepping for VDD1P8
            pub const enable: u32 = 0b0;

            /// 0b1: Disable stepping for VDD1P8
            pub const disable: u32 = 0b1;
        }
    }
}

/// DCDC Register 4
pub mod REG4 {

    /// Configures CTRL0\[ENABLE\] (DCDC Enable) for Setpoints 0-15
    pub mod ENABLE_SP {
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
}

/// DCDC Register 5
pub mod REG5 {

    /// Configures CTRL0\[DIG_EN\] (DCDC_DIG Enable) for Setpoints 0-15. Always set these bits to 1.
    pub mod DIG_EN_SP {
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
}

/// DCDC Register 6
pub mod REG6 {

    /// Configures CTRL0\[LP_MODE_EN\] (LP Mode Enable) for Setpoints 0-15
    pub mod LP_MODE_SP {
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
}

/// DCDC Register 7
pub mod REG7 {

    /// Configures CTRL0\[STBY_EN\] (Standby Enable) for Setpoints 0-15
    pub mod STBY_EN_SP {
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
}

/// DCDC Register 7 plus
pub mod REG7P {

    /// Configures CTRL0\[STBY_LP_MODE_EN\] (LP Mode via GPC Enable) for Setpoints 0-15
    pub mod STBY_LP_MODE_SP {
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
}

/// DCDC Register 8
pub mod REG8 {

    /// Configures CTRL1\[VDD1P8CTRL_TRG\] FOR Setpoints 0-3
    pub mod ANA_TRG_SP0 {
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

/// DCDC Register 9
pub mod REG9 {

    /// Configures CTRL1\[VDD1P8CTRL_TRG\] FOR Setpoints 4-7
    pub mod ANA_TRG_SP1 {
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

/// DCDC Register 10
pub mod REG10 {

    /// Configures CTRL1\[VDD1P8CTRL_TRG\] FOR Setpoints 8-11
    pub mod ANA_TRG_SP2 {
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

/// DCDC Register 11
pub mod REG11 {

    /// Configures CTRL1\[VDD1P8CTRL_TRG\] FOR Setpoints 12-15
    pub mod ANA_TRG_SP3 {
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

/// DCDC Register 12
pub mod REG12 {

    /// Configures CTRL1\[VDD1P0CTRL_TRG\] FOR Setpoints 0-3
    pub mod DIG_TRG_SP0 {
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

/// DCDC Register 13
pub mod REG13 {

    /// Configures CTRL1\[VDD1P0CTRL_TRG\] FOR Setpoints 4-7
    pub mod DIG_TRG_SP1 {
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

/// DCDC Register 14
pub mod REG14 {

    /// Configures CTRL1\[VDD1P0CTRL_TRG\] FOR Setpoints 8-11
    pub mod DIG_TRG_SP2 {
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

/// DCDC Register 15
pub mod REG15 {

    /// Configures CTRL1\[VDD1P0CTRL_TRG\] FOR Setpoints 12-15
    pub mod DIG_TRG_SP3 {
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

/// DCDC Register 16
pub mod REG16 {

    /// Configures CTRL1\[VDD1P8CTRL_STBY_TRG\] FOR Setpoints 0-3
    pub mod ANA_STBY_TRG_SP0 {
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

/// DCDC Register 17
pub mod REG17 {

    /// Configures CTRL1\[VDD1P8CTRL_STBY_TRG\] FOR Setpoints 4-7
    pub mod ANA_STBY_TRG_SP1 {
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

/// DCDC Register 18
pub mod REG18 {

    /// Configures CTRL1\[VDD1P8CTRL_STBY_TRG\] FOR Setpoints 8-11
    pub mod ANA_STBY_TRG_SP2 {
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

/// DCDC Register 19
pub mod REG19 {

    /// Configures CTRL1\[VDD1P8CTRL_STBY_TRG\] FOR Setpoints 12-15
    pub mod ANA_STBY_TRG_SP3 {
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

/// DCDC Register 20
pub mod REG20 {

    /// Configures CTRL1\[VDD1P0CTRL_STBY_TRG\] FOR Setpoints 0-3
    pub mod DIG_STBY_TRG_SP0 {
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

/// DCDC Register 21
pub mod REG21 {

    /// Configures CTRL1\[VDD1P0CTRL_STBY_TRG\] FOR Setpoints 4-7
    pub mod DIG_STBY_TRG_SP1 {
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

/// DCDC Register 22
pub mod REG22 {

    /// Configures CTRL1\[VDD1P0CTRL_STBY_TRG\] FOR Setpoints 8-11
    pub mod DIG_STBY_TRG_SP2 {
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

/// DCDC Register 23
pub mod REG23 {

    /// Configures CTRL1\[VDD1P0CTRL_STBY_TRG\] FOR Setpoints 12-15
    pub mod DIG_STBY_TRG_SP3 {
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

/// DCDC Register 24
pub mod REG24 {

    /// Internal count for dcdc_ok timeout
    pub mod OK_COUNT {
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
#[repr(C)]
pub struct RegisterBlock {
    /// DCDC Control Register 0
    pub CTRL0: RWRegister<u32>,

    /// DCDC Control Register 1
    pub CTRL1: RWRegister<u32>,

    /// DCDC Register 0
    pub REG0: RWRegister<u32>,

    /// DCDC Register 1
    pub REG1: RWRegister<u32>,

    /// DCDC Register 2
    pub REG2: RWRegister<u32>,

    /// DCDC Register 3
    pub REG3: RWRegister<u32>,

    /// DCDC Register 4
    pub REG4: RWRegister<u32>,

    /// DCDC Register 5
    pub REG5: RWRegister<u32>,

    /// DCDC Register 6
    pub REG6: RWRegister<u32>,

    /// DCDC Register 7
    pub REG7: RWRegister<u32>,

    /// DCDC Register 7 plus
    pub REG7P: RWRegister<u32>,

    /// DCDC Register 8
    pub REG8: RWRegister<u32>,

    /// DCDC Register 9
    pub REG9: RWRegister<u32>,

    /// DCDC Register 10
    pub REG10: RWRegister<u32>,

    /// DCDC Register 11
    pub REG11: RWRegister<u32>,

    /// DCDC Register 12
    pub REG12: RWRegister<u32>,

    /// DCDC Register 13
    pub REG13: RWRegister<u32>,

    /// DCDC Register 14
    pub REG14: RWRegister<u32>,

    /// DCDC Register 15
    pub REG15: RWRegister<u32>,

    /// DCDC Register 16
    pub REG16: RWRegister<u32>,

    /// DCDC Register 17
    pub REG17: RWRegister<u32>,

    /// DCDC Register 18
    pub REG18: RWRegister<u32>,

    /// DCDC Register 19
    pub REG19: RWRegister<u32>,

    /// DCDC Register 20
    pub REG20: RWRegister<u32>,

    /// DCDC Register 21
    pub REG21: RWRegister<u32>,

    /// DCDC Register 22
    pub REG22: RWRegister<u32>,

    /// DCDC Register 23
    pub REG23: RWRegister<u32>,

    /// DCDC Register 24
    pub REG24: RWRegister<u32>,
}
pub struct ResetValues {
    pub CTRL0: u32,
    pub CTRL1: u32,
    pub REG0: u32,
    pub REG1: u32,
    pub REG2: u32,
    pub REG3: u32,
    pub REG4: u32,
    pub REG5: u32,
    pub REG6: u32,
    pub REG7: u32,
    pub REG7P: u32,
    pub REG8: u32,
    pub REG9: u32,
    pub REG10: u32,
    pub REG11: u32,
    pub REG12: u32,
    pub REG13: u32,
    pub REG14: u32,
    pub REG15: u32,
    pub REG16: u32,
    pub REG17: u32,
    pub REG18: u32,
    pub REG19: u32,
    pub REG20: u32,
    pub REG21: u32,
    pub REG22: u32,
    pub REG23: u32,
    pub REG24: u32,
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
