#[doc = "Port"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Port capability register"]
    pub PCAPR: u32,
    #[doc = "Port MAC capability register"]
    pub PMCAPR: u32,
    #[doc = "Port I/O capability register"]
    pub PIOCAPR: u32,
    _reserved0: [u8; 0x04],
    #[doc = "Port configuration register"]
    pub PCR: u32,
    _reserved1: [u8; 0x0c],
    #[doc = "Port MAC address register 0"]
    pub PMAR0: u32,
    #[doc = "Port MAC address register 1"]
    pub PMAR1: u32,
    _reserved2: [u8; 0x28],
    #[doc = "Port TPID acceptance register"]
    pub PTAR: u32,
    #[doc = "Port QoS mode register"]
    pub PQOSMR: u32,
    _reserved3: [u8; 0x28],
    #[doc = "Port parser configuration register"]
    pub PPCR: u32,
    #[doc = "Port ingress port filter configuration register"]
    pub PIPFCR: u32,
    _reserved4: [u8; 0x18],
    #[doc = "Port stream gate configuration register"]
    pub PSGCR: u32,
    _reserved5: [u8; 0x5c],
    #[doc = "Port operational register"]
    pub POR: u32,
    #[doc = "Port status register"]
    pub PSR: u32,
    #[doc = "Port receive SDU overhead register"]
    pub PRXSDUOR: u32,
    #[doc = "Port transmit SDU overhead register"]
    pub PTXSDUOR: u32,
    #[doc = "Port time gate scheduling control register"]
    pub PTGSCR: u32,
    #[doc = "Port time gate scheduling admin gate list status register"]
    pub PTGAGLSR: u32,
    #[doc = "Port time gate scheduling admin gate list length register"]
    pub PTGAGLLR: u32,
    #[doc = "Port time gating operational gate list length register"]
    pub PTGOGLLR: u32,
    #[doc = "Port time gate scheduling advance time offset register"]
    pub PTGSATOR: u32,
    #[doc = "Port time gate scheduling hold advance register"]
    pub PTGSHAR: u32,
    #[doc = "Port time gate scheduling release advance register"]
    pub PTGSRAR: u32,
    #[doc = "Port time gate scheduling hold configuration register"]
    pub PTGSHCR: u32,
    _reserved6: [u8; 0x04],
    #[doc = "Port frame preemption configuration register"]
    pub PFPCR: u32,
    #[doc = "Port default gate state register"]
    pub PDGSR: u32,
    _reserved7: [u8; 0x84],
    #[doc = "Port Rx discard count register"]
    pub PRXDCR: u32,
    _reserved8: [u8; 0x04],
    #[doc = "Port Rx discard count reason register 0"]
    pub PRXDCRR0: u32,
    #[doc = "Port Rx discard count reason register 1"]
    pub PRXDCRR1: u32,
    _reserved9: [u8; 0x30],
    #[doc = "Array of registers: PTGSTCSR, PTCTMSDUR, PTCCBSR0, PTCCBSR1"]
    pub TCT_NUM: [TCTNUM::RegisterBlock; 8usize],
    _reserved10: [u8; 0x0138],
    #[doc = "Port PCP DEI mapping register"]
    pub PPCPDEIMR: u32,
    _reserved11: [u8; 0x24],
    #[doc = "Port ingress stream identification configuration register"]
    pub PISIDCR: u32,
}
#[doc = "Port capability register"]
pub mod PCAPR {
    pub use crate::RO as access;
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    pub mod LINK_TYPE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    pub mod NUM_TC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported Formula: NUM_Q+1 Valid if link is bound to a switch"]
    pub mod NUM_Q {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of congestion groups supported Formula: NUM_CG+1 Valid if link end is bound to a switch port"]
    pub mod NUM_CG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Time Gate Scheduling"]
    pub mod TGS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Credit Based Shaping"]
    pub mod CBS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC capability register"]
pub mod PMCAPR {
    pub use crate::RO as access;
    #[doc = "MAC Variant"]
    pub mod MAC_VAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Egress frame padding capability indicates if egress frames smaller than 64B are padded with zeroes"]
    pub mod EFPAD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates if configurable Preamble and IPG is supported 0: Static Inter Frame Gap (IPG) size is 12B and Preamble is 7B 1: Configurable IPG and Preamble size Valid if MAC_VAR=1"]
    pub mod PIPG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates if Half Duplex Mode is supported on this link"]
    pub mod HD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates if frame preemption is supported"]
    pub mod FP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    pub mod MIN_MPDU {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates the MII protocol supported"]
    pub mod MII_PROT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port I/O capability register"]
pub mod PIOCAPR {
    pub use crate::RO as access;
    #[doc = "PCS protocols supported"]
    pub mod PCS_PROT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IO Variants supported"]
    pub mod IO_VAR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "External MDIO supported."]
    pub mod EMDIO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "RevMII MII rate"]
    pub mod REVMII_RATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reverse Mode Device Configuration"]
    pub mod REVMII {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port configuration register"]
pub mod PCR {
    pub use crate::RW as access;
    #[doc = "Indicates the frame format received/transmitted on the port 0 Ethernet frame format 1 Reserved"]
    pub mod HDR_FMT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "L2 Ethernet DoS Protection Enable 0 L2 IP DoS protection is disabled"]
    pub mod L2DOSE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timer Clock Selection: On receive, this field determines which of the two Rx timestamps (synchronized or free running) is reported to the host"]
    pub mod TIMER_CS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit Port Speed The speed in 10Mbps increments at which the port is assumed to be running for scheduling purposes and to determine if cut-through forwarding can proceed"]
    pub mod PSPEED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC address register 0"]
pub mod PMAR0 {
    pub use crate::RW as access;
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
#[doc = "Port MAC address register 1"]
pub mod PMAR1 {
    pub use crate::RW as access;
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
#[doc = "Port TPID acceptance register"]
pub mod PTAR {
    pub use crate::RW as access;
    #[doc = "Outer VLAN TPID List : bitmap identifying which TPIDs are acceptable as Outer VLAN tag xxx1 Standard C-VLAN 0x8100 xx1x Standard S-VLAN 0x88A8 x1xx Custom VLAN as defined by CVLANR1\\[ETYPE\\] 1xxx Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    pub mod OVTPIDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Outer VLAN TPID List : bitmap identifying which TPIDs are acceptable as Inner VLAN tag xxx1 Standard C-VLAN 0x8100 xx1x Standard S-VLAN 0x88A8 x1xx Custom VLAN as defined by CVLANR1\\[ETYPE\\] 1xxx Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    pub mod IVTPIDL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port QoS mode register"]
pub mod PQOSMR {
    pub use crate::RW as access;
    #[doc = "VLAN tag select: 0 Inner VLAN tag will be used if VE is set 1 Outer VLAN tag will be used if VE is set"]
    pub mod VS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "VLAN enable 0 Defaults are used 1 Enables use of VLAN info to determine IPV and DR (based on VLANIPVMPaR0/1 and VLANDRMPaR)"]
    pub mod VE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Default DR Sets the default DR for the port."]
    pub mod DDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Default IPV Sets the default IPV for the port."]
    pub mod DIPV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mapping profile index"]
    pub mod VQMP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port parser configuration register"]
pub mod PPCR {
    pub use crate::RW as access;
    #[doc = "Unused"]
    pub mod L1PFS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "L2 payload fields size in bytes This is the largest L2 payload size used by any table lookup key by this port"]
    pub mod L2PFS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "L3 header fields present"]
    pub mod L3HFP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No L3 header present. Indicates to the parser of not parsing the L3 header. Parsing in this case would go as far as the L2 header regardless of whether or not there is an L3 header in the frame. This option should not be used if there are any table lookup entries that contain L3/L4 key fields that could be matched against a frame."]
            pub const L3_HDR_NOT_PRESET: u32 = 0;
            #[doc = "Parse L3 header if present in the frame."]
            pub const L3_HDR_PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "L3 payload fields size in bytes"]
    pub mod L3PFS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "L4 Header fields present"]
    pub mod L4HFP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No L4 header present. Indicates to the parser of not parsing the L4 header. Parsing in this case would go as far as the L3 header if configured to parse it (L3HFP=1) regardless of whether or not there is an L4 header in the frame. This option should not be used if there are any table lookup entries that contain L4 key fields that could be matched against a frame."]
            pub const L4_HDR_NOT_PRESENT: u32 = 0;
            #[doc = "Parse L4 header if present in the frame"]
            pub const L4_HDR_PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "L4 payload fields size in bytes"]
    pub mod L4PFS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port ingress port filter configuration register"]
pub mod PIPFCR {
    pub use crate::RW as access;
    #[doc = "0 Ingress Port Filter table lookup is disabled for this port 1 Ingress Port Filter table lookup is enabled for this port"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port stream gate configuration register"]
pub mod PSGCR {
    pub use crate::RW as access;
    #[doc = "Link propagation delay in nanoseconds"]
    pub mod PDELAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Stream Gate Open Gate Check mode"]
    pub mod OGC {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port operational register"]
pub mod POR {
    pub use crate::RW as access;
    #[doc = "Tx Disable."]
    pub mod TXDIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Tx path is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Tx path is disabled."]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Rx Disable."]
    pub mod RXDIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Rx path is enabled."]
            pub const ENABLE: u32 = 0;
            #[doc = "Rx path is disabled."]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port status register"]
pub mod PSR {
    pub use crate::RO as access;
    #[doc = "Transmit busy."]
    pub mod TX_BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Idle"]
            pub const IDLE: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive busy."]
    pub mod RX_BUSY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Idle"]
            pub const IDLE: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port receive SDU overhead register"]
pub mod PRXSDUOR {
    pub use crate::RW as access;
    #[doc = "PPDU Byte count overhead"]
    pub mod PPDU_BCO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MACSec byte count overhead"]
    pub mod MACSEC_BCO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port transmit SDU overhead register"]
pub mod PTXSDUOR {
    pub use crate::RW as access;
    #[doc = "PPDU Byte count overhead"]
    pub mod PPDU_BCO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MACSec byte count overhead"]
    pub mod MACSEC_BCO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port time gate scheduling control register"]
pub mod PTGSCR {
    pub use crate::RW as access;
    #[doc = "Time Gating Enable"]
    pub mod TGE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Time gating disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Time gating enabled."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port time gate scheduling admin gate list status register"]
pub mod PTGAGLSR {
    pub use crate::RO as access;
    #[doc = "Time gated (state of the operational gate control list) 0 No operational gate control list is active 1 Operational gate control list is active"]
    pub mod TG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Administrative gate control list pending 0 No administrative gate control list is configured 1 Administrative gate control list is pending (configured but not installed yet)"]
    pub mod CFG_PEND {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port time gate scheduling admin gate list length register"]
pub mod PTGAGLLR {
    pub use crate::RO as access;
    #[doc = "Administrative gate control list length (number of entries)"]
    pub mod ADMIN_GATE_LIST_LENGTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port time gating operational gate list length register"]
pub mod PTGOGLLR {
    pub use crate::RO as access;
    #[doc = "Operational gate control list length (number of entries)"]
    pub mod OPER_GATE_LIST_LENGTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port time gate scheduling advance time offset register"]
pub mod PTGSATOR {
    pub use crate::RW as access;
    #[doc = "Advance time offset This field specifies the amount of time to advance the IEEE 1588 time scale used by the time-based scheduler, to adjust for the latency encountered across the MAC plus if needed, delays outside of NETC (e"]
    pub mod ADV_TIME_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port time gate scheduling hold advance register"]
pub mod PTGSHAR {
    pub use crate::RO as access;
    #[doc = "This field indicates the amount of time prior to the Set-And-Hold-MAC time slot for asserting the Hold"]
    pub mod HOLDADVANCE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port time gate scheduling release advance register"]
pub mod PTGSRAR {
    pub use crate::RO as access;
    #[doc = "This field indicates the amount of time prior to the Set-And-Release-MAC time slot for asserting the Release"]
    pub mod RELEASEADVANCE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port time gate scheduling hold configuration register"]
pub mod PTGSHCR {
    pub use crate::RW as access;
    #[doc = "Hold-Skew in nanoseconds"]
    pub mod HOLD_SKEW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port frame preemption configuration register"]
pub mod PFPCR {
    pub use crate::RW as access;
    #[doc = "Frame preemption enable for traffic class 0 0 Disabled"]
    pub mod FPE_TC0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame preemption enable for traffic class 1 0 Disabled"]
    pub mod FPE_TC1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame preemption enable for traffic class 2 0 Disabled"]
    pub mod FPE_TC2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame preemption enable for traffic class 3 0 Disabled"]
    pub mod FPE_TC3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame preemption enable for traffic class 4 0 Disabled"]
    pub mod FPE_TC4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame preemption enable for traffic class 5 0 Disabled"]
    pub mod FPE_TC5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame preemption enable for traffic class 6 0 Disabled"]
    pub mod FPE_TC6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Frame preemption enable for traffic class 7 0 Disabled"]
    pub mod FPE_TC7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port default gate state register"]
pub mod PDGSR {
    pub use crate::RW as access;
    #[doc = "Default Gate State for Traffic Class 0 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Default Gate State for Traffic Class 1 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Default Gate State for Traffic Class 2 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Default Gate State for Traffic Class 3 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Default Gate State for Traffic Class 4 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Default Gate State for Traffic Class 5 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Default Gate State for Traffic Class 6 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Default Gate State for Traffic Class 7 Configures the default state of the port's traffic class gates"]
    pub mod DGS_TC7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Closed"]
            pub const ZERO: u32 = 0;
            #[doc = "Open"]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port Rx discard count register"]
pub mod PRXDCR {
    pub use crate::RO as access;
    #[doc = "Number of receive frames discarded"]
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
#[doc = "Port Rx discard count reason register 0"]
pub mod PRXDCRR0 {
    pub use crate::RW as access;
    #[doc = "Pre-Classification Discard Reason Occurs if Parse Classifier Engine (PCE) block does not have any free processing thread to process the received frame"]
    pub mod PCDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Shared Memory Resource Exhaustion Discard Reason Occurs if Ethernet Rx I/F cannot allocate shared internal buffer memory to store the entire received frame"]
    pub mod SMREDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive disable discard reason due to port being in receive disabled state (POR\\[RXDIS\\]=1)"]
    pub mod RXDISDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ingress Port Filter Discard Reason. Cleared when written to 1."]
    pub mod IPFDR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Rate Policer Discard Reason"]
    pub mod RPDR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ingress Stream Forwarding Discard Reason Occurs if frame is associated with an ingress stream whose Forwarding Action (FA) is equal to discard"]
    pub mod ISFDR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Stream Gate Closed Discard Reason Frame received on this port that is dropped because it didn't pass the stream gate"]
    pub mod SGCDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Stream Gate Octet Exceeded Discard Reason Frame dropped due to octet count exceeded maximum specified for the open gate interval Cleared when written to 1"]
    pub mod SGOEDR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ingress Stream Maximum Service Data Unit Exceeded Discard Reason Occurs if packet received is greater than the Maximum SDU size configured for the stream"]
    pub mod MSDUEDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Invalid Table Entry Discard Reason Table entry ID reference is not found in the table (entry has not been added to the table) Table entry ID is outside of its table allocation range Cleared when written to 1"]
    pub mod ITEDR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "ECC Error Discard Reason"]
    pub mod ECCEDR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Station Interface Filter Discard Reason Occurs if packet is discarded by the L2 Filtering function"]
    pub mod SIFDR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Layer 2 Denial of Service Discard Reason Occurs if a packet is discarded due to L2DOS protection"]
    pub mod L2DOSDR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "No Destination Discard Reason"]
    pub mod NODESTDR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port Rx discard count reason register 1"]
pub mod PRXDCRR1 {
    pub use crate::RW as access;
    #[doc = "Entry Id who last incremented discard count"]
    pub mod ENTRYID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Table type which caused the discard"]
    pub mod TT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port PCP DEI mapping register"]
pub mod PPCPDEIMR {
    pub use crate::RW as access;
    #[doc = "Ingress PCP to PCP Mapping Profile instance"]
    pub mod IPCPMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ingress PCP to PCP Mapping Profile is valid. Only applicable if outer VLAN tag is present."]
    pub mod IPCPMPV {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Ingress PCP to PCP Mapping Profile is not valid."]
            pub const INVALID: u32 = 0;
            #[doc = "Ingress frame modification of outer VLAN tag's PCP value is mapped to a new value based on IPCPMP instance"]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Egress PCP to PCP Mapping Profile instance"]
    pub mod EPCPMP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Egress PCP to PCP Mapping Profile is valid."]
    pub mod EPCPMPV {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Egress PCP to PCP Mapping Profile is not valid."]
            pub const INVALID: u32 = 0;
            #[doc = "Egress frame modification of outer VLAN tag's PCP value is mapped to a new value based on EPCPMP instance."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mapping of internal QoS's DR value 0 to VLAN DEI."]
    pub mod DR0DEI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mapping of internal QoS's DR value 1 to VLAN DEI."]
    pub mod DR1DEI {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mapping of internal QoS's DR value 2 to VLAN DEI."]
    pub mod DR2DEI {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Mapping of internal QoS's DR value 3 to VLAN DEI."]
    pub mod DR3DEI {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates if mapping of internal QoS's DR value d to VLAN DEI is enabled."]
    pub mod DRME {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Preserve the DR value in the outer VLAN."]
            pub const PRESERVE: u32 = 0;
            #[doc = "Update DR value in the outer VLAN based on DEnDEI field."]
            pub const UPDATE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port ingress stream identification configuration register"]
pub mod PISIDCR {
    pub use crate::RW as access;
    #[doc = "Indicates which Ingress Stream Identification Key Construction pair to use for this port"]
    pub mod KCPAIR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Key Construction 0 Enable"]
    pub mod KC0EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Key Construction 1 Enable"]
    pub mod KC1EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Default Ingress Stream Entry ID"]
    pub mod ISEID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
pub mod TCTNUM {
    #[doc = "Array of registers: PTGSTCSR, PTCTMSDUR, PTCCBSR0, PTCCBSR1"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Port time gate scheduling traffic class a status register"]
        pub PTGSTCSR: u32,
        _reserved0: [u8; 0x04],
        #[doc = "Port traffic class a transmit maximum SDU register"]
        pub PTCTMSDUR: u32,
        _reserved1: [u8; 0x04],
        #[doc = "Port transmit traffic class a credit based shaper register 0"]
        pub PTCCBSR0: u32,
        #[doc = "Port traffic class a credit based shaper register 1"]
        pub PTCCBSR1: u32,
        _reserved2: [u8; 0x08],
    }
    #[doc = "Port time gate scheduling traffic class a status register"]
    pub mod PTGSTCSR {
        pub use crate::RO as access;
        #[doc = "Look-ahead gate state 0 Gate closed 1 Gate open IERB registers SaTGSLR / EaTGSLR\\[MIN_LOOKAHEAD\\] + the per port register PTGSATOR\\[ADV_TIME_OFFSET\\] specify the advance time of the gate state"]
        pub mod LH_STATE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Port traffic class a transmit maximum SDU register"]
    pub mod PTCTMSDUR {
        pub use crate::RW as access;
        #[doc = "Transmit Maximum SDU size in bytes (i"]
        pub mod MAXSDU {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Specifies the type of PDU/SDU (Protocol/Service Data Unit) whose length is being validated as seen on the link"]
        pub mod SDU_TYPE {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "When cleared, the Tx Max SDU check is performed for Store and Forward frames"]
        pub mod SF_MAXSDU_DIS {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x01 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Port transmit traffic class a credit based shaper register 0"]
    pub mod PTCCBSR0 {
        pub use crate::RW as access;
        #[doc = "Bandwidth"]
        pub mod BW {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x7f << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "Credit Based Shaper Enable"]
        pub mod CBSE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0x01 << offset;
            pub use crate::RW as access;
            #[doc(hidden)]
            pub mod vals {
                #[doc = "Disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Enabled"]
                pub const ENABLE: u32 = 0x01;
            }
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "Port traffic class a credit based shaper register 1"]
    pub mod PTCCBSR1 {
        pub use crate::RW as access;
        #[doc = "hicredit (in credit units)"]
        pub mod HI_CREDIT {
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
