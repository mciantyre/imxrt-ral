#[doc = "SAI"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: u32,
    #[doc = "Parameter"]
    pub PARAM: u32,
    #[doc = "Transmit Control"]
    pub TCSR: u32,
    #[doc = "Transmit Configuration 1"]
    pub TCR1: u32,
    #[doc = "Transmit Configuration 2"]
    pub TCR2: u32,
    #[doc = "Transmit Configuration 3"]
    pub TCR3: u32,
    #[doc = "Transmit Configuration 4"]
    pub TCR4: u32,
    #[doc = "Transmit Configuration 5"]
    pub TCR5: u32,
    #[doc = "Transmit Data"]
    pub TDR: [u32; 2usize],
    _reserved0: [u8; 0x18],
    #[doc = "Transmit FIFO"]
    pub TFR: [u32; 2usize],
    _reserved1: [u8; 0x18],
    #[doc = "Transmit Mask"]
    pub TMR: u32,
    _reserved2: [u8; 0x24],
    #[doc = "Receive Control"]
    pub RCSR: u32,
    #[doc = "Receive Configuration 1"]
    pub RCR1: u32,
    #[doc = "Receive Configuration 2"]
    pub RCR2: u32,
    #[doc = "Receive Configuration 3"]
    pub RCR3: u32,
    #[doc = "Receive Configuration 4"]
    pub RCR4: u32,
    #[doc = "Receive Configuration 5"]
    pub RCR5: u32,
    #[doc = "Receive Data"]
    pub RDR: [u32; 2usize],
    _reserved3: [u8; 0x18],
    #[doc = "Receive FIFO"]
    pub RFR: [u32; 2usize],
    _reserved4: [u8; 0x18],
    #[doc = "Receive Mask"]
    pub RMR: u32,
}
#[doc = "Version ID"]
pub mod VERID {
    pub use crate::RO as access;
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Standard feature set."]
            pub const STD: u32 = 0;
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
#[doc = "Parameter"]
pub mod PARAM {
    pub use crate::RO as access;
    #[doc = "Number of Datalinks"]
    pub mod DATALINE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Size"]
    pub mod FIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame Size"]
    pub mod FRAME {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Control"]
pub mod TCSR {
    pub use crate::RW as access;
    #[doc = "FIFO Request DMA Enable"]
    pub mod FRDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables the DMA request."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the DMA request."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Warning DMA Enable"]
    pub mod FWDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables the DMA warning."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the DMA warning."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Request Interrupt Enable"]
    pub mod FRIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables the interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    pub mod FWIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables the interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables the interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sync Error Interrupt Enable"]
    pub mod SEIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables interrupt."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Word Start Interrupt Enable"]
    pub mod WSIE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables interrupt."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Request Flag"]
    pub mod FRF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Transmit FIFO watermark has not been reached."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Transmit FIFO watermark has been reached."]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Warning Flag"]
    pub mod FWF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No enabled transmit FIFO is empty."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled transmit FIFO is empty."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Transmit underrun not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Transmit underrun detected."]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sync Error Flag"]
    pub mod SEF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Sync error not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Frame sync error detected."]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Word Start Flag"]
    pub mod WSF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Start of word not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Start of word detected."]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software Reset"]
    pub mod SR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect"]
            pub const DISABLE: u32 = 0;
            #[doc = "Software reset"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Reset"]
    pub mod FR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "FIFO reset."]
            pub const RESET: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit Clock Enable"]
    pub mod BCE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Transmit bit clock is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit bit clock is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Debug Enable"]
    pub mod DBGE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Transmitter is disabled in Debug mode, after completing the current frame."]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmitter is enabled in Debug mode."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Stop Enable"]
    pub mod STOPE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Transmitter disabled in Stop mode."]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmitter enabled in Stop mode."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmitter Enable"]
    pub mod TE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Transmitter is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmitter is enabled, or transmitter has been disabled and has not yet reached end of frame."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Configuration 1"]
pub mod TCR1 {
    pub use crate::RW as access;
    #[doc = "Transmit FIFO Watermark"]
    pub mod TFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1 FIFO word"]
            pub const MIN: u32 = 0;
            #[doc = "2 FIFO words"]
            pub const TWO: u32 = 0x01;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_2: u32 = 0x02;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_3: u32 = 0x03;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_4: u32 = 0x04;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_5: u32 = 0x05;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_6: u32 = 0x06;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_7: u32 = 0x07;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_8: u32 = 0x08;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_9: u32 = 0x09;
            #[doc = "16 FIFO words"]
            pub const MAX: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Configuration 2"]
pub mod TCR2 {
    pub use crate::RW as access;
    #[doc = "Bit Clock Divide"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit Clock Bypass"]
    pub mod BYP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal bit clock is generated from bit clock divider."]
            pub const DISABLE: u32 = 0;
            #[doc = "Internal bit clock is divide-by-one of the audio master clock."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit Clock Direction"]
    pub mod BCD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bit clock is generated externally in Slave mode."]
            pub const EXT_IN_SLAVE: u32 = 0;
            #[doc = "Bit clock is generated internally in Master mode."]
            pub const INT_IN_MASTER: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit Clock Polarity"]
    pub mod BCP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MCLK Select"]
    pub mod MSEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bus Clock selected."]
            pub const BUS_CLOCK: u32 = 0;
            #[doc = "Master Clock (MCLK) 1 option selected."]
            pub const MCLK1: u32 = 0x01;
            #[doc = "Master Clock (MCLK) 2 option selected."]
            pub const MCLK2: u32 = 0x02;
            #[doc = "Master Clock (MCLK) 3 option selected."]
            pub const MCLK3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit Clock Input"]
    pub mod BCI {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect."]
            pub const DISABLE: u32 = 0;
            #[doc = "Internal logic is clocked as if bit clock was externally generated."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit Clock Swap"]
    pub mod BCS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use the normal bit clock source."]
            pub const DISABLE: u32 = 0;
            #[doc = "Swap the bit clock source."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Synchronous Mode"]
    pub mod SYNC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Asynchronous mode"]
            pub const ASYNC: u32 = 0;
            #[doc = "Synchronous with receiver"]
            pub const SYNC_W_RX: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Configuration 3"]
pub mod TCR3 {
    pub use crate::RW as access;
    #[doc = "Word Flag Configuration"]
    pub mod WDFL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit Channel Enable"]
    pub mod TCE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel FIFO Reset"]
    pub mod CFR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Configuration 4"]
pub mod TCR4 {
    pub use crate::RW as access;
    #[doc = "Frame Sync Direction"]
    pub mod FSD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Frame sync is generated externally in Slave mode."]
            pub const EXT_IN_SLAVE_MODE: u32 = 0;
            #[doc = "Frame sync is generated internally in Master mode."]
            pub const INT_IN_MASTER_MODE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame Sync Polarity"]
    pub mod FSP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Frame sync is active high."]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Frame sync is active low."]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "On Demand Mode"]
    pub mod ONDEM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal frame sync is generated continuously."]
            pub const CONTINUOUS_FRAME_SYNC: u32 = 0;
            #[doc = "Internal frame sync is generated when the FIFO warning flag is 0."]
            pub const ON_DEMAND_FRAME_SYNC: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame Sync Early"]
    pub mod FSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Frame sync asserts with the first bit of the frame."]
            pub const DISABLE: u32 = 0;
            #[doc = "Frame sync asserts one bit before the first bit of the frame."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MSB First"]
    pub mod MF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LSB is transmitted first."]
            pub const DISABLE: u32 = 0;
            #[doc = "MSB is transmitted first."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel Mode"]
    pub mod CHMOD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TDM mode, transmit data pins are 3-stated when slots are masked or channels are disabled."]
            pub const TDM_MODE: u32 = 0;
            #[doc = "Output mode, transmit data pins are never 3-stated and output zero when slots are masked or channels are disabled."]
            pub const OUTPUT_MODE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sync Width"]
    pub mod SYWD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame size"]
    pub mod FRSZ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Packing Mode"]
    pub mod FPACK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO packing is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "8-bit FIFO packing is enabled."]
            pub const EIGHT_BIT_FIFO_PACKING: u32 = 0x02;
            #[doc = "16-bit FIFO packing is enabled."]
            pub const SIXTEEN_BIT_FIFO_PACKING: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Combine Mode"]
    pub mod FCOMB {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO Combine mode disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "FIFO Combine mode enabled on FIFO reads (from transmit shift registers)."]
            pub const ENABLED_ON_FIFO_READS: u32 = 0x01;
            #[doc = "FIFO Combine mode enabled on FIFO writes (by software)."]
            pub const ENABLED_ON_FIFO_WRITES: u32 = 0x02;
            #[doc = "FIFO Combine mode enabled on FIFO reads (from transmit shift registers) and writes (by software)."]
            pub const ENABLED_ON_FIFO_READS_WRITES: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Continue on Error"]
    pub mod FCONT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "On FIFO error, SAI continues from the start of the next frame after the FIFO error flag has been cleared."]
            pub const DISABLE: u32 = 0;
            #[doc = "On FIFO error, SAI continues from the same word that caused the FIFO error to become 1 after the FIFO warning flag returns to 0."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Configuration 5"]
pub mod TCR5 {
    pub use crate::RW as access;
    #[doc = "First Bit Shifted"]
    pub mod FBT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bit index is 0."]
            pub const INDEX0: u32 = 0;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_1: u32 = 0x01;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_2: u32 = 0x02;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_3: u32 = 0x03;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_4: u32 = 0x04;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_5: u32 = 0x05;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_6: u32 = 0x06;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_7: u32 = 0x07;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_8: u32 = 0x08;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_9: u32 = 0x09;
            #[doc = "Bit index is 31."]
            pub const INDEX31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Word 0 Width"]
    pub mod W0W {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "8 bits per word"]
            pub const EIGHT: u32 = 0x07;
            #[doc = "9 bits per word"]
            pub const NINE: u32 = 0x08;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_9: u32 = 0x09;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_10: u32 = 0x0a;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_11: u32 = 0x0b;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_12: u32 = 0x0c;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_13: u32 = 0x0d;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_14: u32 = 0x0e;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_15: u32 = 0x0f;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_16: u32 = 0x10;
            #[doc = "32 bits per word"]
            pub const MAX: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Word N Width"]
    pub mod WNW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "8 bits per word"]
            pub const EIGHT: u32 = 0x07;
            #[doc = "9 bits per word"]
            pub const NINE: u32 = 0x08;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_9: u32 = 0x09;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_10: u32 = 0x0a;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_11: u32 = 0x0b;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_12: u32 = 0x0c;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_13: u32 = 0x0d;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_14: u32 = 0x0e;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_15: u32 = 0x0f;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_16: u32 = 0x10;
            #[doc = "32 bits per word"]
            pub const MAX: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Data"]
pub mod TDR {
    pub use crate::RW as access;
    #[doc = "Transmit Data Register"]
    pub mod TDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit FIFO"]
pub mod TFR {
    pub use crate::RO as access;
    #[doc = "Read FIFO Pointer"]
    pub mod RFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write FIFO Pointer"]
    pub mod WFP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write Channel Pointer"]
    pub mod WCP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect"]
            pub const DISABLE: u32 = 0;
            #[doc = "FIFO Combine mode is enabled for FIFO writes and this FIFO will be written on the next FIFO write."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Mask"]
pub mod TMR {
    pub use crate::RW as access;
    #[doc = "Transmit Word Mask"]
    pub mod TWM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Word N is enabled."]
            pub const WORD_N_ENABLED: u32 = 0;
            #[doc = "Word N is masked. The transmit data pins are 3-stated or drive zero when masked."]
            pub const WORD_N_MASKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Control"]
pub mod RCSR {
    pub use crate::RW as access;
    #[doc = "FIFO Request DMA Enable"]
    pub mod FRDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables the DMA request."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the DMA request."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Warning DMA Enable"]
    pub mod FWDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables DMA warnings."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables DMA warnings."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Request Interrupt Enable"]
    pub mod FRIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables the interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    pub mod FWIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables the interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables the interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sync Error Interrupt Enable"]
    pub mod SEIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables interrupt."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Word Start Interrupt Enable"]
    pub mod WSIE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disables interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables interrupt."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Request Flag"]
    pub mod FRF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Receive FIFO watermark not reached."]
            pub const BELOW_WATERMARK: u32 = 0;
            #[doc = "Receive FIFO watermark has been reached."]
            pub const WATERMARK_REACHED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Warning Flag"]
    pub mod FWF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No enabled receive FIFO is full."]
            pub const NOT_FULL: u32 = 0;
            #[doc = "Enabled receive FIFO is full."]
            pub const FULL: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Receive overflow not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Receive overflow detected."]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sync Error Flag"]
    pub mod SEF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Sync error not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Frame sync error detected."]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Word Start Flag"]
    pub mod WSF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Start of word not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Start of word detected."]
            pub const FLAG: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software Reset"]
    pub mod SR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Software reset."]
            pub const SW_RESET: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Reset"]
    pub mod FR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "FIFO reset."]
            pub const FIFO_RESET: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit Clock Enable"]
    pub mod BCE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Receive bit clock is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive bit clock is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Debug Enable"]
    pub mod DBGE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Receiver is disabled in Debug mode, after completing the current frame."]
            pub const DISABLE: u32 = 0;
            #[doc = "Receiver is enabled in Debug mode."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Stop Enable"]
    pub mod STOPE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Receiver disabled in Stop mode."]
            pub const DISABLE: u32 = 0;
            #[doc = "Receiver enabled in Stop mode."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receiver Enable"]
    pub mod RE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Receiver is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Configuration 1"]
pub mod RCR1 {
    pub use crate::RW as access;
    #[doc = "Receive FIFO Watermark"]
    pub mod RFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1 FIFO word"]
            pub const MIN: u32 = 0;
            #[doc = "2 FIFO words"]
            pub const TWO: u32 = 0x01;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_2: u32 = 0x02;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_3: u32 = 0x03;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_4: u32 = 0x04;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_5: u32 = 0x05;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_6: u32 = 0x06;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_7: u32 = 0x07;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_8: u32 = 0x08;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_9: u32 = 0x09;
            #[doc = "16 FIFO words"]
            pub const MAX: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Configuration 2"]
pub mod RCR2 {
    pub use crate::RW as access;
    #[doc = "Bit Clock Divide"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit Clock Bypass"]
    pub mod BYP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal bit clock is generated from bit clock divider."]
            pub const DISABLE: u32 = 0;
            #[doc = "Internal bit clock is divide-by-one of the audio master clock."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit Clock Direction"]
    pub mod BCD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bit clock is generated externally in Slave mode."]
            pub const EXT_SLAVE_MODE: u32 = 0;
            #[doc = "Bit clock is generated internally in Master mode."]
            pub const INT_MASTER_MODE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit Clock Polarity"]
    pub mod BCP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MCLK Select"]
    pub mod MSEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bus Clock selected."]
            pub const BUS_CLOCK: u32 = 0;
            #[doc = "Master Clock (MCLK) 1 option selected."]
            pub const MCLK1: u32 = 0x01;
            #[doc = "Master Clock (MCLK) 2 option selected."]
            pub const MCLK2: u32 = 0x02;
            #[doc = "Master Clock (MCLK) 3 option selected."]
            pub const MCLK3: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit Clock Input"]
    pub mod BCI {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Internal logic is clocked as if bit clock was externally generated."]
            pub const CLOCKED_AS_IF_EXT_GENERATED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit Clock Swap"]
    pub mod BCS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use the normal bit clock source."]
            pub const NORMAL: u32 = 0;
            #[doc = "Swap the bit clock source."]
            pub const SWAP_BIT_CLK_SOURCE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Synchronous Mode"]
    pub mod SYNC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Asynchronous mode"]
            pub const ASYNC: u32 = 0;
            #[doc = "Synchronous with transmitter"]
            pub const SYNC_W_TX: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Configuration 3"]
pub mod RCR3 {
    pub use crate::RW as access;
    #[doc = "Word Flag Configuration"]
    pub mod WDFL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Word 1"]
            pub const WORD_1: u32 = 0;
            #[doc = "Word 2"]
            pub const WORD_2: u32 = 0x01;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_2: u32 = 0x02;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_3: u32 = 0x03;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_4: u32 = 0x04;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_5: u32 = 0x05;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_6: u32 = 0x06;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_7: u32 = 0x07;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_8: u32 = 0x08;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_9: u32 = 0x09;
            #[doc = "Word 32"]
            pub const WORD_MAX: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive Channel Enable"]
    pub mod RCE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Channel FIFO Reset"]
    pub mod CFR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Configuration 4"]
pub mod RCR4 {
    pub use crate::RW as access;
    #[doc = "Frame Sync Direction"]
    pub mod FSD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Frame Sync is generated externally in Slave mode."]
            pub const EXT_SLAVE_MODE: u32 = 0;
            #[doc = "Frame Sync is generated internally in Master mode."]
            pub const INT_MASTER_MODE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame Sync Polarity"]
    pub mod FSP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Frame sync is active high."]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Frame sync is active low."]
            pub const ACTIVE_LOW: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "On Demand Mode"]
    pub mod ONDEM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Internal frame sync is generated continuously."]
            pub const DISABLE: u32 = 0;
            #[doc = "Internal frame sync is generated when the FIFO warning flag is 0."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame Sync Early"]
    pub mod FSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Frame sync asserts with the first bit of the frame."]
            pub const DISABLE: u32 = 0;
            #[doc = "Frame sync asserts one bit before the first bit of the frame."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MSB First"]
    pub mod MF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "LSB is received first."]
            pub const DISABLE: u32 = 0;
            #[doc = "MSB is received first."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sync Width"]
    pub mod SYWD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1 bit-clock cycle"]
            pub const MIN: u32 = 0;
            #[doc = "2 bit-clock cycle"]
            pub const TWO_CLOCKS: u32 = 0x01;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_2: u32 = 0x02;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_3: u32 = 0x03;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_4: u32 = 0x04;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_5: u32 = 0x05;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_6: u32 = 0x06;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_7: u32 = 0x07;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_8: u32 = 0x08;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_9: u32 = 0x09;
            #[doc = "32 bit-clock cycle"]
            pub const THIRTYTWO_CLOCKS: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame Size"]
    pub mod FRSZ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1 word per frame"]
            pub const ONE_WORD: u32 = 0;
            #[doc = "2 words per frame"]
            pub const TWO_WORDS: u32 = 0x01;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_2: u32 = 0x02;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_3: u32 = 0x03;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_4: u32 = 0x04;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_5: u32 = 0x05;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_6: u32 = 0x06;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_7: u32 = 0x07;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_8: u32 = 0x08;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_9: u32 = 0x09;
            #[doc = "32 words per frame"]
            pub const MAX_WORDS: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Packing Mode"]
    pub mod FPACK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO packing is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "8-bit FIFO packing is enabled"]
            pub const EIGHT_BIT_PACKING: u32 = 0x02;
            #[doc = "16-bit FIFO packing is enabled"]
            pub const SIXTEEN_BIT_PACKING: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Combine Mode"]
    pub mod FCOMB {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "FIFO Combine mode disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "FIFO Combine mode enabled on FIFO writes (from receive shift registers)."]
            pub const ENA_ON_FIFO_WRITES: u32 = 0x01;
            #[doc = "FIFO Combine mode enabled on FIFO reads (by software)."]
            pub const ENA_ON_FIFO_READS: u32 = 0x02;
            #[doc = "FIFO Combine mode enabled on FIFO writes (from receive shift registers) and reads (by software)."]
            pub const ENA_ON_FIFO_WRITES_READS: u32 = 0x03;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "FIFO Continue on Error"]
    pub mod FCONT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "On FIFO error, SAI continues from the start of the next frame after the FIFO error flag returns to 0."]
            pub const DISABLE: u32 = 0;
            #[doc = "On FIFO error, SAI continues from the same word that caused the FIFO error to become 1 after the FIFO warning flag returns to 0."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Configuration 5"]
pub mod RCR5 {
    pub use crate::RW as access;
    #[doc = "First Bit Shifted"]
    pub mod FBT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Bit index is 0."]
            pub const INDEX0: u32 = 0;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_1: u32 = 0x01;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_2: u32 = 0x02;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_3: u32 = 0x03;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_4: u32 = 0x04;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_5: u32 = 0x05;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_6: u32 = 0x06;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_7: u32 = 0x07;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_8: u32 = 0x08;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_9: u32 = 0x09;
            #[doc = "Bit index is 31."]
            pub const INDEX31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Word 0 Width"]
    pub mod W0W {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1 bit per word"]
            pub const MIN: u32 = 0;
            #[doc = "2 bits per word"]
            pub const TWO: u32 = 0x01;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_2: u32 = 0x02;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_3: u32 = 0x03;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_4: u32 = 0x04;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_5: u32 = 0x05;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_6: u32 = 0x06;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_7: u32 = 0x07;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_8: u32 = 0x08;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_9: u32 = 0x09;
            #[doc = "32 bits per word"]
            pub const MAX: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Word N Width"]
    pub mod WNW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "8 bits per word"]
            pub const EIGHT: u32 = 0x07;
            #[doc = "9 bits per word"]
            pub const NINE: u32 = 0x08;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_9: u32 = 0x09;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_10: u32 = 0x0a;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_11: u32 = 0x0b;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_12: u32 = 0x0c;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_13: u32 = 0x0d;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_14: u32 = 0x0e;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_15: u32 = 0x0f;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_16: u32 = 0x10;
            #[doc = "32 bits per word"]
            pub const MAX: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Data"]
pub mod RDR {
    pub use crate::RO as access;
    #[doc = "Receive Data Register"]
    pub mod RDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive FIFO"]
pub mod RFR {
    pub use crate::RO as access;
    #[doc = "Read FIFO Pointer"]
    pub mod RFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive Channel Pointer"]
    pub mod RCP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect."]
            pub const DISABLE: u32 = 0;
            #[doc = "FIFO Combine mode is enabled for FIFO reads and this FIFO will be read on the next FIFO read."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write FIFO Pointer"]
    pub mod WFP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Mask"]
pub mod RMR {
    pub use crate::RW as access;
    #[doc = "Receive Word Mask"]
    pub mod RWM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Word N is enabled."]
            pub const WORD_N_ENABLED: u32 = 0;
            #[doc = "Word N is masked."]
            pub const WORD_N_MASKED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
