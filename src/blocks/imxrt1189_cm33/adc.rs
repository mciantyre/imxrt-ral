#[doc = "ADC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID Register"]
    pub VERID: u32,
    #[doc = "Parameter Register"]
    pub PARAM: u32,
    _reserved0: [u8; 0x08],
    #[doc = "Control Register"]
    pub CTRL: u32,
    #[doc = "Status Register"]
    pub STAT: u32,
    #[doc = "Interrupt Enable Register"]
    pub IE: u32,
    #[doc = "DMA Enable Register"]
    pub DE: u32,
    #[doc = "Configuration Register"]
    pub CFG: u32,
    #[doc = "Pause Register"]
    pub PAUSE: u32,
    _reserved1: [u8; 0x0c],
    #[doc = "Software Trigger Register"]
    pub SWTRIG: u32,
    #[doc = "Trigger Status Register"]
    pub TSTAT: u32,
    _reserved2: [u8; 0x04],
    #[doc = "Offset Trim 16 bit Register"]
    pub OFSTRIM16: u32,
    #[doc = "Offset Trim 12 bit Register"]
    pub OFSTRIM12: u32,
    _reserved3: [u8; 0x58],
    #[doc = "Trigger Control Register"]
    pub TCTRL: [u32; 8usize],
    _reserved4: [u8; 0x20],
    #[doc = "FIFO Control Register"]
    pub FCTRL: [u32; 2usize],
    _reserved5: [u8; 0x08],
    #[doc = "Gain Calibration Control"]
    pub GCC: [u32; 2usize],
    #[doc = "Gain Calculation Result"]
    pub GCR: [u32; 2usize],
    #[doc = "Command Low Buffer Register"]
    pub CMDL1: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH1: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL2: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH2: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL3: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH3: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL4: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH4: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL5: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH5: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL6: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH6: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL7: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH7: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL8: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH8: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL9: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH9: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL10: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH10: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL11: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH11: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL12: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH12: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL13: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH13: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL14: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH14: u32,
    #[doc = "Command Low Buffer Register"]
    pub CMDL15: u32,
    #[doc = "Command High Buffer Register"]
    pub CMDH15: u32,
    _reserved6: [u8; 0x88],
    #[doc = "Compare Value Register"]
    pub CV: [u32; 4usize],
    _reserved7: [u8; 0xf0],
    #[doc = "Data Result FIFO Register"]
    pub RESFIFO: [u32; 2usize],
    _reserved8: [u8; 0xf8],
    #[doc = "Calibration General A-Side Registers"]
    pub CAL_GAR: [u32; 33usize],
    _reserved9: [u8; 0x7c],
    #[doc = "Calibration General B-Side Registers"]
    pub CAL_GBR: [u32; 33usize],
    _reserved10: [u8; 0x0a74],
    #[doc = "Configuration 2 Register"]
    pub CFG2: u32,
}
#[doc = "Version ID Register"]
pub mod VERID {
    pub use crate::RO as access;
    #[doc = "Resolution"]
    pub mod RES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Up to 13-bit differential/12-bit single ended resolution supported."]
            pub const MAX_13_BIT: u32 = 0;
            #[doc = "Up to 16-bit differential/16-bit single ended resolution supported."]
            pub const MAX_16_BIT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Supported"]
    pub mod DIFFEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Differential operation not supported."]
            pub const DIFFERENTIAL_NOT_SUPPORTED: u32 = 0;
            #[doc = "Differential operation supported. CMDLa\\[CTYPE\\] controls fields implemented."]
            pub const DIFFERENTIAL_SUPPORTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Multi Vref Implemented"]
    pub mod MVI {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single voltage reference high (VREFH) input supported."]
            pub const MULTIPLE_REF_NOT_SUPPORTED: u32 = 0;
            #[doc = "Multiple voltage reference high (VREFH) inputs supported."]
            pub const MULTIPLE_REF_SUPPORTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale Width"]
    pub mod CSW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Channel scaling not supported. CSCALE control field not implemented."]
            pub const CSCALE_NOT_SUPPORTED: u32 = 0;
            #[doc = "Channel scaling supported. 1-bit CSCALE control field."]
            pub const BIT_WIDTH_1: u32 = 0x01;
            #[doc = "Channel scaling supported. 6-bit CSCALE control field."]
            pub const BIT_WIDTH_6: u32 = 0x06;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Voltage Reference 1 Range Control Bit Implemented"]
    pub mod VR1RNGI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Range control not required. CFG\\[VREF1RNG\\] is not implemented."]
            pub const REF1_FIXED_VOLTAGE_RANGE: u32 = 0;
            #[doc = "Range control required. CFG\\[VREF1RNG\\] is implemented."]
            pub const REF1_SELECTABLE_VOLTAGE_RANGE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Internal ADC Clock Implemented"]
    pub mod IADCKI {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal clock source not implemented."]
            pub const INTERNAL_CLK_NOT_AVAILABLE: u32 = 0;
            #[doc = "Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
            pub const INTERNAL_CLK_AVAILABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Calibration Function Implemented"]
    pub mod CALOFSI {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Calibration Not Implemented."]
            pub const CAL_FUNCTION_NOT_AVAILABLE: u32 = 0;
            #[doc = "Calibration Implemented."]
            pub const CAL_FUNCTION_AVAILABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of Single Ended Outputs Supported"]
    pub mod NUM_SEC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This design supports one single ended conversion at a time."]
            pub const SINGLE_CONVERTOR: u32 = 0;
            #[doc = "This design supports two simultaneous single ended conversions."]
            pub const DUAL_CONVERTOR: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of FIFOs"]
    pub mod NUM_FIFO {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "N/A"]
            pub const NO_FIFO_IMPLEMENTED: u32 = 0;
            #[doc = "This design supports one result FIFO."]
            pub const CNT_1: u32 = 0x01;
            #[doc = "This design supports two result FIFOs."]
            pub const CNT_2: u32 = 0x02;
            #[doc = "This design supports three result FIFOs."]
            pub const CNT_3: u32 = 0x03;
            #[doc = "This design supports four result FIFOs."]
            pub const CNT_4: u32 = 0x04;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Minor Version Number"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Major Version Number"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Parameter Register"]
pub mod PARAM {
    pub use crate::RO as access;
    #[doc = "Trigger Number"]
    pub mod TRIG_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Result FIFO Depth"]
    pub mod FIFOSIZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Result FIFO depth = 2 dataword."]
            pub const ENTRIES_2: u32 = 0x01;
            #[doc = "Result FIFO depth = 4 datawords."]
            pub const ENTRIES_4: u32 = 0x04;
            #[doc = "Result FIFO depth = 8 datawords."]
            pub const ENTRIES_8: u32 = 0x08;
            #[doc = "Result FIFO depth = 16 datawords."]
            pub const ENTRIES_16: u32 = 0x10;
            #[doc = "Result FIFO depth = 32 datawords."]
            pub const ENTRIES_32: u32 = 0x20;
            #[doc = "Result FIFO depth = 64 datawords."]
            pub const ENTRIES_64: u32 = 0x40;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Compare Value Number"]
    pub mod CV_NUM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command Buffer Number"]
    pub mod CMD_NUM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Control Register"]
pub mod CTRL {
    pub use crate::RW as access;
    #[doc = "ADC Enable"]
    pub mod ADCEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ADC is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "ADC is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software Reset"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ADC logic is not reset."]
            pub const RELEASED_FROM_RESET: u32 = 0;
            #[doc = "ADC logic is reset."]
            pub const HELD_IN_RESET: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Doze Enable"]
    pub mod DOZEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ADC is enabled in low power mode."]
            pub const ENABLED: u32 = 0;
            #[doc = "ADC is disabled in low power mode."]
            pub const DISABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Auto-Calibration Request"]
    pub mod CAL_REQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No request for hardware calibration has been made."]
            pub const NO_CALIBRATION_REQUEST: u32 = 0;
            #[doc = "A request for hardware calibration has been made"]
            pub const CALIBRATION_REQUEST_PENDING: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Offset Calibration Request"]
    pub mod CALOFS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Calibration function disabled"]
            pub const NO_ACTIVE_OFFSET_CALIBRATION_REQUEST: u32 = 0;
            #[doc = "Request for offset calibration function"]
            pub const OFFSET_CALIBRATION_REQUEST_PENDING: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Configure Mode for Offset Calibration Function"]
    pub mod CALOFSMODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Configure offset calibration for 12-bit mode."]
            pub const OFSTRIM_12_BIT: u32 = 0;
            #[doc = "Configure offset calibration for 16-bit mode."]
            pub const OFSTRIM_16_BIT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reset FIFO 0"]
    pub mod RSTFIFO0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect."]
            pub const NO_ACTION: u32 = 0;
            #[doc = "FIFO 0 is reset."]
            pub const TRIGGER_RESET: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reset FIFO 1"]
    pub mod RSTFIFO1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect."]
            pub const NO_ACTION: u32 = 0;
            #[doc = "FIFO 1 is reset."]
            pub const TRIGGER_RESET: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Auto-Calibration Averages"]
    pub mod CAL_AVGS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Status Register"]
pub mod STAT {
    pub use crate::RW as access;
    #[doc = "Result FIFO 0 Ready Flag"]
    pub mod RDY0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Result FIFO 0 data level not above watermark level."]
            pub const BELOW_THRESHOLD: u32 = 0;
            #[doc = "Result FIFO 0 holding data above watermark level."]
            pub const ABOVE_THRESHOLD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Result FIFO 0 Overflow Flag"]
    pub mod FOF0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No result FIFO 0 overflow has occurred since the last time the flag was cleared."]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
            pub const OVERFLOW_DETECTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Result FIFO1 Ready Flag"]
    pub mod RDY1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Result FIFO1 data level not above watermark level."]
            pub const BELOW_THRESHOLD: u32 = 0;
            #[doc = "Result FIFO1 holding data above watermark level."]
            pub const ABOVE_THRESHOLD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Result FIFO1 Overflow Flag"]
    pub mod FOF1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
            pub const OVERFLOW_DETECTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Interrupt Flag For High Priority Trigger Exception"]
    pub mod TEXC_INT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger exceptions have occurred."]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "A trigger exception has occurred and is pending acknowledgement."]
            pub const EXCEPTION_DETECTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Interrupt Flag For Trigger Completion"]
    pub mod TCOMP_INT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
            pub const FLAG_CLEAR: u32 = 0;
            #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
            pub const COMPLETION_DETECTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Calibration Ready"]
    pub mod CAL_RDY {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Calibration is incomplete or hasn't been ran."]
            pub const NOT_SET: u32 = 0;
            #[doc = "The ADC is calibrated."]
            pub const HARDWARE_CAL_STEP_COMPLETED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADC Active"]
    pub mod ADC_ACTIVE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The ADC is IDLE. There are no pending triggers to service and no active commands are being processed."]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "The ADC is processing a conversion, running through the power up delay, or servicing a trigger."]
            pub const BUSY: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger Active"]
    pub mod TRGACT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
            pub const TRIG_0: u32 = 0;
            #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
            pub const TRIG_1: u32 = 0x01;
            #[doc = "Command (sequence) associated with Trigger 2 currently being executed."]
            pub const TRIG_2: u32 = 0x02;
            #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
            pub const TRIG_X_3: u32 = 0x03;
            #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
            pub const TRIG_X_4: u32 = 0x04;
            #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
            pub const TRIG_X_5: u32 = 0x05;
            #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
            pub const TRIG_X_6: u32 = 0x06;
            #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
            pub const TRIG_X_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command Active"]
    pub mod CMDACT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No command is currently in progress."]
            pub const NO_COMMAND_ACTIVE: u32 = 0;
            #[doc = "Command 1 currently being executed."]
            pub const COMMAND_1: u32 = 0x01;
            #[doc = "Command 2 currently being executed."]
            pub const COMMAND_2: u32 = 0x02;
            #[doc = "Associated command number is currently being executed."]
            pub const COMMAND_X_3: u32 = 0x03;
            #[doc = "Associated command number is currently being executed."]
            pub const COMMAND_X_4: u32 = 0x04;
            #[doc = "Associated command number is currently being executed."]
            pub const COMMAND_X_5: u32 = 0x05;
            #[doc = "Associated command number is currently being executed."]
            pub const COMMAND_X_6: u32 = 0x06;
            #[doc = "Associated command number is currently being executed."]
            pub const COMMAND_X_7: u32 = 0x07;
            #[doc = "Associated command number is currently being executed."]
            pub const COMMAND_X_8: u32 = 0x08;
            #[doc = "Associated command number is currently being executed."]
            pub const COMMAND_X_9: u32 = 0x09;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Interrupt Enable Register"]
pub mod IE {
    pub use crate::RW as access;
    #[doc = "FIFO 0 Watermark Interrupt Enable"]
    pub mod FWMIE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO 0 watermark interrupts are not enabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "FIFO 0 watermark interrupts are enabled."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Result FIFO 0 Overflow Interrupt Enable"]
    pub mod FOFIE0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO 0 overflow interrupts are not enabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "FIFO 0 overflow interrupts are enabled."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO1 Watermark Interrupt Enable"]
    pub mod FWMIE1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO1 watermark interrupts are not enabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "FIFO1 watermark interrupts are enabled."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Result FIFO1 Overflow Interrupt Enable"]
    pub mod FOFIE1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
            pub const DISABLED: u32 = 0;
            #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger Exception Interrupt Enable"]
    pub mod TEXC_IE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Trigger exception interrupts are disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Trigger exception interrupts are enabled."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger Completion Interrupt Enable"]
    pub mod TCOMP_IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Trigger completion interrupts are disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
            pub const TRIGGER_0_COMPLETE_ENABLED: u32 = 0x01;
            #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
            pub const TRIGGER_1_COMPLETE_ENABLED: u32 = 0x02;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_3: u32 = 0x03;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_4: u32 = 0x04;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_5: u32 = 0x05;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_6: u32 = 0x06;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_7: u32 = 0x07;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_8: u32 = 0x08;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_9: u32 = 0x09;
            #[doc = "Trigger completion interrupts are enabled for every trigger source."]
            pub const ALL_TRIGGER_COMPLETES_ENABLED: u32 = 0xff;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DMA Enable Register"]
pub mod DE {
    pub use crate::RW as access;
    #[doc = "FIFO 0 Watermark DMA Enable"]
    pub mod FWMDE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DMA request disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "DMA request enabled."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO1 Watermark DMA Enable"]
    pub mod FWMDE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DMA request disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "DMA request enabled."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Configuration Register"]
pub mod CFG {
    pub use crate::RW as access;
    #[doc = "ADC Trigger Priority Control"]
    pub mod TPRICTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
            pub const ABORT_CURRENT_ON_PRIORITY: u32 = 0;
            #[doc = "If a higher priority trigger is received during command processing, the current command is stopped after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\] will be ignored and the higher priority trigger will be serviced."]
            pub const FINISH_CURRENT_ON_PRIORITY: u32 = 0x01;
            #[doc = "If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
            pub const FINISH_SEQUENCE_ON_PRIORITY: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Voltage Reference Selection"]
    pub mod REFSEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "(Default) Option 1 setting."]
            pub const OPTION_1: u32 = 0;
            #[doc = "Option 2 setting."]
            pub const OPTION_2: u32 = 0x01;
            #[doc = "Option 3 setting."]
            pub const OPTION_3: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger Resume Enable"]
    pub mod TRES {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Trigger sequences interrupted by a high priority trigger exception are not automatically resumed or restarted."]
            pub const DISABLED: u32 = 0;
            #[doc = "Trigger sequences interrupted by a high priority trigger exception are automatically resumed or restarted."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger Command Resume"]
    pub mod TCMDRES {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Trigger sequences interrupted by a high priority trigger exception is automatically restarted."]
            pub const DISABLED: u32 = 0;
            #[doc = "Trigger sequences interrupted by a high priority trigger exception is resumed from the command executing before the exception."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "High Priority Trigger Exception Disable"]
    pub mod HPT_EXDI {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "High priority trigger exceptions are enabled."]
            pub const ENABLED: u32 = 0;
            #[doc = "High priority trigger exceptions are disabled."]
            pub const DISABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Power Up Delay"]
    pub mod PUDLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADC Analog Pre-Enable"]
    pub mod PWREN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
            pub const NOT_PRE_ENABLED: u32 = 0;
            #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). Note that a single power up delay (CFG\\[PUDLY\\]) is executed immediately once PWREN is set, and any detected trigger does not begin ADC operation until the power up delay time has passed. After this initial delay expires the analog remains pre-enabled and no additional delays are executed."]
            pub const PRE_ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Pause Register"]
pub mod PAUSE {
    pub use crate::RW as access;
    #[doc = "Pause Delay"]
    pub mod PAUSEDLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PAUSE Option Enable"]
    pub mod PAUSEEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pause operation disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Pause operation enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Software Trigger Register"]
pub mod SWTRIG {
    pub use crate::RW as access;
    #[doc = "Software Trigger 0 Event"]
    pub mod SWT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger 0 event generated."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 0 event generated."]
            pub const INITIATE_TRIGGER_0: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software Trigger 1 Event"]
    pub mod SWT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger 1 event generated."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 1 event generated."]
            pub const INITIATE_TRIGGER_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software Trigger 2 Event"]
    pub mod SWT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger 2 event generated."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 2 event generated."]
            pub const INITIATE_TRIGGER_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software Trigger 3 Event"]
    pub mod SWT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger 3 event generated."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 3 event generated."]
            pub const INITIATE_TRIGGER_3: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger 4 event"]
    pub mod SWT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger 4 event generated."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 4 event generated."]
            pub const INITIATE_TRIGGER_4: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger 5 event"]
    pub mod SWT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger 5 event generated."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 5 event generated."]
            pub const INITIATE_TRIGGER_5: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger 6 event"]
    pub mod SWT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger 6 event generated."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 6 event generated."]
            pub const INITIATE_TRIGGER_6: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger 7 event"]
    pub mod SWT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger 7 event generated."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 7 event generated."]
            pub const INITIATE_TRIGGER_7: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Trigger Status Register"]
pub mod TSTAT {
    pub use crate::RW as access;
    #[doc = "Trigger Exception Number"]
    pub mod TEXC_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No triggers have been interrupted by a high priority exception. Or CFG\\[TRES\\] = 1."]
            pub const NO_EXCEPTIONS: u32 = 0;
            #[doc = "Trigger 0 has been interrupted by a high priority exception."]
            pub const BIT0_MEANS_TRIGGER_0_INTERRUPTED: u32 = 0x01;
            #[doc = "Trigger 1 has been interrupted by a high priority exception."]
            pub const BIT1_MEANS_TRIGGER_1_INTERRUPTED: u32 = 0x02;
            #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_3: u32 = 0x03;
            #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_4: u32 = 0x04;
            #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_5: u32 = 0x05;
            #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_6: u32 = 0x06;
            #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_7: u32 = 0x07;
            #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_8: u32 = 0x08;
            #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_9: u32 = 0x09;
            #[doc = "Every trigger sequence has been interrupted by a high priority exception."]
            pub const ALL_BITS_SET_INDICATE_ALL_TRIGGERS_INTERRUPTED: u32 = 0xff;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger Completion Flag"]
    pub mod TCOMP_FLAG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 0 has been completed and trigger 0 has enabled completion interrupts."]
            pub const BIT0_MEANS_TRIGGER_0_COMPLETED: u32 = 0x01;
            #[doc = "Trigger 1 has been completed and trigger 1 has enabled completion interrupts."]
            pub const BIT1_MEANS_TRIGGER_1_COMPLETED: u32 = 0x02;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_3: u32 = 0x03;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_4: u32 = 0x04;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_5: u32 = 0x05;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_6: u32 = 0x06;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_7: u32 = 0x07;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_8: u32 = 0x08;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_9: u32 = 0x09;
            #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
            pub const ALL_BITS_SET_INDICATE_ALL_TRIGGERS_COMPLETED: u32 = 0xff;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Offset Trim 16 bit Register"]
pub mod OFSTRIM16 {
    pub use crate::RW as access;
    #[doc = "Trim for Offset in A-side Converter for 16-bit Conversions"]
    pub mod OFSTRIM_A {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trim for Offset in B-side Converter for 16-bit Conversions"]
    pub mod OFSTRIM_B {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Offset Trim 12 bit Register"]
pub mod OFSTRIM12 {
    pub use crate::RW as access;
    #[doc = "Trim for Offset in A-side Converter for 12-bit Conversions"]
    pub mod OFSTRIM_A {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trim for Offset in B-side Converter for 12-bit Conversions"]
    pub mod OFSTRIM_B {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Trigger Control Register"]
pub mod TCTRL {
    pub use crate::RW as access;
    #[doc = "Trigger Enable"]
    pub mod HTEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Hardware trigger source disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Hardware trigger source enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SAR Result Destination for Channel A"]
    pub mod FIFO_SEL_A {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Result written to FIFO 0"]
            pub const STORE_TO_FIFO0: u32 = 0;
            #[doc = "Result written to FIFO 1"]
            pub const STORE_TO_FIFO1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SAR Result Destination for Channel B"]
    pub mod FIFO_SEL_B {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Result written to FIFO 0"]
            pub const STORE_TO_FIFO0: u32 = 0;
            #[doc = "Result written to FIFO 1"]
            pub const STORE_TO_FIFO1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger Priority Setting"]
    pub mod TPRI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Set to highest priority, Level 1"]
            pub const HIGHEST_PRIORITY: u32 = 0;
            #[doc = "Set to corresponding priority level"]
            pub const CORRESPONDING_LOWER_PRIORITY_1: u32 = 0x01;
            #[doc = "Set to corresponding priority level"]
            pub const CORRESPONDING_LOWER_PRIORITY_2: u32 = 0x02;
            #[doc = "Set to corresponding priority level"]
            pub const CORRESPONDING_LOWER_PRIORITY_3: u32 = 0x03;
            #[doc = "Set to corresponding priority level"]
            pub const CORRESPONDING_LOWER_PRIORITY_4: u32 = 0x04;
            #[doc = "Set to corresponding priority level"]
            pub const CORRESPONDING_LOWER_PRIORITY_5: u32 = 0x05;
            #[doc = "Set to corresponding priority level"]
            pub const CORRESPONDING_LOWER_PRIORITY_6: u32 = 0x06;
            #[doc = "Set to lowest priority, Level 8"]
            pub const LOWEST_PRIORITY: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger Resync"]
    pub mod RSYNC {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger Delay Select"]
    pub mod TDLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger Command Select"]
    pub mod TCMD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "CMD1 is executed"]
            pub const EXECUTE_CMD1: u32 = 0x01;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_2: u32 = 0x02;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_3: u32 = 0x03;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_4: u32 = 0x04;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_5: u32 = 0x05;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_6: u32 = 0x06;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_7: u32 = 0x07;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_8: u32 = 0x08;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_9: u32 = 0x09;
            #[doc = "CMD15 is executed"]
            pub const EXECUTE_CMD15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "FIFO Control Register"]
pub mod FCTRL {
    pub use crate::RW as access;
    #[doc = "Result FIFO Counter"]
    pub mod FCOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Watermark Level Selection"]
    pub mod FWMARK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gain Calibration Control"]
pub mod GCC {
    pub use crate::RO as access;
    #[doc = "Gain Calibration Value"]
    pub mod GAIN_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Calculated GAIN_CAL Value Ready"]
    pub mod RDY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The GAIN_CAL value is invalid. Run the hardware calibration routine for this value to be set."]
            pub const GAIN_CAL_NOT_VALID: u32 = 0;
            #[doc = "The GAIN_CAL value is valid. GAIN_CAL should be used by software to derive GCRa\\[GCALR\\]."]
            pub const HARDWARE_CAL_ROUTINE_COMPLETED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Gain Calculation Result"]
pub mod GCR {
    pub use crate::RW as access;
    #[doc = "Gain Calculation Result"]
    pub mod GCALR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Gain Calculation Ready"]
    pub mod RDY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The GCALR value is invalid."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "The GCALR value is valid."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL1 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH1 {
    pub use crate::RW as access;
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Compare disabled."]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Compare enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 0x02;
            #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL2 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH2 {
    pub use crate::RW as access;
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Compare disabled."]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Compare enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 0x02;
            #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL3 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH3 {
    pub use crate::RW as access;
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Compare disabled."]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Compare enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 0x02;
            #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL4 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH4 {
    pub use crate::RW as access;
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Compare disabled."]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Compare enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 0x02;
            #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL5 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH5 {
    pub use crate::RW as access;
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL6 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH6 {
    pub use crate::RW as access;
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL7 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH7 {
    pub use crate::RW as access;
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL8 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH8 {
    pub use crate::RW as access;
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL9 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH9 {
    pub use crate::RW as access;
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL10 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH10 {
    pub use crate::RW as access;
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL11 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH11 {
    pub use crate::RW as access;
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL12 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH12 {
    pub use crate::RW as access;
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL13 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH13 {
    pub use crate::RW as access;
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL14 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH14 {
    pub use crate::RW as access;
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL15 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-Ended Mode. Only A side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended Mode. Only B side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 0x01;
            #[doc = "Differential Mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 0x02;
            #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
            pub const DATA_16_BITS: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Scale"]
    pub mod CSCALE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_2: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 0x01;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 0x02;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 0x03;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 0x09;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 0x1e;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ALTBEN_ADCH disabled. Channel A and Channel B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel A inputs selected by ADCH setting and Channel B inputs selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Alt Channel B Scale"]
    pub mod ALTB_CSCALE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Scale selected analog channel (Factor of 1/2)"]
            pub const HALF_SCALE: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const FULL_SCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH15 {
    pub use crate::RW as access;
    #[doc = "Wait for Trigger Assertion before Execution."]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "This command will be automatically executed."]
            pub const DISABLED: u32 = 0;
            #[doc = "The active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const ENABLED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
            pub const SAMPLE_5P5: u32 = 0x01;
            #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
            pub const SAMPLE_7P5: u32 = 0x02;
            #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
            pub const SAMPLE_11P5: u32 = 0x03;
            #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
            pub const SAMPLE_19P5: u32 = 0x04;
            #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
            pub const SAMPLE_35P5: u32 = 0x05;
            #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
            pub const SAMPLE_67P5: u32 = 0x06;
            #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
            pub const SAMPLE_131P5: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 0x07;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 0x08;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 0x09;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 0x0a;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Looping not enabled. Command executes 1 time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const CMD_EXEC_2X: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const CMD_EXEC_3X: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const DO_CMD1_NEXT: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const DO_CMD15_NEXT: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Compare Value Register"]
pub mod CV {
    pub use crate::RW as access;
    #[doc = "Compare Value Low"]
    pub mod CVL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Compare Value High"]
    pub mod CVH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Data Result FIFO Register"]
pub mod RESFIFO {
    pub use crate::RO as access;
    #[doc = "Data Result"]
    pub mod D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger Source"]
    pub mod TSRC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Trigger source 0 initiated this conversion."]
            pub const TRIGGER_0: u32 = 0;
            #[doc = "Trigger source 1 initiated this conversion."]
            pub const TRIGGER_1: u32 = 0x01;
            #[doc = "Corresponding trigger source initiated this conversion."]
            pub const CORRESPONDING_TRIGGER_2: u32 = 0x02;
            #[doc = "Corresponding trigger source initiated this conversion."]
            pub const CORRESPONDING_TRIGGER_3: u32 = 0x03;
            #[doc = "Corresponding trigger source initiated this conversion."]
            pub const CORRESPONDING_TRIGGER_4: u32 = 0x04;
            #[doc = "Corresponding trigger source initiated this conversion."]
            pub const CORRESPONDING_TRIGGER_5: u32 = 0x05;
            #[doc = "Corresponding trigger source initiated this conversion."]
            pub const CORRESPONDING_TRIGGER_6: u32 = 0x06;
            #[doc = "Trigger source 7 initiated this conversion."]
            pub const TRIGGER_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop Count Value"]
    pub mod LOOPCNT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Result is from initial conversion in command."]
            pub const RESULT_1: u32 = 0;
            #[doc = "Result is from second conversion in command."]
            pub const RESULT_2: u32 = 0x01;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const CORRESPONDING_RESULT_2: u32 = 0x02;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const CORRESPONDING_RESULT_3: u32 = 0x03;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const CORRESPONDING_RESULT_4: u32 = 0x04;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const CORRESPONDING_RESULT_5: u32 = 0x05;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const CORRESPONDING_RESULT_6: u32 = 0x06;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const CORRESPONDING_RESULT_7: u32 = 0x07;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const CORRESPONDING_RESULT_8: u32 = 0x08;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const CORRESPONDING_RESULT_9: u32 = 0x09;
            #[doc = "Result is from 16th conversion in command."]
            pub const RESULT_16: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command Buffer Source"]
    pub mod CMDSRC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not a valid value CMDSRC value for a dataword in RESFIFO. 0x0 is only found in initial FIFO state prior to an ADC conversion result dataword being stored to a RESFIFO buffer."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "CMD1 buffer used as control settings for this conversion."]
            pub const CMD1: u32 = 0x01;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_2: u32 = 0x02;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_3: u32 = 0x03;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_4: u32 = 0x04;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_5: u32 = 0x05;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_6: u32 = 0x06;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_7: u32 = 0x07;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_8: u32 = 0x08;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_9: u32 = 0x09;
            #[doc = "CMD15 buffer used as control settings for this conversion."]
            pub const CMD15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Entry is Valid"]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO is empty. Discard any read from RESFIFO."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "FIFO record read from RESFIFO is valid."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Calibration General A-Side Registers"]
pub mod CAL_GAR {
    pub use crate::RW as access;
    #[doc = "Calibration General A Side Register Element"]
    pub mod CAL_GAR_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Calibration General B-Side Registers"]
pub mod CAL_GBR {
    pub use crate::RW as access;
    #[doc = "Calibration General B Side Register Element"]
    pub mod CAL_GBR_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Configuration 2 Register"]
pub mod CFG2 {
    pub use crate::RW as access;
    #[doc = "Justified Left Enable register"]
    pub mod JLEFT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "For 12-bit single-ended conversions, RESFIFO data format is in standard format with data presented in bits RESFIFOa\\[D\\]\\[14:3\\]."]
            pub const DATA_12B_SE_STD_FORMAT: u32 = 0;
            #[doc = "For 12-bit single-ended conversions, RESFIFO data format is left-justified."]
            pub const DATA_12B_SE_LEFT_JUSTIFIED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
