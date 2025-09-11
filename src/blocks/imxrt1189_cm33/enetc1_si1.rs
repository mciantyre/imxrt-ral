#[doc = "ENETC Station Interface"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Station interface mode register"]
    pub SIMR: u32,
    #[doc = "Station interface status register"]
    pub SISR: u32,
    _reserved0: [u8; 0x10],
    #[doc = "Station interface current time register 0"]
    pub SICTR0: u32,
    #[doc = "Station interface current time register 1"]
    pub SICTR1: u32,
    #[doc = "Station interface port capability register 0"]
    pub SIPCAPR0: u32,
    #[doc = "Station interface port capability register 1"]
    pub SIPCAPR1: u32,
    _reserved1: [u8; 0x10],
    #[doc = "Station interface receive BDR group control register"]
    pub SIRBGCR: u32,
    _reserved2: [u8; 0x04],
    #[doc = "Station interface buffer cache attribute register"]
    pub SIBCAR: u32,
    _reserved3: [u8; 0x04],
    #[doc = "Station interface command cache attribute register"]
    pub SICCAR: u32,
    _reserved4: [u8; 0x34],
    #[doc = "Station interface primary MAC address register 0"]
    pub SIPMAR0: u32,
    #[doc = "Station interface primary MAC address register 1"]
    pub SIPMAR1: u32,
    _reserved5: [u8; 0x08],
    #[doc = "Station interface custom VLAN register 1"]
    pub SICVLANR1: u32,
    #[doc = "Station interface custom VLAN register 2"]
    pub SICVLANR2: u32,
    _reserved6: [u8; 0x68],
    #[doc = "Station interface VLAN to IPV mapping register 0"]
    pub SIVLANIPVMR0: u32,
    #[doc = "Station interface VLAN to IPV mapping register 1"]
    pub SIVLANIPVMR1: u32,
    _reserved7: [u8; 0x48],
    #[doc = "Station interface IPV to ring mapping register"]
    pub SIIPVBDRMR0: u32,
    _reserved8: [u8; 0xb0],
    #[doc = "Virtual station interface message send register"]
    pub VSIMSGSR: u32,
    #[doc = "Virtual station interface message receive register"]
    pub VSIMSGRR: u32,
    _reserved9: [u8; 0x04],
    #[doc = "Virtual station interface message send register 0"]
    pub VSIMSGSNDAR0: u32,
    #[doc = "Virtual station interface message send address register 1"]
    pub VSIMSGSNDAR1: u32,
    _reserved10: [u8; 0xe8],
    #[doc = "Station interface receive octets counter (ifInOctets) 0"]
    pub SIROCT0: u32,
    #[doc = "Station interface receive octets counter (ifInOctets) 1"]
    pub SIROCT1: u32,
    #[doc = "Station interface receive frame counter (aFrameReceivedOK) 0"]
    pub SIRFRM0: u32,
    #[doc = "Station interface receive frame counter (aFrameReceivedOK) 1"]
    pub SIRFRM1: u32,
    #[doc = "Station interface receive unicast frame counter (ifInUcastPkts) 0"]
    pub SIRUCA0: u32,
    #[doc = "Station interface receive unicast frame counter (ifInUcastPkts) 1"]
    pub SIRUCA1: u32,
    #[doc = "Station interface receive multicast frame counter (ifInMulticastPkts) 0"]
    pub SIRMCA0: u32,
    #[doc = "Station interface receive multicast frame counter (ifInMulticastPkts) 1"]
    pub SIRMCA1: u32,
    #[doc = "Station interface transmit octets counter (ifOutOctets) 0"]
    pub SITOCT0: u32,
    #[doc = "Station interface transmit octets counter (ifOutOctets) 1"]
    pub SITOCT1: u32,
    #[doc = "Station interface transmit frame counter (aFrameTransmittedOK) 0"]
    pub SITFRM0: u32,
    #[doc = "Station interface transmit frame counter (aFrameTransmittedOK) 1"]
    pub SITFRM1: u32,
    #[doc = "Station interface transmit unicast frame counter (ifOutUcastPkts) 0"]
    pub SITUCA0: u32,
    #[doc = "Station interface transmit unicast frame counter (ifOutUcastPkts) 1"]
    pub SITUCA1: u32,
    #[doc = "Station interface transmit multicast frame counter (ifOutMulticastPkts) 0"]
    pub SITMCA0: u32,
    #[doc = "Station interface transmit multicast frame counter (ifOutMulticastPkts) 1"]
    pub SITMCA1: u32,
    _reserved11: [u8; 0xb0],
    #[doc = "Station interface boot loader parameter register a"]
    pub SIBLPR: [u32; 2usize],
    _reserved12: [u8; 0x0408],
    #[doc = "Station interface command BDR mode register"]
    pub SICBDRMR: u32,
    #[doc = "Station interface command BDR status register"]
    pub SICBDRSR: u32,
    _reserved13: [u8; 0x08],
    #[doc = "Station interface command BDR base address register 0"]
    pub SICBDRBAR0: u32,
    #[doc = "Station interface command BDR base address register 1"]
    pub SICBDRBAR1: u32,
    #[doc = "Station interface command BDR producer index register"]
    pub SICBDRPIR: u32,
    #[doc = "Station interface command BDR consumer index register"]
    pub SICBDRCIR: u32,
    #[doc = "Station interface command BDR length register"]
    pub SICBDRLENR: u32,
    _reserved14: [u8; 0x7c],
    #[doc = "Station interface command BDR interrupt enable register"]
    pub SICBDRIER: u32,
    #[doc = "Station interface command BDR interrupt detect register"]
    pub SICBDRIDR: u32,
    _reserved15: [u8; 0x58],
    #[doc = "Station interface capability register 0"]
    pub SICAPR0: u32,
    #[doc = "Station interface capability register 1"]
    pub SICAPR1: u32,
    #[doc = "Station interface capability register 2"]
    pub SICAPR2: u32,
    _reserved16: [u8; 0xf4],
    #[doc = "Virtual station interface interrupt enable register"]
    pub VSIIER: u32,
    _reserved17: [u8; 0x04],
    #[doc = "Virtual station interface interrupt detect register"]
    pub VSIIDR: u32,
    _reserved18: [u8; 0x0c],
    #[doc = "Station interface transmit interrupt detect register 0"]
    pub SITXIDR0: u32,
    _reserved19: [u8; 0x0c],
    #[doc = "Station interface receive interrupt detect register 0"]
    pub SIRXIDR0: u32,
    _reserved20: [u8; 0x04],
    #[doc = "Station interface MSI-X vector register"]
    pub SIMSIVR: u32,
    #[doc = "Station interface command MSI-X vector register"]
    pub SICMSIVR: u32,
    _reserved21: [u8; 0x08],
    #[doc = "Station interface timer interrupt enable register"]
    pub SITMRIER: u32,
    #[doc = "Station interface timer interrupt detect register"]
    pub SITMRIDR: u32,
    _reserved22: [u8; 0x04],
    #[doc = "Station interface timer MSI-X vector register"]
    pub SITMRMSIVR: u32,
    _reserved23: [u8; 0xb0],
    #[doc = "Station interface MSI-X transmit ring a vector register"]
    pub SIMSITRVR: [u32; 10usize],
    _reserved24: [u8; 0x58],
    #[doc = "Station interface MSI-X receive ring a vector register"]
    pub SIMSIRRVR: [u32; 10usize],
    _reserved25: [u8; 0x0258],
    #[doc = "Station interface correctable memory error configuration register"]
    pub SICMECR: u32,
    #[doc = "Station interface correctable memory error status register"]
    pub SICMESR: u32,
    _reserved26: [u8; 0x04],
    #[doc = "Station interface correctable memory error count register"]
    pub SICMECTR: u32,
    #[doc = "Station interface uncorrectable programming error configuration register"]
    pub SIUPECR: u32,
    #[doc = "Station interface uncorrectable programming error status register"]
    pub SIUPESR: u32,
    _reserved27: [u8; 0x04],
    #[doc = "Station interface uncorrectable programming error count register"]
    pub SIUPECTR: u32,
    #[doc = "Station interface uncorrectable non-fatal system bus error configuration register"]
    pub SIUNSBECR: u32,
    #[doc = "Station interface uncorrectable non-fatal system bus error status register"]
    pub SIUNSBESR: u32,
    _reserved28: [u8; 0x04],
    #[doc = "Station interface uncorrectable non-fatal system bus error count register"]
    pub SIUNSBECTR: u32,
    #[doc = "Station interface uncorrectable fatal system bus error configuration register"]
    pub SIUFSBECR: u32,
    #[doc = "Station interface uncorrectable fatal system bus error status register"]
    pub SIUFSBESR: u32,
    _reserved29: [u8; 0x08],
    #[doc = "Station interface uncorrectable non-fatal memory error configuration register"]
    pub SIUNMECR: u32,
    #[doc = "Station interface uncorrectable non-fatal memory error status register 0"]
    pub SIUNMESR0: u32,
    #[doc = "Station interface uncorrectable non-fatal memory error status register 1"]
    pub SIUNMESR1: u32,
    #[doc = "Station interface uncorrectable non-fatal memory error count register"]
    pub SIUNMECTR: u32,
    #[doc = "Station interface uncorrectable fatal memory error configuration register"]
    pub SIUFMECR: u32,
    #[doc = "Station interface uncorrectable fatal memory error status register 0"]
    pub SIUFMESR0: u32,
    #[doc = "Station interface uncorrectable fatal memory error status register 1"]
    pub SIUFMESR1: u32,
    _reserved30: [u8; 0x01a4],
    #[doc = "Station interface MAC address filter table capability register"]
    pub SIMAFTCAPR: u32,
    _reserved31: [u8; 0xfc],
    #[doc = "Station interface VLAN filter table capability register"]
    pub SIVFTCAPR: u32,
    _reserved32: [u8; 0x6efc],
    #[doc = "Transmitter and Receiver Buffer descriptor ring register set."]
    pub BDR_NUM: [BDRNUM::RegisterBlock; 10usize],
}
#[doc = "Station interface mode register"]
pub mod SIMR {
    pub use crate::RW as access;
    #[doc = "RSS classification enable 0 RSS classification is disabled 1 RSS classification is enabled This field is valid if SIPCAPR0\\[RSS\\] is set to 1"]
    pub mod RSSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive no unicast-cast mode Suppresses uni-cast receive for the SI"]
    pub mod RNUM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive no multi-cast mode Suppresses multi-cast receive for the SI"]
    pub mod RNMM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive no broad-cast mode Suppresses broad-cast receive for the SI"]
    pub mod RNBM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "VLAN to IPV mapping enable Maps the VLAN PCP/DEI to internal priority value (IPV)"]
    pub mod V2IPVE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Default receive group The default receive group is used when there is no match for RFS and RSS is disabled"]
    pub mod DEFAULT_RX_GROUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable station interface"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface status register"]
pub mod SISR {
    pub use crate::RO as access;
    #[doc = "Transmit busy"]
    pub mod TX_BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SI MAC unicast promiscuous"]
    pub mod MAC_UP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SI MAC multicast promiscuous"]
    pub mod MAC_MP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SI VLAN promiscuous"]
    pub mod VLAN_P {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SI VLAN untagged frames accepted"]
    pub mod VLAN_UTA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface current time register 0"]
pub mod SICTR0 {
    pub use crate::RO as access;
    #[doc = "Time in units of nanoseconds - low order 32-bits (31-0)."]
    pub mod CURR_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface current time register 1"]
pub mod SICTR1 {
    pub use crate::RO as access;
    #[doc = "Time in units of nanoseconds - high order 32-bits (63-32)."]
    pub mod CURR_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface port capability register 0"]
pub mod SIPCAPR0 {
    pub use crate::RO as access;
    #[doc = "Receive Flow Steering"]
    pub mod RFS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supported"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame Preemption"]
    pub mod FP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not Supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Time Gate Scheduling"]
    pub mod TGS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supported"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Time Specific Departure"]
    pub mod TSD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not Supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Credit Based Shaping (CBS)"]
    pub mod CBS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not Supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive Side Scaling"]
    pub mod RSS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supported"]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Per-Stream Filtering and Policing (IEEE 802.1Qci)"]
    pub mod PSFP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not Supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ingress Port Filtering"]
    pub mod IPFLT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Rate Policing"]
    pub mod RP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not Supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Wake-on-LAN"]
    pub mod WO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Functional Safety"]
    pub mod FS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not supported"]
            pub const NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const SUPPORTED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface port capability register 1"]
pub mod SIPCAPR1 {
    pub use crate::RO as access;
    #[doc = "Number of traffic classes 0 - 1 Traffic class (0) 1 - 2 Traffic classes (0-1)"]
    pub mod NUM_TCS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of multicast hash entries per SI 00 64 multicast addresses 01 128 multicast addresses 10 256 multicast addresses 11 512 multicast addresses"]
    pub mod NUM_MCH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of unicast hash entries (1 bit per entry) per SI 00 64 unicast addresses 01 128 unicast addresses 10 256 unicast addresses 11 512 unicast addresses"]
    pub mod NUM_UCH {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of MSI-X vectors per physical/virtual function Range: 1..64 Formula: NUM_MSIX+1"]
    pub mod NUM_MSIX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates the number of IPVs supported. 0: 8 IPVs 1: 16 IPVs"]
    pub mod NUM_IPV {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface receive BDR group control register"]
pub mod SIRBGCR {
    pub use crate::RW as access;
    #[doc = "Number of groups Number of groups used, 0-2"]
    pub mod NUM_GROUPS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of rings per group The number of rings per groups is RINGS_PER_GROUP + 1, e"]
    pub mod RINGS_PER_GROUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface buffer cache attribute register"]
pub mod SIBCAR {
    pub use crate::RW as access;
    #[doc = "Buffer descriptor write cache type This is the cache attribute setting used when NETC writes the buffer descriptor to memory, which includes receive and transmit BD update"]
    pub mod BD_WRCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Buffer descriptor write domain This is the domain attribute setting used when NETC writes the buffer descriptor to memory, which includes receive and transmit BD update"]
    pub mod BD_WRDOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Buffer descriptor write snoop See table above for valid settings."]
    pub mod BD_WRSNP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write data cache type This is the cache attribute setting used when NETC writes frame data to memory on receive"]
    pub mod WRCACHE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write data domain This is the domain attribute setting used when NETC writes frame data to memory on receive"]
    pub mod WRDOMAIN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write data snoop See table above for valid settings."]
    pub mod WRSNP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Buffer descriptor read cache type This is the cache attribute setting used when NETC read the buffer descriptor from memory for transmit"]
    pub mod BD_RDCACHE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Buffer descriptor read domain This is the domain attribute setting used when NETC read the buffer descriptor from memory for transmit"]
    pub mod BD_RDDOMAIN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Buffer descriptor read snoop See table above for valid settings."]
    pub mod BD_RDSNP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read data cache type This is the cache attribute setting used when NETC reads frame data from memory for transmit"]
    pub mod RDCACHE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read data domain This is the domain attribute setting used when NETC reads frame data from memory for transmit"]
    pub mod RDDOMAIN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read data snoop See table above for valid settings."]
    pub mod RDSNP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface command cache attribute register"]
pub mod SICCAR {
    pub use crate::RW as access;
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when NETC writes the buffer descriptor to memory, which includes receive and transmit BD update"]
    pub mod CBD_WRCACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when NETC writes the buffer descriptor to memory, which includes receive and transmit BD update"]
    pub mod CBD_WRDOMAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command buffer descriptor write snoop See table above for valid settings."]
    pub mod CBD_WRSNP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write data cache type This is the cache attribute setting used when NETC writes frame data to memory on receive"]
    pub mod CWRCACHE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write data domain This is the domain attribute setting used when NETC writes frame data to memory on receive"]
    pub mod CWRDOMAIN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write data snoop See table above for valid settings."]
    pub mod CWRSNP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when NETC read the buffer descriptor from memory for transmit"]
    pub mod CBD_RDCACHE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when NETC read the buffer descriptor from memory for transmit"]
    pub mod CBD_RDDOMAIN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Command buffer descriptor read snoop See table above for valid settings."]
    pub mod CBD_RDSNP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read data cache type This is the cache attribute setting used when NETC reads frame data from memory for transmit"]
    pub mod CRDCACHE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read data domain This is the domain attribute setting used when NETC reads frame data from memory for transmit"]
    pub mod CRDDOMAIN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Read data snoop See table above for valid settings."]
    pub mod CRDSNP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface primary MAC address register 0"]
pub mod SIPMAR0 {
    pub use crate::RO as access;
    #[doc = "Primary MAC address This field is defined in network byte order (big-endian)"]
    pub mod PRIM_MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface primary MAC address register 1"]
pub mod SIPMAR1 {
    pub use crate::RO as access;
    #[doc = "Primary MAC address This field is defined in network byte order (big-endian)"]
    pub mod PRIM_MAC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface custom VLAN register 1"]
pub mod SICVLANR1 {
    pub use crate::RO as access;
    #[doc = "Ethertype"]
    pub mod ETYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0 not valid 1 valid"]
    pub mod V {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface custom VLAN register 2"]
pub mod SICVLANR2 {
    pub use crate::RO as access;
    #[doc = "Ethertype"]
    pub mod ETYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0 not valid 1 valid"]
    pub mod V {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface VLAN to IPV mapping register 0"]
pub mod SIVLANIPVMR0 {
    pub use crate::RW as access;
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface VLAN to IPV mapping register 1"]
pub mod SIVLANIPVMR1 {
    pub use crate::RW as access;
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_8 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_9 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_10 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_11 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_12 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_13 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_14 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IPV value used for VLAN PCP+DEI attribute."]
    pub mod PCP_DEI_15 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface IPV to ring mapping register"]
pub mod SIIPVBDRMR0 {
    pub use crate::RW as access;
    #[doc = "BD ring used within the group for IPV 0."]
    pub mod IPV0BDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BD ring used within the group for IPV 1."]
    pub mod IPV1BDR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BD ring used within the group for IPV 2."]
    pub mod IPV2BDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BD ring used within the group for IPV 3."]
    pub mod IPV3BDR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BD ring used within the group for IPV 4."]
    pub mod IPV4BDR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BD ring used within the group for IPV 5."]
    pub mod IPV5BDR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BD ring used within the group for IPV 6."]
    pub mod IPV6BDR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "BD ring used within the group for IPV 7."]
    pub mod IPV7BDR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Virtual station interface message send register"]
pub mod VSIMSGSR {
    pub use crate::RO as access;
    #[doc = "Message busy This bit will clear automatically and status field MS set accordingly"]
    pub mod MB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Message status The status bit indicates if the command was executed successfully"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Message code This field provides additional information about the completion status of the command"]
    pub mod MC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Virtual station interface message receive register"]
pub mod VSIMSGRR {
    pub use crate::RO as access;
    #[doc = "Message Received When set, indicates that a message, provided by field MC, has been received from the PSI"]
    pub mod MR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Message code This field provides a user defined 16-bit message value from the PSI."]
    pub mod MC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Virtual station interface message send register 0"]
pub mod VSIMSGSNDAR0 {
    pub use crate::RW as access;
    #[doc = "Message size Size of message as a multiple of 32 bytes"]
    pub mod MSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Message address low Lower address bits 31-6 of message to be sent. Message is 32B address aligned."]
    pub mod ADDRL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Virtual station interface message send address register 1"]
pub mod VSIMSGSNDAR1 {
    pub use crate::RW as access;
    #[doc = "Message address high High address bits 63-32 of message to be sent."]
    pub mod ADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface receive octets counter (ifInOctets) 0"]
pub mod SIROCT0 {
    pub use crate::RO as access;
    #[doc = "Counter value - low-order bits 32-0"]
    pub mod ROCT_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface receive octets counter (ifInOctets) 1"]
pub mod SIROCT1 {
    pub use crate::RO as access;
    #[doc = "Counter value - high-order bits 63-32"]
    pub mod ROCT_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface receive frame counter (aFrameReceivedOK) 0"]
pub mod SIRFRM0 {
    pub use crate::RO as access;
    #[doc = "Counter value - low-order bits 31-0"]
    pub mod RFRM_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface receive frame counter (aFrameReceivedOK) 1"]
pub mod SIRFRM1 {
    pub use crate::RO as access;
    #[doc = "Counter value - high-order bits 63-32"]
    pub mod RFRM_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface receive unicast frame counter (ifInUcastPkts) 0"]
pub mod SIRUCA0 {
    pub use crate::RO as access;
    #[doc = "Counter value - low-order bits 31-0"]
    pub mod RUCA_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface receive unicast frame counter (ifInUcastPkts) 1"]
pub mod SIRUCA1 {
    pub use crate::RO as access;
    #[doc = "Counter value - high-order bits 63-32"]
    pub mod RUCA_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface receive multicast frame counter (ifInMulticastPkts) 0"]
pub mod SIRMCA0 {
    pub use crate::RO as access;
    #[doc = "Counter value - high-order bits 31-0"]
    pub mod RMCA_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface receive multicast frame counter (ifInMulticastPkts) 1"]
pub mod SIRMCA1 {
    pub use crate::RO as access;
    #[doc = "Counter value - high-order bits 63-32"]
    pub mod RMCA_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface transmit octets counter (ifOutOctets) 0"]
pub mod SITOCT0 {
    pub use crate::RO as access;
    #[doc = "Counter value - low-order bits 31-0"]
    pub mod TOCT_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface transmit octets counter (ifOutOctets) 1"]
pub mod SITOCT1 {
    pub use crate::RO as access;
    #[doc = "Counter value - high-order bits 63-32"]
    pub mod TOCT_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface transmit frame counter (aFrameTransmittedOK) 0"]
pub mod SITFRM0 {
    pub use crate::RO as access;
    #[doc = "Counter value - low-order bits 31-0"]
    pub mod TFRM_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface transmit frame counter (aFrameTransmittedOK) 1"]
pub mod SITFRM1 {
    pub use crate::RO as access;
    #[doc = "Counter value - high-order bits 63-32"]
    pub mod TFRM_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface transmit unicast frame counter (ifOutUcastPkts) 0"]
pub mod SITUCA0 {
    pub use crate::RO as access;
    #[doc = "Counter value - low-order bits 31-0"]
    pub mod TUCA_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface transmit unicast frame counter (ifOutUcastPkts) 1"]
pub mod SITUCA1 {
    pub use crate::RO as access;
    #[doc = "Counter value - high-order bits 63-32"]
    pub mod TUCA_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface transmit multicast frame counter (ifOutMulticastPkts) 0"]
pub mod SITMCA0 {
    pub use crate::RO as access;
    #[doc = "Counter value - low-order bits 31-0"]
    pub mod TMCA_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface transmit multicast frame counter (ifOutMulticastPkts) 1"]
pub mod SITMCA1 {
    pub use crate::RO as access;
    #[doc = "Counter value - high-order bits 63-32"]
    pub mod TMCA_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface boot loader parameter register a"]
pub mod SIBLPR {
    pub use crate::RO as access;
    #[doc = "Boot loader parameter value."]
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
#[doc = "Station interface command BDR mode register"]
pub mod SICBDRMR {
    pub use crate::RW as access;
    #[doc = "Enable command buffer descriptor ring 0 Disabled"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface command BDR status register"]
pub mod SICBDRSR {
    pub use crate::RO as access;
    #[doc = "Busy. The command BD ring is busy processing commands 0 Idle 1 Busy"]
    pub mod BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface command BDR base address register 0"]
pub mod SICBDRBAR0 {
    pub use crate::RW as access;
    #[doc = "Lower address bits 31-7."]
    pub mod ADDRL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface command BDR base address register 1"]
pub mod SICBDRBAR1 {
    pub use crate::RW as access;
    #[doc = "Upper address bits 63-32."]
    pub mod ADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface command BDR producer index register"]
pub mod SICBDRPIR {
    pub use crate::RW as access;
    #[doc = "Command buffer descriptor ring producer index"]
    pub mod BDR_INDEX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface command BDR consumer index register"]
pub mod SICBDRCIR {
    pub use crate::RW as access;
    #[doc = "Command buffer descriptor ring consumer index"]
    pub mod BDR_INDEX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface command BDR length register"]
pub mod SICBDRLENR {
    pub use crate::RW as access;
    #[doc = "BD ring length Size of ring in sets of 8 BDs. Maximum ring size is 1K."]
    pub mod LENGTH {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface command BDR interrupt enable register"]
pub mod SICBDRIER {
    pub use crate::RW as access;
    #[doc = "Command BD completion interrupt enable 0 Disabled 1 Enabled"]
    pub mod CBDCIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface command BDR interrupt detect register"]
pub mod SICBDRIDR {
    pub use crate::RW as access;
    #[doc = "Command BD completed 0 No BD with CI=1 completed 1 Processed BD with CI=1"]
    pub mod CBDC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface capability register 0"]
pub mod SICAPR0 {
    pub use crate::RO as access;
    #[doc = "Number of transmit buffer descriptor rings assigned to the SI"]
    pub mod NUM_TX_BDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of receive buffer descriptor rings assigned to the SI"]
    pub mod NUM_RX_BDR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of MAC addresses Formula: NUM_MAC_ADDR+1"]
    pub mod NUM_MAC_ADDR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface capability register 1"]
pub mod SICAPR1 {
    pub use crate::RO as access;
    #[doc = "Max number of receive buffer descriptor ring groups available to the SI"]
    pub mod NUM_RX_GRP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface capability register 2"]
pub mod SICAPR2 {
    pub use crate::RO as access;
    #[doc = "VLAN types permitted Bit: 0 Standard C-VLAN 0x8100 1 Standard S-VLAN 0x88A8 2 Custom VLAN as defined by CVLANR1\\[ETYPE\\] 3 Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    pub mod VTP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Virtual station interface interrupt enable register"]
pub mod VSIIER {
    pub use crate::RW as access;
    #[doc = "Message sent interrupt enable 0 Disabled 1 Enabled"]
    pub mod MSIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Message received interrupt enable 0 Disabled 1 Enabled"]
    pub mod MRIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Virtual station interface interrupt detect register"]
pub mod VSIIDR {
    pub use crate::RW as access;
    #[doc = "Summary of detected interrupts for all transmit rings belonging to the SI 0 No interrupt detected for transmit ring(s) 1 Interrupt detected for transmit ring(s), see SITXIDR Read only, clear using SITXIDR"]
    pub mod TXR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Message sent 0 Message sent to PSI has not completed 1 Message sent to PSI has completed and response received"]
    pub mod MS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Message received 0 No message 1 Message received from PSI, see VSIMSGRR"]
    pub mod MR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for all receive rings belonging to the SI 0 No interrupt detected for receive ring(s) 1 Interrupt detected for receive ring(s), see SIRXIDR Read only, clear using SIRXIDR"]
    pub mod RXR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface transmit interrupt detect register 0"]
pub mod SITXIDR0 {
    pub use crate::RW as access;
    #[doc = "Summary of detected threshold interrupts for transmit ring 0 assigned to SI 0 No interrupt detected for transmit ring 0 1 Threshold interrupt detected for transmit ring 0"]
    pub mod TXT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 1 assigned to SI 0 No interrupt detected for transmit ring 1 1 Threshold interrupt detected for transmit ring 1"]
    pub mod TXT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 2 assigned to SI 0 No interrupt detected for transmit ring 2 1 Threshold interrupt detected for transmit ring 2"]
    pub mod TXT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 3 assigned to SI 0 No interrupt detected for transmit ring 3 1 Threshold interrupt detected for transmit ring 3"]
    pub mod TXT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 4 assigned to SI 0 No interrupt detected for transmit ring 4 1 Threshold interrupt detected for transmit ring 4"]
    pub mod TXT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 5 assigned to SI 0 No interrupt detected for transmit ring 5 1 Threshold interrupt detected for transmit ring 5"]
    pub mod TXT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 6 assigned to SI 0 No interrupt detected for transmit ring 6 1 Threshold interrupt detected for transmit ring 6"]
    pub mod TXT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 7 assigned to SI 0 No interrupt detected for transmit ring 7 1 Threshold interrupt detected for transmit ring 7"]
    pub mod TXT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 8 assigned to SI 0 No interrupt detected for transmit ring 8 1 Threshold interrupt detected for transmit ring 8"]
    pub mod TXT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 9 assigned to SI 0 No interrupt detected for transmit ring 9 1 Threshold interrupt detected for transmit ring 9"]
    pub mod TXT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 10 assigned to SI 0 No interrupt detected for transmit ring 10 1 Threshold interrupt detected for transmit ring 10"]
    pub mod TXT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 11 assigned to SI 0 No interrupt detected for transmit ring 11 1 Threshold interrupt detected for transmit ring 11"]
    pub mod TXT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 12 assigned to SI 0 No interrupt detected for transmit ring 12 1 Threshold interrupt detected for transmit ring 12"]
    pub mod TXT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected threshold interrupts for transmit ring 13 assigned to SI 0 No interrupt detected for transmit ring 13 1 Threshold interrupt detected for transmit ring 13"]
    pub mod TXT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 0 assigned to SI 0 No interrupt detected for transmit ring 0 1 Frame transmit interrupt detected for transmit ring 0"]
    pub mod TXF0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 1 assigned to SI 0 No interrupt detected for transmit ring 1 1 Frame transmit interrupt detected for transmit ring 1"]
    pub mod TXF1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 2 assigned to SI 0 No interrupt detected for transmit ring 2 1 Frame transmit interrupt detected for transmit ring 2"]
    pub mod TXF2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 3 assigned to SI 0 No interrupt detected for transmit ring 3 1 Frame transmit interrupt detected for transmit ring 3"]
    pub mod TXF3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 4 assigned to SI 0 No interrupt detected for transmit ring 4 1 Frame transmit interrupt detected for transmit ring 4"]
    pub mod TXF4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 5 assigned to SI 0 No interrupt detected for transmit ring 5 1 Frame transmit interrupt detected for transmit ring 5"]
    pub mod TXF5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 6 assigned to SI 0 No interrupt detected for transmit ring 6 1 Frame transmit interrupt detected for transmit ring 6"]
    pub mod TXF6 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 7 assigned to SI 0 No interrupt detected for transmit ring 7 1 Frame transmit interrupt detected for transmit ring 7"]
    pub mod TXF7 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 8 assigned to SI 0 No interrupt detected for transmit ring 8 1 Frame transmit interrupt detected for transmit ring 8"]
    pub mod TXF8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 9 assigned to SI 0 No interrupt detected for transmit ring 9 1 Frame transmit interrupt detected for transmit ring 9"]
    pub mod TXF9 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 10 assigned to SI 0 No interrupt detected for transmit ring 10 1 Frame transmit interrupt detected for transmit ring 10"]
    pub mod TXF10 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 11 assigned to SI 0 No interrupt detected for transmit ring 11 1 Frame transmit interrupt detected for transmit ring 11"]
    pub mod TXF11 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 12 assigned to SI 0 No interrupt detected for transmit ring 12 1 Frame transmit interrupt detected for transmit ring 12"]
    pub mod TXF12 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected transmit frame interrupts for transmit ring 13 assigned to SI 0 No interrupt detected for transmit ring 13 1 Frame transmit interrupt detected for transmit ring 13"]
    pub mod TXF13 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface receive interrupt detect register 0"]
pub mod SIRXIDR0 {
    pub use crate::RW as access;
    #[doc = "Summary of detected interrupts for receive ring 0 assigned to SI 0 No interrupt detected for receive ring 0 1 Interrupt detected for receive ring 0"]
    pub mod RX0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 1 assigned to SI 0 No interrupt detected for receive ring 1 1 Interrupt detected for receive ring 1"]
    pub mod RX1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 2 assigned to SI 0 No interrupt detected for receive ring 2 1 Interrupt detected for receive ring 2"]
    pub mod RX2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 3 assigned to SI 0 No interrupt detected for receive ring 3 1 Interrupt detected for receive ring 3"]
    pub mod RX3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 4 assigned to SI 0 No interrupt detected for receive ring 4 1 Interrupt detected for receive ring 4"]
    pub mod RX4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 5 assigned to SI 0 No interrupt detected for receive ring 5 1 Interrupt detected for receive ring 5"]
    pub mod RX5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 6 assigned to SI 0 No interrupt detected for receive ring 6 1 Interrupt detected for receive ring 6"]
    pub mod RX6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 7 assigned to SI 0 No interrupt detected for receive ring 7 1 Interrupt detected for receive ring 7"]
    pub mod RX7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 8 assigned to SI 0 No interrupt detected for receive ring 8 1 Interrupt detected for receive ring 8"]
    pub mod RX8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 9 assigned to SI 0 No interrupt detected for receive ring 9 1 Interrupt detected for receive ring 9"]
    pub mod RX9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 10 assigned to SI 0 No interrupt detected for receive ring 10 1 Interrupt detected for receive ring 10"]
    pub mod RX10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 11 assigned to SI 0 No interrupt detected for receive ring 11 1 Interrupt detected for receive ring 11"]
    pub mod RX11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 12 assigned to SI 0 No interrupt detected for receive ring 12 1 Interrupt detected for receive ring 12"]
    pub mod RX12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Summary of detected interrupts for receive ring 13 assigned to SI 0 No interrupt detected for receive ring 13 1 Interrupt detected for receive ring 13"]
    pub mod RX13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface MSI-X vector register"]
pub mod SIMSIVR {
    pub use crate::RW as access;
    #[doc = "Vector Index into MSI-X address/data table"]
    pub mod VECTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface command MSI-X vector register"]
pub mod SICMSIVR {
    pub use crate::RW as access;
    #[doc = "Vector Index into MSI-X address/data table"]
    pub mod VECTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface timer interrupt enable register"]
pub mod SITMRIER {
    pub use crate::RW as access;
    #[doc = "Timer synchronous state change interrupt enable 0 Disabled 1 Enabled"]
    pub mod SYNCE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface timer interrupt detect register"]
pub mod SITMRIDR {
    pub use crate::RW as access;
    #[doc = "Timer synchronous state change detected when set."]
    pub mod SYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface timer MSI-X vector register"]
pub mod SITMRMSIVR {
    pub use crate::RW as access;
    #[doc = "Vector Index into MSI-X address/data table"]
    pub mod VECTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface MSI-X transmit ring a vector register"]
pub mod SIMSITRVR {
    pub use crate::RW as access;
    #[doc = "Vector Index into MSI-X address/data table"]
    pub mod VECTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface MSI-X receive ring a vector register"]
pub mod SIMSIRRVR {
    pub use crate::RW as access;
    #[doc = "Vector Index into MSI-X address/data table"]
    pub mod VECTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface correctable memory error configuration register"]
pub mod SICMECR {
    pub use crate::RW as access;
    #[doc = "Threshold"]
    pub mod THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface correctable memory error status register"]
pub mod SICMESR {
    pub use crate::RW as access;
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Single-bit ECC error"]
    pub mod SBEE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface correctable memory error count register"]
pub mod SICMECTR {
    pub use crate::RO as access;
    #[doc = "Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface uncorrectable programming error configuration register"]
pub mod SIUPECR {
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
#[doc = "Station interface uncorrectable programming error status register"]
pub mod SIUPESR {
    pub use crate::RW as access;
    #[doc = "Station interface disabled drop error."]
    pub mod DROP_SI_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Station interface ring disabled drop error."]
    pub mod DROP_RING_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Non-existing receive ring error."]
    pub mod DROP_RING_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
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
    #[doc = "Programming error"]
    pub mod PE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface uncorrectable programming error count register"]
pub mod SIUPECTR {
    pub use crate::RO as access;
    #[doc = "Count"]
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
#[doc = "Station interface uncorrectable non-fatal system bus error configuration register"]
pub mod SIUNSBECR {
    pub use crate::RW as access;
    #[doc = "Threshold"]
    pub mod THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface uncorrectable non-fatal system bus error status register"]
pub mod SIUNSBESR {
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
#[doc = "Station interface uncorrectable non-fatal system bus error count register"]
pub mod SIUNSBECTR {
    pub use crate::RO as access;
    #[doc = "Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface uncorrectable fatal system bus error configuration register"]
pub mod SIUFSBECR {
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
#[doc = "Station interface uncorrectable fatal system bus error status register"]
pub mod SIUFSBESR {
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
#[doc = "Station interface uncorrectable non-fatal memory error configuration register"]
pub mod SIUNMECR {
    pub use crate::RW as access;
    #[doc = "Threshold"]
    pub mod THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
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
#[doc = "Station interface uncorrectable non-fatal memory error status register 0"]
pub mod SIUNMESR0 {
    pub use crate::RW as access;
    #[doc = "Syndrome"]
    pub mod SYNDROME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Multi-bit ECC error"]
    pub mod MBEE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface uncorrectable non-fatal memory error status register 1"]
pub mod SIUNMESR1 {
    pub use crate::RO as access;
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface uncorrectable non-fatal memory error count register"]
pub mod SIUNMECTR {
    pub use crate::RO as access;
    #[doc = "Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface uncorrectable fatal memory error configuration register"]
pub mod SIUFMECR {
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
#[doc = "Station interface uncorrectable fatal memory error status register 0"]
pub mod SIUFMESR0 {
    pub use crate::RW as access;
    #[doc = "Syndrome"]
    pub mod SYNDROME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
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
    #[doc = "Multi-bit ECC error"]
    pub mod MBEE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface uncorrectable fatal memory error status register 1"]
pub mod SIUFMESR1 {
    pub use crate::RO as access;
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface MAC address filter table capability register"]
pub mod SIMAFTCAPR {
    pub use crate::RO as access;
    #[doc = "Number of MAC address filter table entries for use by SI"]
    pub mod NUM_MAC_AFTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Station interface VLAN filter table capability register"]
pub mod SIVFTCAPR {
    pub use crate::RO as access;
    #[doc = "Number of VLAN filter table entries for use by SI"]
    pub mod NUM_VLAN_FTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
pub mod BDRNUM {
    #[doc = "Transmitter and Receiver Buffer descriptor ring register set."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Tx BDR a mode register"]
        pub TBMR: u32,
        #[doc = "Tx BDR a status register"]
        pub TBSR: u32,
        _reserved0: [u8; 0x08],
        #[doc = "Tx BDR a base address register 0"]
        pub TBBAR0: u32,
        #[doc = "Tx BDR a base address register 1"]
        pub TBBAR1: u32,
        #[doc = "Tx BDR a producer index register"]
        pub TBPIR: u32,
        #[doc = "Tx BDR a consumer index register"]
        pub TBCIR: u32,
        #[doc = "Tx BDR a length register"]
        pub TBLENR: u32,
        _reserved1: [u8; 0x7c],
        #[doc = "Tx BDR a interrupt enable register"]
        pub TBIER: u32,
        #[doc = "Tx BDR a interrupt detect register"]
        pub TBIDR: u32,
        #[doc = "Tx BDR a interrupt coalescing register 0"]
        pub TBICR0: u32,
        #[doc = "Tx BDR a interrupt coalescing register 1"]
        pub TBICR1: u32,
        _reserved2: [u8; 0x50],
        #[doc = "Rx BDR a mode register"]
        pub RBMR: u32,
        #[doc = "Rx BDR a status register"]
        pub RBSR: u32,
        #[doc = "Rx BDR a buffer size register"]
        pub RBBSR: u32,
        #[doc = "Rx BDR a consumer index register"]
        pub RBCIR: u32,
        #[doc = "Rx BDR a base address register 0"]
        pub RBBAR0: u32,
        #[doc = "Rx BDR a base address register 1"]
        pub RBBAR1: u32,
        #[doc = "Rx BDR a producer index register"]
        pub RBPIR: u32,
        _reserved3: [u8; 0x04],
        #[doc = "Rx BDR a length register"]
        pub RBLENR: u32,
        _reserved4: [u8; 0x5c],
        #[doc = "Rx BDR a drop count register"]
        pub RBDCR: u32,
        _reserved5: [u8; 0x1c],
        #[doc = "Rx BDR a interrupt enable register"]
        pub RBIER: u32,
        #[doc = "Rx BDR a interrupt detect register"]
        pub RBIDR: u32,
        #[doc = "Rx BDR a interrupt coalescing register 0"]
        pub RBICR0: u32,
        #[doc = "Rx BDR a interrupt coalescing register 1"]
        pub RBICR1: u32,
        _reserved6: [u8; 0x50],
    }
    #[doc = "Tx BDR a mode register"]
    pub mod TBMR {
        pub use crate::RW as access;
        #[doc = "Priority Priority of the transmit buffer descriptor ring"]
        pub mod PRIO {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "WRR weight Weight used for arbitration when two or more rings have the same priority within the same VSI"]
        pub mod WRR {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "User CRC provided Determines if user provides CRC32 (FCS) at the end of the frame"]
        pub mod CRC {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "VLAN Insert Hint If set (b1) then SW intends to use VLAN insertion offload by providing VLAN tag data in the Tx BD"]
        pub mod VIH {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Enable transmit buffer descriptor ring 0: Disabled"]
        pub mod EN {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Tx BDR a status register"]
    pub mod TBSR {
        pub use crate::RW as access;
        #[doc = "Busy"]
        pub mod BUSY {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "System bus error A system bus error has occurred during one or more transactions related to this transmit ring, including possibly the transmit BD writeback entry itself"]
        pub mod SBE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Tx BDR a base address register 0"]
    pub mod TBBAR0 {
        pub use crate::RW as access;
        #[doc = "Lower address bits 31-7."]
        pub mod ADDRL {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0x01ff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Tx BDR a base address register 1"]
    pub mod TBBAR1 {
        pub use crate::RW as access;
        #[doc = "Upper address bits 63-32."]
        pub mod ADDRH {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Tx BDR a producer index register"]
    pub mod TBPIR {
        pub use crate::RW as access;
        #[doc = "Transmit buffer descriptor ring producer index"]
        pub mod BDR_INDEX {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Tx BDR a consumer index register"]
    pub mod TBCIR {
        pub use crate::RW as access;
        #[doc = "Transmit buffer descriptor ring consumer index"]
        pub mod BDR_INDEX {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Status identifier"]
        pub mod STAT_ID {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Tx BDR a length register"]
    pub mod TBLENR {
        pub use crate::RW as access;
        #[doc = "BD ring length Size of ring in sets of 8 BDs. Maximum ring size is 64K."]
        pub mod LENGTH {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0x3fff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Tx BDR a interrupt enable register"]
    pub mod TBIER {
        pub use crate::RW as access;
        #[doc = "Transmit threshold interrupt enable 0 Disabled 1 Enabled"]
        pub mod TXTIE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Transmit frame interrupt enable 0 Disabled 1 Enabled"]
        pub mod TXFIE {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Tx BDR a interrupt detect register"]
    pub mod TBIDR {
        pub use crate::RO as access;
        #[doc = "Transmit threshold 0 No threshold event detected 1 Transmit ring has transmitted at least the number of packets specified by TBaICR0\\[ICPT\\] or the threshold timer has expired since last transmitted packet as specified by TBaICR1\\[ICTT\\]"]
        pub mod TXT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Transmit of frame 0 No transmit of frame detected with BD\\[FI\\]=1 1 Transmit of frame detected with BD\\[FI\\]=1"]
        pub mod TXF {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Tx BDR a interrupt coalescing register 0"]
    pub mod TBICR0 {
        pub use crate::RW as access;
        #[doc = "Interrupt coalescing packet threshold While interrupt coalescing is enabled, ICEN=1, this values determines the minimum number of packets transmitted before raising an interrupt"]
        pub mod ICPT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x0f << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Interrupt coalescing enable 0 Interrupt coalescing is disabled 1 Interrupt coalescing is enabled"]
        pub mod ICEN {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Tx BDR a interrupt coalescing register 1"]
    pub mod TBICR1 {
        pub use crate::RW as access;
        #[doc = "Interrupt coalescing timer threshold, specified in units of NETC clock cycles"]
        pub mod ICTT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Rx BDR a mode register"]
    pub mod RBMR {
        pub use crate::RW as access;
        #[doc = "Header alignment If set, an additional 2-byte alignment is applied to the header in the 1st buffer descriptor"]
        pub mod AL {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "BD Size A BD ring can use either the standard 16B or extended 32B buffer descriptor format"]
        pub mod BDS {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Congestion mode Determines the congestion scheme for the receive ring"]
        pub mod CM {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "VLAN tag extract enable Controls whether the outer VLAN, as seen by the SI, is extracted"]
        pub mod VTE {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "VLAN tag presentation disable Controls whether the extracted VLAN tag is provided in the Rx BD"]
        pub mod VTPD {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Indicates if CRC32 (FCS) is to be stripped or preserved at the end of the frame"]
        pub mod CRC {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Enable receive buffer descriptor ring 0 Disabled. 1 Enabled."]
        pub mod EN {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Rx BDR a status register"]
    pub mod RBSR {
        pub use crate::RW as access;
        #[doc = "Ring is empty Indicates the ring is empty of received BDs. Valid when RBaMR\\[EN\\]=1."]
        pub mod EMPTY {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "System bus error A system bus error has occurred during one or more transactions related to this receive ring, including possibly the receive BD writeback entry itself"]
        pub mod SBE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Rx BDR a buffer size register"]
    pub mod RBBSR {
        pub use crate::RW as access;
        #[doc = "Buffer size Indicates the buffer size for the buffer pool used"]
        pub mod BSIZE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Rx BDR a consumer index register"]
    pub mod RBCIR {
        pub use crate::RW as access;
        #[doc = "Receive buffer descriptor ring consumer index. Range of index depends on ring size RBaLENR\\[LENGTH\\]."]
        pub mod BDR_INDEX {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Rx BDR a base address register 0"]
    pub mod RBBAR0 {
        pub use crate::RW as access;
        #[doc = "Lower address bits 31-7."]
        pub mod ADDRL {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0x01ff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Rx BDR a base address register 1"]
    pub mod RBBAR1 {
        pub use crate::RW as access;
        #[doc = "Upper address bits 63-32."]
        pub mod ADDRH {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Rx BDR a producer index register"]
    pub mod RBPIR {
        pub use crate::RW as access;
        #[doc = "Receive buffer descriptor ring producer index. Range of index depends on ring size RBaLENR\\[LENGTH\\]."]
        pub mod BDR_INDEX {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Rx BDR a length register"]
    pub mod RBLENR {
        pub use crate::RW as access;
        #[doc = "BD ring length Size of ring in sets of 8 BDs"]
        pub mod LENGTH {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0x3fff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Rx BDR a drop count register"]
    pub mod RBDCR {
        pub use crate::RO as access;
        #[doc = "Count Number of frames dropped due to lack of receive buffer descriptors available."]
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
    #[doc = "Rx BDR a interrupt enable register"]
    pub mod RBIER {
        pub use crate::RW as access;
        #[doc = "Receive threshold interrupt enable 0 Disabled 1 Enabled"]
        pub mod RXTIE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Rx BDR a interrupt detect register"]
    pub mod RBIDR {
        pub use crate::RO as access;
        #[doc = "Receive threshold 0 No threshold event detected 1 Receive ring holds at least the number of packets specified by RBaICR0\\[ICPT\\] or the threshold timer has expired since last received packet as specified by RBaICR1\\[ICTT\\]"]
        pub mod RXT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Rx BDR a interrupt coalescing register 0"]
    pub mod RBICR0 {
        pub use crate::RW as access;
        #[doc = "Interrupt coalescing packet threshold While interrupt coalescing is enabled, ICEN=1, this values determines the minimum number of packets received before raising an interrupt"]
        pub mod ICPT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01ff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Interrupt coalescing enable 0 Interrupt coalescing is disabled 1 Interrupt coalescing is enabled"]
        pub mod ICEN {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Rx BDR a interrupt coalescing register 1"]
    pub mod RBICR1 {
        pub use crate::RW as access;
        #[doc = "Interrupt coalescing timer threshold ,specified in units of NETC clock cycles"]
        pub mod ICTT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
}
