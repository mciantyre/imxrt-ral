#[doc = "Ethernet MAC-NET Core"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "Interrupt Event Register"]
    pub EIR: u32,
    #[doc = "Interrupt Mask Register"]
    pub EIMR: u32,
    _reserved1: [u8; 0x04],
    #[doc = "Receive Descriptor Active Register"]
    pub RDAR: u32,
    #[doc = "Transmit Descriptor Active Register"]
    pub TDAR: u32,
    _reserved2: [u8; 0x0c],
    #[doc = "Ethernet Control Register"]
    pub ECR: u32,
    _reserved3: [u8; 0x18],
    #[doc = "MII Management Frame Register"]
    pub MMFR: u32,
    #[doc = "MII Speed Control Register"]
    pub MSCR: u32,
    _reserved4: [u8; 0x1c],
    #[doc = "MIB Control Register"]
    pub MIBC: u32,
    _reserved5: [u8; 0x1c],
    #[doc = "Receive Control Register"]
    pub RCR: u32,
    _reserved6: [u8; 0x3c],
    #[doc = "Transmit Control Register"]
    pub TCR: u32,
    _reserved7: [u8; 0x1c],
    #[doc = "Physical Address Lower Register"]
    pub PALR: u32,
    #[doc = "Physical Address Upper Register"]
    pub PAUR: u32,
    #[doc = "Opcode/Pause Duration Register"]
    pub OPD: u32,
    #[doc = "Transmit Interrupt Coalescing Register"]
    pub TXIC: u32,
    _reserved8: [u8; 0x0c],
    #[doc = "Receive Interrupt Coalescing Register"]
    pub RXIC: u32,
    _reserved9: [u8; 0x14],
    #[doc = "Descriptor Individual Upper Address Register"]
    pub IAUR: u32,
    #[doc = "Descriptor Individual Lower Address Register"]
    pub IALR: u32,
    #[doc = "Descriptor Group Upper Address Register"]
    pub GAUR: u32,
    #[doc = "Descriptor Group Lower Address Register"]
    pub GALR: u32,
    _reserved10: [u8; 0x1c],
    #[doc = "Transmit FIFO Watermark Register"]
    pub TFWR: u32,
    _reserved11: [u8; 0x38],
    #[doc = "Receive Descriptor Ring Start Register"]
    pub RDSR: u32,
    #[doc = "Transmit Buffer Descriptor Ring Start Register"]
    pub TDSR: u32,
    #[doc = "Maximum Receive Buffer Size Register"]
    pub MRBR: u32,
    _reserved12: [u8; 0x04],
    #[doc = "Receive FIFO Section Full Threshold"]
    pub RSFL: u32,
    #[doc = "Receive FIFO Section Empty Threshold"]
    pub RSEM: u32,
    #[doc = "Receive FIFO Almost Empty Threshold"]
    pub RAEM: u32,
    #[doc = "Receive FIFO Almost Full Threshold"]
    pub RAFL: u32,
    #[doc = "Transmit FIFO Section Empty Threshold"]
    pub TSEM: u32,
    #[doc = "Transmit FIFO Almost Empty Threshold"]
    pub TAEM: u32,
    #[doc = "Transmit FIFO Almost Full Threshold"]
    pub TAFL: u32,
    #[doc = "Transmit Inter-Packet Gap"]
    pub TIPG: u32,
    #[doc = "Frame Truncation Length"]
    pub FTRL: u32,
    _reserved13: [u8; 0x0c],
    #[doc = "Transmit Accelerator Function Configuration"]
    pub TACC: u32,
    #[doc = "Receive Accelerator Function Configuration"]
    pub RACC: u32,
    _reserved14: [u8; 0x38],
    #[doc = "Reserved Statistic Register"]
    pub RMON_T_DROP: u32,
    #[doc = "Tx Packet Count Statistic Register"]
    pub RMON_T_PACKETS: u32,
    #[doc = "Tx Broadcast Packets Statistic Register"]
    pub RMON_T_BC_PKT: u32,
    #[doc = "Tx Multicast Packets Statistic Register"]
    pub RMON_T_MC_PKT: u32,
    #[doc = "Tx Packets with CRC/Align Error Statistic Register"]
    pub RMON_T_CRC_ALIGN: u32,
    #[doc = "Tx Packets Less Than Bytes and Good CRC Statistic Register"]
    pub RMON_T_UNDERSIZE: u32,
    #[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
    pub RMON_T_OVERSIZE: u32,
    #[doc = "Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
    pub RMON_T_FRAG: u32,
    #[doc = "Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
    pub RMON_T_JAB: u32,
    #[doc = "Tx Collision Count Statistic Register"]
    pub RMON_T_COL: u32,
    #[doc = "Tx 64-Byte Packets Statistic Register"]
    pub RMON_T_P64: u32,
    #[doc = "Tx 65- to 127-byte Packets Statistic Register"]
    pub RMON_T_P65TO127: u32,
    #[doc = "Tx 128- to 255-byte Packets Statistic Register"]
    pub RMON_T_P128TO255: u32,
    #[doc = "Tx 256- to 511-byte Packets Statistic Register"]
    pub RMON_T_P256TO511: u32,
    #[doc = "Tx 512- to 1023-byte Packets Statistic Register"]
    pub RMON_T_P512TO1023: u32,
    #[doc = "Tx 1024- to 2047-byte Packets Statistic Register"]
    pub RMON_T_P1024TO2047: u32,
    #[doc = "Tx Packets Greater Than 2048 Bytes Statistic Register"]
    pub RMON_T_P_GTE2048: u32,
    #[doc = "Tx Octets Statistic Register"]
    pub RMON_T_OCTETS: u32,
    #[doc = "Reserved Statistic Register"]
    pub IEEE_T_DROP: u32,
    #[doc = "Frames Transmitted OK Statistic Register"]
    pub IEEE_T_FRAME_OK: u32,
    #[doc = "Frames Transmitted with Single Collision Statistic Register"]
    pub IEEE_T_1COL: u32,
    #[doc = "Frames Transmitted with Multiple Collisions Statistic Register"]
    pub IEEE_T_MCOL: u32,
    #[doc = "Frames Transmitted after Deferral Delay Statistic Register"]
    pub IEEE_T_DEF: u32,
    #[doc = "Frames Transmitted with Late Collision Statistic Register"]
    pub IEEE_T_LCOL: u32,
    #[doc = "Frames Transmitted with Excessive Collisions Statistic Register"]
    pub IEEE_T_EXCOL: u32,
    #[doc = "Frames Transmitted with Tx FIFO Underrun Statistic Register"]
    pub IEEE_T_MACERR: u32,
    #[doc = "Frames Transmitted with Carrier Sense Error Statistic Register"]
    pub IEEE_T_CSERR: u32,
    #[doc = "Reserved Statistic Register"]
    pub IEEE_T_SQE: u32,
    #[doc = "Flow Control Pause Frames Transmitted Statistic Register"]
    pub IEEE_T_FDXFC: u32,
    #[doc = "Octet Count for Frames Transmitted w/o Error Statistic Register"]
    pub IEEE_T_OCTETS_OK: u32,
    _reserved15: [u8; 0x0c],
    #[doc = "Rx Packet Count Statistic Register"]
    pub RMON_R_PACKETS: u32,
    #[doc = "Rx Broadcast Packets Statistic Register"]
    pub RMON_R_BC_PKT: u32,
    #[doc = "Rx Multicast Packets Statistic Register"]
    pub RMON_R_MC_PKT: u32,
    #[doc = "Rx Packets with CRC/Align Error Statistic Register"]
    pub RMON_R_CRC_ALIGN: u32,
    #[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
    pub RMON_R_UNDERSIZE: u32,
    #[doc = "Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
    pub RMON_R_OVERSIZE: u32,
    #[doc = "Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
    pub RMON_R_FRAG: u32,
    #[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
    pub RMON_R_JAB: u32,
    #[doc = "Reserved Statistic Register"]
    pub RMON_R_RESVD_0: u32,
    #[doc = "Rx 64-Byte Packets Statistic Register"]
    pub RMON_R_P64: u32,
    #[doc = "Rx 65- to 127-Byte Packets Statistic Register"]
    pub RMON_R_P65TO127: u32,
    #[doc = "Rx 128- to 255-Byte Packets Statistic Register"]
    pub RMON_R_P128TO255: u32,
    #[doc = "Rx 256- to 511-Byte Packets Statistic Register"]
    pub RMON_R_P256TO511: u32,
    #[doc = "Rx 512- to 1023-Byte Packets Statistic Register"]
    pub RMON_R_P512TO1023: u32,
    #[doc = "Rx 1024- to 2047-Byte Packets Statistic Register"]
    pub RMON_R_P1024TO2047: u32,
    #[doc = "Rx Packets Greater than 2048 Bytes Statistic Register"]
    pub RMON_R_P_GTE2048: u32,
    #[doc = "Rx Octets Statistic Register"]
    pub RMON_R_OCTETS: u32,
    #[doc = "Frames not Counted Correctly Statistic Register"]
    pub IEEE_R_DROP: u32,
    #[doc = "Frames Received OK Statistic Register"]
    pub IEEE_R_FRAME_OK: u32,
    #[doc = "Frames Received with CRC Error Statistic Register"]
    pub IEEE_R_CRC: u32,
    #[doc = "Frames Received with Alignment Error Statistic Register"]
    pub IEEE_R_ALIGN: u32,
    #[doc = "Receive FIFO Overflow Count Statistic Register"]
    pub IEEE_R_MACERR: u32,
    #[doc = "Flow Control Pause Frames Received Statistic Register"]
    pub IEEE_R_FDXFC: u32,
    #[doc = "Octet Count for Frames Received without Error Statistic Register"]
    pub IEEE_R_OCTETS_OK: u32,
    _reserved16: [u8; 0x011c],
    #[doc = "Adjustable Timer Control Register"]
    pub ATCR: u32,
    #[doc = "Timer Value Register"]
    pub ATVR: u32,
    #[doc = "Timer Offset Register"]
    pub ATOFF: u32,
    #[doc = "Timer Period Register"]
    pub ATPER: u32,
    #[doc = "Timer Correction Register"]
    pub ATCOR: u32,
    #[doc = "Time-Stamping Clock Period Register"]
    pub ATINC: u32,
    #[doc = "Timestamp of Last Transmitted Frame"]
    pub ATSTMP: u32,
    _reserved17: [u8; 0x01e8],
    #[doc = "Timer Global Status Register"]
    pub TGSR: u32,
    #[doc = "Timer Control Status Register"]
    pub TCSR0: u32,
    #[doc = "Timer Compare Capture Register"]
    pub TCCR0: u32,
    #[doc = "Timer Control Status Register"]
    pub TCSR1: u32,
    #[doc = "Timer Compare Capture Register"]
    pub TCCR1: u32,
    #[doc = "Timer Control Status Register"]
    pub TCSR2: u32,
    #[doc = "Timer Compare Capture Register"]
    pub TCCR2: u32,
    #[doc = "Timer Control Status Register"]
    pub TCSR3: u32,
    #[doc = "Timer Compare Capture Register"]
    pub TCCR3: u32,
}
#[doc = "Interrupt Event Register"]
pub mod EIR {
    pub use crate::RW as access;
    #[doc = "Timestamp Timer"]
    pub mod TS_TIMER {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit Timestamp Available"]
    pub mod TS_AVAIL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Node Wakeup Request Indication"]
    pub mod WAKEUP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Receive Error"]
    pub mod PLR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit FIFO Underrun"]
    pub mod UN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Collision Retry Limit"]
    pub mod RL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Late Collision"]
    pub mod LC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ethernet Bus Error"]
    pub mod EBERR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MII Interrupt."]
    pub mod MII {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive Buffer Interrupt"]
    pub mod RXB {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive Frame Interrupt"]
    pub mod RXF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit Buffer Interrupt"]
    pub mod TXB {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit Frame Interrupt"]
    pub mod TXF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Graceful Stop Complete"]
    pub mod GRA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Babbling Transmit Error"]
    pub mod BABT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Babbling Receive Error"]
    pub mod BABR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Interrupt Mask Register"]
pub mod EIMR {
    pub use crate::RW as access;
    #[doc = "TS_TIMER Interrupt Mask"]
    pub mod TS_TIMER {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "TS_AVAIL Interrupt Mask"]
    pub mod TS_AVAIL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "WAKEUP Interrupt Mask"]
    pub mod WAKEUP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PLR Interrupt Mask"]
    pub mod PLR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "UN Interrupt Mask"]
    pub mod UN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RL Interrupt Mask"]
    pub mod RL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "LC Interrupt Mask"]
    pub mod LC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EBERR Interrupt Mask"]
    pub mod EBERR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MII Interrupt Mask"]
    pub mod MII {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RXB Interrupt Mask"]
    pub mod RXB {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RXF Interrupt Mask"]
    pub mod RXF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "TXB Interrupt Mask"]
    pub mod TXB {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The corresponding interrupt source is masked."]
            pub const TXB_0: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const TXB_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "TXF Interrupt Mask"]
    pub mod TXF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The corresponding interrupt source is masked."]
            pub const TXF_0: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const TXF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "GRA Interrupt Mask"]
    pub mod GRA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The corresponding interrupt source is masked."]
            pub const GRA_0: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const GRA_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BABT Interrupt Mask"]
    pub mod BABT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The corresponding interrupt source is masked."]
            pub const BABT_0: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const BABT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BABR Interrupt Mask"]
    pub mod BABR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The corresponding interrupt source is masked."]
            pub const BABR_0: u32 = 0;
            #[doc = "The corresponding interrupt source is not masked."]
            pub const BABR_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Descriptor Active Register"]
pub mod RDAR {
    pub use crate::RW as access;
    #[doc = "Receive Descriptor Active"]
    pub mod RDAR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Descriptor Active Register"]
pub mod TDAR {
    pub use crate::RW as access;
    #[doc = "Transmit Descriptor Active"]
    pub mod TDAR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ethernet Control Register"]
pub mod ECR {
    pub use crate::RW as access;
    #[doc = "Ethernet MAC Reset"]
    pub mod RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ethernet Enable"]
    pub mod ETHEREN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
            pub const ETHEREN_0: u32 = 0;
            #[doc = "MAC is enabled, and reception and transmission are possible."]
            pub const ETHEREN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Magic Packet Detection Enable"]
    pub mod MAGICEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Magic detection logic disabled."]
            pub const MAGICEN_0: u32 = 0;
            #[doc = "The MAC core detects magic packets and asserts EIR\\[WAKEUP\\] when a frame is detected."]
            pub const MAGICEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sleep Mode Enable"]
    pub mod SLEEP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Normal operating mode."]
            pub const SLEEP_0: u32 = 0;
            #[doc = "Sleep mode."]
            pub const SLEEP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EN1588 Enable"]
    pub mod EN1588 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Legacy FEC buffer descriptors and functions enabled."]
            pub const EN1588_0: u32 = 0;
            #[doc = "Enhanced frame time-stamping functions enabled."]
            pub const EN1588_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Debug Enable"]
    pub mod DBGEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "MAC continues operation in debug mode."]
            pub const DBGEN_0: u32 = 0;
            #[doc = "MAC enters hardware freeze mode when the processor is in debug mode."]
            pub const DBGEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Descriptor Byte Swapping Enable"]
    pub mod DBSWP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The buffer descriptor bytes are not swapped to support big-endian devices."]
            pub const DBSWP_0: u32 = 0;
            #[doc = "The buffer descriptor bytes are swapped to support little-endian devices."]
            pub const DBSWP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "MII Management Frame Register"]
pub mod MMFR {
    pub use crate::RW as access;
    #[doc = "Management Frame Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Turn Around"]
    pub mod TA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Register Address"]
    pub mod RA {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PHY Address"]
    pub mod PA {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Operation Code"]
    pub mod OP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Start Of Frame Delimiter"]
    pub mod ST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "MII Speed Control Register"]
pub mod MSCR {
    pub use crate::RW as access;
    #[doc = "MII Speed"]
    pub mod MII_SPEED {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable Preamble"]
    pub mod DIS_PRE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Preamble enabled."]
            pub const DIS_PRE_0: u32 = 0;
            #[doc = "Preamble (32 ones) is not prepended to the MII management frame."]
            pub const DIS_PRE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Hold time On MDIO Output"]
    pub mod HOLDTIME {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1 internal module clock cycle"]
            pub const HOLDTIME_0: u32 = 0;
            #[doc = "2 internal module clock cycles"]
            pub const HOLDTIME_1: u32 = 0x01;
            #[doc = "3 internal module clock cycles"]
            pub const HOLDTIME_2: u32 = 0x02;
            #[doc = "8 internal module clock cycles"]
            pub const HOLDTIME_7: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "MIB Control Register"]
pub mod MIBC {
    pub use crate::RW as access;
    #[doc = "MIB Clear"]
    pub mod MIB_CLEAR {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "See note above."]
            pub const MIB_CLEAR_0: u32 = 0;
            #[doc = "All statistics counters are reset to 0."]
            pub const MIB_CLEAR_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MIB Idle"]
    pub mod MIB_IDLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The MIB block is updating MIB counters."]
            pub const MIB_IDLE_0: u32 = 0;
            #[doc = "The MIB block is not currently updating any MIB counters."]
            pub const MIB_IDLE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable MIB Logic"]
    pub mod MIB_DIS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "MIB logic is enabled."]
            pub const MIB_DIS_0: u32 = 0;
            #[doc = "MIB logic is disabled. The MIB logic halts and does not update any MIB counters."]
            pub const MIB_DIS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Control Register"]
pub mod RCR {
    pub use crate::RW as access;
    #[doc = "Internal Loopback"]
    pub mod LOOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Loopback disabled."]
            pub const LOOP_0: u32 = 0;
            #[doc = "Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
            pub const LOOP_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Disable Receive On Transmit"]
    pub mod DRT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode."]
            pub const DRT_0: u32 = 0;
            #[doc = "Disable reception of frames while transmitting. (Normally used for half-duplex mode.)"]
            pub const DRT_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Media Independent Interface Mode"]
    pub mod MII_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "MII or RMII mode, as indicated by the RMII_MODE field."]
            pub const MII_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Promiscuous Mode"]
    pub mod PROM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled."]
            pub const PROM_0: u32 = 0;
            #[doc = "Enabled."]
            pub const PROM_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Broadcast Frame Reject"]
    pub mod BC_REJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Flow Control Enable"]
    pub mod FCE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RMII Mode Enable"]
    pub mod RMII_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "MAC configured for MII mode."]
            pub const RMII_MODE_0: u32 = 0;
            #[doc = "MAC configured for RMII operation."]
            pub const RMII_MODE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables 10-Mbit/s mode of the RMII ."]
    pub mod RMII_10T {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "100-Mbit/s operation."]
            pub const RMII_10T_0: u32 = 0;
            #[doc = "10-Mbit/s operation."]
            pub const RMII_10T_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Frame Padding Remove On Receive"]
    pub mod PADEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No padding is removed on receive by the MAC."]
            pub const PADEN_0: u32 = 0;
            #[doc = "Padding is removed from received frames."]
            pub const PADEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Terminate/Forward Pause Frames"]
    pub mod PAUFWD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pause frames are terminated and discarded in the MAC."]
            pub const PAUFWD_0: u32 = 0;
            #[doc = "Pause frames are forwarded to the user application."]
            pub const PAUFWD_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Terminate/Forward Received CRC"]
    pub mod CRCFWD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The CRC field of received frames is transmitted to the user application."]
            pub const CRCFWD_0: u32 = 0;
            #[doc = "The CRC field is stripped from the frame."]
            pub const CRCFWD_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MAC Control Frame Enable"]
    pub mod CFEN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
            pub const CFEN_0: u32 = 0;
            #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
            pub const CFEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Maximum Frame Length"]
    pub mod MAX_FL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Length Check Disable"]
    pub mod NLC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The payload length check is disabled."]
            pub const NLC_0: u32 = 0;
            #[doc = "The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\\[PLR\\] field."]
            pub const NLC_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Graceful Receive Stopped"]
    pub mod GRS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Control Register"]
pub mod TCR {
    pub use crate::RW as access;
    #[doc = "Graceful Transmit Stop"]
    pub mod GTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Full-Duplex Enable"]
    pub mod FDEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit Frame Control Pause"]
    pub mod TFC_PAUSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No PAUSE frame transmitted."]
            pub const TFC_PAUSE_0: u32 = 0;
            #[doc = "The MAC stops transmission of data frames after the current transmission is complete."]
            pub const TFC_PAUSE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive Frame Control Pause"]
    pub mod RFC_PAUSE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Source MAC Address Select On Transmit"]
    pub mod ADDSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Node MAC address programmed on PADDR1/2 registers."]
            pub const ADDSEL_0: u32 = 0;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Set MAC Address On Transmit"]
    pub mod ADDINS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The source MAC address is not modified by the MAC."]
            pub const ADDINS_0: u32 = 0;
            #[doc = "The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL."]
            pub const ADDINS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Forward Frame From Application With CRC"]
    pub mod CRCFWD {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "TxBD\\[TC\\] controls whether the frame has a CRC from the application."]
            pub const CRCFWD_0: u32 = 0;
            #[doc = "The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application."]
            pub const CRCFWD_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Physical Address Lower Register"]
pub mod PALR {
    pub use crate::RW as access;
    #[doc = "Pause Address"]
    pub mod PADDR1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Physical Address Upper Register"]
pub mod PAUR {
    pub use crate::RW as access;
    #[doc = "Type Field In PAUSE Frames"]
    pub mod TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames"]
    pub mod PADDR2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Opcode/Pause Duration Register"]
pub mod OPD {
    pub use crate::RW as access;
    #[doc = "Pause Duration"]
    pub mod PAUSE_DUR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Opcode Field In PAUSE Frames"]
    pub mod OPCODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Interrupt Coalescing Register"]
pub mod TXIC {
    pub use crate::RW as access;
    #[doc = "Interrupt coalescing timer threshold"]
    pub mod ICTT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Interrupt coalescing frame count threshold"]
    pub mod ICFT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Interrupt Coalescing Timer Clock Source Select"]
    pub mod ICCS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use MII/GMII TX clocks."]
            pub const ICCS_0: u32 = 0;
            #[doc = "Use ENET system clock."]
            pub const ICCS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Interrupt Coalescing Enable"]
    pub mod ICEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable Interrupt coalescing."]
            pub const ICEN_0: u32 = 0;
            #[doc = "Enable Interrupt coalescing."]
            pub const ICEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Interrupt Coalescing Register"]
pub mod RXIC {
    pub use crate::RW as access;
    #[doc = "Interrupt coalescing timer threshold"]
    pub mod ICTT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Interrupt coalescing frame count threshold"]
    pub mod ICFT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Interrupt Coalescing Timer Clock Source Select"]
    pub mod ICCS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Use MII/GMII TX clocks."]
            pub const ICCS_0: u32 = 0;
            #[doc = "Use ENET system clock."]
            pub const ICCS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Interrupt Coalescing Enable"]
    pub mod ICEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable Interrupt coalescing."]
            pub const ICEN_0: u32 = 0;
            #[doc = "Enable Interrupt coalescing."]
            pub const ICEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Individual Upper Address Register"]
pub mod IAUR {
    pub use crate::RW as access;
    #[doc = "Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    pub mod IADDR1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Individual Lower Address Register"]
pub mod IALR {
    pub use crate::RW as access;
    #[doc = "Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    pub mod IADDR2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Group Upper Address Register"]
pub mod GAUR {
    pub use crate::RW as access;
    #[doc = "Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    pub mod GADDR1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Descriptor Group Lower Address Register"]
pub mod GALR {
    pub use crate::RW as access;
    #[doc = "Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    pub mod GADDR2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit FIFO Watermark Register"]
pub mod TFWR {
    pub use crate::RW as access;
    #[doc = "Transmit FIFO Write"]
    pub mod TFWR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64 bytes written."]
            pub const TFWR_0: u32 = 0;
            #[doc = "64 bytes written."]
            pub const TFWR_1: u32 = 0x01;
            #[doc = "128 bytes written."]
            pub const TFWR_2: u32 = 0x02;
            #[doc = "192 bytes written."]
            pub const TFWR_3: u32 = 0x03;
            #[doc = "1984 bytes written."]
            pub const TFWR_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Store And Forward Enable"]
    pub mod STRFWD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
            pub const STRFWD_0: u32 = 0;
            #[doc = "Enabled."]
            pub const STRFWD_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Descriptor Ring Start Register"]
pub mod RDSR {
    pub use crate::RW as access;
    #[doc = "Pointer to the beginning of the receive buffer descriptor queue."]
    pub mod R_DES_START {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Buffer Descriptor Ring Start Register"]
pub mod TDSR {
    pub use crate::RW as access;
    #[doc = "Pointer to the beginning of the transmit buffer descriptor queue."]
    pub mod X_DES_START {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Maximum Receive Buffer Size Register"]
pub mod MRBR {
    pub use crate::RW as access;
    #[doc = "Receive buffer size in bytes"]
    pub mod R_BUF_SIZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive FIFO Section Full Threshold"]
pub mod RSFL {
    pub use crate::RW as access;
    #[doc = "Value Of Receive FIFO Section Full Threshold"]
    pub mod RX_SECTION_FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive FIFO Section Empty Threshold"]
pub mod RSEM {
    pub use crate::RW as access;
    #[doc = "Value Of The Receive FIFO Section Empty Threshold"]
    pub mod RX_SECTION_EMPTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RX Status FIFO Section Empty Threshold"]
    pub mod STAT_SECTION_EMPTY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive FIFO Almost Empty Threshold"]
pub mod RAEM {
    pub use crate::RW as access;
    #[doc = "Value Of The Receive FIFO Almost Empty Threshold"]
    pub mod RX_ALMOST_EMPTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive FIFO Almost Full Threshold"]
pub mod RAFL {
    pub use crate::RW as access;
    #[doc = "Value Of The Receive FIFO Almost Full Threshold"]
    pub mod RX_ALMOST_FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit FIFO Section Empty Threshold"]
pub mod TSEM {
    pub use crate::RW as access;
    #[doc = "Value Of The Transmit FIFO Section Empty Threshold"]
    pub mod TX_SECTION_EMPTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit FIFO Almost Empty Threshold"]
pub mod TAEM {
    pub use crate::RW as access;
    #[doc = "Value of Transmit FIFO Almost Empty Threshold"]
    pub mod TX_ALMOST_EMPTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit FIFO Almost Full Threshold"]
pub mod TAFL {
    pub use crate::RW as access;
    #[doc = "Value Of The Transmit FIFO Almost Full Threshold"]
    pub mod TX_ALMOST_FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Inter-Packet Gap"]
pub mod TIPG {
    pub use crate::RW as access;
    #[doc = "Transmit Inter-Packet Gap"]
    pub mod IPG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frame Truncation Length"]
pub mod FTRL {
    pub use crate::RW as access;
    #[doc = "Frame Truncation Length"]
    pub mod TRUNC_FL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Transmit Accelerator Function Configuration"]
pub mod TACC {
    pub use crate::RW as access;
    #[doc = "TX FIFO Shift-16"]
    pub mod SHIFT16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled."]
            pub const SHIFT16_0: u32 = 0;
            #[doc = "Indicates to the transmit data FIFO that the written frames contain two additional octets before the frame data. This means the actual frame begins at bit 16 of the first word written into the FIFO. This function allows putting the frame payload on a 32-bit boundary in memory, as the 14-byte Ethernet header is extended to a 16-byte header."]
            pub const SHIFT16_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables insertion of IP header checksum."]
    pub mod IPCHK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Checksum is not inserted."]
            pub const IPCHK_0: u32 = 0;
            #[doc = "If an IP frame is transmitted, the checksum is inserted automatically. The IP header checksum field must be cleared. If a non-IP frame is transmitted the frame is not modified."]
            pub const IPCHK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables insertion of protocol checksum."]
    pub mod PROCHK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Checksum not inserted."]
            pub const PROCHK_0: u32 = 0;
            #[doc = "If an IP frame with a known protocol is transmitted, the checksum is inserted automatically into the frame. The checksum field must be cleared. The other frames are not modified."]
            pub const PROCHK_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive Accelerator Function Configuration"]
pub mod RACC {
    pub use crate::RW as access;
    #[doc = "Enable Padding Removal For Short IP Frames"]
    pub mod PADREM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Padding not removed."]
            pub const PADREM_0: u32 = 0;
            #[doc = "Any bytes following the IP payload section of the frame are removed from the frame."]
            pub const PADREM_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    pub mod IPDIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Frames with wrong IPv4 header checksum are not discarded."]
            pub const IPDIS_0: u32 = 0;
            #[doc = "If an IPv4 frame is received with a mismatching header checksum, the frame is discarded. IPv6 has no header checksum and is not affected by this setting. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
            pub const IPDIS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Discard Of Frames With Wrong Protocol Checksum"]
    pub mod PRODIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Frames with wrong checksum are not discarded."]
            pub const PRODIS_0: u32 = 0;
            #[doc = "If a TCP/IP, UDP/IP, or ICMP/IP frame is received that has a wrong TCP, UDP, or ICMP checksum, the frame is discarded. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
            pub const PRODIS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Discard Of Frames With MAC Layer Errors"]
    pub mod LINEDIS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Frames with errors are not discarded."]
            pub const LINEDIS_0: u32 = 0;
            #[doc = "Any frame received with a CRC, length, or PHY error is automatically discarded and not forwarded to the user application interface."]
            pub const LINEDIS_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RX FIFO Shift-16"]
    pub mod SHIFT16 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disabled."]
            pub const SHIFT16_0: u32 = 0;
            #[doc = "Instructs the MAC to write two additional bytes in front of each frame received into the RX FIFO."]
            pub const SHIFT16_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Reserved Statistic Register"]
pub mod RMON_T_DROP {
    pub use crate::RO as access;
}
#[doc = "Tx Packet Count Statistic Register"]
pub mod RMON_T_PACKETS {
    pub use crate::RO as access;
    #[doc = "Packet count"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx Broadcast Packets Statistic Register"]
pub mod RMON_T_BC_PKT {
    pub use crate::RO as access;
    #[doc = "Broadcast packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx Multicast Packets Statistic Register"]
pub mod RMON_T_MC_PKT {
    pub use crate::RO as access;
    #[doc = "Multicast packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx Packets with CRC/Align Error Statistic Register"]
pub mod RMON_T_CRC_ALIGN {
    pub use crate::RO as access;
    #[doc = "Packets with CRC/align error"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx Packets Less Than Bytes and Good CRC Statistic Register"]
pub mod RMON_T_UNDERSIZE {
    pub use crate::RO as access;
    #[doc = "Number of transmit packets less than 64 bytes with good CRC"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
pub mod RMON_T_OVERSIZE {
    pub use crate::RO as access;
    #[doc = "Number of transmit packets greater than MAX_FL bytes with good CRC"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
pub mod RMON_T_FRAG {
    pub use crate::RO as access;
    #[doc = "Number of packets less than 64 bytes with bad CRC"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
pub mod RMON_T_JAB {
    pub use crate::RO as access;
    #[doc = "Number of transmit packets greater than MAX_FL bytes and bad CRC"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx Collision Count Statistic Register"]
pub mod RMON_T_COL {
    pub use crate::RO as access;
    #[doc = "Number of transmit collisions"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx 64-Byte Packets Statistic Register"]
pub mod RMON_T_P64 {
    pub use crate::RO as access;
    #[doc = "Number of 64-byte transmit packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx 65- to 127-byte Packets Statistic Register"]
pub mod RMON_T_P65TO127 {
    pub use crate::RO as access;
    #[doc = "Number of 65- to 127-byte transmit packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx 128- to 255-byte Packets Statistic Register"]
pub mod RMON_T_P128TO255 {
    pub use crate::RO as access;
    #[doc = "Number of 128- to 255-byte transmit packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx 256- to 511-byte Packets Statistic Register"]
pub mod RMON_T_P256TO511 {
    pub use crate::RO as access;
    #[doc = "Number of 256- to 511-byte transmit packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx 512- to 1023-byte Packets Statistic Register"]
pub mod RMON_T_P512TO1023 {
    pub use crate::RO as access;
    #[doc = "Number of 512- to 1023-byte transmit packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx 1024- to 2047-byte Packets Statistic Register"]
pub mod RMON_T_P1024TO2047 {
    pub use crate::RO as access;
    #[doc = "Number of 1024- to 2047-byte transmit packets"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx Packets Greater Than 2048 Bytes Statistic Register"]
pub mod RMON_T_P_GTE2048 {
    pub use crate::RO as access;
    #[doc = "Number of transmit packets greater than 2048 bytes"]
    pub mod TXPKTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Tx Octets Statistic Register"]
pub mod RMON_T_OCTETS {
    pub use crate::RO as access;
    #[doc = "Number of transmit octets"]
    pub mod TXOCTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Reserved Statistic Register"]
pub mod IEEE_T_DROP {
    pub use crate::RO as access;
}
#[doc = "Frames Transmitted OK Statistic Register"]
pub mod IEEE_T_FRAME_OK {
    pub use crate::RO as access;
    #[doc = "Number of frames transmitted OK"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frames Transmitted with Single Collision Statistic Register"]
pub mod IEEE_T_1COL {
    pub use crate::RO as access;
    #[doc = "Number of frames transmitted with one collision"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frames Transmitted with Multiple Collisions Statistic Register"]
pub mod IEEE_T_MCOL {
    pub use crate::RO as access;
    #[doc = "Number of frames transmitted with multiple collisions"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frames Transmitted after Deferral Delay Statistic Register"]
pub mod IEEE_T_DEF {
    pub use crate::RO as access;
    #[doc = "Number of frames transmitted with deferral delay"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frames Transmitted with Late Collision Statistic Register"]
pub mod IEEE_T_LCOL {
    pub use crate::RO as access;
    #[doc = "Number of frames transmitted with late collision"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frames Transmitted with Excessive Collisions Statistic Register"]
pub mod IEEE_T_EXCOL {
    pub use crate::RO as access;
    #[doc = "Number of frames transmitted with excessive collisions"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frames Transmitted with Tx FIFO Underrun Statistic Register"]
pub mod IEEE_T_MACERR {
    pub use crate::RO as access;
    #[doc = "Number of frames transmitted with transmit FIFO underrun"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frames Transmitted with Carrier Sense Error Statistic Register"]
pub mod IEEE_T_CSERR {
    pub use crate::RO as access;
    #[doc = "Number of frames transmitted with carrier sense error"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Reserved Statistic Register"]
pub mod IEEE_T_SQE {
    pub use crate::RO as access;
    #[doc = "This read-only field is reserved and always has the value 0"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Flow Control Pause Frames Transmitted Statistic Register"]
pub mod IEEE_T_FDXFC {
    pub use crate::RO as access;
    #[doc = "Number of flow-control pause frames transmitted"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Octet Count for Frames Transmitted w/o Error Statistic Register"]
pub mod IEEE_T_OCTETS_OK {
    pub use crate::RO as access;
    #[doc = "Octet count for frames transmitted without error Counts total octets (includes header and FCS fields)."]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx Packet Count Statistic Register"]
pub mod RMON_R_PACKETS {
    pub use crate::RO as access;
    #[doc = "Number of packets received"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx Broadcast Packets Statistic Register"]
pub mod RMON_R_BC_PKT {
    pub use crate::RO as access;
    #[doc = "Number of receive broadcast packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx Multicast Packets Statistic Register"]
pub mod RMON_R_MC_PKT {
    pub use crate::RO as access;
    #[doc = "Number of receive multicast packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx Packets with CRC/Align Error Statistic Register"]
pub mod RMON_R_CRC_ALIGN {
    pub use crate::RO as access;
    #[doc = "Number of receive packets with CRC or align error"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
pub mod RMON_R_UNDERSIZE {
    pub use crate::RO as access;
    #[doc = "Number of receive packets with less than 64 bytes and good CRC"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
pub mod RMON_R_OVERSIZE {
    pub use crate::RO as access;
    #[doc = "Number of receive packets greater than MAX_FL and good CRC"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
pub mod RMON_R_FRAG {
    pub use crate::RO as access;
    #[doc = "Number of receive packets with less than 64 bytes and bad CRC"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
pub mod RMON_R_JAB {
    pub use crate::RO as access;
    #[doc = "Number of receive packets greater than MAX_FL and bad CRC"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Reserved Statistic Register"]
pub mod RMON_R_RESVD_0 {
    pub use crate::RO as access;
}
#[doc = "Rx 64-Byte Packets Statistic Register"]
pub mod RMON_R_P64 {
    pub use crate::RO as access;
    #[doc = "Number of 64-byte receive packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx 65- to 127-Byte Packets Statistic Register"]
pub mod RMON_R_P65TO127 {
    pub use crate::RO as access;
    #[doc = "Number of 65- to 127-byte recieve packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx 128- to 255-Byte Packets Statistic Register"]
pub mod RMON_R_P128TO255 {
    pub use crate::RO as access;
    #[doc = "Number of 128- to 255-byte recieve packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx 256- to 511-Byte Packets Statistic Register"]
pub mod RMON_R_P256TO511 {
    pub use crate::RO as access;
    #[doc = "Number of 256- to 511-byte recieve packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx 512- to 1023-Byte Packets Statistic Register"]
pub mod RMON_R_P512TO1023 {
    pub use crate::RO as access;
    #[doc = "Number of 512- to 1023-byte recieve packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx 1024- to 2047-Byte Packets Statistic Register"]
pub mod RMON_R_P1024TO2047 {
    pub use crate::RO as access;
    #[doc = "Number of 1024- to 2047-byte recieve packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx Packets Greater than 2048 Bytes Statistic Register"]
pub mod RMON_R_P_GTE2048 {
    pub use crate::RO as access;
    #[doc = "Number of greater-than-2048-byte recieve packets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rx Octets Statistic Register"]
pub mod RMON_R_OCTETS {
    pub use crate::RO as access;
    #[doc = "Number of receive octets"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frames not Counted Correctly Statistic Register"]
pub mod IEEE_R_DROP {
    pub use crate::RO as access;
    #[doc = "Frame count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frames Received OK Statistic Register"]
pub mod IEEE_R_FRAME_OK {
    pub use crate::RO as access;
    #[doc = "Number of frames received OK"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frames Received with CRC Error Statistic Register"]
pub mod IEEE_R_CRC {
    pub use crate::RO as access;
    #[doc = "Number of frames received with CRC error"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frames Received with Alignment Error Statistic Register"]
pub mod IEEE_R_ALIGN {
    pub use crate::RO as access;
    #[doc = "Number of frames received with alignment error"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Receive FIFO Overflow Count Statistic Register"]
pub mod IEEE_R_MACERR {
    pub use crate::RO as access;
    #[doc = "Receive FIFO overflow count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Flow Control Pause Frames Received Statistic Register"]
pub mod IEEE_R_FDXFC {
    pub use crate::RO as access;
    #[doc = "Number of flow-control pause frames received"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Octet Count for Frames Received without Error Statistic Register"]
pub mod IEEE_R_OCTETS_OK {
    pub use crate::RO as access;
    #[doc = "Number of octets for frames received without error"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Adjustable Timer Control Register"]
pub mod ATCR {
    pub use crate::RW as access;
    #[doc = "Enable Timer"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The timer stops at the current value."]
            pub const EN_0: u32 = 0;
            #[doc = "The timer starts incrementing."]
            pub const EN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable One-Shot Offset Event"]
    pub mod OFFEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable."]
            pub const OFFEN_0: u32 = 0;
            #[doc = "The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field."]
            pub const OFFEN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reset Timer On Offset Event"]
    pub mod OFFRST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached."]
            pub const OFFRST_0: u32 = 0;
            #[doc = "If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt."]
            pub const OFFRST_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Periodical Event"]
    pub mod PEREN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable."]
            pub const PEREN_0: u32 = 0;
            #[doc = "A period event interrupt can be generated (EIR\\[TS_TIMER\\]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details."]
            pub const PEREN_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enables event signal output assertion on period event"]
    pub mod PINPER {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Disable."]
            pub const PINPER_0: u32 = 0;
            #[doc = "Enable."]
            pub const PINPER_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reset Timer"]
    pub mod RESTART {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Capture Timer Value"]
    pub mod CAPTURE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No effect."]
            pub const CAPTURE_0: u32 = 0;
            #[doc = "The current time is captured and can be read from the ATVR register."]
            pub const CAPTURE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Timer Slave Mode"]
    pub mod SLAVE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The timer is active and all configuration fields in this register are relevant."]
            pub const SLAVE_0: u32 = 0;
            #[doc = "The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value."]
            pub const SLAVE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Value Register"]
pub mod ATVR {
    pub use crate::RW as access;
    #[doc = "A write sets the timer"]
    pub mod ATIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Offset Register"]
pub mod ATOFF {
    pub use crate::RW as access;
    #[doc = "Offset value for one-shot event generation"]
    pub mod OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Period Register"]
pub mod ATPER {
    pub use crate::RW as access;
    #[doc = "Value for generating periodic events"]
    pub mod PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Correction Register"]
pub mod ATCOR {
    pub use crate::RW as access;
    #[doc = "Correction Counter Wrap-Around Value"]
    pub mod COR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Time-Stamping Clock Period Register"]
pub mod ATINC {
    pub use crate::RW as access;
    #[doc = "Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    pub mod INC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Correction Increment Value"]
    pub mod INC_CORR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timestamp of Last Transmitted Frame"]
pub mod ATSTMP {
    pub use crate::RO as access;
    #[doc = "Timestamp of the last frame transmitted by the core that had TxBD\\[TS\\] set"]
    pub mod TIMESTAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Global Status Register"]
pub mod TGSR {
    pub use crate::RW as access;
    #[doc = "Copy Of Timer Flag For Channel 0"]
    pub mod TF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Timer Flag for Channel 0 is clear"]
            pub const TF0_0: u32 = 0;
            #[doc = "Timer Flag for Channel 0 is set"]
            pub const TF0_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Copy Of Timer Flag For Channel 1"]
    pub mod TF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Timer Flag for Channel 1 is clear"]
            pub const TF1_0: u32 = 0;
            #[doc = "Timer Flag for Channel 1 is set"]
            pub const TF1_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Copy Of Timer Flag For Channel 2"]
    pub mod TF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Timer Flag for Channel 2 is clear"]
            pub const TF2_0: u32 = 0;
            #[doc = "Timer Flag for Channel 2 is set"]
            pub const TF2_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Copy Of Timer Flag For Channel 3"]
    pub mod TF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Timer Flag for Channel 3 is clear"]
            pub const TF3_0: u32 = 0;
            #[doc = "Timer Flag for Channel 3 is set"]
            pub const TF3_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Control Status Register"]
pub mod TCSR0 {
    pub use crate::RW as access;
    #[doc = "Timer DMA Request Enable"]
    pub mod TDRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DMA request is disabled"]
            pub const TDRE_0: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const TDRE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Mode"]
    pub mod TMODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Timer Channel is disabled."]
            pub const TMODE_0: u32 = 0;
            #[doc = "Timer Channel is configured for Input Capture on rising edge."]
            pub const TMODE_1: u32 = 0x01;
            #[doc = "Timer Channel is configured for Input Capture on falling edge."]
            pub const TMODE_2: u32 = 0x02;
            #[doc = "Timer Channel is configured for Input Capture on both edges."]
            pub const TMODE_3: u32 = 0x03;
            #[doc = "Timer Channel is configured for Output Compare - software only."]
            pub const TMODE_4: u32 = 0x04;
            #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
            pub const TMODE_5: u32 = 0x05;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
            pub const TMODE_6: u32 = 0x06;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
            pub const TMODE_7: u32 = 0x07;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
            pub const TMODE_9: u32 = 0x09;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
            pub const TMODE_10: u32 = 0x0a;
            #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMODE_14: u32 = 0x0e;
            #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMODE_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Interrupt Enable"]
    pub mod TIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt is disabled"]
            pub const TIE_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const TIE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Flag"]
    pub mod TF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Input Capture or Output Compare has not occurred."]
            pub const TF_0: u32 = 0;
            #[doc = "Input Capture or Output Compare has occurred."]
            pub const TF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer PulseWidth Control"]
    pub mod TPWC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pulse width is one 1588-clock cycle."]
            pub const TPWC_0: u32 = 0;
            #[doc = "Pulse width is two 1588-clock cycles."]
            pub const TPWC_1: u32 = 0x01;
            #[doc = "Pulse width is three 1588-clock cycles."]
            pub const TPWC_2: u32 = 0x02;
            #[doc = "Pulse width is four 1588-clock cycles."]
            pub const TPWC_3: u32 = 0x03;
            #[doc = "Pulse width is 32 1588-clock cycles."]
            pub const TPWC_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Compare Capture Register"]
pub mod TCCR0 {
    pub use crate::RW as access;
    #[doc = "Timer Capture Compare"]
    pub mod TCC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Control Status Register"]
pub mod TCSR1 {
    pub use crate::RW as access;
    #[doc = "Timer DMA Request Enable"]
    pub mod TDRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DMA request is disabled"]
            pub const TDRE_0: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const TDRE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Mode"]
    pub mod TMODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Timer Channel is disabled."]
            pub const TMODE_0: u32 = 0;
            #[doc = "Timer Channel is configured for Input Capture on rising edge."]
            pub const TMODE_1: u32 = 0x01;
            #[doc = "Timer Channel is configured for Input Capture on falling edge."]
            pub const TMODE_2: u32 = 0x02;
            #[doc = "Timer Channel is configured for Input Capture on both edges."]
            pub const TMODE_3: u32 = 0x03;
            #[doc = "Timer Channel is configured for Output Compare - software only."]
            pub const TMODE_4: u32 = 0x04;
            #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
            pub const TMODE_5: u32 = 0x05;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
            pub const TMODE_6: u32 = 0x06;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
            pub const TMODE_7: u32 = 0x07;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
            pub const TMODE_9: u32 = 0x09;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
            pub const TMODE_10: u32 = 0x0a;
            #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMODE_14: u32 = 0x0e;
            #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMODE_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Interrupt Enable"]
    pub mod TIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt is disabled"]
            pub const TIE_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const TIE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Flag"]
    pub mod TF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Input Capture or Output Compare has not occurred."]
            pub const TF_0: u32 = 0;
            #[doc = "Input Capture or Output Compare has occurred."]
            pub const TF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer PulseWidth Control"]
    pub mod TPWC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pulse width is one 1588-clock cycle."]
            pub const TPWC_0: u32 = 0;
            #[doc = "Pulse width is two 1588-clock cycles."]
            pub const TPWC_1: u32 = 0x01;
            #[doc = "Pulse width is three 1588-clock cycles."]
            pub const TPWC_2: u32 = 0x02;
            #[doc = "Pulse width is four 1588-clock cycles."]
            pub const TPWC_3: u32 = 0x03;
            #[doc = "Pulse width is 32 1588-clock cycles."]
            pub const TPWC_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Compare Capture Register"]
pub mod TCCR1 {
    pub use crate::RW as access;
    #[doc = "Timer Capture Compare"]
    pub mod TCC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Control Status Register"]
pub mod TCSR2 {
    pub use crate::RW as access;
    #[doc = "Timer DMA Request Enable"]
    pub mod TDRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DMA request is disabled"]
            pub const TDRE_0: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const TDRE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Mode"]
    pub mod TMODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Timer Channel is disabled."]
            pub const TMODE_0: u32 = 0;
            #[doc = "Timer Channel is configured for Input Capture on rising edge."]
            pub const TMODE_1: u32 = 0x01;
            #[doc = "Timer Channel is configured for Input Capture on falling edge."]
            pub const TMODE_2: u32 = 0x02;
            #[doc = "Timer Channel is configured for Input Capture on both edges."]
            pub const TMODE_3: u32 = 0x03;
            #[doc = "Timer Channel is configured for Output Compare - software only."]
            pub const TMODE_4: u32 = 0x04;
            #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
            pub const TMODE_5: u32 = 0x05;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
            pub const TMODE_6: u32 = 0x06;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
            pub const TMODE_7: u32 = 0x07;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
            pub const TMODE_9: u32 = 0x09;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
            pub const TMODE_10: u32 = 0x0a;
            #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMODE_14: u32 = 0x0e;
            #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMODE_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Interrupt Enable"]
    pub mod TIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt is disabled"]
            pub const TIE_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const TIE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Flag"]
    pub mod TF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Input Capture or Output Compare has not occurred."]
            pub const TF_0: u32 = 0;
            #[doc = "Input Capture or Output Compare has occurred."]
            pub const TF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer PulseWidth Control"]
    pub mod TPWC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pulse width is one 1588-clock cycle."]
            pub const TPWC_0: u32 = 0;
            #[doc = "Pulse width is two 1588-clock cycles."]
            pub const TPWC_1: u32 = 0x01;
            #[doc = "Pulse width is three 1588-clock cycles."]
            pub const TPWC_2: u32 = 0x02;
            #[doc = "Pulse width is four 1588-clock cycles."]
            pub const TPWC_3: u32 = 0x03;
            #[doc = "Pulse width is 32 1588-clock cycles."]
            pub const TPWC_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Compare Capture Register"]
pub mod TCCR2 {
    pub use crate::RW as access;
    #[doc = "Timer Capture Compare"]
    pub mod TCC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Control Status Register"]
pub mod TCSR3 {
    pub use crate::RW as access;
    #[doc = "Timer DMA Request Enable"]
    pub mod TDRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "DMA request is disabled"]
            pub const TDRE_0: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const TDRE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Mode"]
    pub mod TMODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Timer Channel is disabled."]
            pub const TMODE_0: u32 = 0;
            #[doc = "Timer Channel is configured for Input Capture on rising edge."]
            pub const TMODE_1: u32 = 0x01;
            #[doc = "Timer Channel is configured for Input Capture on falling edge."]
            pub const TMODE_2: u32 = 0x02;
            #[doc = "Timer Channel is configured for Input Capture on both edges."]
            pub const TMODE_3: u32 = 0x03;
            #[doc = "Timer Channel is configured for Output Compare - software only."]
            pub const TMODE_4: u32 = 0x04;
            #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
            pub const TMODE_5: u32 = 0x05;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
            pub const TMODE_6: u32 = 0x06;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
            pub const TMODE_7: u32 = 0x07;
            #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
            pub const TMODE_9: u32 = 0x09;
            #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
            pub const TMODE_10: u32 = 0x0a;
            #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMODE_14: u32 = 0x0e;
            #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
            pub const TMODE_15: u32 = 0x0f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Interrupt Enable"]
    pub mod TIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Interrupt is disabled"]
            pub const TIE_0: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const TIE_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Flag"]
    pub mod TF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Input Capture or Output Compare has not occurred."]
            pub const TF_0: u32 = 0;
            #[doc = "Input Capture or Output Compare has occurred."]
            pub const TF_1: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer PulseWidth Control"]
    pub mod TPWC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x1f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Pulse width is one 1588-clock cycle."]
            pub const TPWC_0: u32 = 0;
            #[doc = "Pulse width is two 1588-clock cycles."]
            pub const TPWC_1: u32 = 0x01;
            #[doc = "Pulse width is three 1588-clock cycles."]
            pub const TPWC_2: u32 = 0x02;
            #[doc = "Pulse width is four 1588-clock cycles."]
            pub const TPWC_3: u32 = 0x03;
            #[doc = "Pulse width is 32 1588-clock cycles."]
            pub const TPWC_31: u32 = 0x1f;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Timer Compare Capture Register"]
pub mod TCCR3 {
    pub use crate::RW as access;
    #[doc = "Timer Capture Compare"]
    pub mod TCC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
