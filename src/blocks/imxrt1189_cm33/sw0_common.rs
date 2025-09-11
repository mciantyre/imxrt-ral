#[doc = "Switch and ENETC common base"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    #[doc = "Ingress port capability register"]
    pub IPCAPR: u32,
    #[doc = "Egress port capability register"]
    pub EPCAPR: u32,
    _reserved1: [u8; 0x08],
    #[doc = "Operational state register"]
    pub OSR: u32,
    _reserved2: [u8; 0x2c],
    #[doc = "Correctable memory error configuration register"]
    pub CMECR: u32,
    #[doc = "Correctable memory error status register"]
    pub CMESR: u32,
    _reserved3: [u8; 0x04],
    #[doc = "Correctable memory error count register"]
    pub CMECTR: u32,
    _reserved4: [u8; 0x10],
    #[doc = "Uncorrectable non-fatal MAC error configuration register"]
    pub UNMACECR: u32,
    #[doc = "Uncorrectable non-fatal MAC error status register"]
    pub UNMACESR: u32,
    _reserved5: [u8; 0x08],
    #[doc = "Uncorrectable non-fatal system bus error configuration register"]
    pub UNSBECR: u32,
    #[doc = "Uncorrectable non-fatal system bus error status register"]
    pub UNSBESR: u32,
    _reserved6: [u8; 0x04],
    #[doc = "Uncorrectable non-fatal system bus error count register"]
    pub UNSBECTR: u32,
    #[doc = "Uncorrectable fatal system bus error configuration register"]
    pub UFSBECR: u32,
    #[doc = "Uncorrectable fatal system bus error status register"]
    pub UFSBESR: u32,
    _reserved7: [u8; 0x08],
    #[doc = "Uncorrectable non-fatal memory error configuration register"]
    pub UNMECR: u32,
    #[doc = "Uncorrectable non-fatal memory error status register 0"]
    pub UNMESR0: u32,
    #[doc = "Uncorrectable non-fatal memory error status register 1"]
    pub UNMESR1: u32,
    #[doc = "Uncorrectable non-fatal memory error count register"]
    pub UNMECTR: u32,
    #[doc = "Uncorrectable fatal memory error configuration register"]
    pub UFMECR: u32,
    #[doc = "Uncorrectable fatal memory error status register 0"]
    pub UFMESR0: u32,
    #[doc = "Uncorrectable fatal memory error status register 1"]
    pub UFMESR1: u32,
    _reserved8: [u8; 0x34],
    #[doc = "Internal MDIO interrupt reason register"]
    pub IMDIOIRR: u32,
    #[doc = "Internal MDIO MSI-X vector register"]
    pub IMDIOMSIVR: u32,
    #[doc = "External MDIO interrupt reason register"]
    pub EMDIOIRR: u32,
    #[doc = "External MDIO MSI-X vector register"]
    pub EMDIOMSIVR: u32,
    _reserved9: [u8; 0x10],
    #[doc = "Time capture configuration register"]
    pub TCCR: u32,
    #[doc = "Time capture interrupt enable register"]
    pub TCIER: u32,
    #[doc = "Time capture receive port interrupt detect register"]
    pub TCRPIDR: u32,
    #[doc = "Time capture receive port status register"]
    pub TCRPSR: u32,
    _reserved10: [u8; 0x04],
    #[doc = "Time capture receive port timestamp register"]
    pub TCRPTSR: u32,
    #[doc = "Time capture MSI-X vector register"]
    pub TCMSIVR: u32,
    _reserved11: [u8; 0xe4],
    #[doc = "Custom VLAN Ethertype register 1"]
    pub CVLANR1: u32,
    #[doc = "Custom VLAN Ethertype register 2"]
    pub CVLANR2: u32,
    #[doc = "Pre-Standard RTAG Ethertype register"]
    pub PSRTAGETR: u32,
    _reserved12: [u8; 0x14],
    #[doc = "DoS L2 configuration register"]
    pub DOSL2CR: u32,
    _reserved13: [u8; 0xdc],
    #[doc = "VLAN to IPV mapping profile register set."]
    pub NUM_PROFILE: [NUMPROFILE::RegisterBlock; 2usize],
    _reserved14: [u8; 0x0320],
    #[doc = "Ingress port filter capability register"]
    pub IPFCAPR: u32,
    #[doc = "Ingress port filter table capability register"]
    pub IPFTCAPR: u32,
    #[doc = "Ingress port filter table memory operational register"]
    pub IPFTMOR: u32,
    _reserved15: [u8; 0x01b4],
    #[doc = "Index table memory capability register"]
    pub ITMCAPR: u32,
    _reserved16: [u8; 0x0c],
    #[doc = "Rate policer capability register"]
    pub RPCAPR: u32,
    #[doc = "Rate policer index table capability register"]
    pub RPITCAPR: u32,
    #[doc = "Rate policer index table memory allocation register"]
    pub RPITMAR: u32,
    #[doc = "Rate policer index table operational register"]
    pub RPITOR: u32,
    _reserved17: [u8; 0x04],
    #[doc = "Ingress stream counter index table capability register"]
    pub ISCITCAPR: u32,
    #[doc = "Ingress stream counter index table memory allocation register"]
    pub ISCITMAR: u32,
    #[doc = "Ingress stream counter index table operational register"]
    pub ISCITOR: u32,
    #[doc = "Ingress stream capability register"]
    pub ISCAPR: u32,
    #[doc = "Ingress stream index table capability register"]
    pub ISITCAPR: u32,
    #[doc = "Ingress stream index table memory allocation register"]
    pub ISITMAR: u32,
    #[doc = "Ingress stream index table operational register"]
    pub ISITOR: u32,
    _reserved18: [u8; 0x04],
    #[doc = "Ingress sequence generation index table capability register"]
    pub ISQGITCAPR: u32,
    #[doc = "Ingress sequence generation index table memory allocation register"]
    pub ISQGITMAR: u32,
    #[doc = "Ingress sequence generation index table operational register"]
    pub ISQGITOR: u32,
    _reserved19: [u8; 0x10],
    #[doc = "Stream gate capability register"]
    pub SGCAPR: u32,
    #[doc = "Stream gate instance index table capability register"]
    pub SGIITCAPR: u32,
    #[doc = "Stream gate instance index table memory allocation register"]
    pub SGIITMAR: u32,
    #[doc = "Stream gate instance index table operational register"]
    pub SGIITOR: u32,
    _reserved20: [u8; 0x04],
    #[doc = "Stream gate control list index table capability register"]
    pub SGCLITCAPR: u32,
    #[doc = "Stream gate control list index table memory allocation register"]
    pub SGCLITMAR: u32,
    #[doc = "Stream gate control list table memory operational register"]
    pub SGCLTMOR: u32,
    #[doc = "Frame modification ingress capability register"]
    pub FMICAPR: u32,
    #[doc = "Frame modification egress capability register"]
    pub FMECAPR: u32,
    #[doc = "Frame modification index table capability register"]
    pub FMITCAPR: u32,
    #[doc = "Frame modification index table memory allocation register"]
    pub FMITMAR: u32,
    #[doc = "Frame modification index table operational register"]
    pub FMITOR: u32,
    #[doc = "Frame modification data index table capability register"]
    pub FMDITCAPR: u32,
    #[doc = "Frame modification data index table memory allocation register"]
    pub FMDITMAR: u32,
    _reserved21: [u8; 0x24],
    #[doc = "Egress treatment capability register"]
    pub ETCAPR: u32,
    #[doc = "Egress treatment table capability register"]
    pub ETTCAPR: u32,
    _reserved22: [u8; 0x04],
    #[doc = "Egress treatment table operational register"]
    pub ETTOR: u32,
    _reserved23: [u8; 0x04],
    #[doc = "Time gate scheduling table capability register"]
    pub TGSTCAPR: u32,
    _reserved24: [u8; 0x04],
    #[doc = "Time gate scheduling table memory operation register"]
    pub TGSTMOR: u32,
    #[doc = "Egress sequence recovery capability register"]
    pub ESQRCAPR: u32,
    #[doc = "Egress sequence recovery table capability register"]
    pub ESQRTCAPR: u32,
    _reserved25: [u8; 0x04],
    #[doc = "Egress counter table capability register"]
    pub ECTCAPR: u32,
    _reserved26: [u8; 0x10],
    #[doc = "Hash table memory capability register"]
    pub HTMCAPR: u32,
    #[doc = "Hash table memory operational register"]
    pub HTMOR: u32,
    _reserved27: [u8; 0x08],
    #[doc = "Ingress stream identification capability register"]
    pub ISIDCAPR: u32,
    #[doc = "Ingress stream identification hash table capability register"]
    pub ISIDHTCAPR: u32,
    _reserved28: [u8; 0x08],
    #[doc = "Ingress stream identification key construction 0 operational register"]
    pub ISIDKC0OR: u32,
    #[doc = "Ingress stream identification key construction 0 configuration register 0"]
    pub ISIDKC0CR0: u32,
    _reserved29: [u8; 0x08],
    #[doc = "Ingress stream identification key construction 0 payload field 0 configuration register"]
    pub ISIDKC0PF0CR: u32,
    #[doc = "Ingress stream identification key construction 0 payload field 1 configuration register"]
    pub ISIDKC0PF1CR: u32,
    #[doc = "Ingress stream identification key construction 0 payload field 2 configuration register"]
    pub ISIDKC0PF2CR: u32,
    #[doc = "Ingress stream identification key construction 0 payload field 3 configuration register"]
    pub ISIDKC0PF3CR: u32,
    #[doc = "Ingress stream identification key construction 1 operational register"]
    pub ISIDKC1OR: u32,
    #[doc = "Ingress stream identification key construction 1 configuration register 0"]
    pub ISIDKC1CR0: u32,
    _reserved30: [u8; 0x08],
    #[doc = "Ingress stream identification key construction 1 payload field 0 configuration register"]
    pub ISIDKC1PF0CR: u32,
    #[doc = "Ingress stream identification key construction 1 payload field 1 configuration register"]
    pub ISIDKC1PF1CR: u32,
    #[doc = "Ingress stream identification key construction 1 payload field 2 configuration register"]
    pub ISIDKC1PF2CR: u32,
    #[doc = "Ingress stream identification key construction 1 payload field 3 configuration register"]
    pub ISIDKC1PF3CR: u32,
    #[doc = "Ingress stream identification key construction 2 operational register"]
    pub ISIDKC2OR: u32,
    #[doc = "Ingress stream identification key construction 2 configuration register 0"]
    pub ISIDKC2CR0: u32,
    _reserved31: [u8; 0x08],
    #[doc = "Ingress stream identification key construction 2 payload field 0 configuration register"]
    pub ISIDKC2PF0CR: u32,
    #[doc = "Ingress stream identification key construction 2 payload field 1 configuration register"]
    pub ISIDKC2PF1CR: u32,
    #[doc = "Ingress stream identification key construction 2 payload field 2 configuration register"]
    pub ISIDKC2PF2CR: u32,
    #[doc = "Ingress stream identification key construction 2 payload field 3 configuration register"]
    pub ISIDKC2PF3CR: u32,
    #[doc = "Ingress stream identification key construction 3 operational register"]
    pub ISIDKC3OR: u32,
    #[doc = "Ingress stream identification key construction 3 configuration register 0"]
    pub ISIDKC3CR0: u32,
    _reserved32: [u8; 0x08],
    #[doc = "Ingress stream identification key construction 3 payload field 0 configuration register"]
    pub ISIDKC3PF0CR: u32,
    #[doc = "Ingress stream identification key construction 3 payload field 1 configuration register"]
    pub ISIDKC3PF1CR: u32,
    #[doc = "Ingress stream identification key construction 3 payload field 2 configuration register"]
    pub ISIDKC3PF2CR: u32,
    #[doc = "Ingress stream identification key construction 3 payload field 3 configuration register"]
    pub ISIDKC3PF3CR: u32,
    _reserved33: [u8; 0x60],
    #[doc = "Ingress stream filter hash table capability register"]
    pub ISFHTCAPR: u32,
    #[doc = "Ingress stream filter hash table operational register"]
    pub ISFHTOR: u32,
}
#[doc = "Ingress port capability register"]
pub mod IPCAPR {
    pub use crate::RO as access;
    #[doc = "Rate Policer function supported. 0: Not supported 1: Supported See RPCAPR for more information."]
    pub mod RP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ingress Port Filtering supported (that is,: Ingress Port Filter table lookup)"]
    pub mod IPFLT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ingress Stream Identification functionality supported"]
    pub mod ISID {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates support for various PDU/SDUs (Protocol/Service Data Unit) definitions."]
    pub mod SDU {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Specifies the number a of receive VLAN PCP/DE to QoS mapping profiles supported; see registers VLANIPVMPaR0/1 and VLANDRMPaR, where a={0"]
    pub mod NUM_VQMP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Egress port capability register"]
pub mod EPCAPR {
    pub use crate::RO as access;
    #[doc = "Egress Treatment function supported. 0: Not supported 1: Supported See ETCAPR for more information."]
    pub mod ET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates support for various PDU/SDUs (Protocol/Service Data Unit) definitions."]
    pub mod SDU {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Specifies the number of transmit QoS to VLAN PCP mapping profiles supported; see register QOSVLANMPaR0/1/2/3 where a={0"]
    pub mod NUM_QVMP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Operational state register"]
pub mod OSR {
    pub use crate::RO as access;
    #[doc = "Indicates the function's operational state 0: Function is operationally ready"]
    pub mod STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates the index table memory (common memory) operational state"]
    pub mod ITM_STATE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Correctable memory error configuration register"]
pub mod CMECR {
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
#[doc = "Correctable memory error status register"]
pub mod CMESR {
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
#[doc = "Correctable memory error count register"]
pub mod CMECTR {
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
#[doc = "Uncorrectable non-fatal MAC error configuration register"]
pub mod UNMACECR {
    pub use crate::RW as access;
    #[doc = "Report disable port"]
    pub mod PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Report disable port"]
    pub mod PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Report disable port"]
    pub mod PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Report disable port"]
    pub mod PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Report disable port"]
    pub mod PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Uncorrectable non-fatal MAC error status register"]
pub mod UNMACESR {
    pub use crate::RO as access;
    #[doc = "Port 0 MAC error"]
    pub mod PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Port 1 MAC error"]
    pub mod PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Port 2 MAC error"]
    pub mod PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Port 3 MAC error"]
    pub mod PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Port 4 MAC error"]
    pub mod PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Uncorrectable non-fatal system bus error configuration register"]
pub mod UNSBECR {
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
#[doc = "Uncorrectable non-fatal system bus error status register"]
pub mod UNSBESR {
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
#[doc = "Uncorrectable non-fatal system bus error count register"]
pub mod UNSBECTR {
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
#[doc = "Uncorrectable fatal system bus error configuration register"]
pub mod UFSBECR {
    pub use crate::RW as access;
    #[doc = "Report disable"]
    pub mod RD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Uncorrectable fatal system bus error status register"]
pub mod UFSBESR {
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
#[doc = "Uncorrectable non-fatal memory error configuration register"]
pub mod UNMECR {
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
#[doc = "Uncorrectable non-fatal memory error status register 0"]
pub mod UNMESR0 {
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
#[doc = "Uncorrectable non-fatal memory error status register 1"]
pub mod UNMESR1 {
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
#[doc = "Uncorrectable non-fatal memory error count register"]
pub mod UNMECTR {
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
#[doc = "Uncorrectable fatal memory error configuration register"]
pub mod UFMECR {
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
#[doc = "Uncorrectable fatal memory error status register 0"]
pub mod UFMESR0 {
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
#[doc = "Uncorrectable fatal memory error status register 1"]
pub mod UFMESR1 {
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
#[doc = "Internal MDIO interrupt reason register"]
pub mod IMDIOIRR {
    pub use crate::RO as access;
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    pub mod PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    pub mod PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    pub mod PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    pub mod PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    pub mod PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Internal MDIO MSI-X vector register"]
pub mod IMDIOMSIVR {
    pub use crate::RW as access;
    #[doc = "Index into MSI-X address/data table"]
    pub mod VECTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "External MDIO interrupt reason register"]
pub mod EMDIOIRR {
    pub use crate::RO as access;
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    pub mod PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    pub mod PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    pub mod PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    pub mod PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    pub mod PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "External MDIO MSI-X vector register"]
pub mod EMDIOMSIVR {
    pub use crate::RW as access;
    #[doc = "Index into MSI-X address/data table"]
    pub mod VECTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Time capture configuration register"]
pub mod TCCR {
    pub use crate::RW as access;
    #[doc = "Indicates the duration time in nanoseconds the timestamp capture function is armed"]
    pub mod TIMEOUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates if Timestamp Capture function is armed"]
    pub mod ARM {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Time capture interrupt enable register"]
pub mod TCIER {
    pub use crate::RW as access;
    #[doc = "Transmit interrupt"]
    pub mod TRANSMIT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timeout interrupt"]
    pub mod TIMEOUT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Time capture receive port interrupt detect register"]
pub mod TCRPIDR {
    pub use crate::RW as access;
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    pub mod TX_PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    pub mod TX_PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    pub mod TX_PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    pub mod TX_PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    pub mod TX_PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit interrupt"]
    pub mod TRANSMIT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timeout interrupt"]
    pub mod TIMEOUT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Time capture receive port status register"]
pub mod TCRPSR {
    pub use crate::RO as access;
    #[doc = "Receive port which captured the receive timestamp stored in TCRPTSR"]
    pub mod RX_PORT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Count the number of times the time capture function was triggered since it was ARMed"]
    pub mod RX_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Time capture receive port timestamp register"]
pub mod TCRPTSR {
    pub use crate::RO as access;
    #[doc = "Timestamp value in ns relative to SFD of the received frame"]
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
#[doc = "Time capture MSI-X vector register"]
pub mod TCMSIVR {
    pub use crate::RW as access;
    #[doc = "Index into MSI-X address/data table"]
    pub mod VECTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Custom VLAN Ethertype register 1"]
pub mod CVLANR1 {
    pub use crate::RW as access;
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
    #[doc = "Valid"]
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
#[doc = "Custom VLAN Ethertype register 2"]
pub mod CVLANR2 {
    pub use crate::RW as access;
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
    #[doc = "Valid"]
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
#[doc = "Pre-Standard RTAG Ethertype register"]
pub mod PSRTAGETR {
    pub use crate::RW as access;
    #[doc = "802"]
    pub mod ETHERTYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "DoS L2 configuration register"]
pub mod DOSL2CR {
    pub use crate::RW as access;
    #[doc = "This field specifies whether received frames with SMAC = DMAC are discarded"]
    pub mod SAMEADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "This field specifies whether received frames with Multicast SMAC address are discarded"]
    pub mod MSAMCC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress port filter capability register"]
pub mod IPFCAPR {
    pub use crate::RO as access;
    #[doc = "Rate Policer function supported"]
    pub mod RP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ingress Stream Identification supported."]
    pub mod ISID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress port filter table capability register"]
pub mod IPFTCAPR {
    pub use crate::RO as access;
    #[doc = "Number of ternary memory words supported"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates if table entries are managed by software driver or by hardware"]
    pub mod MGMT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates which Configuration Access Methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Maximum number of consecutive words which can form a TM Entry"]
    pub mod ENTRY_MAX_WORDS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Word size in bits of the ternary memory. 0: 48 bits 1-3: Reserved"]
    pub mod WORD_SIZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress port filter table memory operational register"]
pub mod IPFTMOR {
    pub use crate::RO as access;
    #[doc = "Number of words in-use."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Index table memory capability register"]
pub mod ITMCAPR {
    pub use crate::RO as access;
    #[doc = "Number of Words in the Index table memory"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Word size in bytes. 0: 24 bytes 1-3: Reserved"]
    pub mod WORD_SIZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1: Reserved"]
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
#[doc = "Rate policer capability register"]
pub mod RPCAPR {
    pub use crate::RO as access;
    #[doc = "Two-Rate Three-Color Marker supported per MEF 10.3 standard."]
    pub mod TRTCM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Color Mode capability 0: Support Color Blind mode only 1: Support Color Blind and Color Aware modes"]
    pub mod CM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rate policer index table capability register"]
pub mod RPITCAPR {
    pub use crate::RO as access;
    #[doc = "The number of entries assigned to this table. Reset value is specified by ROUNDDOWN(RPITMAR/4)."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rate policer index table memory allocation register"]
pub mod RPITMAR {
    pub use crate::RW as access;
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Rate policer index table operational register"]
pub mod RPITOR {
    pub use crate::RO as access;
    #[doc = "The number of entries in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream counter index table capability register"]
pub mod ISCITCAPR {
    pub use crate::RO as access;
    #[doc = "The number of entries assigned to this table. The reset value of this field comes from ISCITMAR."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream counter index table memory allocation register"]
pub mod ISCITMAR {
    pub use crate::RW as access;
    #[doc = "The number of words from index table memory assigned to this table. Each Entry consist of 1 word."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream counter index table operational register"]
pub mod ISCITOR {
    pub use crate::RO as access;
    #[doc = "The number of entries allocated / in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream capability register"]
pub mod ISCAPR {
    pub use crate::RO as access;
    #[doc = "Ingress Sequence Generation specification supported"]
    pub mod ISQG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Stream Gating specification is supported"]
    pub mod SG {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Rate Policer function specification supported"]
    pub mod RP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Maximum SDU check supported"]
    pub mod MAXSDU {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "When set, can specify a set of destination to forward the frame."]
    pub mod FWD {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Egress Treatment table entries specification supported. 0: Not supported 1: Supported"]
    pub mod ET {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream index table capability register"]
pub mod ISITCAPR {
    pub use crate::RO as access;
    #[doc = "The number of entries assigned to this table. The reset value of this field comes from ISITMAR."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream index table memory allocation register"]
pub mod ISITMAR {
    pub use crate::RW as access;
    #[doc = "The number of words from index table memory assigned to this table. Each Entry consist of 1 word."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream index table operational register"]
pub mod ISITOR {
    pub use crate::RO as access;
    #[doc = "The number of entries in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress sequence generation index table capability register"]
pub mod ISQGITCAPR {
    pub use crate::RO as access;
    #[doc = "The number of entries assigned to this table"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress sequence generation index table memory allocation register"]
pub mod ISQGITMAR {
    pub use crate::RW as access;
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress sequence generation index table operational register"]
pub mod ISQGITOR {
    pub use crate::RO as access;
    #[doc = "The number of entries allocated / in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Stream gate capability register"]
pub mod SGCAPR {
    pub use crate::RO as access;
    #[doc = "Support Administrative and Operational Gate Control List. 0: Not supported 1: Supported"]
    pub mod GLC_AO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Support configurable option indicating if GCL's Gate Check is from SFD only or SFD until EOF"]
    pub mod GLC_GC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Each Gate Control List Entry supports Interval Max Octet check. 0: Not supported 1: Supported"]
    pub mod GLC_IO {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Each Gate Control List Entry supports configurable IPV. 0: Not supported 1: Supported"]
    pub mod GLC_IPV {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Each Gate Control List Entry supports configurable CTD (Cut-Through Disable state)"]
    pub mod GLC_CTD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Stream gate instance index table capability register"]
pub mod SGIITCAPR {
    pub use crate::RO as access;
    #[doc = "The number of entries assigned to this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Stream gate instance index table memory allocation register"]
pub mod SGIITMAR {
    pub use crate::RW as access;
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Stream gate instance index table operational register"]
pub mod SGIITOR {
    pub use crate::RO as access;
    #[doc = "The number of entries allocated / in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Stream gate control list index table capability register"]
pub mod SGCLITCAPR {
    pub use crate::RO as access;
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Stream gate control list index table memory allocation register"]
pub mod SGCLITMAR {
    pub use crate::RW as access;
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Stream gate control list table memory operational register"]
pub mod SGCLTMOR {
    pub use crate::RO as access;
    #[doc = "Number of words in-use."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frame modification ingress capability register"]
pub mod FMICAPR {
    pub use crate::RO as access;
    #[doc = "Layer 2 frame modification actions supported"]
    pub mod L2_ACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frame modification egress capability register"]
pub mod FMECAPR {
    pub use crate::RO as access;
    #[doc = "Layer 2 frame modification actions"]
    pub mod L2_ACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Layer 3 frame modification actions"]
    pub mod L3_ACT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frame modification index table capability register"]
pub mod FMITCAPR {
    pub use crate::RO as access;
    #[doc = "The number of entries assigned to this table. The reset value is taken from FMITMAR\\[NUM_WORDS\\]."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frame modification index table memory allocation register"]
pub mod FMITMAR {
    pub use crate::RW as access;
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frame modification index table operational register"]
pub mod FMITOR {
    pub use crate::RO as access;
    #[doc = "The number of entries in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frame modification data index table capability register"]
pub mod FMDITCAPR {
    pub use crate::RO as access;
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Frame modification data index table memory allocation register"]
pub mod FMDITMAR {
    pub use crate::RW as access;
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Egress treatment capability register"]
pub mod ETCAPR {
    pub use crate::RO as access;
    #[doc = "Egress Sequence Recovery supported"]
    pub mod ESQR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Egress treatment table capability register"]
pub mod ETTCAPR {
    pub use crate::RO as access;
    #[doc = "The number of entries assigned to this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Egress treatment table operational register"]
pub mod ETTOR {
    pub use crate::RO as access;
    #[doc = "Number of entries allocated / in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Time gate scheduling table capability register"]
pub mod TGSTCAPR {
    pub use crate::RO as access;
    #[doc = "Number of Words"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Access Method"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Index"]
            pub const INDEX: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Maximum Gate Control List Length"]
    pub mod MAX_GCL_LEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "64"]
            pub const LEN_64: u32 = 0;
            #[doc = "128"]
            pub const LEN_128: u32 = 0x01;
            #[doc = "256"]
            pub const LEN_256: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Time gate scheduling table memory operation register"]
pub mod TGSTMOR {
    pub use crate::RO as access;
    #[doc = "The number of words in-use."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Egress sequence recovery capability register"]
pub mod ESQRCAPR {
    pub use crate::RO as access;
    #[doc = "Support Individual Recovery function and/or Sequence Recovery function x1: Individual Recovery function supported 1x: Sequence Recovery function supported"]
    pub mod SQR_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sequence Recovery Supported Algorithm. x1: Match Sequence Algorithm 1x: Vector Sequence Algorithm"]
    pub mod SQR_ALG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Maximum history length capability used by sequence recovery function. 0: 64 1: 128 2-7: Reserved"]
    pub mod SQR_MAX_HL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Egress sequence recovery table capability register"]
pub mod ESQRTCAPR {
    pub use crate::RO as access;
    #[doc = "The number of entries assigned to this table"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Egress counter table capability register"]
pub mod ECTCAPR {
    pub use crate::RO as access;
    #[doc = "The number of entries assigned to this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Hash table memory capability register"]
pub mod HTMCAPR {
    pub use crate::RO as access;
    #[doc = "Maximum number of words allotted to exact match hash table from the common memory's shared region"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Word size in bytes. 0: 24 bytes 1-3: Reserved"]
    pub mod WORD_SIZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1: Reserved"]
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
#[doc = "Hash table memory operational register"]
pub mod HTMOR {
    pub use crate::RO as access;
    #[doc = "Number of Words in use by this function which has been allocated by the various hash tables."]
    pub mod AMOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "High WaterMark of Words allocated. Value reset to AMOUNT when read."]
    pub mod WATERMARK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification capability register"]
pub mod ISIDCAPR {
    pub use crate::RO as access;
    #[doc = "Number of Exact Match Key Construction Instances supported for Ingress Stream Identification"]
    pub mod NUM_KC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Number of configurable Payload Fields supported"]
    pub mod NUM_PF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Maximum Key Size in bytes which can be constructed using the frame's fields."]
    pub mod MAX_KSIZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Unknown Frame Type (no header field parsing of the frame is necessary to construct the key) supported"]
    pub mod UFT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ethernet Frame Type (frame begins with standard 802"]
    pub mod ETHFT {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification hash table capability register"]
pub mod ISIDHTCAPR {
    pub use crate::RO as access;
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 0 operational register"]
pub mod ISIDKC0OR {
    pub use crate::RO as access;
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
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
#[doc = "Ingress stream identification key construction 0 configuration register 0"]
pub mod ISIDKC0CR0 {
    pub use crate::RW as access;
    #[doc = "Valid"]
    pub mod VALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The entire key construction rule is not valid including any configuration payload key fields defined."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "The key construction rule is valid."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    pub mod PORTP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    pub mod SPMP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Destination MAC (address) Present"]
    pub mod DMACP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_INCLUDED: u32 = 0;
            #[doc = "Present"]
            pub const INCLUDED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Source MAC (address) Present."]
    pub mod SMACP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Outer VID Present"]
    pub mod OVIDP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Outer PCP Present"]
    pub mod OPCPP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Outer VLAN header's PCP field not present in the key"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Outer VLAN header's PCP field present in the key"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Inner VID Present."]
    pub mod IVIDP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Inner PCP Present."]
    pub mod IPCPP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sequence Tag (code point) Present."]
    pub mod SQTP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EtherType Present."]
    pub mod ETP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 0 payload field 0 configuration register"]
pub mod ISIDKC0PF0CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 0 payload field 1 configuration register"]
pub mod ISIDKC0PF1CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 0 payload field 2 configuration register"]
pub mod ISIDKC0PF2CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 0 payload field 3 configuration register"]
pub mod ISIDKC0PF3CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 1 operational register"]
pub mod ISIDKC1OR {
    pub use crate::RO as access;
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
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
#[doc = "Ingress stream identification key construction 1 configuration register 0"]
pub mod ISIDKC1CR0 {
    pub use crate::RW as access;
    #[doc = "Valid"]
    pub mod VALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The entire key construction rule is not valid including any configuration payload key fields defined."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "The key construction rule is valid."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    pub mod PORTP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    pub mod SPMP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Destination MAC (address) Present"]
    pub mod DMACP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_INCLUDED: u32 = 0;
            #[doc = "Present"]
            pub const INCLUDED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Source MAC (address) Present."]
    pub mod SMACP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Outer VID Present"]
    pub mod OVIDP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Outer PCP Present"]
    pub mod OPCPP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Outer VLAN header's PCP field not present in the key"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Outer VLAN header's PCP field present in the key"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Inner VID Present."]
    pub mod IVIDP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Inner PCP Present."]
    pub mod IPCPP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sequence Tag (code point) Present."]
    pub mod SQTP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EtherType Present."]
    pub mod ETP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 1 payload field 0 configuration register"]
pub mod ISIDKC1PF0CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 1 payload field 1 configuration register"]
pub mod ISIDKC1PF1CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 1 payload field 2 configuration register"]
pub mod ISIDKC1PF2CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 1 payload field 3 configuration register"]
pub mod ISIDKC1PF3CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 2 operational register"]
pub mod ISIDKC2OR {
    pub use crate::RO as access;
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
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
#[doc = "Ingress stream identification key construction 2 configuration register 0"]
pub mod ISIDKC2CR0 {
    pub use crate::RW as access;
    #[doc = "Valid"]
    pub mod VALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The entire key construction rule is not valid including any configuration payload key fields defined."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "The key construction rule is valid."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    pub mod PORTP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    pub mod SPMP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Destination MAC (address) Present"]
    pub mod DMACP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_INCLUDED: u32 = 0;
            #[doc = "Present"]
            pub const INCLUDED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Source MAC (address) Present."]
    pub mod SMACP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Outer VID Present"]
    pub mod OVIDP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Outer PCP Present"]
    pub mod OPCPP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Outer VLAN header's PCP field not present in the key"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Outer VLAN header's PCP field present in the key"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Inner VID Present."]
    pub mod IVIDP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Inner PCP Present."]
    pub mod IPCPP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sequence Tag (code point) Present."]
    pub mod SQTP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EtherType Present."]
    pub mod ETP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 2 payload field 0 configuration register"]
pub mod ISIDKC2PF0CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 2 payload field 1 configuration register"]
pub mod ISIDKC2PF1CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 2 payload field 2 configuration register"]
pub mod ISIDKC2PF2CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 2 payload field 3 configuration register"]
pub mod ISIDKC2PF3CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 3 operational register"]
pub mod ISIDKC3OR {
    pub use crate::RO as access;
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
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
#[doc = "Ingress stream identification key construction 3 configuration register 0"]
pub mod ISIDKC3CR0 {
    pub use crate::RW as access;
    #[doc = "Valid"]
    pub mod VALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "The entire key construction rule is not valid including any configuration payload key fields defined."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "The key construction rule is valid."]
            pub const VALID: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    pub mod PORTP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    pub mod SPMP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Destination MAC (address) Present"]
    pub mod DMACP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_INCLUDED: u32 = 0;
            #[doc = "Present"]
            pub const INCLUDED: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Source MAC (address) Present."]
    pub mod SMACP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Outer VID Present"]
    pub mod OVIDP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Outer PCP Present"]
    pub mod OPCPP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Outer VLAN header's PCP field not present in the key"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Outer VLAN header's PCP field present in the key"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Inner VID Present."]
    pub mod IVIDP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Inner PCP Present."]
    pub mod IPCPP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Sequence Tag (code point) Present."]
    pub mod SQTP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "EtherType Present."]
    pub mod ETP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 3 payload field 0 configuration register"]
pub mod ISIDKC3PF0CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 3 payload field 1 configuration register"]
pub mod ISIDKC3PF1CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 3 payload field 2 configuration register"]
pub mod ISIDKC3PF2CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream identification key construction 3 payload field 3 configuration register"]
pub mod ISIDKC3PF3CR {
    pub use crate::RW as access;
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream filter hash table capability register"]
pub mod ISFHTCAPR {
    pub use crate::RO as access;
    #[doc = "Indicates which configuration access methods are supported: xxx1: Index xx1x: EntryId x1xx: Search 1xxx: Reserved"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Ingress stream filter hash table operational register"]
pub mod ISFHTOR {
    pub use crate::RO as access;
    #[doc = "Number of entries in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
pub mod NUMPROFILE {
    #[doc = "VLAN to IPV mapping profile register set."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "VLAN to IPV mapping profile v register 0"]
        pub VLANIPVMPR0: u32,
        #[doc = "VLAN to IPV mapping profile v register 1"]
        pub VLANIPVMPR1: u32,
        #[doc = "VLAN to DR mapping profile v register"]
        pub VLANDRMPR: u32,
        _reserved0: [u8; 0x04],
    }
    #[doc = "VLAN to IPV mapping profile v register 0"]
    pub mod VLANIPVMPR0 {
        pub use crate::RW as access;
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_1 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_2 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_3 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_4 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_5 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_6 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_7 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "VLAN to IPV mapping profile v register 1"]
    pub mod VLANIPVMPR1 {
        pub use crate::RW as access;
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_8 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_9 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_10 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_11 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_12 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_13 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_14 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_15 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x07 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
    #[doc = "VLAN to DR mapping profile v register"]
    pub mod VLANDRMPR {
        pub use crate::RW as access;
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_1 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_2 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_3 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_4 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_5 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_6 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_7 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_8 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_9 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_10 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_11 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_12 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_13 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_14 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_15 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0x03 << offset;
            pub use super::access;
            #[doc(hidden)]
            pub mod vals {}
            #[doc(inline)]
            pub use vals::*;
        }
    }
}
