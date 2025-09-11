#[doc = "NETC global"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Shared memory capability register"]
    pub SMCAPR: u32,
    #[doc = "Shared memory depletion threshold register"]
    pub SMDTR: u32,
    #[doc = "Shared memory available count register"]
    pub SMACR: u32,
    _reserved0: [u8; 0x04],
    #[doc = "Shared memory count low watermark register"]
    pub SMCLWMR: u32,
    #[doc = "Shared memory buffer unassigned count register"]
    pub SMBUCR: u32,
    #[doc = "Shared memory buffer unassigned count high watermark register"]
    pub SMBUCHWMR: u32,
    #[doc = "Shared memory loss count register"]
    pub SMLCR: u32,
    #[doc = "Hash bucket table capability register"]
    pub HBTCAPR: u32,
    #[doc = "Hash bucket table operational register 0"]
    pub HBTOR0: u32,
    _reserved1: [u8; 0x04],
    #[doc = "Hash bucket table operational register 2"]
    pub HBTOR2: u32,
    _reserved2: [u8; 0x10],
    #[doc = "Shared memory ENETC receive buffer capability register"]
    pub SMERBCAPR: u32,
    #[doc = "Shared memory ENETC receive buffer operational register 0"]
    pub SMERBOR0: u32,
    #[doc = "Shared memory ENETC receive buffer operational 1"]
    pub SMERBOR1: u32,
    _reserved3: [u8; 0xb4],
    #[doc = "PCE 0 operational register"]
    pub PCE0OR: u32,
    #[doc = "Replication Forwarding Engine 0 operational register"]
    pub RFE0OR: u32,
    _reserved4: [u8; 0x5c],
    #[doc = "NETC clock register"]
    pub NETCCLKR: u32,
    _reserved5: [u8; 0x98],
    #[doc = "HTA 0 capability register"]
    pub HTA0CAPR: u32,
    #[doc = "HTA 0 receive frame count operational register"]
    pub HTA0RFCOR: u32,
    #[doc = "HTA 0 high priority byte count operational register"]
    pub HTA0HPBCOR: u32,
    #[doc = "HTA 0 low priority byte count operational register"]
    pub HTA0LPBCOR: u32,
    _reserved6: [u8; 0x14],
    #[doc = "HTA 0 transmit frame count operational register"]
    pub HTA0TFCOR: u32,
    _reserved7: [u8; 0xd8],
    #[doc = "Root complex 0 system bus read latency average register"]
    pub RC0SBRLAR: u32,
    #[doc = "Root complex 0 system bus read latency high watermark register"]
    pub RC0SBRLHWMR: u32,
    #[doc = "Root complex 0 system bus write latency average register"]
    pub RC0SBWLAR: u32,
    #[doc = "Root complex 0 system bus write latency high watermark register"]
    pub RC0SBWLHWMR: u32,
    _reserved8: [u8; 0x08e8],
    #[doc = "IP block revision register 0"]
    pub IPBRR0: u32,
    #[doc = "IP block revision register 1"]
    pub IPBRR1: u32,
    _reserved9: [u8; 0x0100],
    #[doc = "Function boot loader parameter register a"]
    pub FBLPR: [u32; 2usize],
    _reserved10: [u8; 0x0118],
    #[doc = "EMDIO uncorrectable fatal system bus error configuration register"]
    pub EMDIOUFSBECR: u32,
    #[doc = "EMDIO uncorrectable fatal system bus error status register"]
    pub EMDIOUFSBESR: u32,
}
#[doc = "Shared memory capability register"]
pub mod SMCAPR {
    pub use crate::RO as access;
    #[doc = "Total amount of words in common memory available for free list to NETC buffers, frame descriptors and hash tables"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Shared memory depletion threshold register"]
pub mod SMDTR {
    pub use crate::RO as access;
    #[doc = "Shared memory depletion threshold in Words"]
    pub mod THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Shared memory available count register"]
pub mod SMACR {
    pub use crate::RO as access;
    #[doc = "Shows the current amount of available shared memory in words"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Shared memory count low watermark register"]
pub mod SMCLWMR {
    pub use crate::RO as access;
    #[doc = "Shows the low watermark for shared memory in words since the last read of this register"]
    pub mod WATERMARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Shared memory buffer unassigned count register"]
pub mod SMBUCR {
    pub use crate::RO as access;
    #[doc = "Shows the current amount of unassigned Shared memory buffer, in words"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Shared memory buffer unassigned count high watermark register"]
pub mod SMBUCHWMR {
    pub use crate::RO as access;
    #[doc = "Shows the high watermark for unassigned memory since the last read of this register, in words"]
    pub mod WATERMARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Shared memory loss count register"]
pub mod SMLCR {
    pub use crate::RO as access;
    #[doc = "Determinate number of lost words"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indeterminate free list corruption Number of words lost due to free list corruption"]
    pub mod IFLC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indeterminate frame descriptor corruption Number of words lost due to frame's key meta data (reference count or number of IMUs) corruption"]
    pub mod IFDC {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Hash bucket table capability register"]
pub mod HBTCAPR {
    pub use crate::RO as access;
    #[doc = "Specifies the total number of allocate bucket entries for use."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Specifies the maximum exact match hash collisions chain allowed"]
    pub mod MAX_COL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Specifies the maximum number of hash entries visited during a table management search command"]
    pub mod MAX_VISITS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Hash bucket table operational register 0"]
pub mod HBTOR0 {
    pub use crate::RO as access;
    #[doc = "Specifies the amount of entries used"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Specifies the high watermark of used buckets Register is reset to NUM_ENTRIES after read."]
    pub mod HWM_ENTRIES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Hash bucket table operational register 2"]
pub mod HBTOR2 {
    pub use crate::RO as access;
    #[doc = "The fractional portion of the running average length of hash lookup"]
    pub mod RUN_AVG_FRACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The integer portion of the running average length of hash lookup"]
    pub mod RUN_AVG_INT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Length of the longest hash collision chain noticed since the last read. Range: 0..8"]
    pub mod HWM_COL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Shared memory ENETC receive buffer capability register"]
pub mod SMERBCAPR {
    pub use crate::RO as access;
    #[doc = "Threshold in words of receive buffer memory used by all ENETC functions"]
    pub mod THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Word size in bytes. 0: 24 Bytes 1-3: reserved"]
    pub mod WORD_SIZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates memory location 0: Common memory 1-3: Reserved"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Shared memory ENETC receive buffer operational register 0"]
pub mod SMERBOR0 {
    pub use crate::RO as access;
    #[doc = "Number of words in use for buffers and frame descriptors."]
    pub mod AMOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Shared memory ENETC receive buffer operational 1"]
pub mod SMERBOR1 {
    pub use crate::RO as access;
    #[doc = "High watermark of words in use by buffers and frame descriptors since the last read"]
    pub mod WATERMARK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PCE 0 operational register"]
pub mod PCE0OR {
    pub use crate::RO as access;
    #[doc = "Number of active frames currently being processed by the Parse Classifier Engine."]
    pub mod NUM_FRAMES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "High watermark of concurrent frames being processed since the last read of this register"]
    pub mod HWM_FRAMES {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Maximum number of concurrent frames that can be processed by the PCE block"]
    pub mod MAX_FRAMES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Replication Forwarding Engine 0 operational register"]
pub mod RFE0OR {
    pub use crate::RO as access;
    #[doc = "Number of active frames currently being processed by the Replication and Forwarding Engine."]
    pub mod NUM_FRAMES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "High watermark of concurrent frames being processed since the last read of this register"]
    pub mod HWM_FRAMES {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Maximum number of concurrent frames that can be processed by the RFE block"]
    pub mod MAX_FRAMES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "NETC clock register"]
pub mod NETCCLKR {
    pub use crate::RO as access;
    #[doc = "Frequency"]
    pub mod FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "HTA 0 capability register"]
pub mod HTA0CAPR {
    pub use crate::RO as access;
    #[doc = "Maximum number of Rx frames the HTA hardware block can process concurrently."]
    pub mod MAX_RX_FRAMES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Maximum number of Tx frames the HTA hardware block can process concurrently."]
    pub mod MAX_TX_FRAMES {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "HTA 0 receive frame count operational register"]
pub mod HTA0RFCOR {
    pub use crate::RO as access;
    #[doc = "Number of active high priority tier Rx frames currently being processed by the host transfer engine"]
    pub mod HP_COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "High watermark of concurrent Rx frames being processed by HTA since the last read of this register"]
    pub mod HP_HWM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of active low priority tier Rx frames currently being processed by the host transfer engine."]
    pub mod LP_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Low watermark of concurrent Rx frames being processed since the last read of this register"]
    pub mod LP_HWM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "HTA 0 high priority byte count operational register"]
pub mod HTA0HPBCOR {
    pub use crate::RO as access;
    #[doc = "Amount of DMA bytes in progress for all high priority tier HTA threads."]
    pub mod HP_COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "High watermark of concurrent frames being processed since the last read of this register"]
    pub mod HWM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "HTA 0 low priority byte count operational register"]
pub mod HTA0LPBCOR {
    pub use crate::RO as access;
    #[doc = "Amount of DMA bytes in progress for all low priority tier HTA threads."]
    pub mod LP_COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "High watermark of concurrent frames being processed since the last read of this register"]
    pub mod HWM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "HTA 0 transmit frame count operational register"]
pub mod HTA0TFCOR {
    pub use crate::RO as access;
    #[doc = "Number of active high priority tier Tx frames currently being processed by the host transfer engine"]
    pub mod HP_COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "High watermark of concurrent Tx frames being processed by HTA since the last read of this register"]
    pub mod HP_HWM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of active low priority tier Tx frames currently being processed by the host transfer engine."]
    pub mod LP_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Low watermark of concurrent Tx frames being processed since the last read of this register"]
    pub mod LP_HWM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Root complex 0 system bus read latency average register"]
pub mod RC0SBRLAR {
    pub use crate::RW as access;
    #[doc = "Fractional portion of the latency value."]
    pub mod FRACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Integer portion of the latency value."]
    pub mod INT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Root complex 0 system bus read latency high watermark register"]
pub mod RC0SBRLHWMR {
    pub use crate::RO as access;
    #[doc = "Fractional portion of the latency value."]
    pub mod FRACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Integer portion of the latency value."]
    pub mod INT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Root complex 0 system bus write latency average register"]
pub mod RC0SBWLAR {
    pub use crate::RW as access;
    #[doc = "Fractional portion of the latency value."]
    pub mod FRACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Integer portion of the latency value."]
    pub mod INT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Root complex 0 system bus write latency high watermark register"]
pub mod RC0SBWLHWMR {
    pub use crate::RO as access;
    #[doc = "Fractional portion of the latency value."]
    pub mod FRACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Integer portion of the latency value."]
    pub mod INT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IP block revision register 0"]
pub mod IPBRR0 {
    pub use crate::RO as access;
    #[doc = "Minor revision"]
    pub mod IP_MN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Major revision"]
    pub mod IP_MJ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IP block ID"]
    pub mod IP_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "IP block revision register 1"]
pub mod IPBRR1 {
    pub use crate::RO as access;
    #[doc = "IP block configuration options"]
    pub mod IP_CFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IP block maintenance version"]
    pub mod IP_MNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IP block integration options Bit 23-16: Reserved"]
    pub mod IP_INT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Function boot loader parameter register a"]
pub mod FBLPR {
    pub use crate::RO as access;
    #[doc = "Boot loader parameter value"]
    pub mod PARAM_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "EMDIO uncorrectable fatal system bus error configuration register"]
pub mod EMDIOUFSBECR {
    pub use crate::RW as access;
    #[doc = "Report disable"]
    pub mod RD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "EMDIO uncorrectable fatal system bus error status register"]
pub mod EMDIOUFSBESR {
    pub use crate::RW as access;
    #[doc = "System Bus ID"]
    pub mod SB_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Multiple"]
    pub mod M {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "System bus error"]
    pub mod SBE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
