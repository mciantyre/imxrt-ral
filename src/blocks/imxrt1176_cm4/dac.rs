#[doc = "DAC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version Identifier Register"]
    pub VERID: u32,
    #[doc = "Parameter Register"]
    pub PARAM: u32,
    #[doc = "DAC Data Register"]
    pub DATA: u32,
    #[doc = "DAC Status and Control Register"]
    pub CR: u32,
    #[doc = "DAC FIFO Pointer Register"]
    pub PTR: u32,
    #[doc = "DAC Status and Control Register 2"]
    pub CR2: u32,
}
#[doc = "Version Identifier Register"]
pub mod VERID {
    pub use crate::RO as access;
    #[doc = "Feature Identification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard feature set"]
            pub const FEATURE_0: u32 = 0;
            #[doc = "C40 feature set"]
            pub const FEATURE_1: u32 = 0x01;
            #[doc = "5V DAC feature set"]
            pub const FEATURE_2: u32 = 0x02;
            #[doc = "ADC BIST feature set"]
            pub const FEATURE_4: u32 = 0x04;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Minor version number"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Major version number"]
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
    #[doc = "FIFO size"]
    pub mod FIFOSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO depth is 2"]
            pub const FIFOSZ_0: u32 = 0;
            #[doc = "FIFO depth is 4"]
            pub const FIFOSZ_1: u32 = 0x01;
            #[doc = "FIFO depth is 8"]
            pub const FIFOSZ_2: u32 = 0x02;
            #[doc = "FIFO depth is 16"]
            pub const FIFOSZ_3: u32 = 0x03;
            #[doc = "FIFO depth is 32"]
            pub const FIFOSZ_4: u32 = 0x04;
            #[doc = "FIFO depth is 64"]
            pub const FIFOSZ_5: u32 = 0x05;
            #[doc = "FIFO depth is 128"]
            pub const FIFOSZ_6: u32 = 0x06;
            #[doc = "FIFO depth is 256"]
            pub const FIFOSZ_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DAC Data Register"]
pub mod DATA {
    pub use crate::WO as access;
    #[doc = "FIFO DATA0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DAC Status and Control Register"]
pub mod CR {
    pub use crate::RW as access;
    #[doc = "Full Flag"]
    pub mod FULLF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO is not full."]
            pub const FULLF_0: u32 = 0;
            #[doc = "FIFO is full."]
            pub const FULLF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Nearly Empty Flag"]
    pub mod NEMPTF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "More than one data is available in the FIFO."]
            pub const NEMPTF_0: u32 = 0;
            #[doc = "One data is available in the FIFO."]
            pub const NEMPTF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Watermark Status Flag"]
    pub mod WMF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The DAC buffer read pointer has not reached the watermark level."]
            pub const WMF_0: u32 = 0;
            #[doc = "The DAC buffer read pointer has reached the watermark level."]
            pub const WMF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Underflow Flag"]
    pub mod UDFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No underflow has occurred since the last time the flag was cleared."]
            pub const UDFF_0: u32 = 0;
            #[doc = "At least one trigger underflow has occurred since the last time the flag was cleared."]
            pub const UDFF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Overflow Flag"]
    pub mod OVFF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No overflow has occurred since the last time the flag was cleared."]
            pub const OVFF_0: u32 = 0;
            #[doc = "At least one FIFO overflow has occurred since the last time the flag was cleared."]
            pub const OVFF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Full Interrupt Enable"]
    pub mod FULLIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO Full interrupt is disabled."]
            pub const FULLIE_0: u32 = 0;
            #[doc = "FIFO Full interrupt is enabled."]
            pub const FULLIE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Nearly Empty Interrupt Enable"]
    pub mod EMPTIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO Nearly Empty interrupt is disabled."]
            pub const EMPTIE_0: u32 = 0;
            #[doc = "FIFO Nearly Empty interrupt is enabled."]
            pub const EMPTIE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Watermark Interrupt Enable"]
    pub mod WTMIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Watermark interrupt is disabled."]
            pub const WTMIE_0: u32 = 0;
            #[doc = "Watermark interrupt is enabled."]
            pub const WTMIE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DAC Software Trigger"]
    pub mod SWTRG {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The DAC soft trigger is not valid."]
            pub const SWTRG_0: u32 = 0;
            #[doc = "The DAC soft trigger is valid."]
            pub const SWTRG_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DAC Trigger Select"]
    pub mod TRGSEL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The DAC hardware trigger is selected."]
            pub const TRGSEL_0: u32 = 0;
            #[doc = "The DAC software trigger is selected."]
            pub const TRGSEL_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DAC Reference Select"]
    pub mod DACRFS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The DAC selects DACREF_1 as the reference voltage."]
            pub const DACRFS_0: u32 = 0;
            #[doc = "The DAC selects DACREF_2 as the reference voltage."]
            pub const DACRFS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DAC Enable"]
    pub mod DACEN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The DAC system is disabled."]
            pub const DACEN_0: u32 = 0;
            #[doc = "The DAC system is enabled."]
            pub const DACEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Enable"]
    pub mod FIFOEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO is disabled and only one level buffer is enabled. Any data written from this buffer goes to conversion."]
            pub const FIFOEN_0: u32 = 0;
            #[doc = "FIFO is enabled. Data will first read from FIFO to buffer then go to conversion."]
            pub const FIFOEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DAC FIFO Mode Select"]
    pub mod SWMD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Normal mode"]
            pub const SWMD_0: u32 = 0;
            #[doc = "Swing back mode"]
            pub const SWMD_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Underflow and overflow interrupt enable"]
    pub mod UVIE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Underflow and overflow interrupt is disabled."]
            pub const UVIE_0: u32 = 0;
            #[doc = "Underflow and overflow interrupt is enabled."]
            pub const UVIE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Reset"]
    pub mod FIFORST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect"]
            pub const FIFORST_0: u32 = 0;
            #[doc = "FIFO reset"]
            pub const FIFORST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software reset"]
    pub mod SWRST {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DMA Enable Select"]
    pub mod DMAEN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DMA is disabled."]
            pub const DMAEN_0: u32 = 0;
            #[doc = "DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time."]
            pub const DMAEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Watermark Level Select"]
    pub mod WML {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DAC FIFO Pointer Register"]
pub mod PTR {
    pub use crate::RO as access;
    #[doc = "DACWFP"]
    pub mod DACWFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DACRFP"]
    pub mod DACRFP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DAC Status and Control Register 2"]
pub mod CR2 {
    pub use crate::RW as access;
    #[doc = "Buffer Enable"]
    pub mod BFEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Opamp is not used as buffer"]
            pub const BFEN_0: u32 = 0;
            #[doc = "Opamp is used as buffer"]
            pub const BFEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Optional Enable"]
    pub mod OEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Output buffer is not bypassed"]
            pub const OEN_0: u32 = 0;
            #[doc = "Output buffer is bypassed"]
            pub const OEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Buffer Middle Speed Select"]
    pub mod BFMS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Buffer middle speed not selected"]
            pub const BFMS_0: u32 = 0;
            #[doc = "Buffer middle speed selected"]
            pub const BFMS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Buffer High Speed Select"]
    pub mod BFHS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Buffer high speed not selected"]
            pub const BFHS_0: u32 = 0;
            #[doc = "Buffer high speed selected"]
            pub const BFHS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Internal PTAT (Proportional To Absolute Temperature) Current Reference Select"]
    pub mod IREF2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal PTAT Current Reference not selected"]
            pub const IREF2_0: u32 = 0;
            #[doc = "Internal PTAT Current Reference selected"]
            pub const IREF2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Internal ZTC (Zero Temperature Coefficient) Current Reference Select"]
    pub mod IREF1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal ZTC Current Reference not selected"]
            pub const IREF1_0: u32 = 0;
            #[doc = "Internal ZTC Current Reference selected"]
            pub const IREF1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Internal Current Reference Select"]
    pub mod IREF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal Current Reference not selected"]
            pub const IREF_0: u32 = 0;
            #[doc = "Internal Current Reference selected"]
            pub const IREF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
