#[doc = "LPADC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID Register"]
    pub VERID: u32,
    #[doc = "Parameter Register"]
    pub PARAM: u32,
    _reserved0: [u8; 0x08],
    #[doc = "LPADC Control Register"]
    pub CTRL: u32,
    #[doc = "LPADC Status Register"]
    pub STAT: u32,
    #[doc = "Interrupt Enable Register"]
    pub IE: u32,
    #[doc = "DMA Enable Register"]
    pub DE: u32,
    #[doc = "LPADC Configuration Register"]
    pub CFG: u32,
    #[doc = "LPADC Pause Register"]
    pub PAUSE: u32,
    _reserved1: [u8; 0x08],
    #[doc = "LPADC FIFO Control Register"]
    pub FCTRL: u32,
    #[doc = "Software Trigger Register"]
    pub SWTRIG: u32,
    _reserved2: [u8; 0x88],
    #[doc = "Trigger Control Register"]
    pub TCTRL: [u32; 8usize],
    _reserved3: [u8; 0x20],
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL1: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH1: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL2: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH2: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL3: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH3: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL4: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH4: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL5: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH5: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL6: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH6: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL7: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH7: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL8: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH8: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL9: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH9: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL10: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH10: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL11: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH11: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL12: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH12: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL13: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH13: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL14: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH14: u32,
    #[doc = "LPADC Command Low Buffer Register"]
    pub CMDL15: u32,
    #[doc = "LPADC Command High Buffer Register"]
    pub CMDH15: u32,
    _reserved4: [u8; 0x88],
    #[doc = "Compare Value Register"]
    pub CV: [u32; 4usize],
    _reserved5: [u8; 0xf0],
    #[doc = "LPADC Data Result FIFO Register"]
    pub RESFIFO: u32,
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
            pub const RES_0: u32 = 0;
            #[doc = "Up to 16-bit differential/15-bit single ended resolution supported."]
            pub const RES_1: u32 = 0x01;
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
            pub const DIFFEN_0: u32 = 0;
            #[doc = "Differential operation supported. CMDLa\\[DIFF\\] and CMDLa\\[ABSEL\\] control fields implemented."]
            pub const DIFFEN_1: u32 = 0x01;
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
            #[doc = "Single voltage reference input supported."]
            pub const MVI_0: u32 = 0;
            #[doc = "Multiple voltage reference inputs supported."]
            pub const MVI_1: u32 = 0x01;
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
            #[doc = "Channel scaling not supported."]
            pub const CSW_0: u32 = 0;
            #[doc = "Channel scaling supported. 1-bit CSCALE control field."]
            pub const CSW_1: u32 = 0x01;
            #[doc = "Channel scaling supported. 6-bit CSCALE control field."]
            pub const CSW_6: u32 = 0x06;
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
            pub const VR1RNGI_0: u32 = 0;
            #[doc = "Range control required. CFG\\[VREF1RNG\\] is implemented."]
            pub const VR1RNGI_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Internal LPADC Clock implemented"]
    pub mod IADCKI {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal clock source not implemented."]
            pub const IADCKI_0: u32 = 0;
            #[doc = "Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
            pub const IADCKI_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Calibration Offset Function Implemented"]
    pub mod CALOFSI {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Offset calibration and offset trimming not implemented."]
            pub const CALOFSI_0: u32 = 0;
            #[doc = "Offset calibration and offset trimming implemented."]
            pub const CALOFSI_1: u32 = 0x01;
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
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "8 hardware triggers implemented"]
            pub const TRIG_NUM_8: u32 = 0x08;
        }
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
            #[doc = "Result FIFO depth = 16 datawords."]
            pub const FIFOSIZE_16: u32 = 0x10;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Compare Value Number"]
    pub mod CV_NUM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4 compare value registers implemented"]
            pub const CV_NUM_4: u32 = 0x04;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command Buffer Number"]
    pub mod CMD_NUM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "15 command buffers implemented"]
            pub const CMD_NUM_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Control Register"]
pub mod CTRL {
    pub use crate::RW as access;
    #[doc = "LPADC Enable"]
    pub mod ADCEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPADC is disabled."]
            pub const ADCEN_0: u32 = 0;
            #[doc = "LPADC is enabled."]
            pub const ADCEN_1: u32 = 0x01;
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
            #[doc = "LPADC logic is not reset."]
            pub const RST_0: u32 = 0;
            #[doc = "LPADC logic is reset."]
            pub const RST_1: u32 = 0x01;
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
            #[doc = "LPADC is enabled in Doze mode."]
            pub const DOZEN_0: u32 = 0;
            #[doc = "LPADC is disabled in Doze mode."]
            pub const DOZEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware trigger source selection"]
    pub mod TRIG_SRC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ADC_ETC hw trigger , and HW trigger are enabled"]
            pub const TRIG_SRC_0: u32 = 0;
            #[doc = "ADC_ETC hw trigger is enabled"]
            pub const TRIG_SRC_1: u32 = 0x01;
            #[doc = "HW trigger is enabled"]
            pub const TRIG_SRC_2: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reset FIFO"]
    pub mod RSTFIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect."]
            pub const RSTFIFO_0: u32 = 0;
            #[doc = "FIFO is reset."]
            pub const RSTFIFO_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Status Register"]
pub mod STAT {
    pub use crate::RW as access;
    #[doc = "Result FIFO Ready Flag"]
    pub mod RDY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Result FIFO data level not above watermark level."]
            pub const RDY_0: u32 = 0;
            #[doc = "Result FIFO holding data above watermark level."]
            pub const RDY_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Result FIFO Overflow Flag"]
    pub mod FOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No result FIFO overflow has occurred since the last time the flag was cleared."]
            pub const FOF_0: u32 = 0;
            #[doc = "At least one result FIFO overflow has occurred since the last time the flag was cleared."]
            pub const FOF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ADC Active"]
    pub mod ADC_ACTIVE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The LPADC is IDLE. There are no pending triggers to service and no active commands are being processed."]
            pub const ADC_ACTIVE_0: u32 = 0;
            #[doc = "The LPADC is processing a conversion, running through the power up delay, or servicing a trigger."]
            pub const ADC_ACTIVE_1: u32 = 0x01;
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
            pub const TRGACT_0: u32 = 0;
            #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
            pub const TRGACT_1: u32 = 0x01;
            #[doc = "Command (sequence) associated with Trigger 2 currently being executed."]
            pub const TRGACT_2: u32 = 0x02;
            #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
            pub const TRGACT_3: u32 = 0x03;
            #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
            pub const TRGACT_4: u32 = 0x04;
            #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
            pub const TRGACT_5: u32 = 0x05;
            #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
            pub const TRGACT_6: u32 = 0x06;
            #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
            pub const TRGACT_7: u32 = 0x07;
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
            pub const CMDACT_0: u32 = 0;
            #[doc = "Command 1 currently being executed."]
            pub const CMDACT_1: u32 = 0x01;
            #[doc = "Command 2 currently being executed."]
            pub const CMDACT_2: u32 = 0x02;
            #[doc = "Associated command number is currently being executed."]
            pub const CMDACT_3: u32 = 0x03;
            #[doc = "Associated command number is currently being executed."]
            pub const CMDACT_4: u32 = 0x04;
            #[doc = "Associated command number is currently being executed."]
            pub const CMDACT_5: u32 = 0x05;
            #[doc = "Associated command number is currently being executed."]
            pub const CMDACT_6: u32 = 0x06;
            #[doc = "Associated command number is currently being executed."]
            pub const CMDACT_7: u32 = 0x07;
            #[doc = "Associated command number is currently being executed."]
            pub const CMDACT_8: u32 = 0x08;
            #[doc = "Associated command number is currently being executed."]
            pub const CMDACT_9: u32 = 0x09;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Interrupt Enable Register"]
pub mod IE {
    pub use crate::RW as access;
    #[doc = "FIFO Watermark Interrupt Enable"]
    pub mod FWMIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO watermark interrupts are not enabled."]
            pub const FWMIE_0: u32 = 0;
            #[doc = "FIFO watermark interrupts are enabled."]
            pub const FWMIE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Result FIFO Overflow Interrupt Enable"]
    pub mod FOFIE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO overflow interrupts are not enabled."]
            pub const FOFIE_0: u32 = 0;
            #[doc = "FIFO overflow interrupts are enabled."]
            pub const FOFIE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DMA Enable Register"]
pub mod DE {
    pub use crate::RW as access;
    #[doc = "FIFO Watermark DMA Enable"]
    pub mod FWMDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DMA request disabled."]
            pub const FWMDE_0: u32 = 0;
            #[doc = "DMA request enabled."]
            pub const FWMDE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Configuration Register"]
pub mod CFG {
    pub use crate::RW as access;
    #[doc = "LPADC trigger priority control"]
    pub mod TPRICTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
            pub const TPRICTRL_0: u32 = 0;
            #[doc = "If a higher priority trigger is received during command processing, the current conversion is completed (including averaging iterations if enabled) and stored to the RESFIFO before the higher priority trigger/command is initiated. Note that compare until true commands can be interrupted prior to resulting in a true conversion."]
            pub const TPRICTRL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Power Configuration Select"]
    pub mod PWRSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Level 1 (Lowest power setting)"]
            pub const PWRSEL_0: u32 = 0;
            #[doc = "Level 2"]
            pub const PWRSEL_1: u32 = 0x01;
            #[doc = "Level 3"]
            pub const PWRSEL_2: u32 = 0x02;
            #[doc = "Level 4 (Highest power setting)"]
            pub const PWRSEL_3: u32 = 0x03;
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
            pub const REFSEL_0: u32 = 0;
            #[doc = "Option 2 setting."]
            pub const REFSEL_1: u32 = 0x01;
            #[doc = "Option 3 setting."]
            pub const REFSEL_2: u32 = 0x02;
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
    #[doc = "LPADC Analog Pre-Enable"]
    pub mod PWREN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LPADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
            pub const PWREN_0: u32 = 0;
            #[doc = "LPADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). When PWREN is set, the power up delay is enforced such that any detected trigger does not begin ADC operation until the power up delay time has passed."]
            pub const PWREN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Pause Register"]
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
            pub const PAUSEEN_0: u32 = 0;
            #[doc = "Pause operation enabled"]
            pub const PAUSEEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC FIFO Control Register"]
pub mod FCTRL {
    pub use crate::RW as access;
    #[doc = "Result FIFO counter"]
    pub mod FCOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No data stored in FIFO"]
            pub const FCOUNT_0: u32 = 0;
            #[doc = "1 dataword stored in FIFO"]
            pub const FCOUNT_1: u32 = 0x01;
            #[doc = "2 datawords stored in FIFO"]
            pub const FCOUNT_2: u32 = 0x02;
            #[doc = "4 datawords stored in FIFO"]
            pub const FCOUNT_4: u32 = 0x04;
            #[doc = "8 datawords stored in FIFO"]
            pub const FCOUNT_8: u32 = 0x08;
            #[doc = "16 datawords stored in FIFO"]
            pub const FCOUNT_16: u32 = 0x10;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Watermark level selection"]
    pub mod FWMARK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Generates STAT\\[RDY\\] flag after 1st successful conversion - single conversion"]
            pub const FWMARK_0: u32 = 0;
            #[doc = "Generates STAT\\[RDY\\] flag after 2nd successful conversion"]
            pub const FWMARK_1: u32 = 0x01;
            #[doc = "Generates STAT\\[RDY\\] flag after 3rd successful conversion"]
            pub const FWMARK_2: u32 = 0x02;
            #[doc = "Generates STAT\\[RDY\\] flag after 4th successful conversion"]
            pub const FWMARK_3: u32 = 0x03;
            #[doc = "Generates STAT\\[RDY\\] flag after 5th successful conversion"]
            pub const FWMARK_4: u32 = 0x04;
            #[doc = "Generates STAT\\[RDY\\] flag after 6th successful conversion"]
            pub const FWMARK_5: u32 = 0x05;
            #[doc = "Generates STAT\\[RDY\\] flag after 7th successful conversion"]
            pub const FWMARK_6: u32 = 0x06;
            #[doc = "Generates STAT\\[RDY\\] flag after 8th successful conversion"]
            pub const FWMARK_7: u32 = 0x07;
            #[doc = "Generates STAT\\[RDY\\] flag after 9th successful conversion"]
            pub const FWMARK_8: u32 = 0x08;
            #[doc = "Generates STAT\\[RDY\\] flag after 10th successful conversion"]
            pub const FWMARK_9: u32 = 0x09;
            #[doc = "Generates STAT\\[RDY\\] flag after 11th successful conversion"]
            pub const FWMARK_10: u32 = 0x0a;
            #[doc = "Generates STAT\\[RDY\\] flag after 12th successful conversion"]
            pub const FWMARK_11: u32 = 0x0b;
            #[doc = "Generates STAT\\[RDY\\] flag after 13th successful conversion"]
            pub const FWMARK_12: u32 = 0x0c;
            #[doc = "Generates STAT\\[RDY\\] flag after 14th successful conversion"]
            pub const FWMARK_13: u32 = 0x0d;
            #[doc = "Generates STAT\\[RDY\\] flag after 15th successful conversion"]
            pub const FWMARK_14: u32 = 0x0e;
            #[doc = "Generates STAT\\[RDY\\] flag after 16th successful conversion"]
            pub const FWMARK_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Software Trigger Register"]
pub mod SWTRIG {
    pub use crate::RW as access;
    #[doc = "Software trigger 0 event"]
    pub mod SWT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger 0 event generated."]
            pub const SWT0_0: u32 = 0;
            #[doc = "Trigger 0 event generated."]
            pub const SWT0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger 1 event"]
    pub mod SWT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger 1 event generated."]
            pub const SWT1_0: u32 = 0;
            #[doc = "Trigger 1 event generated."]
            pub const SWT1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger 2 event"]
    pub mod SWT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger 2 event generated."]
            pub const SWT2_0: u32 = 0;
            #[doc = "Trigger 2 event generated."]
            pub const SWT2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software trigger 3 event"]
    pub mod SWT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No trigger 3 event generated."]
            pub const SWT3_0: u32 = 0;
            #[doc = "Trigger 3 event generated."]
            pub const SWT3_1: u32 = 0x01;
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
            pub const SWT4_0: u32 = 0;
            #[doc = "Trigger 4 event generated."]
            pub const SWT4_1: u32 = 0x01;
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
            pub const SWT5_0: u32 = 0;
            #[doc = "Trigger 5 event generated."]
            pub const SWT5_1: u32 = 0x01;
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
            pub const SWT6_0: u32 = 0;
            #[doc = "Trigger 6 event generated."]
            pub const SWT6_1: u32 = 0x01;
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
            pub const SWT7_0: u32 = 0;
            #[doc = "Trigger 7 event generated."]
            pub const SWT7_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Trigger Control Register"]
pub mod TCTRL {
    pub use crate::RW as access;
    #[doc = "Trigger enable"]
    pub mod HTEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Hardware trigger source disabled"]
            pub const HTEN_0: u32 = 0;
            #[doc = "Hardware trigger source enabled"]
            pub const HTEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The command number is selected by software TCMD or hardware tcmd signal"]
    pub mod CMD_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TCTRLa\\[TCMD\\] will determine the command"]
            pub const CMD_SEL_0: u32 = 0;
            #[doc = "Software TCDM is bypassed , and hardware TCMD from ADC_ETC module will be used. The trigger command is then defined by ADC hardware trigger command selection field in ADC_ETC->TRIGx_CHAINy_z_n\\[CSEL\\]."]
            pub const CMD_SEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger priority setting"]
    pub mod TPRI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Set to highest priority, Level 1"]
            pub const TPRI_0: u32 = 0;
            #[doc = "Set to corresponding priority level"]
            pub const TPRI_1: u32 = 0x01;
            #[doc = "Set to corresponding priority level"]
            pub const TPRI_2: u32 = 0x02;
            #[doc = "Set to corresponding priority level"]
            pub const TPRI_3: u32 = 0x03;
            #[doc = "Set to corresponding priority level"]
            pub const TPRI_4: u32 = 0x04;
            #[doc = "Set to corresponding priority level"]
            pub const TPRI_5: u32 = 0x05;
            #[doc = "Set to corresponding priority level"]
            pub const TPRI_6: u32 = 0x06;
            #[doc = "Set to lowest priority, Level 8"]
            pub const TPRI_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger delay select"]
    pub mod TDLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Trigger command select"]
    pub mod TCMD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const TCMD_0: u32 = 0;
            #[doc = "CMD1 is executed"]
            pub const TCMD_1: u32 = 0x01;
            #[doc = "Corresponding CMD is executed"]
            pub const TCMD_2: u32 = 0x02;
            #[doc = "Corresponding CMD is executed"]
            pub const TCMD_3: u32 = 0x03;
            #[doc = "Corresponding CMD is executed"]
            pub const TCMD_4: u32 = 0x04;
            #[doc = "Corresponding CMD is executed"]
            pub const TCMD_5: u32 = 0x05;
            #[doc = "Corresponding CMD is executed"]
            pub const TCMD_6: u32 = 0x06;
            #[doc = "Corresponding CMD is executed"]
            pub const TCMD_7: u32 = 0x07;
            #[doc = "Corresponding CMD is executed"]
            pub const TCMD_8: u32 = 0x08;
            #[doc = "Corresponding CMD is executed"]
            pub const TCMD_9: u32 = 0x09;
            #[doc = "CMD15 is executed"]
            pub const TCMD_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL1 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
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
            pub const CMPEN_0: u32 = 0;
            #[doc = "Compare enabled. Store on true."]
            pub const CMPEN_2: u32 = 0x02;
            #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
            pub const CMPEN_3: u32 = 0x03;
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
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL2 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
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
            pub const CMPEN_0: u32 = 0;
            #[doc = "Compare enabled. Store on true."]
            pub const CMPEN_2: u32 = 0x02;
            #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
            pub const CMPEN_3: u32 = 0x03;
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
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL3 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
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
            pub const CMPEN_0: u32 = 0;
            #[doc = "Compare enabled. Store on true."]
            pub const CMPEN_2: u32 = 0x02;
            #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
            pub const CMPEN_3: u32 = 0x03;
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
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL4 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
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
            pub const CMPEN_0: u32 = 0;
            #[doc = "Compare enabled. Store on true."]
            pub const CMPEN_2: u32 = 0x02;
            #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
            pub const CMPEN_3: u32 = 0x03;
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
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL5 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
pub mod CMDH5 {
    pub use crate::RW as access;
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL6 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
pub mod CMDH6 {
    pub use crate::RW as access;
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL7 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
pub mod CMDH7 {
    pub use crate::RW as access;
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL8 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
pub mod CMDH8 {
    pub use crate::RW as access;
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL9 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
pub mod CMDH9 {
    pub use crate::RW as access;
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL10 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
pub mod CMDH10 {
    pub use crate::RW as access;
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL11 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
pub mod CMDH11 {
    pub use crate::RW as access;
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL12 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
pub mod CMDH12 {
    pub use crate::RW as access;
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL13 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
pub mod CMDH13 {
    pub use crate::RW as access;
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL14 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
pub mod CMDH14 {
    pub use crate::RW as access;
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command Low Buffer Register"]
pub mod CMDL15 {
    pub use crate::RW as access;
    #[doc = "Input channel select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
            pub const ADCH_0: u32 = 0;
            #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
            pub const ADCH_1: u32 = 0x01;
            #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
            pub const ADCH_2: u32 = 0x02;
            #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
            pub const ADCH_3: u32 = 0x03;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_4: u32 = 0x04;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_5: u32 = 0x05;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_6: u32 = 0x06;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_7: u32 = 0x07;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_8: u32 = 0x08;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const ADCH_9: u32 = 0x09;
            #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
            pub const ADCH_30: u32 = 0x1e;
            #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "A-side vs. B-side Select"]
    pub mod ABSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
            pub const ABSEL_0: u32 = 0;
            #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
            pub const ABSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Differential Mode Enable"]
    pub mod DIFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single-ended mode."]
            pub const DIFF_0: u32 = 0;
            #[doc = "Differential mode."]
            pub const DIFF_1: u32 = 0x01;
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
            #[doc = "Scale selected analog channel (Factor of 30/64)"]
            pub const CSCALE_0: u32 = 0;
            #[doc = "(Default) Full scale (Factor of 1)"]
            pub const CSCALE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "LPADC Command High Buffer Register"]
pub mod CMDH15 {
    pub use crate::RW as access;
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Auto channel increment disabled"]
            pub const LWI_0: u32 = 0;
            #[doc = "Auto channel increment enabled"]
            pub const LWI_1: u32 = 0x01;
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
            #[doc = "Minimum sample time of 3 ADCK cycles."]
            pub const STS_0: u32 = 0;
            #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
            pub const STS_1: u32 = 0x01;
            #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
            pub const STS_2: u32 = 0x02;
            #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
            pub const STS_3: u32 = 0x03;
            #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
            pub const STS_4: u32 = 0x04;
            #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
            pub const STS_5: u32 = 0x05;
            #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
            pub const STS_6: u32 = 0x06;
            #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
            pub const STS_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Single conversion."]
            pub const AVGS_0: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "4 conversions averaged."]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "8 conversions averaged."]
            pub const AVGS_3: u32 = 0x03;
            #[doc = "16 conversions averaged."]
            pub const AVGS_4: u32 = 0x04;
            #[doc = "32 conversions averaged."]
            pub const AVGS_5: u32 = 0x05;
            #[doc = "64 conversions averaged."]
            pub const AVGS_6: u32 = 0x06;
            #[doc = "128 conversions averaged."]
            pub const AVGS_7: u32 = 0x07;
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
            pub const LOOP_0: u32 = 0;
            #[doc = "Loop 1 time. Command executes 2 times."]
            pub const LOOP_1: u32 = 0x01;
            #[doc = "Loop 2 times. Command executes 3 times."]
            pub const LOOP_2: u32 = 0x02;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_3: u32 = 0x03;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_4: u32 = 0x04;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_5: u32 = 0x05;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_6: u32 = 0x06;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_7: u32 = 0x07;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_8: u32 = 0x08;
            #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
            pub const LOOP_9: u32 = 0x09;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const LOOP_15: u32 = 0x0f;
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
            pub const NEXT_0: u32 = 0;
            #[doc = "Select CMD1 command buffer register as next command."]
            pub const NEXT_1: u32 = 0x01;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_2: u32 = 0x02;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_3: u32 = 0x03;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_4: u32 = 0x04;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_5: u32 = 0x05;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_6: u32 = 0x06;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_7: u32 = 0x07;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_8: u32 = 0x08;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const NEXT_9: u32 = 0x09;
            #[doc = "Select CMD15 command buffer register as next command."]
            pub const NEXT_15: u32 = 0x0f;
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
    #[doc = "Compare Value High."]
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
#[doc = "LPADC Data Result FIFO Register"]
pub mod RESFIFO {
    pub use crate::RO as access;
    #[doc = "Data result"]
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
            pub const TSRC_0: u32 = 0;
            #[doc = "Trigger source 1 initiated this conversion."]
            pub const TSRC_1: u32 = 0x01;
            #[doc = "Corresponding trigger source initiated this conversion."]
            pub const TSRC_2: u32 = 0x02;
            #[doc = "Corresponding trigger source initiated this conversion."]
            pub const TSRC_3: u32 = 0x03;
            #[doc = "Corresponding trigger source initiated this conversion."]
            pub const TSRC_4: u32 = 0x04;
            #[doc = "Corresponding trigger source initiated this conversion."]
            pub const TSRC_5: u32 = 0x05;
            #[doc = "Corresponding trigger source initiated this conversion."]
            pub const TSRC_6: u32 = 0x06;
            #[doc = "Trigger source 7 initiated this conversion."]
            pub const TSRC_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loop count value"]
    pub mod LOOPCNT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Result is from initial conversion in command."]
            pub const LOOPCNT_0: u32 = 0;
            #[doc = "Result is from second conversion in command."]
            pub const LOOPCNT_1: u32 = 0x01;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const LOOPCNT_2: u32 = 0x02;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const LOOPCNT_3: u32 = 0x03;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const LOOPCNT_4: u32 = 0x04;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const LOOPCNT_5: u32 = 0x05;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const LOOPCNT_6: u32 = 0x06;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const LOOPCNT_7: u32 = 0x07;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const LOOPCNT_8: u32 = 0x08;
            #[doc = "Result is from LOOPCNT+1 conversion in command."]
            pub const LOOPCNT_9: u32 = 0x09;
            #[doc = "Result is from 16th conversion in command."]
            pub const LOOPCNT_15: u32 = 0x0f;
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
            pub const CMDSRC_0: u32 = 0;
            #[doc = "CMD1 buffer used as control settings for this conversion."]
            pub const CMDSRC_1: u32 = 0x01;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CMDSRC_2: u32 = 0x02;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CMDSRC_3: u32 = 0x03;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CMDSRC_4: u32 = 0x04;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CMDSRC_5: u32 = 0x05;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CMDSRC_6: u32 = 0x06;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CMDSRC_7: u32 = 0x07;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CMDSRC_8: u32 = 0x08;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CMDSRC_9: u32 = 0x09;
            #[doc = "CMD15 buffer used as control settings for this conversion."]
            pub const CMDSRC_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO entry is valid"]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO is empty. Discard any read from RESFIFO."]
            pub const VALID_0: u32 = 0;
            #[doc = "FIFO record read from RESFIFO is valid."]
            pub const VALID_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
