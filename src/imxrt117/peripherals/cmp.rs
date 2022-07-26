#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CMP
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Version ID Register
pub mod VERID {

    /// Feature Specification Number. This read only filed returns the feature set number.
    pub mod FEATURE {
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

    /// Minor Version Number. This read only field returns the minor version number for the module specification.
    pub mod MINOR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Major Version Number. This read only field returns the major version number for the module specification.
    pub mod MAJOR {
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

/// Parameter Register
pub mod PARAM {

    /// Parameter Registers. This read only filed returns the feature parameters implemented along with the Version ID register.
    pub mod PARAM {
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

/// CMP Control Register 0
pub mod C0 {

    /// Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level
    pub mod HYSTCTR {
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

            /// 0b00: The hard block output has level 0 hysteresis internally.
            pub const HYSTCTR_0: u32 = 0b00;

            /// 0b01: The hard block output has level 1 hysteresis internally.
            pub const HYSTCTR_1: u32 = 0b01;

            /// 0b10: The hard block output has level 2 hysteresis internally.
            pub const HYSTCTR_2: u32 = 0b10;

            /// 0b11: The hard block output has level 3 hysteresis internally.
            pub const HYSTCTR_3: u32 = 0b11;
        }
    }

    /// Filter Sample Count
    pub mod FILTER_CNT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA.
            pub const FILTER_CNT_0: u32 = 0b000;

            /// 0b001: 1 consecutive sample must agree (comparator output is simply sampled).
            pub const FILTER_CNT_1: u32 = 0b001;

            /// 0b010: 2 consecutive samples must agree.
            pub const FILTER_CNT_2: u32 = 0b010;

            /// 0b011: 3 consecutive samples must agree.
            pub const FILTER_CNT_3: u32 = 0b011;

            /// 0b100: 4 consecutive samples must agree.
            pub const FILTER_CNT_4: u32 = 0b100;

            /// 0b101: 5 consecutive samples must agree.
            pub const FILTER_CNT_5: u32 = 0b101;

            /// 0b110: 6 consecutive samples must agree.
            pub const FILTER_CNT_6: u32 = 0b110;

            /// 0b111: 7 consecutive samples must agree.
            pub const FILTER_CNT_7: u32 = 0b111;
        }
    }

    /// Comparator Module Enable
    pub mod EN {
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

            /// 0b0: Analog Comparator is disabled.
            pub const EN_0: u32 = 0b0;

            /// 0b1: Analog Comparator is enabled.
            pub const EN_1: u32 = 0b1;
        }
    }

    /// Comparator Output Pin Enable
    pub mod OPE {
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

            /// 0b0: When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin.
            pub const OPE_0: u32 = 0b0;

            /// 0b1: When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin.
            pub const OPE_1: u32 = 0b1;
        }
    }

    /// Comparator Output Select
    pub mod COS {
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

            /// 0b0: Set CMPO to equal COUT (filtered comparator output).
            pub const COS_0: u32 = 0b0;

            /// 0b1: Set CMPO to equal COUTA (unfiltered comparator output).
            pub const COS_1: u32 = 0b1;
        }
    }

    /// Comparator invert
    pub mod INVT {
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

            /// 0b0: Does not invert the comparator output.
            pub const INVT_0: u32 = 0b0;

            /// 0b1: Inverts the comparator output.
            pub const INVT_1: u32 = 0b1;
        }
    }

    /// Power Mode Select
    pub mod PMODE {
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

            /// 0b0: Low Speed (LS) comparison mode is selected.
            pub const PMODE_0: u32 = 0b0;

            /// 0b1: High Speed (HS) comparison mode is selected.
            pub const PMODE_1: u32 = 0b1;
        }
    }

    /// Windowing Enable
    pub mod WE {
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

            /// 0b0: Windowing mode is not selected.
            pub const WE_0: u32 = 0b0;

            /// 0b1: Windowing mode is selected.
            pub const WE_1: u32 = 0b1;
        }
    }

    /// Sample Enable
    pub mod SE {
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

            /// 0b0: Sampling mode is not selected.
            pub const SE_0: u32 = 0b0;

            /// 0b1: Sampling mode is selected.
            pub const SE_1: u32 = 0b1;
        }
    }

    /// Filter Sample Period
    pub mod FPR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog Comparator Output
    pub mod COUT {
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

    /// Analog Comparator Flag Falling
    pub mod CFF {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A falling edge has not been detected on COUT.
            pub const CFF_0: u32 = 0b0;

            /// 0b1: A falling edge on COUT has occurred.
            pub const CFF_1: u32 = 0b1;
        }
    }

    /// Analog Comparator Flag Rising
    pub mod CFR {
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

            /// 0b0: A rising edge has not been detected on COUT.
            pub const CFR_0: u32 = 0b0;

            /// 0b1: A rising edge on COUT has occurred.
            pub const CFR_1: u32 = 0b1;
        }
    }

    /// Comparator Interrupt Enable Falling
    pub mod IEF {
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

            /// 0b0: Interrupt is disabled.
            pub const IEF_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled.
            pub const IEF_1: u32 = 0b1;
        }
    }

    /// Comparator Interrupt Enable Rising
    pub mod IER {
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

            /// 0b0: Interrupt is disabled.
            pub const IER_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled.
            pub const IER_1: u32 = 0b1;
        }
    }

    /// DMA Enable
    pub mod DMAEN {
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

            /// 0b0: DMA is disabled.
            pub const DMAEN_0: u32 = 0b0;

            /// 0b1: DMA is enabled.
            pub const DMAEN_1: u32 = 0b1;
        }
    }

    /// CMP to DAC link enable.
    pub mod LINKEN {
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

            /// 0b0: CMP to DAC link is disabled
            pub const LINKEN_0: u32 = 0b0;

            /// 0b1: CMP to DAC link is enabled.
            pub const LINKEN_1: u32 = 0b1;
        }
    }
}

/// CMP Control Register 1
pub mod C1 {

    /// DAC Output Voltage Select
    pub mod VOSEL {
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

    /// DAC Mode Selection
    pub mod DMODE {
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

            /// 0b0: DAC is selected to work in low speed and low power mode.
            pub const DMODE_0: u32 = 0b0;

            /// 0b1: DAC is selected to work in high speed high power mode.
            pub const DMODE_1: u32 = 0b1;
        }
    }

    /// Supply Voltage Reference Source Select
    pub mod VRSEL {
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

            /// 0b0: Vin1 is selected as resistor ladder network supply reference Vin. Vin1 is from internal PMC.
            pub const VRSEL_0: u32 = 0b0;

            /// 0b1: Vin2 is selected as resistor ladder network supply reference Vin. Vin2 is from PAD.
            pub const VRSEL_1: u32 = 0b1;
        }
    }

    /// DAC Enable
    pub mod DACEN {
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

            /// 0b0: DAC is disabled.
            pub const DACEN_0: u32 = 0b0;

            /// 0b1: DAC is enabled.
            pub const DACEN_1: u32 = 0b1;
        }
    }

    /// Channel 0 input enable
    pub mod CHN0 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel 1 input enable
    pub mod CHN1 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel 2 input enable
    pub mod CHN2 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel 3 input enable
    pub mod CHN3 {
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

    /// Channel 4 input enable
    pub mod CHN4 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel 5 input enable
    pub mod CHN5 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Minus Input MUX Control
    pub mod MSEL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Internal Negative Input 0 for Minus Channel -- Internal Minus Input
            pub const MSEL_0: u32 = 0b000;

            /// 0b001: External Input 1 for Minus Channel -- Reference Input 0
            pub const MSEL_1: u32 = 0b001;

            /// 0b010: External Input 2 for Minus Channel -- Reference Input 1
            pub const MSEL_2: u32 = 0b010;

            /// 0b011: External Input 3 for Minus Channel -- Reference Input 2
            pub const MSEL_3: u32 = 0b011;

            /// 0b100: External Input 4 for Minus Channel -- Reference Input 3
            pub const MSEL_4: u32 = 0b100;

            /// 0b101: External Input 5 for Minus Channel -- Reference Input 4
            pub const MSEL_5: u32 = 0b101;

            /// 0b110: External Input 6 for Minus Channel -- Reference Input 5
            pub const MSEL_6: u32 = 0b110;

            /// 0b111: Internal 8b DAC output
            pub const MSEL_7: u32 = 0b111;
        }
    }

    /// Plus Input MUX Control
    pub mod PSEL {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (3 bits: 0b111 << 28)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Internal Positive Input 0 for Plus Channel -- Internal Plus Input
            pub const PSEL_0: u32 = 0b000;

            /// 0b001: External Input 1 for Plus Channel -- Reference Input 0
            pub const PSEL_1: u32 = 0b001;

            /// 0b010: External Input 2 for Plus Channel -- Reference Input 1
            pub const PSEL_2: u32 = 0b010;

            /// 0b011: External Input 3 for Plus Channel -- Reference Input 2
            pub const PSEL_3: u32 = 0b011;

            /// 0b100: External Input 4 for Plus Channel -- Reference Input 3
            pub const PSEL_4: u32 = 0b100;

            /// 0b101: External Input 5 for Plus Channel -- Reference Input 4
            pub const PSEL_5: u32 = 0b101;

            /// 0b110: External Input 6 for Plus Channel -- Reference Input 5
            pub const PSEL_6: u32 = 0b110;

            /// 0b111: Internal 8b DAC output
            pub const PSEL_7: u32 = 0b111;
        }
    }
}

/// CMP Control Register 2
pub mod C2 {

    /// ACOn
    pub mod ACOn {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Comparator and DAC initialization delay modulus.
    pub mod INITMOD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of sample clocks
    pub mod NSAM {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: The comparison result is sampled as soon as the active channel is scanned in one round-robin clock.
            pub const NSAM_0: u32 = 0b00;

            /// 0b01: The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock.
            pub const NSAM_1: u32 = 0b01;

            /// 0b10: The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock.
            pub const NSAM_2: u32 = 0b10;

            /// 0b11: The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock.
            pub const NSAM_3: u32 = 0b11;
        }
    }

    /// CH0F
    pub mod CH0F {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CH1F
    pub mod CH1F {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CH2F
    pub mod CH2F {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CH3F
    pub mod CH3F {
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

    /// CH4F
    pub mod CH4F {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CH5F
    pub mod CH5F {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fixed channel selection
    pub mod FXMXCH {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (3 bits: 0b111 << 25)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: External Reference Input 0 is selected as the fixed reference input for the fixed mux port.
            pub const FXMXCH_0: u32 = 0b000;

            /// 0b001: External Reference Input 1 is selected as the fixed reference input for the fixed mux port.
            pub const FXMXCH_1: u32 = 0b001;

            /// 0b010: External Reference Input 2 is selected as the fixed reference input for the fixed mux port.
            pub const FXMXCH_2: u32 = 0b010;

            /// 0b011: External Reference Input 3 is selected as the fixed reference input for the fixed mux port.
            pub const FXMXCH_3: u32 = 0b011;

            /// 0b100: External Reference Input 4 is selected as the fixed reference input for the fixed mux port.
            pub const FXMXCH_4: u32 = 0b100;

            /// 0b101: External Reference Input 5 is selected as the fixed reference input for the fixed mux port.
            pub const FXMXCH_5: u32 = 0b101;

            /// 0b111: The 8bit DAC is selected as the fixed reference input for the fixed mux port.
            pub const FXMXCH_7: u32 = 0b111;
        }
    }

    /// Fixed MUX Port
    pub mod FXMP {
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

            /// 0b0: The Plus port is fixed. Only the inputs to the Minus port are swept in each round.
            pub const FXMP_0: u32 = 0b0;

            /// 0b1: The Minus port is fixed. Only the inputs to the Plus port are swept in each round.
            pub const FXMP_1: u32 = 0b1;
        }
    }

    /// Round-Robin interrupt enable
    pub mod RRIE {
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

            /// 0b0: The round-robin interrupt is disabled.
            pub const RRIE_0: u32 = 0b0;

            /// 0b1: The round-robin interrupt is enabled when a comparison result changes from the last sample.
            pub const RRIE_1: u32 = 0b1;
        }
    }
}

/// CMP Control Register 3
pub mod C3 {

    /// Analog Comparator Phase2 Timing Control.
    pub mod ACPH2TC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Phase2 active time in one sampling period equals to T
            pub const ACPH2TC_0: u32 = 0b000;

            /// 0b001: Phase2 active time in one sampling period equals to 2*T
            pub const ACPH2TC_1: u32 = 0b001;

            /// 0b010: Phase2 active time in one sampling period equals to 4*T
            pub const ACPH2TC_2: u32 = 0b010;

            /// 0b011: Phase2 active time in one sampling period equals to 8*T
            pub const ACPH2TC_3: u32 = 0b011;

            /// 0b100: Phase2 active time in one sampling period equals to 16*T
            pub const ACPH2TC_4: u32 = 0b100;

            /// 0b101: Phase2 active time in one sampling period equals to 32*T
            pub const ACPH2TC_5: u32 = 0b101;

            /// 0b110: Phase2 active time in one sampling period equals to 64*T
            pub const ACPH2TC_6: u32 = 0b110;

            /// 0b111: Phase2 active time in one sampling period equals to 16*T
            pub const ACPH2TC_7: u32 = 0b111;
        }
    }

    /// Analog Comparator Phase1 Timing Control.
    pub mod ACPH1TC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Phase1 active time in one sampling period equals to T
            pub const ACPH1TC_0: u32 = 0b000;

            /// 0b001: Phase1 active time in one sampling period equals to 2*T
            pub const ACPH1TC_1: u32 = 0b001;

            /// 0b010: Phase1 active time in one sampling period equals to 4*T
            pub const ACPH1TC_2: u32 = 0b010;

            /// 0b011: Phase1 active time in one sampling period equals to 8*T
            pub const ACPH1TC_3: u32 = 0b011;

            /// 0b100: Phase1 active time in one sampling period equals to T
            pub const ACPH1TC_4: u32 = 0b100;

            /// 0b101: Phase1 active time in one sampling period equals to T
            pub const ACPH1TC_5: u32 = 0b101;

            /// 0b110: Phase1 active time in one sampling period equals to T
            pub const ACPH1TC_6: u32 = 0b110;

            /// 0b111: Phase1 active time in one sampling period equals to 0
            pub const ACPH1TC_7: u32 = 0b111;
        }
    }

    /// Analog Comparator Sampling Time control.
    pub mod ACSAT {
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

            /// 0b000: The sampling time equals to T
            pub const ACSAT_0: u32 = 0b000;

            /// 0b001: The sampling time equasl to 2*T
            pub const ACSAT_1: u32 = 0b001;

            /// 0b010: The sampling time equasl to 4*T
            pub const ACSAT_2: u32 = 0b010;

            /// 0b011: The sampling time equasl to 8*T
            pub const ACSAT_3: u32 = 0b011;

            /// 0b100: The sampling time equasl to 16*T
            pub const ACSAT_4: u32 = 0b100;

            /// 0b101: The sampling time equasl to 32*T
            pub const ACSAT_5: u32 = 0b101;

            /// 0b110: The sampling time equasl to 64*T
            pub const ACSAT_6: u32 = 0b110;

            /// 0b111: The sampling time equasl to 256*T
            pub const ACSAT_7: u32 = 0b111;
        }
    }

    /// Discrete Mode Clock Selection
    pub mod DMCS {
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

            /// 0b0: Slow clock is selected for the timing generation.
            pub const DMCS_0: u32 = 0b0;

            /// 0b1: Fast clock is selected for the timing generation.
            pub const DMCS_1: u32 = 0b1;
        }
    }

    /// Resistor Divider Enable
    pub mod RDIVE {
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

            /// 0b0: The resistor is not enabled even when either NCHEN or PCHEN is set to1 but the actual input is in the range of 0 - 1.8v.
            pub const RDIVE_0: u32 = 0b0;

            /// 0b1: The resistor is enabled because the inputs are above 1.8v.
            pub const RDIVE_1: u32 = 0b1;
        }
    }

    /// Negative Channel Continuous Mode Enable.
    pub mod NCHCTEN {
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

            /// 0b0: Negative channel is in Discrete Mode and special timing needs to be configured.
            pub const NCHCTEN_0: u32 = 0b0;

            /// 0b1: Negative channel is in Continuous Mode and no special timing is requried.
            pub const NCHCTEN_1: u32 = 0b1;
        }
    }

    /// Positive Channel Continuous Mode Enable.
    pub mod PCHCTEN {
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

            /// 0b0: Positive channel is in Discrete Mode and special timing needs to be configured.
            pub const PCHCTEN_0: u32 = 0b0;

            /// 0b1: Positive channel is in Continuous Mode and no special timing is requried.
            pub const PCHCTEN_1: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Version ID Register
    pub VERID: RORegister<u32>,

    /// Parameter Register
    pub PARAM: RORegister<u32>,

    /// CMP Control Register 0
    pub C0: RWRegister<u32>,

    /// CMP Control Register 1
    pub C1: RWRegister<u32>,

    /// CMP Control Register 2
    pub C2: RWRegister<u32>,

    /// CMP Control Register 3
    pub C3: RWRegister<u32>,
}
pub struct ResetValues {
    pub VERID: u32,
    pub PARAM: u32,
    pub C0: u32,
    pub C1: u32,
    pub C2: u32,
    pub C3: u32,
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
