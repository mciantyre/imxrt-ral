#[doc = "Analog-to-Digital Converter"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control register for hardware triggers"]
    pub HC0: u32,
    #[doc = "Control register for hardware triggers"]
    pub HC: [u32; 7usize],
    #[doc = "Status register for HW triggers"]
    pub HS: u32,
    #[doc = "Data result register for HW triggers"]
    pub R0: u32,
    #[doc = "Data result register for HW triggers"]
    pub R: [u32; 7usize],
    #[doc = "Configuration register"]
    pub CFG: u32,
    #[doc = "General control register"]
    pub GC: u32,
    #[doc = "General status register"]
    pub GS: u32,
    #[doc = "Compare value register"]
    pub CV: u32,
    #[doc = "Offset correction value register"]
    pub OFS: u32,
    #[doc = "Calibration value register"]
    pub CAL: u32,
}
#[doc = "Control register for hardware triggers"]
pub mod HC0 {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "External channel selection from ADC_ETC"]
            pub const ADCH_16: u32 = 0x10;
            #[doc = "VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
            pub const ADCH_25: u32 = 0x19;
            #[doc = "Conversion Disabled. Hardware Triggers will not initiate any conversion."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Complete Interrupt Enable/Disable Control"]
    pub mod AIEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Conversion complete interrupt disabled"]
            pub const AIEN_0: u32 = 0;
            #[doc = "Conversion complete interrupt enabled"]
            pub const AIEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Control register for hardware triggers"]
pub mod HC {
    pub use crate::RW as access;
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "External channel selection from ADC_ETC"]
            pub const ADCH_16: u32 = 0x10;
            #[doc = "VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
            pub const ADCH_25: u32 = 0x19;
            #[doc = "Conversion Disabled. Hardware Triggers will not initiate any conversion."]
            pub const ADCH_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Complete Interrupt Enable/Disable Control"]
    pub mod AIEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Conversion complete interrupt disabled"]
            pub const AIEN_0: u32 = 0;
            #[doc = "Conversion complete interrupt enabled"]
            pub const AIEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Status register for HW triggers"]
pub mod HS {
    pub use crate::RO as access;
    #[doc = "Conversion Complete Flag"]
    pub mod COCO0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Data result register for HW triggers"]
pub mod R0 {
    pub use crate::RO as access;
    #[doc = "Data (result of an ADC conversion)"]
    pub mod CDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Data result register for HW triggers"]
pub mod R {
    pub use crate::RO as access;
    #[doc = "Data (result of an ADC conversion)"]
    pub mod CDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Configuration register"]
pub mod CFG {
    pub use crate::RW as access;
    #[doc = "Input Clock Select"]
    pub mod ADICLK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "IPG clock"]
            pub const ADICLK_0: u32 = 0;
            #[doc = "IPG clock divided by 2"]
            pub const ADICLK_1: u32 = 0x01;
            #[doc = "Alternate clock (ALTCLK)"]
            pub const ADICLK_2: u32 = 0x02;
            #[doc = "Asynchronous clock (ADACK)"]
            pub const ADICLK_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Mode Selection"]
    pub mod MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "8-bit conversion"]
            pub const MODE_0: u32 = 0;
            #[doc = "10-bit conversion"]
            pub const MODE_1: u32 = 0x01;
            #[doc = "12-bit conversion"]
            pub const MODE_2: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Long Sample Time Configuration"]
    pub mod ADLSMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Short sample mode."]
            pub const ADLSMP_0: u32 = 0;
            #[doc = "Long sample mode."]
            pub const ADLSMP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Clock Divide Select"]
    pub mod ADIV {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Input clock"]
            pub const ADIV_0: u32 = 0;
            #[doc = "Input clock / 2"]
            pub const ADIV_1: u32 = 0x01;
            #[doc = "Input clock / 4"]
            pub const ADIV_2: u32 = 0x02;
            #[doc = "Input clock / 8"]
            pub const ADIV_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Low-Power Configuration"]
    pub mod ADLPC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "ADC hard block not in low power mode."]
            pub const ADLPC_0: u32 = 0;
            #[doc = "ADC hard block in low power mode."]
            pub const ADLPC_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Defines the sample time duration"]
    pub mod ADSTS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Sample period (ADC clocks) = 2 if ADLSMP=0b Sample period (ADC clocks) = 12 if ADLSMP=1b"]
            pub const ADSTS_0: u32 = 0;
            #[doc = "Sample period (ADC clocks) = 4 if ADLSMP=0b Sample period (ADC clocks) = 16 if ADLSMP=1b"]
            pub const ADSTS_1: u32 = 0x01;
            #[doc = "Sample period (ADC clocks) = 6 if ADLSMP=0b Sample period (ADC clocks) = 20 if ADLSMP=1b"]
            pub const ADSTS_2: u32 = 0x02;
            #[doc = "Sample period (ADC clocks) = 8 if ADLSMP=0b Sample period (ADC clocks) = 24 if ADLSMP=1b"]
            pub const ADSTS_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "High Speed Configuration"]
    pub mod ADHSC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Normal conversion selected."]
            pub const ADHSC_0: u32 = 0;
            #[doc = "High speed conversion selected."]
            pub const ADHSC_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Voltage Reference Selection"]
    pub mod REFSEL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Selects VREFH/VREFL as reference voltage."]
            pub const REFSEL_0: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Conversion Trigger Select"]
    pub mod ADTRG {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Software trigger selected"]
            pub const ADTRG_0: u32 = 0;
            #[doc = "Hardware trigger selected"]
            pub const ADTRG_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware Average select"]
    pub mod AVGS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "4 samples averaged"]
            pub const AVGS_0: u32 = 0;
            #[doc = "8 samples averaged"]
            pub const AVGS_1: u32 = 0x01;
            #[doc = "16 samples averaged"]
            pub const AVGS_2: u32 = 0x02;
            #[doc = "32 samples averaged"]
            pub const AVGS_3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Data Overwrite Enable"]
    pub mod OVWREN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable the overwriting. Existing Data in Data result register will not be overwritten by subsequent converted data."]
            pub const OVWREN_0: u32 = 0;
            #[doc = "Enable the overwriting."]
            pub const OVWREN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General control register"]
pub mod GC {
    pub use crate::RW as access;
    #[doc = "Asynchronous clock output enable"]
    pub mod ADACKEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active."]
            pub const ADACKEN_0: u32 = 0;
            #[doc = "Asynchronous clock and clock output enabled regardless of the state of the ADC"]
            pub const ADACKEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "DMA Enable"]
    pub mod DMAEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DMA disabled (default)"]
            pub const DMAEN_0: u32 = 0;
            #[doc = "DMA enabled"]
            pub const DMAEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Compare Function Range Enable"]
    pub mod ACREN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Range function disabled. Only the compare value 1 of ADC_CV register (CV1) is compared."]
            pub const ACREN_0: u32 = 0;
            #[doc = "Range function enabled. Both compare values of ADC_CV registers (CV1 and CV2) are compared."]
            pub const ACREN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Compare Function Greater Than Enable"]
    pub mod ACFGT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Configures \"Less Than Threshold, Outside Range Not Inclusive and Inside Range Not Inclusive\" functionality based on the values placed in the ADC_CV register."]
            pub const ACFGT_0: u32 = 0;
            #[doc = "Configures \"Greater Than Or Equal To Threshold, Outside Range Inclusive and Inside Range Inclusive\" functionality based on the values placed in the ADC_CV registers."]
            pub const ACFGT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Compare Function Enable"]
    pub mod ACFE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Compare function disabled"]
            pub const ACFE_0: u32 = 0;
            #[doc = "Compare function enabled"]
            pub const ACFE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hardware average enable"]
    pub mod AVGE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Hardware average function disabled"]
            pub const AVGE_0: u32 = 0;
            #[doc = "Hardware average function enabled"]
            pub const AVGE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Continuous Conversion Enable"]
    pub mod ADCO {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "One conversion or one set of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
            pub const ADCO_0: u32 = 0;
            #[doc = "Continuous conversions or sets of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
            pub const ADCO_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Calibration"]
    pub mod CAL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "General status register"]
pub mod GS {
    pub use crate::RW as access;
    #[doc = "Conversion Active"]
    pub mod ADACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Conversion not in progress."]
            pub const ADACT_0: u32 = 0;
            #[doc = "Conversion in progress."]
            pub const ADACT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Calibration Failed Flag"]
    pub mod CALF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Calibration completed normally."]
            pub const CALF_0: u32 = 0;
            #[doc = "Calibration failed. ADC accuracy specifications are not guaranteed."]
            pub const CALF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Asynchronous wakeup interrupt status"]
    pub mod AWKST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No asynchronous interrupt."]
            pub const AWKST_0: u32 = 0;
            #[doc = "Asynchronous wake up interrupt occurred in stop mode."]
            pub const AWKST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Compare value register"]
pub mod CV {
    pub use crate::RW as access;
    #[doc = "Compare Value 1"]
    pub mod CV1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Compare Value 2"]
    pub mod CV2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Offset correction value register"]
pub mod OFS {
    pub use crate::RW as access;
    #[doc = "Offset value"]
    pub mod OFS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sign bit"]
    pub mod SIGN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The offset value is added with the raw result"]
            pub const SIGN_0: u32 = 0;
            #[doc = "The offset value is subtracted from the raw converted value"]
            pub const SIGN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Calibration value register"]
pub mod CAL {
    pub use crate::RW as access;
    #[doc = "Calibration Result Value"]
    pub mod CAL_CODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
