#[doc = "Ethernet MAC port"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "Port MAC 0 Command and Configuration Register"]
    pub PM0_COMMAND_CONFIG: u32,
    #[doc = "Port MAC 0 MAC Address Register 0"]
    pub PM0_MAC_ADDR_0: u32,
    #[doc = "Port MAC 0 MAC Address Register 1"]
    pub PM0_MAC_ADDR_1: u32,
    #[doc = "Port MAC 0 Maximum Frame Length Register"]
    pub PM0_MAXFRM: u32,
    #[doc = "Port MAC 0 Minimum Frame Length Register"]
    pub PM0_MINFRM: u32,
    _reserved1: [u8; 0x24],
    #[doc = "Port MAC 0 Interrupt Event Register"]
    pub PM0_IEVENT: u32,
    #[doc = "Port MAC 0 Transmit Inter-Packet Gap Length and Flexible Preamble length Register"]
    pub PM0_TX_IPG_PREAMBLE: u32,
    _reserved2: [u8; 0x04],
    #[doc = "Port MAC 0 Interrupt Mask Register(INT_MASK)"]
    pub PM0_IMASK: u32,
    _reserved3: [u8; 0x04],
    #[doc = "Port MAC 0 Pause Quanta Register"]
    pub PM0_PAUSE_QUANTA: u32,
    _reserved4: [u8; 0x0c],
    #[doc = "Port MAC 0 Pause Quanta Threshold Register"]
    pub PM0_PAUSE_THRESH: u32,
    _reserved5: [u8; 0x0c],
    #[doc = "Port MAC 0 Receive Pause Status Register"]
    pub PM0_RX_PAUSE_STATUS: u32,
    _reserved6: [u8; 0x40],
    #[doc = "Port MAC 0 EEE Low Power Wakeup Timer Register"]
    pub PM0_LPWAKE_TIMER: u32,
    #[doc = "Port MAC 0 Transmit EEE Low Power Timer Register"]
    pub PM0_SLEEP_TIMER: u32,
    #[doc = "Port MAC 0 IEEE1588 Single-Step Control Register"]
    pub PM0_SINGLE_STEP: u32,
    _reserved7: [u8; 0x0c],
    #[doc = "Port MAC 0 half-duplex backoff entropy register"]
    pub PM0_HD_BACKOFF_ENTROPY: u32,
    #[doc = "Port MAC 0 Half-Duplex Flow Control Register"]
    pub PM0_HD_FLOW_CTRL: u32,
    _reserved8: [u8; 0x08],
    #[doc = "Port MAC 0 Statistics Configuration Register"]
    pub PM0_STATN_CONFIG: u32,
    _reserved9: [u8; 0x1c],
    #[doc = "Port MAC 0 Receive Ethernet Octets Counter(etherStatsOctetsn)"]
    pub PM0_REOCTN: u64,
    #[doc = "Port MAC 0 Receive Octets Counter(iflnOctetsn)"]
    pub PM0_ROCTN: u64,
    _reserved10: [u8; 0x08],
    #[doc = "Port MAC 0 Receive Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
    pub PM0_RXPFN: u64,
    #[doc = "Port MAC 0 Receive Frame Counter Register(aFramesReceivedOKn)"]
    pub PM0_RFRMN: u64,
    #[doc = "Port MAC 0 Receive Frame Check Sequence Error Counter Register()"]
    pub PM0_RFCSN: u64,
    #[doc = "Port MAC 0 Receive VLAN Frame Counter Register(VLANReceivedOKn)"]
    pub PM0_RVLANN: u64,
    #[doc = "Port MAC 0 Receive Frame Error Counter Register(ifInErrorsn)"]
    pub PM0_RERRN: u64,
    #[doc = "Port MAC 0 Receive Unicast Frame Counter Register(ifInUcastPktsn)"]
    pub PM0_RUCAN: u64,
    #[doc = "Port MAC 0 Receive Multicast Frame Counter Register(ifInMulticastPktsn)"]
    pub PM0_RMCAN: u64,
    #[doc = "Port MAC 0 Receive Broadcast Frame Counter Register(ifInBroadcastPktsn)"]
    pub PM0_RBCAN: u64,
    #[doc = "Port MAC 0 Receive Dropped Packets Counter Register(etherStatsDropEventsn)"]
    pub PM0_RDRPN: u64,
    #[doc = "Port MAC 0 Receive Packets Counter Register(etherStatsPktsn)"]
    pub PM0_RPKTN: u64,
    #[doc = "Port MAC 0 Receive Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
    pub PM0_RUNDN: u64,
    #[doc = "Port MAC 0 Receive 64-Octet Packet Counter Register(etherStatsPkts64OctetsN)"]
    pub PM0_R64N: u64,
    #[doc = "Port MAC 0 Receive 65 to 127-Octet Packet Counter Register(etherStatsPkts65to127OctetsN)"]
    pub PM0_R127N: u64,
    #[doc = "Port MAC 0 Receive 128 to 255-Octet Packet Counter Register(etherStatsPkts128to255OctetsN)"]
    pub PM0_R255N: u64,
    #[doc = "Port MAC 0 Receive 256 to 511-Octet Packet Counter Register(etherStatsPkts256to511OctetsN)"]
    pub PM0_R511N: u64,
    #[doc = "Port MAC 0 Receive 512 to 1023-Octet Packet Counter Register(etherStatsPkts512to1023OctetsN)"]
    pub PM0_R1023N: u64,
    #[doc = "Port MAC 0 Receive 1024 to 1522-Octet Packet Counter Register(etherStatsPkts1024to1522OctetsN)"]
    pub PM0_R1522N: u64,
    #[doc = "Port MAC 0 Receive 1523 to Max-Octet Packet Counter Register(etherStatsPkts1523toMaxOctetsN)"]
    pub PM0_R1523XN: u64,
    #[doc = "Port MAC 0 Receive Oversized Packet Counter Register(etherStatsOversizePktsn)"]
    pub PM0_ROVRN: u64,
    #[doc = "Port MAC 0 Receive Jabber Packet Counter Register(etherStatsJabbersn)"]
    pub PM0_RJBRN: u64,
    #[doc = "Port MAC 0 Receive Fragment Packet Counter Register(etherStatsFragmentsn"]
    pub PM0_RFRGN: u64,
    #[doc = "Port MAC 0 Receive Control Packet Counter Register"]
    pub PM0_RCNPN: u64,
    #[doc = "Port MAC 0 Receive Dropped Not Truncated Packets Counter Register(etherStatsDropEventsn)"]
    pub PM0_RDRNTPN: u64,
    #[doc = "Port MAC 0 Receive Valid Small Packet Counter Register"]
    pub PM0_RMIN63N: u64,
    _reserved11: [u8; 0x28],
    #[doc = "Port MAC 0 Transmit Ethernet Octets Counter(etherStatsOctetsn)"]
    pub PM0_TEOCTN: u64,
    #[doc = "Port MAC 0 Transmit Octets Counter Register(ifOutOctetsn)"]
    pub PM0_TOCTN: u64,
    _reserved12: [u8; 0x08],
    #[doc = "Port MAC 0 Transmit Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
    pub PM0_TXPFN: u64,
    #[doc = "Port MAC 0 Transmit Frame Counter Register(aFramesTransmittedOKn)"]
    pub PM0_TFRMN: u64,
    #[doc = "Port MAC 0 Transmit Frame Check Sequence Error Counter Register()"]
    pub PM0_TFCSN: u64,
    #[doc = "Port MAC 0 Transmit VLAN Frame Counter Register(VLANTransmittedOKn)"]
    pub PM0_TVLANN: u64,
    #[doc = "Port MAC 0 Transmit Frame Error Counter Register(ifOutErrorsn)"]
    pub PM0_TERRN: u64,
    #[doc = "Port MAC 0 Transmit Unicast Frame Counter Register(ifOutUcastPktsn)"]
    pub PM0_TUCAN: u64,
    #[doc = "Port MAC 0 Transmit Multicast Frame Counter Register(ifOutMulticastPktsn)"]
    pub PM0_TMCAN: u64,
    #[doc = "Port MAC 0 Transmit Broadcast Frame Counter Register(ifOutBroadcastPktsn)"]
    pub PM0_TBCAN: u64,
    _reserved13: [u8; 0x08],
    #[doc = "Port MAC 0 Transmit Packets Counter Register(etherStatsPktsn)"]
    pub PM0_TPKTN: u64,
    #[doc = "Port MAC 0 Transmit Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
    pub PM0_TUNDN: u64,
    #[doc = "Port MAC 0 Transmit 64-Octet Packet Counter Register (etherStatsPkts64OctetsN)"]
    pub PM0_T64N: u64,
    #[doc = "Port MAC 0 Transmit 65 to 127-Octet Packet Counter Register (etherStatsPkts65to127OctetsN)"]
    pub PM0_T127N: u64,
    #[doc = "Port MAC 0 Transmit 128 to 255-Octet Packet Counter Register (etherStatsPkts128to255OctetsN)"]
    pub PM0_T255N: u64,
    #[doc = "Port MAC 0 Transmit 256 to 511-Octet Packet Counter Register (etherStatsPkts256to511OctetsN)"]
    pub PM0_T511N: u64,
    #[doc = "Port MAC 0 Transmit 512 to 1023-Octet Packet Counter Register (etherStatsPkts512to1023OctetsN)"]
    pub PM0_T1023N: u64,
    #[doc = "Port MAC 0 Transmit 1024 to 1522-Octet Packet Counter Register (etherStatsPkts1024to1522OctetsN)"]
    pub PM0_T1522N: u64,
    #[doc = "Port MAC 0 Transmit 1523 to TX_MTU-Octet Packet Counter Register (etherStatsPkts1523toMaxOctetsN)"]
    pub PM0_T1523XN: u64,
    _reserved14: [u8; 0x18],
    #[doc = "Port MAC 0 Transmit Control Packet Counter Register"]
    pub PM0_TCNPN: u64,
    _reserved15: [u8; 0x08],
    #[doc = "Port MAC 0 Transmit Deferred Packet Counter Register(aFramesWithDeferredXmissions)"]
    pub PM0_TDFRN: u64,
    #[doc = "Port MAC 0 Transmit Multiple Collisions Counter Register(aMultipleCollisionFrames)"]
    pub PM0_TMCOLN: u64,
    #[doc = "Port MAC 0 Transmit Single Collision Counter(aSingleCollisionFrames) Register"]
    pub PM0_TSCOLN: u64,
    #[doc = "Port MAC 0 Transmit Late Collision Counter(aLateCollisions) Register"]
    pub PM0_TLCOLN: u64,
    #[doc = "Port MAC 0 Transmit Excessive Collisions Counter Register"]
    pub PM0_TECOLN: u64,
    _reserved16: [u8; 0x08],
    #[doc = "Port MAC 0 Interface Mode Control Register"]
    pub PM0_IF_MODE: u32,
    _reserved17: [u8; 0x0104],
    #[doc = "Port MAC 1 Command and Configuration Register"]
    pub PM1_COMMAND_CONFIG: u32,
    #[doc = "Port MAC 1 MAC Address Register 0"]
    pub PM1_MAC_ADDR_0: u32,
    #[doc = "Port MAC 1 MAC Address Register 1"]
    pub PM1_MAC_ADDR_1: u32,
    #[doc = "Port MAC 1 Maximum Frame Length Register"]
    pub PM1_MAXFRM: u32,
    #[doc = "Port MAC 1 Minimum Frame Length Register"]
    pub PM1_MINFRM: u32,
    _reserved18: [u8; 0x24],
    #[doc = "Port MAC 1 Interrupt Event Register"]
    pub PM1_IEVENT: u32,
    #[doc = "Port MAC 1 Transmit Inter-Packet Gap Length and Flexible Preamble length Register"]
    pub PM1_TX_IPG_PREAMBLE: u32,
    _reserved19: [u8; 0x04],
    #[doc = "Port MAC 1 Interrupt Mask Register(INT_MASK)"]
    pub PM1_IMASK: u32,
    _reserved20: [u8; 0x04],
    #[doc = "Port MAC 1 Pause Quanta Register"]
    pub PM1_PAUSE_QUANTA: u32,
    _reserved21: [u8; 0x0c],
    #[doc = "Port MAC 1 Pause Quanta Threshold Register"]
    pub PM1_PAUSE_THRESH: u32,
    _reserved22: [u8; 0x0c],
    #[doc = "Port MAC 1 Receive Pause Status Register"]
    pub PM1_RX_PAUSE_STATUS: u32,
    _reserved23: [u8; 0x40],
    #[doc = "Port MAC 1 EEE Low Power Wakeup Timer Register"]
    pub PM1_LPWAKE_TIMER: u32,
    #[doc = "Port MAC 1 Transmit EEE Low Power Timer Register"]
    pub PM1_SLEEP_TIMER: u32,
    #[doc = "Port MAC 1 IEEE1588 Single-Step Control Register"]
    pub PM1_SINGLE_STEP: u32,
    _reserved24: [u8; 0x0c],
    #[doc = "Port MAC 1 half-duplex backoff entropy register"]
    pub PM1_HD_BACKOFF_ENTROPY: u32,
    #[doc = "Port MAC 1 Half-Duplex Flow Control Register"]
    pub PM1_HD_FLOW_CTRL: u32,
    _reserved25: [u8; 0x08],
    #[doc = "Port MAC 1 Statistics Configuration Register"]
    pub PM1_STATN_CONFIG: u32,
    _reserved26: [u8; 0x1c],
    #[doc = "Port MAC 1 Receive Ethernet Octets Counter(etherStatsOctetsn)"]
    pub PM1_REOCTN: u64,
    #[doc = "Port MAC 1 Receive Octets Counter(iflnOctetsn)"]
    pub PM1_ROCTN: u64,
    _reserved27: [u8; 0x08],
    #[doc = "Port MAC 1 Receive Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
    pub PM1_RXPFN: u64,
    #[doc = "Port MAC 1 Receive Frame Counter Register(aFramesReceivedOKn)"]
    pub PM1_RFRMN: u64,
    #[doc = "Port MAC 1 Receive Frame Check Sequence Error Counter Register()"]
    pub PM1_RFCSN: u64,
    #[doc = "Port MAC 1 Receive VLAN Frame Counter Register(VLANReceivedOKn)"]
    pub PM1_RVLANN: u64,
    #[doc = "Port MAC 1 Receive Frame Error Counter Register(ifInErrorsn)"]
    pub PM1_RERRN: u64,
    #[doc = "Port MAC 1 Receive Unicast Frame Counter Register(ifInUcastPktsn)"]
    pub PM1_RUCAN: u64,
    #[doc = "Port MAC 1 Receive Multicast Frame Counter Register(ifInMulticastPktsn)"]
    pub PM1_RMCAN: u64,
    #[doc = "Port MAC 1 Receive Broadcast Frame Counter Register(ifInBroadcastPktsn)"]
    pub PM1_RBCAN: u64,
    #[doc = "Port MAC 1 Receive Dropped Packets Counter Register(etherStatsDropEventsn)"]
    pub PM1_RDRPN: u64,
    #[doc = "Port MAC 1 Receive Packets Counter Register(etherStatsPktsn)"]
    pub PM1_RPKTN: u64,
    #[doc = "Port MAC 1 Receive Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
    pub PM1_RUNDN: u64,
    #[doc = "Port MAC 1 Receive 64-Octet Packet Counter Register(etherStatsPkts64OctetsN)"]
    pub PM1_R64N: u64,
    #[doc = "Port MAC 1 Receive 65 to 127-Octet Packet Counter Register(etherStatsPkts65to127OctetsN)"]
    pub PM1_R127N: u64,
    #[doc = "Port MAC 1 Receive 128 to 255-Octet Packet Counter Register(etherStatsPkts128to255OctetsN)"]
    pub PM1_R255N: u64,
    #[doc = "Port MAC 1 Receive 256 to 511-Octet Packet Counter Register(etherStatsPkts256to511OctetsN)"]
    pub PM1_R511N: u64,
    #[doc = "Port MAC 1 Receive 512 to 1023-Octet Packet Counter Register(etherStatsPkts512to1023OctetsN)"]
    pub PM1_R1023N: u64,
    #[doc = "Port MAC 1 Receive 1024 to 1522-Octet Packet Counter Register(etherStatsPkts1024to1522OctetsN)"]
    pub PM1_R1522N: u64,
    #[doc = "Port MAC 1 Receive 1523 to Max-Octet Packet Counter Register(etherStatsPkts1523toMaxOctetsN)"]
    pub PM1_R1523XN: u64,
    #[doc = "Port MAC 1 Receive Oversized Packet Counter Register(etherStatsOversizePktsn)"]
    pub PM1_ROVRN: u64,
    #[doc = "Port MAC 1 Receive Jabber Packet Counter Register(etherStatsJabbersn)"]
    pub PM1_RJBRN: u64,
    #[doc = "Port MAC 1 Receive Fragment Packet Counter Register(etherStatsFragmentsn"]
    pub PM1_RFRGN: u64,
    #[doc = "Port MAC 1 Receive Control Packet Counter Register"]
    pub PM1_RCNPN: u64,
    #[doc = "Port MAC 1 Receive Dropped Not Truncated Packets Counter Register(etherStatsDropEventsn)"]
    pub PM1_RDRNTPN: u64,
    #[doc = "Port MAC 1 Receive Valid Small Packet Counter Register"]
    pub PM1_RMIN63N: u64,
    _reserved28: [u8; 0x28],
    #[doc = "Port MAC 1 Transmit Ethernet Octets Counter(etherStatsOctetsn)"]
    pub PM1_TEOCTN: u64,
    #[doc = "Port MAC 1 Transmit Octets Counter Register(ifOutOctetsn)"]
    pub PM1_TOCTN: u64,
    _reserved29: [u8; 0x08],
    #[doc = "Port MAC 1 Transmit Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
    pub PM1_TXPFN: u64,
    #[doc = "Port MAC 1 Transmit Frame Counter Register(aFramesTransmittedOKn)"]
    pub PM1_TFRMN: u64,
    #[doc = "Port MAC 1 Transmit Frame Check Sequence Error Counter Register()"]
    pub PM1_TFCSN: u64,
    #[doc = "Port MAC 1 Transmit VLAN Frame Counter Register(VLANTransmittedOKn)"]
    pub PM1_TVLANN: u64,
    #[doc = "Port MAC 1 Transmit Frame Error Counter Register(ifOutErrorsn)"]
    pub PM1_TERRN: u64,
    #[doc = "Port MAC 1 Transmit Unicast Frame Counter Register(ifOutUcastPktsn)"]
    pub PM1_TUCAN: u64,
    #[doc = "Port MAC 1 Transmit Multicast Frame Counter Register(ifOutMulticastPktsn)"]
    pub PM1_TMCAN: u64,
    #[doc = "Port MAC 1 Transmit Broadcast Frame Counter Register(ifOutBroadcastPktsn)"]
    pub PM1_TBCAN: u64,
    _reserved30: [u8; 0x08],
    #[doc = "Port MAC 1 Transmit Packets Counter Register(etherStatsPktsn)"]
    pub PM1_TPKTN: u64,
    #[doc = "Port MAC 1 Transmit Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
    pub PM1_TUNDN: u64,
    #[doc = "Port MAC 1 Transmit 64-Octet Packet Counter Register (etherStatsPkts64OctetsN)"]
    pub PM1_T64N: u64,
    #[doc = "Port MAC 1 Transmit 65 to 127-Octet Packet Counter Register (etherStatsPkts65to127OctetsN)"]
    pub PM1_T127N: u64,
    #[doc = "Port MAC 1 Transmit 128 to 255-Octet Packet Counter Register (etherStatsPkts128to255OctetsN)"]
    pub PM1_T255N: u64,
    #[doc = "Port MAC 1 Transmit 256 to 511-Octet Packet Counter Register (etherStatsPkts256to511OctetsN)"]
    pub PM1_T511N: u64,
    #[doc = "Port MAC 1 Transmit 512 to 1023-Octet Packet Counter Register (etherStatsPkts512to1023OctetsN)"]
    pub PM1_T1023N: u64,
    #[doc = "Port MAC 1 Transmit 1024 to 1522-Octet Packet Counter Register (etherStatsPkts1024to1522OctetsN)"]
    pub PM1_T1522N: u64,
    #[doc = "Port MAC 1 Transmit 1523 to TX_MTU-Octet Packet Counter Register (etherStatsPkts1523toMaxOctetsN)"]
    pub PM1_T1523XN: u64,
    _reserved31: [u8; 0x18],
    #[doc = "Port MAC 1 Transmit Control Packet Counter Register"]
    pub PM1_TCNPN: u64,
    _reserved32: [u8; 0x08],
    #[doc = "Port MAC 1 Transmit Deferred Packet Counter Register(aFramesWithDeferredXmissions)"]
    pub PM1_TDFRN: u64,
    #[doc = "Port MAC 1 Transmit Multiple Collisions Counter Register(aMultipleCollisionFrames)"]
    pub PM1_TMCOLN: u64,
    #[doc = "Port MAC 1 Transmit Single Collision Counter(aSingleCollisionFrames) Register"]
    pub PM1_TSCOLN: u64,
    #[doc = "Port MAC 1 Transmit Late Collision Counter(aLateCollisions) Register"]
    pub PM1_TLCOLN: u64,
    #[doc = "Port MAC 1 Transmit Excessive Collisions Counter Register"]
    pub PM1_TECOLN: u64,
    _reserved33: [u8; 0x08],
    #[doc = "Port MAC 1 Interface Mode Control Register"]
    pub PM1_IF_MODE: u32,
    _reserved34: [u8; 0xfc],
    #[doc = "Port MAC Merge Control and Status Register"]
    pub MAC_MERGE_MMCSR: u32,
    _reserved35: [u8; 0x04],
    #[doc = "Port MAC Merge Frame Assembly Error Count Register"]
    pub MAC_MERGE_MMFAECR: u32,
    #[doc = "Port MAC Merge Frame SMD Error Count Register"]
    pub MAC_MERGE_MMFSECR: u32,
    #[doc = "Port MAC Merge Frame Assembly OK Count Register"]
    pub MAC_MERGE_MMFAOCR: u32,
    #[doc = "Port MAC Merge Fragment Count RX Register"]
    pub MAC_MERGE_MMFCRXR: u32,
    #[doc = "Port MAC Merge Fragment Count TX Register"]
    pub MAC_MERGE_MMFCTXR: u32,
    #[doc = "Port MAC Merge Hold Count Register"]
    pub MAC_MERGE_MMHCR: u32,
    _reserved36: [u8; 0x03e0],
    #[doc = "Port external MDIO configuration register"]
    pub PEMDIOCR: u32,
    #[doc = "Port external MDIO interface control register"]
    pub PEMDIOICR: u32,
    #[doc = "Port external MDIO interface data register"]
    pub PEMDIOIDR: u32,
    #[doc = "Port external MDIO register address register"]
    pub PEMDIORAR: u32,
    #[doc = "Port external MDIO status register"]
    pub PEMDIOSR: u32,
    _reserved37: [u8; 0x0c],
    #[doc = "PHY status configuration register"]
    pub PPSCR: u32,
    #[doc = "Port PHY status control register"]
    pub PPSCTRLR: u32,
    #[doc = "Port PHY status data register"]
    pub PPSDR: u32,
    #[doc = "Port PHY status register address register"]
    pub PPSRAR: u32,
    #[doc = "Port PHY status event register"]
    pub PPSER: u32,
    #[doc = "Port PHY status mask register"]
    pub PPSMR: u32,
}
#[doc = "Port MAC 0 Command and Configuration Register"]
pub mod PM0_COMMAND_CONFIG {
    pub use crate::RW as access;
    #[doc = "MAC transmit path enable"]
    pub mod TX_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MAC receive path enable"]
    pub mod RX_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Terminate/forward received PAUSE frames"]
    pub mod PAUSE_FWD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ignore PAUSE frame quanta"]
    pub mod PAUSE_IGN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit source MAC address insertion"]
    pub mod TX_ADDR_INS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loopback enable"]
    pub mod LOOP_ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loopback mode"]
    pub mod LPBK_MODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control frame reception enable"]
    pub mod CNT_FRM_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timestamp Point"]
    pub mod TS_PNT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable padding of frames in transmit direction (1, default)."]
    pub mod TXP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Half Duplex Flow Control Enable"]
    pub mod HD_FCEN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Tx flush"]
    pub mod TX_FLUSH {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit Low Power Idle Enable."]
    pub mod TX_LOWP_ENA {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "(default), the MAC operates in normal mode."]
            pub const DISABLE: u32 = 0;
            #[doc = "The MAC completes the transmission of the current Frame and generates Low Power Idle Sequences to the line. It is advised to inspect IEVENT\\[TX_EMPTY\\] is set before enabling the LPI."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software Reset. Self clearing bit."]
    pub mod SWR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ingress flush enable"]
    pub mod RX_FLUSH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit timestamp mode"]
    pub mod TS_MODE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Magic Packet detection enable."]
    pub mod MG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 MAC Address Register 0"]
pub mod PM0_MAC_ADDR_0 {
    pub use crate::RO as access;
    #[doc = "MAC address 0"]
    pub mod MAC_ADDR_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 MAC Address Register 1"]
pub mod PM0_MAC_ADDR_1 {
    pub use crate::RO as access;
    #[doc = "MAC address 1"]
    pub mod MAC_ADDR_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Maximum Frame Length Register"]
pub mod PM0_MAXFRM {
    pub use crate::RW as access;
    #[doc = "Maximum supported received frame length."]
    pub mod MAXFRM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Maximum transmit frame length"]
    pub mod TX_MTU {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Minimum Frame Length Register"]
pub mod PM0_MINFRM {
    pub use crate::RW as access;
    #[doc = "Receive Minimum Frame Length size in bytes."]
    pub mod NUM_BYTES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Interrupt Event Register"]
pub mod PM0_IEVENT {
    pub use crate::RW as access;
    #[doc = "Transmit fifo empty event"]
    pub mod TX_EMPTY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive idle event"]
    pub mod RX_EMPTY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit FIFO overflow event."]
    pub mod TX_OVFL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit FIFO underflow event."]
    pub mod TX_UNFL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive FIFO overflow event."]
    pub mod RX_OVFL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Magic packet detection indication event"]
    pub mod MGI {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Tx Clock Stop Detection"]
    pub mod TX_CSD {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Rx Clock Stop Detection"]
    pub mod RX_CSD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Speed/Duplex Change"]
    pub mod SPD_DUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MAC merge frame SMD error received event"]
    pub mod MRG_SERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MAC merge frame assembly error event"]
    pub mod MRG_AERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Inter-Packet Gap Length and Flexible Preamble length Register"]
pub mod PM0_TX_IPG_PREAMBLE {
    pub use crate::RW as access;
    #[doc = "Transmit inter-packet gap value."]
    pub mod IPG_LEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Flexible Preamble Count"]
    pub mod FLEX_PREAMBLE_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Flexible Preamble Count"]
    pub mod FLEX_PREAMBLE_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Interrupt Mask Register(INT_MASK)"]
pub mod PM0_IMASK {
    pub use crate::RW as access;
    #[doc = "Magic packet detection indication event mask."]
    pub mod MGI {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Tx Clock Stop Detection"]
    pub mod TX_CSD {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Rx Clock Stop Detection"]
    pub mod RX_CSD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Speed/Duplex change event mask."]
    pub mod SPD_DUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MAC merge frame SMD error received event interrupt mask"]
    pub mod MRG_SERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MAC merge frame assembly error event interrupt mask"]
    pub mod MRG_AERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Pause Quanta Register"]
pub mod PM0_PAUSE_QUANTA {
    pub use crate::RW as access;
    #[doc = "Value to be used for the quanta value when XOFF is triggered."]
    pub mod PQNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Pause Quanta Threshold Register"]
pub mod PM0_PAUSE_THRESH {
    pub use crate::RW as access;
    #[doc = "Quanta threshold."]
    pub mod QTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Pause Status Register"]
pub mod PM0_RX_PAUSE_STATUS {
    pub use crate::RO as access;
    #[doc = "Pause status."]
    pub mod PSTAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 EEE Low Power Wakeup Timer Register"]
pub mod PM0_LPWAKE_TIMER {
    pub use crate::RW as access;
    #[doc = "EEE System transmit wait time"]
    pub mod TW_SYS_TX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit EEE Low Power Timer Register"]
pub mod PM0_SLEEP_TIMER {
    pub use crate::RW as access;
    #[doc = "Defines the number of NETC cycles (which represents time) where Tx is idle before mac transmits low power EEE"]
    pub mod SLEEPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 IEEE1588 Single-Step Control Register"]
pub mod PM0_SINGLE_STEP {
    pub use crate::RW as access;
    #[doc = "Checksum update"]
    pub mod CH {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Start offset from the beginning of a frame where the field to update is found (index to MS byte)"]
    pub mod OFFSET {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IEEE-1588 Single-Step enable."]
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
#[doc = "Port MAC 0 half-duplex backoff entropy register"]
pub mod PM0_HD_BACKOFF_ENTROPY {
    pub use crate::RW as access;
    #[doc = "Half duplex backoff entropy"]
    pub mod HD_BACKOFF_ENTROPY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SW programmable entropy valid"]
    pub mod SW_ENTROPY_VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Half-Duplex Flow Control Register"]
pub mod PM0_HD_FLOW_CTRL {
    pub use crate::RW as access;
    #[doc = "Half-Duplex Back-Pressure Off Minimum"]
    pub mod HD_BP_OFF_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Half-Duplex Back-Pressure On Maximum"]
    pub mod HD_BP_ON_MAX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Statistics Configuration Register"]
pub mod PM0_STATN_CONFIG {
    pub use crate::RW as access;
    #[doc = "0 - (default) counters are wrapping around1- the counters saturate at the maximum value"]
    pub mod SAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0 - (default) counters are not affected by read.1 - a read to a counter resets it to 0."]
    pub mod COD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - all counters will be reset to 0"]
    pub mod CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write enable for Tx/Rx stats registers"]
    pub mod WEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Ethernet Octets Counter(etherStatsOctetsn)"]
pub mod PM0_REOCTN {
    pub use crate::RO as access;
    #[doc = "Incremented for each octet received in both good and bad packets."]
    pub mod REOCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Octets Counter(iflnOctetsn)"]
pub mod PM0_ROCTN {
    pub use crate::RO as access;
    #[doc = "Incremented for each octet received except preamble (that is, Header, Payload, Pad and FCS) for all valid frames and valid PAUSE frames received."]
    pub mod ROCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
pub mod PM0_RXPFN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid PAUSE frame received ."]
    pub mod RXPFN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Frame Counter Register(aFramesReceivedOKn)"]
pub mod PM0_RFRMN {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame received without error, including PAUSE frames."]
    pub mod RFRMN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Frame Check Sequence Error Counter Register()"]
pub mod PM0_RFCSN {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame received with a CRC-32 error but the frame is otherwise of correct length."]
    pub mod RFCSN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive VLAN Frame Counter Register(VLANReceivedOKn)"]
pub mod PM0_RVLANN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid VLAN tagged frame received with ethertype 0x8100"]
    pub mod RVLANN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Frame Error Counter Register(ifInErrorsn)"]
pub mod PM0_RERRN {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame received with an error (except for undersized/fragment frame):"]
    pub mod RERRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Unicast Frame Counter Register(ifInUcastPktsn)"]
pub mod PM0_RUCAN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame received (on the receive FIFO interface) in which bit 0 of the destination address was 0 ."]
    pub mod RUCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Multicast Frame Counter Register(ifInMulticastPktsn)"]
pub mod PM0_RMCAN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame received (on the receive FIFO interface) in which bit 0 of the destination address was 1 but not the broadcast address (all bits set to 1)"]
    pub mod RMCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Broadcast Frame Counter Register(ifInBroadcastPktsn)"]
pub mod PM0_RBCAN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame received (on the receive FIFO interface) in which all bits of the destination address were 1 ."]
    pub mod RBCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Dropped Packets Counter Register(etherStatsDropEventsn)"]
pub mod PM0_RDRPN {
    pub use crate::RO as access;
    #[doc = "Incremented for each dropped packet due to internal errors of the MAC client"]
    pub mod RDRPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Packets Counter Register(etherStatsPktsn)"]
pub mod PM0_RPKTN {
    pub use crate::RO as access;
    #[doc = "Incremented for each good or bad packet received."]
    pub mod RPKTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
pub mod PM0_RUNDN {
    pub use crate::RO as access;
    #[doc = "Incremented for each packet received that was less than the length programmed in PMa_MINFRM register and greater than or equal to 18 octets"]
    pub mod RUNDN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive 64-Octet Packet Counter Register(etherStatsPkts64OctetsN)"]
pub mod PM0_R64N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 64-octet frame received, good or bad."]
    pub mod R64N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive 65 to 127-Octet Packet Counter Register(etherStatsPkts65to127OctetsN)"]
pub mod PM0_R127N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 65- to 127-octet frame received, good or bad."]
    pub mod R127N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive 128 to 255-Octet Packet Counter Register(etherStatsPkts128to255OctetsN)"]
pub mod PM0_R255N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 128- to 255-octet frame received, good or bad."]
    pub mod R255N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive 256 to 511-Octet Packet Counter Register(etherStatsPkts256to511OctetsN)"]
pub mod PM0_R511N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 256- to 511-octet frame received, good or bad."]
    pub mod R511N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive 512 to 1023-Octet Packet Counter Register(etherStatsPkts512to1023OctetsN)"]
pub mod PM0_R1023N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 512- to 1023-octet frame received, good or bad."]
    pub mod R1023N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive 1024 to 1522-Octet Packet Counter Register(etherStatsPkts1024to1522OctetsN)"]
pub mod PM0_R1522N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 1024- to 1522-octet frame received, good or bad."]
    pub mod R1522N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive 1523 to Max-Octet Packet Counter Register(etherStatsPkts1523toMaxOctetsN)"]
pub mod PM0_R1523XN {
    pub use crate::RO as access;
    #[doc = "Incremented for each 1523-octet frame and larger (up to the maximum frame length specified in register PMa_MAXFRM\\[MAXFRM\\]) received, good or bad"]
    pub mod R1523XN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Oversized Packet Counter Register(etherStatsOversizePktsn)"]
pub mod PM0_ROVRN {
    pub use crate::RO as access;
    #[doc = "Incremented for each packet which is larger than the maximum frame length specified in the MAXFRM(FRAME_LENGTH) register (excluding framing bits, but including FCS octets) received with a good frame check sequence"]
    pub mod ROVRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Jabber Packet Counter Register(etherStatsJabbersn)"]
pub mod PM0_RJBRN {
    pub use crate::RO as access;
    #[doc = "Incremented for each packet which is larger than the maximum frame length specified in register PMa_MAXFRM\\[MAXFRM\\] (excluding framing bits, but including FCS octets) received with a bad frame check sequence"]
    pub mod RJBRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Fragment Packet Counter Register(etherStatsFragmentsn"]
pub mod PM0_RFRGN {
    pub use crate::RO as access;
    #[doc = "Incremented for each packet which is shorter than the length programmed in PMa_MINFRM register and received with a wrong CRC"]
    pub mod RFRGN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Control Packet Counter Register"]
pub mod PM0_RCNPN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid control packet (type 0x8808) but not for PAUSE packets"]
    pub mod RCNPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Dropped Not Truncated Packets Counter Register(etherStatsDropEventsn)"]
pub mod PM0_RDRNTPN {
    pub use crate::RO as access;
    #[doc = "Incremented for each fully dropped packet (not truncated) due to internal errors of the MAC client. Occurs when a receive FIFO overflows."]
    pub mod RDRNTPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Receive Valid Small Packet Counter Register"]
pub mod PM0_RMIN63N {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid small packet less than 64B but greater or equal to the length programmed in PMa_MINFRM register"]
    pub mod RMIN63N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Ethernet Octets Counter(etherStatsOctetsn)"]
pub mod PM0_TEOCTN {
    pub use crate::RO as access;
    #[doc = "Incremented for each octet transmitted in both good and bad packets."]
    pub mod TEOCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Octets Counter Register(ifOutOctetsn)"]
pub mod PM0_TOCTN {
    pub use crate::RO as access;
    #[doc = "Incremented for each octet transmitted except preamble (that is, Header, Payload, Pad and FCS) for all valid frames and valid PAUSE frames transmitted"]
    pub mod TOCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
pub mod PM0_TXPFN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid PAUSE frame transmitted . Note: Pause frames forwarded to the MAC from MAC Client are not counted by TXPFn."]
    pub mod TXPFN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Frame Counter Register(aFramesTransmittedOKn)"]
pub mod PM0_TFRMN {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame transmitted without error, including PAUSE frames."]
    pub mod TFRMN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Frame Check Sequence Error Counter Register()"]
pub mod PM0_TFCSN {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame transmitted with a CRC-32 error except for underflows."]
    pub mod TFCSN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit VLAN Frame Counter Register(VLANTransmittedOKn)"]
pub mod PM0_TVLANN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid VLAN tagged frame transmitted with ethertype 0x8100."]
    pub mod TVLANN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Frame Error Counter Register(ifOutErrorsn)"]
pub mod PM0_TERRN {
    pub use crate::RO as access;
    #[doc = "Transmit frame error count"]
    pub mod TERRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Unicast Frame Counter Register(ifOutUcastPktsn)"]
pub mod PM0_TUCAN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame transmitted (to the FIFO interface) in which bit 0 of the destination address was 0."]
    pub mod TUCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Multicast Frame Counter Register(ifOutMulticastPktsn)"]
pub mod PM0_TMCAN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame transmitted (to the FIFO interface) in which bit 0 of the destination address was 1 but not the broadcast address (all bits set to 1 )"]
    pub mod TMCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Broadcast Frame Counter Register(ifOutBroadcastPktsn)"]
pub mod PM0_TBCAN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame transmitted (to the FIFO interface) in which all bits of the destination address were 1 ."]
    pub mod TBCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Packets Counter Register(etherStatsPktsn)"]
pub mod PM0_TPKTN {
    pub use crate::RO as access;
    #[doc = "Incremented for each good or bad packet transmitted."]
    pub mod TPKTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
pub mod PM0_TUNDN {
    pub use crate::RO as access;
    #[doc = "Incremented for each packet transmitted that was less than 64 octets long with a good CRC."]
    pub mod TUNDN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit 64-Octet Packet Counter Register (etherStatsPkts64OctetsN)"]
pub mod PM0_T64N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 64-octet frame transmitted, good or bad."]
    pub mod T64N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit 65 to 127-Octet Packet Counter Register (etherStatsPkts65to127OctetsN)"]
pub mod PM0_T127N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 65 to 127-octet frame transmitted, good or bad."]
    pub mod T127N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit 128 to 255-Octet Packet Counter Register (etherStatsPkts128to255OctetsN)"]
pub mod PM0_T255N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 128 to 255-octet frame transmitted, good or bad."]
    pub mod T255N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit 256 to 511-Octet Packet Counter Register (etherStatsPkts256to511OctetsN)"]
pub mod PM0_T511N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 256 to 511-octet frame transmitted, good or bad."]
    pub mod T511N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit 512 to 1023-Octet Packet Counter Register (etherStatsPkts512to1023OctetsN)"]
pub mod PM0_T1023N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 512 to 1023-octet frame transmitted, good or bad."]
    pub mod T1023N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit 1024 to 1522-Octet Packet Counter Register (etherStatsPkts1024to1522OctetsN)"]
pub mod PM0_T1522N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 1024- to 1522-octet frame transmitted, good or bad."]
    pub mod T1522N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit 1523 to TX_MTU-Octet Packet Counter Register (etherStatsPkts1523toMaxOctetsN)"]
pub mod PM0_T1523XN {
    pub use crate::RO as access;
    #[doc = "Incremented for each 1523-octet frame and larger (up to the maximum frame length specified in register PMa_MAXFRM\\[TX_MTU\\]) transmitted, good or bad"]
    pub mod T1523XN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Control Packet Counter Register"]
pub mod PM0_TCNPN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid control packet transmitted (type 0x8808) but not for PAUSE packets"]
    pub mod TCNPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Deferred Packet Counter Register(aFramesWithDeferredXmissions)"]
pub mod PM0_TDFRN {
    pub use crate::RO as access;
    #[doc = "Increments for successful transmissions, without retransmits, that were deferred (half-duplex only)."]
    pub mod TDFRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Multiple Collisions Counter Register(aMultipleCollisionFrames)"]
pub mod PM0_TMCOLN {
    pub use crate::RO as access;
    #[doc = "Increments for successful transmission after more than one retransmission (half-duplex only)."]
    pub mod TMCOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Single Collision Counter(aSingleCollisionFrames) Register"]
pub mod PM0_TSCOLN {
    pub use crate::RO as access;
    #[doc = "Increments for successful transmission after one retransmission (half-duplex only)."]
    pub mod TSCOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Late Collision Counter(aLateCollisions) Register"]
pub mod PM0_TLCOLN {
    pub use crate::RO as access;
    #[doc = "Late collision occurred. Frame corrupted / discarded (half-duplex only)"]
    pub mod TLCOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Transmit Excessive Collisions Counter Register"]
pub mod PM0_TECOLN {
    pub use crate::RO as access;
    #[doc = "Excessive collisions occurred. Frame was discarded (half-duplex only)"]
    pub mod TECOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 0 Interface Mode Control Register"]
pub mod PM0_IF_MODE {
    pub use crate::RW as access;
    #[doc = "Interface mode"]
    pub mod IFMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reverse Mode"]
    pub mod REVMII {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reverse mode disabled - port is in MAC mode"]
            pub const MAC: u32 = 0;
            #[doc = "Reverse mode enabled - port is in PHY mode"]
            pub const PHY: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0 - 100 Mbps RMII/MII 1 - 10 Mbps RMII/MII (MII speed select is valid only in case of RevMII)"]
    pub mod M10 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "100 Mbps"]
            pub const M100: u32 = 0;
            #[doc = "10 Mbps"]
            pub const M10: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Half-duplex"]
    pub mod HD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "full duplex"]
            pub const FD: u32 = 0;
            #[doc = "half duplex"]
            pub const HD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Clock Stop"]
    pub mod CLK_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not stoppable"]
            pub const NO_STOP: u32 = 0;
            #[doc = "Stoppable"]
            pub const STOP: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Set Speed"]
    pub mod SSP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "100 Mbps"]
            pub const M100: u32 = 0;
            #[doc = "10 Mbps"]
            pub const M10: u32 = 0x01;
            #[doc = "1 Gbps"]
            pub const G1: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Command and Configuration Register"]
pub mod PM1_COMMAND_CONFIG {
    pub use crate::RW as access;
    #[doc = "MAC transmit path enable"]
    pub mod TX_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MAC receive path enable"]
    pub mod RX_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Terminate/forward received PAUSE frames"]
    pub mod PAUSE_FWD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ignore PAUSE frame quanta"]
    pub mod PAUSE_IGN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit source MAC address insertion"]
    pub mod TX_ADDR_INS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loopback enable"]
    pub mod LOOP_ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Loopback mode"]
    pub mod LPBK_MODE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Control frame reception enable"]
    pub mod CNT_FRM_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Timestamp Point"]
    pub mod TS_PNT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable padding of frames in transmit direction (1, default)."]
    pub mod TXP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Half Duplex Flow Control Enable"]
    pub mod HD_FCEN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Tx flush"]
    pub mod TX_FLUSH {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit Low Power Idle Enable."]
    pub mod TX_LOWP_ENA {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "(default), the MAC operates in normal mode."]
            pub const DISABLE: u32 = 0;
            #[doc = "The MAC completes the transmission of the current Frame and generates Low Power Idle Sequences to the line. It is advised to inspect IEVENT\\[TX_EMPTY\\] is set before enabling the LPI."]
            pub const ENABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Software Reset. Self clearing bit."]
    pub mod SWR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Ingress flush enable"]
    pub mod RX_FLUSH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit timestamp mode"]
    pub mod TS_MODE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Magic Packet detection enable."]
    pub mod MG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 MAC Address Register 0"]
pub mod PM1_MAC_ADDR_0 {
    pub use crate::RO as access;
    #[doc = "MAC address 0"]
    pub mod MAC_ADDR_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 MAC Address Register 1"]
pub mod PM1_MAC_ADDR_1 {
    pub use crate::RO as access;
    #[doc = "MAC address 1"]
    pub mod MAC_ADDR_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Maximum Frame Length Register"]
pub mod PM1_MAXFRM {
    pub use crate::RW as access;
    #[doc = "Maximum supported received frame length."]
    pub mod MAXFRM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Maximum transmit frame length"]
    pub mod TX_MTU {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Minimum Frame Length Register"]
pub mod PM1_MINFRM {
    pub use crate::RW as access;
    #[doc = "Receive Minimum Frame Length size in bytes."]
    pub mod NUM_BYTES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Interrupt Event Register"]
pub mod PM1_IEVENT {
    pub use crate::RW as access;
    #[doc = "Transmit fifo empty event"]
    pub mod TX_EMPTY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive idle event"]
    pub mod RX_EMPTY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit FIFO overflow event."]
    pub mod TX_OVFL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Transmit FIFO underflow event."]
    pub mod TX_UNFL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Receive FIFO overflow event."]
    pub mod RX_OVFL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Magic packet detection indication event"]
    pub mod MGI {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Tx Clock Stop Detection"]
    pub mod TX_CSD {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Rx Clock Stop Detection"]
    pub mod RX_CSD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Speed/Duplex Change"]
    pub mod SPD_DUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MAC merge frame SMD error received event"]
    pub mod MRG_SERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MAC merge frame assembly error event"]
    pub mod MRG_AERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Inter-Packet Gap Length and Flexible Preamble length Register"]
pub mod PM1_TX_IPG_PREAMBLE {
    pub use crate::RW as access;
    #[doc = "Transmit inter-packet gap value."]
    pub mod IPG_LEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Flexible Preamble Count"]
    pub mod FLEX_PREAMBLE_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Flexible Preamble Count"]
    pub mod FLEX_PREAMBLE_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Interrupt Mask Register(INT_MASK)"]
pub mod PM1_IMASK {
    pub use crate::RW as access;
    #[doc = "Magic packet detection indication event mask."]
    pub mod MGI {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Tx Clock Stop Detection"]
    pub mod TX_CSD {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Rx Clock Stop Detection"]
    pub mod RX_CSD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Speed/Duplex change event mask."]
    pub mod SPD_DUP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MAC merge frame SMD error received event interrupt mask"]
    pub mod MRG_SERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MAC merge frame assembly error event interrupt mask"]
    pub mod MRG_AERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Pause Quanta Register"]
pub mod PM1_PAUSE_QUANTA {
    pub use crate::RW as access;
    #[doc = "Value to be used for the quanta value when XOFF is triggered."]
    pub mod PQNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Pause Quanta Threshold Register"]
pub mod PM1_PAUSE_THRESH {
    pub use crate::RW as access;
    #[doc = "Quanta threshold."]
    pub mod QTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Pause Status Register"]
pub mod PM1_RX_PAUSE_STATUS {
    pub use crate::RO as access;
    #[doc = "Pause status."]
    pub mod PSTAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 EEE Low Power Wakeup Timer Register"]
pub mod PM1_LPWAKE_TIMER {
    pub use crate::RW as access;
    #[doc = "EEE System transmit wait time"]
    pub mod TW_SYS_TX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit EEE Low Power Timer Register"]
pub mod PM1_SLEEP_TIMER {
    pub use crate::RW as access;
    #[doc = "Defines the number of NETC cycles (which represents time) where Tx is idle before mac transmits low power EEE"]
    pub mod SLEEPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 IEEE1588 Single-Step Control Register"]
pub mod PM1_SINGLE_STEP {
    pub use crate::RW as access;
    #[doc = "Checksum update"]
    pub mod CH {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Start offset from the beginning of a frame where the field to update is found (index to MS byte)"]
    pub mod OFFSET {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "IEEE-1588 Single-Step enable."]
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
#[doc = "Port MAC 1 half-duplex backoff entropy register"]
pub mod PM1_HD_BACKOFF_ENTROPY {
    pub use crate::RW as access;
    #[doc = "Half duplex backoff entropy"]
    pub mod HD_BACKOFF_ENTROPY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "SW programmable entropy valid"]
    pub mod SW_ENTROPY_VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Half-Duplex Flow Control Register"]
pub mod PM1_HD_FLOW_CTRL {
    pub use crate::RW as access;
    #[doc = "Half-Duplex Back-Pressure Off Minimum"]
    pub mod HD_BP_OFF_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Half-Duplex Back-Pressure On Maximum"]
    pub mod HD_BP_ON_MAX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Statistics Configuration Register"]
pub mod PM1_STATN_CONFIG {
    pub use crate::RW as access;
    #[doc = "0 - (default) counters are wrapping around1- the counters saturate at the maximum value"]
    pub mod SAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0 - (default) counters are not affected by read.1 - a read to a counter resets it to 0."]
    pub mod COD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "1 - all counters will be reset to 0"]
    pub mod CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Write enable for Tx/Rx stats registers"]
    pub mod WEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Ethernet Octets Counter(etherStatsOctetsn)"]
pub mod PM1_REOCTN {
    pub use crate::RO as access;
    #[doc = "Incremented for each octet received in both good and bad packets."]
    pub mod REOCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Octets Counter(iflnOctetsn)"]
pub mod PM1_ROCTN {
    pub use crate::RO as access;
    #[doc = "Incremented for each octet received except preamble (that is, Header, Payload, Pad and FCS) for all valid frames and valid PAUSE frames received."]
    pub mod ROCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
pub mod PM1_RXPFN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid PAUSE frame received ."]
    pub mod RXPFN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Frame Counter Register(aFramesReceivedOKn)"]
pub mod PM1_RFRMN {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame received without error, including PAUSE frames."]
    pub mod RFRMN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Frame Check Sequence Error Counter Register()"]
pub mod PM1_RFCSN {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame received with a CRC-32 error but the frame is otherwise of correct length."]
    pub mod RFCSN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive VLAN Frame Counter Register(VLANReceivedOKn)"]
pub mod PM1_RVLANN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid VLAN tagged frame received with ethertype 0x8100"]
    pub mod RVLANN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Frame Error Counter Register(ifInErrorsn)"]
pub mod PM1_RERRN {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame received with an error (except for undersized/fragment frame):"]
    pub mod RERRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Unicast Frame Counter Register(ifInUcastPktsn)"]
pub mod PM1_RUCAN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame received (on the receive FIFO interface) in which bit 0 of the destination address was 0 ."]
    pub mod RUCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Multicast Frame Counter Register(ifInMulticastPktsn)"]
pub mod PM1_RMCAN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame received (on the receive FIFO interface) in which bit 0 of the destination address was 1 but not the broadcast address (all bits set to 1)"]
    pub mod RMCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Broadcast Frame Counter Register(ifInBroadcastPktsn)"]
pub mod PM1_RBCAN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame received (on the receive FIFO interface) in which all bits of the destination address were 1 ."]
    pub mod RBCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Dropped Packets Counter Register(etherStatsDropEventsn)"]
pub mod PM1_RDRPN {
    pub use crate::RO as access;
    #[doc = "Incremented for each dropped packet due to internal errors of the MAC client"]
    pub mod RDRPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Packets Counter Register(etherStatsPktsn)"]
pub mod PM1_RPKTN {
    pub use crate::RO as access;
    #[doc = "Incremented for each good or bad packet received."]
    pub mod RPKTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
pub mod PM1_RUNDN {
    pub use crate::RO as access;
    #[doc = "Incremented for each packet received that was less than the length programmed in PMa_MINFRM register and greater than or equal to 18 octets"]
    pub mod RUNDN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive 64-Octet Packet Counter Register(etherStatsPkts64OctetsN)"]
pub mod PM1_R64N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 64-octet frame received, good or bad."]
    pub mod R64N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive 65 to 127-Octet Packet Counter Register(etherStatsPkts65to127OctetsN)"]
pub mod PM1_R127N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 65- to 127-octet frame received, good or bad."]
    pub mod R127N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive 128 to 255-Octet Packet Counter Register(etherStatsPkts128to255OctetsN)"]
pub mod PM1_R255N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 128- to 255-octet frame received, good or bad."]
    pub mod R255N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive 256 to 511-Octet Packet Counter Register(etherStatsPkts256to511OctetsN)"]
pub mod PM1_R511N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 256- to 511-octet frame received, good or bad."]
    pub mod R511N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive 512 to 1023-Octet Packet Counter Register(etherStatsPkts512to1023OctetsN)"]
pub mod PM1_R1023N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 512- to 1023-octet frame received, good or bad."]
    pub mod R1023N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive 1024 to 1522-Octet Packet Counter Register(etherStatsPkts1024to1522OctetsN)"]
pub mod PM1_R1522N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 1024- to 1522-octet frame received, good or bad."]
    pub mod R1522N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive 1523 to Max-Octet Packet Counter Register(etherStatsPkts1523toMaxOctetsN)"]
pub mod PM1_R1523XN {
    pub use crate::RO as access;
    #[doc = "Incremented for each 1523-octet frame and larger (up to the maximum frame length specified in register PMa_MAXFRM\\[MAXFRM\\]) received, good or bad"]
    pub mod R1523XN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Oversized Packet Counter Register(etherStatsOversizePktsn)"]
pub mod PM1_ROVRN {
    pub use crate::RO as access;
    #[doc = "Incremented for each packet which is larger than the maximum frame length specified in the MAXFRM(FRAME_LENGTH) register (excluding framing bits, but including FCS octets) received with a good frame check sequence"]
    pub mod ROVRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Jabber Packet Counter Register(etherStatsJabbersn)"]
pub mod PM1_RJBRN {
    pub use crate::RO as access;
    #[doc = "Incremented for each packet which is larger than the maximum frame length specified in register PMa_MAXFRM\\[MAXFRM\\] (excluding framing bits, but including FCS octets) received with a bad frame check sequence"]
    pub mod RJBRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Fragment Packet Counter Register(etherStatsFragmentsn"]
pub mod PM1_RFRGN {
    pub use crate::RO as access;
    #[doc = "Incremented for each packet which is shorter than the length programmed in PMa_MINFRM register and received with a wrong CRC"]
    pub mod RFRGN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Control Packet Counter Register"]
pub mod PM1_RCNPN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid control packet (type 0x8808) but not for PAUSE packets"]
    pub mod RCNPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Dropped Not Truncated Packets Counter Register(etherStatsDropEventsn)"]
pub mod PM1_RDRNTPN {
    pub use crate::RO as access;
    #[doc = "Incremented for each fully dropped packet (not truncated) due to internal errors of the MAC client. Occurs when a receive FIFO overflows."]
    pub mod RDRNTPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Receive Valid Small Packet Counter Register"]
pub mod PM1_RMIN63N {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid small packet less than 64B but greater or equal to the length programmed in PMa_MINFRM register"]
    pub mod RMIN63N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Ethernet Octets Counter(etherStatsOctetsn)"]
pub mod PM1_TEOCTN {
    pub use crate::RO as access;
    #[doc = "Incremented for each octet transmitted in both good and bad packets."]
    pub mod TEOCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Octets Counter Register(ifOutOctetsn)"]
pub mod PM1_TOCTN {
    pub use crate::RO as access;
    #[doc = "Incremented for each octet transmitted except preamble (that is, Header, Payload, Pad and FCS) for all valid frames and valid PAUSE frames transmitted"]
    pub mod TOCTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Valid Pause Frame Counter Register(aPAUSEMACCtrlFramesReceivedn)"]
pub mod PM1_TXPFN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid PAUSE frame transmitted . Note: Pause frames forwarded to the MAC from MAC Client are not counted by TXPFn."]
    pub mod TXPFN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Frame Counter Register(aFramesTransmittedOKn)"]
pub mod PM1_TFRMN {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame transmitted without error, including PAUSE frames."]
    pub mod TFRMN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Frame Check Sequence Error Counter Register()"]
pub mod PM1_TFCSN {
    pub use crate::RO as access;
    #[doc = "Incremented for each frame transmitted with a CRC-32 error except for underflows."]
    pub mod TFCSN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit VLAN Frame Counter Register(VLANTransmittedOKn)"]
pub mod PM1_TVLANN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid VLAN tagged frame transmitted with ethertype 0x8100."]
    pub mod TVLANN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Frame Error Counter Register(ifOutErrorsn)"]
pub mod PM1_TERRN {
    pub use crate::RO as access;
    #[doc = "Transmit frame error count"]
    pub mod TERRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Unicast Frame Counter Register(ifOutUcastPktsn)"]
pub mod PM1_TUCAN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame transmitted (to the FIFO interface) in which bit 0 of the destination address was 0."]
    pub mod TUCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Multicast Frame Counter Register(ifOutMulticastPktsn)"]
pub mod PM1_TMCAN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame transmitted (to the FIFO interface) in which bit 0 of the destination address was 1 but not the broadcast address (all bits set to 1 )"]
    pub mod TMCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Broadcast Frame Counter Register(ifOutBroadcastPktsn)"]
pub mod PM1_TBCAN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid frame transmitted (to the FIFO interface) in which all bits of the destination address were 1 ."]
    pub mod TBCAN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Packets Counter Register(etherStatsPktsn)"]
pub mod PM1_TPKTN {
    pub use crate::RO as access;
    #[doc = "Incremented for each good or bad packet transmitted."]
    pub mod TPKTN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Undersized Packet Counter Register(etherStatsUndersizePktsn)"]
pub mod PM1_TUNDN {
    pub use crate::RO as access;
    #[doc = "Incremented for each packet transmitted that was less than 64 octets long with a good CRC."]
    pub mod TUNDN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit 64-Octet Packet Counter Register (etherStatsPkts64OctetsN)"]
pub mod PM1_T64N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 64-octet frame transmitted, good or bad."]
    pub mod T64N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit 65 to 127-Octet Packet Counter Register (etherStatsPkts65to127OctetsN)"]
pub mod PM1_T127N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 65 to 127-octet frame transmitted, good or bad."]
    pub mod T127N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit 128 to 255-Octet Packet Counter Register (etherStatsPkts128to255OctetsN)"]
pub mod PM1_T255N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 128 to 255-octet frame transmitted, good or bad."]
    pub mod T255N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit 256 to 511-Octet Packet Counter Register (etherStatsPkts256to511OctetsN)"]
pub mod PM1_T511N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 256 to 511-octet frame transmitted, good or bad."]
    pub mod T511N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit 512 to 1023-Octet Packet Counter Register (etherStatsPkts512to1023OctetsN)"]
pub mod PM1_T1023N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 512 to 1023-octet frame transmitted, good or bad."]
    pub mod T1023N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit 1024 to 1522-Octet Packet Counter Register (etherStatsPkts1024to1522OctetsN)"]
pub mod PM1_T1522N {
    pub use crate::RO as access;
    #[doc = "Incremented for each 1024- to 1522-octet frame transmitted, good or bad."]
    pub mod T1522N {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit 1523 to TX_MTU-Octet Packet Counter Register (etherStatsPkts1523toMaxOctetsN)"]
pub mod PM1_T1523XN {
    pub use crate::RO as access;
    #[doc = "Incremented for each 1523-octet frame and larger (up to the maximum frame length specified in register PMa_MAXFRM\\[TX_MTU\\]) transmitted, good or bad"]
    pub mod T1523XN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Control Packet Counter Register"]
pub mod PM1_TCNPN {
    pub use crate::RO as access;
    #[doc = "Incremented for each valid control packet transmitted (type 0x8808) but not for PAUSE packets"]
    pub mod TCNPN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Deferred Packet Counter Register(aFramesWithDeferredXmissions)"]
pub mod PM1_TDFRN {
    pub use crate::RO as access;
    #[doc = "Increments for successful transmissions, without retransmits, that were deferred (half-duplex only)."]
    pub mod TDFRN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Multiple Collisions Counter Register(aMultipleCollisionFrames)"]
pub mod PM1_TMCOLN {
    pub use crate::RO as access;
    #[doc = "Increments for successful transmission after more than one retransmission (half-duplex only)."]
    pub mod TMCOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Single Collision Counter(aSingleCollisionFrames) Register"]
pub mod PM1_TSCOLN {
    pub use crate::RO as access;
    #[doc = "Increments for successful transmission after one retransmission (half-duplex only)."]
    pub mod TSCOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Late Collision Counter(aLateCollisions) Register"]
pub mod PM1_TLCOLN {
    pub use crate::RO as access;
    #[doc = "Late collision occurred. Frame corrupted / discarded (half-duplex only)"]
    pub mod TLCOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Transmit Excessive Collisions Counter Register"]
pub mod PM1_TECOLN {
    pub use crate::RO as access;
    #[doc = "Excessive collisions occurred. Frame was discarded (half-duplex only)"]
    pub mod TECOLN {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC 1 Interface Mode Control Register"]
pub mod PM1_IF_MODE {
    pub use crate::RW as access;
    #[doc = "Interface mode"]
    pub mod IFMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Reverse Mode"]
    pub mod REVMII {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Reverse mode disabled - port is in MAC mode"]
            pub const MAC: u32 = 0;
            #[doc = "Reverse mode enabled - port is in PHY mode"]
            pub const PHY: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "0 - 100 Mbps RMII/MII 1 - 10 Mbps RMII/MII (MII speed select is valid only in case of RevMII)"]
    pub mod M10 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "100 Mbps"]
            pub const M100: u32 = 0;
            #[doc = "10 Mbps"]
            pub const M10: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Half-duplex"]
    pub mod HD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "full duplex"]
            pub const FD: u32 = 0;
            #[doc = "half duplex"]
            pub const HD: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Clock Stop"]
    pub mod CLK_STOP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Not stoppable"]
            pub const NO_STOP: u32 = 0;
            #[doc = "Stoppable"]
            pub const STOP: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Set Speed"]
    pub mod SSP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "100 Mbps"]
            pub const M100: u32 = 0;
            #[doc = "10 Mbps"]
            pub const M10: u32 = 0x01;
            #[doc = "1 Gbps"]
            pub const G1: u32 = 0x02;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC Merge Control and Status Register"]
pub mod MAC_MERGE_MMCSR {
    pub use crate::RW as access;
    #[doc = "Local preemption supported"]
    pub mod LPS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Local preemption enabled"]
    pub mod LPE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Local preemption active"]
    pub mod LPA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Local additional fragment size"]
    pub mod LAFS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Remote preemption supported"]
    pub mod RPS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Remote preemption enabled"]
    pub mod RPE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Remote preemption active"]
    pub mod RPA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Remote additional fragment size"]
    pub mod RAFS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Merge enabled"]
    pub mod ME {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Verify disabled"]
    pub mod VDIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Verify status"]
    pub mod VSTS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Merge status"]
    pub mod TXSTS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Verify Time"]
    pub mod VT {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x7f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Link Fail"]
    pub mod LINK_FAIL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC Merge Frame Assembly Error Count Register"]
pub mod MAC_MERGE_MMFAECR {
    pub use crate::RW as access;
    #[doc = "A count of MAC frames with reassembly errors."]
    pub mod MMFAEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC Merge Frame SMD Error Count Register"]
pub mod MAC_MERGE_MMFSECR {
    pub use crate::RW as access;
    #[doc = "A count of received MAC frames / MAC frame fragments rejected due to unknown SMD value or arriving with an SMD-C when no frame is in progress"]
    pub mod MMFSEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC Merge Frame Assembly OK Count Register"]
pub mod MAC_MERGE_MMFAOCR {
    pub use crate::RW as access;
    #[doc = "A count of MAC frames that were successfully reassembled and delivered to the MAC."]
    pub mod MMFAOC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC Merge Fragment Count RX Register"]
pub mod MAC_MERGE_MMFCRXR {
    pub use crate::RW as access;
    #[doc = "A count of the number of additional mPackets received due to preemption."]
    pub mod MMFCRX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC Merge Fragment Count TX Register"]
pub mod MAC_MERGE_MMFCTXR {
    pub use crate::RW as access;
    #[doc = "A count of the number of additional mPackets transmitted due to preemption."]
    pub mod MMFCTX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port MAC Merge Hold Count Register"]
pub mod MAC_MERGE_MMHCR {
    pub use crate::RW as access;
    #[doc = "A count of the number of times the variable hold transitions from false to true."]
    pub mod MMHC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port external MDIO configuration register"]
pub mod PEMDIOCR {
    pub use crate::RW as access;
    #[doc = "Busy 2 (same as bit 31)"]
    pub mod BSY2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "An MDIO transaction is not occurring; software may access other MDIO registers."]
            pub const ZERO: u32 = 0;
            #[doc = "An MDIO transaction is occurring."]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MDIO Read (and write) Error"]
    pub mod MDIO_RD_ER {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "No error on last MDIO transaction (read or write)."]
            pub const ZERO: u32 = 0;
            #[doc = "An error was detected on the last MDIO transaction (read or write). Errors on internal MDIO accesses can be triggered by an access to an invalid device, or by a write to a shared on-die PHY device that has not been locked."]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MDIO Hold Time"]
    pub mod MDIO_HOLD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "1 NETC cycle"]
            pub const NETC1: u32 = 0;
            #[doc = "3 NETC cycles"]
            pub const NETC3: u32 = 0x01;
            #[doc = "5 NETC cycles (default - recommended value)"]
            pub const NETC5: u32 = 0x02;
            #[doc = "7 NETC cycles"]
            pub const NETC7: u32 = 0x03;
            #[doc = "9 NETC cycles"]
            pub const NETC9: u32 = 0x04;
            #[doc = "11 NETC cycles"]
            pub const NETC11: u32 = 0x05;
            #[doc = "13 NETC cycles"]
            pub const NETC13: u32 = 0x06;
            #[doc = "15 NETC cycles"]
            pub const NETC15: u32 = 0x07;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MDIO Preamble Disable"]
    pub mod PRE_DIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Generation of MDIO preamble is enabled (default operation)."]
            pub const ENABLE: u32 = 0;
            #[doc = "Generation of MDIO preamble is disabled."]
            pub const DISABLE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Enable Clause 45 Support"]
    pub mod ENC45 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Clause 22 transactions are used."]
            pub const ZERO: u32 = 0;
            #[doc = "Clause 45 transactions are used."]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MDIO Clock Divisor"]
    pub mod MDIO_CLK_DIV {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Returns the link ID"]
    pub mod WHOAMI {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Extended HOLD"]
    pub mod EHOLD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Normal operation, MDIO hold time is as specified in PEMDIOCR\\[MDIO_HOLD\\]."]
            pub const ZERO: u32 = 0;
            #[doc = "Extended operation"]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Negative Edge"]
    pub mod NEG {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "normal operation - positive edge"]
            pub const ZERO: u32 = 0;
            #[doc = "MDIO is driven by master on MDC negative edge (default for external MDIOs)"]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Address Error"]
    pub mod ADDR_ERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Normal"]
            pub const ZERO: u32 = 0;
            #[doc = "Error. An access control violation has occurred. The request address used does not match the MDIO PHY's address (clause 22) or MDIO port address (clause 45) assigned."]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MDIO Command Completion Interrupt Mask"]
    pub mod CIM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "Masked"]
            pub const ZERO: u32 = 0;
            #[doc = "Enabled"]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MDIO Command Completion"]
    pub mod CMP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "An MDIO command completion did not occur."]
            pub const ZERO: u32 = 0;
            #[doc = "An MDIO command completion occurred."]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Busy 1"]
    pub mod BSY1 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use crate::RW as access;
        #[doc(hidden)]
        pub mod vals {
            #[doc = "An MDIO transaction is not occurring; software may access other MDIO registers."]
            pub const ZERO: u32 = 0;
            #[doc = "An MDIO transaction is occurring."]
            pub const ONE: u32 = 0x01;
        }
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port external MDIO interface control register"]
pub mod PEMDIOICR {
    pub use crate::RW as access;
    #[doc = "5-bit MDIO device address (Clause 45) / register address (Clause 22)"]
    pub mod DEV_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "5-bit MDIO port address (Clause 45) / PHY address (Clause 22)"]
    pub mod PORT_ADDR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MDIO read with address post-increment initiation. Self-clearing once transaction is complete."]
    pub mod POST_INC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MDIO read initiation."]
    pub mod READ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MDIO busy"]
    pub mod BSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port external MDIO interface data register"]
pub mod PEMDIOIDR {
    pub use crate::RW as access;
    #[doc = "16-bit MDIO data."]
    pub mod MDIO_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port external MDIO register address register"]
pub mod PEMDIORAR {
    pub use crate::RW as access;
    #[doc = "MDIO PHY register address."]
    pub mod REGADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port external MDIO status register"]
pub mod PEMDIOSR {
    pub use crate::RO as access;
    #[doc = "Global MDIO busy"]
    pub mod BSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PHY white list"]
    pub mod WHT_LIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PHY white list enable"]
    pub mod WHT_LIST_ENA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Port ID"]
    pub mod PORT_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Port ID"]
    pub mod REQ_TYPE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "PHY status configuration register"]
pub mod PPSCR {
    pub use crate::RW as access;
    #[doc = "MDIO busy"]
    pub mod BSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "MDIO read error"]
    pub mod MDIO_RD_ER {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "PHY status read interval"]
    pub mod STATUS_INTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port PHY status control register"]
pub mod PPSCTRLR {
    pub use crate::RW as access;
    #[doc = "5-bit MDIO device address (Clause 45) / register address (Clause 22)"]
    pub mod DEV_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "5-bit MDIO port address (Clause 45) / PHY address (Clause 22)"]
    pub mod PORT_ADDR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port PHY status data register"]
pub mod PPSDR {
    pub use crate::RO as access;
    #[doc = "16-bit MDIO data"]
    pub mod MDIO_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Current count"]
    pub mod CURR_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port PHY status register address register"]
pub mod PPSRAR {
    pub use crate::RW as access;
    #[doc = "MDIO PHY register address. Address of the register within the Clause 45 PHY device from which data is to be read."]
    pub mod REGADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port PHY status event register"]
pub mod PPSER {
    pub use crate::RW as access;
    #[doc = "Status event high-to-low. Set to 1 if a 1->0 transition on a corresponding data bit has occurred. Write 1 to clear."]
    pub mod STATUS_EVENT_HL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Status event low-to-high. Set to 1 if a 0->1 transition on a corresponding data bit has occurred. Write 1 to clear."]
    pub mod STATUS_EVENT_LH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
#[doc = "Port PHY status mask register"]
pub mod PPSMR {
    pub use crate::RW as access;
    #[doc = "Status high-to-low mask. If set to 1, assert an interrupt if the corresponding event bit is set."]
    pub mod STATUS_MASK_HL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
    #[doc = "Status mask low-to-high. If set to 1, assert an interrupt if the corresponding event bit is set."]
    pub mod STATUS_MASK_LH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub use super::access;
        #[doc(hidden)]
        pub mod vals {}
        #[doc(inline)]
        pub use vals::*;
    }
}
