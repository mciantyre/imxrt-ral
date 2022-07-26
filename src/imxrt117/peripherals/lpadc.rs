#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPADC
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Version ID Register
pub mod VERID {

    /// Resolution
    pub mod RES {
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

            /// 0b0: Up to 13-bit differential/12-bit single ended resolution supported.
            pub const RES_0: u32 = 0b0;

            /// 0b1: Up to 16-bit differential/15-bit single ended resolution supported.
            pub const RES_1: u32 = 0b1;
        }
    }

    /// Differential Supported
    pub mod DIFFEN {
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

            /// 0b0: Differential operation not supported.
            pub const DIFFEN_0: u32 = 0b0;

            /// 0b1: Differential operation supported. CMDLa\[DIFF\] and CMDLa\[ABSEL\] control fields implemented.
            pub const DIFFEN_1: u32 = 0b1;
        }
    }

    /// Multi Vref Implemented
    pub mod MVI {
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

            /// 0b0: Single voltage reference input supported.
            pub const MVI_0: u32 = 0b0;

            /// 0b1: Multiple voltage reference inputs supported.
            pub const MVI_1: u32 = 0b1;
        }
    }

    /// Channel Scale Width
    pub mod CSW {
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

            /// 0b000: Channel scaling not supported.
            pub const CSW_0: u32 = 0b000;

            /// 0b001: Channel scaling supported. 1-bit CSCALE control field.
            pub const CSW_1: u32 = 0b001;

            /// 0b110: Channel scaling supported. 6-bit CSCALE control field.
            pub const CSW_6: u32 = 0b110;
        }
    }

    /// Voltage Reference 1 Range Control Bit Implemented
    pub mod VR1RNGI {
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

            /// 0b0: Range control not required. CFG\[VREF1RNG\] is not implemented.
            pub const VR1RNGI_0: u32 = 0b0;

            /// 0b1: Range control required. CFG\[VREF1RNG\] is implemented.
            pub const VR1RNGI_1: u32 = 0b1;
        }
    }

    /// Internal LPADC Clock implemented
    pub mod IADCKI {
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

            /// 0b0: Internal clock source not implemented.
            pub const IADCKI_0: u32 = 0b0;

            /// 0b1: Internal clock source (and CFG\[ADCKEN\]) implemented.
            pub const IADCKI_1: u32 = 0b1;
        }
    }

    /// Calibration Offset Function Implemented
    pub mod CALOFSI {
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

            /// 0b0: Offset calibration and offset trimming not implemented.
            pub const CALOFSI_0: u32 = 0b0;

            /// 0b1: Offset calibration and offset trimming implemented.
            pub const CALOFSI_1: u32 = 0b1;
        }
    }

    /// Minor Version Number
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

    /// Major Version Number
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

    /// Trigger Number
    pub mod TRIG_NUM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00001000: 8 hardware triggers implemented
            pub const TRIG_NUM_8: u32 = 0b00001000;
        }
    }

    /// Result FIFO Depth
    pub mod FIFOSIZE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00010000: Result FIFO depth = 16 datawords.
            pub const FIFOSIZE_16: u32 = 0b00010000;
        }
    }

    /// Compare Value Number
    pub mod CV_NUM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000100: 4 compare value registers implemented
            pub const CV_NUM_4: u32 = 0b00000100;
        }
    }

    /// Command Buffer Number
    pub mod CMD_NUM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00001111: 15 command buffers implemented
            pub const CMD_NUM_15: u32 = 0b00001111;
        }
    }
}

/// LPADC Control Register
pub mod CTRL {

    /// LPADC Enable
    pub mod ADCEN {
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

            /// 0b0: LPADC is disabled.
            pub const ADCEN_0: u32 = 0b0;

            /// 0b1: LPADC is enabled.
            pub const ADCEN_1: u32 = 0b1;
        }
    }

    /// Software Reset
    pub mod RST {
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

            /// 0b0: LPADC logic is not reset.
            pub const RST_0: u32 = 0b0;

            /// 0b1: LPADC logic is reset.
            pub const RST_1: u32 = 0b1;
        }
    }

    /// Doze Enable
    pub mod DOZEN {
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

            /// 0b0: LPADC is enabled in Doze mode.
            pub const DOZEN_0: u32 = 0b0;

            /// 0b1: LPADC is disabled in Doze mode.
            pub const DOZEN_1: u32 = 0b1;
        }
    }

    /// Hardware trigger source selection
    pub mod TRIG_SRC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: ADC_ETC hw trigger , and HW trigger are enabled
            pub const TRIG_SRC_0: u32 = 0b00;

            /// 0b01: ADC_ETC hw trigger is enabled
            pub const TRIG_SRC_1: u32 = 0b01;

            /// 0b10: HW trigger is enabled
            pub const TRIG_SRC_2: u32 = 0b10;
        }
    }

    /// Reset FIFO
    pub mod RSTFIFO {
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

            /// 0b0: No effect.
            pub const RSTFIFO_0: u32 = 0b0;

            /// 0b1: FIFO is reset.
            pub const RSTFIFO_1: u32 = 0b1;
        }
    }
}

/// LPADC Status Register
pub mod STAT {

    /// Result FIFO Ready Flag
    pub mod RDY {
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

            /// 0b0: Result FIFO data level not above watermark level.
            pub const RDY_0: u32 = 0b0;

            /// 0b1: Result FIFO holding data above watermark level.
            pub const RDY_1: u32 = 0b1;
        }
    }

    /// Result FIFO Overflow Flag
    pub mod FOF {
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

            /// 0b0: No result FIFO overflow has occurred since the last time the flag was cleared.
            pub const FOF_0: u32 = 0b0;

            /// 0b1: At least one result FIFO overflow has occurred since the last time the flag was cleared.
            pub const FOF_1: u32 = 0b1;
        }
    }

    /// ADC Active
    pub mod ADC_ACTIVE {
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

            /// 0b0: The LPADC is IDLE. There are no pending triggers to service and no active commands are being processed.
            pub const ADC_ACTIVE_0: u32 = 0b0;

            /// 0b1: The LPADC is processing a conversion, running through the power up delay, or servicing a trigger.
            pub const ADC_ACTIVE_1: u32 = 0b1;
        }
    }

    /// Trigger Active
    pub mod TRGACT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Command (sequence) associated with Trigger 0 currently being executed.
            pub const TRGACT_0: u32 = 0b000;

            /// 0b001: Command (sequence) associated with Trigger 1 currently being executed.
            pub const TRGACT_1: u32 = 0b001;

            /// 0b010: Command (sequence) associated with Trigger 2 currently being executed.
            pub const TRGACT_2: u32 = 0b010;

            /// 0b011: Command (sequence) from the associated Trigger number is currently being executed.
            pub const TRGACT_3: u32 = 0b011;

            /// 0b100: Command (sequence) from the associated Trigger number is currently being executed.
            pub const TRGACT_4: u32 = 0b100;

            /// 0b101: Command (sequence) from the associated Trigger number is currently being executed.
            pub const TRGACT_5: u32 = 0b101;

            /// 0b110: Command (sequence) from the associated Trigger number is currently being executed.
            pub const TRGACT_6: u32 = 0b110;

            /// 0b111: Command (sequence) from the associated Trigger number is currently being executed.
            pub const TRGACT_7: u32 = 0b111;
        }
    }

    /// Command Active
    pub mod CMDACT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No command is currently in progress.
            pub const CMDACT_0: u32 = 0b0000;

            /// 0b0001: Command 1 currently being executed.
            pub const CMDACT_1: u32 = 0b0001;

            /// 0b0010: Command 2 currently being executed.
            pub const CMDACT_2: u32 = 0b0010;

            /// 0b0011: Associated command number is currently being executed.
            pub const CMDACT_3: u32 = 0b0011;

            /// 0b0100: Associated command number is currently being executed.
            pub const CMDACT_4: u32 = 0b0100;

            /// 0b0101: Associated command number is currently being executed.
            pub const CMDACT_5: u32 = 0b0101;

            /// 0b0110: Associated command number is currently being executed.
            pub const CMDACT_6: u32 = 0b0110;

            /// 0b0111: Associated command number is currently being executed.
            pub const CMDACT_7: u32 = 0b0111;

            /// 0b1000: Associated command number is currently being executed.
            pub const CMDACT_8: u32 = 0b1000;

            /// 0b1001: Associated command number is currently being executed.
            pub const CMDACT_9: u32 = 0b1001;
        }
    }
}

/// Interrupt Enable Register
pub mod IE {

    /// FIFO Watermark Interrupt Enable
    pub mod FWMIE {
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

            /// 0b0: FIFO watermark interrupts are not enabled.
            pub const FWMIE_0: u32 = 0b0;

            /// 0b1: FIFO watermark interrupts are enabled.
            pub const FWMIE_1: u32 = 0b1;
        }
    }

    /// Result FIFO Overflow Interrupt Enable
    pub mod FOFIE {
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

            /// 0b0: FIFO overflow interrupts are not enabled.
            pub const FOFIE_0: u32 = 0b0;

            /// 0b1: FIFO overflow interrupts are enabled.
            pub const FOFIE_1: u32 = 0b1;
        }
    }
}

/// DMA Enable Register
pub mod DE {

    /// FIFO Watermark DMA Enable
    pub mod FWMDE {
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

            /// 0b0: DMA request disabled.
            pub const FWMDE_0: u32 = 0b0;

            /// 0b1: DMA request enabled.
            pub const FWMDE_1: u32 = 0b1;
        }
    }
}

/// LPADC Configuration Register
pub mod CFG {

    /// LPADC trigger priority control
    pub mod TPRICTRL {
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

            /// 0b0: If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started.
            pub const TPRICTRL_0: u32 = 0b0;

            /// 0b1: If a higher priority trigger is received during command processing, the current conversion is completed (including averaging iterations if enabled) and stored to the RESFIFO before the higher priority trigger/command is initiated. Note that compare until true commands can be interrupted prior to resulting in a true conversion.
            pub const TPRICTRL_1: u32 = 0b1;
        }
    }

    /// Power Configuration Select
    pub mod PWRSEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Level 1 (Lowest power setting)
            pub const PWRSEL_0: u32 = 0b00;

            /// 0b01: Level 2
            pub const PWRSEL_1: u32 = 0b01;

            /// 0b10: Level 3
            pub const PWRSEL_2: u32 = 0b10;

            /// 0b11: Level 4 (Highest power setting)
            pub const PWRSEL_3: u32 = 0b11;
        }
    }

    /// Voltage Reference Selection
    pub mod REFSEL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: (Default) Option 1 setting.
            pub const REFSEL_0: u32 = 0b00;

            /// 0b01: Option 2 setting.
            pub const REFSEL_1: u32 = 0b01;

            /// 0b10: Option 3 setting.
            pub const REFSEL_2: u32 = 0b10;
        }
    }

    /// Power Up Delay
    pub mod PUDLY {
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

    /// LPADC Analog Pre-Enable
    pub mod PWREN {
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

            /// 0b0: LPADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays.
            pub const PWREN_0: u32 = 0b0;

            /// 0b1: LPADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). When PWREN is set, the power up delay is enforced such that any detected trigger does not begin ADC operation until the power up delay time has passed.
            pub const PWREN_1: u32 = 0b1;
        }
    }
}

/// LPADC Pause Register
pub mod PAUSE {

    /// Pause Delay
    pub mod PAUSEDLY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PAUSE Option Enable
    pub mod PAUSEEN {
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

            /// 0b0: Pause operation disabled
            pub const PAUSEEN_0: u32 = 0b0;

            /// 0b1: Pause operation enabled
            pub const PAUSEEN_1: u32 = 0b1;
        }
    }
}

/// LPADC FIFO Control Register
pub mod FCTRL {

    /// Result FIFO counter
    pub mod FCOUNT {
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

            /// 0b00000: No data stored in FIFO
            pub const FCOUNT_0: u32 = 0b00000;

            /// 0b00001: 1 dataword stored in FIFO
            pub const FCOUNT_1: u32 = 0b00001;

            /// 0b00010: 2 datawords stored in FIFO
            pub const FCOUNT_2: u32 = 0b00010;

            /// 0b00100: 4 datawords stored in FIFO
            pub const FCOUNT_4: u32 = 0b00100;

            /// 0b01000: 8 datawords stored in FIFO
            pub const FCOUNT_8: u32 = 0b01000;

            /// 0b10000: 16 datawords stored in FIFO
            pub const FCOUNT_16: u32 = 0b10000;
        }
    }

    /// Watermark level selection
    pub mod FWMARK {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Generates STAT\[RDY\] flag after 1st successful conversion - single conversion
            pub const FWMARK_0: u32 = 0b0000;

            /// 0b0001: Generates STAT\[RDY\] flag after 2nd successful conversion
            pub const FWMARK_1: u32 = 0b0001;

            /// 0b0010: Generates STAT\[RDY\] flag after 3rd successful conversion
            pub const FWMARK_2: u32 = 0b0010;

            /// 0b0011: Generates STAT\[RDY\] flag after 4th successful conversion
            pub const FWMARK_3: u32 = 0b0011;

            /// 0b0100: Generates STAT\[RDY\] flag after 5th successful conversion
            pub const FWMARK_4: u32 = 0b0100;

            /// 0b0101: Generates STAT\[RDY\] flag after 6th successful conversion
            pub const FWMARK_5: u32 = 0b0101;

            /// 0b0110: Generates STAT\[RDY\] flag after 7th successful conversion
            pub const FWMARK_6: u32 = 0b0110;

            /// 0b0111: Generates STAT\[RDY\] flag after 8th successful conversion
            pub const FWMARK_7: u32 = 0b0111;

            /// 0b1000: Generates STAT\[RDY\] flag after 9th successful conversion
            pub const FWMARK_8: u32 = 0b1000;

            /// 0b1001: Generates STAT\[RDY\] flag after 10th successful conversion
            pub const FWMARK_9: u32 = 0b1001;

            /// 0b1010: Generates STAT\[RDY\] flag after 11th successful conversion
            pub const FWMARK_10: u32 = 0b1010;

            /// 0b1011: Generates STAT\[RDY\] flag after 12th successful conversion
            pub const FWMARK_11: u32 = 0b1011;

            /// 0b1100: Generates STAT\[RDY\] flag after 13th successful conversion
            pub const FWMARK_12: u32 = 0b1100;

            /// 0b1101: Generates STAT\[RDY\] flag after 14th successful conversion
            pub const FWMARK_13: u32 = 0b1101;

            /// 0b1110: Generates STAT\[RDY\] flag after 15th successful conversion
            pub const FWMARK_14: u32 = 0b1110;

            /// 0b1111: Generates STAT\[RDY\] flag after 16th successful conversion
            pub const FWMARK_15: u32 = 0b1111;
        }
    }
}

/// Software Trigger Register
pub mod SWTRIG {

    /// Software trigger 0 event
    pub mod SWT0 {
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

            /// 0b0: No trigger 0 event generated.
            pub const SWT0_0: u32 = 0b0;

            /// 0b1: Trigger 0 event generated.
            pub const SWT0_1: u32 = 0b1;
        }
    }

    /// Software trigger 1 event
    pub mod SWT1 {
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

            /// 0b0: No trigger 1 event generated.
            pub const SWT1_0: u32 = 0b0;

            /// 0b1: Trigger 1 event generated.
            pub const SWT1_1: u32 = 0b1;
        }
    }

    /// Software trigger 2 event
    pub mod SWT2 {
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

            /// 0b0: No trigger 2 event generated.
            pub const SWT2_0: u32 = 0b0;

            /// 0b1: Trigger 2 event generated.
            pub const SWT2_1: u32 = 0b1;
        }
    }

    /// Software trigger 3 event
    pub mod SWT3 {
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

            /// 0b0: No trigger 3 event generated.
            pub const SWT3_0: u32 = 0b0;

            /// 0b1: Trigger 3 event generated.
            pub const SWT3_1: u32 = 0b1;
        }
    }

    /// Software trigger 4 event
    pub mod SWT4 {
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

            /// 0b0: No trigger 4 event generated.
            pub const SWT4_0: u32 = 0b0;

            /// 0b1: Trigger 4 event generated.
            pub const SWT4_1: u32 = 0b1;
        }
    }

    /// Software trigger 5 event
    pub mod SWT5 {
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

            /// 0b0: No trigger 5 event generated.
            pub const SWT5_0: u32 = 0b0;

            /// 0b1: Trigger 5 event generated.
            pub const SWT5_1: u32 = 0b1;
        }
    }

    /// Software trigger 6 event
    pub mod SWT6 {
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

            /// 0b0: No trigger 6 event generated.
            pub const SWT6_0: u32 = 0b0;

            /// 0b1: Trigger 6 event generated.
            pub const SWT6_1: u32 = 0b1;
        }
    }

    /// Software trigger 7 event
    pub mod SWT7 {
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

            /// 0b0: No trigger 7 event generated.
            pub const SWT7_0: u32 = 0b0;

            /// 0b1: Trigger 7 event generated.
            pub const SWT7_1: u32 = 0b1;
        }
    }
}

/// Trigger Control Register
pub mod TCTRL0 {

    /// Trigger enable
    pub mod HTEN {
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

            /// 0b0: Hardware trigger source disabled
            pub const HTEN_0: u32 = 0b0;

            /// 0b1: Hardware trigger source enabled
            pub const HTEN_1: u32 = 0b1;
        }
    }

    /// The command number is selected by software TCMD or hardware tcmd signal
    pub mod CMD_SEL {
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

            /// 0b0: TCTRLa\[TCMD\] will determine the command
            pub const CMD_SEL_0: u32 = 0b0;

            /// 0b1: Software TCDM is bypassed , and hardware TCMD from ADC_ETC module will be used. The trigger command is then defined by ADC hardware trigger command selection field in ADC_ETC->TRIGx_CHAINy_z_n\[CSEL\].
            pub const CMD_SEL_1: u32 = 0b1;
        }
    }

    /// Trigger priority setting
    pub mod TPRI {
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

            /// 0b000: Set to highest priority, Level 1
            pub const TPRI_0: u32 = 0b000;

            /// 0b001: Set to corresponding priority level
            pub const TPRI_1: u32 = 0b001;

            /// 0b010: Set to corresponding priority level
            pub const TPRI_2: u32 = 0b010;

            /// 0b011: Set to corresponding priority level
            pub const TPRI_3: u32 = 0b011;

            /// 0b100: Set to corresponding priority level
            pub const TPRI_4: u32 = 0b100;

            /// 0b101: Set to corresponding priority level
            pub const TPRI_5: u32 = 0b101;

            /// 0b110: Set to corresponding priority level
            pub const TPRI_6: u32 = 0b110;

            /// 0b111: Set to lowest priority, Level 8
            pub const TPRI_7: u32 = 0b111;
        }
    }

    /// Trigger delay select
    pub mod TDLY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger command select
    pub mod TCMD {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not a valid selection from the command buffer. Trigger event is ignored.
            pub const TCMD_0: u32 = 0b0000;

            /// 0b0001: CMD1 is executed
            pub const TCMD_1: u32 = 0b0001;

            /// 0b0010: Corresponding CMD is executed
            pub const TCMD_2: u32 = 0b0010;

            /// 0b0011: Corresponding CMD is executed
            pub const TCMD_3: u32 = 0b0011;

            /// 0b0100: Corresponding CMD is executed
            pub const TCMD_4: u32 = 0b0100;

            /// 0b0101: Corresponding CMD is executed
            pub const TCMD_5: u32 = 0b0101;

            /// 0b0110: Corresponding CMD is executed
            pub const TCMD_6: u32 = 0b0110;

            /// 0b0111: Corresponding CMD is executed
            pub const TCMD_7: u32 = 0b0111;

            /// 0b1000: Corresponding CMD is executed
            pub const TCMD_8: u32 = 0b1000;

            /// 0b1001: Corresponding CMD is executed
            pub const TCMD_9: u32 = 0b1001;

            /// 0b1111: CMD15 is executed
            pub const TCMD_15: u32 = 0b1111;
        }
    }
}

/// Trigger Control Register
pub mod TCTRL1 {
    pub use super::TCTRL0::CMD_SEL;
    pub use super::TCTRL0::HTEN;
    pub use super::TCTRL0::TCMD;
    pub use super::TCTRL0::TDLY;
    pub use super::TCTRL0::TPRI;
}

/// Trigger Control Register
pub mod TCTRL2 {
    pub use super::TCTRL0::CMD_SEL;
    pub use super::TCTRL0::HTEN;
    pub use super::TCTRL0::TCMD;
    pub use super::TCTRL0::TDLY;
    pub use super::TCTRL0::TPRI;
}

/// Trigger Control Register
pub mod TCTRL3 {
    pub use super::TCTRL0::CMD_SEL;
    pub use super::TCTRL0::HTEN;
    pub use super::TCTRL0::TCMD;
    pub use super::TCTRL0::TDLY;
    pub use super::TCTRL0::TPRI;
}

/// Trigger Control Register
pub mod TCTRL4 {
    pub use super::TCTRL0::CMD_SEL;
    pub use super::TCTRL0::HTEN;
    pub use super::TCTRL0::TCMD;
    pub use super::TCTRL0::TDLY;
    pub use super::TCTRL0::TPRI;
}

/// Trigger Control Register
pub mod TCTRL5 {
    pub use super::TCTRL0::CMD_SEL;
    pub use super::TCTRL0::HTEN;
    pub use super::TCTRL0::TCMD;
    pub use super::TCTRL0::TDLY;
    pub use super::TCTRL0::TPRI;
}

/// Trigger Control Register
pub mod TCTRL6 {
    pub use super::TCTRL0::CMD_SEL;
    pub use super::TCTRL0::HTEN;
    pub use super::TCTRL0::TCMD;
    pub use super::TCTRL0::TDLY;
    pub use super::TCTRL0::TPRI;
}

/// Trigger Control Register
pub mod TCTRL7 {
    pub use super::TCTRL0::CMD_SEL;
    pub use super::TCTRL0::HTEN;
    pub use super::TCTRL0::TCMD;
    pub use super::TCTRL0::TDLY;
    pub use super::TCTRL0::TPRI;
}

/// LPADC Command Low Buffer Register
pub mod CMDL1 {

    /// Input channel select
    pub mod ADCH {
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

            /// 0b00000: Select CH0A or CH0B or CH0A/CH0B pair.
            pub const ADCH_0: u32 = 0b00000;

            /// 0b00001: Select CH1A or CH1B or CH1A/CH1B pair.
            pub const ADCH_1: u32 = 0b00001;

            /// 0b00010: Select CH2A or CH2B or CH2A/CH2B pair.
            pub const ADCH_2: u32 = 0b00010;

            /// 0b00011: Select CH3A or CH3B or CH3A/CH3B pair.
            pub const ADCH_3: u32 = 0b00011;

            /// 0b00100: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair.
            pub const ADCH_4: u32 = 0b00100;

            /// 0b00101: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair.
            pub const ADCH_5: u32 = 0b00101;

            /// 0b00110: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair.
            pub const ADCH_6: u32 = 0b00110;

            /// 0b00111: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair.
            pub const ADCH_7: u32 = 0b00111;

            /// 0b01000: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair.
            pub const ADCH_8: u32 = 0b01000;

            /// 0b01001: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair.
            pub const ADCH_9: u32 = 0b01001;

            /// 0b11110: Select CH30A or CH30B or CH30A/CH30B pair.
            pub const ADCH_30: u32 = 0b11110;

            /// 0b11111: Select CH31A or CH31B or CH31A/CH31B pair.
            pub const ADCH_31: u32 = 0b11111;
        }
    }

    /// A-side vs. B-side Select
    pub mod ABSEL {
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

            /// 0b0: When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB).
            pub const ABSEL_0: u32 = 0b0;

            /// 0b1: When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA).
            pub const ABSEL_1: u32 = 0b1;
        }
    }

    /// Differential Mode Enable
    pub mod DIFF {
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

            /// 0b0: Single-ended mode.
            pub const DIFF_0: u32 = 0b0;

            /// 0b1: Differential mode.
            pub const DIFF_1: u32 = 0b1;
        }
    }

    /// Channel Scale
    pub mod CSCALE {
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

            /// 0b0: Scale selected analog channel (Factor of 30/64)
            pub const CSCALE_0: u32 = 0b0;

            /// 0b1: (Default) Full scale (Factor of 1)
            pub const CSCALE_1: u32 = 0b1;
        }
    }
}

/// LPADC Command High Buffer Register
pub mod CMDH1 {

    /// Compare Function Enable
    pub mod CMPEN {
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

            /// 0b00: Compare disabled.
            pub const CMPEN_0: u32 = 0b00;

            /// 0b10: Compare enabled. Store on true.
            pub const CMPEN_2: u32 = 0b10;

            /// 0b11: Compare enabled. Repeat channel acquisition (sample/convert/compare) until true.
            pub const CMPEN_3: u32 = 0b11;
        }
    }

    /// Loop with Increment
    pub mod LWI {
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

            /// 0b0: Auto channel increment disabled
            pub const LWI_0: u32 = 0b0;

            /// 0b1: Auto channel increment enabled
            pub const LWI_1: u32 = 0b1;
        }
    }

    /// Sample Time Select
    pub mod STS {
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

            /// 0b000: Minimum sample time of 3 ADCK cycles.
            pub const STS_0: u32 = 0b000;

            /// 0b001: 3 + 21 ADCK cycles; 5 ADCK cycles total sample time.
            pub const STS_1: u32 = 0b001;

            /// 0b010: 3 + 22 ADCK cycles; 7 ADCK cycles total sample time.
            pub const STS_2: u32 = 0b010;

            /// 0b011: 3 + 23 ADCK cycles; 11 ADCK cycles total sample time.
            pub const STS_3: u32 = 0b011;

            /// 0b100: 3 + 24 ADCK cycles; 19 ADCK cycles total sample time.
            pub const STS_4: u32 = 0b100;

            /// 0b101: 3 + 25 ADCK cycles; 35 ADCK cycles total sample time.
            pub const STS_5: u32 = 0b101;

            /// 0b110: 3 + 26 ADCK cycles; 67 ADCK cycles total sample time.
            pub const STS_6: u32 = 0b110;

            /// 0b111: 3 + 27 ADCK cycles; 131 ADCK cycles total sample time.
            pub const STS_7: u32 = 0b111;
        }
    }

    /// Hardware Average Select
    pub mod AVGS {
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

            /// 0b000: Single conversion.
            pub const AVGS_0: u32 = 0b000;

            /// 0b001: 2 conversions averaged.
            pub const AVGS_1: u32 = 0b001;

            /// 0b010: 4 conversions averaged.
            pub const AVGS_2: u32 = 0b010;

            /// 0b011: 8 conversions averaged.
            pub const AVGS_3: u32 = 0b011;

            /// 0b100: 16 conversions averaged.
            pub const AVGS_4: u32 = 0b100;

            /// 0b101: 32 conversions averaged.
            pub const AVGS_5: u32 = 0b101;

            /// 0b110: 64 conversions averaged.
            pub const AVGS_6: u32 = 0b110;

            /// 0b111: 128 conversions averaged.
            pub const AVGS_7: u32 = 0b111;
        }
    }

    /// Loop Count Select
    pub mod LOOP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Looping not enabled. Command executes 1 time.
            pub const LOOP_0: u32 = 0b0000;

            /// 0b0001: Loop 1 time. Command executes 2 times.
            pub const LOOP_1: u32 = 0b0001;

            /// 0b0010: Loop 2 times. Command executes 3 times.
            pub const LOOP_2: u32 = 0b0010;

            /// 0b0011: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_3: u32 = 0b0011;

            /// 0b0100: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_4: u32 = 0b0100;

            /// 0b0101: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_5: u32 = 0b0101;

            /// 0b0110: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_6: u32 = 0b0110;

            /// 0b0111: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_7: u32 = 0b0111;

            /// 0b1000: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_8: u32 = 0b1000;

            /// 0b1001: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_9: u32 = 0b1001;

            /// 0b1111: Loop 15 times. Command executes 16 times.
            pub const LOOP_15: u32 = 0b1111;
        }
    }

    /// Next Command Select
    pub mod NEXT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger.
            pub const NEXT_0: u32 = 0b0000;

            /// 0b0001: Select CMD1 command buffer register as next command.
            pub const NEXT_1: u32 = 0b0001;

            /// 0b0010: Select corresponding CMD command buffer register as next command
            pub const NEXT_2: u32 = 0b0010;

            /// 0b0011: Select corresponding CMD command buffer register as next command
            pub const NEXT_3: u32 = 0b0011;

            /// 0b0100: Select corresponding CMD command buffer register as next command
            pub const NEXT_4: u32 = 0b0100;

            /// 0b0101: Select corresponding CMD command buffer register as next command
            pub const NEXT_5: u32 = 0b0101;

            /// 0b0110: Select corresponding CMD command buffer register as next command
            pub const NEXT_6: u32 = 0b0110;

            /// 0b0111: Select corresponding CMD command buffer register as next command
            pub const NEXT_7: u32 = 0b0111;

            /// 0b1000: Select corresponding CMD command buffer register as next command
            pub const NEXT_8: u32 = 0b1000;

            /// 0b1001: Select corresponding CMD command buffer register as next command
            pub const NEXT_9: u32 = 0b1001;

            /// 0b1111: Select CMD15 command buffer register as next command.
            pub const NEXT_15: u32 = 0b1111;
        }
    }
}

/// LPADC Command Low Buffer Register
pub mod CMDL2 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH2 {
    pub use super::CMDH1::AVGS;
    pub use super::CMDH1::CMPEN;
    pub use super::CMDH1::LOOP;
    pub use super::CMDH1::LWI;
    pub use super::CMDH1::NEXT;
    pub use super::CMDH1::STS;
}

/// LPADC Command Low Buffer Register
pub mod CMDL3 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH3 {
    pub use super::CMDH1::AVGS;
    pub use super::CMDH1::CMPEN;
    pub use super::CMDH1::LOOP;
    pub use super::CMDH1::LWI;
    pub use super::CMDH1::NEXT;
    pub use super::CMDH1::STS;
}

/// LPADC Command Low Buffer Register
pub mod CMDL4 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH4 {
    pub use super::CMDH1::AVGS;
    pub use super::CMDH1::CMPEN;
    pub use super::CMDH1::LOOP;
    pub use super::CMDH1::LWI;
    pub use super::CMDH1::NEXT;
    pub use super::CMDH1::STS;
}

/// LPADC Command Low Buffer Register
pub mod CMDL5 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH5 {

    /// Loop with Increment
    pub mod LWI {
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

            /// 0b0: Auto channel increment disabled
            pub const LWI_0: u32 = 0b0;

            /// 0b1: Auto channel increment enabled
            pub const LWI_1: u32 = 0b1;
        }
    }

    /// Sample Time Select
    pub mod STS {
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

            /// 0b000: Minimum sample time of 3 ADCK cycles.
            pub const STS_0: u32 = 0b000;

            /// 0b001: 3 + 21 ADCK cycles; 5 ADCK cycles total sample time.
            pub const STS_1: u32 = 0b001;

            /// 0b010: 3 + 22 ADCK cycles; 7 ADCK cycles total sample time.
            pub const STS_2: u32 = 0b010;

            /// 0b011: 3 + 23 ADCK cycles; 11 ADCK cycles total sample time.
            pub const STS_3: u32 = 0b011;

            /// 0b100: 3 + 24 ADCK cycles; 19 ADCK cycles total sample time.
            pub const STS_4: u32 = 0b100;

            /// 0b101: 3 + 25 ADCK cycles; 35 ADCK cycles total sample time.
            pub const STS_5: u32 = 0b101;

            /// 0b110: 3 + 26 ADCK cycles; 67 ADCK cycles total sample time.
            pub const STS_6: u32 = 0b110;

            /// 0b111: 3 + 27 ADCK cycles; 131 ADCK cycles total sample time.
            pub const STS_7: u32 = 0b111;
        }
    }

    /// Hardware Average Select
    pub mod AVGS {
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

            /// 0b000: Single conversion.
            pub const AVGS_0: u32 = 0b000;

            /// 0b001: 2 conversions averaged.
            pub const AVGS_1: u32 = 0b001;

            /// 0b010: 4 conversions averaged.
            pub const AVGS_2: u32 = 0b010;

            /// 0b011: 8 conversions averaged.
            pub const AVGS_3: u32 = 0b011;

            /// 0b100: 16 conversions averaged.
            pub const AVGS_4: u32 = 0b100;

            /// 0b101: 32 conversions averaged.
            pub const AVGS_5: u32 = 0b101;

            /// 0b110: 64 conversions averaged.
            pub const AVGS_6: u32 = 0b110;

            /// 0b111: 128 conversions averaged.
            pub const AVGS_7: u32 = 0b111;
        }
    }

    /// Loop Count Select
    pub mod LOOP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Looping not enabled. Command executes 1 time.
            pub const LOOP_0: u32 = 0b0000;

            /// 0b0001: Loop 1 time. Command executes 2 times.
            pub const LOOP_1: u32 = 0b0001;

            /// 0b0010: Loop 2 times. Command executes 3 times.
            pub const LOOP_2: u32 = 0b0010;

            /// 0b0011: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_3: u32 = 0b0011;

            /// 0b0100: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_4: u32 = 0b0100;

            /// 0b0101: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_5: u32 = 0b0101;

            /// 0b0110: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_6: u32 = 0b0110;

            /// 0b0111: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_7: u32 = 0b0111;

            /// 0b1000: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_8: u32 = 0b1000;

            /// 0b1001: Loop corresponding number of times. Command executes LOOP+1 times.
            pub const LOOP_9: u32 = 0b1001;

            /// 0b1111: Loop 15 times. Command executes 16 times.
            pub const LOOP_15: u32 = 0b1111;
        }
    }

    /// Next Command Select
    pub mod NEXT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger.
            pub const NEXT_0: u32 = 0b0000;

            /// 0b0001: Select CMD1 command buffer register as next command.
            pub const NEXT_1: u32 = 0b0001;

            /// 0b0010: Select corresponding CMD command buffer register as next command
            pub const NEXT_2: u32 = 0b0010;

            /// 0b0011: Select corresponding CMD command buffer register as next command
            pub const NEXT_3: u32 = 0b0011;

            /// 0b0100: Select corresponding CMD command buffer register as next command
            pub const NEXT_4: u32 = 0b0100;

            /// 0b0101: Select corresponding CMD command buffer register as next command
            pub const NEXT_5: u32 = 0b0101;

            /// 0b0110: Select corresponding CMD command buffer register as next command
            pub const NEXT_6: u32 = 0b0110;

            /// 0b0111: Select corresponding CMD command buffer register as next command
            pub const NEXT_7: u32 = 0b0111;

            /// 0b1000: Select corresponding CMD command buffer register as next command
            pub const NEXT_8: u32 = 0b1000;

            /// 0b1001: Select corresponding CMD command buffer register as next command
            pub const NEXT_9: u32 = 0b1001;

            /// 0b1111: Select CMD15 command buffer register as next command.
            pub const NEXT_15: u32 = 0b1111;
        }
    }
}

/// LPADC Command Low Buffer Register
pub mod CMDL6 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH6 {
    pub use super::CMDH5::AVGS;
    pub use super::CMDH5::LOOP;
    pub use super::CMDH5::LWI;
    pub use super::CMDH5::NEXT;
    pub use super::CMDH5::STS;
}

/// LPADC Command Low Buffer Register
pub mod CMDL7 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH7 {
    pub use super::CMDH5::AVGS;
    pub use super::CMDH5::LOOP;
    pub use super::CMDH5::LWI;
    pub use super::CMDH5::NEXT;
    pub use super::CMDH5::STS;
}

/// LPADC Command Low Buffer Register
pub mod CMDL8 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH8 {
    pub use super::CMDH5::AVGS;
    pub use super::CMDH5::LOOP;
    pub use super::CMDH5::LWI;
    pub use super::CMDH5::NEXT;
    pub use super::CMDH5::STS;
}

/// LPADC Command Low Buffer Register
pub mod CMDL9 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH9 {
    pub use super::CMDH5::AVGS;
    pub use super::CMDH5::LOOP;
    pub use super::CMDH5::LWI;
    pub use super::CMDH5::NEXT;
    pub use super::CMDH5::STS;
}

/// LPADC Command Low Buffer Register
pub mod CMDL10 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH10 {
    pub use super::CMDH5::AVGS;
    pub use super::CMDH5::LOOP;
    pub use super::CMDH5::LWI;
    pub use super::CMDH5::NEXT;
    pub use super::CMDH5::STS;
}

/// LPADC Command Low Buffer Register
pub mod CMDL11 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH11 {
    pub use super::CMDH5::AVGS;
    pub use super::CMDH5::LOOP;
    pub use super::CMDH5::LWI;
    pub use super::CMDH5::NEXT;
    pub use super::CMDH5::STS;
}

/// LPADC Command Low Buffer Register
pub mod CMDL12 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH12 {
    pub use super::CMDH5::AVGS;
    pub use super::CMDH5::LOOP;
    pub use super::CMDH5::LWI;
    pub use super::CMDH5::NEXT;
    pub use super::CMDH5::STS;
}

/// LPADC Command Low Buffer Register
pub mod CMDL13 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH13 {
    pub use super::CMDH5::AVGS;
    pub use super::CMDH5::LOOP;
    pub use super::CMDH5::LWI;
    pub use super::CMDH5::NEXT;
    pub use super::CMDH5::STS;
}

/// LPADC Command Low Buffer Register
pub mod CMDL14 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH14 {
    pub use super::CMDH5::AVGS;
    pub use super::CMDH5::LOOP;
    pub use super::CMDH5::LWI;
    pub use super::CMDH5::NEXT;
    pub use super::CMDH5::STS;
}

/// LPADC Command Low Buffer Register
pub mod CMDL15 {
    pub use super::CMDL1::ABSEL;
    pub use super::CMDL1::ADCH;
    pub use super::CMDL1::CSCALE;
    pub use super::CMDL1::DIFF;
}

/// LPADC Command High Buffer Register
pub mod CMDH15 {
    pub use super::CMDH5::AVGS;
    pub use super::CMDH5::LOOP;
    pub use super::CMDH5::LWI;
    pub use super::CMDH5::NEXT;
    pub use super::CMDH5::STS;
}

/// Compare Value Register
pub mod CV1 {

    /// Compare Value Low
    pub mod CVL {
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

    /// Compare Value High.
    pub mod CVH {
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

/// Compare Value Register
pub mod CV2 {
    pub use super::CV1::CVH;
    pub use super::CV1::CVL;
}

/// Compare Value Register
pub mod CV3 {
    pub use super::CV1::CVH;
    pub use super::CV1::CVL;
}

/// Compare Value Register
pub mod CV4 {
    pub use super::CV1::CVH;
    pub use super::CV1::CVL;
}

/// LPADC Data Result FIFO Register
pub mod RESFIFO {

    /// Data result
    pub mod D {
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

    /// Trigger Source
    pub mod TSRC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Trigger source 0 initiated this conversion.
            pub const TSRC_0: u32 = 0b000;

            /// 0b001: Trigger source 1 initiated this conversion.
            pub const TSRC_1: u32 = 0b001;

            /// 0b010: Corresponding trigger source initiated this conversion.
            pub const TSRC_2: u32 = 0b010;

            /// 0b011: Corresponding trigger source initiated this conversion.
            pub const TSRC_3: u32 = 0b011;

            /// 0b100: Corresponding trigger source initiated this conversion.
            pub const TSRC_4: u32 = 0b100;

            /// 0b101: Corresponding trigger source initiated this conversion.
            pub const TSRC_5: u32 = 0b101;

            /// 0b110: Corresponding trigger source initiated this conversion.
            pub const TSRC_6: u32 = 0b110;

            /// 0b111: Trigger source 7 initiated this conversion.
            pub const TSRC_7: u32 = 0b111;
        }
    }

    /// Loop count value
    pub mod LOOPCNT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Result is from initial conversion in command.
            pub const LOOPCNT_0: u32 = 0b0000;

            /// 0b0001: Result is from second conversion in command.
            pub const LOOPCNT_1: u32 = 0b0001;

            /// 0b0010: Result is from LOOPCNT+1 conversion in command.
            pub const LOOPCNT_2: u32 = 0b0010;

            /// 0b0011: Result is from LOOPCNT+1 conversion in command.
            pub const LOOPCNT_3: u32 = 0b0011;

            /// 0b0100: Result is from LOOPCNT+1 conversion in command.
            pub const LOOPCNT_4: u32 = 0b0100;

            /// 0b0101: Result is from LOOPCNT+1 conversion in command.
            pub const LOOPCNT_5: u32 = 0b0101;

            /// 0b0110: Result is from LOOPCNT+1 conversion in command.
            pub const LOOPCNT_6: u32 = 0b0110;

            /// 0b0111: Result is from LOOPCNT+1 conversion in command.
            pub const LOOPCNT_7: u32 = 0b0111;

            /// 0b1000: Result is from LOOPCNT+1 conversion in command.
            pub const LOOPCNT_8: u32 = 0b1000;

            /// 0b1001: Result is from LOOPCNT+1 conversion in command.
            pub const LOOPCNT_9: u32 = 0b1001;

            /// 0b1111: Result is from 16th conversion in command.
            pub const LOOPCNT_15: u32 = 0b1111;
        }
    }

    /// Command Buffer Source
    pub mod CMDSRC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not a valid value CMDSRC value for a dataword in RESFIFO. 0x0 is only found in initial FIFO state prior to an ADC conversion result dataword being stored to a RESFIFO buffer.
            pub const CMDSRC_0: u32 = 0b0000;

            /// 0b0001: CMD1 buffer used as control settings for this conversion.
            pub const CMDSRC_1: u32 = 0b0001;

            /// 0b0010: Corresponding command buffer used as control settings for this conversion.
            pub const CMDSRC_2: u32 = 0b0010;

            /// 0b0011: Corresponding command buffer used as control settings for this conversion.
            pub const CMDSRC_3: u32 = 0b0011;

            /// 0b0100: Corresponding command buffer used as control settings for this conversion.
            pub const CMDSRC_4: u32 = 0b0100;

            /// 0b0101: Corresponding command buffer used as control settings for this conversion.
            pub const CMDSRC_5: u32 = 0b0101;

            /// 0b0110: Corresponding command buffer used as control settings for this conversion.
            pub const CMDSRC_6: u32 = 0b0110;

            /// 0b0111: Corresponding command buffer used as control settings for this conversion.
            pub const CMDSRC_7: u32 = 0b0111;

            /// 0b1000: Corresponding command buffer used as control settings for this conversion.
            pub const CMDSRC_8: u32 = 0b1000;

            /// 0b1001: Corresponding command buffer used as control settings for this conversion.
            pub const CMDSRC_9: u32 = 0b1001;

            /// 0b1111: CMD15 buffer used as control settings for this conversion.
            pub const CMDSRC_15: u32 = 0b1111;
        }
    }

    /// FIFO entry is valid
    pub mod VALID {
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

            /// 0b0: FIFO is empty. Discard any read from RESFIFO.
            pub const VALID_0: u32 = 0b0;

            /// 0b1: FIFO record read from RESFIFO is valid.
            pub const VALID_1: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Version ID Register
    pub VERID: RORegister<u32>,

    /// Parameter Register
    pub PARAM: RORegister<u32>,

    _reserved1: [u32; 2],

    /// LPADC Control Register
    pub CTRL: RWRegister<u32>,

    /// LPADC Status Register
    pub STAT: RWRegister<u32>,

    /// Interrupt Enable Register
    pub IE: RWRegister<u32>,

    /// DMA Enable Register
    pub DE: RWRegister<u32>,

    /// LPADC Configuration Register
    pub CFG: RWRegister<u32>,

    /// LPADC Pause Register
    pub PAUSE: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// LPADC FIFO Control Register
    pub FCTRL: RWRegister<u32>,

    /// Software Trigger Register
    pub SWTRIG: RWRegister<u32>,

    _reserved3: [u32; 34],

    /// Trigger Control Register
    pub TCTRL0: RWRegister<u32>,

    /// Trigger Control Register
    pub TCTRL1: RWRegister<u32>,

    /// Trigger Control Register
    pub TCTRL2: RWRegister<u32>,

    /// Trigger Control Register
    pub TCTRL3: RWRegister<u32>,

    /// Trigger Control Register
    pub TCTRL4: RWRegister<u32>,

    /// Trigger Control Register
    pub TCTRL5: RWRegister<u32>,

    /// Trigger Control Register
    pub TCTRL6: RWRegister<u32>,

    /// Trigger Control Register
    pub TCTRL7: RWRegister<u32>,

    _reserved4: [u32; 8],

    /// LPADC Command Low Buffer Register
    pub CMDL1: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH1: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL2: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH2: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL3: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH3: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL4: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH4: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL5: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH5: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL6: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH6: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL7: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH7: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL8: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH8: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL9: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH9: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL10: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH10: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL11: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH11: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL12: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH12: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL13: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH13: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL14: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH14: RWRegister<u32>,

    /// LPADC Command Low Buffer Register
    pub CMDL15: RWRegister<u32>,

    /// LPADC Command High Buffer Register
    pub CMDH15: RWRegister<u32>,

    _reserved5: [u32; 34],

    /// Compare Value Register
    pub CV1: RWRegister<u32>,

    /// Compare Value Register
    pub CV2: RWRegister<u32>,

    /// Compare Value Register
    pub CV3: RWRegister<u32>,

    /// Compare Value Register
    pub CV4: RWRegister<u32>,

    _reserved6: [u32; 60],

    /// LPADC Data Result FIFO Register
    pub RESFIFO: RORegister<u32>,
}
pub struct ResetValues {
    pub VERID: u32,
    pub PARAM: u32,
    pub CTRL: u32,
    pub STAT: u32,
    pub IE: u32,
    pub DE: u32,
    pub CFG: u32,
    pub PAUSE: u32,
    pub FCTRL: u32,
    pub SWTRIG: u32,
    pub TCTRL0: u32,
    pub TCTRL1: u32,
    pub TCTRL2: u32,
    pub TCTRL3: u32,
    pub TCTRL4: u32,
    pub TCTRL5: u32,
    pub TCTRL6: u32,
    pub TCTRL7: u32,
    pub CMDL1: u32,
    pub CMDH1: u32,
    pub CMDL2: u32,
    pub CMDH2: u32,
    pub CMDL3: u32,
    pub CMDH3: u32,
    pub CMDL4: u32,
    pub CMDH4: u32,
    pub CMDL5: u32,
    pub CMDH5: u32,
    pub CMDL6: u32,
    pub CMDH6: u32,
    pub CMDL7: u32,
    pub CMDH7: u32,
    pub CMDL8: u32,
    pub CMDH8: u32,
    pub CMDL9: u32,
    pub CMDH9: u32,
    pub CMDL10: u32,
    pub CMDH10: u32,
    pub CMDL11: u32,
    pub CMDH11: u32,
    pub CMDL12: u32,
    pub CMDH12: u32,
    pub CMDL13: u32,
    pub CMDH13: u32,
    pub CMDL14: u32,
    pub CMDH14: u32,
    pub CMDL15: u32,
    pub CMDH15: u32,
    pub CV1: u32,
    pub CV2: u32,
    pub CV3: u32,
    pub CV4: u32,
    pub RESFIFO: u32,
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
