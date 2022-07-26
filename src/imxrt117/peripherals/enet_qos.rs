#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ENET_QOS
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// MAC Configuration Register
pub mod MAC_CONFIGURATION {

    /// Receiver Enable
    pub mod RE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receiver is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receiver is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Transmitter Enable
    pub mod TE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmitter is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmitter is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Preamble Length for Transmit packets
    pub mod PRELEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 7 bytes of preamble
            pub const BYTES_7: u32 = 0b00;

            /// 0b01: 5 bytes of preamble
            pub const BYTES_5: u32 = 0b01;

            /// 0b10: 3 bytes of preamble
            pub const BYTES_3: u32 = 0b10;
        }
    }

    /// Deferral Check
    pub mod DC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Deferral check function is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Deferral check function is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Back-Off Limit
    pub mod BL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: k = min(n,10)
            pub const MIN_N_10: u32 = 0b00;

            /// 0b01: k = min(n,8)
            pub const MIN_N_8: u32 = 0b01;

            /// 0b10: k = min(n,4)
            pub const MIN_N_4: u32 = 0b10;

            /// 0b11: k = min(n,1)
            pub const MIN_N_1: u32 = 0b11;
        }
    }

    /// Disable Retry
    pub mod DR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enable Retry
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Disable Retry
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Disable Carrier Sense During Transmission
    pub mod DCRS {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enable Carrier Sense During Transmission
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Disable Carrier Sense During Transmission
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Disable Receive Own
    pub mod DO {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enable Receive Own
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Disable Receive Own
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Enable Carrier Sense Before Transmission in Full-Duplex Mode
    pub mod ECRSFD {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ECRSFD is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: ECRSFD is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Loopback Mode
    pub mod LM {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Loopback is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Loopback is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Duplex Mode
    pub mod DM {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Half-duplex mode
            pub const HDUPLX: u32 = 0b0;

            /// 0b1: Full-duplex mode
            pub const FDUPLX: u32 = 0b1;
        }
    }

    /// Speed
    pub mod FES {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 10 Mbps when PS bit is 1 and 1 Gbps when PS bit is 0
            pub const Mbps_10_1000M: u32 = 0b0;

            /// 0b1: 100 Mbps when PS bit is 1 and 2.5 Gbps when PS bit is 0
            pub const Mbps_100_2500M: u32 = 0b1;
        }
    }

    /// Port Select
    pub mod PS {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: For 1000 or 2500 Mbps operations
            pub const bf_1000_2500M: u32 = 0b0;

            /// 0b1: For 10 or 100 Mbps operations
            pub const bf_10_100M: u32 = 0b1;
        }
    }

    /// Jumbo Packet Enable When this bit is set, the MAC allows jumbo packets of 9,018 bytes (9,022 bytes for VLAN tagged packets) without reporting a giant packet error in the Rx packet status.
    pub mod JE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Jumbo packet is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Jumbo packet is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Jabber Disable
    pub mod JD {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Jabber is enabled
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Jabber is disabled
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Packet Burst Enable When this bit is set, the MAC allows packet bursting during transmission in the GMII half-duplex mode.
    pub mod BE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Packet Burst is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Packet Burst is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Watchdog Disable
    pub mod WD {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Watchdog is enabled
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Watchdog is disabled
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes.
    pub mod ACS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic Pad or CRC Stripping is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Automatic Pad or CRC Stripping is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application.
    pub mod CST {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: CRC stripping for Type packets is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: CRC stripping for Type packets is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// IEEE 802.
    pub mod S2KP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Support upto 2K packet is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Support upto 2K packet is Enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Giant Packet Size Limit Control Enable
    pub mod GPSLCE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Giant Packet Size Limit Control is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Giant Packet Size Limit Control is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Inter-Packet Gap These bits control the minimum IPG between packets during transmission.
    pub mod IPG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 96 bit times IPG
            pub const IPG96: u32 = 0b000;

            /// 0b001: 88 bit times IPG
            pub const IPG88: u32 = 0b001;

            /// 0b010: 80 bit times IPG
            pub const IPG80: u32 = 0b010;

            /// 0b011: 72 bit times IPG
            pub const IPG72: u32 = 0b011;

            /// 0b100: 64 bit times IPG
            pub const IPG64: u32 = 0b100;

            /// 0b101: 56 bit times IPG
            pub const IPG56: u32 = 0b101;

            /// 0b110: 48 bit times IPG
            pub const IPG48: u32 = 0b110;

            /// 0b111: 40 bit times IPG
            pub const IPG40: u32 = 0b111;
        }
    }

    /// Checksum Offload
    pub mod IPC {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: IP header/payload checksum checking is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: IP header/payload checksum checking is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Source Address Insertion or Replacement Control
    pub mod SARC {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (3 bits: 0b111 << 28)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: mti_sa_ctrl_i and ati_sa_ctrl_i input signals control the SA field generation
            pub const SA_CTRL_IN: u32 = 0b000;

            /// 0b010: Contents of MAC Addr-0 inserted in SA field
            pub const MAC0_INS_SA: u32 = 0b010;

            /// 0b011: Contents of MAC Addr-0 replaces SA field
            pub const MAC0_REP_SA: u32 = 0b011;

            /// 0b110: Contents of MAC Addr-1 inserted in SA field
            pub const MAC1_INS_SA: u32 = 0b110;

            /// 0b111: Contents of MAC Addr-1 replaces SA field
            pub const MAC1_REP_SA: u32 = 0b111;
        }
    }
}

/// MAC Extended Configuration Register
pub mod MAC_EXT_CONFIGURATION {

    /// Giant Packet Size Limit
    pub mod GPSL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Disable CRC Checking for Received Packets
    pub mod DCRCC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: CRC Checking is enabled
            pub const ENABLE: u32 = 0b0;

            /// 0b1: CRC Checking is disabled
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Slow Protocol Detection Enable
    pub mod SPEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slow Protocol Detection is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Slow Protocol Detection is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Unicast Slow Protocol Packet Detect
    pub mod USP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Unicast Slow Protocol Packet Detection is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Unicast Slow Protocol Packet Detection is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Packet Duplication Control
    pub mod PDC {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Packet Duplication Control is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Packet Duplication Control is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Extended Inter-Packet Gap Enable
    pub mod EIPGEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Extended Inter-Packet Gap is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Extended Inter-Packet Gap is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Extended Inter-Packet Gap
    pub mod EIPG {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (5 bits: 0b11111 << 25)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MAC Packet Filter
pub mod MAC_PACKET_FILTER {

    /// Promiscuous Mode
    pub mod PR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Promiscuous Mode is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Promiscuous Mode is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Hash Unicast
    pub mod HUC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Hash Unicast is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Hash Unicast is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Hash Multicast
    pub mod HMC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Hash Multicast is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Hash Multicast is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// DA Inverse Filtering
    pub mod DAIF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DA Inverse Filtering is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: DA Inverse Filtering is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Pass All Multicast
    pub mod PM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Pass All Multicast is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Pass All Multicast is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Disable Broadcast Packets
    pub mod DBF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enable Broadcast Packets
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Disable Broadcast Packets
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Pass Control Packets These bits control the forwarding of all control packets (including unicast and multicast Pause packets).
    pub mod PCF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: MAC filters all control packets from reaching the application
            pub const FLTR_ALL: u32 = 0b00;

            /// 0b01: MAC forwards all control packets except Pause packets to the application even if they fail the Address filter
            pub const FW_XCPT_PAU: u32 = 0b01;

            /// 0b10: MAC forwards all control packets to the application even if they fail the Address filter
            pub const FW_ALL: u32 = 0b10;

            /// 0b11: MAC forwards the control packets that pass the Address filter
            pub const FW_PASS: u32 = 0b11;
        }
    }

    /// SA Inverse Filtering
    pub mod SAIF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SA Inverse Filtering is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: SA Inverse Filtering is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Source Address Filter Enable
    pub mod SAF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SA Filtering is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: SA Filtering is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Hash or Perfect Filter
    pub mod HPF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Hash or Perfect Filter is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Hash or Perfect Filter is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// VLAN Tag Filter Enable
    pub mod VTFE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VLAN Tag Filter is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: VLAN Tag Filter is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 and Layer 4 Filter Enable
    pub mod IPFE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 and Layer 4 Filters are disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 and Layer 4 Filters are enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Drop Non-TCP/UDP over IP Packets
    pub mod DNTU {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Forward Non-TCP/UDP over IP Packets
            pub const FWD: u32 = 0b0;

            /// 0b1: Drop Non-TCP/UDP over IP Packets
            pub const DROP: u32 = 0b1;
        }
    }

    /// Receive All
    pub mod RA {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive All is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive All is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Watchdog Timeout
pub mod MAC_WATCHDOG_TIMEOUT {

    /// Watchdog Timeout
    pub mod WTO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 2 KB
            pub const bf_2KBYTES: u32 = 0b0000;

            /// 0b0001: 3 KB
            pub const bf_3KBYTES: u32 = 0b0001;

            /// 0b0010: 4 KB
            pub const bf_4KBYTES: u32 = 0b0010;

            /// 0b0011: 5 KB
            pub const bf_5KBYTES: u32 = 0b0011;

            /// 0b0100: 6 KB
            pub const bf_6KBYTES: u32 = 0b0100;

            /// 0b0101: 7 KB
            pub const bf_7KBYTES: u32 = 0b0101;

            /// 0b0110: 8 KB
            pub const bf_8KBYTES: u32 = 0b0110;

            /// 0b0111: 9 KB
            pub const bf_9KBYTES: u32 = 0b0111;

            /// 0b1000: 10 KB
            pub const bf_10KBYTES: u32 = 0b1000;

            /// 0b1001: 11 KB
            pub const bf_11KBYTES: u32 = 0b1001;

            /// 0b1010: 12 KB
            pub const bf_12KBYTES: u32 = 0b1010;

            /// 0b1011: 13 KB
            pub const bf_13KBYTES: u32 = 0b1011;

            /// 0b1100: 14 KB
            pub const bf_14KBYTES: u32 = 0b1100;

            /// 0b1101: 15 KB
            pub const bf_15KBYTES: u32 = 0b1101;

            /// 0b1110: 16383 Bytes
            pub const bf_16383BYTES: u32 = 0b1110;
        }
    }

    /// Programmable Watchdog Enable
    pub mod PWE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Programmable Watchdog is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Programmable Watchdog is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// MAC Hash Table Register 0
pub mod MAC_HASH_TABLE_REG0 {

    /// MAC Hash Table First 32 Bits This field contains the first 32 Bits \[31:0\] of the Hash table.
    pub mod HT31T0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MAC Hash Table Register 1
pub mod MAC_HASH_TABLE_REG1 {

    /// MAC Hash Table Second 32 Bits This field contains the second 32 Bits \[63:32\] of the Hash table.
    pub mod HT63T32 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MAC VLAN Tag Control
pub mod MAC_VLAN_TAG_CTRL {

    /// Operation Busy
    pub mod OB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Operation Busy is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Operation Busy is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Command Type
    pub mod CT {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write operation
            pub const WRITE: u32 = 0b0;

            /// 0b1: Read operation
            pub const READ: u32 = 0b1;
        }
    }

    /// Offset
    pub mod OFS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (5 bits: 0b11111 << 2)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VLAN Tag Inverse Match Enable
    pub mod VTIM {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VLAN Tag Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: VLAN Tag Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable S-VLAN When this bit is set, the MAC transmitter and receiver consider the S-VLAN packets (Type = 0x88A8) as valid VLAN tagged packets.
    pub mod ESVL {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: S-VLAN is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: S-VLAN is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable VLAN Tag Stripping on Receive This field indicates the stripping operation on the outer VLAN Tag in received packet.
    pub mod EVLS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (2 bits: 0b11 << 21)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Do not strip
            pub const DONOT: u32 = 0b00;

            /// 0b01: Strip if VLAN filter passes
            pub const IFPASS: u32 = 0b01;

            /// 0b10: Strip if VLAN filter fails
            pub const IFFAIL: u32 = 0b10;

            /// 0b11: Always strip
            pub const ALWAYS: u32 = 0b11;
        }
    }

    /// Enable VLAN Tag in Rx status
    pub mod EVLRXS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VLAN Tag in Rx status is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: VLAN Tag in Rx status is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// VLAN Tag Hash Table Match Enable
    pub mod VTHM {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VLAN Tag Hash Table Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: VLAN Tag Hash Table Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable Double VLAN Processing
    pub mod EDVLP {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Double VLAN Processing is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Double VLAN Processing is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// ERIVLT
    pub mod ERIVLT {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Inner VLAN tag is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Inner VLAN tag is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable Inner VLAN Tag Stripping on Receive This field indicates the stripping operation on inner VLAN Tag in received packet.
    pub mod EIVLS {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EVLS::RW;
    }

    /// Enable Inner VLAN Tag in Rx Status
    pub mod EIVLRXS {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Inner VLAN Tag in Rx status is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Inner VLAN Tag in Rx status is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// MAC VLAN Tag Data
pub mod MAC_VLAN_TAG_DATA {

    /// VLAN Tag ID
    pub mod VID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VLAN Tag Enable
    pub mod VEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VLAN Tag is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: VLAN Tag is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// 12bits or 16bits VLAN comparison
    pub mod ETV {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 16 bit VLAN comparison
            pub const bf_16BIT: u32 = 0b0;

            /// 0b1: 12 bit VLAN comparison
            pub const bf_12BIT: u32 = 0b1;
        }
    }

    /// Disable VLAN Type Comparison
    pub mod DOVLTC {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VLAN type comparison is enabled
            pub const ENABLE: u32 = 0b0;

            /// 0b1: VLAN type comparison is disabled
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Enable S-VLAN Match for received Frames
    pub mod ERSVLM {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive S-VLAN Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive S-VLAN Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable Inner VLAN Tag Comparison
    pub mod ERIVLT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Inner VLAN tag comparison is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Inner VLAN tag comparison is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// DMA Channel Number Enable
    pub mod DMACHEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel Number is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: DMA Channel Number is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// DMA Channel Number
    pub mod DMACHN {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (3 bits: 0b111 << 25)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MAC VLAN Hash Table
pub mod MAC_VLAN_HASH_TABLE {

    /// VLAN Hash Table This field contains the 16-bit VLAN Hash Table.
    pub mod VLHT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// VLAN Tag Inclusion or Replacement
pub mod MAC_VLAN_INCL {

    /// VLAN Tag for Transmit Packets
    pub mod VLT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VLAN Tag Control in Transmit Packets - 2'b00: No VLAN tag deletion, insertion, or replacement - 2'b01: VLAN tag deletion The MAC removes the VLAN type (bytes 13 and 14) and VLAN tag (bytes 15 and 16) of all transmitted packets with VLAN tags.
    pub mod VLC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No VLAN tag deletion, insertion, or replacement
            pub const NONE: u32 = 0b00;

            /// 0b01: VLAN tag deletion
            pub const DELETE: u32 = 0b01;

            /// 0b10: VLAN tag insertion
            pub const INSERT: u32 = 0b10;

            /// 0b11: VLAN tag replacement
            pub const REPLACE: u32 = 0b11;
        }
    }

    /// VLAN Priority Control
    pub mod VLP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VLAN Priority Control is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: VLAN Priority Control is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// C-VLAN or S-VLAN
    pub mod CSVL {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: C-VLAN type (0x8100) is inserted or replaced
            pub const C_VLAN: u32 = 0b0;

            /// 0b1: S-VLAN type (0x88A8) is inserted or replaced
            pub const S_VLAN: u32 = 0b1;
        }
    }

    /// VLAN Tag Input When this bit is set, it indicates that the VLAN tag to be inserted or replaced in Tx packet should be taken from: - The Tx descriptor
    pub mod VLTI {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VLAN Tag Input is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: VLAN Tag Input is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Channel based tag insertion
    pub mod CBTI {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Channel based tag insertion is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Channel based tag insertion is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Address
    pub mod ADDR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read write control
    pub mod RDWR {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Read operation of indirect access
            pub const READ: u32 = 0b0;

            /// 0b1: Write operation of indirect access
            pub const WRITE: u32 = 0b1;
        }
    }

    /// Busy
    pub mod BUSY {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Busy status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Busy status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// MAC Inner VLAN Tag Inclusion or Replacement
pub mod MAC_INNER_VLAN_INCL {

    /// VLAN Tag for Transmit Packets
    pub mod VLT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VLAN Tag Control in Transmit Packets
    pub mod VLC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No VLAN tag deletion, insertion, or replacement
            pub const NONE: u32 = 0b00;

            /// 0b01: VLAN tag deletion
            pub const DELETE: u32 = 0b01;

            /// 0b10: VLAN tag insertion
            pub const INSERT: u32 = 0b10;

            /// 0b11: VLAN tag replacement
            pub const REPLACE: u32 = 0b11;
        }
    }

    /// VLAN Priority Control
    pub mod VLP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VLAN Priority Control is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: VLAN Priority Control is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// C-VLAN or S-VLAN
    pub mod CSVL {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: C-VLAN type (0x8100) is inserted
            pub const C_VLAN: u32 = 0b0;

            /// 0b1: S-VLAN type (0x88A8) is inserted
            pub const S_VLAN: u32 = 0b1;
        }
    }

    /// VLAN Tag Input When this bit is set, it indicates that the VLAN tag to be inserted or replaced in Tx packet should be taken from: - The Tx descriptor
    pub mod VLTI {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VLAN Tag Input is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: VLAN Tag Input is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// MAC Q0 Tx Flow Control
pub mod MAC_Q0_TX_FLOW_CTRL {

    /// Flow Control Busy or Backpressure Activate
    pub mod FCB_BPA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Flow Control Busy or Backpressure Activate is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Flow Control Busy or Backpressure Activate is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Transmit Flow Control Enable
    pub mod TFE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Flow Control is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmit Flow Control is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Pause Low Threshold
    pub mod PLT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Pause Time minus 4 Slot Times (PT -4 slot times)
            pub const PT4: u32 = 0b000;

            /// 0b001: Pause Time minus 28 Slot Times (PT -28 slot times)
            pub const PT28: u32 = 0b001;

            /// 0b010: Pause Time minus 36 Slot Times (PT -36 slot times)
            pub const PT36: u32 = 0b010;

            /// 0b011: Pause Time minus 144 Slot Times (PT -144 slot times)
            pub const PT144: u32 = 0b011;

            /// 0b100: Pause Time minus 256 Slot Times (PT -256 slot times)
            pub const PT256: u32 = 0b100;

            /// 0b101: Pause Time minus 512 Slot Times (PT -512 slot times)
            pub const PT512: u32 = 0b101;
        }
    }

    /// Disable Zero-Quanta Pause
    pub mod DZPQ {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Zero-Quanta Pause packet generation is enabled
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Zero-Quanta Pause packet generation is disabled
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Pause Time
    pub mod PT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MAC Q1 Tx Flow Control
pub mod MAC_Q1_TX_FLOW_CTRL {
    pub use super::MAC_Q0_TX_FLOW_CTRL::DZPQ;
    pub use super::MAC_Q0_TX_FLOW_CTRL::FCB_BPA;
    pub use super::MAC_Q0_TX_FLOW_CTRL::PLT;
    pub use super::MAC_Q0_TX_FLOW_CTRL::PT;
    pub use super::MAC_Q0_TX_FLOW_CTRL::TFE;
}

/// MAC Q2 Tx Flow Control
pub mod MAC_Q2_TX_FLOW_CTRL {
    pub use super::MAC_Q0_TX_FLOW_CTRL::DZPQ;
    pub use super::MAC_Q0_TX_FLOW_CTRL::FCB_BPA;
    pub use super::MAC_Q0_TX_FLOW_CTRL::PLT;
    pub use super::MAC_Q0_TX_FLOW_CTRL::PT;
    pub use super::MAC_Q0_TX_FLOW_CTRL::TFE;
}

/// MAC Q3 Tx Flow Control
pub mod MAC_Q3_TX_FLOW_CTRL {
    pub use super::MAC_Q0_TX_FLOW_CTRL::DZPQ;
    pub use super::MAC_Q0_TX_FLOW_CTRL::FCB_BPA;
    pub use super::MAC_Q0_TX_FLOW_CTRL::PLT;
    pub use super::MAC_Q0_TX_FLOW_CTRL::PT;
    pub use super::MAC_Q0_TX_FLOW_CTRL::TFE;
}

/// MAC Q4 Tx Flow Control
pub mod MAC_Q4_TX_FLOW_CTRL {
    pub use super::MAC_Q0_TX_FLOW_CTRL::DZPQ;
    pub use super::MAC_Q0_TX_FLOW_CTRL::FCB_BPA;
    pub use super::MAC_Q0_TX_FLOW_CTRL::PLT;
    pub use super::MAC_Q0_TX_FLOW_CTRL::PT;
    pub use super::MAC_Q0_TX_FLOW_CTRL::TFE;
}

/// MAC Rx Flow Control
pub mod MAC_RX_FLOW_CTRL {

    /// Receive Flow Control Enable
    pub mod RFE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Flow Control is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive Flow Control is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Unicast Pause Packet Detect
    pub mod UP {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Unicast Pause Packet Detect disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Unicast Pause Packet Detect enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Priority Based Flow Control Enable
    pub mod PFCE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Priority Based Flow Control is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Priority Based Flow Control is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Receive Queue Control 4
pub mod MAC_RXQ_CTRL4 {

    /// Unicast Address Filter Fail Packets Queuing Enable.
    pub mod UFFQE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Unicast Address Filter Fail Packets Queuing is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Unicast Address Filter Fail Packets Queuing is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Unicast Address Filter Fail Packets Queue.
    pub mod UFFQ {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Multicast Address Filter Fail Packets Queuing Enable.
    pub mod MFFQE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Multicast Address Filter Fail Packets Queuing is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Multicast Address Filter Fail Packets Queuing is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Multicast Address Filter Fail Packets Queue.
    pub mod MFFQ {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VLAN Tag Filter Fail Packets Queuing Enable
    pub mod VFFQE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VLAN tag Filter Fail Packets Queuing is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: VLAN tag Filter Fail Packets Queuing is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// VLAN Tag Filter Fail Packets Queue
    pub mod VFFQ {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (3 bits: 0b111 << 17)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmit Queue Priority Mapping 0
pub mod MAC_TXQ_PRTY_MAP0 {

    /// Priorities Selected in Transmit Queue 0
    pub mod PSTQ0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Priorities Selected in Transmit Queue 1 This bit is similar to the PSTQ0 bit.
    pub mod PSTQ1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Priorities Selected in Transmit Queue 2 This bit is similar to the PSTQ0 bit.
    pub mod PSTQ2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Priorities Selected in Transmit Queue 3 This bit is similar to the PSTQ0 bit.
    pub mod PSTQ3 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmit Queue Priority Mapping 1
pub mod MAC_TXQ_PRTY_MAP1 {

    /// Priorities Selected in Transmit Queue 4
    pub mod PSTQ4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Receive Queue Control 0
pub mod MAC_RXQ_CTRL0 {

    /// Receive Queue 0 Enable This field indicates whether Rx Queue 0 is enabled for AV or DCB.
    pub mod RXQ0EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Queue not enabled
            pub const DISABLE: u32 = 0b00;

            /// 0b01: Queue enabled for AV
            pub const EN_AV: u32 = 0b01;

            /// 0b10: Queue enabled for DCB/Generic
            pub const EN_DCB_GEN: u32 = 0b10;
        }
    }

    /// Receive Queue 1 Enable This field is similar to the RXQ0EN field.
    pub mod RXQ1EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RXQ0EN::RW;
    }

    /// Receive Queue 2 Enable This field is similar to the RXQ0EN field.
    pub mod RXQ2EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RXQ0EN::RW;
    }

    /// Receive Queue 3 Enable This field is similar to the RXQ0EN field.
    pub mod RXQ3EN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RXQ0EN::RW;
    }

    /// Receive Queue 4 Enable This field is similar to the RXQ0EN field.
    pub mod RXQ4EN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RXQ0EN::RW;
    }
}

/// Receive Queue Control 1
pub mod MAC_RXQ_CTRL1 {

    /// AV Untagged Control Packets Queue
    pub mod AVCPQ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Receive Queue 0
            pub const QUEUE0: u32 = 0b000;

            /// 0b001: Receive Queue 1
            pub const QUEUE1: u32 = 0b001;

            /// 0b010: Receive Queue 2
            pub const QUEUE2: u32 = 0b010;

            /// 0b011: Receive Queue 3
            pub const QUEUE3: u32 = 0b011;

            /// 0b100: Receive Queue 4
            pub const QUEUE4: u32 = 0b100;
        }
    }

    /// PTP Packets Queue
    pub mod PTPQ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AVCPQ::RW;
    }

    /// DCB Control Packets Queue
    pub mod DCBCPQ {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AVCPQ::RW;
    }

    /// Untagged Packet Queue
    pub mod UPQ {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AVCPQ::RW;
    }

    /// Multicast and Broadcast Queue
    pub mod MCBCQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AVCPQ::RW;
    }

    /// Multicast and Broadcast Queue Enable This bit specifies that Multicast or Broadcast packets routing to the Rx Queue is enabled and the Multicast or Broadcast packets must be routed to Rx Queue specified in MCBCQ field.
    pub mod MCBCQEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Multicast and Broadcast Queue is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Multicast and Broadcast Queue is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Tagged AV Control Packets Queuing Enable.
    pub mod TACPQE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Tagged AV Control Packets Queuing is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Tagged AV Control Packets Queuing is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Tagged PTP over Ethernet Packets Queuing Control.
    pub mod TPQC {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Frame Preemption Residue Queue
    pub mod FPRQ {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Receive Queue Control 2
pub mod MAC_RXQ_CTRL2 {

    /// Priorities Selected in the Receive Queue 0
    pub mod PSRQ0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Priorities Selected in the Receive Queue 1
    pub mod PSRQ1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Priorities Selected in the Receive Queue 2
    pub mod PSRQ2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Priorities Selected in the Receive Queue 3
    pub mod PSRQ3 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Receive Queue Control 3
pub mod MAC_RXQ_CTRL3 {

    /// Priorities Selected in the Receive Queue 4
    pub mod PSRQ4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Status
pub mod MAC_INTERRUPT_STATUS {

    /// RGMII or SMII Interrupt Status
    pub mod RGSMIIIS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: RGMII or SMII Interrupt Status is not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: RGMII or SMII Interrupt Status is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// PHY Interrupt
    pub mod PHYIS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PHY Interrupt not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: PHY Interrupt detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// PMT Interrupt Status
    pub mod PMTIS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PMT Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: PMT Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// LPI Interrupt Status
    pub mod LPIIS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LPI Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: LPI Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Interrupt Status
    pub mod MMCIS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Interrupt Status
    pub mod MMCRXIS {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Interrupt Status
    pub mod MMCTXIS {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Checksum Offload Interrupt Status
    pub mod MMCRXIPIS {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Checksum Offload Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Checksum Offload Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Timestamp Interrupt Status
    pub mod TSIS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Timestamp Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Transmit Status Interrupt
    pub mod TXSTSIS {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Receive Status Interrupt
    pub mod RXSTSIS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Receive Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Frame Preemption Interrupt Status
    pub mod FPEIS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Frame Preemption Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Frame Preemption Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MDIO Interrupt Status
    pub mod MDIOIS {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MDIO Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MDIO Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC FPE Transmit Interrupt Status
    pub mod MFTIS {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC FPE Transmit Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC FPE Transmit Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC FPE Receive Interrupt Status
    pub mod MFRIS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC FPE Receive Interrupt status not active
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC FPE Receive Interrupt status active
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// Interrupt Enable
pub mod MAC_INTERRUPT_ENABLE {

    /// RGMII or SMII Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of RGSMIIIS bit in MAC_INTERRUPT_STATUS register.
    pub mod RGSMIIIE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: RGMII or SMII Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: RGMII or SMII Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// PHY Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of MAC_INTERRUPT_STATUS\[PHYIS\].
    pub mod PHYIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PHY Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: PHY Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// PMT Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of MAC_INTERRUPT_STATUS\[PMTIS\].
    pub mod PMTIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PMT Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: PMT Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// LPI Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of MAC_INTERRUPT_STATUS\[LPIIS\].
    pub mod LPIIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LPI Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: LPI Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Timestamp Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of MAC_INTERRUPT_STATUS\[TSIS\].
    pub mod TSIE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Timestamp Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Transmit Status Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of MAC_INTERRUPT_STATUS\[TXSTSIS\].
    pub mod TXSTSIE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp Status Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Timestamp Status Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Receive Status Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of MAC_INTERRUPT_STATUS\[RXSTSIS\].
    pub mod RXSTSIE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Status Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive Status Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Frame Preemption Interrupt Enable When this bit is set, it enables the assertion of the interrupt when FPEIS field is set in the MAC_INTERRUPT_STATUS.
    pub mod FPEIE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Frame Preemption Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Frame Preemption Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MDIO Interrupt Enable When this bit is set, it enables the assertion of the interrupt when MDIOIS field is set in the MAC_INTERRUPT_STATUS register.
    pub mod MDIOIE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MDIO Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MDIO Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Receive Transmit Status
pub mod MAC_RX_TX_STATUS {

    /// Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired which happens when the packet size exceeds 2,048 bytes (10,240 bytes when the Jumbo packet is enabled) and JD bit is reset in the MAC_CONFIGURATION register.
    pub mod TJT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Transmit Jabber Timeout
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit Jabber Timeout occurred
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// No Carrier When the DTXSTS bit is set in the MAC_OPERATION_MODE register, this bit indicates that the carrier signal from the PHY is not present at the end of preamble transmission.
    pub mod NCARR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Carrier is present
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: No carrier
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Loss of Carrier When the DTXSTS bit is set in the MAC_OPERATION_MODE register, this bit indicates that the loss of carrier occurred during packet transmission, that is, the phy_crs_i signal was inactive for one or more transmission clock periods during packet transmission.
    pub mod LCARR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Carrier is present
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Loss of carrier
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Excessive Deferral When the DTXSTS bit is set in the MAC_OPERATION_MODE register and the DC bit is set in the MAC_CONFIGURATION register, this bit indicates that the transmission ended because of excessive deferral of over 24,288 bit times (155,680 in 1000/2500 Mbps mode or when Jumbo packet is enabled).
    pub mod EXDEF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Excessive deferral
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Excessive deferral
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Late Collision When the DTXSTS bit is set in the MAC_OPERATION_MODE register, this bit indicates that the packet transmission aborted because a collision occurred after the collision window (64 bytes including Preamble in MII mode; 512 bytes including Preamble and Carrier Extension in GMII mode).
    pub mod LCOL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No collision
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Late collision is sensed
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Excessive Collisions When the DTXSTS bit is set in the MAC_OPERATION_MODE register, this bit indicates that the transmission aborted after 16 successive collisions while attempting to transmit the current packet.
    pub mod EXCOL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No collision
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Excessive collision is sensed
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Receive Watchdog Timeout This bit is set when a packet with length greater than 2,048 bytes is received (10, 240 bytes when Jumbo Packet mode is enabled) and the WD bit is reset in the MAC_CONFIGURATION register.
    pub mod RWT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No receive watchdog timeout
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Receive watchdog timed out
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// PMT Control and Status
pub mod MAC_PMT_CONTROL_STATUS {

    /// Power Down When this bit is set, the MAC receiver drops all received packets until it receives the expected magic packet or remote wake-up packet.
    pub mod PWRDWN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Power down is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Power down is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Magic Packet Enable When this bit is set, a power management event is generated when the MAC receives a magic packet.
    pub mod MGKPKTEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Magic Packet is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Magic Packet is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Remote Wake-Up Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wake-up packet.
    pub mod RWKPKTEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Remote wake-up packet is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Remote wake-up packet is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Magic Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a magic packet.
    pub mod MGKPRCVD {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Magic packet is received
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Magic packet is received
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Remote Wake-Up Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a remote wake-up packet.
    pub mod RWKPRCVD {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Remote wake-up packet is received
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Remote wake-up packet is received
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet.
    pub mod GLBLUCAST {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Global unicast is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Global unicast is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected Wake-up frame.
    pub mod RWKPFE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Remote Wake-up Packet Forwarding is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Remote Wake-up Packet Forwarding is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Remote Wake-up FIFO Pointer This field gives the current value (0 to 7, 15, or 31 when 4, 8, or 16 Remote Wake-up Packet Filters are selected) of the Remote Wake-up Packet Filter register pointer.
    pub mod RWKPTR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000.
    pub mod RWKFILTRST {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Remote Wake-Up Packet Filter Register Pointer is not Reset
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Remote Wake-Up Packet Filter Register Pointer is Reset
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Remote Wakeup Filter
pub mod MAC_RWK_PACKET_FILTER {

    /// RWK Packet Filter This field contains the various controls of RWK Packet filter.
    pub mod WKUPFRMFTR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LPI Control and Status
pub mod MAC_LPI_CONTROL_STATUS {

    /// Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit.
    pub mod TLPIEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit LPI entry not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit LPI entry detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Transmit LPI Exit When this bit is set, it indicates that the MAC transmitter exited the LPI state after the application cleared the LPIEN bit and the LPI TW Timer has expired.
    pub mod TLPIEX {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit LPI exit not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit LPI exit detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Receive LPI Entry When this bit is set, it indicates that the MAC Receiver has received an LPI pattern and entered the LPI state.
    pub mod RLPIEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive LPI entry not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Receive LPI entry detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Receive LPI Exit When this bit is set, it indicates that the MAC Receiver has stopped receiving the LPI pattern on the GMII or MII interface, exited the LPI state, and resumed the normal reception.
    pub mod RLPIEX {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive LPI exit not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Receive LPI exit detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Transmit LPI State When this bit is set, it indicates that the MAC is transmitting the LPI pattern on the GMII or MII interface.
    pub mod TLPIST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit LPI state not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit LPI state detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Receive LPI State When this bit is set, it indicates that the MAC is receiving the LPI pattern on the GMII or MII interface.
    pub mod RLPIST {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive LPI state not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Receive LPI state detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state.
    pub mod LPIEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LPI state is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: LPI state is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// PHY Link Status This bit indicates the link status of the PHY.
    pub mod PLS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: link is down
            pub const DISABLE: u32 = 0b0;

            /// 0b1: link is okay (UP)
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// PHY Link Status Enable This bit enables the link status received on the RGMII, SGMII, or SMII Receive paths to be used for activating the LPI LS TIMER.
    pub mod PLSEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PHY Link Status is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: PHY Link Status is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side.
    pub mod LPITXA {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LPI Tx Automate is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: LPI Tx Automate is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state.
    pub mod LPIATE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LPI Timer is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: LPI Timer is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// LPI Tx Clock Stop Enable When this bit is set, the MAC asserts sbd_tx_clk_gating_ctrl_o signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped.
    pub mod LPITCSE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LPI Tx Clock Stop is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: LPI Tx Clock Stop is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// LPI Timers Control
pub mod MAC_LPI_TIMERS_CONTROL {

    /// LPI TW Timer This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission.
    pub mod TWT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPI LS Timer This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY.
    pub mod LST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (10 bits: 0x3ff << 16)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx LPI Entry Timer Control
pub mod MAC_LPI_ENTRY_TIMER {

    /// LPI Entry Timer This field specifies the time in microseconds the MAC waits to enter LPI mode, after it has transmitted all the frames.
    pub mod LPIET {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (17 bits: 0x1ffff << 3)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// One-microsecond Reference Timer
pub mod MAC_ONEUS_TIC_COUNTER {

    /// 1US TIC Counter The application must program this counter so that the number of clock cycles of CSR clock is 1us.
    pub mod TIC_1US_CNTR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PHY Interface Control and Status
pub mod MAC_PHYIF_CONTROL_STATUS {

    /// Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port.
    pub mod TC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable Transmit Configuration in RGMII, SGMII, or SMII
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable Transmit Configuration in RGMII, SGMII, or SMII
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Link Up or Down This bit indicates whether the link is up or down during transmission of configuration in the RGMII, SGMII, or SMII interface.
    pub mod LUD {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Link down
            pub const LINKDOWN: u32 = 0b0;

            /// 0b1: Link up
            pub const LINKUP: u32 = 0b1;
        }
    }

    /// Link Mode This bit indicates the current mode of operation of the link.
    pub mod LNKMOD {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Half-duplex mode
            pub const HDUPLX: u32 = 0b0;

            /// 0b1: Full-duplex mode
            pub const FDUPLX: u32 = 0b1;
        }
    }

    /// Link Speed This bit indicates the current speed of the link.
    pub mod LNKSPEED {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (2 bits: 0b11 << 17)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 2.5 MHz
            pub const bf_2500K: u32 = 0b00;

            /// 0b01: 25 MHz
            pub const bf_25M: u32 = 0b01;

            /// 0b10: 125 MHz
            pub const bf_125M: u32 = 0b10;
        }
    }

    /// Link Status This bit indicates whether the link is up (1'b1) or down (1'b0).
    pub mod LNKSTS {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Link down
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Link up
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// MAC Version
pub mod MAC_VERSION {

    /// Synopsys-defined Version
    pub mod SNPSVER {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// User-defined Version (8'h10)
    pub mod USERVER {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MAC Debug
pub mod MAC_DEBUG {

    /// MAC GMII or MII Receive Protocol Engine Status When this bit is set, it indicates that the MAC GMII or MII receive protocol engine is actively receiving data, and it is not in the Idle state.
    pub mod RPESTS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MAC GMII or MII Receive Protocol Engine Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MAC GMII or MII Receive Protocol Engine Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MAC Receive Packet Controller FIFO Status When this bit is set, this field indicates the active state of the small FIFO Read and Write controllers of the MAC Receive Packet Controller module.
    pub mod RFCFCSTS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MAC GMII or MII Transmit Protocol Engine Status When this bit is set, it indicates that the MAC GMII or MII transmit protocol engine is actively transmitting data, and it is not in the Idle state.
    pub mod TPESTS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MAC GMII or MII Transmit Protocol Engine Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MAC GMII or MII Transmit Protocol Engine Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MAC Transmit Packet Controller Status This field indicates the state of the MAC Transmit Packet Controller module.
    pub mod TFCSTS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (2 bits: 0b11 << 17)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Idle state
            pub const IDLE: u32 = 0b00;

            /// 0b01: Waiting for one of the following: Status of the previous packet OR IPG or back off period to be over
            pub const WAITING: u32 = 0b01;

            /// 0b10: Generating and transmitting a Pause control packet (in full-duplex mode)
            pub const GEN_TX_PAU: u32 = 0b10;

            /// 0b11: Transferring input packet for transmission
            pub const TRNSFR: u32 = 0b11;
        }
    }
}

/// Optional Features or Functions 0
pub mod MAC_HW_FEATURE0 {

    /// 10 or 100 Mbps Support This bit is set to 1 when 10/100 Mbps is selected as the Mode of Operation
    pub mod MIISEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No 10 or 100 Mbps support
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: 10 or 100 Mbps support
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// 1000 Mbps Support This bit is set to 1 when 1000 Mbps is selected as the Mode of Operation
    pub mod GMIISEL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No 1000 Mbps support
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: 1000 Mbps support
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Half-duplex Support This bit is set to 1 when the half-duplex mode is selected
    pub mod HDSEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Half-duplex support
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Half-duplex support
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// PCS Registers (TBI, SGMII, or RTBI PHY interface) This bit is set to 1 when the TBI, SGMII, or RTBI PHY interface option is selected
    pub mod PCSSEL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No PCS Registers (TBI, SGMII, or RTBI PHY interface)
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: PCS Registers (TBI, SGMII, or RTBI PHY interface)
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// VLAN Hash Filter Selected This bit is set to 1 when the Enable VLAN Hash Table Based Filtering option is selected
    pub mod VLHASH {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VLAN Hash Filter not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: VLAN Hash Filter selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// SMA (MDIO) Interface This bit is set to 1 when the Enable Station Management (MDIO Interface) option is selected
    pub mod SMASEL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SMA (MDIO) Interface not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: SMA (MDIO) Interface selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// PMT Remote Wake-up Packet Enable This bit is set to 1 when the Enable Remote Wake-Up Packet Detection option is selected
    pub mod RWKSEL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PMT Remote Wake-up Packet Enable option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: PMT Remote Wake-up Packet Enable option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// PMT Magic Packet Enable This bit is set to 1 when the Enable Magic Packet Detection option is selected
    pub mod MGKSEL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PMT Magic Packet Enable option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: PMT Magic Packet Enable option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// RMON Module Enable This bit is set to 1 when the Enable MAC Management Counters (MMC) option is selected
    pub mod MMCSEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: RMON Module Enable option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: RMON Module Enable option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// ARP Offload Enabled This bit is set to 1 when the Enable IPv4 ARP Offload option is selected
    pub mod ARPOFFSEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ARP Offload Enable option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: ARP Offload Enable option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// IEEE 1588-2008 Timestamp Enabled This bit is set to 1 when the Enable IEEE 1588 Timestamp Support option is selected
    pub mod TSSEL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: IEEE 1588-2008 Timestamp Enable option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: IEEE 1588-2008 Timestamp Enable option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Energy Efficient Ethernet Enabled This bit is set to 1 when the Enable Energy Efficient Ethernet (EEE) option is selected
    pub mod EEESEL {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Energy Efficient Ethernet Enable option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Energy Efficient Ethernet Enable option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Transmit Checksum Offload Enabled This bit is set to 1 when the Enable Transmit TCP/IP Checksum Insertion option is selected
    pub mod TXCOESEL {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Checksum Offload Enable option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit Checksum Offload Enable option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Receive Checksum Offload Enabled This bit is set to 1 when the Enable Receive TCP/IP Checksum Check option is selected
    pub mod RXCOESEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Checksum Offload Enable option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Receive Checksum Offload Enable option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MAC Addresses 1-31 Selected This bit is set to 1 when the non-zero value is selected for Enable Additional 1-31 MAC Address Registers option
    pub mod ADDMACADRSEL {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (5 bits: 0b11111 << 18)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MAC Addresses 32-63 Selected This bit is set to 1 when the Enable Additional 32 MAC Address Registers (32-63) option is selected
    pub mod MACADR32SEL {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MAC Addresses 32-63 Select option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MAC Addresses 32-63 Select option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MAC Addresses 64-127 Selected This bit is set to 1 when the Enable Additional 64 MAC Address Registers (64-127) option is selected
    pub mod MACADR64SEL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MAC Addresses 64-127 Select option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MAC Addresses 64-127 Select option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Timestamp System Time Source This bit indicates the source of the Timestamp system time: This bit is set to 1 when the Enable IEEE 1588 Timestamp Support option is selected
    pub mod TSSTSSEL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (2 bits: 0b11 << 25)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Internal
            pub const INTRNL: u32 = 0b00;

            /// 0b01: External
            pub const EXTRNL: u32 = 0b01;

            /// 0b10: Both
            pub const BOTH: u32 = 0b10;
        }
    }

    /// Source Address or VLAN Insertion Enable This bit is set to 1 when the Enable SA and VLAN Insertion on Tx option is selected
    pub mod SAVLANINS {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Source Address or VLAN Insertion Enable option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Source Address or VLAN Insertion Enable option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Active PHY Selected When you have multiple PHY interfaces in your configuration, this field indicates the sampled value of phy_intf_sel_i during reset de-assertion.
    pub mod ACTPHYSEL {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (3 bits: 0b111 << 28)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: GMII or MII
            pub const GMII_MII: u32 = 0b000;

            /// 0b001: RGMII
            pub const RGMII: u32 = 0b001;

            /// 0b010: SGMII
            pub const SGMII: u32 = 0b010;

            /// 0b011: TBI
            pub const TBI: u32 = 0b011;

            /// 0b100: RMII
            pub const RMII: u32 = 0b100;

            /// 0b101: RTBI
            pub const RTBI: u32 = 0b101;

            /// 0b110: SMII
            pub const SMII: u32 = 0b110;

            /// 0b111: RevMII
            pub const REVMIII: u32 = 0b111;
        }
    }
}

/// Optional Features or Functions 1
pub mod MAC_HW_FEATURE1 {

    /// MTL Receive FIFO Size This field contains the configured value of MTL Rx FIFO in bytes expressed as Log to base 2 minus 7, that is, Log2(RXFIFO_SIZE) -7:
    pub mod RXFIFOSIZE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 128 bytes
            pub const bf_128B: u32 = 0b00000;

            /// 0b00001: 256 bytes
            pub const bf_256B: u32 = 0b00001;

            /// 0b00010: 512 bytes
            pub const bf_512B: u32 = 0b00010;

            /// 0b00011: 1024 bytes
            pub const bf_1024B: u32 = 0b00011;

            /// 0b00100: 2048 bytes
            pub const bf_2048B: u32 = 0b00100;

            /// 0b00101: 4096 bytes
            pub const bf_4096B: u32 = 0b00101;

            /// 0b00110: 8192 bytes
            pub const bf_8192B: u32 = 0b00110;

            /// 0b00111: 16384 bytes
            pub const bf_16384B: u32 = 0b00111;

            /// 0b01000: 32 KB
            pub const bf_32KB: u32 = 0b01000;

            /// 0b01001: 64 KB
            pub const bf_64KB: u32 = 0b01001;

            /// 0b01010: 128 KB
            pub const bf_128KB: u32 = 0b01010;

            /// 0b01011: 256 KB
            pub const bf_256KB: u32 = 0b01011;
        }
    }

    /// Single Port RAM Enable This bit is set to 1 when the Use single port RAM Feature is selected.
    pub mod SPRAM {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Single Port RAM feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Single Port RAM feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MTL Transmit FIFO Size This field contains the configured value of MTL Tx FIFO in bytes expressed as Log to base 2 minus 7, that is, Log2(TXFIFO_SIZE) -7:
    pub mod TXFIFOSIZE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 128 bytes
            pub const bf_128B: u32 = 0b00000;

            /// 0b00001: 256 bytes
            pub const bf_256B: u32 = 0b00001;

            /// 0b00010: 512 bytes
            pub const bf_512B: u32 = 0b00010;

            /// 0b00011: 1024 bytes
            pub const bf_1024B: u32 = 0b00011;

            /// 0b00100: 2048 bytes
            pub const bf_2048B: u32 = 0b00100;

            /// 0b00101: 4096 bytes
            pub const bf_4096B: u32 = 0b00101;

            /// 0b00110: 8192 bytes
            pub const bf_8192B: u32 = 0b00110;

            /// 0b00111: 16384 bytes
            pub const bf_16384B: u32 = 0b00111;

            /// 0b01000: 32 KB
            pub const bf_32KB: u32 = 0b01000;

            /// 0b01001: 64 KB
            pub const bf_64KB: u32 = 0b01001;

            /// 0b01010: 128 KB
            pub const bf_128KB: u32 = 0b01010;
        }
    }

    /// One-Step Timestamping Enable This bit is set to 1 when the Enable One-Step Timestamp Feature is selected.
    pub mod OSTEN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: One-Step Timestamping feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: One-Step Timestamping feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// PTP Offload Enable This bit is set to 1 when the Enable PTP Timestamp Offload Feature is selected.
    pub mod PTOEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PTP Offload feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: PTP Offload feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// IEEE 1588 High Word Register Enable This bit is set to 1 when the Add IEEE 1588 Higher Word Register option is selected
    pub mod ADVTHWORD {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: IEEE 1588 High Word Register option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: IEEE 1588 High Word Register option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Address Width.
    pub mod ADDR64 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 32
            pub const bf_32: u32 = 0b00;

            /// 0b01: 40
            pub const bf_40: u32 = 0b01;

            /// 0b10: 48
            pub const bf_48: u32 = 0b10;
        }
    }

    /// DCB Feature Enable This bit is set to 1 when the Enable Data Center Bridging option is selected
    pub mod DCBEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DCB Feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: DCB Feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Split Header Feature Enable This bit is set to 1 when the Enable Split Header Structure option is selected
    pub mod SPHEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Split Header Feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Split Header Feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// TCP Segmentation Offload Enable This bit is set to 1 when the Enable TCP Segmentation Offloading for TCP/IP Packets option is selected
    pub mod TSOEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TCP Segmentation Offload Feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: TCP Segmentation Offload Feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// DMA Debug Registers Enable This bit is set to 1 when the Debug Mode Enable option is selected
    pub mod DBGMEMA {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Debug Registers option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: DMA Debug Registers option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// AV Feature Enable This bit is set to 1 when the Enable Audio Video Bridging option is selected.
    pub mod AVSEL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: AV Feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: AV Feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Rx Side Only AV Feature Enable This bit is set to 1 when the Enable Audio Video Bridging option on Rx Side Only is selected.
    pub mod RAVSEL {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Rx Side Only AV Feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Rx Side Only AV Feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// One Step for PTP over UDP/IP Feature Enable This bit is set to 1 when the Enable One step timestamp for PTP over UDP/IP feature is selected.
    pub mod POUOST {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: One Step for PTP over UDP/IP Feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: One Step for PTP over UDP/IP Feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Hash Table Size This field indicates the size of the hash table:
    pub mod HASHTBLSZ {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No hash table
            pub const NO_HT: u32 = 0b00;

            /// 0b01: 64
            pub const bf_64: u32 = 0b01;

            /// 0b10: 128
            pub const bf_128: u32 = 0b10;

            /// 0b11: 256
            pub const bf_256: u32 = 0b11;
        }
    }

    /// Total number of L3 or L4 Filters This field indicates the total number of L3 or L4 filters:
    pub mod L3L4FNUM {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (4 bits: 0b1111 << 27)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No L3 or L4 Filter
            pub const NOFILT: u32 = 0b0000;

            /// 0b0001: 1 L3 or L4 Filter
            pub const bf_1FILT: u32 = 0b0001;

            /// 0b0010: 2 L3 or L4 Filters
            pub const bf_2FILT: u32 = 0b0010;

            /// 0b0011: 3 L3 or L4 Filters
            pub const bf_3FILT: u32 = 0b0011;

            /// 0b0100: 4 L3 or L4 Filters
            pub const bf_4FILT: u32 = 0b0100;

            /// 0b0101: 5 L3 or L4 Filters
            pub const bf_5FILT: u32 = 0b0101;

            /// 0b0110: 6 L3 or L4 Filters
            pub const bf_6FILT: u32 = 0b0110;

            /// 0b0111: 7 L3 or L4 Filters
            pub const bf_7FILT: u32 = 0b0111;

            /// 0b1000: 8 L3 or L4 Filters
            pub const bf_8FILT: u32 = 0b1000;
        }
    }
}

/// Optional Features or Functions 2
pub mod MAC_HW_FEATURE2 {

    /// Number of MTL Receive Queues This field indicates the number of MTL Receive queues:
    pub mod RXQCNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 1 MTL Rx Queue
            pub const bf_1RXQ: u32 = 0b0000;

            /// 0b0001: 2 MTL Rx Queues
            pub const bf_2RXQ: u32 = 0b0001;

            /// 0b0010: 3 MTL Rx Queues
            pub const bf_3RXQ: u32 = 0b0010;

            /// 0b0011: 4 MTL Rx Queues
            pub const bf_4RXQ: u32 = 0b0011;

            /// 0b0100: 5 MTL Rx Queues
            pub const bf_5RXQ: u32 = 0b0100;
        }
    }

    /// Number of MTL Transmit Queues This field indicates the number of MTL Transmit queues:
    pub mod TXQCNT {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (4 bits: 0b1111 << 6)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 1 MTL Tx Queue
            pub const bf_1TXQ: u32 = 0b0000;

            /// 0b0001: 2 MTL Tx Queues
            pub const bf_2TXQ: u32 = 0b0001;

            /// 0b0010: 3 MTL Tx Queues
            pub const bf_3TXQ: u32 = 0b0010;

            /// 0b0011: 4 MTL Tx Queues
            pub const bf_4TXQ: u32 = 0b0011;

            /// 0b0100: 5 MTL Tx Queues
            pub const bf_5TXQ: u32 = 0b0100;
        }
    }

    /// Number of DMA Receive Channels This field indicates the number of DMA Receive channels:
    pub mod RXCHCNT {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 1 MTL Rx Channel
            pub const bf_1RXCH: u32 = 0b0000;

            /// 0b0001: 2 MTL Rx Channels
            pub const bf_2RXCH: u32 = 0b0001;

            /// 0b0010: 3 MTL Rx Channels
            pub const bf_3RXCH: u32 = 0b0010;

            /// 0b0011: 4 MTL Rx Channels
            pub const bf_4RXCH: u32 = 0b0011;

            /// 0b0100: 5 MTL Rx Channels
            pub const bf_5RXCH: u32 = 0b0100;
        }
    }

    /// Number of DMA Transmit Channels This field indicates the number of DMA Transmit channels:
    pub mod TXCHCNT {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (4 bits: 0b1111 << 18)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 1 MTL Tx Channel
            pub const bf_1TXCH: u32 = 0b0000;

            /// 0b0001: 2 MTL Tx Channels
            pub const bf_2TXCH: u32 = 0b0001;

            /// 0b0010: 3 MTL Tx Channels
            pub const bf_3TXCH: u32 = 0b0010;

            /// 0b0011: 4 MTL Tx Channels
            pub const bf_4TXCH: u32 = 0b0011;

            /// 0b0100: 5 MTL Tx Channels
            pub const bf_5TXCH: u32 = 0b0100;
        }
    }

    /// Number of PPS Outputs This field indicates the number of PPS outputs:
    pub mod PPSOUTNUM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No PPS output
            pub const NO_PPSO: u32 = 0b000;

            /// 0b001: 1 PPS output
            pub const bf_1_PPSO: u32 = 0b001;

            /// 0b010: 2 PPS output
            pub const bf_2_PPSO: u32 = 0b010;

            /// 0b011: 3 PPS output
            pub const bf_3_PPSO: u32 = 0b011;

            /// 0b100: 4 PPS output
            pub const bf_4_PPSO: u32 = 0b100;
        }
    }

    /// Number of Auxiliary Snapshot Inputs This field indicates the number of auxiliary snapshot inputs:
    pub mod AUXSNAPNUM {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (3 bits: 0b111 << 28)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No auxiliary input
            pub const NO_AUXI: u32 = 0b000;

            /// 0b001: 1 auxiliary input
            pub const bf_1_AUXI: u32 = 0b001;

            /// 0b010: 2 auxiliary input
            pub const bf_2_AUXI: u32 = 0b010;

            /// 0b011: 3 auxiliary input
            pub const bf_3_AUXI: u32 = 0b011;

            /// 0b100: 4 auxiliary input
            pub const bf_4_AUXI: u32 = 0b100;
        }
    }
}

/// Optional Features or Functions 3
pub mod MAC_HW_FEATURE3 {

    /// Number of Extended VLAN Tag Filters Enabled This field indicates the Number of Extended VLAN Tag Filters selected:
    pub mod NRVF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No Extended Rx VLAN Filters
            pub const NO_ERVLAN: u32 = 0b000;

            /// 0b001: 4 Extended Rx VLAN Filters
            pub const bf_4_ERVLAN: u32 = 0b001;

            /// 0b010: 8 Extended Rx VLAN Filters
            pub const bf_8_ERVLAN: u32 = 0b010;

            /// 0b011: 16 Extended Rx VLAN Filters
            pub const bf_16_ERVLAN: u32 = 0b011;

            /// 0b100: 24 Extended Rx VLAN Filters
            pub const bf_24_ERVLAN: u32 = 0b100;

            /// 0b101: 32 Extended Rx VLAN Filters
            pub const bf_32_ERVLAN: u32 = 0b101;
        }
    }

    /// Queue/Channel based VLAN tag insertion on Tx Enable This bit is set to 1 when the Enable Queue/Channel based VLAN tag insertion on Tx Feature is selected.
    pub mod CBTISEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enable Queue/Channel based VLAN tag insertion on Tx feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Enable Queue/Channel based VLAN tag insertion on Tx feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Double VLAN Tag Processing Selected This bit is set to 1 when the Enable Double VLAN Processing Feature is selected.
    pub mod DVLAN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Double VLAN option is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Double VLAN option is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Broadcast/Multicast Packet Duplication This bit is set to 1 when the Broadcast/Multicast Packet Duplication feature is selected.
    pub mod PDUPSEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Broadcast/Multicast Packet Duplication feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Broadcast/Multicast Packet Duplication feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Flexible Receive Parser Selected This bit is set to 1 when the Enable Flexible Programmable Receive Parser option is selected.
    pub mod FRPSEL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Flexible Receive Parser feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Flexible Receive Parser feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Flexible Receive Parser Buffer size This field indicates the supported Max Number of bytes of the packet data to be Parsed by Flexible Receive Parser.
    pub mod FRPBS {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 64 Bytes
            pub const bf_64BYTES: u32 = 0b00;

            /// 0b01: 128 Bytes
            pub const bf_128BYTES: u32 = 0b01;

            /// 0b10: 256 Bytes
            pub const bf_256BYTES: u32 = 0b10;
        }
    }

    /// Flexible Receive Parser Table Entries size This field indicates the Max Number of Parser Entries supported by Flexible Receive Parser.
    pub mod FRPES {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 64 Entries
            pub const bf_64ENTR: u32 = 0b00;

            /// 0b01: 128 Entries
            pub const bf_128ENTR: u32 = 0b01;

            /// 0b10: 256 Entries
            pub const bf_256ENTR: u32 = 0b10;
        }
    }

    /// Enhancements to Scheduling Traffic Enable This bit is set to 1 when the Enable Enhancements to Scheduling Traffic feature is selected.
    pub mod ESTSEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enable Enhancements to Scheduling Traffic feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Enable Enhancements to Scheduling Traffic feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Depth of the Gate Control List This field indicates the depth of Gate Control list expressed as Log2(DWC_EQOS_EST_DEP)-5
    pub mod ESTDEP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (3 bits: 0b111 << 17)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No Depth configured
            pub const NODEPTH: u32 = 0b000;

            /// 0b001: 64
            pub const DEPTH64: u32 = 0b001;

            /// 0b010: 128
            pub const DEPTH128: u32 = 0b010;

            /// 0b011: 256
            pub const DEPTH256: u32 = 0b011;

            /// 0b100: 512
            pub const DEPTH512: u32 = 0b100;

            /// 0b101: 1024
            pub const DEPTH1024: u32 = 0b101;
        }
    }

    /// Width of the Time Interval field in the Gate Control List This field indicates the width of the Configured Time Interval Field
    pub mod ESTWID {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Width not configured
            pub const NOWIDTH: u32 = 0b00;

            /// 0b01: 16
            pub const WIDTH16: u32 = 0b01;

            /// 0b10: 20
            pub const WIDTH20: u32 = 0b10;

            /// 0b11: 24
            pub const WIDTH24: u32 = 0b11;
        }
    }

    /// Frame Preemption Enable This bit is set to 1 when the Enable Frame preemption feature is selected.
    pub mod FPESEL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Frame Preemption Enable feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Frame Preemption Enable feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Time Based Scheduling Enable This bit is set to 1 when the Time Based Scheduling feature is selected.
    pub mod TBSSEL {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Time Based Scheduling Enable feature is not selected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Time Based Scheduling Enable feature is selected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Automotive Safety Package Following are the encoding for the different Safety features
    pub mod ASP {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No Safety features selected
            pub const NONE: u32 = 0b00;

            /// 0b01: Only "ECC protection for external memory" feature is selected
            pub const ECC_ONLY: u32 = 0b01;

            /// 0b10: All the Automotive Safety features are selected without the "Parity Port Enable for external interface" feature
            pub const AS_NPPE: u32 = 0b10;

            /// 0b11: All the Automotive Safety features are selected with the "Parity Port Enable for external interface" feature
            pub const AS_PPE: u32 = 0b11;
        }
    }
}

/// MDIO Address
pub mod MAC_MDIO_ADDRESS {

    /// GMII Busy The application sets this bit to instruct the SMA to initiate a Read or Write access to the MDIO slave.
    pub mod GB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: GMII Busy is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: GMII Busy is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Clause 45 PHY Enable When this bit is set, Clause 45 capable PHY is connected to MDIO.
    pub mod C45E {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clause 45 PHY is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Clause 45 PHY is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// GMII Operation Command 0 This is the lower bit of the operation command to the PHY or RevMII.
    pub mod GOC_0 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: GMII Operation Command 0 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: GMII Operation Command 0 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// GMII Operation Command 1 This bit is higher bit of the operation command to the PHY or RevMII, GOC_1 and GOC_O is encoded as follows: - 00: Reserved - 01: Write - 10: Post Read Increment Address for Clause 45 PHY - 11: Read When Clause 22 PHY or RevMII is enabled, only Write and Read commands are valid.
    pub mod GOC_1 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: GMII Operation Command 1 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: GMII Operation Command 1 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Skip Address Packet When this bit is set, the SMA does not send the address packets before read, write, or post-read increment address packets.
    pub mod SKAP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Skip Address Packet is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Skip Address Packet is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design: - 0000: CSR clock = 60-100 MHz; MDC clock = CSR clock/42 - 0001: CSR clock = 100-150 MHz; MDC clock = CSR clock/62 - 0010: CSR clock = 20-35 MHz; MDC clock = CSR clock/16 - 0011: CSR clock = 35-60 MHz; MDC clock = CSR clock/26 - 0100: CSR clock = 150-250 MHz; MDC clock = CSR clock/102 - 0101: CSR clock = 250-300 MHz; MDC clock = CSR clock/124 - 0110: CSR clock = 300-500 MHz; MDC clock = CSR clock/204 - 0111: CSR clock = 500-800 MHz; MDC clock = CSR clock/324 The suggested range of CSR clock frequency applicable for each value (when Bit 11 = 0) ensures that the MDC clock is approximately between 1.
    pub mod CR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of Trailing Clocks This field controls the number of trailing clock cycles generated on gmii_mdc_o (MDC) after the end of transmission of MDIO frame.
    pub mod NTC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Register/Device Address These bits select the PHY register in selected Clause 22 PHY device.
    pub mod RDA {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Physical Layer Address This field indicates which Clause 22 PHY devices (out of 32 devices) the MAC is accessing.
    pub mod PA {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (5 bits: 0b11111 << 21)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC informs the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted).
    pub mod BTB {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Back to Back transactions disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Back to Back transactions enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Preamble Suppression Enable When this bit is set, the SMA suppresses the 32-bit preamble and transmits MDIO frames with only 1 preamble bit.
    pub mod PSE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Preamble Suppression disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Preamble Suppression enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// MAC MDIO Data
pub mod MAC_MDIO_DATA {

    /// GMII Data This field contains the 16-bit data value read from the PHY or RevMII after a Management Read operation or the 16-bit data value to be written to the PHY or RevMII before a Management Write operation.
    pub mod GD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Register Address This field is valid only when C45E is set.
    pub mod RA {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CSR Software Control
pub mod MAC_CSR_SW_CTRL {

    /// Register Clear on Write 1 Enable When this bit is set, the access mode of some register fields changes to Clear on Write 1, the application needs to set that respective bit to 1 to clear it.
    pub mod RCWE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Register Clear on Write 1 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Register Clear on Write 1 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Frame Preemption Control
pub mod MAC_FPE_CTRL_STS {

    /// Enable Tx Frame Preemption When set Frame Preemption Tx functionality is enabled.
    pub mod EFPE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Tx Frame Preemption is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Tx Frame Preemption is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Send Verify mPacket When set indicates hardware to send a verify mPacket.
    pub mod SVER {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Send Verify mPacket is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Send Verify mPacket is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Send Respond mPacket When set indicates hardware to send a Respond mPacket.
    pub mod SRSP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Send Respond mPacket is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Send Respond mPacket is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Synopsys Reserved, Must be set to "0".
    pub mod S1_SET_0 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Received Verify Frame Set when a Verify mPacket is received.
    pub mod RVER {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Not received Verify Frame
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Received Verify Frame
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Received Respond Frame Set when a Respond mPacket is received.
    pub mod RRSP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Not received Respond Frame
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Received Respond Frame
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Transmitted Verify Frame Set when a Verify mPacket is transmitted (triggered by setting SVER field).
    pub mod TVER {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Not transmitted Verify Frame
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: transmitted Verify Frame
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Transmitted Respond Frame Set when a Respond mPacket is transmitted (triggered by setting SRSP field).
    pub mod TRSP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Not transmitted Respond Frame
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: transmitted Respond Frame
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// 32-bit Binary Rollover Equivalent Time
pub mod MAC_PRESN_TIME_NS {

    /// MAC 1722 Presentation Time in ns These bits indicate the value of the 32-bit binary rollover equivalent time of the PTP System Time in ns
    pub mod MPTN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MAC 1722 Presentation Time
pub mod MAC_PRESN_TIME_UPDT {

    /// MAC 1722 Presentation Time Update This field holds the init value or the update value for the presentation time.
    pub mod MPTU {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MAC Address0 High
pub mod MAC_ADDRESS0_HIGH {

    /// MAC Address0\[47:32\] This field contains the upper 16 bits \[47:32\] of the first 6-byte MAC address.
    pub mod ADDRHI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address0 content is routed.
    pub mod DCS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Address Enable This bit is always set to 1.
    pub mod AE {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: INVALID : This bit must be always set to 1
            pub const DISABLE: u32 = 0b0;

            /// 0b1: This bit is always set to 1
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// MAC Address0 Low
pub mod MAC_ADDRESS0_LOW {

    /// MAC Address0\[31:0\] This field contains the lower 32 bits of the first 6-byte MAC address.
    pub mod ADDRLO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MAC Address1 High
pub mod MAC_ADDRESS1_HIGH {

    /// MAC ADDRESS1 \[47:32\] This field contains the upper 16 bits\[47:32\] of the second 6-byte MAC address.
    pub mod ADDRHI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Channel Select If the PDC bit of MAC_EXT_CONFIGURATION register is not set: This field contains the binary representation of the DMA Channel number to which an Rx packet whose DA matches the MAC Address(#i) content is routed.
    pub mod DCS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mask Byte Control These bits are mask control bits for comparing each of the MAC Address bytes.
    pub mod MBC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (6 bits: 0x3f << 24)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Source Address When this bit is set, the MAC ADDRESS1\[47:0\] is used to compare with the SA fields of the received packet.
    pub mod SA {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Compare with Destination Address
            pub const DA: u32 = 0b0;

            /// 0b1: Compare with Source Address
            pub const SA: u32 = 0b1;
        }
    }

    /// Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering.
    pub mod AE {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Address is ignored
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Address is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// MAC Address1 Low
pub mod MAC_ADDRESS1_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address2 High
pub mod MAC_ADDRESS2_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address2 Low
pub mod MAC_ADDRESS2_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address3 High
pub mod MAC_ADDRESS3_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address3 Low
pub mod MAC_ADDRESS3_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address4 High
pub mod MAC_ADDRESS4_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address4 Low
pub mod MAC_ADDRESS4_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address5 High
pub mod MAC_ADDRESS5_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address5 Low
pub mod MAC_ADDRESS5_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address6 High
pub mod MAC_ADDRESS6_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address6 Low
pub mod MAC_ADDRESS6_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address7 High
pub mod MAC_ADDRESS7_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address7 Low
pub mod MAC_ADDRESS7_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address8 High
pub mod MAC_ADDRESS8_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address8 Low
pub mod MAC_ADDRESS8_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address9 High
pub mod MAC_ADDRESS9_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address9 Low
pub mod MAC_ADDRESS9_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address10 High
pub mod MAC_ADDRESS10_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address10 Low
pub mod MAC_ADDRESS10_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address11 High
pub mod MAC_ADDRESS11_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address11 Low
pub mod MAC_ADDRESS11_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address12 High
pub mod MAC_ADDRESS12_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address12 Low
pub mod MAC_ADDRESS12_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address13 High
pub mod MAC_ADDRESS13_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address13 Low
pub mod MAC_ADDRESS13_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address14 High
pub mod MAC_ADDRESS14_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address14 Low
pub mod MAC_ADDRESS14_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address15 High
pub mod MAC_ADDRESS15_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address15 Low
pub mod MAC_ADDRESS15_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address16 High
pub mod MAC_ADDRESS16_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address16 Low
pub mod MAC_ADDRESS16_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address17 High
pub mod MAC_ADDRESS17_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address17 Low
pub mod MAC_ADDRESS17_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address18 High
pub mod MAC_ADDRESS18_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address18 Low
pub mod MAC_ADDRESS18_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address19 High
pub mod MAC_ADDRESS19_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address19 Low
pub mod MAC_ADDRESS19_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address20 High
pub mod MAC_ADDRESS20_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address20 Low
pub mod MAC_ADDRESS20_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address21 High
pub mod MAC_ADDRESS21_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address21 Low
pub mod MAC_ADDRESS21_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address22 High
pub mod MAC_ADDRESS22_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address22 Low
pub mod MAC_ADDRESS22_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address23 High
pub mod MAC_ADDRESS23_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address23 Low
pub mod MAC_ADDRESS23_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address24 High
pub mod MAC_ADDRESS24_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address24 Low
pub mod MAC_ADDRESS24_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address25 High
pub mod MAC_ADDRESS25_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address25 Low
pub mod MAC_ADDRESS25_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address26 High
pub mod MAC_ADDRESS26_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address26 Low
pub mod MAC_ADDRESS26_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address27 High
pub mod MAC_ADDRESS27_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address27 Low
pub mod MAC_ADDRESS27_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address28 High
pub mod MAC_ADDRESS28_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address28 Low
pub mod MAC_ADDRESS28_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address29 High
pub mod MAC_ADDRESS29_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address29 Low
pub mod MAC_ADDRESS29_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address30 High
pub mod MAC_ADDRESS30_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address30 Low
pub mod MAC_ADDRESS30_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address31 High
pub mod MAC_ADDRESS31_HIGH {
    pub use super::MAC_ADDRESS1_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS1_HIGH::AE;
    pub use super::MAC_ADDRESS1_HIGH::DCS;
    pub use super::MAC_ADDRESS1_HIGH::MBC;
    pub use super::MAC_ADDRESS1_HIGH::SA;
}

/// MAC Address31 Low
pub mod MAC_ADDRESS31_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address32 High
pub mod MAC_ADDRESS32_HIGH {

    /// MAC ADDRESS32 \[47:32\] This field contains the upper 16 bits (47:32) of the 33rd 6-byte MAC address.
    pub mod ADDRHI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Channel Select This field contains the DMA Channel number to which an Rx packet whose DA matches the MAC ADDRESS32 content is routed.
    pub mod DCS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering.
    pub mod AE {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Address is ignored
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Address is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// MAC Address32 Low
pub mod MAC_ADDRESS32_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address33 High
pub mod MAC_ADDRESS33_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address33 Low
pub mod MAC_ADDRESS33_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address34 High
pub mod MAC_ADDRESS34_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address34 Low
pub mod MAC_ADDRESS34_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address35 High
pub mod MAC_ADDRESS35_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address35 Low
pub mod MAC_ADDRESS35_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address36 High
pub mod MAC_ADDRESS36_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address36 Low
pub mod MAC_ADDRESS36_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address37 High
pub mod MAC_ADDRESS37_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address37 Low
pub mod MAC_ADDRESS37_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address38 High
pub mod MAC_ADDRESS38_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address38 Low
pub mod MAC_ADDRESS38_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address39 High
pub mod MAC_ADDRESS39_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address39 Low
pub mod MAC_ADDRESS39_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address40 High
pub mod MAC_ADDRESS40_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address40 Low
pub mod MAC_ADDRESS40_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address41 High
pub mod MAC_ADDRESS41_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address41 Low
pub mod MAC_ADDRESS41_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address42 High
pub mod MAC_ADDRESS42_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address42 Low
pub mod MAC_ADDRESS42_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address43 High
pub mod MAC_ADDRESS43_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address43 Low
pub mod MAC_ADDRESS43_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address44 High
pub mod MAC_ADDRESS44_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address44 Low
pub mod MAC_ADDRESS44_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address45 High
pub mod MAC_ADDRESS45_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address45 Low
pub mod MAC_ADDRESS45_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address46 High
pub mod MAC_ADDRESS46_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address46 Low
pub mod MAC_ADDRESS46_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address47 High
pub mod MAC_ADDRESS47_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address47 Low
pub mod MAC_ADDRESS47_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address48 High
pub mod MAC_ADDRESS48_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address48 Low
pub mod MAC_ADDRESS48_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address49 High
pub mod MAC_ADDRESS49_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address49 Low
pub mod MAC_ADDRESS49_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address50 High
pub mod MAC_ADDRESS50_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address50 Low
pub mod MAC_ADDRESS50_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address51 High
pub mod MAC_ADDRESS51_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address51 Low
pub mod MAC_ADDRESS51_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address52 High
pub mod MAC_ADDRESS52_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address52 Low
pub mod MAC_ADDRESS52_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address53 High
pub mod MAC_ADDRESS53_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address53 Low
pub mod MAC_ADDRESS53_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address54 High
pub mod MAC_ADDRESS54_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address54 Low
pub mod MAC_ADDRESS54_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address55 High
pub mod MAC_ADDRESS55_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address55 Low
pub mod MAC_ADDRESS55_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address56 High
pub mod MAC_ADDRESS56_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address56 Low
pub mod MAC_ADDRESS56_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address57 High
pub mod MAC_ADDRESS57_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address57 Low
pub mod MAC_ADDRESS57_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address58 High
pub mod MAC_ADDRESS58_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address58 Low
pub mod MAC_ADDRESS58_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address59 High
pub mod MAC_ADDRESS59_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address59 Low
pub mod MAC_ADDRESS59_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address60 High
pub mod MAC_ADDRESS60_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address60 Low
pub mod MAC_ADDRESS60_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address61 High
pub mod MAC_ADDRESS61_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address61 Low
pub mod MAC_ADDRESS61_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address62 High
pub mod MAC_ADDRESS62_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address62 Low
pub mod MAC_ADDRESS62_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MAC Address63 High
pub mod MAC_ADDRESS63_HIGH {
    pub use super::MAC_ADDRESS32_HIGH::ADDRHI;
    pub use super::MAC_ADDRESS32_HIGH::AE;
    pub use super::MAC_ADDRESS32_HIGH::DCS;
}

/// MAC Address63 Low
pub mod MAC_ADDRESS63_LOW {
    pub use super::MAC_ADDRESS0_LOW::ADDRLO;
}

/// MMC Control
pub mod MAC_MMC_CONTROL {

    /// Counters Reset When this bit is set, all counters are reset.
    pub mod CNTRST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Counters are not reset
            pub const DISABLE: u32 = 0b0;

            /// 0b1: All counters are reset
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Counter Stop Rollover When this bit is set, the counter does not roll over to zero after reaching the maximum value.
    pub mod CNTSTOPRO {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Counter Stop Rollover is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Counter Stop Rollover is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Reset on Read When this bit is set, the MMC counters are reset to zero after Read (self-clearing after reset).
    pub mod RSTONRD {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset on Read is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Reset on Read is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Counter Freeze When this bit is set, it freezes all MMC counters to their current value.
    pub mod CNTFREEZ {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Counter Freeze is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Counter Freeze is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Counters Preset When this bit is set, all counters are initialized or preset to almost full or almost half according to the CNTPRSTLVL bit.
    pub mod CNTPRST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Counters Preset is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Counters Preset is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Full-Half Preset When this bit is low and the CNTPRST bit is set, all MMC counters get preset to almost-half value.
    pub mod CNTPRSTLVL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Full-Half Preset is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Full-Half Preset is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Update MMC Counters for Dropped Broadcast Packets Note: The CNTRST bit has a higher priority than the CNTPRST bit.
    pub mod UCDBC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Update MMC Counters for Dropped Broadcast Packets is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Update MMC Counters for Dropped Broadcast Packets is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// MMC Rx Interrupt
pub mod MAC_MMC_RX_INTERRUPT {

    /// MMC Receive Good Bad Packet Counter Interrupt Status This bit is set when the rxpacketcount_gb counter reaches half of the maximum value or the maximum value.
    pub mod RXGBPKTIS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Good Bad Octet Counter Interrupt Status This bit is set when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value.
    pub mod RXGBOCTIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Good Bad Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Good Bad Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Good Octet Counter Interrupt Status This bit is set when the rxoctetcount_g counter reaches half of the maximum value or the maximum value.
    pub mod RXGOCTIS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Good Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Good Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Broadcast Good Packet Counter Interrupt Status This bit is set when the rxbroadcastpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod RXBCGPIS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Broadcast Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Broadcast Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Multicast Good Packet Counter Interrupt Status This bit is set when the rxmulticastpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod RXMCGPIS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Multicast Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Multicast Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive CRC Error Packet Counter Interrupt Status This bit is set when the rxcrcerror counter reaches half of the maximum value or the maximum value.
    pub mod RXCRCERPIS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive CRC Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive CRC Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Alignment Error Packet Counter Interrupt Status This bit is set when the rxalignmenterror counter reaches half of the maximum value or the maximum value.
    pub mod RXALGNERPIS {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Alignment Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Alignment Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Runt Packet Counter Interrupt Status This bit is set when the rxrunterror counter reaches half of the maximum value or the maximum value.
    pub mod RXRUNTPIS {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Runt Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Runt Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Jabber Error Packet Counter Interrupt Status This bit is set when the rxjabbererror counter reaches half of the maximum value or the maximum value.
    pub mod RXJABERPIS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Jabber Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Jabber Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Undersize Good Packet Counter Interrupt Status This bit is set when the rxundersize_g counter reaches half of the maximum value or the maximum value.
    pub mod RXUSIZEGPIS {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Undersize Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Undersize Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Oversize Good Packet Counter Interrupt Status This bit is set when the rxoversize_g counter reaches half of the maximum value or the maximum value.
    pub mod RXOSIZEGPIS {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Oversize Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Oversize Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive 64 Octet Good Bad Packet Counter Interrupt Status This bit is set when the rx64octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RX64OCTGBPIS {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive 64 Octet Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive 64 Octet Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive 65 to 127 Octet Good Bad Packet Counter Interrupt Status This bit is set when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RX65T127OCTGBPIS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive 65 to 127 Octet Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive 65 to 127 Octet Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive 128 to 255 Octet Good Bad Packet Counter Interrupt Status This bit is set when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RX128T255OCTGBPIS {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive 128 to 255 Octet Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive 128 to 255 Octet Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive 256 to 511 Octet Good Bad Packet Counter Interrupt Status This bit is set when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RX256T511OCTGBPIS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive 256 to 511 Octet Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive 256 to 511 Octet Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive 512 to 1023 Octet Good Bad Packet Counter Interrupt Status This bit is set when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RX512T1023OCTGBPIS {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive 512 to 1023 Octet Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive 512 to 1023 Octet Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive 1024 to Maximum Octet Good Bad Packet Counter Interrupt Status This bit is set when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RX1024TMAXOCTGBPIS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive 1024 to Maximum Octet Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive 1024 to Maximum Octet Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Unicast Good Packet Counter Interrupt Status This bit is set when the rxunicastpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod RXUCGPIS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Unicast Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Unicast Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Length Error Packet Counter Interrupt Status This bit is set when the rxlengtherror counter reaches half of the maximum value or the maximum value.
    pub mod RXLENERPIS {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Length Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Length Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Out Of Range Error Packet Counter Interrupt Status.
    pub mod RXORANGEPIS {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Out Of Range Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Out Of Range Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Pause Packet Counter Interrupt Status This bit is set when the rxpausepackets counter reaches half of the maximum value or the maximum value.
    pub mod RXPAUSPIS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Pause Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Pause Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive FIFO Overflow Packet Counter Interrupt Status This bit is set when the rxfifooverflow counter reaches half of the maximum value or the maximum value.
    pub mod RXFOVPIS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive FIFO Overflow Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive FIFO Overflow Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive VLAN Good Bad Packet Counter Interrupt Status This bit is set when the rxvlanpackets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RXVLANGBPIS {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive VLAN Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive VLAN Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Watchdog Error Packet Counter Interrupt Status This bit is set when the rxwatchdog error counter reaches half of the maximum value or the maximum value.
    pub mod RXWDOGPIS {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Watchdog Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Watchdog Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Error Packet Counter Interrupt Status This bit is set when the rxrcverror counter reaches half of the maximum value or the maximum value.
    pub mod RXRCVERRPIS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive Control Packet Counter Interrupt Status This bit is set when the rxctrlpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod RXCTRLPIS {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Control Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive Control Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive LPI microsecond counter interrupt status This bit is set when the Rx_LPI_USEC_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod RXLPIUSCIS {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive LPI microsecond Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive LPI microsecond Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive LPI transition counter interrupt status This bit is set when the Rx_LPI_Tran_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod RXLPITRCIS {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive LPI transition Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive LPI transition Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// MMC Tx Interrupt
pub mod MAC_MMC_TX_INTERRUPT {

    /// MMC Transmit Good Bad Octet Counter Interrupt Status This bit is set when the txoctetcount_gb counter reaches half of the maximum value or the maximum value.
    pub mod TXGBOCTIS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Good Bad Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Good Bad Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Good Bad Packet Counter Interrupt Status This bit is set when the txpacketcount_gb counter reaches half of the maximum value or the maximum value.
    pub mod TXGBPKTIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Broadcast Good Packet Counter Interrupt Status This bit is set when the txbroadcastpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod TXBCGPIS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Broadcast Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Broadcast Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Multicast Good Packet Counter Interrupt Status This bit is set when the txmulticastpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod TXMCGPIS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Multicast Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Multicast Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit 64 Octet Good Bad Packet Counter Interrupt Status This bit is set when the tx64octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TX64OCTGBPIS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit 64 Octet Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit 64 Octet Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit 65 to 127 Octet Good Bad Packet Counter Interrupt Status This bit is set when the tx65to127octets_gb counter reaches half the maximum value, and also when it reaches the maximum value.
    pub mod TX65T127OCTGBPIS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit 65 to 127 Octet Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit 65 to 127 Octet Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit 128 to 255 Octet Good Bad Packet Counter Interrupt Status This bit is set when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TX128T255OCTGBPIS {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit 128 to 255 Octet Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit 128 to 255 Octet Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit 256 to 511 Octet Good Bad Packet Counter Interrupt Status This bit is set when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TX256T511OCTGBPIS {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit 256 to 511 Octet Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit 256 to 511 Octet Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit 512 to 1023 Octet Good Bad Packet Counter Interrupt Status This bit is set when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TX512T1023OCTGBPIS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit 512 to 1023 Octet Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit 512 to 1023 Octet Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit 1024 to Maximum Octet Good Bad Packet Counter Interrupt Status This bit is set when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TX1024TMAXOCTGBPIS {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit 1024 to Maximum Octet Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit 1024 to Maximum Octet Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Unicast Good Bad Packet Counter Interrupt Status This bit is set when the txunicastpackets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TXUCGBPIS {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Unicast Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Unicast Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Multicast Good Bad Packet Counter Interrupt Status The bit is set when the txmulticastpackets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TXMCGBPIS {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Multicast Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Multicast Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Broadcast Good Bad Packet Counter Interrupt Status This bit is set when the txbroadcastpackets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TXBCGBPIS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Broadcast Good Bad Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Broadcast Good Bad Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Underflow Error Packet Counter Interrupt Status This bit is set when the txunderflowerror counter reaches half of the maximum value or the maximum value.
    pub mod TXUFLOWERPIS {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Underflow Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Underflow Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Single Collision Good Packet Counter Interrupt Status This bit is set when the txsinglecol_g counter reaches half of the maximum value or the maximum value.
    pub mod TXSCOLGPIS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Single Collision Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Single Collision Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Multiple Collision Good Packet Counter Interrupt Status This bit is set when the txmulticol_g counter reaches half of the maximum value or the maximum value.
    pub mod TXMCOLGPIS {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Multiple Collision Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Multiple Collision Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Deferred Packet Counter Interrupt Status This bit is set when the txdeferred counter reaches half of the maximum value or the maximum value.
    pub mod TXDEFPIS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Deferred Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Deferred Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Late Collision Packet Counter Interrupt Status This bit is set when the txlatecol counter reaches half of the maximum value or the maximum value.
    pub mod TXLATCOLPIS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Late Collision Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Late Collision Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Excessive Collision Packet Counter Interrupt Status This bit is set when the txexesscol counter reaches half of the maximum value or the maximum value.
    pub mod TXEXCOLPIS {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Excessive Collision Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Excessive Collision Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Carrier Error Packet Counter Interrupt Status This bit is set when the txcarriererror counter reaches half of the maximum value or the maximum value.
    pub mod TXCARERPIS {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Carrier Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Carrier Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Good Octet Counter Interrupt Status This bit is set when the txoctetcount_g counter reaches half of the maximum value or the maximum value.
    pub mod TXGOCTIS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Good Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Good Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Good Packet Counter Interrupt Status This bit is set when the txpacketcount_g counter reaches half of the maximum value or the maximum value.
    pub mod TXGPKTIS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Excessive Deferral Packet Counter Interrupt Status This bit is set when the txexcessdef counter reaches half of the maximum value or the maximum value.
    pub mod TXEXDEFPIS {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Excessive Deferral Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Excessive Deferral Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Pause Packet Counter Interrupt Status This bit is set when the txpausepacketserror counter reaches half of the maximum value or the maximum value.
    pub mod TXPAUSPIS {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Pause Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Pause Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit VLAN Good Packet Counter Interrupt Status This bit is set when the txvlanpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod TXVLANGPIS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit VLAN Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit VLAN Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit Oversize Good Packet Counter Interrupt Status This bit is set when the txoversize_g counter reaches half of the maximum value or the maximum value.
    pub mod TXOSIZEGPIS {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Oversize Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit Oversize Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit LPI microsecond counter interrupt status This bit is set when the Tx_LPI_USEC_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod TXLPIUSCIS {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit LPI microsecond Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit LPI microsecond Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Transmit LPI transition counter interrupt status This bit is set when the Tx_LPI_Tran_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod TXLPITRCIS {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit LPI transition Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Transmit LPI transition Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// MMC Rx Interrupt Mask
pub mod MAC_MMC_RX_INTERRUPT_MASK {

    /// MMC Receive Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxpacketcount_gb counter reaches half of the maximum value or the maximum value.
    pub mod RXGBPKTIM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Good Bad Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value.
    pub mod RXGBOCTIM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Good Bad Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Good Bad Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoctetcount_g counter reaches half of the maximum value or the maximum value.
    pub mod RXGOCTIM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Good Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Good Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Broadcast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxbroadcastpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod RXBCGPIM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Broadcast Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Broadcast Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Multicast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxmulticastpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod RXMCGPIM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Multicast Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Multicast Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive CRC Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxcrcerror counter reaches half of the maximum value or the maximum value.
    pub mod RXCRCERPIM {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive CRC Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive CRC Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Alignment Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxalignmenterror counter reaches half of the maximum value or the maximum value.
    pub mod RXALGNERPIM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Alignment Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Alignment Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Runt Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxrunterror counter reaches half of the maximum value or the maximum value.
    pub mod RXRUNTPIM {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Runt Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Runt Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Jabber Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxjabbererror counter reaches half of the maximum value or the maximum value.
    pub mod RXJABERPIM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Jabber Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Jabber Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Undersize Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxundersize_g counter reaches half of the maximum value or the maximum value.
    pub mod RXUSIZEGPIM {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Undersize Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Undersize Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Oversize Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoversize_g counter reaches half of the maximum value or the maximum value.
    pub mod RXOSIZEGPIM {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Oversize Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Oversize Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive 64 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rx64octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RX64OCTGBPIM {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive 64 Octet Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive 64 Octet Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive 65 to 127 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RX65T127OCTGBPIM {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive 65 to 127 Octet Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive 65 to 127 Octet Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive 128 to 255 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RX128T255OCTGBPIM {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive 128 to 255 Octet Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive 128 to 255 Octet Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive 256 to 511 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RX256T511OCTGBPIM {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive 256 to 511 Octet Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive 256 to 511 Octet Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive 512 to 1023 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RX512T1023OCTGBPIM {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive 512 to 1023 Octet Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive 512 to 1023 Octet Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive 1024 to Maximum Octet Good Bad Packet Counter Interrupt Mask.
    pub mod RX1024TMAXOCTGBPIM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive 1024 to Maximum Octet Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive 1024 to Maximum Octet Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Unicast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxunicastpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod RXUCGPIM {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Unicast Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Unicast Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Length Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxlengtherror counter reaches half of the maximum value or the maximum value.
    pub mod RXLENERPIM {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Length Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Length Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Out Of Range Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoutofrangetype counter reaches half of the maximum value or the maximum value.
    pub mod RXORANGEPIM {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Out Of Range Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Out Of Range Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Pause Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxpausepackets counter reaches half of the maximum value or the maximum value.
    pub mod RXPAUSPIM {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Pause Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Pause Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive FIFO Overflow Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxfifooverflow counter reaches half of the maximum value or the maximum value.
    pub mod RXFOVPIM {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive FIFO Overflow Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive FIFO Overflow Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive VLAN Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxvlanpackets_gb counter reaches half of the maximum value or the maximum value.
    pub mod RXVLANGBPIM {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive VLAN Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive VLAN Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Watchdog Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxwatchdog counter reaches half of the maximum value or the maximum value.
    pub mod RXWDOGPIM {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Watchdog Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Watchdog Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxrcverror counter reaches half of the maximum value or the maximum value.
    pub mod RXRCVERRPIM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive Control Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxctrlpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod RXCTRLPIM {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive Control Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive Control Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive LPI microsecond counter interrupt Mask Setting this bit masks the interrupt when the Rx_LPI_USEC_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod RXLPIUSCIM {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive LPI microsecond counter interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive LPI microsecond counter interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive LPI transition counter interrupt Mask Setting this bit masks the interrupt when the Rx_LPI_Tran_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod RXLPITRCIM {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive LPI transition counter interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive LPI transition counter interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// MMC Tx Interrupt Mask
pub mod MAC_MMC_TX_INTERRUPT_MASK {

    /// MMC Transmit Good Bad Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_gb counter reaches half of the maximum value or the maximum value.
    pub mod TXGBOCTIM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Good Bad Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Good Bad Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txpacketcount_gb counter reaches half of the maximum value or the maximum value.
    pub mod TXGBPKTIM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Broadcast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod TXBCGPIM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Broadcast Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Broadcast Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Multicast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod TXMCGPIM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Multicast Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Multicast Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit 64 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the tx64octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TX64OCTGBPIM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit 64 Octet Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit 64 Octet Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit 65 to 127 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the tx65to127octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TX65T127OCTGBPIM {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit 65 to 127 Octet Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit 65 to 127 Octet Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit 128 to 255 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TX128T255OCTGBPIM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit 128 to 255 Octet Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit 128 to 255 Octet Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit 256 to 511 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TX256T511OCTGBPIM {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit 256 to 511 Octet Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit 256 to 511 Octet Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit 512 to 1023 Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TX512T1023OCTGBPIM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit 512 to 1023 Octet Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit 512 to 1023 Octet Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit 1024 to Maximum Octet Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TX1024TMAXOCTGBPIM {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit 1024 to Maximum Octet Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit 1024 to Maximum Octet Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Unicast Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txunicastpackets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TXUCGBPIM {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Unicast Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Unicast Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Multicast Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastpackets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TXMCGBPIM {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Multicast Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Multicast Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Broadcast Good Bad Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastpackets_gb counter reaches half of the maximum value or the maximum value.
    pub mod TXBCGBPIM {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Broadcast Good Bad Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Broadcast Good Bad Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Underflow Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txunderflowerror counter reaches half of the maximum value or the maximum value.
    pub mod TXUFLOWERPIM {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Underflow Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Underflow Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Single Collision Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txsinglecol_g counter reaches half of the maximum value or the maximum value.
    pub mod TXSCOLGPIM {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Single Collision Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Single Collision Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticol_g counter reaches half of the maximum value or the maximum value.
    pub mod TXMCOLGPIM {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Deferred Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txdeferred counter reaches half of the maximum value or the maximum value.
    pub mod TXDEFPIM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Deferred Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Deferred Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Late Collision Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txlatecol counter reaches half of the maximum value or the maximum value.
    pub mod TXLATCOLPIM {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Late Collision Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Late Collision Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Excessive Collision Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txexcesscol counter reaches half of the maximum value or the maximum value.
    pub mod TXEXCOLPIM {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Excessive Collision Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Excessive Collision Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Carrier Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txcarriererror counter reaches half of the maximum value or the maximum value.
    pub mod TXCARERPIM {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Carrier Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Carrier Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_g counter reaches half of the maximum value or the maximum value.
    pub mod TXGOCTIM {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Good Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Good Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txpacketcount_g counter reaches half of the maximum value or the maximum value.
    pub mod TXGPKTIM {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Excessive Deferral Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txexcessdef counter reaches half of the maximum value or the maximum value.
    pub mod TXEXDEFPIM {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Excessive Deferral Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Excessive Deferral Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Pause Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txpausepackets counter reaches half of the maximum value or the maximum value.
    pub mod TXPAUSPIM {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Pause Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Pause Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit VLAN Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txvlanpackets_g counter reaches half of the maximum value or the maximum value.
    pub mod TXVLANGPIM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit VLAN Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit VLAN Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Oversize Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the txoversize_g counter reaches half of the maximum value or the maximum value.
    pub mod TXOSIZEGPIM {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Oversize Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Oversize Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit LPI microsecond counter interrupt Mask Setting this bit masks the interrupt when the Tx_LPI_USEC_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod TXLPIUSCIM {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit LPI microsecond counter interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit LPI microsecond counter interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit LPI transition counter interrupt Mask Setting this bit masks the interrupt when the Tx_LPI_Tran_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod TXLPITRCIM {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit LPI transition counter interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit LPI transition counter interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Tx Octet Count Good and Bad
pub mod MAC_TX_OCTET_COUNT_GOOD_BAD {

    /// Tx Octet Count Good Bad This field indicates the number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad packets.
    pub mod TXOCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx Packet Count Good and Bad
pub mod MAC_TX_PACKET_COUNT_GOOD_BAD {

    /// Tx Packet Count Good Bad This field indicates the number of good and bad packets transmitted, exclusive of retried packets.
    pub mod TXPKTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx Broadcast Packets Good
pub mod MAC_TX_BROADCAST_PACKETS_GOOD {

    /// Tx Broadcast Packets Good This field indicates the number of good broadcast packets transmitted.
    pub mod TXBCASTG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx Multicast Packets Good
pub mod MAC_TX_MULTICAST_PACKETS_GOOD {

    /// Tx Multicast Packets Good This field indicates the number of good multicast packets transmitted.
    pub mod TXMCASTG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx Good and Bad 64-Byte Packets
pub mod MAC_TX_64OCTETS_PACKETS_GOOD_BAD {

    /// Tx 64Octets Packets Good_Bad This field indicates the number of good and bad packets transmitted with length 64 bytes, exclusive of preamble and retried packets.
    pub mod TX64OCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx Good and Bad 65 to 127-Byte Packets
pub mod MAC_TX_65TO127OCTETS_PACKETS_GOOD_BAD {

    /// Tx 65To127Octets Packets Good Bad This field indicates the number of good and bad packets transmitted with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried packets.
    pub mod TX65_127OCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx Good and Bad 128 to 255-Byte Packets
pub mod MAC_TX_128TO255OCTETS_PACKETS_GOOD_BAD {

    /// Tx 128To255Octets Packets Good Bad This field indicates the number of good and bad packets transmitted with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried packets.
    pub mod TX128_255OCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx Good and Bad 256 to 511-Byte Packets
pub mod MAC_TX_256TO511OCTETS_PACKETS_GOOD_BAD {

    /// Tx 256To511Octets Packets Good Bad This field indicates the number of good and bad packets transmitted with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried packets.
    pub mod TX256_511OCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx Good and Bad 512 to 1023-Byte Packets
pub mod MAC_TX_512TO1023OCTETS_PACKETS_GOOD_BAD {

    /// Tx 512To1023Octets Packets Good Bad This field indicates the number of good and bad packets transmitted with length between 512 and 1023 (inclusive) bytes, exclusive of preamble and retried packets.
    pub mod TX512_1023OCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx Good and Bad 1024 to Max-Byte Packets
pub mod MAC_TX_1024TOMAXOCTETS_PACKETS_GOOD_BAD {

    /// Tx 1024ToMaxOctets Packets Good Bad This field indicates the number of good and bad packets transmitted with length between 1024 and maxsize (inclusive) bytes, exclusive of preamble and retried packets.
    pub mod TX1024_MAXOCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good and Bad Unicast Packets Transmitted
pub mod MAC_TX_UNICAST_PACKETS_GOOD_BAD {

    /// Tx Unicast Packets Good Bad This field indicates the number of good and bad unicast packets transmitted.
    pub mod TXUCASTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good and Bad Multicast Packets Transmitted
pub mod MAC_TX_MULTICAST_PACKETS_GOOD_BAD {

    /// Tx Multicast Packets Good Bad This field indicates the number of good and bad multicast packets transmitted.
    pub mod TXMCASTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good and Bad Broadcast Packets Transmitted
pub mod MAC_TX_BROADCAST_PACKETS_GOOD_BAD {

    /// Tx Broadcast Packets Good Bad This field indicates the number of good and bad broadcast packets transmitted.
    pub mod TXBCASTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx Packets Aborted By Underflow Error
pub mod MAC_TX_UNDERFLOW_ERROR_PACKETS {

    /// Tx Underflow Error Packets This field indicates the number of packets aborted because of packets underflow error.
    pub mod TXUNDRFLW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Single Collision Good Packets Transmitted
pub mod MAC_TX_SINGLE_COLLISION_GOOD_PACKETS {

    /// Tx Single Collision Good Packets This field indicates the number of successfully transmitted packets after a single collision in the half-duplex mode.
    pub mod TXSNGLCOLG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Multiple Collision Good Packets Transmitted
pub mod MAC_TX_MULTIPLE_COLLISION_GOOD_PACKETS {

    /// Tx Multiple Collision Good Packets This field indicates the number of successfully transmitted packets after multiple collisions in the half-duplex mode.
    pub mod TXMULTCOLG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Deferred Packets Transmitted
pub mod MAC_TX_DEFERRED_PACKETS {

    /// Tx Deferred Packets This field indicates the number of successfully transmitted after a deferral in the half-duplex mode.
    pub mod TXDEFRD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Late Collision Packets Transmitted
pub mod MAC_TX_LATE_COLLISION_PACKETS {

    /// Tx Late Collision Packets This field indicates the number of packets aborted because of late collision error.
    pub mod TXLATECOL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Excessive Collision Packets Transmitted
pub mod MAC_TX_EXCESSIVE_COLLISION_PACKETS {

    /// Tx Excessive Collision Packets This field indicates the number of packets aborted because of excessive (16) collision errors.
    pub mod TXEXSCOL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Carrier Error Packets Transmitted
pub mod MAC_TX_CARRIER_ERROR_PACKETS {

    /// Tx Carrier Error Packets This field indicates the number of packets aborted because of carrier sense error (no carrier or loss of carrier).
    pub mod TXCARR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Transmitted in Good Packets
pub mod MAC_TX_OCTET_COUNT_GOOD {

    /// Tx Octet Count Good This field indicates the number of bytes transmitted, exclusive of preamble, only in good packets.
    pub mod TXOCTG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good Packets Transmitted
pub mod MAC_TX_PACKET_COUNT_GOOD {

    /// Tx Packet Count Good This field indicates the number of good packets transmitted.
    pub mod TXPKTG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Packets Aborted By Excessive Deferral Error
pub mod MAC_TX_EXCESSIVE_DEFERRAL_ERROR {

    /// Tx Excessive Deferral Error This field indicates the number of packets aborted because of excessive deferral error (deferred for more than two max-sized packet times).
    pub mod TXEXSDEF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Pause Packets Transmitted
pub mod MAC_TX_PAUSE_PACKETS {

    /// Tx Pause Packets This field indicates the number of good Pause packets transmitted.
    pub mod TXPAUSE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good VLAN Packets Transmitted
pub mod MAC_TX_VLAN_PACKETS_GOOD {

    /// Tx VLAN Packets Good This field provides the number of good VLAN packets transmitted.
    pub mod TXVLANG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good Oversize Packets Transmitted
pub mod MAC_TX_OSIZE_PACKETS_GOOD {

    /// Tx OSize Packets Good This field indicates the number of packets transmitted without errors and with length greater than the maxsize (1,518 or 1,522 bytes for VLAN tagged packets; 2000 bytes if enabled in S2KP bit of the CONFIGURATION register).
    pub mod TXOSIZG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good and Bad Packets Received
pub mod MAC_RX_PACKETS_COUNT_GOOD_BAD {

    /// Rx Packets Count Good Bad This field indicates the number of good and bad packets received.
    pub mod RXPKTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes in Good and Bad Packets Received
pub mod MAC_RX_OCTET_COUNT_GOOD_BAD {

    /// Rx Octet Count Good Bad This field indicates the number of bytes received, exclusive of preamble, in good and bad packets.
    pub mod RXOCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes in Good Packets Received
pub mod MAC_RX_OCTET_COUNT_GOOD {

    /// Rx Octet Count Good This field indicates the number of bytes received, exclusive of preamble, only in good packets.
    pub mod RXOCTG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good Broadcast Packets Received
pub mod MAC_RX_BROADCAST_PACKETS_GOOD {

    /// Rx Broadcast Packets Good This field indicates the number of good broadcast packets received.
    pub mod RXBCASTG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good Multicast Packets Received
pub mod MAC_RX_MULTICAST_PACKETS_GOOD {

    /// Rx Multicast Packets Good This field indicates the number of good multicast packets received.
    pub mod RXMCASTG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CRC Error Packets Received
pub mod MAC_RX_CRC_ERROR_PACKETS {

    /// Rx CRC Error Packets This field indicates the number of packets received with CRC error.
    pub mod RXCRCERR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Alignment Error Packets Received
pub mod MAC_RX_ALIGNMENT_ERROR_PACKETS {

    /// Rx Alignment Error Packets This field indicates the number of packets received with alignment (dribble) error.
    pub mod RXALGNERR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Runt Error Packets Received
pub mod MAC_RX_RUNT_ERROR_PACKETS {

    /// Rx Runt Error Packets This field indicates the number of packets received with runt (length less than 64 bytes and CRC error) error.
    pub mod RXRUNTERR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Jabber Error Packets Received
pub mod MAC_RX_JABBER_ERROR_PACKETS {

    /// Rx Jabber Error Packets This field indicates the number of giant packets received with length (including CRC) greater than 1,518 bytes (1,522 bytes for VLAN tagged) and with CRC error.
    pub mod RXJABERR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good Undersize Packets Received
pub mod MAC_RX_UNDERSIZE_PACKETS_GOOD {

    /// Rx Undersize Packets Good This field indicates the number of packets received with length less than 64 bytes, without any errors.
    pub mod RXUNDERSZG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good Oversize Packets Received
pub mod MAC_RX_OVERSIZE_PACKETS_GOOD {

    /// Rx Oversize Packets Good This field indicates the number of packets received without errors, with length greater than the maxsize (1,518 bytes or 1,522 bytes for VLAN tagged packets; 2000 bytes if enabled in the S2KP bit of the MAC_CONFIGURATION register).
    pub mod RXOVERSZG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good and Bad 64-Byte Packets Received
pub mod MAC_RX_64OCTETS_PACKETS_GOOD_BAD {

    /// Rx 64 Octets Packets Good Bad This field indicates the number of good and bad packets received with length 64 bytes, exclusive of the preamble.
    pub mod RX64OCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good and Bad 64-to-127 Byte Packets Received
pub mod MAC_RX_65TO127OCTETS_PACKETS_GOOD_BAD {

    /// Rx 65-127 Octets Packets Good Bad This field indicates the number of good and bad packets received with length between 65 and 127 (inclusive) bytes, exclusive of the preamble.
    pub mod RX65_127OCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good and Bad 128-to-255 Byte Packets Received
pub mod MAC_RX_128TO255OCTETS_PACKETS_GOOD_BAD {

    /// Rx 128-255 Octets Packets Good Bad This field indicates the number of good and bad packets received with length between 128 and 255 (inclusive) bytes, exclusive of the preamble.
    pub mod RX128_255OCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good and Bad 256-to-511 Byte Packets Received
pub mod MAC_RX_256TO511OCTETS_PACKETS_GOOD_BAD {

    /// Rx 256-511 Octets Packets Good Bad This field indicates the number of good and bad packets received with length between 256 and 511 (inclusive) bytes, exclusive of the preamble.
    pub mod RX256_511OCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good and Bad 512-to-1023 Byte Packets Received
pub mod MAC_RX_512TO1023OCTETS_PACKETS_GOOD_BAD {

    /// RX 512-1023 Octets Packets Good Bad This field indicates the number of good and bad packets received with length between 512 and 1023 (inclusive) bytes, exclusive of the preamble.
    pub mod RX512_1023OCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good and Bad 1024-to-Max Byte Packets Received
pub mod MAC_RX_1024TOMAXOCTETS_PACKETS_GOOD_BAD {

    /// Rx 1024-Max Octets Good Bad This field indicates the number of good and bad packets received with length between 1024 and maxsize (inclusive) bytes, exclusive of the preamble.
    pub mod RX1024_MAXOCTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good Unicast Packets Received
pub mod MAC_RX_UNICAST_PACKETS_GOOD {

    /// Rx Unicast Packets Good This field indicates the number of good unicast packets received.
    pub mod RXUCASTG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Length Error Packets Received
pub mod MAC_RX_LENGTH_ERROR_PACKETS {

    /// Rx Length Error Packets This field indicates the number of packets received with length error (Length Type field not equal to packet size), for all packets with valid length field.
    pub mod RXLENERR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Out-of-range Type Packets Received
pub mod MAC_RX_OUT_OF_RANGE_TYPE_PACKETS {

    /// Rx Out of Range Type Packet This field indicates the number of packets received with length field not equal to the valid packet size (greater than 1,500 but less than 1,536).
    pub mod RXOUTOFRNG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Pause Packets Received
pub mod MAC_RX_PAUSE_PACKETS {

    /// Rx Pause Packets This field indicates the number of good and valid Pause packets received.
    pub mod RXPAUSEPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Missed Packets Due to FIFO Overflow
pub mod MAC_RX_FIFO_OVERFLOW_PACKETS {

    /// Rx FIFO Overflow Packets This field indicates the number of missed received packets because of FIFO overflow.
    pub mod RXFIFOOVFL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good and Bad VLAN Packets Received
pub mod MAC_RX_VLAN_PACKETS_GOOD_BAD {

    /// Rx VLAN Packets Good Bad This field indicates the number of good and bad VLAN packets received.
    pub mod RXVLANPKTGB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Watchdog Error Packets Received
pub mod MAC_RX_WATCHDOG_ERROR_PACKETS {

    /// Rx Watchdog Error Packets This field indicates the number of packets received with error because of watchdog timeout error (packets with a data load larger than 2,048 bytes (when JE and WD bits are reset in MAC_CONFIGURATION register), 10,240 bytes (when JE bit is set and WD bit is reset in MAC_CONFIGURATION register), 16,384 bytes (when WD bit is set in MAC_CONFIGURATION register) or the value programmed in the MAC_WATCHDOG_TIMEOUT register).
    pub mod RXWDGERR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Receive Error Packets Received
pub mod MAC_RX_RECEIVE_ERROR_PACKETS {

    /// Rx Receive Error Packets This field indicates the number of packets received with Receive error or Packet Extension error on the GMII or MII interface.
    pub mod RXRCVERR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good Control Packets Received
pub mod MAC_RX_CONTROL_PACKETS_GOOD {

    /// Rx Control Packets Good This field indicates the number of good control packets received.
    pub mod RXCTRLG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Microseconds Tx LPI Asserted
pub mod MAC_TX_LPI_USEC_CNTR {

    /// Tx LPI Microseconds Counter This field indicates the number of microseconds Tx LPI is asserted.
    pub mod TXLPIUSC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Number of Times Tx LPI Asserted
pub mod MAC_TX_LPI_TRAN_CNTR {

    /// Tx LPI Transition counter This field indicates the number of times Tx LPI Entry has occurred.
    pub mod TXLPITRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Microseconds Rx LPI Sampled
pub mod MAC_RX_LPI_USEC_CNTR {

    /// Rx LPI Microseconds Counter This field indicates the number of microseconds Rx LPI is asserted.
    pub mod RXLPIUSC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Number of Times Rx LPI Entered
pub mod MAC_RX_LPI_TRAN_CNTR {

    /// Rx LPI Transition counter This field indicates the number of times Rx LPI Entry has occurred.
    pub mod RXLPITRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MMC IPC Receive Interrupt Mask
pub mod MAC_MMC_IPC_RX_INTERRUPT_MASK {

    /// MMC Receive IPV4 Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4GPIM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 Header Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4HERPIM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 Header Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 Header Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 No Payload Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4NOPAYPIM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 No Payload Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 No Payload Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 Fragmented Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4FRAGPIM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 Fragmented Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 Fragmented Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 UDP Checksum Disabled Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4UDSBLPIM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 UDP Checksum Disabled Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 UDP Checksum Disabled Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV6 Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV6GPIM {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV6 Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV6 Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV6 Header Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV6HERPIM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV6 Header Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV6 Header Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV6 No Payload Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV6NOPAYPIM {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV6 No Payload Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV6 No Payload Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive UDP Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXUDPGPIM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive UDP Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive UDP Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive UDP Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXUDPERPIM {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive UDP Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive UDP Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive TCP Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXTCPGPIM {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive TCP Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive TCP Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive TCP Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXTCPERPIM {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive TCP Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive TCP Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive ICMP Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXICMPGPIM {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive ICMP Good Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive ICMP Good Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive ICMP Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXICMPERPIM {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive ICMP Error Packet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive ICMP Error Packet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4GOIM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 Good Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 Good Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4HEROIM {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 Header Error Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 Header Error Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4NOPAYOIM {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 No Payload Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 No Payload Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4FRAGOIM {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4UDSBLOIM {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV6GOIM {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV6 Good Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV6 Good Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV6HEROIM {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RXIPV6GOIM::RW;
    }

    /// MMC Receive IPV6 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV6NOPAYOIM {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV6 Header Error Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV6 Header Error Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV6 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXUDPGOIM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV6 No Payload Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive IPV6 No Payload Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive UDP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXUDPEROIM {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive UDP Good Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive UDP Good Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive TCP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXTCPGOIM {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive TCP Good Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive TCP Good Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive TCP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXTCPEROIM {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive TCP Error Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive TCP Error Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive ICMP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXICMPGOIM {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive ICMP Good Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive ICMP Good Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Receive ICMP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXICMPEROIM {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive ICMP Error Octet Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Receive ICMP Error Octet Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// MMC IPC Receive Interrupt
pub mod MAC_MMC_IPC_RX_INTERRUPT {

    /// MMC Receive IPV4 Good Packet Counter Interrupt Status This bit is set when the rxipv4_gd_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4GPIS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 Header Error Packet Counter Interrupt Status This bit is set when the rxipv4_hdrerr_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4HERPIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 Header Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 Header Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 No Payload Packet Counter Interrupt Status This bit is set when the rxipv4_nopay_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4NOPAYPIS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 No Payload Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 No Payload Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 Fragmented Packet Counter Interrupt Status This bit is set when the rxipv4_frag_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4FRAGPIS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 Fragmented Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 Fragmented Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 UDP Checksum Disabled Packet Counter Interrupt Status This bit is set when the rxipv4_udsbl_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4UDSBLPIS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 UDP Checksum Disabled Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 UDP Checksum Disabled Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV6 Good Packet Counter Interrupt Status This bit is set when the rxipv6_gd_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV6GPIS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV6 Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV6 Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV6 Header Error Packet Counter Interrupt Status This bit is set when the rxipv6_hdrerr_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV6HERPIS {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV6 Header Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV6 Header Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV6 No Payload Packet Counter Interrupt Status This bit is set when the rxipv6_nopay_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV6NOPAYPIS {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV6 No Payload Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV6 No Payload Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MC Receive UDP Good Packet Counter Interrupt Status This bit is set when the rxudp_gd_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXUDPGPIS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive UDP Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive UDP Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive UDP Error Packet Counter Interrupt Status This bit is set when the rxudp_err_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXUDPERPIS {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive UDP Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive UDP Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive TCP Good Packet Counter Interrupt Status This bit is set when the rxtcp_gd_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXTCPGPIS {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive TCP Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive TCP Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive TCP Error Packet Counter Interrupt Status This bit is set when the rxtcp_err_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXTCPERPIS {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive TCP Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive TCP Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive ICMP Good Packet Counter Interrupt Status This bit is set when the rxicmp_gd_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXICMPGPIS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive ICMP Good Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive ICMP Good Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive ICMP Error Packet Counter Interrupt Status This bit is set when the rxicmp_err_pkts counter reaches half of the maximum value or the maximum value.
    pub mod RXICMPERPIS {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive ICMP Error Packet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive ICMP Error Packet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 Good Octet Counter Interrupt Status This bit is set when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4GOIS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 Good Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 Good Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 Header Error Octet Counter Interrupt Status This bit is set when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4HEROIS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 Header Error Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 Header Error Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 No Payload Octet Counter Interrupt Status This bit is set when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4NOPAYOIS {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 No Payload Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 No Payload Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 Fragmented Octet Counter Interrupt Status This bit is set when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4FRAGOIS {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 Fragmented Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 Fragmented Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status This bit is set when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV4UDSBLOIS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV6 Good Octet Counter Interrupt Status This bit is set when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV6GOIS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV6 Good Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV6 Good Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV6 Header Error Octet Counter Interrupt Status This bit is set when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV6HEROIS {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV6 Header Error Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV6 Header Error Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive IPV6 No Payload Octet Counter Interrupt Status This bit is set when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXIPV6NOPAYOIS {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive IPV6 No Payload Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive IPV6 No Payload Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive UDP Good Octet Counter Interrupt Status This bit is set when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXUDPGOIS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive UDP Good Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive UDP Good Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive UDP Error Octet Counter Interrupt Status This bit is set when the rxudp_err_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXUDPEROIS {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive UDP Error Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive UDP Error Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive TCP Good Octet Counter Interrupt Status This bit is set when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXTCPGOIS {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive TCP Good Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive TCP Good Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive TCP Error Octet Counter Interrupt Status This bit is set when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXTCPEROIS {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive TCP Error Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive TCP Error Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive ICMP Good Octet Counter Interrupt Status This bit is set when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXICMPGOIS {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive ICMP Good Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive ICMP Good Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Receive ICMP Error Octet Counter Interrupt Status This bit is set when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value.
    pub mod RXICMPEROIS {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Receive ICMP Error Octet Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Receive ICMP Error Octet Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// Good IPv4 Datagrams Received
pub mod MAC_RXIPV4_GOOD_PACKETS {

    /// RxIPv4 Good Packets This field indicates the number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload.
    pub mod RXIPV4GDPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPv4 Datagrams Received with Header Errors
pub mod MAC_RXIPV4_HEADER_ERROR_PACKETS {

    /// RxIPv4 Header Error Packets This field indicates the number of IPv4 datagrams received with header (checksum, length, or version mismatch) errors.
    pub mod RXIPV4HDRERRPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPv4 Datagrams Received with No Payload
pub mod MAC_RXIPV4_NO_PAYLOAD_PACKETS {

    /// RxIPv4 Payload Packets This field indicates the number of IPv4 datagram packets received that did not have a TCP, UDP, or ICMP payload.
    pub mod RXIPV4NOPAYPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPv4 Datagrams Received with Fragmentation
pub mod MAC_RXIPV4_FRAGMENTED_PACKETS {

    /// RxIPv4 Fragmented Packets This field indicates the number of good IPv4 datagrams received with fragmentation.
    pub mod RXIPV4FRAGPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPv4 Datagrams Received with UDP Checksum Disabled
pub mod MAC_RXIPV4_UDP_CHECKSUM_DISABLED_PACKETS {

    /// RxIPv4 UDP Checksum Disabled Packets This field indicates the number of good IPv4 datagrams received that had a UDP payload with checksum disabled.
    pub mod RXIPV4UDSBLPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good IPv6 Datagrams Received
pub mod MAC_RXIPV6_GOOD_PACKETS {

    /// RxIPv6 Good Packets This field indicates the number of good IPv6 datagrams received with the TCP, UDP, or ICMP payload.
    pub mod RXIPV6GDPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPv6 Datagrams Received with Header Errors
pub mod MAC_RXIPV6_HEADER_ERROR_PACKETS {

    /// RxIPv6 Header Error Packets This field indicates the number of IPv6 datagrams received with header (length or version mismatch) errors.
    pub mod RXIPV6HDRERRPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPv6 Datagrams Received with No Payload
pub mod MAC_RXIPV6_NO_PAYLOAD_PACKETS {

    /// RxIPv6 Payload Packets This field indicates the number of IPv6 datagram packets received that did not have a TCP, UDP, or ICMP payload.
    pub mod RXIPV6NOPAYPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPv6 Datagrams Received with Good UDP
pub mod MAC_RXUDP_GOOD_PACKETS {

    /// RxUDP Good Packets This field indicates the number of good IP datagrams received with a good UDP payload.
    pub mod RXUDPGDPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPv6 Datagrams Received with UDP Checksum Error
pub mod MAC_RXUDP_ERROR_PACKETS {

    /// RxUDP Error Packets This field indicates the number of good IP datagrams received whose UDP payload has a checksum error.
    pub mod RXUDPERRPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPv6 Datagrams Received with Good TCP Payload
pub mod MAC_RXTCP_GOOD_PACKETS {

    /// RxTCP Good Packets This field indicates the number of good IP datagrams received with a good TCP payload.
    pub mod RXTCPGDPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPv6 Datagrams Received with TCP Checksum Error
pub mod MAC_RXTCP_ERROR_PACKETS {

    /// RxTCP Error Packets This field indicates the number of good IP datagrams received whose TCP payload has a checksum error.
    pub mod RXTCPERRPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPv6 Datagrams Received with Good ICMP Payload
pub mod MAC_RXICMP_GOOD_PACKETS {

    /// RxICMP Good Packets This field indicates the number of good IP datagrams received with a good ICMP payload.
    pub mod RXICMPGDPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPv6 Datagrams Received with ICMP Checksum Error
pub mod MAC_RXICMP_ERROR_PACKETS {

    /// RxICMP Error Packets This field indicates the number of good IP datagrams received whose ICMP payload has a checksum error.
    pub mod RXICMPERRPKT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Good Bytes Received in IPv4 Datagrams
pub mod MAC_RXIPV4_GOOD_OCTETS {

    /// RxIPv4 Good Octets This field indicates the number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data.
    pub mod RXIPV4GDOCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received in IPv4 Datagrams with Header Errors
pub mod MAC_RXIPV4_HEADER_ERROR_OCTETS {

    /// RxIPv4 Header Error Octets This field indicates the number of bytes received in IPv4 datagrams with header errors (checksum, length, version mismatch).
    pub mod RXIPV4HDRERROCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received in IPv4 Datagrams with No Payload
pub mod MAC_RXIPV4_NO_PAYLOAD_OCTETS {

    /// RxIPv4 Payload Octets This field indicates the number of bytes received in IPv4 datagrams that did not have a TCP, UDP, or ICMP payload.
    pub mod RXIPV4NOPAYOCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received in Fragmented IPv4 Datagrams
pub mod MAC_RXIPV4_FRAGMENTED_OCTETS {

    /// RxIPv4 Fragmented Octets This field indicates the number of bytes received in fragmented IPv4 datagrams.
    pub mod RXIPV4FRAGOCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received with UDP Checksum Disabled
pub mod MAC_RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS {

    /// RxIPv4 UDP Checksum Disable Octets This field indicates the number of bytes received in a UDP segment that had the UDP checksum disabled.
    pub mod RXIPV4UDSBLOCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received in Good IPv6 Datagrams
pub mod MAC_RXIPV6_GOOD_OCTETS {

    /// RxIPv6 Good Octets This field indicates the number of bytes received in good IPv6 datagrams encapsulating TCP, UDP, or ICMP data.
    pub mod RXIPV6GDOCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received in IPv6 Datagrams with Data Errors
pub mod MAC_RXIPV6_HEADER_ERROR_OCTETS {

    /// RxIPv6 Header Error Octets This field indicates the number of bytes received in IPv6 datagrams with header errors (length, version mismatch).
    pub mod RXIPV6HDRERROCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received in IPv6 Datagrams with No Payload
pub mod MAC_RXIPV6_NO_PAYLOAD_OCTETS {

    /// RxIPv6 Payload Octets This field indicates the number of bytes received in IPv6 datagrams that did not have a TCP, UDP, or ICMP payload.
    pub mod RXIPV6NOPAYOCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received in Good UDP Segment
pub mod MAC_RXUDP_GOOD_OCTETS {

    /// RxUDP Good Octets This field indicates the number of bytes received in a good UDP segment.
    pub mod RXUDPGDOCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received in UDP Segment with Checksum Errors
pub mod MAC_RXUDP_ERROR_OCTETS {

    /// RxUDP Error Octets This field indicates the number of bytes received in a UDP segment that had checksum errors.
    pub mod RXUDPERROCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received in Good TCP Segment
pub mod MAC_RXTCP_GOOD_OCTETS {

    /// RxTCP Good Octets This field indicates the number of bytes received in a good TCP segment.
    pub mod RXTCPGDOCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received in TCP Segment with Checksum Errors
pub mod MAC_RXTCP_ERROR_OCTETS {

    /// RxTCP Error Octets This field indicates the number of bytes received in a TCP segment that had checksum errors.
    pub mod RXTCPERROCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received in Good ICMP Segment
pub mod MAC_RXICMP_GOOD_OCTETS {

    /// RxICMP Good Octets This field indicates the number of bytes received in a good ICMP segment.
    pub mod RXICMPGDOCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bytes Received in ICMP Segment with Checksum Errors
pub mod MAC_RXICMP_ERROR_OCTETS {

    /// RxICMP Error Octets This field indicates the number of bytes received in a ICMP segment that had checksum errors.
    pub mod RXICMPERROCT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MMC FPE Transmit Interrupt
pub mod MAC_MMC_FPE_TX_INTERRUPT {

    /// MMC Tx FPE Fragment Counter Interrupt status This bit is set when the Tx_FPE_Fragment_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod FCIS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Tx FPE Fragment Counter Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Tx FPE Fragment Counter Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Tx Hold Request Counter Interrupt Status This bit is set when the Tx_Hold_Req_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod HRCIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Tx Hold Request Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Tx Hold Request Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// MMC FPE Transmit Mask Interrupt
pub mod MAC_MMC_FPE_TX_INTERRUPT_MASK {

    /// MMC Transmit Fragment Counter Interrupt Mask Setting this bit masks the interrupt when the Tx_FPE_Fragment_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod FCIM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Fragment Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Fragment Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Transmit Hold Request Counter Interrupt Mask Setting this bit masks the interrupt when the Tx_Hold_Req_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod HRCIM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Transmit Hold Request Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Transmit Hold Request Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// MMC FPE Transmitted Fragment Counter
pub mod MAC_MMC_TX_FPE_FRAGMENT_CNTR {

    /// Tx FPE Fragment counter This field indicates the number of additional mPackets that has been transmitted due to preemption Exists when any one of the RX/TX MMC counters are enabled during FPE Enabled configuration.
    pub mod TXFFC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MMC FPE Transmitted Hold Request Counter
pub mod MAC_MMC_TX_HOLD_REQ_CNTR {

    /// Tx Hold Request Counter This field indicates count of number of a hold request is given to MAC.
    pub mod TXHRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MMC FPE Receive Interrupt
pub mod MAC_MMC_FPE_RX_INTERRUPT {

    /// MMC Rx Packet Assembly Error Counter Interrupt Status This bit is set when the Rx_Packet_Assemble_Err_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod PAECIS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Rx Packet Assembly Error Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Rx Packet Assembly Error Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Rx Packet SMD Error Counter Interrupt Status This bit is set when the Rx_Packet_SMD_Err_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod PSECIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Rx Packet SMD Error Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Rx Packet SMD Error Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Rx Packet Assembly OK Counter Interrupt Status This bit is set when the Rx_Packet_Assemble_Ok_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod PAOCIS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Rx Packet Assembly OK Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Rx Packet Assembly OK Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MMC Rx FPE Fragment Counter Interrupt Status This bit is set when the Rx_FPE_Fragment_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod FCIS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Rx FPE Fragment Counter Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MMC Rx FPE Fragment Counter Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// MMC FPE Receive Interrupt Mask
pub mod MAC_MMC_FPE_RX_INTERRUPT_MASK {

    /// MMC Rx Packet Assembly Error Counter Interrupt Mask Setting this bit masks the interrupt when the R Rx_Packet_Assemble_Err_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod PAECIM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Rx Packet Assembly Error Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Rx Packet Assembly Error Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Rx Packet SMD Error Counter Interrupt Mask Setting this bit masks the interrupt when the R Rx_Packet_SMD_Err_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod PSECIM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Rx Packet SMD Error Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Rx Packet SMD Error Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Rx Packet Assembly OK Counter Interrupt Mask Setting this bit masks the interrupt when the Rx_Packet_Assemble_Ok_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod PAOCIM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Rx Packet Assembly OK Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Rx Packet Assembly OK Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// MMC Rx FPE Fragment Counter Interrupt Mask Setting this bit masks the interrupt when the Tx_FPE_Fragment_Cntr counter reaches half of the maximum value or the maximum value.
    pub mod FCIM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MMC Rx FPE Fragment Counter Interrupt Mask is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MMC Rx FPE Fragment Counter Interrupt Mask is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// MMC Receive Packet Reassembly Error Counter
pub mod MAC_MMC_RX_PACKET_ASSEMBLY_ERR_CNTR {

    /// Rx Packet Assembly Error Counter This field indicates the number of MAC frames with reassembly errors on the Receiver, due to mismatch in the Fragment Count value.
    pub mod PAEC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MMC Receive Packet SMD Error Counter
pub mod MAC_MMC_RX_PACKET_SMD_ERR_CNTR {

    /// Rx Packet SMD Error Counter This field indicates the number of MAC frames rejected due to unknown SMD value and MAC frame fragments rejected due to arriving with an SMD-C when there was no preceding preempted frame.
    pub mod PSEC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MMC Receive Packet Successful Reassembly Counter
pub mod MAC_MMC_RX_PACKET_ASSEMBLY_OK_CNTR {

    /// Rx Packet Assembly OK Counter This field indicates the number of MAC frames that were successfully reassembled and delivered to MAC.
    pub mod PAOC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MMC FPE Received Fragment Counter
pub mod MAC_MMC_RX_FPE_FRAGMENT_CNTR {

    /// Rx FPE Fragment Counter This field indicates the number of additional mPackets received due to preemption Exists when at least one of the RX/TX MMC counters are enabled during FPE Enabled configuration.
    pub mod FFC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 and Layer 4 Control of Filter 0
pub mod MAC_L3_L4_CONTROL0 {

    /// Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets.
    pub mod L3PEN0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching.
    pub mod L3SAM0 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching.
    pub mod L3SAIM0 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching.
    pub mod L3DAM0 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching.
    pub mod L3DAIM0 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets.
    pub mod L3HSBM0 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets.
    pub mod L3HDBM0 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching.
    pub mod L4PEN0 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching.
    pub mod L4SPM0 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching.
    pub mod L4SPIM0 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching.
    pub mod L4DPM0 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching.
    pub mod L4DPIM0 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed.
    pub mod DMCHN0 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter.
    pub mod DMCHEN0 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel Select is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: DMA Channel Select is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Layer 4 Address 0
pub mod MAC_LAYER4_ADDRESS0 {

    /// Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets.
    pub mod L4SP0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets.
    pub mod L4DP0 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 0 Register 0
pub mod MAC_LAYER3_ADDR0_REG0 {

    /// Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[31:0\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A00 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 1 Register 0
pub mod MAC_LAYER3_ADDR1_REG0 {

    /// Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[63:32\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A10 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 2 Register 0
pub mod MAC_LAYER3_ADDR2_REG0 {

    /// Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A20 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 3 Register 0
pub mod MAC_LAYER3_ADDR3_REG0 {

    /// Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[127:96\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A30 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 and Layer 4 Control of Filter 1
pub mod MAC_L3_L4_CONTROL1 {

    /// Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets.
    pub mod L3PEN1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching.
    pub mod L3SAM1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching.
    pub mod L3SAIM1 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching.
    pub mod L3DAM1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching.
    pub mod L3DAIM1 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets.
    pub mod L3HSBM1 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets.
    pub mod L3HDBM1 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching.
    pub mod L4PEN1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching.
    pub mod L4SPM1 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching.
    pub mod L4SPIM1 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching.
    pub mod L4DPM1 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching.
    pub mod L4DPIM1 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed.
    pub mod DMCHN1 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter.
    pub mod DMCHEN1 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel Select is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: DMA Channel Select is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Layer 4 Address 0
pub mod MAC_LAYER4_ADDRESS1 {

    /// Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets.
    pub mod L4SP1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets.
    pub mod L4DP1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 0 Register 1
pub mod MAC_LAYER3_ADDR0_REG1 {

    /// Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[31:0\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A01 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 1 Register 1
pub mod MAC_LAYER3_ADDR1_REG1 {

    /// Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[63:32\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A11 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 2 Register 1
pub mod MAC_LAYER3_ADDR2_REG1 {

    /// Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A21 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 3 Register 1
pub mod MAC_LAYER3_ADDR3_REG1 {

    /// Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[127:96\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A31 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 and Layer 4 Control of Filter 2
pub mod MAC_L3_L4_CONTROL2 {

    /// Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets.
    pub mod L3PEN2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching.
    pub mod L3SAM2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching.
    pub mod L3SAIM2 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching.
    pub mod L3DAM2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching.
    pub mod L3DAIM2 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets.
    pub mod L3HSBM2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets.
    pub mod L3HDBM2 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching.
    pub mod L4PEN2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching.
    pub mod L4SPM2 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching.
    pub mod L4SPIM2 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching.
    pub mod L4DPM2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching.
    pub mod L4DPIM2 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed.
    pub mod DMCHN2 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter.
    pub mod DMCHEN2 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel Select is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: DMA Channel Select is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Layer 4 Address 2
pub mod MAC_LAYER4_ADDRESS2 {

    /// Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets.
    pub mod L4SP2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets.
    pub mod L4DP2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 0 Register 2
pub mod MAC_LAYER3_ADDR0_REG2 {

    /// Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[31:0\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A02 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 0 Register 2
pub mod MAC_LAYER3_ADDR1_REG2 {

    /// Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[63:32\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A12 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 2 Register 2
pub mod MAC_LAYER3_ADDR2_REG2 {

    /// Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A22 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 3 Register 2
pub mod MAC_LAYER3_ADDR3_REG2 {

    /// Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[127:96\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A32 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 and Layer 4 Control of Filter 3
pub mod MAC_L3_L4_CONTROL3 {

    /// Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets.
    pub mod L3PEN3 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching.
    pub mod L3SAM3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching.
    pub mod L3SAIM3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching.
    pub mod L3DAM3 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching.
    pub mod L3DAIM3 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets.
    pub mod L3HSBM3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets.
    pub mod L3HDBM3 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching.
    pub mod L4PEN3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching.
    pub mod L4SPM3 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching.
    pub mod L4SPIM3 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching.
    pub mod L4DPM3 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching.
    pub mod L4DPIM3 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed.
    pub mod DMCHN3 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter.
    pub mod DMCHEN3 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel Select is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: DMA Channel Select is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Layer 4 Address 3
pub mod MAC_LAYER4_ADDRESS3 {

    /// Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets.
    pub mod L4SP3 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets.
    pub mod L4DP3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 0 Register 3
pub mod MAC_LAYER3_ADDR0_REG3 {

    /// Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[31:0\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A03 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 1 Register 3
pub mod MAC_LAYER3_ADDR1_REG3 {

    /// Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[63:32\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A13 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 2 Register 3
pub mod MAC_LAYER3_ADDR2_REG3 {

    /// Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A23 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 3 Register 3
pub mod MAC_LAYER3_ADDR3_REG3 {

    /// Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[127:96\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A33 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 and Layer 4 Control of Filter 4
pub mod MAC_L3_L4_CONTROL4 {

    /// Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets.
    pub mod L3PEN4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching.
    pub mod L3SAM4 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching.
    pub mod L3SAIM4 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching.
    pub mod L3DAM4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching.
    pub mod L3DAIM4 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets.
    pub mod L3HSBM4 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets.
    pub mod L3HDBM4 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching.
    pub mod L4PEN4 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching.
    pub mod L4SPM4 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching.
    pub mod L4SPIM4 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching.
    pub mod L4DPM4 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching.
    pub mod L4DPIM4 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed.
    pub mod DMCHN4 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter.
    pub mod DMCHEN4 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel Select is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: DMA Channel Select is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Layer 4 Address 4
pub mod MAC_LAYER4_ADDRESS4 {

    /// Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets.
    pub mod L4SP4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets.
    pub mod L4DP4 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 0 Register 4
pub mod MAC_LAYER3_ADDR0_REG4 {

    /// Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[31:0\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A04 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 1 Register 4
pub mod MAC_LAYER3_ADDR1_REG4 {

    /// Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[63:32\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A14 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 2 Register 4
pub mod MAC_LAYER3_ADDR2_REG4 {

    /// Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A24 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 3 Register 4
pub mod MAC_LAYER3_ADDR3_REG4 {

    /// Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[127:96\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A34 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 and Layer 4 Control of Filter 5
pub mod MAC_L3_L4_CONTROL5 {

    /// Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets.
    pub mod L3PEN5 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching.
    pub mod L3SAM5 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching.
    pub mod L3SAIM5 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching.
    pub mod L3DAM5 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching.
    pub mod L3DAIM5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets.
    pub mod L3HSBM5 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets.
    pub mod L3HDBM5 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching.
    pub mod L4PEN5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching.
    pub mod L4SPM5 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching.
    pub mod L4SPIM5 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching.
    pub mod L4DPM5 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching.
    pub mod L4DPIM5 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed.
    pub mod DMCHN5 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter.
    pub mod DMCHEN5 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel Select is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: DMA Channel Select is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Layer 4 Address 5
pub mod MAC_LAYER4_ADDRESS5 {

    /// Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets.
    pub mod L4SP5 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets.
    pub mod L4DP5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 0 Register 5
pub mod MAC_LAYER3_ADDR0_REG5 {

    /// Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[31:0\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A05 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 1 Register 5
pub mod MAC_LAYER3_ADDR1_REG5 {

    /// Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[63:32\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A15 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 2 Register 5
pub mod MAC_LAYER3_ADDR2_REG5 {

    /// Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A25 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 3 Register 5
pub mod MAC_LAYER3_ADDR3_REG5 {

    /// Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[127:96\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A35 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 and Layer 4 Control of Filter 6
pub mod MAC_L3_L4_CONTROL6 {

    /// Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets.
    pub mod L3PEN6 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching.
    pub mod L3SAM6 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching.
    pub mod L3SAIM6 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching.
    pub mod L3DAM6 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching.
    pub mod L3DAIM6 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets.
    pub mod L3HSBM6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets.
    pub mod L3HDBM6 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching.
    pub mod L4PEN6 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching.
    pub mod L4SPM6 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching.
    pub mod L4SPIM6 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching.
    pub mod L4DPM6 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching.
    pub mod L4DPIM6 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed.
    pub mod DMCHN6 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter.
    pub mod DMCHEN6 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel Select is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: DMA Channel Select is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Layer 4 Address 6
pub mod MAC_LAYER4_ADDRESS6 {

    /// Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets.
    pub mod L4SP6 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets.
    pub mod L4DP6 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 0 Register 6
pub mod MAC_LAYER3_ADDR0_REG6 {

    /// Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[31:0\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A06 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 1 Register 6
pub mod MAC_LAYER3_ADDR1_REG6 {

    /// Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[63:32\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A16 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 2 Register 6
pub mod MAC_LAYER3_ADDR2_REG6 {

    /// Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A26 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 3 Register 6
pub mod MAC_LAYER3_ADDR3_REG6 {

    /// Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[127:96\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A36 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 and Layer 4 Control of Filter 0
pub mod MAC_L3_L4_CONTROL7 {

    /// Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets.
    pub mod L3PEN7 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching.
    pub mod L3SAM7 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching.
    pub mod L3SAIM7 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP SA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP SA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching.
    pub mod L3DAM7 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching.
    pub mod L3DAIM7 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 3 IP DA Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 3 IP DA Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 3 IP SA Higher Bits Match IPv4 Packets: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets.
    pub mod L3HSBM7 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 3 IP DA Higher Bits Match IPv4 Packets: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 packets.
    pub mod L3HDBM7 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching.
    pub mod L4PEN7 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Protocol is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Protocol is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching.
    pub mod L4SPM7 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching.
    pub mod L4SPIM7 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Source Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Source Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching.
    pub mod L4DPM7 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching.
    pub mod L4DPIM7 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Layer 4 Destination Port Inverse Match is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Layer 4 Destination Port Inverse Match is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// DMA Channel Number When DMCHEN is set high, this field selects the DMA Channel number to which the packet passed by this filter is routed.
    pub mod DMCHN7 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Channel Select Enable When set, this bit enables the selection of the DMA channel number for the packet that is passed by this L3_L4 filter.
    pub mod DMCHEN7 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel Select is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: DMA Channel Select is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Layer 4 Address 7
pub mod MAC_LAYER4_ADDRESS7 {

    /// Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4SPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets.
    pub mod L4SP7 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets.
    pub mod L4DP7 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 0 Register 7
pub mod MAC_LAYER3_ADDR0_REG7 {

    /// Layer 3 Address 0 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[31:0\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A07 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 1 Register 7
pub mod MAC_LAYER3_ADDR1_REG7 {

    /// Layer 3 Address 1 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[63:32\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A17 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 2 Register 7
pub mod MAC_LAYER3_ADDR2_REG7 {

    /// Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A27 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Layer 3 Address 3 Register 7
pub mod MAC_LAYER3_ADDR3_REG7 {

    /// Layer 3 Address 3 Field When the L3PEN0 and L3SAM0 bits are set in the MAC_L3_L4_CONTROL0 register, this field contains the value to be matched with Bits\[127:96\] of the IP Source Address field in the IPv6 packets.
    pub mod L3A37 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timestamp Control
pub mod MAC_TIMESTAMP_CONTROL {

    /// Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets.
    pub mod TSENA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Timestamp is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp.
    pub mod TSCFUPDT {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Coarse method is used to update system timestamp
            pub const COARSE: u32 = 0b0;

            /// 0b1: Fine method is used to update system timestamp
            pub const FINE: u32 = 0b1;
        }
    }

    /// Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the MAC_System_Time_Seconds_Update and MAC_System_Time_Nanoseconds_Update registers.
    pub mod TSINIT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp is not initialized
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Timestamp is initialized
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in MAC_System_Time_Seconds_Update and MAC_System_Time_Nanoseconds_Update registers.
    pub mod TSUPDT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp is not updated
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Timestamp is updated
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction.
    pub mod TSADDREG {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Addend Register is not updated
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Addend Register is updated
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Presentation Time Generation Enable When this bit is set the Presentation Time generation will be enabled.
    pub mod PTGE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Presentation Time Generation is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Presentation Time Generation is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC.
    pub mod TSENALL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp for All Packets disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Timestamp for All Packets enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9A_C9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds.
    pub mod TSCTRLSSR {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp Digital or Binary Rollover Control is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Timestamp Digital or Binary Rollover Control is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets.
    pub mod TSVER2ENA {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PTP Packet Processing for Version 2 Format is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: PTP Packet Processing for Version 2 Format is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets.
    pub mod TSIPENA {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Processing of PTP over Ethernet Packets is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Processing of PTP over Ethernet Packets is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable Processing of PTP Packets Sent over IPv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets.
    pub mod TSIPV6ENA {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Processing of PTP Packets Sent over IPv6-UDP is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Processing of PTP Packets Sent over IPv6-UDP is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets.
    pub mod TSIPV4ENA {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Processing of PTP Packets Sent over IPv4-UDP is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Processing of PTP Packets Sent over IPv4-UDP is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp).
    pub mod TSEVNTENA {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp Snapshot for Event Messages is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Timestamp Snapshot for Event Messages is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node.
    pub mod TSMSTRENA {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Snapshot for Messages Relevant to Master is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Snapshot for Messages Relevant to Master is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, decide the set of PTP packet types for which snapshot needs to be taken.
    pub mod SNAPTYPSEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet.
    pub mod TSENMACADDR {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MAC Address for PTP Packet Filtering is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: MAC Address for PTP Packet Filtering is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable checksum correction during OST for PTP over UDP/IPv4 packets When this bit is set, the last two bytes of PTP message sent over UDP/IPv4 is updated to keep the UDP checksum correct, for changes made to origin timestamp and/or correction field as part of one step timestamp operation.
    pub mod CSC {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: checksum correction during OST for PTP over UDP/IPv4 packets is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: checksum correction during OST for PTP over UDP/IPv4 packets is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// External System Time Input When this bit is set, the MAC uses the external 64-bit reference System Time input for the following: - To take the timestamp provided as status - To insert the timestamp in transmit PTP packets when One-step Timestamp or Timestamp Offload feature is enabled.
    pub mod ESTI {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External System Time Input is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: External System Time Input is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software.
    pub mod TXTSSTSM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Timestamp Status Mode is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmit Timestamp Status Mode is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// AV 802.
    pub mod AV8021ASMEN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: AV 802.1AS Mode is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: AV 802.1AS Mode is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Subsecond Increment
pub mod MAC_SUB_SECOND_INCREMENT {

    /// Sub-nanosecond Increment Value This field contains the sub-nanosecond increment value, represented in nanoseconds multiplied by 2^8.
    pub mod SNSINC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Sub-second Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the sub-second register.
    pub mod SSINC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// System Time Seconds
pub mod MAC_SYSTEM_TIME_SECONDS {

    /// Timestamp Second The value in this field indicates the current value in seconds of the System Time maintained by the MAC.
    pub mod TSS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// System Time Nanoseconds
pub mod MAC_SYSTEM_TIME_NANOSECONDS {

    /// Timestamp Sub Seconds The value in this field has the sub-second representation of time, with an accuracy of 0.
    pub mod TSSS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// System Time Seconds Update
pub mod MAC_SYSTEM_TIME_SECONDS_UPDATE {

    /// Timestamp Seconds The value in this field is the seconds part of the update.
    pub mod TSS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// System Time Nanoseconds Update
pub mod MAC_SYSTEM_TIME_NANOSECONDS_UPDATE {

    /// Timestamp Sub Seconds The value in this field is the sub-seconds part of the update.
    pub mod TSSS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register.
    pub mod ADDSUB {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Add time
            pub const ADD: u32 = 0b0;

            /// 0b1: Subtract time
            pub const SUB: u32 = 0b1;
        }
    }
}

/// Timestamp Addend
pub mod MAC_TIMESTAMP_ADDEND {

    /// Timestamp Addend Register This field indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization.
    pub mod TSAR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// System Time - Higher Word Seconds
pub mod MAC_SYSTEM_TIME_HIGHER_WORD_SECONDS {

    /// Timestamp Higher Word Register This field contains the most-significant 16-bits of timestamp seconds value.
    pub mod TSHWR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timestamp Status
pub mod MAC_TIMESTAMP_STATUS {

    /// Timestamp Seconds Overflow When this bit is set, it indicates that the seconds value of the timestamp (when supporting version 2 format) has overflowed beyond 32'hFFFF_FFFF.
    pub mod TSSOVF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp Seconds Overflow status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Timestamp Seconds Overflow status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Timestamp Target Time Reached When set, this bit indicates that the value of system time is greater than or equal to the value specified in the MAC_PPS0_Target_Time_Seconds and MAC_PPS0_Target_Time_Nanoseconds registers.
    pub mod TSTARGT0 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp Target Time Reached status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Timestamp Target Time Reached status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Auxiliary Timestamp Trigger Snapshot This bit is set high when the auxiliary snapshot is written to the FIFO.
    pub mod AUXTSTRIG {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Auxiliary Timestamp Trigger Snapshot status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Auxiliary Timestamp Trigger Snapshot status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Timestamp Target Time Error This bit is set when the latest target time programmed in the MAC_PPS0_Target_Time_Seconds and MAC_PPS0_Target_Time_Nanoseconds registers elapses.
    pub mod TSTRGTERR0 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp Target Time Error status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Timestamp Target Time Error status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Timestamp Target Time Reached for Target Time PPS1 When set, this bit indicates that the value of system time is greater than or equal to the value specified in the MAC_PPS1_TARGET_TIME_SECONDS and MAC_PPS1_TARGET_TIME_NANOSECONDS registers.
    pub mod TSTARGT1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp Target Time Reached for Target Time PPS1 status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Timestamp Target Time Reached for Target Time PPS1 status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Timestamp Target Time Error This bit is set when the latest target time programmed in the MAC_PPS1_TARGET_TIME_SECONDS and MAC_PPS1_TARGET_TIME_NANOSECONDS registers elapses.
    pub mod TSTRGTERR1 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TSTRGTERR0::RW;
    }

    /// Timestamp Target Time Reached for Target Time PPS2 When set, this bit indicates that the value of system time is greater than or equal to the value specified in the MAC_PPS2_TARGET_TIME_SECONDS and MAC_PPS2_TARGET_TIME_NANOSECONDS registers.
    pub mod TSTARGT2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp Target Time Reached for Target Time PPS2 status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Timestamp Target Time Reached for Target Time PPS2 status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Timestamp Target Time Error This bit is set when the latest target time programmed in the MAC_PPS2_TARGET_TIME_SECONDS and MAC_PPS2_TARGET_TIME_NANOSECONDS registers elapses.
    pub mod TSTRGTERR2 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TSTRGTERR0::RW;
    }

    /// Timestamp Target Time Reached for Target Time PPS3 When this bit is set, it indicates that the value of system time is greater than or equal to the value specified in the MAC_PPS3_TARGET_TIME_SECONDS and MAC_PPS3_TARGET_TIME_NANOSECONDS registers.
    pub mod TSTARGT3 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timestamp Target Time Reached for Target Time PPS3 status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Timestamp Target Time Reached for Target Time PPS3 status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Timestamp Target Time Error This bit is set when the latest target time programmed in the MAC_PPS3_TARGET_TIME_SECONDS and MAC_PPS3_TARGET_TIME_NANOSECONDS registers elapses.
    pub mod TSTRGTERR3 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TSTRGTERR0::RW;
    }

    /// Tx Timestamp Status Interrupt Status In non-EQOS_CORE configurations when drop transmit status is enabled in MTL, this bit is set when the captured transmit timestamp is updated in the MAC_TX_TIMESTAMP_STATUS_NANOSECONDS and MAC_TX_TIMESTAMP_STATUS_SECONDS registers.
    pub mod TXTSSIS {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Tx Timestamp Status Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Tx Timestamp Status Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable.
    pub mod ATSSTN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Auxiliary Timestamp Snapshot Trigger Missed This bit is set when the Auxiliary timestamp snapshot FIFO is full and external trigger was set.
    pub mod ATSSTM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Auxiliary Timestamp Snapshot Trigger Missed status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Auxiliary Timestamp Snapshot Trigger Missed status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Number of Auxiliary Timestamp Snapshots This field indicates the number of Snapshots available in the FIFO.
    pub mod ATSNS {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (5 bits: 0b11111 << 25)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmit Timestamp Status Nanoseconds
pub mod MAC_TX_TIMESTAMP_STATUS_NANOSECONDS {

    /// Transmit Timestamp Status Low This field contains the 31 bits of the Nanoseconds field of the Transmit packet's captured timestamp.
    pub mod TXTSSLO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit Timestamp Status Missed When this bit is set, it indicates one of the following: - The timestamp of the current packet is ignored if TXTSSTSM bit of the TIMESTAMP_CONTROL register is reset - The timestamp of the previous packet is overwritten with timestamp of the current packet if TXTSSTSM bit of the MAC_TIMESTAMP_CONTROL register is set.
    pub mod TXTSSMIS {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Timestamp Status Missed status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit Timestamp Status Missed status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// Transmit Timestamp Status Seconds
pub mod MAC_TX_TIMESTAMP_STATUS_SECONDS {

    /// Transmit Timestamp Status High This field contains the lower 32 bits of the Seconds field of Transmit packet's captured timestamp.
    pub mod TXTSSHI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Auxiliary Timestamp Control
pub mod MAC_AUXILIARY_CONTROL {

    /// Auxiliary Snapshot FIFO Clear When set, this bit resets the pointers of the Auxiliary Snapshot FIFO.
    pub mod ATSFC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Auxiliary Snapshot FIFO Clear is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Auxiliary Snapshot FIFO Clear is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Auxiliary Snapshot 0 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 0.
    pub mod ATSEN0 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Auxiliary Snapshot $i is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Auxiliary Snapshot $i is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Auxiliary Snapshot 1 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 1.
    pub mod ATSEN1 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ATSEN0::RW;
    }

    /// Auxiliary Snapshot 2 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 2.
    pub mod ATSEN2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ATSEN0::RW;
    }

    /// Auxiliary Snapshot 3 Enable This bit controls the capturing of Auxiliary Snapshot Trigger 3.
    pub mod ATSEN3 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ATSEN0::RW;
    }
}

/// Auxiliary Timestamp Nanoseconds
pub mod MAC_AUXILIARY_TIMESTAMP_NANOSECONDS {

    /// Auxiliary Timestamp Contains the lower 31 bits (nanoseconds field) of the auxiliary timestamp.
    pub mod AUXTSLO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Auxiliary Timestamp Seconds
pub mod MAC_AUXILIARY_TIMESTAMP_SECONDS {

    /// Auxiliary Timestamp Contains the lower 32 bits of the Seconds field of the auxiliary timestamp.
    pub mod AUXTSHI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timestamp Ingress Asymmetry Correction
pub mod MAC_TIMESTAMP_INGRESS_ASYM_CORR {

    /// One-Step Timestamp Ingress Asymmetry Correction This field contains the ingress path asymmetry value to be added to correctionField of Pdelay_Resp PTP packet.
    pub mod OSTIAC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// imestamp Egress Asymmetry Correction
pub mod MAC_TIMESTAMP_EGRESS_ASYM_CORR {

    /// One-Step Timestamp Egress Asymmetry Correction This field contains the egress path asymmetry value to be subtracted from correctionField of Pdelay_Resp PTP packet.
    pub mod OSTEAC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timestamp Ingress Correction Nanosecond
pub mod MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND {

    /// Timestamp Ingress Correction This field contains the ingress path correction value as defined by the Ingress Correction expression.
    pub mod TSIC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timestamp Egress Correction Nanosecond
pub mod MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND {

    /// Timestamp Egress Correction This field contains the nanoseconds part of the egress path correction value as defined by the Egress Correction expression.
    pub mod TSEC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timestamp Ingress Correction Subnanosecond
pub mod MAC_TIMESTAMP_INGRESS_CORR_SUBNANOSEC {

    /// Timestamp Ingress Correction, sub-nanoseconds This field contains the sub-nanoseconds part of the ingress path correction value as defined by the "Ingress Correction" expression.
    pub mod TSICSNS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timestamp Egress Correction Subnanosecond
pub mod MAC_TIMESTAMP_EGRESS_CORR_SUBNANOSEC {

    /// Timestamp Egress Correction, sub-nanoseconds This field contains the sub-nanoseconds part of the egress path correction value as defined by the "Egress Correction" expression.
    pub mod TSECSNS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timestamp Ingress Latency
pub mod MAC_TIMESTAMP_INGRESS_LATENCY {

    /// Ingress Timestamp Latency, in nanoseconds This register holds the average latency in nanoseconds between the input ports (phy_rxd_i) of MAC and the actual point (GMII/MII) where the ingress timestamp is taken.
    pub mod ITLSNS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Ingress Timestamp Latency, in sub-nanoseconds This register holds the average latency in sub-nanoseconds between the input ports (phy_rxd_i) of MAC and the actual point (GMII/MII) where the ingress timestamp is taken.
    pub mod ITLNS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timestamp Egress Latency
pub mod MAC_TIMESTAMP_EGRESS_LATENCY {

    /// Egress Timestamp Latency, in sub-nanoseconds This register holds the average latency in sub-nanoseconds between the actual point (GMII/MII) where the egress timestamp is taken and the output ports (phy_txd_o) of the MAC.
    pub mod ETLSNS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Egress Timestamp Latency, in nanoseconds This register holds the average latency in nanoseconds between the actual point (GMII/MII) where the egress timestamp is taken and the output ports (phy_txd_o) of the MAC.
    pub mod ETLNS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS Control
pub mod MAC_PPS_CONTROL {

    /// PPS Output Frequency Control This field controls the frequency of the PPS0 output (ptp_pps_o\[0\]) signal.
    pub mod PPSCTRL_PPSCMD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Flexible PPS Output Mode Enable When this bit is set, Bits\[3:0\] function as PPSCMD.
    pub mod PPSEN0 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Flexible PPS Output Mode is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Flexible PPS Output Mode is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Target Time Register Mode for PPS0 Output This field indicates the Target Time registers (MAC_PPS0_TARGET_TIME_SECONDS and MAC_PPS0_TARGET_TIME_NANOSECONDS) mode for PPS0 output signal:
    pub mod TRGTMODSEL0 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Target Time registers are programmed only for generating the interrupt event. The Flexible PPS function must not be enabled in this mode, otherwise spurious transitions may be observed on the corresponding ptp_pps_o output port
            pub const ONLY_INT: u32 = 0b00;

            /// 0b10: Target Time registers are programmed for generating the interrupt event and starting or stopping the PPS0 output signal generation
            pub const INT_ST: u32 = 0b10;

            /// 0b11: Target Time registers are programmed only for starting or stopping the PPS0 output signal generation. No interrupt is asserted
            pub const ONLY_ST: u32 = 0b11;
        }
    }

    /// MCGR Mode Enable for PPS0 Output This field enables the 0th PPS instance to operate in PPS or MCGR mode.
    pub mod MCGREN0 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 0th PPS instance is enabled to operate in PPS mode
            pub const PPS: u32 = 0b0;

            /// 0b1: 0th PPS instance is enabled to operate in MCGR mode
            pub const MCGR: u32 = 0b1;
        }
    }

    /// Flexible PPS1 Output Control This field controls the flexible PPS1 output (ptp_pps_o\[1\]) signal.
    pub mod PPSCMD1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Target Time Register Mode for PPS1 Output This field indicates the Target Time registers (MAC_PPS1_TARGET_TIME_SECONDS and MAC_PPS1_TARGET_TIME_NANOSECONDS) mode for PPS1 output signal.
    pub mod TRGTMODSEL1 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRGTMODSEL0::RW;
    }

    /// MCGR Mode Enable for PPS1 Output This field enables the 1st PPS instance to operate in PPS or MCGR mode.
    pub mod MCGREN1 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 1st PPS instance is disabled to operate in PPS or MCGR mode
            pub const DISABLE: u32 = 0b0;

            /// 0b1: 1st PPS instance is enabled to operate in PPS or MCGR mode
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Flexible PPS2 Output Control This field controls the flexible PPS2 output (ptp_pps_o\[2\]) signal.
    pub mod PPSCMD2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Target Time Register Mode for PPS2 Output This field indicates the Target Time registers (MAC_PPS2_TARGET_TIME_SECONDS and MAC_PPS2_TARGET_TIME_NANOSECONDS) mode for PPS2 output signal.
    pub mod TRGTMODSEL2 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (2 bits: 0b11 << 21)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRGTMODSEL0::RW;
    }

    /// MCGR Mode Enable for PPS2 Output This field enables the 2nd PPS instance to operate in PPS or MCGR mode.
    pub mod MCGREN2 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 2nd PPS instance is disabled to operate in PPS or MCGR mode
            pub const DISABLE: u32 = 0b0;

            /// 0b1: 2nd PPS instance is enabled to operate in PPS or MCGR mode
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Flexible PPS3 Output Control This field controls the flexible PPS3 output (ptp_pps_o\[3\]) signal.
    pub mod PPSCMD3 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Target Time Register Mode for PPS3 Output This field indicates the Target Time registers (MAC_PPS3_TARGET_TIME_SECONDS and MAC_PPS3_TARGET_TIME_NANOSECONDS) mode for PPS3 output signal.
    pub mod TRGTMODSEL3 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TRGTMODSEL0::RW;
    }

    /// MCGR Mode Enable for PPS3 Output This field enables the 3rd PPS instance to operate in PPS or MCGR mode.
    pub mod MCGREN3 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS0 Target Time Seconds
pub mod MAC_PPS0_TARGET_TIME_SECONDS {

    /// PPS Target Time Seconds Register This field stores the time in seconds.
    pub mod TSTRH0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS0 Target Time Nanoseconds
pub mod MAC_PPS0_TARGET_TIME_NANOSECONDS {

    /// Target Time Low for PPS Register This register stores the time in (signed) nanoseconds.
    pub mod TTSL0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS_CONTROL register is programmed to 010 or 011.
    pub mod TRGTBUSY0 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PPS Target Time Register Busy status is not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: PPS Target Time Register Busy is detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// PPS0 Interval
pub mod MAC_PPS0_INTERVAL {

    /// PPS Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output.
    pub mod PPSINT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS0 Width
pub mod MAC_PPS0_WIDTH {

    /// PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS0 signal output.
    pub mod PPSWIDTH0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS1 Target Time Seconds
pub mod MAC_PPS1_TARGET_TIME_SECONDS {

    /// PPS Target Time Seconds Register This field stores the time in seconds.
    pub mod TSTRH1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS1 Target Time Nanoseconds
pub mod MAC_PPS1_TARGET_TIME_NANOSECONDS {

    /// Target Time Low for PPS Register This register stores the time in (signed) nanoseconds.
    pub mod TTSL1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS_CONTROL register is programmed to 010 or 011.
    pub mod TRGTBUSY1 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PPS Target Time Register Busy status is not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: PPS Target Time Register Busy is detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// PPS1 Interval
pub mod MAC_PPS1_INTERVAL {

    /// PPS Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output.
    pub mod PPSINT1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS1 Width
pub mod MAC_PPS1_WIDTH {

    /// PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS0 signal output.
    pub mod PPSWIDTH1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS2 Target Time Seconds
pub mod MAC_PPS2_TARGET_TIME_SECONDS {

    /// PPS Target Time Seconds Register This field stores the time in seconds.
    pub mod TSTRH2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS2 Target Time Nanoseconds
pub mod MAC_PPS2_TARGET_TIME_NANOSECONDS {

    /// Target Time Low for PPS Register This register stores the time in (signed) nanoseconds.
    pub mod TTSL2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS_CONTROL register is programmed to 010 or 011.
    pub mod TRGTBUSY2 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PPS Target Time Register Busy status is not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: PPS Target Time Register Busy is detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// PPS2 Interval
pub mod MAC_PPS2_INTERVAL {

    /// PPS Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output.
    pub mod PPSINT2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS2 Width
pub mod MAC_PPS2_WIDTH {

    /// PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS0 signal output.
    pub mod PPSWIDTH2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS3 Target Time Seconds
pub mod MAC_PPS3_TARGET_TIME_SECONDS {

    /// PPS Target Time Seconds Register This field stores the time in seconds.
    pub mod TSTRH3 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS3 Target Time Nanoseconds
pub mod MAC_PPS3_TARGET_TIME_NANOSECONDS {

    /// Target Time Low for PPS Register This register stores the time in (signed) nanoseconds.
    pub mod TTSL3 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS_CONTROL register is programmed to 010 or 011.
    pub mod TRGTBUSY3 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PPS Target Time Register Busy status is not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: PPS Target Time Register Busy is detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// PPS3 Interval
pub mod MAC_PPS3_INTERVAL {

    /// PPS Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output.
    pub mod PPSINT3 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PPS3 Width
pub mod MAC_PPS3_WIDTH {

    /// PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS0 signal output.
    pub mod PPSWIDTH3 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PTP Offload Engine Control
pub mod MAC_PTO_CONTROL {

    /// PTP Offload Enable When this bit is set, the PTP Offload feature is enabled.
    pub mod PTOEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PTP Offload feature is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: PTP Offload feature is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Automatic PTP SYNC message Enable When this bit is set, PTP SYNC message is generated periodically based on interval programmed or trigger from application, when the MAC is programmed to be in Clock Master mode.
    pub mod ASYNCEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic PTP SYNC message is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Automatic PTP SYNC message is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Automatic PTP Pdelay_Req message Enable When this bit is set, PTP Pdelay_Req message is generated periodically based on interval programmed or trigger from application, when the MAC is programmed to be in Peer-to-Peer Transparent mode.
    pub mod APDREQEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic PTP Pdelay_Req message is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Automatic PTP Pdelay_Req message is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Automatic PTP SYNC message Trigger When this bit is set, one PTP SYNC message is transmitted.
    pub mod ASYNCTRIG {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic PTP SYNC message Trigger is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Automatic PTP SYNC message Trigger is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Automatic PTP Pdelay_Req message Trigger When this bit is set, one PTP Pdelay_Req message is transmitted.
    pub mod APDREQTRIG {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic PTP Pdelay_Req message Trigger is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Automatic PTP Pdelay_Req message Trigger is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Disable PTO Delay Request/Response response generation When this bit is set, the Delay Request and Delay response is not generated for received SYNC and Delay request packet respectively, as required by the programmed mode.
    pub mod DRRDIS {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PTO Delay Request/Response response generation is enabled
            pub const ENABLE: u32 = 0b0;

            /// 0b1: PTO Delay Request/Response response generation is disabled
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Disable Peer Delay Response response generation When this bit is set, the Peer Delay Response (Pdelay_Resp) response is not be generated for received Peer Delay Request (Pdelay_Req) request packet, as required by the programmed mode.
    pub mod PDRDIS {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Peer Delay Response response generation is enabled
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Peer Delay Response response generation is disabled
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Domain Number This field indicates the domain Number in which the PTP node is operating.
    pub mod DN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Source Port Identity 0
pub mod MAC_SOURCE_PORT_IDENTITY0 {

    /// Source Port Identity 0 This field indicates bits \[31:0\] of sourcePortIdentity of PTP node.
    pub mod SPI0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Source Port Identity 1
pub mod MAC_SOURCE_PORT_IDENTITY1 {

    /// Source Port Identity 1 This field indicates bits \[63:32\] of sourcePortIdentity of PTP node.
    pub mod SPI1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Source Port Identity 2
pub mod MAC_SOURCE_PORT_IDENTITY2 {

    /// Source Port Identity 2 This field indicates bits \[79:64\] of sourcePortIdentity of PTP node.
    pub mod SPI2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Log Message Interval
pub mod MAC_LOG_MESSAGE_INTERVAL {

    /// Log Sync Interval This field indicates the periodicity of the automatically generated SYNC message when the PTP node is Master.
    pub mod LSI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay_Req to SYNC Ratio In Slave mode, it is used for controlling frequency of Delay_Req messages transmitted.
    pub mod DRSYNCR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: DelayReq generated for every received SYNC
            pub const SYNC1: u32 = 0b000;

            /// 0b001: DelayReq generated every alternate reception of SYNC
            pub const SYNC2: u32 = 0b001;

            /// 0b010: for every 4 SYNC messages
            pub const SYNC4: u32 = 0b010;

            /// 0b011: for every 8 SYNC messages
            pub const SYNC8: u32 = 0b011;

            /// 0b100: for every 16 SYNC messages
            pub const SYNC16: u32 = 0b100;

            /// 0b101: for every 32 SYNC messages
            pub const SYNC32: u32 = 0b101;
        }
    }

    /// Log Min Pdelay_Req Interval This field indicates logMinPdelayReqInterval of PTP node.
    pub mod LMPDRI {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MTL Operation Mode
pub mod MTL_OPERATION_MODE {

    /// Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL.
    pub mod DTXSTS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Drop Transmit Status is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Drop Transmit Status is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Receive Arbitration Algorithm This field is used to select the arbitration algorithm for the Rx side.
    pub mod RAA {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Strict priority (SP)
            pub const SP: u32 = 0b0;

            /// 0b1: Weighted Strict Priority (WSP)
            pub const WSP: u32 = 0b1;
        }
    }

    /// Tx Scheduling Algorithm This field indicates the algorithm for Tx scheduling:
    pub mod SCHALG {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: WRR algorithm
            pub const WRR: u32 = 0b00;

            /// 0b01: WFQ algorithm when DCB feature is selected.Otherwise, Reserved
            pub const WFQ: u32 = 0b01;

            /// 0b10: DWRR algorithm when DCB feature is selected.Otherwise, Reserved
            pub const DWRR: u32 = 0b10;

            /// 0b11: Strict priority algorithm
            pub const SP: u32 = 0b11;
        }
    }

    /// Counters Preset When this bit is set, - MTL_TxQ\[0-7\]_Underflow register is initialized/preset to 12'h7F0.
    pub mod CNTPRST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Counters Preset is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Counters Preset is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Counters Reset When this bit is set, all counters are reset.
    pub mod CNTCLR {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Counters are not reset
            pub const DISABLE: u32 = 0b0;

            /// 0b1: All counters are reset
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Flexible Rx parser Enable When this bit is set to 1, the Programmable Rx Parser functionality is enabled.
    pub mod FRPE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Flexible Rx parser is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Flexible Rx parser is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// FIFO Debug Access Control and Status
pub mod MTL_DBG_CTL {

    /// FIFO Debug Access Enable When this bit is set, it indicates that the debug mode access to the FIFO is enabled.
    pub mod FDBGEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: FIFO Debug Access is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: FIFO Debug Access is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Debug Mode Access to FIFO When this bit is set, it indicates that the current access to the FIFO is read, write, and debug access.
    pub mod DBGMOD {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Debug Mode Access to FIFO is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Debug Mode Access to FIFO is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Byte Enables This field indicates the number of data bytes valid in the data register during Write operation.
    pub mod BYTEEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Byte 0 valid
            pub const B0_VAL: u32 = 0b00;

            /// 0b01: Byte 0 and Byte 1 are valid
            pub const B01_VAL: u32 = 0b01;

            /// 0b10: Byte 0, Byte 1, and Byte 2 are valid
            pub const B012_VAL: u32 = 0b10;

            /// 0b11: All four bytes are valid
            pub const B0123_VAL: u32 = 0b11;
        }
    }

    /// Encoded Packet State This field is used to write the control information to the Tx FIFO or Rx FIFO.
    pub mod PKTSTATE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Packet Data
            pub const PKT_DATA: u32 = 0b00;

            /// 0b01: Control Word/Normal Status
            pub const CW_NS: u32 = 0b01;

            /// 0b10: SOP Data/Last Status
            pub const SOP_LS: u32 = 0b10;

            /// 0b11: EOP Data/EOP
            pub const EOP: u32 = 0b11;
        }
    }

    /// Reset All Pointers When this bit is set, the pointers of all FIFOs are reset when FIFO Debug Access is enabled.
    pub mod RSTALL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset All Pointers is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Reset All Pointers is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Reset Pointers of Selected FIFO When this bit is set, the pointers of the currently-selected FIFO are reset when FIFO Debug Access is enabled.
    pub mod RSTSEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset Pointers of Selected FIFO is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Reset Pointers of Selected FIFO is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// FIFO Read Enable When this bit is set, it enables the Read operation on selected FIFO when FIFO Debug Access is enabled.
    pub mod FIFORDEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: FIFO Read is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: FIFO Read is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// FIFO Write Enable When this bit is set, it enables the Write operation on selected FIFO when FIFO Debug Access is enabled.
    pub mod FIFOWREN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: FIFO Write is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: FIFO Write is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// FIFO Selected for Access This field indicates the FIFO selected for debug access:
    pub mod FIFOSEL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Tx FIFO
            pub const TXFIFO: u32 = 0b00;

            /// 0b01: Tx Status FIFO (only read access when SLVMOD is set)
            pub const TXSTSFIFO: u32 = 0b01;

            /// 0b10: TSO FIFO (cannot be accessed when SLVMOD is set)
            pub const TSOFIFO: u32 = 0b10;

            /// 0b11: Rx FIFO
            pub const RXFIFO: u32 = 0b11;
        }
    }

    /// Receive Packet Available Interrupt Status Enable When this bit is set, an interrupt is generated when EOP of received packet is written to the Rx FIFO.
    pub mod PKTIE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Packet Available Interrupt Status is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive Packet Available Interrupt Status is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Transmit Status Available Interrupt Status Enable When this bit is set, an interrupt is generated when Transmit status is available in slave mode.
    pub mod STSIE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Packet Available Interrupt Status is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmit Packet Available Interrupt Status is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// FIFO Debug Status
pub mod MTL_DBG_STS {

    /// FIFO Busy When set, this bit indicates that a FIFO operation is in progress in the MAC and content of the following fields is not valid: - All other fields of this register - All fields of the MTL_FIFO_DEBUG_DATA register
    pub mod FIFOBUSY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: FIFO Busy not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: FIFO Busy detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Encoded Packet State This field is used to get the control or status information of the selected FIFO.
    pub mod PKTSTATE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Packet Data
            pub const PKT_DATA: u32 = 0b00;

            /// 0b01: Control Word/Normal Status
            pub const CW_NS: u32 = 0b01;

            /// 0b10: SOP Data/Last Status
            pub const SOP_LS: u32 = 0b10;

            /// 0b11: EOP Data/EOP
            pub const EOP: u32 = 0b11;
        }
    }

    /// Byte Enables This field indicates the number of data bytes valid in the data register during Read operation.
    pub mod BYTEEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Byte 0 valid
            pub const B0_VAL: u32 = 0b00;

            /// 0b01: Byte 0 and Byte 1 are valid
            pub const B01_VAL: u32 = 0b01;

            /// 0b10: Byte 0, Byte 1, and Byte 2 are valid
            pub const B012_VAL: u32 = 0b10;

            /// 0b11: All four bytes are valid
            pub const B0123_VAL: u32 = 0b11;
        }
    }

    /// Receive Packet Available Interrupt Status When set, this bit indicates that MAC layer has written the EOP of received packet to the Rx FIFO.
    pub mod PKTI {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Packet Available Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Receive Packet Available Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Transmit Status Available Interrupt Status When set, this bit indicates that the Slave mode Tx packet is transmitted, and the status is available in Tx Status FIFO.
    pub mod STSI {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Status Available Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit Status Available Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Remaining Locations in the FIFO Slave Access Mode: This field indicates the space available in selected FIFO.
    pub mod LOCR {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (17 bits: 0x1ffff << 15)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// FIFO Debug Data
pub mod MTL_FIFO_DEBUG_DATA {

    /// FIFO Debug Data During debug or slave access write operation, this field contains the data to be written to the Tx FIFO, Rx FIFO, or TSO FIFO.
    pub mod FDBGDATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MTL Interrupt Status
pub mod MTL_INTERRUPT_STATUS {

    /// Queue 0 Interrupt status This bit indicates that there is an interrupt from Queue 0.
    pub mod Q0IS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Queue 0 Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Queue 0 Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Queue 1 Interrupt status This bit indicates that there is an interrupt from Queue 1.
    pub mod Q1IS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Queue 1 Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Queue 1 Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Queue 2 Interrupt status This bit indicates that there is an interrupt from Queue 2.
    pub mod Q2IS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Queue 2 Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Queue 2 Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Queue 3 Interrupt status This bit indicates that there is an interrupt from Queue 3.
    pub mod Q3IS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Queue 3 Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Queue 3 Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Queue 4 Interrupt status This bit indicates that there is an interrupt from Queue 4.
    pub mod Q4IS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Queue 4 Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Queue 4 Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Debug Interrupt status This bit indicates an interrupt event during the slave access.
    pub mod DBGIS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Debug Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Debug Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// EST (TAS- 802.
    pub mod ESTIS {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: EST (TAS- 802.1Qbv) Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: EST (TAS- 802.1Qbv) Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MTL Rx Parser Interrupt Status This bit indicates that there is an interrupt from Rx Parser Block.
    pub mod MTLPIS {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MTL Rx Parser Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MTL Rx Parser Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// Receive Queue and DMA Channel Mapping 0
pub mod MTL_RXQ_DMA_MAP0 {

    /// Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: - 000: DMA Channel 0 - 001: DMA Channel 1 - 010: DMA Channel 2 - 011: DMA Channel 3 - 100: DMA Channel 4 - 101: Reserved - 110: Reserved - 111: Reserved This field is valid when the Q0DDMACH field is reset.
    pub mod Q0MDMACH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
    pub mod Q0DDMACH {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Queue 0 disabled for DA-based DMA Channel Selection
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Queue 0 enabled for DA-based DMA Channel Selection
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: - 000: DMA Channel 0 - 001: DMA Channel 1 - 010: DMA Channel 2 - 011: DMA Channel 3 - 100: DMA Channel 4 - 101: Reserved - 110: Reserved - 111: Reserved This field is valid when the Q1DDMACH field is reset.
    pub mod Q1MDMACH {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
    pub mod Q1DDMACH {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Queue 1 disabled for DA-based DMA Channel Selection
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Queue 1 enabled for DA-based DMA Channel Selection
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Queue 2 Mapped to DMA Channel This field controls the routing of the received packet in Queue 2 to the DMA channel: - 000: DMA Channel 0 - 001: DMA Channel 1 - 010: DMA Channel 2 - 011: DMA Channel 3 - 100: DMA Channel 4 - 101: Reserved - 110: Reserved - 111: Reserved This field is valid when the Q2DDMACH field is reset.
    pub mod Q2MDMACH {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Queue 2 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 2 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
    pub mod Q2DDMACH {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Queue 2 disabled for DA-based DMA Channel Selection
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Queue 2 enabled for DA-based DMA Channel Selection
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Queue 3 Mapped to DMA Channel This field controls the routing of the received packet in Queue 3 to the DMA channel: - 000: DMA Channel 0 - 001: DMA Channel 1 - 010: DMA Channel 2 - 011: DMA Channel 3 - 100: DMA Channel 4 - 101: Reserved - 110: Reserved - 111: Reserved This field is valid when the Q3DDMACH field is reset.
    pub mod Q3MDMACH {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Queue 3 Enabled for Dynamic (per packet) DMA Channel Selection When set, this bit indicates that the packets received in Queue 3 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
    pub mod Q3DDMACH {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Queue 3 disabled for DA-based DMA Channel Selection
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Queue 3 enabled for DA-based DMA Channel Selection
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Receive Queue and DMA Channel Mapping 1
pub mod MTL_RXQ_DMA_MAP1 {

    /// Queue 4 Mapped to DMA Channel This field controls the routing of the packet received in Queue 4 to the DMA channel: - 000: DMA Channel 0 - 001: DMA Channel 1 - 010: DMA Channel 2 - 011: DMA Channel 3 - 100: DMA Channel 4 - 101: Reserved - 110: Reserved - 111: Reserved This field is valid when the Q4DDMACH field is reset.
    pub mod Q4MDMACH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Queue 4 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 4 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
    pub mod Q4DDMACH {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Queue 4 disabled for DA-based DMA Channel Selection
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Queue 4 enabled for DA-based DMA Channel Selection
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Time Based Scheduling Control
pub mod MTL_TBS_CTRL {

    /// EST offset Mode When this bit is set, the Launch Time value used in Time Based Scheduling is interpreted as an EST offset value and is added to the Base Time Register (BTR) of the current list.
    pub mod ESTM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: EST offset Mode is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: EST offset Mode is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Launch Expiry Offset Valid When set indicates the LEOS field is valid.
    pub mod LEOV {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LEOS field is invalid
            pub const INVALID: u32 = 0b0;

            /// 0b1: LEOS field is valid
            pub const VALID: u32 = 0b1;
        }
    }

    /// Launch Expiry GSN Offset The number GSN slots that has to be added to the Launch GSN to compute the Launch Expiry time.
    pub mod LEGOS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Launch Expiry Offset The value in units of 256 nanoseconds that has to be added to the Launch time to compute the Launch Expiry time.
    pub mod LEOS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Enhancements to Scheduled Transmission Control
pub mod MTL_EST_CONTROL {

    /// Enable EST When reset, the gate control list processing is halted and all gates are assumed to be in Open state.
    pub mod EEST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: EST is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: EST is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Switch to S/W owned list When set indicates that the software has programmed that list that it currently owns (SWOL) and the hardware should switch to the new list based on the new BTR.
    pub mod SSWL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Switch to S/W owned list is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Switch to S/W owned list is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Do not Drop frames during Frame Size Error When set, frames are not be dropped during Head-of-Line blocking due to Frame Size Error (HLBF field of MTL_EST_STATUS register).
    pub mod DDBF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Drop frames during Frame Size Error
            pub const DROP: u32 = 0b0;

            /// 0b1: Do not Drop frames during Frame Size Error
            pub const DONT_DROP: u32 = 0b1;
        }
    }

    /// Drop Frames causing Scheduling Error When set frames reported to cause HOL Blocking due to not getting scheduled (HLBS field of EST_STATUS register) after 4,8,16,32 (based on LCSE field of this register) GCL iterations are dropped.
    pub mod DFBS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Do not Drop Frames causing Scheduling Error
            pub const DONT_DROP: u32 = 0b0;

            /// 0b1: Drop Frames causing Scheduling Error
            pub const DROP: u32 = 0b1;
        }
    }

    /// Loop Count to report Scheduling Error Programmable number of GCL list iterations before reporting an HLBS error defined in EST_STATUS register.
    pub mod LCSE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 4 iterations
            pub const bf_4_ITERNS: u32 = 0b00;

            /// 0b01: 8 iterations
            pub const bf_8_ITERNS: u32 = 0b01;

            /// 0b10: 16 iterations
            pub const bf_16_ITERNS: u32 = 0b10;

            /// 0b11: 32 iterations
            pub const bf_32_ITERNS: u32 = 0b11;
        }
    }

    /// Time Interval Left Shift Amount This field provides the left shift amount for the programmed Time Interval values used in the Gate Control Lists.
    pub mod TILS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Current Time Offset Value Provides a 12 bit time offset value in nano second that is added to the current time to compensate for all the implementation pipeline delays such as the CDC sync delay, buffering delays, data path delays etc.
    pub mod CTOV {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (12 bits: 0xfff << 12)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PTP Time Offset Value The value of PTP Clock period multiplied by 6 in nanoseconds.
    pub mod PTOV {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Enhancements to Scheduled Transmission Status
pub mod MTL_EST_STATUS {

    /// Switch to S/W owned list Complete When "1" indicates the hardware has successfully switched to the SWOL, and the SWOL bit has been updated to that effect.
    pub mod SWLC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Switch to S/W owned list Complete not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Switch to S/W owned list Complete detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// BTR Error When "1" indicates a programming error in the BTR of SWOL where the programmed value is less than current time.
    pub mod BTRE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: BTR Error not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: BTR Error detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Head-Of-Line Blocking due to Frame Size Set when HOL Blocking is noticed on one or more Queues as a result of none of the Time Intervals of gate open in the GCL being greater than or equal to the duration needed for frame size (or frame fragment size when preemption is enabled) transmission.
    pub mod HLBF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Head-Of-Line Blocking due to Frame Size not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Head-Of-Line Blocking due to Frame Size detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Head-Of-Line Blocking due to Scheduling Set when the frame is not able to win arbitration and get scheduled even after 4 iterations of the GCL.
    pub mod HLBS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Head-Of-Line Blocking due to Scheduling not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Head-Of-Line Blocking due to Scheduling detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Constant Gate Control Error This error occurs when the list length (LLR) is 1 and the programmed Time Interval (TI) value after the optional Left Shifting is less than or equal to the Cycle Time (CTR).
    pub mod CGCE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Constant Gate Control Error not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Constant Gate Control Error detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// S/W owned list When '0' indicates Gate control list number "0" is owned by software and when "1" indicates the Gate Control list "1" is owned by the software.
    pub mod SWOL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Gate control list number "0" is owned by software
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Gate control list number "1" is owned by software
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// BTR Error Loop Count Provides the minimum count (N) for which the equation Current Time =< New BTR + (N * New Cycle Time) becomes true.
    pub mod BTRL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Current GCL Slot Number Indicates the slot number of the GCL list.
    pub mod CGSN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EST Scheduling Error
pub mod MTL_EST_SCH_ERROR {

    /// Schedule Error Queue Number The One Hot Encoded Queue Numbers that have experienced error/timeout described in HLBS field of status register.
    pub mod SEQN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EST Frame Size Error
pub mod MTL_EST_FRM_SIZE_ERROR {

    /// Frame Size Error Queue Number The One Hot Encoded Queue Numbers that have experienced error described in HLBF field of status register.
    pub mod FEQN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EST Frame Size Capture
pub mod MTL_EST_FRM_SIZE_CAPTURE {

    /// Frame Size of HLBF Captures the Frame Size of the dropped frame related to queue number indicated in HBFQ field of this register.
    pub mod HBFS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (15 bits: 0x7fff << 0)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Queue Number of HLBF Captures the binary value of the of the first Queue (number) experiencing HLBF error (see HLBF field of status register).
    pub mod HBFQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EST Interrupt Enable
pub mod MTL_EST_INTR_ENABLE {

    /// Interrupt Enable for Switch List When set, generates interrupt when the configuration change is successful and the hardware has switched to the new list.
    pub mod IECC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt for Switch List is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Interrupt for Switch List is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Interrupt Enable for BTR Error When set, generates interrupt when the BTR Error occurs and is indicated in the status.
    pub mod IEBE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt for BTR Error is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Interrupt for BTR Error is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Interrupt Enable for HLBF When set, generates interrupt when the Head-of-Line Blocking due to Frame Size error occurs and is indicated in the status.
    pub mod IEHF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt for HLBF is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Interrupt for HLBF is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Interrupt Enable for HLBS When set, generates interrupt when the Head-of-Line Blocking due to Scheduling issue and is indicated in the status.
    pub mod IEHS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt for HLBS is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Interrupt for HLBS is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Interrupt Enable for CGCE When set, generates interrupt when the Constant Gate Control Error occurs and is indicated in the status.
    pub mod CGCE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt for CGCE is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Interrupt for CGCE is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// EST GCL Control
pub mod MTL_EST_GCL_CONTROL {

    /// Start Read/Write Op When set indicates a Read/Write Op has started and is in progress.
    pub mod SRWO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Start Read/Write Op disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Start Read/Write Op enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Read '1', Write '0': When set to '1': Read Operation When set to '0': Write Operation.
    pub mod R1W0 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write Operation
            pub const WRITE: u32 = 0b0;

            /// 0b1: Read Operation
            pub const READ: u32 = 0b1;
        }
    }

    /// Gate Control Related Registers When set to "1" indicates the R/W access is for the GCL related registers (BTR, CTR, TER, LLR) whose address is provided by GCRA.
    pub mod GCRR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Gate Control Related Registers are disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Gate Control Related Registers are enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Debug Mode When set to "1" indicates R/W in debug mode where the memory bank (for GCL and Time related registers) is explicitly provided by DBGB value, when set to "0" SWOL bit is used to determine which bank to use.
    pub mod DBGM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Debug Mode is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Debug Mode is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Debug Mode Bank Select When set to "0" indicates R/W in debug mode should be directed to Bank 0 (GCL0 and corresponding Time related registers).
    pub mod DBGB {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: R/W in debug mode should be directed to Bank 0
            pub const BANK0: u32 = 0b0;

            /// 0b1: R/W in debug mode should be directed to Bank 1
            pub const BANK1: u32 = 0b1;
        }
    }

    /// Gate Control List Address: (GCLA when GCRR is "0").
    pub mod ADDR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (9 bits: 0x1ff << 8)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// When set indicates the last write operation was aborted as software writes to GCL and GCL registers is prohibited when SSWL bit of MTL_EST_CONTROL Register is set.
    pub mod ERR0 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ERR0 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: ERR1 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// EST ECC Inject Error Enable When set along with EEST bit of MTL_EST_CONTROL register, enables the ECC error injection feature.
    pub mod ESTEIEE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: EST ECC Inject Error is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: EST ECC Inject Error is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// ECC Inject Error Control for EST Memory When EIEE bit of this register is set, following are the errors inserted based on the value encoded in this field.
    pub mod ESTEIEC {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Insert 1 bit error
            pub const bf_1BIT: u32 = 0b00;

            /// 0b01: Insert 2 bit errors
            pub const bf_2BIT: u32 = 0b01;

            /// 0b10: Insert 3 bit errors
            pub const bf_3BIT: u32 = 0b10;

            /// 0b11: Insert 1 bit error in address field
            pub const bf_1BIT_ADDR: u32 = 0b11;
        }
    }
}

/// EST GCL Data
pub mod MTL_EST_GCL_DATA {

    /// Gate Control Data The data corresponding to the address selected in the MTL_GCL_CONTROL register.
    pub mod GCD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Frame Preemption Control and Status
pub mod MTL_FPE_CTRL_STS {

    /// Additional Fragment Size used to indicate, in units of 64 bytes, the minimum number of bytes over 64 bytes required in non-final fragments of preempted frames.
    pub mod AFSZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Preemption Classification When set indicates the corresponding Queue must be classified as preemptable, when '0' Queue is classified as express.
    pub mod PEC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Hold/Release Status - 1: Indicates a Set-and-Hold-MAC operation was last executed and the pMAC is in Hold State.
    pub mod HRS {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Indicates a Set-and-Release-MAC operation was last executed and the pMAC is in Release State
            pub const SET_REL: u32 = 0b0;

            /// 0b1: Indicates a Set-and-Hold-MAC operation was last executed and the pMAC is in Hold State
            pub const SET_HOLD: u32 = 0b1;
        }
    }
}

/// Frame Preemption Hold and Release Advance
pub mod MTL_FPE_ADVANCE {

    /// Hold Advance The maximum time in nanoseconds that can elapse between issuing a HOLD to the MAC and the MAC ceasing to transmit any preemptable frame that is in the process of transmission or any preemptable frames that are queued for transmission.
    pub mod HADV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Release Advance The maximum time in nanoseconds that can elapse between issuing a RELEASE to the MAC and the MAC being ready to resume transmission of preemptable frames, in the absence of there being any express frames available for transmission.
    pub mod RADV {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// RXP Control Status
pub mod MTL_RXP_CONTROL_STATUS {

    /// Number of valid entries in the Instruction table This control indicates the number of valid entries in the Instruction Memory.
    pub mod NVE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of parsable entries in the Instruction table This control indicates the number of parsable entries in the Instruction Memory.
    pub mod NPE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RX Parser in Idle state This status bit is set to 1 when the Rx parser is in Idle State and waiting for a new packet for processing.
    pub mod RXPI {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: RX Parser not in Idle state
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: RX Parser in Idle state
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// RXP Interrupt Control Status
pub mod MTL_RXP_INTERRUPT_CONTROL_STATUS {

    /// Number of Valid Entries Overflow Interrupt Status While parsing if the Instruction address found to be more than NVE (Number of Valid Entries in MTL_RXP_CONTROL register), then this bit is set to 1.
    pub mod NVEOVIS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Number of Valid Entries Overflow Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Number of Valid Entries Overflow Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Number of Parsable Entries Overflow Interrupt Status While parsing a packet if the number of parsed entries found to be more than NPE\[\] (Number of Parseable Entries in MTL_RXP_CONTROL register),then this bit is set to 1.
    pub mod NPEOVIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Number of Parsable Entries Overflow Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Number of Parsable Entries Overflow Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Frame Offset Overflow Interrupt Status While parsing if the Instruction table entry's 'Frame Offset' found to be more than EOF offset, then then this bit is set.
    pub mod FOOVIS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Frame Offset Overflow Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Frame Offset Overflow Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Packet Dropped due to RF Interrupt Status If the Rx Parser result says to drop the packet by setting RF=1 in the instruction memory, then this bit is set to 1.
    pub mod PDRFIS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Packet Dropped due to RF Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Packet Dropped due to RF Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Number of Valid Entries Overflow Interrupt Enable When this bit is set, the NVEOVIS interrupt is enabled.
    pub mod NVEOVIE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Number of Valid Entries Overflow Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Number of Valid Entries Overflow Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Number of Parsable Entries Overflow Interrupt Enable When this bit is set, the NPEOVIS interrupt is enabled.
    pub mod NPEOVIE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Number of Parsable Entries Overflow Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Number of Parsable Entries Overflow Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Frame Offset Overflow Interrupt Enable When this bit is set, the FOOVIS interrupt is enabled.
    pub mod FOOVIE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Frame Offset Overflow Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Frame Offset Overflow Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Packet Drop due to RF Interrupt Enable When this bit is set, the PDRFIS interrupt is enabled.
    pub mod PDRFIE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Packet Drop due to RF Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Packet Drop due to RF Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// RXP Drop Count
pub mod MTL_RXP_DROP_CNT {

    /// Rx Parser Drop count This 31-bit counter is implemented whenever a Rx Parser Drops a packet due to RF =1.
    pub mod RXPDC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rx Parser Drop Counter Overflow Bit When set, this bit indicates that the MTL_RXP_DROP_CNT (RXPDC) Counter field crossed the maximum limit.
    pub mod RXPDCOVF {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Rx Parser Drop count overflow not occurred
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Rx Parser Drop count overflow occurred
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// RXP Error Count
pub mod MTL_RXP_ERROR_CNT {

    /// Rx Parser Error count This 31-bit counter is implemented whenever a Rx Parser encounters following Error scenarios - Entry address >= NVE\[\] - Number Parsed Entries >= NPE\[\] - Entry address > EOF data entry address The counter is cleared when the register is read.
    pub mod RXPEC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rx Parser Error Counter Overflow Bit When set, this bit indicates that the MTL_RXP_ERROR_CNT (RXPEC) Counter field crossed the maximum limit.
    pub mod RXPECOVF {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Rx Parser Error count overflow not occurred
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Rx Parser Error count overflow occurred
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// RXP Indirect Access Control and Status
pub mod MTL_RXP_INDIRECT_ACC_CONTROL_STATUS {

    /// FRP Instruction Table Offset Address This field indicates the ADDR of the 32-bit entry in Rx parser instruction table.
    pub mod ADDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read Write Control When this bit is set to 1 indicates the write operation to the Rx Parser Memory.
    pub mod WRRDN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Read operation to the Rx Parser Memory
            pub const READ: u32 = 0b0;

            /// 0b1: Write operation to the Rx Parser Memory
            pub const WRITE: u32 = 0b1;
        }
    }

    /// FRP Instruction Table Access Busy When this bit is set to 1 by the software then it indicates to start the Read/Write operation from/to the Rx Parser Memory.
    pub mod STARTBUSY {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: hardware not busy
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: hardware is busy (Read/Write operation from/to the Rx Parser Memory)
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// RXP Indirect Access Data
pub mod MTL_RXP_INDIRECT_ACC_DATA {

    /// FRP Instruction Table Write/Read Data Software should write this register before issuing any write command.
    pub mod DATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Queue 0 Transmit Operation Mode
pub mod MTL_TXQ0_OPERATION_MODE {

    /// Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values.
    pub mod FTQ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Flush Transmit Queue is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Flush Transmit Queue is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue.
    pub mod TSF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Store and Forward is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmit Store and Forward is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Transmit Queue Enable This field is used to enable/disable the transmit queue 0.
    pub mod TXQEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Not enabled
            pub const DISABLE: u32 = 0b00;

            /// 0b01: Enable in AV mode (Reserved in non-AV)
            pub const EN_IF_AV: u32 = 0b01;

            /// 0b10: Enabled
            pub const ENABLE: u32 = 0b10;
        }
    }

    /// Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue.
    pub mod TTC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 32
            pub const bf_32BYTES: u32 = 0b000;

            /// 0b001: 64
            pub const bf_64BYTES: u32 = 0b001;

            /// 0b010: 96
            pub const bf_96BYTES: u32 = 0b010;

            /// 0b011: 128
            pub const bf_128BYTES: u32 = 0b011;

            /// 0b100: 192
            pub const bf_192BYTES: u32 = 0b100;

            /// 0b101: 256
            pub const bf_256BYTES: u32 = 0b101;

            /// 0b110: 384
            pub const bf_384BYTES: u32 = 0b110;

            /// 0b111: 512
            pub const bf_512BYTES: u32 = 0b111;
        }
    }

    /// Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes.
    pub mod TQS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Queue 0 Underflow Counter
pub mod MTL_TXQ0_UNDERFLOW {

    /// Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx Queue Underflow.
    pub mod UFFRMCNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count.
    pub mod UFCNTOVF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Overflow not detected for Underflow Packet Counter
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Overflow detected for Underflow Packet Counter
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// Queue 0 Transmit Debug
pub mod MTL_TXQ0_DEBUG {

    /// Transmit Queue in Pause When this bit is high and the Rx flow control is enabled, it indicates that the Tx Queue is in the Pause condition (in the full-duplex only mode) because of the following: - Reception of the PFC packet for the priorities assigned to the Tx Queue when PFC is enabled - Reception of 802.
    pub mod TXQPAUSED {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Queue in Pause status is not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit Queue in Pause status is detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MTL Tx Queue Read Controller Status This field indicates the state of the Tx Queue Read Controller:
    pub mod TRCSTS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Idle state
            pub const IDLE: u32 = 0b00;

            /// 0b01: Read state (transferring data to the MAC transmitter)
            pub const READ: u32 = 0b01;

            /// 0b10: Waiting for pending Tx Status from the MAC transmitter
            pub const WAIT: u32 = 0b10;

            /// 0b11: Flushing the Tx queue because of the Packet Abort request from the MAC
            pub const FLUSH: u32 = 0b11;
        }
    }

    /// MTL Tx Queue Write Controller Status When high, this bit indicates that the MTL Tx Queue Write Controller is active, and it is transferring the data to the Tx Queue.
    pub mod TWCSTS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MTL Tx Queue Write Controller status is not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MTL Tx Queue Write Controller status is detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MTL Tx Queue Not Empty Status When this bit is high, it indicates that the MTL Tx Queue is not empty and some data is left for transmission.
    pub mod TXQSTS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MTL Tx Queue Not Empty status is not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MTL Tx Queue Not Empty status is detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MTL Tx Status FIFO Full Status When high, this bit indicates that the MTL Tx Status FIFO is full.
    pub mod TXSTSFSTS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MTL Tx Status FIFO Full status is not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MTL Tx Status FIFO Full status is detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Number of Packets in the Transmit Queue This field indicates the current number of packets in the Tx Queue.
    pub mod PTXQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of Status Words in Tx Status FIFO of Queue This field indicates the current number of status in the Tx Status FIFO of this queue.
    pub mod STXSTSF {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Queue 0 ETS Status
pub mod MTL_TXQ0_ETS_STATUS {

    /// Average Bits per Slot This field contains the average transmitted bits per slot.
    pub mod ABS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Queue 0 Quantum or Weights
pub mod MTL_TXQ0_QUANTUM_WEIGHT {

    /// Quantum or Weights When the DCB operation is enabled with DWRR algorithm for Queue 0 traffic, this field contains the quantum value in bytes to be added to credit during every queue scanning cycle.
    pub mod ISCQW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (21 bits: 0x1fffff << 0)
        pub const mask: u32 = 0x1fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Queue 0 Interrupt Control Status
pub mod MTL_Q0_INTERRUPT_CONTROL_STATUS {

    /// Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet.
    pub mod TXUNFIS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Queue Underflow Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit Queue Underflow Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value.
    pub mod ABPSIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Average Bits Per Slot Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Average Bits Per Slot Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled.
    pub mod TXUIE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Queue Underflow Interrupt Status is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmit Queue Underflow Interrupt Status is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the sbd_intr_o or mci_intr_o interrupt when the average bits per slot status is updated.
    pub mod ABPSIE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Average Bits Per Slot Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Average Bits Per Slot Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet.
    pub mod RXOVFIS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Queue Overflow Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Receive Queue Overflow Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled.
    pub mod RXOIE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Queue Overflow Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive Queue Overflow Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Queue 0 Receive Operation Mode
pub mod MTL_RXQ0_OPERATION_MODE {

    /// Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): The received packet is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold.
    pub mod RTC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 64
            pub const bf_64BYTE: u32 = 0b00;

            /// 0b01: 32
            pub const bf_32BYTE: u32 = 0b01;

            /// 0b10: 96
            pub const bf_96BYTE: u32 = 0b10;

            /// 0b11: 128
            pub const bf_128BYTE: u32 = 0b11;
        }
    }

    /// Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC.
    pub mod FUP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Forward Undersized Good Packets is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Forward Undersized Good Packets is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, GMII_ER, watchdog timeout, or overflow).
    pub mod FEP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Forward Error Packets is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Forward Error Packets is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Receive Queue Store and Forward When this bit is set, the DWC_ether_qos reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register.
    pub mod RSF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Queue Store and Forward is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive Queue Store and Forward is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine.
    pub mod DIS_TCP_EF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Dropping of TCP/IP Checksum Error Packets is enabled
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Dropping of TCP/IP Checksum Error Packets is disabled
            pub const DISABLE: u32 = 0b1;
        }
    }

    /// Enable Hardware Flow Control When this bit is set, the flow control signal operation, based on the fill-level of Rx queue, is enabled.
    pub mod EHFC {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Hardware Flow Control is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Hardware Flow Control is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Threshold for Activating Flow Control (in half-duplex and full-duplex These bits control the threshold (fill-level of Rx queue) at which the flow control is activated: For more information on encoding for this field, see RFD.
    pub mod RFA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (fill-level of Rx queue) at which the flow control is de-asserted after activation: - 0: Full minus 1 KB, that is, FULL 1 KB - 1: Full minus 1.
    pub mod RFD {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (4 bits: 0b1111 << 14)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive Queue Size This field indicates the size of the allocated Receive queues in blocks of 256 bytes.
    pub mod RQS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (5 bits: 0b11111 << 20)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Queue 0 Missed Packet and Overflow Counter
pub mod MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT {

    /// Overflow Packet Counter This field indicates the number of packets discarded by the DWC_ether_qos because of Receive queue overflow.
    pub mod OVFPKTCNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit.
    pub mod OVFCNTOVF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Overflow Counter overflow not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Overflow Counter overflow detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Missed Packet Counter This field indicates the number of packets missed by the DWC_ether_qos because the application asserted ari_pkt_flush_i\[\] for this queue.
    pub mod MISPKTCNT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Missed Packet Counter Overflow Bit When set, this bit indicates that the Rx Queue Missed Packet Counter crossed the maximum limit.
    pub mod MISCNTOVF {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Missed Packet Counter overflow not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Missed Packet Counter overflow detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// Queue 0 Receive Debug
pub mod MTL_RXQ0_DEBUG {

    /// MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue.
    pub mod RWCSTS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MTL Rx Queue Write Controller Active Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MTL Rx Queue Write Controller Active Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller:
    pub mod RRCSTS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Idle state
            pub const IDLE: u32 = 0b00;

            /// 0b01: Reading packet data
            pub const READ_DATA: u32 = 0b01;

            /// 0b10: Reading packet status (or timestamp)
            pub const READ_STS: u32 = 0b10;

            /// 0b11: Flushing the packet data and status
            pub const FLUSH: u32 = 0b11;
        }
    }

    /// MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx Queue:
    pub mod RXQSTS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Rx Queue empty
            pub const EMPTY: u32 = 0b00;

            /// 0b01: Rx Queue fill-level below flow-control deactivate threshold
            pub const BLW_THR: u32 = 0b01;

            /// 0b10: Rx Queue fill-level above flow-control activate threshold
            pub const ABV_THR: u32 = 0b10;

            /// 0b11: Rx Queue full
            pub const FULL: u32 = 0b11;
        }
    }

    /// Number of Packets in Receive Queue This field indicates the current number of packets in the Rx Queue.
    pub mod PRXQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (14 bits: 0x3fff << 16)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Queue 0 Receive Control
pub mod MTL_RXQ0_CONTROL {

    /// Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0.
    pub mod RXQ_WEGT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive Queue Packet Arbitration When this bit is set, the DWC_ether_qos drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue.
    pub mod RXQ_FRM_ARBIT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Queue Packet Arbitration is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive Queue Packet Arbitration is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Queue 1 Transmit Operation Mode
pub mod MTL_TXQ1_OPERATION_MODE {
    pub use super::MTL_TXQ0_OPERATION_MODE::FTQ;
    pub use super::MTL_TXQ0_OPERATION_MODE::TQS;
    pub use super::MTL_TXQ0_OPERATION_MODE::TSF;
    pub use super::MTL_TXQ0_OPERATION_MODE::TTC;
    pub use super::MTL_TXQ0_OPERATION_MODE::TXQEN;
}

/// Queue 1 Underflow Counter
pub mod MTL_TXQ1_UNDERFLOW {
    pub use super::MTL_TXQ0_UNDERFLOW::UFCNTOVF;
    pub use super::MTL_TXQ0_UNDERFLOW::UFFRMCNT;
}

/// Queue 1 Transmit Debug
pub mod MTL_TXQ1_DEBUG {
    pub use super::MTL_TXQ0_DEBUG::PTXQ;
    pub use super::MTL_TXQ0_DEBUG::STXSTSF;
    pub use super::MTL_TXQ0_DEBUG::TRCSTS;
    pub use super::MTL_TXQ0_DEBUG::TWCSTS;
    pub use super::MTL_TXQ0_DEBUG::TXQPAUSED;
    pub use super::MTL_TXQ0_DEBUG::TXQSTS;
    pub use super::MTL_TXQ0_DEBUG::TXSTSFSTS;
}

/// Queue 1 ETS Control
pub mod MTL_TXQ1_ETS_CONTROL {

    /// AV Algorithm When Queue 1 is programmed for AV, this field configures the scheduling algorithm for this queue: This bit when set, indicates credit based shaper algorithm (CBS) is selected for Queue 1 traffic.
    pub mod AVALG {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: CBS Algorithm is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: CBS Algorithm is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Credit Control When this bit is set, the accumulated credit parameter in the credit-based shaper algorithm logic is not reset to zero when there is positive credit and no packet to transmit in Channel 1.
    pub mod CC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Credit Control is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Credit Control is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Slot Count If the credit-based shaper algorithm is enabled, the software can program the number of slots (of duration programmed in DMA_CH\[n\]_Slot_Interval register) over which the average transmitted bits per slot, provided in the MTL_TXQ\[N\]_ETS_STATUS register, need to be computed for Queue.
    pub mod SLC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 1 slot
            pub const bf_1_SLOT: u32 = 0b000;

            /// 0b001: 2 slots
            pub const bf_2_SLOT: u32 = 0b001;

            /// 0b010: 4 slots
            pub const bf_4_SLOT: u32 = 0b010;

            /// 0b011: 8 slots
            pub const bf_8_SLOT: u32 = 0b011;

            /// 0b100: 16 slots
            pub const bf_16_SLOT: u32 = 0b100;
        }
    }
}

/// Queue 1 ETS Status
pub mod MTL_TXQ1_ETS_STATUS {
    pub use super::MTL_TXQ0_ETS_STATUS::ABS;
}

/// Queue 1 idleSlopeCredit, Quantum or Weights
pub mod MTL_TXQ1_QUANTUM_WEIGHT {
    pub use super::MTL_TXQ0_QUANTUM_WEIGHT::ISCQW;
}

/// Queue 1 sendSlopeCredit
pub mod MTL_TXQ1_SENDSLOPECREDIT {

    /// sendSlopeCredit Value When AV operation is enabled, this field contains the sendSlopeCredit value required for credit-based shaper algorithm for Queue 1.
    pub mod SSC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Queue 1 hiCredit
pub mod MTL_TXQ1_HICREDIT {

    /// hiCredit Value When the AV feature is enabled, this field contains the hiCredit value required for the credit-based shaper algorithm.
    pub mod HC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (29 bits: 0x1fffffff << 0)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Queue 1 loCredit
pub mod MTL_TXQ1_LOCREDIT {

    /// loCredit Value When AV operation is enabled, this field contains the loCredit value required for the credit-based shaper algorithm.
    pub mod LC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (29 bits: 0x1fffffff << 0)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Queue 1 Interrupt Control Status
pub mod MTL_Q1_INTERRUPT_CONTROL_STATUS {
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::ABPSIE;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::ABPSIS;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::RXOIE;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::RXOVFIS;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::TXUIE;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::TXUNFIS;
}

/// Queue 1 Receive Operation Mode
pub mod MTL_RXQ1_OPERATION_MODE {
    pub use super::MTL_RXQ0_OPERATION_MODE::DIS_TCP_EF;
    pub use super::MTL_RXQ0_OPERATION_MODE::EHFC;
    pub use super::MTL_RXQ0_OPERATION_MODE::FEP;
    pub use super::MTL_RXQ0_OPERATION_MODE::FUP;
    pub use super::MTL_RXQ0_OPERATION_MODE::RFA;
    pub use super::MTL_RXQ0_OPERATION_MODE::RFD;
    pub use super::MTL_RXQ0_OPERATION_MODE::RQS;
    pub use super::MTL_RXQ0_OPERATION_MODE::RSF;
    pub use super::MTL_RXQ0_OPERATION_MODE::RTC;
}

/// Queue 1 Missed Packet and Overflow Counter
pub mod MTL_RXQ1_MISSED_PACKET_OVERFLOW_CNT {
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::MISCNTOVF;
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::MISPKTCNT;
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::OVFCNTOVF;
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::OVFPKTCNT;
}

/// Queue 1 Receive Debug
pub mod MTL_RXQ1_DEBUG {
    pub use super::MTL_RXQ0_DEBUG::PRXQ;
    pub use super::MTL_RXQ0_DEBUG::RRCSTS;
    pub use super::MTL_RXQ0_DEBUG::RWCSTS;
    pub use super::MTL_RXQ0_DEBUG::RXQSTS;
}

/// Queue 1 Receive Control
pub mod MTL_RXQ1_CONTROL {
    pub use super::MTL_RXQ0_CONTROL::RXQ_FRM_ARBIT;
    pub use super::MTL_RXQ0_CONTROL::RXQ_WEGT;
}

/// Queue 2 Transmit Operation Mode
pub mod MTL_TXQ2_OPERATION_MODE {
    pub use super::MTL_TXQ0_OPERATION_MODE::FTQ;
    pub use super::MTL_TXQ0_OPERATION_MODE::TQS;
    pub use super::MTL_TXQ0_OPERATION_MODE::TSF;
    pub use super::MTL_TXQ0_OPERATION_MODE::TTC;
    pub use super::MTL_TXQ0_OPERATION_MODE::TXQEN;
}

/// Queue 2 Underflow Counter
pub mod MTL_TXQ2_UNDERFLOW {
    pub use super::MTL_TXQ0_UNDERFLOW::UFCNTOVF;
    pub use super::MTL_TXQ0_UNDERFLOW::UFFRMCNT;
}

/// Queue 2 Transmit Debug
pub mod MTL_TXQ2_DEBUG {
    pub use super::MTL_TXQ0_DEBUG::PTXQ;
    pub use super::MTL_TXQ0_DEBUG::STXSTSF;
    pub use super::MTL_TXQ0_DEBUG::TRCSTS;
    pub use super::MTL_TXQ0_DEBUG::TWCSTS;
    pub use super::MTL_TXQ0_DEBUG::TXQPAUSED;
    pub use super::MTL_TXQ0_DEBUG::TXQSTS;
    pub use super::MTL_TXQ0_DEBUG::TXSTSFSTS;
}

/// Queue 2 ETS Control
pub mod MTL_TXQ2_ETS_CONTROL {
    pub use super::MTL_TXQ1_ETS_CONTROL::AVALG;
    pub use super::MTL_TXQ1_ETS_CONTROL::CC;
    pub use super::MTL_TXQ1_ETS_CONTROL::SLC;
}

/// Queue 2 ETS Status
pub mod MTL_TXQ2_ETS_STATUS {
    pub use super::MTL_TXQ0_ETS_STATUS::ABS;
}

/// Queue 2 idleSlopeCredit, Quantum or Weights
pub mod MTL_TXQ2_QUANTUM_WEIGHT {
    pub use super::MTL_TXQ0_QUANTUM_WEIGHT::ISCQW;
}

/// Queue 2 sendSlopeCredit
pub mod MTL_TXQ2_SENDSLOPECREDIT {
    pub use super::MTL_TXQ1_SENDSLOPECREDIT::SSC;
}

/// Queue 2 hiCredit
pub mod MTL_TXQ2_HICREDIT {
    pub use super::MTL_TXQ1_HICREDIT::HC;
}

/// Queue 2 loCredit
pub mod MTL_TXQ2_LOCREDIT {
    pub use super::MTL_TXQ1_LOCREDIT::LC;
}

/// Queue 2 Interrupt Control Status
pub mod MTL_Q2_INTERRUPT_CONTROL_STATUS {
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::ABPSIE;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::ABPSIS;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::RXOIE;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::RXOVFIS;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::TXUIE;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::TXUNFIS;
}

/// Queue 2 Receive Operation Mode
pub mod MTL_RXQ2_OPERATION_MODE {
    pub use super::MTL_RXQ0_OPERATION_MODE::DIS_TCP_EF;
    pub use super::MTL_RXQ0_OPERATION_MODE::EHFC;
    pub use super::MTL_RXQ0_OPERATION_MODE::FEP;
    pub use super::MTL_RXQ0_OPERATION_MODE::FUP;
    pub use super::MTL_RXQ0_OPERATION_MODE::RFA;
    pub use super::MTL_RXQ0_OPERATION_MODE::RFD;
    pub use super::MTL_RXQ0_OPERATION_MODE::RQS;
    pub use super::MTL_RXQ0_OPERATION_MODE::RSF;
    pub use super::MTL_RXQ0_OPERATION_MODE::RTC;
}

/// Queue 2 Missed Packet and Overflow Counter
pub mod MTL_RXQ2_MISSED_PACKET_OVERFLOW_CNT {
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::MISCNTOVF;
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::MISPKTCNT;
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::OVFCNTOVF;
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::OVFPKTCNT;
}

/// Queue 2 Receive Debug
pub mod MTL_RXQ2_DEBUG {
    pub use super::MTL_RXQ0_DEBUG::PRXQ;
    pub use super::MTL_RXQ0_DEBUG::RRCSTS;
    pub use super::MTL_RXQ0_DEBUG::RWCSTS;
    pub use super::MTL_RXQ0_DEBUG::RXQSTS;
}

/// Queue 2 Receive Control
pub mod MTL_RXQ2_CONTROL {
    pub use super::MTL_RXQ0_CONTROL::RXQ_FRM_ARBIT;
    pub use super::MTL_RXQ0_CONTROL::RXQ_WEGT;
}

/// Queue 3 Transmit Operation Mode
pub mod MTL_TXQ3_OPERATION_MODE {
    pub use super::MTL_TXQ0_OPERATION_MODE::FTQ;
    pub use super::MTL_TXQ0_OPERATION_MODE::TQS;
    pub use super::MTL_TXQ0_OPERATION_MODE::TSF;
    pub use super::MTL_TXQ0_OPERATION_MODE::TTC;
    pub use super::MTL_TXQ0_OPERATION_MODE::TXQEN;
}

/// Queue 3 Underflow Counter
pub mod MTL_TXQ3_UNDERFLOW {
    pub use super::MTL_TXQ0_UNDERFLOW::UFCNTOVF;
    pub use super::MTL_TXQ0_UNDERFLOW::UFFRMCNT;
}

/// Queue 3 Transmit Debug
pub mod MTL_TXQ3_DEBUG {
    pub use super::MTL_TXQ0_DEBUG::PTXQ;
    pub use super::MTL_TXQ0_DEBUG::STXSTSF;
    pub use super::MTL_TXQ0_DEBUG::TRCSTS;
    pub use super::MTL_TXQ0_DEBUG::TWCSTS;
    pub use super::MTL_TXQ0_DEBUG::TXQPAUSED;
    pub use super::MTL_TXQ0_DEBUG::TXQSTS;
    pub use super::MTL_TXQ0_DEBUG::TXSTSFSTS;
}

/// Queue 3 ETS Control
pub mod MTL_TXQ3_ETS_CONTROL {
    pub use super::MTL_TXQ1_ETS_CONTROL::AVALG;
    pub use super::MTL_TXQ1_ETS_CONTROL::CC;
    pub use super::MTL_TXQ1_ETS_CONTROL::SLC;
}

/// Queue 3 ETS Status
pub mod MTL_TXQ3_ETS_STATUS {
    pub use super::MTL_TXQ0_ETS_STATUS::ABS;
}

/// Queue 3 idleSlopeCredit, Quantum or Weights
pub mod MTL_TXQ3_QUANTUM_WEIGHT {
    pub use super::MTL_TXQ0_QUANTUM_WEIGHT::ISCQW;
}

/// Queue 3 sendSlopeCredit
pub mod MTL_TXQ3_SENDSLOPECREDIT {
    pub use super::MTL_TXQ1_SENDSLOPECREDIT::SSC;
}

/// Queue 3 hiCredit
pub mod MTL_TXQ3_HICREDIT {
    pub use super::MTL_TXQ1_HICREDIT::HC;
}

/// Queue 3 loCredit
pub mod MTL_TXQ3_LOCREDIT {
    pub use super::MTL_TXQ1_LOCREDIT::LC;
}

/// Queue 3 Interrupt Control Status
pub mod MTL_Q3_INTERRUPT_CONTROL_STATUS {
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::ABPSIE;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::ABPSIS;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::RXOIE;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::RXOVFIS;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::TXUIE;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::TXUNFIS;
}

/// Queue 3 Receive Operation Mode
pub mod MTL_RXQ3_OPERATION_MODE {
    pub use super::MTL_RXQ0_OPERATION_MODE::DIS_TCP_EF;
    pub use super::MTL_RXQ0_OPERATION_MODE::EHFC;
    pub use super::MTL_RXQ0_OPERATION_MODE::FEP;
    pub use super::MTL_RXQ0_OPERATION_MODE::FUP;
    pub use super::MTL_RXQ0_OPERATION_MODE::RFA;
    pub use super::MTL_RXQ0_OPERATION_MODE::RFD;
    pub use super::MTL_RXQ0_OPERATION_MODE::RQS;
    pub use super::MTL_RXQ0_OPERATION_MODE::RSF;
    pub use super::MTL_RXQ0_OPERATION_MODE::RTC;
}

/// Queue 3 Missed Packet and Overflow Counter
pub mod MTL_RXQ3_MISSED_PACKET_OVERFLOW_CNT {
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::MISCNTOVF;
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::MISPKTCNT;
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::OVFCNTOVF;
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::OVFPKTCNT;
}

/// Queue 3 Receive Debug
pub mod MTL_RXQ3_DEBUG {
    pub use super::MTL_RXQ0_DEBUG::PRXQ;
    pub use super::MTL_RXQ0_DEBUG::RRCSTS;
    pub use super::MTL_RXQ0_DEBUG::RWCSTS;
    pub use super::MTL_RXQ0_DEBUG::RXQSTS;
}

/// Queue 3 Receive Control
pub mod MTL_RXQ3_CONTROL {
    pub use super::MTL_RXQ0_CONTROL::RXQ_FRM_ARBIT;
    pub use super::MTL_RXQ0_CONTROL::RXQ_WEGT;
}

/// Queue 4 Transmit Operation Mode
pub mod MTL_TXQ4_OPERATION_MODE {
    pub use super::MTL_TXQ0_OPERATION_MODE::FTQ;
    pub use super::MTL_TXQ0_OPERATION_MODE::TQS;
    pub use super::MTL_TXQ0_OPERATION_MODE::TSF;
    pub use super::MTL_TXQ0_OPERATION_MODE::TTC;
    pub use super::MTL_TXQ0_OPERATION_MODE::TXQEN;
}

/// Queue 4 Underflow Counter
pub mod MTL_TXQ4_UNDERFLOW {
    pub use super::MTL_TXQ0_UNDERFLOW::UFCNTOVF;
    pub use super::MTL_TXQ0_UNDERFLOW::UFFRMCNT;
}

/// Queue 4 Transmit Debug
pub mod MTL_TXQ4_DEBUG {
    pub use super::MTL_TXQ0_DEBUG::PTXQ;
    pub use super::MTL_TXQ0_DEBUG::STXSTSF;
    pub use super::MTL_TXQ0_DEBUG::TRCSTS;
    pub use super::MTL_TXQ0_DEBUG::TWCSTS;
    pub use super::MTL_TXQ0_DEBUG::TXQPAUSED;
    pub use super::MTL_TXQ0_DEBUG::TXQSTS;
    pub use super::MTL_TXQ0_DEBUG::TXSTSFSTS;
}

/// Queue 4 ETS Control
pub mod MTL_TXQ4_ETS_CONTROL {
    pub use super::MTL_TXQ1_ETS_CONTROL::AVALG;
    pub use super::MTL_TXQ1_ETS_CONTROL::CC;
    pub use super::MTL_TXQ1_ETS_CONTROL::SLC;
}

/// Queue 4 ETS Status
pub mod MTL_TXQ4_ETS_STATUS {
    pub use super::MTL_TXQ0_ETS_STATUS::ABS;
}

/// Queue 4 idleSlopeCredit, Quantum or Weights
pub mod MTL_TXQ4_QUANTUM_WEIGHT {
    pub use super::MTL_TXQ0_QUANTUM_WEIGHT::ISCQW;
}

/// Queue 4 sendSlopeCredit
pub mod MTL_TXQ4_SENDSLOPECREDIT {
    pub use super::MTL_TXQ1_SENDSLOPECREDIT::SSC;
}

/// Queue 4 hiCredit
pub mod MTL_TXQ4_HICREDIT {
    pub use super::MTL_TXQ1_HICREDIT::HC;
}

/// Queue 4 loCredit
pub mod MTL_TXQ4_LOCREDIT {
    pub use super::MTL_TXQ1_LOCREDIT::LC;
}

/// Queue 4 Interrupt Control Status
pub mod MTL_Q4_INTERRUPT_CONTROL_STATUS {
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::ABPSIE;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::ABPSIS;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::RXOIE;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::RXOVFIS;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::TXUIE;
    pub use super::MTL_Q0_INTERRUPT_CONTROL_STATUS::TXUNFIS;
}

/// Queue 4 Receive Operation Mode
pub mod MTL_RXQ4_OPERATION_MODE {
    pub use super::MTL_RXQ0_OPERATION_MODE::DIS_TCP_EF;
    pub use super::MTL_RXQ0_OPERATION_MODE::EHFC;
    pub use super::MTL_RXQ0_OPERATION_MODE::FEP;
    pub use super::MTL_RXQ0_OPERATION_MODE::FUP;
    pub use super::MTL_RXQ0_OPERATION_MODE::RFA;
    pub use super::MTL_RXQ0_OPERATION_MODE::RFD;
    pub use super::MTL_RXQ0_OPERATION_MODE::RQS;
    pub use super::MTL_RXQ0_OPERATION_MODE::RSF;
    pub use super::MTL_RXQ0_OPERATION_MODE::RTC;
}

/// Queue 4 Missed Packet and Overflow Counter
pub mod MTL_RXQ4_MISSED_PACKET_OVERFLOW_CNT {
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::MISCNTOVF;
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::MISPKTCNT;
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::OVFCNTOVF;
    pub use super::MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT::OVFPKTCNT;
}

/// Queue 4 Receive Debug
pub mod MTL_RXQ4_DEBUG {
    pub use super::MTL_RXQ0_DEBUG::PRXQ;
    pub use super::MTL_RXQ0_DEBUG::RRCSTS;
    pub use super::MTL_RXQ0_DEBUG::RWCSTS;
    pub use super::MTL_RXQ0_DEBUG::RXQSTS;
}

/// Queue 4 Receive Control
pub mod MTL_RXQ4_CONTROL {
    pub use super::MTL_RXQ0_CONTROL::RXQ_FRM_ARBIT;
    pub use super::MTL_RXQ0_CONTROL::RXQ_WEGT;
}

/// DMA Bus Mode
pub mod DMA_MODE {

    /// Software Reset When this bit is set, the MAC and the DMA controller reset the logic and all internal registers of the DMA, MTL, and MAC.
    pub mod SWR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Software Reset is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Software Reset is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Descriptor Posted Write When this bit is set to 0, the descriptor writes are always non-posted.
    pub mod DSPW {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Descriptor Posted Write is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Descriptor Posted Write is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Interrupt Mode This field defines the interrupt mode of DWC_ether_qos.
    pub mod INTM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: See above description
            pub const MODE0: u32 = 0b00;

            /// 0b01: See above description
            pub const MODE1: u32 = 0b01;

            /// 0b10: See above description
            pub const MODE2: u32 = 0b10;
        }
    }
}

/// DMA System Bus Mode
pub mod DMA_SYSBUS_MODE {

    /// Fixed Burst Length When this bit is set to 1, the EQOS-AXI master initiates burst transfers of specified lengths as given below.
    pub mod FB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Fixed Burst Length is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Fixed Burst Length is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// AXI Burst Length 4 When this bit is set to 1 or the FB bit is set to 0, the EQOS-AXI master can select a burst length of 4 on the AXI interface.
    pub mod BLEN4 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const DISABLE: u32 = 0b0;

            /// 0b1: AXI Burst Length 4
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// AXI Burst Length 8 When this bit is set to 1 or the FB bit is set to 0, the EQOS-AXI master can select a burst length of 8 on the AXI interface.
    pub mod BLEN8 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const DISABLE: u32 = 0b0;

            /// 0b1: AXI Burst Length 8
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// AXI Burst Length 16 When this bit is set to 1 or the FB bit is set to 0, the EQOS-AXI master can select a burst length of 16 on the AXI interface.
    pub mod BLEN16 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const DISABLE: u32 = 0b0;

            /// 0b1: AXI Burst Length 16
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Automatic AXI LPI enable When set to 1, enables the AXI master to enter into LPI state when there is no activity in the DWC_ether_qos for number of system clock cycles programmed in the LPIEI field of DMA_AXI_LPI_ENTRY_INTERVAL register.
    pub mod AALE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Automatic AXI LPI is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Automatic AXI LPI is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Address-Aligned Beats When this bit is set to 1, the EQOS-AXI or EQOS-AHB master performs address-aligned burst transfers on Read and Write channels.
    pub mod AAL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Address-Aligned Beats is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Address-Aligned Beats is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// 1 KB Boundary Crossing Enable for the EQOS-AXI Master When set, the burst transfers performed by the EQOS-AXI master do not cross 1 KB boundary.
    pub mod ONEKBBE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 1 KB Boundary Crossing for the EQOS-AXI Master Beats is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: 1 KB Boundary Crossing for the EQOS-AXI Master Beats is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// AXI Maximum Read Outstanding Request Limit This value limits the maximum outstanding request on the AXI read interface.
    pub mod RD_OSR_LMT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AXI Maximum Write Outstanding Request Limit This value limits the maximum outstanding request on the AXI write interface.
    pub mod WR_OSR_LMT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Unlock on Magic Packet or Remote Wake-Up Packet When set to 1, this bit enables the AXI master to come out of the LPI mode only when the magic packet or remote wake-up packet is received.
    pub mod LPI_XIT_PKT {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Unlock on Magic Packet or Remote Wake-Up Packet is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Unlock on Magic Packet or Remote Wake-Up Packet is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable Low Power Interface (LPI) When set to 1, this bit enables the LPI mode supported by the EQOS-AXI configuration and accepts the LPI request from the AXI System Clock controller.
    pub mod EN_LPI {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Low Power Interface (LPI) is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Low Power Interface (LPI) is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// DMA Interrupt Status
pub mod DMA_INTERRUPT_STATUS {

    /// DMA Channel 0 Interrupt Status This bit indicates an interrupt event in DMA Channel 0.
    pub mod DC0IS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel 0 Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: DMA Channel 0 Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// DMA Channel 1 Interrupt Status This bit indicates an interrupt event in DMA Channel 1.
    pub mod DC1IS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel 1 Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: DMA Channel 1 Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// DMA Channel 2 Interrupt Status This bit indicates an interrupt event in DMA Channel 2.
    pub mod DC2IS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel 2 Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: DMA Channel 2 Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// DMA Channel 3 Interrupt Status This bit indicates an interrupt event in DMA Channel 3.
    pub mod DC3IS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel 3 Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: DMA Channel 3 Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// DMA Channel 4 Interrupt Status This bit indicates an interrupt event in DMA Channel 4.
    pub mod DC4IS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA Channel 4 Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: DMA Channel 4 Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MTL Interrupt Status This bit indicates an interrupt event in the MTL.
    pub mod MTLIS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MTL Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MTL Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// MAC Interrupt Status This bit indicates an interrupt event in the MAC.
    pub mod MACIS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: MAC Interrupt Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: MAC Interrupt Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// DMA Debug Status 0
pub mod DMA_DEBUG_STATUS0 {

    /// AXI Master Write Channel When high, this bit indicates that the write channel of the AXI master is active, and it is transferring data.
    pub mod AXWHSTS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: AXI Master Write Channel or AHB Master Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: AXI Master Write Channel or AHB Master Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// AXI Master Read Channel Status When high, this bit indicates that the read channel of the AXI master is active, and it is transferring the data.
    pub mod AXRHSTS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: AXI Master Read Channel Status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: AXI Master Read Channel Status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// DMA Channel 0 Receive Process State This field indicates the Rx DMA FSM state for Channel 0.
    pub mod RPS0 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Stopped (Reset or Stop Receive Command issued)
            pub const STOP: u32 = 0b0000;

            /// 0b0001: Running (Fetching Rx Transfer Descriptor)
            pub const RUN_FRTD: u32 = 0b0001;

            /// 0b0011: Running (Waiting for Rx packet)
            pub const RUN_WRP: u32 = 0b0011;

            /// 0b0100: Suspended (Rx Descriptor Unavailable)
            pub const SUSPND: u32 = 0b0100;

            /// 0b0101: Running (Closing the Rx Descriptor)
            pub const RUN_CRD: u32 = 0b0101;

            /// 0b0110: Timestamp write state
            pub const TSTMP: u32 = 0b0110;

            /// 0b0111: Running (Transferring the received packet data from the Rx buffer to the system memory)
            pub const RUN_TRP: u32 = 0b0111;
        }
    }

    /// DMA Channel 0 Transmit Process State This field indicates the Tx DMA FSM state for Channel 0.
    pub mod TPS0 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Stopped (Reset or Stop Transmit Command issued)
            pub const STOP: u32 = 0b0000;

            /// 0b0001: Running (Fetching Tx Transfer Descriptor)
            pub const RUN_FTTD: u32 = 0b0001;

            /// 0b0010: Running (Waiting for status)
            pub const RUN_WS: u32 = 0b0010;

            /// 0b0011: Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))
            pub const RUN_RDS: u32 = 0b0011;

            /// 0b0100: Timestamp write state
            pub const TSTMP_WS: u32 = 0b0100;

            /// 0b0110: Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)
            pub const SUSPND: u32 = 0b0110;

            /// 0b0111: Running (Closing Tx Descriptor)
            pub const RUN_CTD: u32 = 0b0111;
        }
    }

    /// DMA Channel 1 Receive Process State This field indicates the Rx DMA FSM state for Channel 1.
    pub mod RPS1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RPS0::RW;
    }

    /// DMA Channel 1 Transmit Process State This field indicates the Tx DMA FSM state for Channel 1.
    pub mod TPS1 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TPS0::RW;
    }

    /// DMA Channel 2 Receive Process State This field indicates the Rx DMA FSM state for Channel 2.
    pub mod RPS2 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RPS0::RW;
    }

    /// DMA Channel 2 Transmit Process State This field indicates the Tx DMA FSM state for Channel 2.
    pub mod TPS2 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TPS0::RW;
    }
}

/// DMA Debug Status 1
pub mod DMA_DEBUG_STATUS1 {

    /// DMA Channel 3 Receive Process State This field indicates the Rx DMA FSM state for Channel 3.
    pub mod RPS3 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Stopped (Reset or Stop Receive Command issued)
            pub const STOP: u32 = 0b0000;

            /// 0b0001: Running (Fetching Rx Transfer Descriptor)
            pub const RUN_FRTD: u32 = 0b0001;

            /// 0b0011: Running (Waiting for Rx packet)
            pub const RUN_WRP: u32 = 0b0011;

            /// 0b0100: Suspended (Rx Descriptor Unavailable)
            pub const SUSPND: u32 = 0b0100;

            /// 0b0101: Running (Closing the Rx Descriptor)
            pub const RUN_CRD: u32 = 0b0101;

            /// 0b0110: Timestamp write state
            pub const TSTMP: u32 = 0b0110;

            /// 0b0111: Running (Transferring the received packet data from the Rx buffer to the system memory)
            pub const RUN_TRP: u32 = 0b0111;
        }
    }

    /// DMA Channel 3 Transmit Process State This field indicates the Tx DMA FSM state for Channel 3.
    pub mod TPS3 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Stopped (Reset or Stop Transmit Command issued)
            pub const STOP: u32 = 0b0000;

            /// 0b0001: Running (Fetching Tx Transfer Descriptor)
            pub const RUN_FTTD: u32 = 0b0001;

            /// 0b0010: Running (Waiting for status)
            pub const RUN_WS: u32 = 0b0010;

            /// 0b0011: Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))
            pub const RUN_RDS: u32 = 0b0011;

            /// 0b0100: Timestamp write state
            pub const TSTMP_WS: u32 = 0b0100;

            /// 0b0110: Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)
            pub const SUSPND: u32 = 0b0110;

            /// 0b0111: Running (Closing Tx Descriptor)
            pub const RUN_CTD: u32 = 0b0111;
        }
    }

    /// DMA Channel 4 Receive Process State This field indicates the Rx DMA FSM state for Channel 4.
    pub mod RPS4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::RPS3::RW;
    }

    /// DMA Channel 4 Transmit Process State This field indicates the Tx DMA FSM state for Channel 4.
    pub mod TPS4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TPS3::RW;
    }
}

/// AXI LPI Entry Interval Control
pub mod DMA_AXI_LPI_ENTRY_INTERVAL {

    /// LPI Entry Interval Contains the number of system clock cycles, multiplied by 64, to wait for an activity in the DWC_ether_qos to enter into the AXI low power state 0 indicates 64 clock cycles
    pub mod LPIEI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TBS Control
pub mod DMA_TBS_CTRL {

    /// Fetch Time Offset Valid When set indicates the FTOS field is valid.
    pub mod FTOV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Fetch Time Offset is invalid
            pub const INVALID: u32 = 0b0;

            /// 0b1: Fetch Time Offset is valid
            pub const VALID: u32 = 0b1;
        }
    }

    /// Fetch GSN Offset The number GSN slots that must be deducted from the Launch GSN to compute the Fetch GSN.
    pub mod FGOS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fetch Time Offset The value in units of 256 nanoseconds, that has to be deducted from the Launch time to compute the Fetch Time.
    pub mod FTOS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA Channel 0 Control
pub mod DMA_CH0_CONTROL {

    /// 8xPBL mode When this bit is set, the PBL value programmed in Bits\[21:16\] in DMA_CH0_TX_CONTROL and Bits\[21:16\] in DMA_CH0_RX_CONTROL is multiplied by eight times.
    pub mod PBLx8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 8xPBL mode is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: 8xPBL mode is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Descriptor Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32-bit, 64-bit, or 128-bit bus) to skip between two unchained descriptors.
    pub mod DSL {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA Channel 0 Transmit Control
pub mod DMA_CH0_TX_CONTROL {

    /// Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state.
    pub mod ST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Stop Transmission Command
            pub const STOP: u32 = 0b0;

            /// 0b1: Start Transmission Command
            pub const START: u32 = 0b1;
        }
    }

    /// Operate on Second Packet When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained.
    pub mod OSF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Operate on Second Packet disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Operate on Second Packet enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Ignore PBL Requirement When this bit is set, the DMA does not check for PBL number of locations in the MTL before initiating a transfer.
    pub mod IPBL {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Ignore PBL Requirement is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Ignore PBL Requirement is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA block data transfer.
    pub mod TxPBL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enhanced Descriptor Enable When this bit is set, the corresponding channel uses Enhanced Descriptors that are 32 Bytes for both Normal and Context Descriptors.
    pub mod EDSE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enhanced Descriptor is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enhanced Descriptor is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// DMA Channel 0 Receive Control
pub mod DMA_CH0_RX_CONTROL {

    /// Start or Stop Receive When this bit is set, the DMA tries to acquire the descriptor from the Receive list and processes the incoming packets.
    pub mod SR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Stop Receive
            pub const STOP: u32 = 0b0;

            /// 0b1: Start Receive
            pub const START: u32 = 0b1;
        }
    }

    /// Receive Buffer size Low RBSZ\[13:0\] is split into two fields RBSZ_13_y and RBSZ_x_0.
    pub mod RBSZ_x_0 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive Buffer size High RBSZ\[13:0\] is split into two fields higher RBSZ_13_y and lower RBSZ_x_0.
    pub mod RBSZ_13_y {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (11 bits: 0x7ff << 4)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA block data transfer.
    pub mod RxPBL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rx Packet Flush.
    pub mod RPF {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Rx Packet Flush is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Rx Packet Flush is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Channel 0 Tx Descriptor List Address register
pub mod DMA_CH0_TXDESC_LIST_ADDRESS {

    /// Start of Transmit List This field contains the base address of the first descriptor in the Transmit descriptor list.
    pub mod TDESLA {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 0 Rx Descriptor List Address register
pub mod DMA_CH0_RXDESC_LIST_ADDRESS {

    /// Start of Receive List This field contains the base address of the first descriptor in the Rx Descriptor list.
    pub mod RDESLA {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 0 Tx Descriptor Tail Pointer
pub mod DMA_CH0_TXDESC_TAIL_POINTER {

    /// Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring.
    pub mod TDTP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 0 Rx Descriptor Tail Pointer
pub mod DMA_CH0_RXDESC_TAIL_POINTER {

    /// Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring.
    pub mod RDTP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 0 Tx Descriptor Ring Length
pub mod DMA_CH0_TXDESC_RING_LENGTH {

    /// Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring.
    pub mod TDRL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 0 Rx Descriptor Ring Length
pub mod DMA_CH0_RXDESC_RING_LENGTH {

    /// Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring.
    pub mod RDRL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 0 Interrupt Enable
pub mod DMA_CH0_INTERRUPT_ENABLE {

    /// Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled.
    pub mod TIE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmit Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled.
    pub mod TXSE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Stopped is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmit Stopped is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled.
    pub mod TBUE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Buffer Unavailable is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Transmit Buffer Unavailable is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled.
    pub mod RIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled.
    pub mod RBUE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Buffer Unavailable is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive Buffer Unavailable is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled.
    pub mod RSE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Stopped is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive Stopped is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled.
    pub mod RWTE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Watchdog Timeout is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Receive Watchdog Timeout is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled.
    pub mod ETIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Early Transmit Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Early Transmit Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled.
    pub mod ERIE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Early Receive Interrupt is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Early Receive Interrupt is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled.
    pub mod FBEE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Fatal Bus Error is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Fatal Bus Error is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Context Descriptor Error Enable When this bit is set along with the AIE bit, the Descriptor error interrupt is enabled.
    pub mod CDEE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Context Descriptor Error is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Context Descriptor Error is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled.
    pub mod AIE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Abnormal Interrupt Summary is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Abnormal Interrupt Summary is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled.
    pub mod NIE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal Interrupt Summary is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Normal Interrupt Summary is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Channel 0 Receive Interrupt Watchdog Timer
pub mod DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER {

    /// Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set.
    pub mod RWT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive Interrupt Watchdog Timer Count Units This fields indicates the number of system clock cycles corresponding to one unit in RWT field.
    pub mod RWTU {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 0 Slot Function Control and Status
pub mod DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS {

    /// Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field.
    pub mod ESC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slot Comparison is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Slot Comparison is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Advance Slot Check When set, this bit enables the DMA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is - equal to the reference slot number given in the RSN field or - ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set.
    pub mod ASC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Advance Slot Check is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Advance Slot Check is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Slot Interval Value This field controls the period of the slot interval in which the TxDMA fetches the scheduled packets.
    pub mod SIV {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reference Slot Number This field gives the current value of the reference slot number in the DMA.
    pub mod RSN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 0 Current Application Transmit Descriptor
pub mod DMA_CH0_CURRENT_APP_TXDESC {

    /// Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation.
    pub mod CURTDESAPTR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 0 Current Application Receive Descriptor
pub mod DMA_CH0_CURRENT_APP_RXDESC {

    /// Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation.
    pub mod CURRDESAPTR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 0 Current Application Transmit Buffer Address
pub mod DMA_CH0_CURRENT_APP_TXBUFFER {

    /// Application Transmit Buffer Address Pointer The DMA updates this pointer during Tx operation.
    pub mod CURTBUFAPTR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 0 Current Application Receive Buffer Address
pub mod DMA_CH0_CURRENT_APP_RXBUFFER {

    /// Application Receive Buffer Address Pointer The DMA updates this pointer during Rx operation.
    pub mod CURRBUFAPTR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA Channel 0 Status
pub mod DMA_CH0_STATUS {

    /// Transmit Interrupt This bit indicates that the packet transmission is complete.
    pub mod TI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Transmit Process Stopped This bit is set when the transmission is stopped.
    pub mod TPS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Process Stopped status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit Process Stopped status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the Transmit list, and the DMA cannot acquire it.
    pub mod TBU {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Buffer Unavailable status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Transmit Buffer Unavailable status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Receive Interrupt This bit indicates that the packet reception is complete.
    pub mod RI {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Receive Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Receive Buffer Unavailable This bit indicates that the application owns the next descriptor in the Receive list, and the DMA cannot acquire it.
    pub mod RBU {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Buffer Unavailable status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Receive Buffer Unavailable status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state.
    pub mod RPS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Process Stopped status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Receive Process Stopped status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Receive Watchdog Timeout This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received.
    pub mod RWT {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Watchdog Timeout status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Receive Watchdog Timeout status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Early Transmit Interrupt This bit when set indicates that the TxDMA has completed the transfer of packet data to the MTL TXFIFO memory.
    pub mod ETI {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Early Transmit Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Early Transmit Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Early Receive Interrupt This bit when set indicates that the RxDMA has completed the transfer of packet data to the memory.
    pub mod ERI {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Early Receive Interrupt status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Early Receive Interrupt status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field).
    pub mod FBE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Fatal Bus Error status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Fatal Bus Error status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Context Descriptor Error This bit indicates that the DMA Tx/Rx engine received a descriptor error, which indicates invalid context in the middle of packet flow (intermediate descriptor) or all ones descriptor in Tx case and on Rx side it indicates DMA has read a descriptor with either of the buffer address as ones which is considered to be invalid.
    pub mod CDE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Context Descriptor Error status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Context Descriptor Error status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the INTERRUPT_ENABLE register: - Bit 1: Transmit Process Stopped - Bit 7: Receive Buffer Unavailable - Bit 8: Receive Process Stopped - Bit 10: Early Transmit Interrupt - Bit 12: Fatal Bus Error - Bit 13: Context Descriptor Error Only unmasked bits affect the Abnormal Interrupt Summary bit.
    pub mod AIS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Abnormal Interrupt Summary status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Abnormal Interrupt Summary status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the DMA_CH0_INTERRUPT_ENABLE register: - Bit 0: Transmit Interrupt - Bit 2: Transmit Buffer Unavailable - Bit 6: Receive Interrupt - Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in INTERRUPT_ENABLE register) affect the Normal Interrupt Summary bit.
    pub mod NIS {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal Interrupt Summary status not detected
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Normal Interrupt Summary status detected
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Tx DMA Error Bits This field indicates the type of error that caused a Bus Error.
    pub mod TEB {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rx DMA Error Bits This field indicates the type of error that caused a Bus Error.
    pub mod REB {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (3 bits: 0b111 << 19)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 0 Missed Frame Counter
pub mod DMA_CH0_MISS_FRAME_CNT {

    /// Dropped Packet Counters This counter indicates the number of packet counters that are dropped by the DMA either because of bus error or because of programming RPF field in DMA_CH0_RX_CONTROL register.
    pub mod MFC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overflow status of the MFC Counter When this bit is set then the MFC counter does not get incremented further.
    pub mod MFCO {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Miss Frame Counter overflow not occurred
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Miss Frame Counter overflow occurred
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// Channel 0 RXP Frames Accepted Counter
pub mod DMA_CH0_RXP_ACCEPT_CNT {

    /// Rx Parser Accept Counter This 31-bit counter is implemented whenever a Rx Parser Accept a packet due to AF =1.
    pub mod RXPAC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rx Parser Accept Counter Overflow Bit When set, this bit indicates that the RXPAC Counter field crossed the maximum limit.
    pub mod RXPACOF {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Rx Parser Accept Counter overflow not occurred
            pub const INACTIVE: u32 = 0b0;

            /// 0b1: Rx Parser Accept Counter overflow occurred
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// Channel 0 Receive ERI Counter
pub mod DMA_CH0_RX_ERI_CNT {

    /// ERI Counter When ERIC bit of RX_CONTROL register is set, this counter increments for burst transfer completed by the Rx DMA from the start of packet transfer.
    pub mod ECNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA Channel 1 Control
pub mod DMA_CH1_CONTROL {
    pub use super::DMA_CH0_CONTROL::PBLx8;
    pub use super::DMA_CH0_CONTROL::DSL;
}

/// DMA Channel 1 Transmit Control
pub mod DMA_CH1_TX_CONTROL {
    pub use super::DMA_CH0_TX_CONTROL::TxPBL;
    pub use super::DMA_CH0_TX_CONTROL::EDSE;
    pub use super::DMA_CH0_TX_CONTROL::IPBL;
    pub use super::DMA_CH0_TX_CONTROL::OSF;
    pub use super::DMA_CH0_TX_CONTROL::ST;
}

/// DMA Channel 1 Receive Control
pub mod DMA_CH1_RX_CONTROL {
    pub use super::DMA_CH0_RX_CONTROL::RBSZ_13_y;
    pub use super::DMA_CH0_RX_CONTROL::RBSZ_x_0;
    pub use super::DMA_CH0_RX_CONTROL::RxPBL;
    pub use super::DMA_CH0_RX_CONTROL::RPF;
    pub use super::DMA_CH0_RX_CONTROL::SR;
}

/// Channel 1 Tx Descriptor List Address
pub mod DMA_CH1_TXDESC_LIST_ADDRESS {
    pub use super::DMA_CH0_TXDESC_LIST_ADDRESS::TDESLA;
}

/// Channel 1 Rx Descriptor List Address
pub mod DMA_CH1_RXDESC_LIST_ADDRESS {
    pub use super::DMA_CH0_RXDESC_LIST_ADDRESS::RDESLA;
}

/// Channel 1 Tx Descriptor Tail Pointer
pub mod DMA_CH1_TXDESC_TAIL_POINTER {
    pub use super::DMA_CH0_TXDESC_TAIL_POINTER::TDTP;
}

/// Channel 1 Rx Descriptor Tail Pointer
pub mod DMA_CH1_RXDESC_TAIL_POINTER {
    pub use super::DMA_CH0_RXDESC_TAIL_POINTER::RDTP;
}

/// Channel 1 Tx Descriptor Ring Length
pub mod DMA_CH1_TXDESC_RING_LENGTH {
    pub use super::DMA_CH0_TXDESC_RING_LENGTH::TDRL;
}

/// Channel 1 Rx Descriptor Ring Length
pub mod DMA_CH1_RXDESC_RING_LENGTH {
    pub use super::DMA_CH0_RXDESC_RING_LENGTH::RDRL;
}

/// Channel 1 Interrupt Enable
pub mod DMA_CH1_INTERRUPT_ENABLE {
    pub use super::DMA_CH0_INTERRUPT_ENABLE::AIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::CDEE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::ERIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::ETIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::FBEE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::NIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RBUE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RSE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RWTE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::TBUE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::TIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::TXSE;
}

/// Channel 1 Receive Interrupt Watchdog Timer
pub mod DMA_CH1_RX_INTERRUPT_WATCHDOG_TIMER {
    pub use super::DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER::RWT;
    pub use super::DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER::RWTU;
}

/// Channel 1 Slot Function Control and Status
pub mod DMA_CH1_SLOT_FUNCTION_CONTROL_STATUS {
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::ASC;
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::ESC;
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::RSN;
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::SIV;
}

/// Channel 1 Current Application Transmit Descriptor
pub mod DMA_CH1_CURRENT_APP_TXDESC {
    pub use super::DMA_CH0_CURRENT_APP_TXDESC::CURTDESAPTR;
}

/// Channel 1 Current Application Receive Descriptor
pub mod DMA_CH1_CURRENT_APP_RXDESC {
    pub use super::DMA_CH0_CURRENT_APP_RXDESC::CURRDESAPTR;
}

/// Channel 1 Current Application Transmit Buffer Address
pub mod DMA_CH1_CURRENT_APP_TXBUFFER {
    pub use super::DMA_CH0_CURRENT_APP_TXBUFFER::CURTBUFAPTR;
}

/// Channel 1 Current Application Receive Buffer Address
pub mod DMA_CH1_CURRENT_APP_RXBUFFER {
    pub use super::DMA_CH0_CURRENT_APP_RXBUFFER::CURRBUFAPTR;
}

/// DMA Channel 1 Status
pub mod DMA_CH1_STATUS {
    pub use super::DMA_CH0_STATUS::AIS;
    pub use super::DMA_CH0_STATUS::CDE;
    pub use super::DMA_CH0_STATUS::ERI;
    pub use super::DMA_CH0_STATUS::ETI;
    pub use super::DMA_CH0_STATUS::FBE;
    pub use super::DMA_CH0_STATUS::NIS;
    pub use super::DMA_CH0_STATUS::RBU;
    pub use super::DMA_CH0_STATUS::REB;
    pub use super::DMA_CH0_STATUS::RI;
    pub use super::DMA_CH0_STATUS::RPS;
    pub use super::DMA_CH0_STATUS::RWT;
    pub use super::DMA_CH0_STATUS::TBU;
    pub use super::DMA_CH0_STATUS::TEB;
    pub use super::DMA_CH0_STATUS::TI;
    pub use super::DMA_CH0_STATUS::TPS;
}

/// Channel 1 Missed Frame Counter
pub mod DMA_CH1_MISS_FRAME_CNT {
    pub use super::DMA_CH0_MISS_FRAME_CNT::MFC;
    pub use super::DMA_CH0_MISS_FRAME_CNT::MFCO;
}

/// Channel 1 RXP Frames Accepted Counter
pub mod DMA_CH1_RXP_ACCEPT_CNT {
    pub use super::DMA_CH0_RXP_ACCEPT_CNT::RXPAC;
    pub use super::DMA_CH0_RXP_ACCEPT_CNT::RXPACOF;
}

/// Channel 1 Receive ERI Counter
pub mod DMA_CH1_RX_ERI_CNT {
    pub use super::DMA_CH0_RX_ERI_CNT::ECNT;
}

/// DMA Channel 2 Control
pub mod DMA_CH2_CONTROL {
    pub use super::DMA_CH0_CONTROL::PBLx8;
    pub use super::DMA_CH0_CONTROL::DSL;
}

/// DMA Channel 2 Transmit Control
pub mod DMA_CH2_TX_CONTROL {
    pub use super::DMA_CH0_TX_CONTROL::TxPBL;
    pub use super::DMA_CH0_TX_CONTROL::EDSE;
    pub use super::DMA_CH0_TX_CONTROL::IPBL;
    pub use super::DMA_CH0_TX_CONTROL::OSF;
    pub use super::DMA_CH0_TX_CONTROL::ST;
}

/// DMA Channel 2 Receive Control
pub mod DMA_CH2_RX_CONTROL {
    pub use super::DMA_CH0_RX_CONTROL::RBSZ_13_y;
    pub use super::DMA_CH0_RX_CONTROL::RBSZ_x_0;
    pub use super::DMA_CH0_RX_CONTROL::RxPBL;
    pub use super::DMA_CH0_RX_CONTROL::RPF;
    pub use super::DMA_CH0_RX_CONTROL::SR;
}

/// Channel 2 Tx Descriptor List Address
pub mod DMA_CH2_TXDESC_LIST_ADDRESS {
    pub use super::DMA_CH0_TXDESC_LIST_ADDRESS::TDESLA;
}

/// Channel 2 Rx Descriptor List Address
pub mod DMA_CH2_RXDESC_LIST_ADDRESS {
    pub use super::DMA_CH0_RXDESC_LIST_ADDRESS::RDESLA;
}

/// Channel 2 Tx Descriptor Tail Pointer
pub mod DMA_CH2_TXDESC_TAIL_POINTER {
    pub use super::DMA_CH0_TXDESC_TAIL_POINTER::TDTP;
}

/// Channel 2 Rx Descriptor Tail Pointer
pub mod DMA_CH2_RXDESC_TAIL_POINTER {
    pub use super::DMA_CH0_RXDESC_TAIL_POINTER::RDTP;
}

/// Channel 2 Tx Descriptor Ring Length
pub mod DMA_CH2_TXDESC_RING_LENGTH {
    pub use super::DMA_CH0_TXDESC_RING_LENGTH::TDRL;
}

/// Channel 2 Rx Descriptor Ring Length
pub mod DMA_CH2_RXDESC_RING_LENGTH {
    pub use super::DMA_CH0_RXDESC_RING_LENGTH::RDRL;
}

/// Channel 2 Interrupt Enable
pub mod DMA_CH2_INTERRUPT_ENABLE {
    pub use super::DMA_CH0_INTERRUPT_ENABLE::AIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::CDEE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::ERIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::ETIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::FBEE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::NIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RBUE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RSE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RWTE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::TBUE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::TIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::TXSE;
}

/// Channel 2 Receive Interrupt Watchdog Timer
pub mod DMA_CH2_RX_INTERRUPT_WATCHDOG_TIMER {
    pub use super::DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER::RWT;
    pub use super::DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER::RWTU;
}

/// Channel 2 Slot Function Control and Status
pub mod DMA_CH2_SLOT_FUNCTION_CONTROL_STATUS {
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::ASC;
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::ESC;
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::RSN;
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::SIV;
}

/// Channel 2 Current Application Transmit Descriptor
pub mod DMA_CH2_CURRENT_APP_TXDESC {
    pub use super::DMA_CH0_CURRENT_APP_TXDESC::CURTDESAPTR;
}

/// Channel 2 Current Application Receive Descriptor
pub mod DMA_CH2_CURRENT_APP_RXDESC {
    pub use super::DMA_CH0_CURRENT_APP_RXDESC::CURRDESAPTR;
}

/// Channel 2 Current Application Transmit Buffer Address
pub mod DMA_CH2_CURRENT_APP_TXBUFFER {
    pub use super::DMA_CH0_CURRENT_APP_TXBUFFER::CURTBUFAPTR;
}

/// Channel 2 Current Application Receive Buffer Address
pub mod DMA_CH2_CURRENT_APP_RXBUFFER {
    pub use super::DMA_CH0_CURRENT_APP_RXBUFFER::CURRBUFAPTR;
}

/// DMA Channel 2 Status
pub mod DMA_CH2_STATUS {
    pub use super::DMA_CH0_STATUS::AIS;
    pub use super::DMA_CH0_STATUS::CDE;
    pub use super::DMA_CH0_STATUS::ERI;
    pub use super::DMA_CH0_STATUS::ETI;
    pub use super::DMA_CH0_STATUS::FBE;
    pub use super::DMA_CH0_STATUS::NIS;
    pub use super::DMA_CH0_STATUS::RBU;
    pub use super::DMA_CH0_STATUS::REB;
    pub use super::DMA_CH0_STATUS::RI;
    pub use super::DMA_CH0_STATUS::RPS;
    pub use super::DMA_CH0_STATUS::RWT;
    pub use super::DMA_CH0_STATUS::TBU;
    pub use super::DMA_CH0_STATUS::TEB;
    pub use super::DMA_CH0_STATUS::TI;
    pub use super::DMA_CH0_STATUS::TPS;
}

/// Channel 2 Missed Frame Counter
pub mod DMA_CH2_MISS_FRAME_CNT {
    pub use super::DMA_CH0_MISS_FRAME_CNT::MFC;
    pub use super::DMA_CH0_MISS_FRAME_CNT::MFCO;
}

/// Channel 2 RXP Frames Accepted Counter
pub mod DMA_CH2_RXP_ACCEPT_CNT {
    pub use super::DMA_CH0_RXP_ACCEPT_CNT::RXPAC;
    pub use super::DMA_CH0_RXP_ACCEPT_CNT::RXPACOF;
}

/// Channel 2 Receive ERI Counter
pub mod DMA_CH2_RX_ERI_CNT {
    pub use super::DMA_CH0_RX_ERI_CNT::ECNT;
}

/// DMA Channel 3 Control
pub mod DMA_CH3_CONTROL {
    pub use super::DMA_CH0_CONTROL::PBLx8;
    pub use super::DMA_CH0_CONTROL::DSL;
}

/// DMA Channel 3 Transmit Control
pub mod DMA_CH3_TX_CONTROL {
    pub use super::DMA_CH0_TX_CONTROL::TxPBL;
    pub use super::DMA_CH0_TX_CONTROL::EDSE;
    pub use super::DMA_CH0_TX_CONTROL::IPBL;
    pub use super::DMA_CH0_TX_CONTROL::OSF;
    pub use super::DMA_CH0_TX_CONTROL::ST;
}

/// DMA Channel 3 Receive Control
pub mod DMA_CH3_RX_CONTROL {
    pub use super::DMA_CH0_RX_CONTROL::RBSZ_13_y;
    pub use super::DMA_CH0_RX_CONTROL::RBSZ_x_0;
    pub use super::DMA_CH0_RX_CONTROL::RxPBL;
    pub use super::DMA_CH0_RX_CONTROL::RPF;
    pub use super::DMA_CH0_RX_CONTROL::SR;
}

/// Channel 3 Tx Descriptor List Address
pub mod DMA_CH3_TXDESC_LIST_ADDRESS {
    pub use super::DMA_CH0_TXDESC_LIST_ADDRESS::TDESLA;
}

/// Channel 3 Rx Descriptor List Address
pub mod DMA_CH3_RXDESC_LIST_ADDRESS {
    pub use super::DMA_CH0_RXDESC_LIST_ADDRESS::RDESLA;
}

/// Channel 3 Tx Descriptor Tail Pointer
pub mod DMA_CH3_TXDESC_TAIL_POINTER {
    pub use super::DMA_CH0_TXDESC_TAIL_POINTER::TDTP;
}

/// Channel 3 Rx Descriptor Tail Pointer
pub mod DMA_CH3_RXDESC_TAIL_POINTER {
    pub use super::DMA_CH0_RXDESC_TAIL_POINTER::RDTP;
}

/// Channel 3 Tx Descriptor Ring Length
pub mod DMA_CH3_TXDESC_RING_LENGTH {
    pub use super::DMA_CH0_TXDESC_RING_LENGTH::TDRL;
}

/// Channel 3 Rx Descriptor Ring Length
pub mod DMA_CH3_RXDESC_RING_LENGTH {
    pub use super::DMA_CH0_RXDESC_RING_LENGTH::RDRL;
}

/// Channel 3 Interrupt Enable
pub mod DMA_CH3_INTERRUPT_ENABLE {
    pub use super::DMA_CH0_INTERRUPT_ENABLE::AIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::CDEE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::ERIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::ETIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::FBEE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::NIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RBUE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RSE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RWTE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::TBUE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::TIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::TXSE;
}

/// Channel 3 Receive Interrupt Watchdog Time
pub mod DMA_CH3_RX_INTERRUPT_WATCHDOG_TIMER {
    pub use super::DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER::RWT;
    pub use super::DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER::RWTU;
}

/// Channel 3 Slot Function Control and Status
pub mod DMA_CH3_SLOT_FUNCTION_CONTROL_STATUS {
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::ASC;
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::ESC;
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::RSN;
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::SIV;
}

/// Channel 3 Current Application Transmit Descriptor
pub mod DMA_CH3_CURRENT_APP_TXDESC {
    pub use super::DMA_CH0_CURRENT_APP_TXDESC::CURTDESAPTR;
}

/// Channel 3 Current Application Receive Descriptor
pub mod DMA_CH3_CURRENT_APP_RXDESC {
    pub use super::DMA_CH0_CURRENT_APP_RXDESC::CURRDESAPTR;
}

/// Channel 3 Current Application Transmit Buffer Address
pub mod DMA_CH3_CURRENT_APP_TXBUFFER {
    pub use super::DMA_CH0_CURRENT_APP_TXBUFFER::CURTBUFAPTR;
}

/// Channel 3 Current Application Receive Buffer Address
pub mod DMA_CH3_CURRENT_APP_RXBUFFER {
    pub use super::DMA_CH0_CURRENT_APP_RXBUFFER::CURRBUFAPTR;
}

/// DMA Channel 3 Status
pub mod DMA_CH3_STATUS {
    pub use super::DMA_CH0_STATUS::AIS;
    pub use super::DMA_CH0_STATUS::CDE;
    pub use super::DMA_CH0_STATUS::ERI;
    pub use super::DMA_CH0_STATUS::ETI;
    pub use super::DMA_CH0_STATUS::FBE;
    pub use super::DMA_CH0_STATUS::NIS;
    pub use super::DMA_CH0_STATUS::RBU;
    pub use super::DMA_CH0_STATUS::REB;
    pub use super::DMA_CH0_STATUS::RI;
    pub use super::DMA_CH0_STATUS::RPS;
    pub use super::DMA_CH0_STATUS::RWT;
    pub use super::DMA_CH0_STATUS::TBU;
    pub use super::DMA_CH0_STATUS::TEB;
    pub use super::DMA_CH0_STATUS::TI;
    pub use super::DMA_CH0_STATUS::TPS;
}

/// Channel 3 Missed Frame Counter
pub mod DMA_CH3_MISS_FRAME_CNT {
    pub use super::DMA_CH0_MISS_FRAME_CNT::MFC;
    pub use super::DMA_CH0_MISS_FRAME_CNT::MFCO;
}

/// Channel 3 RXP Frames Accepted Counter
pub mod DMA_CH3_RXP_ACCEPT_CNT {
    pub use super::DMA_CH0_RXP_ACCEPT_CNT::RXPAC;
    pub use super::DMA_CH0_RXP_ACCEPT_CNT::RXPACOF;
}

/// Channel 3 Receive ERI Counter
pub mod DMA_CH3_RX_ERI_CNT {
    pub use super::DMA_CH0_RX_ERI_CNT::ECNT;
}

/// DMA Channel 4 Control
pub mod DMA_CH4_CONTROL {
    pub use super::DMA_CH0_CONTROL::PBLx8;
    pub use super::DMA_CH0_CONTROL::DSL;
}

/// DMA Channel 4 Transmit Control
pub mod DMA_CH4_TX_CONTROL {
    pub use super::DMA_CH0_TX_CONTROL::TxPBL;
    pub use super::DMA_CH0_TX_CONTROL::EDSE;
    pub use super::DMA_CH0_TX_CONTROL::IPBL;
    pub use super::DMA_CH0_TX_CONTROL::OSF;
    pub use super::DMA_CH0_TX_CONTROL::ST;
}

/// DMA Channel 4 Receive Control
pub mod DMA_CH4_RX_CONTROL {
    pub use super::DMA_CH0_RX_CONTROL::RBSZ_13_y;
    pub use super::DMA_CH0_RX_CONTROL::RBSZ_x_0;
    pub use super::DMA_CH0_RX_CONTROL::RxPBL;
    pub use super::DMA_CH0_RX_CONTROL::RPF;
    pub use super::DMA_CH0_RX_CONTROL::SR;
}

/// Channel 4 Tx Descriptor List Address
pub mod DMA_CH4_TXDESC_LIST_ADDRESS {
    pub use super::DMA_CH0_TXDESC_LIST_ADDRESS::TDESLA;
}

/// Channel 4 Rx Descriptor List Address
pub mod DMA_CH4_RXDESC_LIST_ADDRESS {
    pub use super::DMA_CH0_RXDESC_LIST_ADDRESS::RDESLA;
}

/// Channel 4 Tx Descriptor Tail Pointer
pub mod DMA_CH4_TXDESC_TAIL_POINTER {
    pub use super::DMA_CH0_TXDESC_TAIL_POINTER::TDTP;
}

/// Channel 4 Rx Descriptor Tail Pointer
pub mod DMA_CH4_RXDESC_TAIL_POINTER {
    pub use super::DMA_CH0_RXDESC_TAIL_POINTER::RDTP;
}

/// Channel 4 Tx Descriptor Ring Length
pub mod DMA_CH4_TXDESC_RING_LENGTH {
    pub use super::DMA_CH0_TXDESC_RING_LENGTH::TDRL;
}

/// Channel 4 Rx Descriptor Ring Length
pub mod DMA_CH4_RXDESC_RING_LENGTH {
    pub use super::DMA_CH0_RXDESC_RING_LENGTH::RDRL;
}

/// Channel 4 Interrupt Enable
pub mod DMA_CH4_INTERRUPT_ENABLE {
    pub use super::DMA_CH0_INTERRUPT_ENABLE::AIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::CDEE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::ERIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::ETIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::FBEE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::NIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RBUE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RSE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::RWTE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::TBUE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::TIE;
    pub use super::DMA_CH0_INTERRUPT_ENABLE::TXSE;
}

/// Channel 4 Receive Interrupt Watchdog Timer
pub mod DMA_CH4_RX_INTERRUPT_WATCHDOG_TIMER {
    pub use super::DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER::RWT;
    pub use super::DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER::RWTU;
}

/// Channel 4 Slot Function Control and Status
pub mod DMA_CH4_SLOT_FUNCTION_CONTROL_STATUS {
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::ASC;
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::ESC;
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::RSN;
    pub use super::DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS::SIV;
}

/// Channel 4 Current Application Transmit Descriptor
pub mod DMA_CH4_CURRENT_APP_TXDESC {
    pub use super::DMA_CH0_CURRENT_APP_TXDESC::CURTDESAPTR;
}

/// Channel 4 Current Application Receive Descriptor
pub mod DMA_CH4_CURRENT_APP_RXDESC {
    pub use super::DMA_CH0_CURRENT_APP_RXDESC::CURRDESAPTR;
}

/// Channel 4 Current Application Transmit Buffer Address
pub mod DMA_CH4_CURRENT_APP_TXBUFFER {
    pub use super::DMA_CH0_CURRENT_APP_TXBUFFER::CURTBUFAPTR;
}

/// Channel 4 Current Application Receive Buffer Address
pub mod DMA_CH4_CURRENT_APP_RXBUFFER {
    pub use super::DMA_CH0_CURRENT_APP_RXBUFFER::CURRBUFAPTR;
}

/// DMA Channel 4 Status
pub mod DMA_CH4_STATUS {
    pub use super::DMA_CH0_STATUS::AIS;
    pub use super::DMA_CH0_STATUS::CDE;
    pub use super::DMA_CH0_STATUS::ERI;
    pub use super::DMA_CH0_STATUS::ETI;
    pub use super::DMA_CH0_STATUS::FBE;
    pub use super::DMA_CH0_STATUS::NIS;
    pub use super::DMA_CH0_STATUS::RBU;
    pub use super::DMA_CH0_STATUS::REB;
    pub use super::DMA_CH0_STATUS::RI;
    pub use super::DMA_CH0_STATUS::RPS;
    pub use super::DMA_CH0_STATUS::RWT;
    pub use super::DMA_CH0_STATUS::TBU;
    pub use super::DMA_CH0_STATUS::TEB;
    pub use super::DMA_CH0_STATUS::TI;
    pub use super::DMA_CH0_STATUS::TPS;
}

/// Channel 4 Missed Frame Counter
pub mod DMA_CH4_MISS_FRAME_CNT {
    pub use super::DMA_CH0_MISS_FRAME_CNT::MFC;
    pub use super::DMA_CH0_MISS_FRAME_CNT::MFCO;
}

/// Channel 4 RXP Frames Accepted Counter
pub mod DMA_CH4_RXP_ACCEPT_CNT {
    pub use super::DMA_CH0_RXP_ACCEPT_CNT::RXPAC;
    pub use super::DMA_CH0_RXP_ACCEPT_CNT::RXPACOF;
}

/// Channel 4 Receive ERI Counter
pub mod DMA_CH4_RX_ERI_CNT {
    pub use super::DMA_CH0_RX_ERI_CNT::ECNT;
}
#[repr(C)]
pub struct RegisterBlock {
    /// MAC Configuration Register
    pub MAC_CONFIGURATION: RWRegister<u32>,

    /// MAC Extended Configuration Register
    pub MAC_EXT_CONFIGURATION: RWRegister<u32>,

    /// MAC Packet Filter
    pub MAC_PACKET_FILTER: RWRegister<u32>,

    /// Watchdog Timeout
    pub MAC_WATCHDOG_TIMEOUT: RWRegister<u32>,

    /// MAC Hash Table Register 0
    pub MAC_HASH_TABLE_REG0: RWRegister<u32>,

    /// MAC Hash Table Register 1
    pub MAC_HASH_TABLE_REG1: RWRegister<u32>,

    _reserved1: [u32; 14],

    /// MAC VLAN Tag Control
    pub MAC_VLAN_TAG_CTRL: RWRegister<u32>,

    /// MAC VLAN Tag Data
    pub MAC_VLAN_TAG_DATA: RWRegister<u32>,

    /// MAC VLAN Hash Table
    pub MAC_VLAN_HASH_TABLE: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// VLAN Tag Inclusion or Replacement
    pub MAC_VLAN_INCL: RWRegister<u32>,

    /// MAC Inner VLAN Tag Inclusion or Replacement
    pub MAC_INNER_VLAN_INCL: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// MAC Q0 Tx Flow Control
    pub MAC_Q0_TX_FLOW_CTRL: RWRegister<u32>,

    /// MAC Q1 Tx Flow Control
    pub MAC_Q1_TX_FLOW_CTRL: RWRegister<u32>,

    /// MAC Q2 Tx Flow Control
    pub MAC_Q2_TX_FLOW_CTRL: RWRegister<u32>,

    /// MAC Q3 Tx Flow Control
    pub MAC_Q3_TX_FLOW_CTRL: RWRegister<u32>,

    /// MAC Q4 Tx Flow Control
    pub MAC_Q4_TX_FLOW_CTRL: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// MAC Rx Flow Control
    pub MAC_RX_FLOW_CTRL: RWRegister<u32>,

    /// Receive Queue Control 4
    pub MAC_RXQ_CTRL4: RWRegister<u32>,

    /// Transmit Queue Priority Mapping 0
    pub MAC_TXQ_PRTY_MAP0: RWRegister<u32>,

    /// Transmit Queue Priority Mapping 1
    pub MAC_TXQ_PRTY_MAP1: RWRegister<u32>,

    /// Receive Queue Control 0
    pub MAC_RXQ_CTRL0: RWRegister<u32>,

    /// Receive Queue Control 1
    pub MAC_RXQ_CTRL1: RWRegister<u32>,

    /// Receive Queue Control 2
    pub MAC_RXQ_CTRL2: RWRegister<u32>,

    /// Receive Queue Control 3
    pub MAC_RXQ_CTRL3: RWRegister<u32>,

    /// Interrupt Status
    pub MAC_INTERRUPT_STATUS: RORegister<u32>,

    /// Interrupt Enable
    pub MAC_INTERRUPT_ENABLE: RWRegister<u32>,

    /// Receive Transmit Status
    pub MAC_RX_TX_STATUS: RORegister<u32>,

    _reserved5: [u32; 1],

    /// PMT Control and Status
    pub MAC_PMT_CONTROL_STATUS: RWRegister<u32>,

    /// Remote Wakeup Filter
    pub MAC_RWK_PACKET_FILTER: RWRegister<u32>,

    _reserved6: [u32; 2],

    /// LPI Control and Status
    pub MAC_LPI_CONTROL_STATUS: RWRegister<u32>,

    /// LPI Timers Control
    pub MAC_LPI_TIMERS_CONTROL: RWRegister<u32>,

    /// Tx LPI Entry Timer Control
    pub MAC_LPI_ENTRY_TIMER: RWRegister<u32>,

    /// One-microsecond Reference Timer
    pub MAC_ONEUS_TIC_COUNTER: RWRegister<u32>,

    _reserved7: [u32; 6],

    /// PHY Interface Control and Status
    pub MAC_PHYIF_CONTROL_STATUS: RWRegister<u32>,

    _reserved8: [u32; 5],

    /// MAC Version
    pub MAC_VERSION: RORegister<u32>,

    /// MAC Debug
    pub MAC_DEBUG: RORegister<u32>,

    _reserved9: [u32; 1],

    /// Optional Features or Functions 0
    pub MAC_HW_FEATURE0: RORegister<u32>,

    /// Optional Features or Functions 1
    pub MAC_HW_FEATURE1: RORegister<u32>,

    /// Optional Features or Functions 2
    pub MAC_HW_FEATURE2: RORegister<u32>,

    /// Optional Features or Functions 3
    pub MAC_HW_FEATURE3: RORegister<u32>,

    _reserved10: [u32; 53],

    /// MDIO Address
    pub MAC_MDIO_ADDRESS: RWRegister<u32>,

    /// MAC MDIO Data
    pub MAC_MDIO_DATA: RWRegister<u32>,

    _reserved11: [u32; 10],

    /// CSR Software Control
    pub MAC_CSR_SW_CTRL: RWRegister<u32>,

    /// Frame Preemption Control
    pub MAC_FPE_CTRL_STS: RWRegister<u32>,

    _reserved12: [u32; 2],

    /// 32-bit Binary Rollover Equivalent Time
    pub MAC_PRESN_TIME_NS: RORegister<u32>,

    /// MAC 1722 Presentation Time
    pub MAC_PRESN_TIME_UPDT: RWRegister<u32>,

    _reserved13: [u32; 46],

    /// MAC Address0 High
    pub MAC_ADDRESS0_HIGH: RWRegister<u32>,

    /// MAC Address0 Low
    pub MAC_ADDRESS0_LOW: RWRegister<u32>,

    /// MAC Address1 High
    pub MAC_ADDRESS1_HIGH: RWRegister<u32>,

    /// MAC Address1 Low
    pub MAC_ADDRESS1_LOW: RWRegister<u32>,

    /// MAC Address2 High
    pub MAC_ADDRESS2_HIGH: RWRegister<u32>,

    /// MAC Address2 Low
    pub MAC_ADDRESS2_LOW: RWRegister<u32>,

    /// MAC Address3 High
    pub MAC_ADDRESS3_HIGH: RWRegister<u32>,

    /// MAC Address3 Low
    pub MAC_ADDRESS3_LOW: RWRegister<u32>,

    /// MAC Address4 High
    pub MAC_ADDRESS4_HIGH: RWRegister<u32>,

    /// MAC Address4 Low
    pub MAC_ADDRESS4_LOW: RWRegister<u32>,

    /// MAC Address5 High
    pub MAC_ADDRESS5_HIGH: RWRegister<u32>,

    /// MAC Address5 Low
    pub MAC_ADDRESS5_LOW: RWRegister<u32>,

    /// MAC Address6 High
    pub MAC_ADDRESS6_HIGH: RWRegister<u32>,

    /// MAC Address6 Low
    pub MAC_ADDRESS6_LOW: RWRegister<u32>,

    /// MAC Address7 High
    pub MAC_ADDRESS7_HIGH: RWRegister<u32>,

    /// MAC Address7 Low
    pub MAC_ADDRESS7_LOW: RWRegister<u32>,

    /// MAC Address8 High
    pub MAC_ADDRESS8_HIGH: RWRegister<u32>,

    /// MAC Address8 Low
    pub MAC_ADDRESS8_LOW: RWRegister<u32>,

    /// MAC Address9 High
    pub MAC_ADDRESS9_HIGH: RWRegister<u32>,

    /// MAC Address9 Low
    pub MAC_ADDRESS9_LOW: RWRegister<u32>,

    /// MAC Address10 High
    pub MAC_ADDRESS10_HIGH: RWRegister<u32>,

    /// MAC Address10 Low
    pub MAC_ADDRESS10_LOW: RWRegister<u32>,

    /// MAC Address11 High
    pub MAC_ADDRESS11_HIGH: RWRegister<u32>,

    /// MAC Address11 Low
    pub MAC_ADDRESS11_LOW: RWRegister<u32>,

    /// MAC Address12 High
    pub MAC_ADDRESS12_HIGH: RWRegister<u32>,

    /// MAC Address12 Low
    pub MAC_ADDRESS12_LOW: RWRegister<u32>,

    /// MAC Address13 High
    pub MAC_ADDRESS13_HIGH: RWRegister<u32>,

    /// MAC Address13 Low
    pub MAC_ADDRESS13_LOW: RWRegister<u32>,

    /// MAC Address14 High
    pub MAC_ADDRESS14_HIGH: RWRegister<u32>,

    /// MAC Address14 Low
    pub MAC_ADDRESS14_LOW: RWRegister<u32>,

    /// MAC Address15 High
    pub MAC_ADDRESS15_HIGH: RWRegister<u32>,

    /// MAC Address15 Low
    pub MAC_ADDRESS15_LOW: RWRegister<u32>,

    /// MAC Address16 High
    pub MAC_ADDRESS16_HIGH: RWRegister<u32>,

    /// MAC Address16 Low
    pub MAC_ADDRESS16_LOW: RWRegister<u32>,

    /// MAC Address17 High
    pub MAC_ADDRESS17_HIGH: RWRegister<u32>,

    /// MAC Address17 Low
    pub MAC_ADDRESS17_LOW: RWRegister<u32>,

    /// MAC Address18 High
    pub MAC_ADDRESS18_HIGH: RWRegister<u32>,

    /// MAC Address18 Low
    pub MAC_ADDRESS18_LOW: RWRegister<u32>,

    /// MAC Address19 High
    pub MAC_ADDRESS19_HIGH: RWRegister<u32>,

    /// MAC Address19 Low
    pub MAC_ADDRESS19_LOW: RWRegister<u32>,

    /// MAC Address20 High
    pub MAC_ADDRESS20_HIGH: RWRegister<u32>,

    /// MAC Address20 Low
    pub MAC_ADDRESS20_LOW: RWRegister<u32>,

    /// MAC Address21 High
    pub MAC_ADDRESS21_HIGH: RWRegister<u32>,

    /// MAC Address21 Low
    pub MAC_ADDRESS21_LOW: RWRegister<u32>,

    /// MAC Address22 High
    pub MAC_ADDRESS22_HIGH: RWRegister<u32>,

    /// MAC Address22 Low
    pub MAC_ADDRESS22_LOW: RWRegister<u32>,

    /// MAC Address23 High
    pub MAC_ADDRESS23_HIGH: RWRegister<u32>,

    /// MAC Address23 Low
    pub MAC_ADDRESS23_LOW: RWRegister<u32>,

    /// MAC Address24 High
    pub MAC_ADDRESS24_HIGH: RWRegister<u32>,

    /// MAC Address24 Low
    pub MAC_ADDRESS24_LOW: RWRegister<u32>,

    /// MAC Address25 High
    pub MAC_ADDRESS25_HIGH: RWRegister<u32>,

    /// MAC Address25 Low
    pub MAC_ADDRESS25_LOW: RWRegister<u32>,

    /// MAC Address26 High
    pub MAC_ADDRESS26_HIGH: RWRegister<u32>,

    /// MAC Address26 Low
    pub MAC_ADDRESS26_LOW: RWRegister<u32>,

    /// MAC Address27 High
    pub MAC_ADDRESS27_HIGH: RWRegister<u32>,

    /// MAC Address27 Low
    pub MAC_ADDRESS27_LOW: RWRegister<u32>,

    /// MAC Address28 High
    pub MAC_ADDRESS28_HIGH: RWRegister<u32>,

    /// MAC Address28 Low
    pub MAC_ADDRESS28_LOW: RWRegister<u32>,

    /// MAC Address29 High
    pub MAC_ADDRESS29_HIGH: RWRegister<u32>,

    /// MAC Address29 Low
    pub MAC_ADDRESS29_LOW: RWRegister<u32>,

    /// MAC Address30 High
    pub MAC_ADDRESS30_HIGH: RWRegister<u32>,

    /// MAC Address30 Low
    pub MAC_ADDRESS30_LOW: RWRegister<u32>,

    /// MAC Address31 High
    pub MAC_ADDRESS31_HIGH: RWRegister<u32>,

    /// MAC Address31 Low
    pub MAC_ADDRESS31_LOW: RWRegister<u32>,

    /// MAC Address32 High
    pub MAC_ADDRESS32_HIGH: RWRegister<u32>,

    /// MAC Address32 Low
    pub MAC_ADDRESS32_LOW: RWRegister<u32>,

    /// MAC Address33 High
    pub MAC_ADDRESS33_HIGH: RWRegister<u32>,

    /// MAC Address33 Low
    pub MAC_ADDRESS33_LOW: RWRegister<u32>,

    /// MAC Address34 High
    pub MAC_ADDRESS34_HIGH: RWRegister<u32>,

    /// MAC Address34 Low
    pub MAC_ADDRESS34_LOW: RWRegister<u32>,

    /// MAC Address35 High
    pub MAC_ADDRESS35_HIGH: RWRegister<u32>,

    /// MAC Address35 Low
    pub MAC_ADDRESS35_LOW: RWRegister<u32>,

    /// MAC Address36 High
    pub MAC_ADDRESS36_HIGH: RWRegister<u32>,

    /// MAC Address36 Low
    pub MAC_ADDRESS36_LOW: RWRegister<u32>,

    /// MAC Address37 High
    pub MAC_ADDRESS37_HIGH: RWRegister<u32>,

    /// MAC Address37 Low
    pub MAC_ADDRESS37_LOW: RWRegister<u32>,

    /// MAC Address38 High
    pub MAC_ADDRESS38_HIGH: RWRegister<u32>,

    /// MAC Address38 Low
    pub MAC_ADDRESS38_LOW: RWRegister<u32>,

    /// MAC Address39 High
    pub MAC_ADDRESS39_HIGH: RWRegister<u32>,

    /// MAC Address39 Low
    pub MAC_ADDRESS39_LOW: RWRegister<u32>,

    /// MAC Address40 High
    pub MAC_ADDRESS40_HIGH: RWRegister<u32>,

    /// MAC Address40 Low
    pub MAC_ADDRESS40_LOW: RWRegister<u32>,

    /// MAC Address41 High
    pub MAC_ADDRESS41_HIGH: RWRegister<u32>,

    /// MAC Address41 Low
    pub MAC_ADDRESS41_LOW: RWRegister<u32>,

    /// MAC Address42 High
    pub MAC_ADDRESS42_HIGH: RWRegister<u32>,

    /// MAC Address42 Low
    pub MAC_ADDRESS42_LOW: RWRegister<u32>,

    /// MAC Address43 High
    pub MAC_ADDRESS43_HIGH: RWRegister<u32>,

    /// MAC Address43 Low
    pub MAC_ADDRESS43_LOW: RWRegister<u32>,

    /// MAC Address44 High
    pub MAC_ADDRESS44_HIGH: RWRegister<u32>,

    /// MAC Address44 Low
    pub MAC_ADDRESS44_LOW: RWRegister<u32>,

    /// MAC Address45 High
    pub MAC_ADDRESS45_HIGH: RWRegister<u32>,

    /// MAC Address45 Low
    pub MAC_ADDRESS45_LOW: RWRegister<u32>,

    /// MAC Address46 High
    pub MAC_ADDRESS46_HIGH: RWRegister<u32>,

    /// MAC Address46 Low
    pub MAC_ADDRESS46_LOW: RWRegister<u32>,

    /// MAC Address47 High
    pub MAC_ADDRESS47_HIGH: RWRegister<u32>,

    /// MAC Address47 Low
    pub MAC_ADDRESS47_LOW: RWRegister<u32>,

    /// MAC Address48 High
    pub MAC_ADDRESS48_HIGH: RWRegister<u32>,

    /// MAC Address48 Low
    pub MAC_ADDRESS48_LOW: RWRegister<u32>,

    /// MAC Address49 High
    pub MAC_ADDRESS49_HIGH: RWRegister<u32>,

    /// MAC Address49 Low
    pub MAC_ADDRESS49_LOW: RWRegister<u32>,

    /// MAC Address50 High
    pub MAC_ADDRESS50_HIGH: RWRegister<u32>,

    /// MAC Address50 Low
    pub MAC_ADDRESS50_LOW: RWRegister<u32>,

    /// MAC Address51 High
    pub MAC_ADDRESS51_HIGH: RWRegister<u32>,

    /// MAC Address51 Low
    pub MAC_ADDRESS51_LOW: RWRegister<u32>,

    /// MAC Address52 High
    pub MAC_ADDRESS52_HIGH: RWRegister<u32>,

    /// MAC Address52 Low
    pub MAC_ADDRESS52_LOW: RWRegister<u32>,

    /// MAC Address53 High
    pub MAC_ADDRESS53_HIGH: RWRegister<u32>,

    /// MAC Address53 Low
    pub MAC_ADDRESS53_LOW: RWRegister<u32>,

    /// MAC Address54 High
    pub MAC_ADDRESS54_HIGH: RWRegister<u32>,

    /// MAC Address54 Low
    pub MAC_ADDRESS54_LOW: RWRegister<u32>,

    /// MAC Address55 High
    pub MAC_ADDRESS55_HIGH: RWRegister<u32>,

    /// MAC Address55 Low
    pub MAC_ADDRESS55_LOW: RWRegister<u32>,

    /// MAC Address56 High
    pub MAC_ADDRESS56_HIGH: RWRegister<u32>,

    /// MAC Address56 Low
    pub MAC_ADDRESS56_LOW: RWRegister<u32>,

    /// MAC Address57 High
    pub MAC_ADDRESS57_HIGH: RWRegister<u32>,

    /// MAC Address57 Low
    pub MAC_ADDRESS57_LOW: RWRegister<u32>,

    /// MAC Address58 High
    pub MAC_ADDRESS58_HIGH: RWRegister<u32>,

    /// MAC Address58 Low
    pub MAC_ADDRESS58_LOW: RWRegister<u32>,

    /// MAC Address59 High
    pub MAC_ADDRESS59_HIGH: RWRegister<u32>,

    /// MAC Address59 Low
    pub MAC_ADDRESS59_LOW: RWRegister<u32>,

    /// MAC Address60 High
    pub MAC_ADDRESS60_HIGH: RWRegister<u32>,

    /// MAC Address60 Low
    pub MAC_ADDRESS60_LOW: RWRegister<u32>,

    /// MAC Address61 High
    pub MAC_ADDRESS61_HIGH: RWRegister<u32>,

    /// MAC Address61 Low
    pub MAC_ADDRESS61_LOW: RWRegister<u32>,

    /// MAC Address62 High
    pub MAC_ADDRESS62_HIGH: RWRegister<u32>,

    /// MAC Address62 Low
    pub MAC_ADDRESS62_LOW: RWRegister<u32>,

    /// MAC Address63 High
    pub MAC_ADDRESS63_HIGH: RWRegister<u32>,

    /// MAC Address63 Low
    pub MAC_ADDRESS63_LOW: RWRegister<u32>,

    _reserved14: [u32; 128],

    /// MMC Control
    pub MAC_MMC_CONTROL: RWRegister<u32>,

    /// MMC Rx Interrupt
    pub MAC_MMC_RX_INTERRUPT: RORegister<u32>,

    /// MMC Tx Interrupt
    pub MAC_MMC_TX_INTERRUPT: RORegister<u32>,

    /// MMC Rx Interrupt Mask
    pub MAC_MMC_RX_INTERRUPT_MASK: RWRegister<u32>,

    /// MMC Tx Interrupt Mask
    pub MAC_MMC_TX_INTERRUPT_MASK: RWRegister<u32>,

    /// Tx Octet Count Good and Bad
    pub MAC_TX_OCTET_COUNT_GOOD_BAD: RORegister<u32>,

    /// Tx Packet Count Good and Bad
    pub MAC_TX_PACKET_COUNT_GOOD_BAD: RORegister<u32>,

    /// Tx Broadcast Packets Good
    pub MAC_TX_BROADCAST_PACKETS_GOOD: RORegister<u32>,

    /// Tx Multicast Packets Good
    pub MAC_TX_MULTICAST_PACKETS_GOOD: RORegister<u32>,

    /// Tx Good and Bad 64-Byte Packets
    pub MAC_TX_64OCTETS_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Tx Good and Bad 65 to 127-Byte Packets
    pub MAC_TX_65TO127OCTETS_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Tx Good and Bad 128 to 255-Byte Packets
    pub MAC_TX_128TO255OCTETS_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Tx Good and Bad 256 to 511-Byte Packets
    pub MAC_TX_256TO511OCTETS_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Tx Good and Bad 512 to 1023-Byte Packets
    pub MAC_TX_512TO1023OCTETS_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Tx Good and Bad 1024 to Max-Byte Packets
    pub MAC_TX_1024TOMAXOCTETS_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Good and Bad Unicast Packets Transmitted
    pub MAC_TX_UNICAST_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Good and Bad Multicast Packets Transmitted
    pub MAC_TX_MULTICAST_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Good and Bad Broadcast Packets Transmitted
    pub MAC_TX_BROADCAST_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Tx Packets Aborted By Underflow Error
    pub MAC_TX_UNDERFLOW_ERROR_PACKETS: RORegister<u32>,

    /// Single Collision Good Packets Transmitted
    pub MAC_TX_SINGLE_COLLISION_GOOD_PACKETS: RORegister<u32>,

    /// Multiple Collision Good Packets Transmitted
    pub MAC_TX_MULTIPLE_COLLISION_GOOD_PACKETS: RORegister<u32>,

    /// Deferred Packets Transmitted
    pub MAC_TX_DEFERRED_PACKETS: RORegister<u32>,

    /// Late Collision Packets Transmitted
    pub MAC_TX_LATE_COLLISION_PACKETS: RORegister<u32>,

    /// Excessive Collision Packets Transmitted
    pub MAC_TX_EXCESSIVE_COLLISION_PACKETS: RORegister<u32>,

    /// Carrier Error Packets Transmitted
    pub MAC_TX_CARRIER_ERROR_PACKETS: RORegister<u32>,

    /// Bytes Transmitted in Good Packets
    pub MAC_TX_OCTET_COUNT_GOOD: RORegister<u32>,

    /// Good Packets Transmitted
    pub MAC_TX_PACKET_COUNT_GOOD: RORegister<u32>,

    /// Packets Aborted By Excessive Deferral Error
    pub MAC_TX_EXCESSIVE_DEFERRAL_ERROR: RORegister<u32>,

    /// Pause Packets Transmitted
    pub MAC_TX_PAUSE_PACKETS: RORegister<u32>,

    /// Good VLAN Packets Transmitted
    pub MAC_TX_VLAN_PACKETS_GOOD: RORegister<u32>,

    /// Good Oversize Packets Transmitted
    pub MAC_TX_OSIZE_PACKETS_GOOD: RORegister<u32>,

    _reserved15: [u32; 1],

    /// Good and Bad Packets Received
    pub MAC_RX_PACKETS_COUNT_GOOD_BAD: RORegister<u32>,

    /// Bytes in Good and Bad Packets Received
    pub MAC_RX_OCTET_COUNT_GOOD_BAD: RORegister<u32>,

    /// Bytes in Good Packets Received
    pub MAC_RX_OCTET_COUNT_GOOD: RORegister<u32>,

    /// Good Broadcast Packets Received
    pub MAC_RX_BROADCAST_PACKETS_GOOD: RORegister<u32>,

    /// Good Multicast Packets Received
    pub MAC_RX_MULTICAST_PACKETS_GOOD: RORegister<u32>,

    /// CRC Error Packets Received
    pub MAC_RX_CRC_ERROR_PACKETS: RORegister<u32>,

    /// Alignment Error Packets Received
    pub MAC_RX_ALIGNMENT_ERROR_PACKETS: RORegister<u32>,

    /// Runt Error Packets Received
    pub MAC_RX_RUNT_ERROR_PACKETS: RORegister<u32>,

    /// Jabber Error Packets Received
    pub MAC_RX_JABBER_ERROR_PACKETS: RORegister<u32>,

    /// Good Undersize Packets Received
    pub MAC_RX_UNDERSIZE_PACKETS_GOOD: RORegister<u32>,

    /// Good Oversize Packets Received
    pub MAC_RX_OVERSIZE_PACKETS_GOOD: RORegister<u32>,

    /// Good and Bad 64-Byte Packets Received
    pub MAC_RX_64OCTETS_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Good and Bad 64-to-127 Byte Packets Received
    pub MAC_RX_65TO127OCTETS_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Good and Bad 128-to-255 Byte Packets Received
    pub MAC_RX_128TO255OCTETS_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Good and Bad 256-to-511 Byte Packets Received
    pub MAC_RX_256TO511OCTETS_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Good and Bad 512-to-1023 Byte Packets Received
    pub MAC_RX_512TO1023OCTETS_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Good and Bad 1024-to-Max Byte Packets Received
    pub MAC_RX_1024TOMAXOCTETS_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Good Unicast Packets Received
    pub MAC_RX_UNICAST_PACKETS_GOOD: RORegister<u32>,

    /// Length Error Packets Received
    pub MAC_RX_LENGTH_ERROR_PACKETS: RORegister<u32>,

    /// Out-of-range Type Packets Received
    pub MAC_RX_OUT_OF_RANGE_TYPE_PACKETS: RORegister<u32>,

    /// Pause Packets Received
    pub MAC_RX_PAUSE_PACKETS: RORegister<u32>,

    /// Missed Packets Due to FIFO Overflow
    pub MAC_RX_FIFO_OVERFLOW_PACKETS: RORegister<u32>,

    /// Good and Bad VLAN Packets Received
    pub MAC_RX_VLAN_PACKETS_GOOD_BAD: RORegister<u32>,

    /// Watchdog Error Packets Received
    pub MAC_RX_WATCHDOG_ERROR_PACKETS: RORegister<u32>,

    /// Receive Error Packets Received
    pub MAC_RX_RECEIVE_ERROR_PACKETS: RORegister<u32>,

    /// Good Control Packets Received
    pub MAC_RX_CONTROL_PACKETS_GOOD: RORegister<u32>,

    _reserved16: [u32; 1],

    /// Microseconds Tx LPI Asserted
    pub MAC_TX_LPI_USEC_CNTR: RORegister<u32>,

    /// Number of Times Tx LPI Asserted
    pub MAC_TX_LPI_TRAN_CNTR: RORegister<u32>,

    /// Microseconds Rx LPI Sampled
    pub MAC_RX_LPI_USEC_CNTR: RORegister<u32>,

    /// Number of Times Rx LPI Entered
    pub MAC_RX_LPI_TRAN_CNTR: RORegister<u32>,

    _reserved17: [u32; 1],

    /// MMC IPC Receive Interrupt Mask
    pub MAC_MMC_IPC_RX_INTERRUPT_MASK: RWRegister<u32>,

    _reserved18: [u32; 1],

    /// MMC IPC Receive Interrupt
    pub MAC_MMC_IPC_RX_INTERRUPT: RORegister<u32>,

    _reserved19: [u32; 1],

    /// Good IPv4 Datagrams Received
    pub MAC_RXIPV4_GOOD_PACKETS: RORegister<u32>,

    /// IPv4 Datagrams Received with Header Errors
    pub MAC_RXIPV4_HEADER_ERROR_PACKETS: RORegister<u32>,

    /// IPv4 Datagrams Received with No Payload
    pub MAC_RXIPV4_NO_PAYLOAD_PACKETS: RORegister<u32>,

    /// IPv4 Datagrams Received with Fragmentation
    pub MAC_RXIPV4_FRAGMENTED_PACKETS: RORegister<u32>,

    /// IPv4 Datagrams Received with UDP Checksum Disabled
    pub MAC_RXIPV4_UDP_CHECKSUM_DISABLED_PACKETS: RORegister<u32>,

    /// Good IPv6 Datagrams Received
    pub MAC_RXIPV6_GOOD_PACKETS: RORegister<u32>,

    /// IPv6 Datagrams Received with Header Errors
    pub MAC_RXIPV6_HEADER_ERROR_PACKETS: RORegister<u32>,

    /// IPv6 Datagrams Received with No Payload
    pub MAC_RXIPV6_NO_PAYLOAD_PACKETS: RORegister<u32>,

    /// IPv6 Datagrams Received with Good UDP
    pub MAC_RXUDP_GOOD_PACKETS: RORegister<u32>,

    /// IPv6 Datagrams Received with UDP Checksum Error
    pub MAC_RXUDP_ERROR_PACKETS: RORegister<u32>,

    /// IPv6 Datagrams Received with Good TCP Payload
    pub MAC_RXTCP_GOOD_PACKETS: RORegister<u32>,

    /// IPv6 Datagrams Received with TCP Checksum Error
    pub MAC_RXTCP_ERROR_PACKETS: RORegister<u32>,

    /// IPv6 Datagrams Received with Good ICMP Payload
    pub MAC_RXICMP_GOOD_PACKETS: RORegister<u32>,

    /// IPv6 Datagrams Received with ICMP Checksum Error
    pub MAC_RXICMP_ERROR_PACKETS: RORegister<u32>,

    _reserved20: [u32; 2],

    /// Good Bytes Received in IPv4 Datagrams
    pub MAC_RXIPV4_GOOD_OCTETS: RORegister<u32>,

    /// Bytes Received in IPv4 Datagrams with Header Errors
    pub MAC_RXIPV4_HEADER_ERROR_OCTETS: RORegister<u32>,

    /// Bytes Received in IPv4 Datagrams with No Payload
    pub MAC_RXIPV4_NO_PAYLOAD_OCTETS: RORegister<u32>,

    /// Bytes Received in Fragmented IPv4 Datagrams
    pub MAC_RXIPV4_FRAGMENTED_OCTETS: RORegister<u32>,

    /// Bytes Received with UDP Checksum Disabled
    pub MAC_RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS: RORegister<u32>,

    /// Bytes Received in Good IPv6 Datagrams
    pub MAC_RXIPV6_GOOD_OCTETS: RORegister<u32>,

    /// Bytes Received in IPv6 Datagrams with Data Errors
    pub MAC_RXIPV6_HEADER_ERROR_OCTETS: RORegister<u32>,

    /// Bytes Received in IPv6 Datagrams with No Payload
    pub MAC_RXIPV6_NO_PAYLOAD_OCTETS: RORegister<u32>,

    /// Bytes Received in Good UDP Segment
    pub MAC_RXUDP_GOOD_OCTETS: RORegister<u32>,

    /// Bytes Received in UDP Segment with Checksum Errors
    pub MAC_RXUDP_ERROR_OCTETS: RORegister<u32>,

    /// Bytes Received in Good TCP Segment
    pub MAC_RXTCP_GOOD_OCTETS: RORegister<u32>,

    /// Bytes Received in TCP Segment with Checksum Errors
    pub MAC_RXTCP_ERROR_OCTETS: RORegister<u32>,

    /// Bytes Received in Good ICMP Segment
    pub MAC_RXICMP_GOOD_OCTETS: RORegister<u32>,

    /// Bytes Received in ICMP Segment with Checksum Errors
    pub MAC_RXICMP_ERROR_OCTETS: RORegister<u32>,

    _reserved21: [u32; 6],

    /// MMC FPE Transmit Interrupt
    pub MAC_MMC_FPE_TX_INTERRUPT: RORegister<u32>,

    /// MMC FPE Transmit Mask Interrupt
    pub MAC_MMC_FPE_TX_INTERRUPT_MASK: RWRegister<u32>,

    /// MMC FPE Transmitted Fragment Counter
    pub MAC_MMC_TX_FPE_FRAGMENT_CNTR: RORegister<u32>,

    /// MMC FPE Transmitted Hold Request Counter
    pub MAC_MMC_TX_HOLD_REQ_CNTR: RORegister<u32>,

    _reserved22: [u32; 4],

    /// MMC FPE Receive Interrupt
    pub MAC_MMC_FPE_RX_INTERRUPT: RORegister<u32>,

    /// MMC FPE Receive Interrupt Mask
    pub MAC_MMC_FPE_RX_INTERRUPT_MASK: RWRegister<u32>,

    /// MMC Receive Packet Reassembly Error Counter
    pub MAC_MMC_RX_PACKET_ASSEMBLY_ERR_CNTR: RORegister<u32>,

    /// MMC Receive Packet SMD Error Counter
    pub MAC_MMC_RX_PACKET_SMD_ERR_CNTR: RORegister<u32>,

    /// MMC Receive Packet Successful Reassembly Counter
    pub MAC_MMC_RX_PACKET_ASSEMBLY_OK_CNTR: RORegister<u32>,

    /// MMC FPE Received Fragment Counter
    pub MAC_MMC_RX_FPE_FRAGMENT_CNTR: RORegister<u32>,

    _reserved23: [u32; 10],

    /// Layer 3 and Layer 4 Control of Filter 0
    pub MAC_L3_L4_CONTROL0: RWRegister<u32>,

    /// Layer 4 Address 0
    pub MAC_LAYER4_ADDRESS0: RWRegister<u32>,

    _reserved24: [u32; 2],

    /// Layer 3 Address 0 Register 0
    pub MAC_LAYER3_ADDR0_REG0: RWRegister<u32>,

    /// Layer 3 Address 1 Register 0
    pub MAC_LAYER3_ADDR1_REG0: RWRegister<u32>,

    /// Layer 3 Address 2 Register 0
    pub MAC_LAYER3_ADDR2_REG0: RWRegister<u32>,

    /// Layer 3 Address 3 Register 0
    pub MAC_LAYER3_ADDR3_REG0: RWRegister<u32>,

    _reserved25: [u32; 4],

    /// Layer 3 and Layer 4 Control of Filter 1
    pub MAC_L3_L4_CONTROL1: RWRegister<u32>,

    /// Layer 4 Address 0
    pub MAC_LAYER4_ADDRESS1: RWRegister<u32>,

    _reserved26: [u32; 2],

    /// Layer 3 Address 0 Register 1
    pub MAC_LAYER3_ADDR0_REG1: RWRegister<u32>,

    /// Layer 3 Address 1 Register 1
    pub MAC_LAYER3_ADDR1_REG1: RWRegister<u32>,

    /// Layer 3 Address 2 Register 1
    pub MAC_LAYER3_ADDR2_REG1: RWRegister<u32>,

    /// Layer 3 Address 3 Register 1
    pub MAC_LAYER3_ADDR3_REG1: RWRegister<u32>,

    _reserved27: [u32; 4],

    /// Layer 3 and Layer 4 Control of Filter 2
    pub MAC_L3_L4_CONTROL2: RWRegister<u32>,

    /// Layer 4 Address 2
    pub MAC_LAYER4_ADDRESS2: RWRegister<u32>,

    _reserved28: [u32; 2],

    /// Layer 3 Address 0 Register 2
    pub MAC_LAYER3_ADDR0_REG2: RWRegister<u32>,

    /// Layer 3 Address 0 Register 2
    pub MAC_LAYER3_ADDR1_REG2: RWRegister<u32>,

    /// Layer 3 Address 2 Register 2
    pub MAC_LAYER3_ADDR2_REG2: RWRegister<u32>,

    /// Layer 3 Address 3 Register 2
    pub MAC_LAYER3_ADDR3_REG2: RWRegister<u32>,

    _reserved29: [u32; 4],

    /// Layer 3 and Layer 4 Control of Filter 3
    pub MAC_L3_L4_CONTROL3: RWRegister<u32>,

    /// Layer 4 Address 3
    pub MAC_LAYER4_ADDRESS3: RWRegister<u32>,

    _reserved30: [u32; 2],

    /// Layer 3 Address 0 Register 3
    pub MAC_LAYER3_ADDR0_REG3: RWRegister<u32>,

    /// Layer 3 Address 1 Register 3
    pub MAC_LAYER3_ADDR1_REG3: RWRegister<u32>,

    /// Layer 3 Address 2 Register 3
    pub MAC_LAYER3_ADDR2_REG3: RWRegister<u32>,

    /// Layer 3 Address 3 Register 3
    pub MAC_LAYER3_ADDR3_REG3: RWRegister<u32>,

    _reserved31: [u32; 4],

    /// Layer 3 and Layer 4 Control of Filter 4
    pub MAC_L3_L4_CONTROL4: RWRegister<u32>,

    /// Layer 4 Address 4
    pub MAC_LAYER4_ADDRESS4: RWRegister<u32>,

    _reserved32: [u32; 2],

    /// Layer 3 Address 0 Register 4
    pub MAC_LAYER3_ADDR0_REG4: RWRegister<u32>,

    /// Layer 3 Address 1 Register 4
    pub MAC_LAYER3_ADDR1_REG4: RWRegister<u32>,

    /// Layer 3 Address 2 Register 4
    pub MAC_LAYER3_ADDR2_REG4: RWRegister<u32>,

    /// Layer 3 Address 3 Register 4
    pub MAC_LAYER3_ADDR3_REG4: RWRegister<u32>,

    _reserved33: [u32; 4],

    /// Layer 3 and Layer 4 Control of Filter 5
    pub MAC_L3_L4_CONTROL5: RWRegister<u32>,

    /// Layer 4 Address 5
    pub MAC_LAYER4_ADDRESS5: RWRegister<u32>,

    _reserved34: [u32; 2],

    /// Layer 3 Address 0 Register 5
    pub MAC_LAYER3_ADDR0_REG5: RWRegister<u32>,

    /// Layer 3 Address 1 Register 5
    pub MAC_LAYER3_ADDR1_REG5: RWRegister<u32>,

    /// Layer 3 Address 2 Register 5
    pub MAC_LAYER3_ADDR2_REG5: RWRegister<u32>,

    /// Layer 3 Address 3 Register 5
    pub MAC_LAYER3_ADDR3_REG5: RWRegister<u32>,

    _reserved35: [u32; 4],

    /// Layer 3 and Layer 4 Control of Filter 6
    pub MAC_L3_L4_CONTROL6: RWRegister<u32>,

    /// Layer 4 Address 6
    pub MAC_LAYER4_ADDRESS6: RWRegister<u32>,

    _reserved36: [u32; 2],

    /// Layer 3 Address 0 Register 6
    pub MAC_LAYER3_ADDR0_REG6: RWRegister<u32>,

    /// Layer 3 Address 1 Register 6
    pub MAC_LAYER3_ADDR1_REG6: RWRegister<u32>,

    /// Layer 3 Address 2 Register 6
    pub MAC_LAYER3_ADDR2_REG6: RWRegister<u32>,

    /// Layer 3 Address 3 Register 6
    pub MAC_LAYER3_ADDR3_REG6: RWRegister<u32>,

    _reserved37: [u32; 4],

    /// Layer 3 and Layer 4 Control of Filter 0
    pub MAC_L3_L4_CONTROL7: RWRegister<u32>,

    /// Layer 4 Address 7
    pub MAC_LAYER4_ADDRESS7: RWRegister<u32>,

    _reserved38: [u32; 2],

    /// Layer 3 Address 0 Register 7
    pub MAC_LAYER3_ADDR0_REG7: RWRegister<u32>,

    /// Layer 3 Address 1 Register 7
    pub MAC_LAYER3_ADDR1_REG7: RWRegister<u32>,

    /// Layer 3 Address 2 Register 7
    pub MAC_LAYER3_ADDR2_REG7: RWRegister<u32>,

    /// Layer 3 Address 3 Register 7
    pub MAC_LAYER3_ADDR3_REG7: RWRegister<u32>,

    _reserved39: [u32; 36],

    /// Timestamp Control
    pub MAC_TIMESTAMP_CONTROL: RWRegister<u32>,

    /// Subsecond Increment
    pub MAC_SUB_SECOND_INCREMENT: RWRegister<u32>,

    /// System Time Seconds
    pub MAC_SYSTEM_TIME_SECONDS: RORegister<u32>,

    /// System Time Nanoseconds
    pub MAC_SYSTEM_TIME_NANOSECONDS: RORegister<u32>,

    /// System Time Seconds Update
    pub MAC_SYSTEM_TIME_SECONDS_UPDATE: RWRegister<u32>,

    /// System Time Nanoseconds Update
    pub MAC_SYSTEM_TIME_NANOSECONDS_UPDATE: RWRegister<u32>,

    /// Timestamp Addend
    pub MAC_TIMESTAMP_ADDEND: RWRegister<u32>,

    /// System Time - Higher Word Seconds
    pub MAC_SYSTEM_TIME_HIGHER_WORD_SECONDS: RWRegister<u32>,

    /// Timestamp Status
    pub MAC_TIMESTAMP_STATUS: RORegister<u32>,

    _reserved40: [u32; 3],

    /// Transmit Timestamp Status Nanoseconds
    pub MAC_TX_TIMESTAMP_STATUS_NANOSECONDS: RORegister<u32>,

    /// Transmit Timestamp Status Seconds
    pub MAC_TX_TIMESTAMP_STATUS_SECONDS: RORegister<u32>,

    _reserved41: [u32; 2],

    /// Auxiliary Timestamp Control
    pub MAC_AUXILIARY_CONTROL: RWRegister<u32>,

    _reserved42: [u32; 1],

    /// Auxiliary Timestamp Nanoseconds
    pub MAC_AUXILIARY_TIMESTAMP_NANOSECONDS: RORegister<u32>,

    /// Auxiliary Timestamp Seconds
    pub MAC_AUXILIARY_TIMESTAMP_SECONDS: RORegister<u32>,

    /// Timestamp Ingress Asymmetry Correction
    pub MAC_TIMESTAMP_INGRESS_ASYM_CORR: RWRegister<u32>,

    /// imestamp Egress Asymmetry Correction
    pub MAC_TIMESTAMP_EGRESS_ASYM_CORR: RWRegister<u32>,

    /// Timestamp Ingress Correction Nanosecond
    pub MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND: RWRegister<u32>,

    /// Timestamp Egress Correction Nanosecond
    pub MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND: RWRegister<u32>,

    /// Timestamp Ingress Correction Subnanosecond
    pub MAC_TIMESTAMP_INGRESS_CORR_SUBNANOSEC: RWRegister<u32>,

    /// Timestamp Egress Correction Subnanosecond
    pub MAC_TIMESTAMP_EGRESS_CORR_SUBNANOSEC: RWRegister<u32>,

    /// Timestamp Ingress Latency
    pub MAC_TIMESTAMP_INGRESS_LATENCY: RORegister<u32>,

    /// Timestamp Egress Latency
    pub MAC_TIMESTAMP_EGRESS_LATENCY: RORegister<u32>,

    /// PPS Control
    pub MAC_PPS_CONTROL: RWRegister<u32>,

    _reserved43: [u32; 3],

    /// PPS0 Target Time Seconds
    pub MAC_PPS0_TARGET_TIME_SECONDS: RWRegister<u32>,

    /// PPS0 Target Time Nanoseconds
    pub MAC_PPS0_TARGET_TIME_NANOSECONDS: RWRegister<u32>,

    /// PPS0 Interval
    pub MAC_PPS0_INTERVAL: RWRegister<u32>,

    /// PPS0 Width
    pub MAC_PPS0_WIDTH: RWRegister<u32>,

    /// PPS1 Target Time Seconds
    pub MAC_PPS1_TARGET_TIME_SECONDS: RWRegister<u32>,

    /// PPS1 Target Time Nanoseconds
    pub MAC_PPS1_TARGET_TIME_NANOSECONDS: RWRegister<u32>,

    /// PPS1 Interval
    pub MAC_PPS1_INTERVAL: RWRegister<u32>,

    /// PPS1 Width
    pub MAC_PPS1_WIDTH: RWRegister<u32>,

    /// PPS2 Target Time Seconds
    pub MAC_PPS2_TARGET_TIME_SECONDS: RWRegister<u32>,

    /// PPS2 Target Time Nanoseconds
    pub MAC_PPS2_TARGET_TIME_NANOSECONDS: RWRegister<u32>,

    /// PPS2 Interval
    pub MAC_PPS2_INTERVAL: RWRegister<u32>,

    /// PPS2 Width
    pub MAC_PPS2_WIDTH: RWRegister<u32>,

    /// PPS3 Target Time Seconds
    pub MAC_PPS3_TARGET_TIME_SECONDS: RWRegister<u32>,

    /// PPS3 Target Time Nanoseconds
    pub MAC_PPS3_TARGET_TIME_NANOSECONDS: RWRegister<u32>,

    /// PPS3 Interval
    pub MAC_PPS3_INTERVAL: RWRegister<u32>,

    /// PPS3 Width
    pub MAC_PPS3_WIDTH: RWRegister<u32>,

    /// PTP Offload Engine Control
    pub MAC_PTO_CONTROL: RWRegister<u32>,

    /// Source Port Identity 0
    pub MAC_SOURCE_PORT_IDENTITY0: RWRegister<u32>,

    /// Source Port Identity 1
    pub MAC_SOURCE_PORT_IDENTITY1: RWRegister<u32>,

    /// Source Port Identity 2
    pub MAC_SOURCE_PORT_IDENTITY2: RWRegister<u32>,

    /// Log Message Interval
    pub MAC_LOG_MESSAGE_INTERVAL: RWRegister<u32>,

    _reserved44: [u32; 11],

    /// MTL Operation Mode
    pub MTL_OPERATION_MODE: RWRegister<u32>,

    _reserved45: [u32; 1],

    /// FIFO Debug Access Control and Status
    pub MTL_DBG_CTL: RWRegister<u32>,

    /// FIFO Debug Status
    pub MTL_DBG_STS: RWRegister<u32>,

    /// FIFO Debug Data
    pub MTL_FIFO_DEBUG_DATA: RWRegister<u32>,

    _reserved46: [u32; 3],

    /// MTL Interrupt Status
    pub MTL_INTERRUPT_STATUS: RORegister<u32>,

    _reserved47: [u32; 3],

    /// Receive Queue and DMA Channel Mapping 0
    pub MTL_RXQ_DMA_MAP0: RWRegister<u32>,

    /// Receive Queue and DMA Channel Mapping 1
    pub MTL_RXQ_DMA_MAP1: RWRegister<u32>,

    _reserved48: [u32; 2],

    /// Time Based Scheduling Control
    pub MTL_TBS_CTRL: RWRegister<u32>,

    _reserved49: [u32; 3],

    /// Enhancements to Scheduled Transmission Control
    pub MTL_EST_CONTROL: RWRegister<u32>,

    _reserved50: [u32; 1],

    /// Enhancements to Scheduled Transmission Status
    pub MTL_EST_STATUS: RWRegister<u32>,

    _reserved51: [u32; 1],

    /// EST Scheduling Error
    pub MTL_EST_SCH_ERROR: RWRegister<u32>,

    /// EST Frame Size Error
    pub MTL_EST_FRM_SIZE_ERROR: RWRegister<u32>,

    /// EST Frame Size Capture
    pub MTL_EST_FRM_SIZE_CAPTURE: RORegister<u32>,

    _reserved52: [u32; 1],

    /// EST Interrupt Enable
    pub MTL_EST_INTR_ENABLE: RWRegister<u32>,

    _reserved53: [u32; 3],

    /// EST GCL Control
    pub MTL_EST_GCL_CONTROL: RWRegister<u32>,

    /// EST GCL Data
    pub MTL_EST_GCL_DATA: RWRegister<u32>,

    _reserved54: [u32; 2],

    /// Frame Preemption Control and Status
    pub MTL_FPE_CTRL_STS: RWRegister<u32>,

    /// Frame Preemption Hold and Release Advance
    pub MTL_FPE_ADVANCE: RWRegister<u32>,

    _reserved55: [u32; 2],

    /// RXP Control Status
    pub MTL_RXP_CONTROL_STATUS: RWRegister<u32>,

    /// RXP Interrupt Control Status
    pub MTL_RXP_INTERRUPT_CONTROL_STATUS: RWRegister<u32>,

    /// RXP Drop Count
    pub MTL_RXP_DROP_CNT: RORegister<u32>,

    /// RXP Error Count
    pub MTL_RXP_ERROR_CNT: RORegister<u32>,

    /// RXP Indirect Access Control and Status
    pub MTL_RXP_INDIRECT_ACC_CONTROL_STATUS: RWRegister<u32>,

    /// RXP Indirect Access Data
    pub MTL_RXP_INDIRECT_ACC_DATA: RWRegister<u32>,

    _reserved56: [u32; 18],

    /// Queue 0 Transmit Operation Mode
    pub MTL_TXQ0_OPERATION_MODE: RWRegister<u32>,

    /// Queue 0 Underflow Counter
    pub MTL_TXQ0_UNDERFLOW: RORegister<u32>,

    /// Queue 0 Transmit Debug
    pub MTL_TXQ0_DEBUG: RORegister<u32>,

    _reserved57: [u32; 2],

    /// Queue 0 ETS Status
    pub MTL_TXQ0_ETS_STATUS: RORegister<u32>,

    /// Queue 0 Quantum or Weights
    pub MTL_TXQ0_QUANTUM_WEIGHT: RWRegister<u32>,

    _reserved58: [u32; 4],

    /// Queue 0 Interrupt Control Status
    pub MTL_Q0_INTERRUPT_CONTROL_STATUS: RWRegister<u32>,

    /// Queue 0 Receive Operation Mode
    pub MTL_RXQ0_OPERATION_MODE: RWRegister<u32>,

    /// Queue 0 Missed Packet and Overflow Counter
    pub MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT: RORegister<u32>,

    /// Queue 0 Receive Debug
    pub MTL_RXQ0_DEBUG: RORegister<u32>,

    /// Queue 0 Receive Control
    pub MTL_RXQ0_CONTROL: RWRegister<u32>,

    /// Queue 1 Transmit Operation Mode
    pub MTL_TXQ1_OPERATION_MODE: RWRegister<u32>,

    /// Queue 1 Underflow Counter
    pub MTL_TXQ1_UNDERFLOW: RORegister<u32>,

    /// Queue 1 Transmit Debug
    pub MTL_TXQ1_DEBUG: RORegister<u32>,

    _reserved59: [u32; 1],

    /// Queue 1 ETS Control
    pub MTL_TXQ1_ETS_CONTROL: RWRegister<u32>,

    /// Queue 1 ETS Status
    pub MTL_TXQ1_ETS_STATUS: RORegister<u32>,

    /// Queue 1 idleSlopeCredit, Quantum or Weights
    pub MTL_TXQ1_QUANTUM_WEIGHT: RWRegister<u32>,

    /// Queue 1 sendSlopeCredit
    pub MTL_TXQ1_SENDSLOPECREDIT: RWRegister<u32>,

    /// Queue 1 hiCredit
    pub MTL_TXQ1_HICREDIT: RWRegister<u32>,

    /// Queue 1 loCredit
    pub MTL_TXQ1_LOCREDIT: RWRegister<u32>,

    _reserved60: [u32; 1],

    /// Queue 1 Interrupt Control Status
    pub MTL_Q1_INTERRUPT_CONTROL_STATUS: RWRegister<u32>,

    /// Queue 1 Receive Operation Mode
    pub MTL_RXQ1_OPERATION_MODE: RWRegister<u32>,

    /// Queue 1 Missed Packet and Overflow Counter
    pub MTL_RXQ1_MISSED_PACKET_OVERFLOW_CNT: RORegister<u32>,

    /// Queue 1 Receive Debug
    pub MTL_RXQ1_DEBUG: RORegister<u32>,

    /// Queue 1 Receive Control
    pub MTL_RXQ1_CONTROL: RWRegister<u32>,

    /// Queue 2 Transmit Operation Mode
    pub MTL_TXQ2_OPERATION_MODE: RWRegister<u32>,

    /// Queue 2 Underflow Counter
    pub MTL_TXQ2_UNDERFLOW: RORegister<u32>,

    /// Queue 2 Transmit Debug
    pub MTL_TXQ2_DEBUG: RORegister<u32>,

    _reserved61: [u32; 1],

    /// Queue 2 ETS Control
    pub MTL_TXQ2_ETS_CONTROL: RWRegister<u32>,

    /// Queue 2 ETS Status
    pub MTL_TXQ2_ETS_STATUS: RORegister<u32>,

    /// Queue 2 idleSlopeCredit, Quantum or Weights
    pub MTL_TXQ2_QUANTUM_WEIGHT: RWRegister<u32>,

    /// Queue 2 sendSlopeCredit
    pub MTL_TXQ2_SENDSLOPECREDIT: RWRegister<u32>,

    /// Queue 2 hiCredit
    pub MTL_TXQ2_HICREDIT: RWRegister<u32>,

    /// Queue 2 loCredit
    pub MTL_TXQ2_LOCREDIT: RWRegister<u32>,

    _reserved62: [u32; 1],

    /// Queue 2 Interrupt Control Status
    pub MTL_Q2_INTERRUPT_CONTROL_STATUS: RWRegister<u32>,

    /// Queue 2 Receive Operation Mode
    pub MTL_RXQ2_OPERATION_MODE: RWRegister<u32>,

    /// Queue 2 Missed Packet and Overflow Counter
    pub MTL_RXQ2_MISSED_PACKET_OVERFLOW_CNT: RORegister<u32>,

    /// Queue 2 Receive Debug
    pub MTL_RXQ2_DEBUG: RORegister<u32>,

    /// Queue 2 Receive Control
    pub MTL_RXQ2_CONTROL: RWRegister<u32>,

    /// Queue 3 Transmit Operation Mode
    pub MTL_TXQ3_OPERATION_MODE: RWRegister<u32>,

    /// Queue 3 Underflow Counter
    pub MTL_TXQ3_UNDERFLOW: RORegister<u32>,

    /// Queue 3 Transmit Debug
    pub MTL_TXQ3_DEBUG: RORegister<u32>,

    _reserved63: [u32; 1],

    /// Queue 3 ETS Control
    pub MTL_TXQ3_ETS_CONTROL: RWRegister<u32>,

    /// Queue 3 ETS Status
    pub MTL_TXQ3_ETS_STATUS: RORegister<u32>,

    /// Queue 3 idleSlopeCredit, Quantum or Weights
    pub MTL_TXQ3_QUANTUM_WEIGHT: RWRegister<u32>,

    /// Queue 3 sendSlopeCredit
    pub MTL_TXQ3_SENDSLOPECREDIT: RWRegister<u32>,

    /// Queue 3 hiCredit
    pub MTL_TXQ3_HICREDIT: RWRegister<u32>,

    /// Queue 3 loCredit
    pub MTL_TXQ3_LOCREDIT: RWRegister<u32>,

    _reserved64: [u32; 1],

    /// Queue 3 Interrupt Control Status
    pub MTL_Q3_INTERRUPT_CONTROL_STATUS: RWRegister<u32>,

    /// Queue 3 Receive Operation Mode
    pub MTL_RXQ3_OPERATION_MODE: RWRegister<u32>,

    /// Queue 3 Missed Packet and Overflow Counter
    pub MTL_RXQ3_MISSED_PACKET_OVERFLOW_CNT: RORegister<u32>,

    /// Queue 3 Receive Debug
    pub MTL_RXQ3_DEBUG: RORegister<u32>,

    /// Queue 3 Receive Control
    pub MTL_RXQ3_CONTROL: RWRegister<u32>,

    /// Queue 4 Transmit Operation Mode
    pub MTL_TXQ4_OPERATION_MODE: RWRegister<u32>,

    /// Queue 4 Underflow Counter
    pub MTL_TXQ4_UNDERFLOW: RORegister<u32>,

    /// Queue 4 Transmit Debug
    pub MTL_TXQ4_DEBUG: RORegister<u32>,

    _reserved65: [u32; 1],

    /// Queue 4 ETS Control
    pub MTL_TXQ4_ETS_CONTROL: RWRegister<u32>,

    /// Queue 4 ETS Status
    pub MTL_TXQ4_ETS_STATUS: RORegister<u32>,

    /// Queue 4 idleSlopeCredit, Quantum or Weights
    pub MTL_TXQ4_QUANTUM_WEIGHT: RWRegister<u32>,

    /// Queue 4 sendSlopeCredit
    pub MTL_TXQ4_SENDSLOPECREDIT: RWRegister<u32>,

    /// Queue 4 hiCredit
    pub MTL_TXQ4_HICREDIT: RWRegister<u32>,

    /// Queue 4 loCredit
    pub MTL_TXQ4_LOCREDIT: RWRegister<u32>,

    _reserved66: [u32; 1],

    /// Queue 4 Interrupt Control Status
    pub MTL_Q4_INTERRUPT_CONTROL_STATUS: RWRegister<u32>,

    /// Queue 4 Receive Operation Mode
    pub MTL_RXQ4_OPERATION_MODE: RWRegister<u32>,

    /// Queue 4 Missed Packet and Overflow Counter
    pub MTL_RXQ4_MISSED_PACKET_OVERFLOW_CNT: RORegister<u32>,

    /// Queue 4 Receive Debug
    pub MTL_RXQ4_DEBUG: RORegister<u32>,

    /// Queue 4 Receive Control
    pub MTL_RXQ4_CONTROL: RWRegister<u32>,

    _reserved67: [u32; 112],

    /// DMA Bus Mode
    pub DMA_MODE: RWRegister<u32>,

    /// DMA System Bus Mode
    pub DMA_SYSBUS_MODE: RWRegister<u32>,

    /// DMA Interrupt Status
    pub DMA_INTERRUPT_STATUS: RORegister<u32>,

    /// DMA Debug Status 0
    pub DMA_DEBUG_STATUS0: RORegister<u32>,

    /// DMA Debug Status 1
    pub DMA_DEBUG_STATUS1: RORegister<u32>,

    _reserved68: [u32; 11],

    /// AXI LPI Entry Interval Control
    pub DMA_AXI_LPI_ENTRY_INTERVAL: RWRegister<u32>,

    _reserved69: [u32; 3],

    /// TBS Control
    pub DMA_TBS_CTRL: RWRegister<u32>,

    _reserved70: [u32; 43],

    /// DMA Channel 0 Control
    pub DMA_CH0_CONTROL: RWRegister<u32>,

    /// DMA Channel 0 Transmit Control
    pub DMA_CH0_TX_CONTROL: RWRegister<u32>,

    /// DMA Channel 0 Receive Control
    pub DMA_CH0_RX_CONTROL: RWRegister<u32>,

    _reserved71: [u32; 2],

    /// Channel 0 Tx Descriptor List Address register
    pub DMA_CH0_TXDESC_LIST_ADDRESS: RWRegister<u32>,

    _reserved72: [u32; 1],

    /// Channel 0 Rx Descriptor List Address register
    pub DMA_CH0_RXDESC_LIST_ADDRESS: RWRegister<u32>,

    /// Channel 0 Tx Descriptor Tail Pointer
    pub DMA_CH0_TXDESC_TAIL_POINTER: RWRegister<u32>,

    _reserved73: [u32; 1],

    /// Channel 0 Rx Descriptor Tail Pointer
    pub DMA_CH0_RXDESC_TAIL_POINTER: RWRegister<u32>,

    /// Channel 0 Tx Descriptor Ring Length
    pub DMA_CH0_TXDESC_RING_LENGTH: RWRegister<u32>,

    /// Channel 0 Rx Descriptor Ring Length
    pub DMA_CH0_RXDESC_RING_LENGTH: RWRegister<u32>,

    /// Channel 0 Interrupt Enable
    pub DMA_CH0_INTERRUPT_ENABLE: RWRegister<u32>,

    /// Channel 0 Receive Interrupt Watchdog Timer
    pub DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER: RWRegister<u32>,

    /// Channel 0 Slot Function Control and Status
    pub DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS: RWRegister<u32>,

    _reserved74: [u32; 1],

    /// Channel 0 Current Application Transmit Descriptor
    pub DMA_CH0_CURRENT_APP_TXDESC: RORegister<u32>,

    _reserved75: [u32; 1],

    /// Channel 0 Current Application Receive Descriptor
    pub DMA_CH0_CURRENT_APP_RXDESC: RORegister<u32>,

    _reserved76: [u32; 1],

    /// Channel 0 Current Application Transmit Buffer Address
    pub DMA_CH0_CURRENT_APP_TXBUFFER: RORegister<u32>,

    _reserved77: [u32; 1],

    /// Channel 0 Current Application Receive Buffer Address
    pub DMA_CH0_CURRENT_APP_RXBUFFER: RORegister<u32>,

    /// DMA Channel 0 Status
    pub DMA_CH0_STATUS: RWRegister<u32>,

    /// Channel 0 Missed Frame Counter
    pub DMA_CH0_MISS_FRAME_CNT: RORegister<u32>,

    /// Channel 0 RXP Frames Accepted Counter
    pub DMA_CH0_RXP_ACCEPT_CNT: RORegister<u32>,

    /// Channel 0 Receive ERI Counter
    pub DMA_CH0_RX_ERI_CNT: RORegister<u32>,

    _reserved78: [u32; 4],

    /// DMA Channel 1 Control
    pub DMA_CH1_CONTROL: RWRegister<u32>,

    /// DMA Channel 1 Transmit Control
    pub DMA_CH1_TX_CONTROL: RWRegister<u32>,

    /// DMA Channel 1 Receive Control
    pub DMA_CH1_RX_CONTROL: RWRegister<u32>,

    _reserved79: [u32; 2],

    /// Channel 1 Tx Descriptor List Address
    pub DMA_CH1_TXDESC_LIST_ADDRESS: RWRegister<u32>,

    _reserved80: [u32; 1],

    /// Channel 1 Rx Descriptor List Address
    pub DMA_CH1_RXDESC_LIST_ADDRESS: RWRegister<u32>,

    /// Channel 1 Tx Descriptor Tail Pointer
    pub DMA_CH1_TXDESC_TAIL_POINTER: RWRegister<u32>,

    _reserved81: [u32; 1],

    /// Channel 1 Rx Descriptor Tail Pointer
    pub DMA_CH1_RXDESC_TAIL_POINTER: RWRegister<u32>,

    /// Channel 1 Tx Descriptor Ring Length
    pub DMA_CH1_TXDESC_RING_LENGTH: RWRegister<u32>,

    /// Channel 1 Rx Descriptor Ring Length
    pub DMA_CH1_RXDESC_RING_LENGTH: RWRegister<u32>,

    /// Channel 1 Interrupt Enable
    pub DMA_CH1_INTERRUPT_ENABLE: RWRegister<u32>,

    /// Channel 1 Receive Interrupt Watchdog Timer
    pub DMA_CH1_RX_INTERRUPT_WATCHDOG_TIMER: RWRegister<u32>,

    /// Channel 1 Slot Function Control and Status
    pub DMA_CH1_SLOT_FUNCTION_CONTROL_STATUS: RWRegister<u32>,

    _reserved82: [u32; 1],

    /// Channel 1 Current Application Transmit Descriptor
    pub DMA_CH1_CURRENT_APP_TXDESC: RORegister<u32>,

    _reserved83: [u32; 1],

    /// Channel 1 Current Application Receive Descriptor
    pub DMA_CH1_CURRENT_APP_RXDESC: RORegister<u32>,

    _reserved84: [u32; 1],

    /// Channel 1 Current Application Transmit Buffer Address
    pub DMA_CH1_CURRENT_APP_TXBUFFER: RORegister<u32>,

    _reserved85: [u32; 1],

    /// Channel 1 Current Application Receive Buffer Address
    pub DMA_CH1_CURRENT_APP_RXBUFFER: RORegister<u32>,

    /// DMA Channel 1 Status
    pub DMA_CH1_STATUS: RWRegister<u32>,

    /// Channel 1 Missed Frame Counter
    pub DMA_CH1_MISS_FRAME_CNT: RORegister<u32>,

    /// Channel 1 RXP Frames Accepted Counter
    pub DMA_CH1_RXP_ACCEPT_CNT: RORegister<u32>,

    /// Channel 1 Receive ERI Counter
    pub DMA_CH1_RX_ERI_CNT: RORegister<u32>,

    _reserved86: [u32; 4],

    /// DMA Channel 2 Control
    pub DMA_CH2_CONTROL: RWRegister<u32>,

    /// DMA Channel 2 Transmit Control
    pub DMA_CH2_TX_CONTROL: RWRegister<u32>,

    /// DMA Channel 2 Receive Control
    pub DMA_CH2_RX_CONTROL: RWRegister<u32>,

    _reserved87: [u32; 2],

    /// Channel 2 Tx Descriptor List Address
    pub DMA_CH2_TXDESC_LIST_ADDRESS: RWRegister<u32>,

    _reserved88: [u32; 1],

    /// Channel 2 Rx Descriptor List Address
    pub DMA_CH2_RXDESC_LIST_ADDRESS: RWRegister<u32>,

    /// Channel 2 Tx Descriptor Tail Pointer
    pub DMA_CH2_TXDESC_TAIL_POINTER: RWRegister<u32>,

    _reserved89: [u32; 1],

    /// Channel 2 Rx Descriptor Tail Pointer
    pub DMA_CH2_RXDESC_TAIL_POINTER: RWRegister<u32>,

    /// Channel 2 Tx Descriptor Ring Length
    pub DMA_CH2_TXDESC_RING_LENGTH: RWRegister<u32>,

    /// Channel 2 Rx Descriptor Ring Length
    pub DMA_CH2_RXDESC_RING_LENGTH: RWRegister<u32>,

    /// Channel 2 Interrupt Enable
    pub DMA_CH2_INTERRUPT_ENABLE: RWRegister<u32>,

    /// Channel 2 Receive Interrupt Watchdog Timer
    pub DMA_CH2_RX_INTERRUPT_WATCHDOG_TIMER: RWRegister<u32>,

    /// Channel 2 Slot Function Control and Status
    pub DMA_CH2_SLOT_FUNCTION_CONTROL_STATUS: RWRegister<u32>,

    _reserved90: [u32; 1],

    /// Channel 2 Current Application Transmit Descriptor
    pub DMA_CH2_CURRENT_APP_TXDESC: RORegister<u32>,

    _reserved91: [u32; 1],

    /// Channel 2 Current Application Receive Descriptor
    pub DMA_CH2_CURRENT_APP_RXDESC: RORegister<u32>,

    _reserved92: [u32; 1],

    /// Channel 2 Current Application Transmit Buffer Address
    pub DMA_CH2_CURRENT_APP_TXBUFFER: RORegister<u32>,

    _reserved93: [u32; 1],

    /// Channel 2 Current Application Receive Buffer Address
    pub DMA_CH2_CURRENT_APP_RXBUFFER: RORegister<u32>,

    /// DMA Channel 2 Status
    pub DMA_CH2_STATUS: RWRegister<u32>,

    /// Channel 2 Missed Frame Counter
    pub DMA_CH2_MISS_FRAME_CNT: RORegister<u32>,

    /// Channel 2 RXP Frames Accepted Counter
    pub DMA_CH2_RXP_ACCEPT_CNT: RORegister<u32>,

    /// Channel 2 Receive ERI Counter
    pub DMA_CH2_RX_ERI_CNT: RORegister<u32>,

    _reserved94: [u32; 4],

    /// DMA Channel 3 Control
    pub DMA_CH3_CONTROL: RWRegister<u32>,

    /// DMA Channel 3 Transmit Control
    pub DMA_CH3_TX_CONTROL: RWRegister<u32>,

    /// DMA Channel 3 Receive Control
    pub DMA_CH3_RX_CONTROL: RWRegister<u32>,

    _reserved95: [u32; 2],

    /// Channel 3 Tx Descriptor List Address
    pub DMA_CH3_TXDESC_LIST_ADDRESS: RWRegister<u32>,

    _reserved96: [u32; 1],

    /// Channel 3 Rx Descriptor List Address
    pub DMA_CH3_RXDESC_LIST_ADDRESS: RWRegister<u32>,

    /// Channel 3 Tx Descriptor Tail Pointer
    pub DMA_CH3_TXDESC_TAIL_POINTER: RWRegister<u32>,

    _reserved97: [u32; 1],

    /// Channel 3 Rx Descriptor Tail Pointer
    pub DMA_CH3_RXDESC_TAIL_POINTER: RWRegister<u32>,

    /// Channel 3 Tx Descriptor Ring Length
    pub DMA_CH3_TXDESC_RING_LENGTH: RWRegister<u32>,

    /// Channel 3 Rx Descriptor Ring Length
    pub DMA_CH3_RXDESC_RING_LENGTH: RWRegister<u32>,

    /// Channel 3 Interrupt Enable
    pub DMA_CH3_INTERRUPT_ENABLE: RWRegister<u32>,

    /// Channel 3 Receive Interrupt Watchdog Time
    pub DMA_CH3_RX_INTERRUPT_WATCHDOG_TIMER: RWRegister<u32>,

    /// Channel 3 Slot Function Control and Status
    pub DMA_CH3_SLOT_FUNCTION_CONTROL_STATUS: RWRegister<u32>,

    _reserved98: [u32; 1],

    /// Channel 3 Current Application Transmit Descriptor
    pub DMA_CH3_CURRENT_APP_TXDESC: RORegister<u32>,

    _reserved99: [u32; 1],

    /// Channel 3 Current Application Receive Descriptor
    pub DMA_CH3_CURRENT_APP_RXDESC: RORegister<u32>,

    _reserved100: [u32; 1],

    /// Channel 3 Current Application Transmit Buffer Address
    pub DMA_CH3_CURRENT_APP_TXBUFFER: RORegister<u32>,

    _reserved101: [u32; 1],

    /// Channel 3 Current Application Receive Buffer Address
    pub DMA_CH3_CURRENT_APP_RXBUFFER: RORegister<u32>,

    /// DMA Channel 3 Status
    pub DMA_CH3_STATUS: RWRegister<u32>,

    /// Channel 3 Missed Frame Counter
    pub DMA_CH3_MISS_FRAME_CNT: RORegister<u32>,

    /// Channel 3 RXP Frames Accepted Counter
    pub DMA_CH3_RXP_ACCEPT_CNT: RORegister<u32>,

    /// Channel 3 Receive ERI Counter
    pub DMA_CH3_RX_ERI_CNT: RORegister<u32>,

    _reserved102: [u32; 4],

    /// DMA Channel 4 Control
    pub DMA_CH4_CONTROL: RWRegister<u32>,

    /// DMA Channel 4 Transmit Control
    pub DMA_CH4_TX_CONTROL: RWRegister<u32>,

    /// DMA Channel 4 Receive Control
    pub DMA_CH4_RX_CONTROL: RWRegister<u32>,

    _reserved103: [u32; 2],

    /// Channel 4 Tx Descriptor List Address
    pub DMA_CH4_TXDESC_LIST_ADDRESS: RWRegister<u32>,

    _reserved104: [u32; 1],

    /// Channel 4 Rx Descriptor List Address
    pub DMA_CH4_RXDESC_LIST_ADDRESS: RWRegister<u32>,

    /// Channel 4 Tx Descriptor Tail Pointer
    pub DMA_CH4_TXDESC_TAIL_POINTER: RWRegister<u32>,

    _reserved105: [u32; 1],

    /// Channel 4 Rx Descriptor Tail Pointer
    pub DMA_CH4_RXDESC_TAIL_POINTER: RWRegister<u32>,

    /// Channel 4 Tx Descriptor Ring Length
    pub DMA_CH4_TXDESC_RING_LENGTH: RWRegister<u32>,

    /// Channel 4 Rx Descriptor Ring Length
    pub DMA_CH4_RXDESC_RING_LENGTH: RWRegister<u32>,

    /// Channel 4 Interrupt Enable
    pub DMA_CH4_INTERRUPT_ENABLE: RWRegister<u32>,

    /// Channel 4 Receive Interrupt Watchdog Timer
    pub DMA_CH4_RX_INTERRUPT_WATCHDOG_TIMER: RWRegister<u32>,

    /// Channel 4 Slot Function Control and Status
    pub DMA_CH4_SLOT_FUNCTION_CONTROL_STATUS: RWRegister<u32>,

    _reserved106: [u32; 1],

    /// Channel 4 Current Application Transmit Descriptor
    pub DMA_CH4_CURRENT_APP_TXDESC: RORegister<u32>,

    _reserved107: [u32; 1],

    /// Channel 4 Current Application Receive Descriptor
    pub DMA_CH4_CURRENT_APP_RXDESC: RORegister<u32>,

    _reserved108: [u32; 1],

    /// Channel 4 Current Application Transmit Buffer Address
    pub DMA_CH4_CURRENT_APP_TXBUFFER: RORegister<u32>,

    _reserved109: [u32; 1],

    /// Channel 4 Current Application Receive Buffer Address
    pub DMA_CH4_CURRENT_APP_RXBUFFER: RORegister<u32>,

    /// DMA Channel 4 Status
    pub DMA_CH4_STATUS: RWRegister<u32>,

    /// Channel 4 Missed Frame Counter
    pub DMA_CH4_MISS_FRAME_CNT: RORegister<u32>,

    /// Channel 4 RXP Frames Accepted Counter
    pub DMA_CH4_RXP_ACCEPT_CNT: RORegister<u32>,

    /// Channel 4 Receive ERI Counter
    pub DMA_CH4_RX_ERI_CNT: RORegister<u32>,
}
pub struct ResetValues {
    pub MAC_CONFIGURATION: u32,
    pub MAC_EXT_CONFIGURATION: u32,
    pub MAC_PACKET_FILTER: u32,
    pub MAC_WATCHDOG_TIMEOUT: u32,
    pub MAC_HASH_TABLE_REG0: u32,
    pub MAC_HASH_TABLE_REG1: u32,
    pub MAC_VLAN_TAG_CTRL: u32,
    pub MAC_VLAN_TAG_DATA: u32,
    pub MAC_VLAN_HASH_TABLE: u32,
    pub MAC_VLAN_INCL: u32,
    pub MAC_INNER_VLAN_INCL: u32,
    pub MAC_Q0_TX_FLOW_CTRL: u32,
    pub MAC_Q1_TX_FLOW_CTRL: u32,
    pub MAC_Q2_TX_FLOW_CTRL: u32,
    pub MAC_Q3_TX_FLOW_CTRL: u32,
    pub MAC_Q4_TX_FLOW_CTRL: u32,
    pub MAC_RX_FLOW_CTRL: u32,
    pub MAC_RXQ_CTRL4: u32,
    pub MAC_TXQ_PRTY_MAP0: u32,
    pub MAC_TXQ_PRTY_MAP1: u32,
    pub MAC_RXQ_CTRL0: u32,
    pub MAC_RXQ_CTRL1: u32,
    pub MAC_RXQ_CTRL2: u32,
    pub MAC_RXQ_CTRL3: u32,
    pub MAC_INTERRUPT_STATUS: u32,
    pub MAC_INTERRUPT_ENABLE: u32,
    pub MAC_RX_TX_STATUS: u32,
    pub MAC_PMT_CONTROL_STATUS: u32,
    pub MAC_RWK_PACKET_FILTER: u32,
    pub MAC_LPI_CONTROL_STATUS: u32,
    pub MAC_LPI_TIMERS_CONTROL: u32,
    pub MAC_LPI_ENTRY_TIMER: u32,
    pub MAC_ONEUS_TIC_COUNTER: u32,
    pub MAC_PHYIF_CONTROL_STATUS: u32,
    pub MAC_VERSION: u32,
    pub MAC_DEBUG: u32,
    pub MAC_HW_FEATURE0: u32,
    pub MAC_HW_FEATURE1: u32,
    pub MAC_HW_FEATURE2: u32,
    pub MAC_HW_FEATURE3: u32,
    pub MAC_MDIO_ADDRESS: u32,
    pub MAC_MDIO_DATA: u32,
    pub MAC_CSR_SW_CTRL: u32,
    pub MAC_FPE_CTRL_STS: u32,
    pub MAC_PRESN_TIME_NS: u32,
    pub MAC_PRESN_TIME_UPDT: u32,
    pub MAC_ADDRESS0_HIGH: u32,
    pub MAC_ADDRESS0_LOW: u32,
    pub MAC_ADDRESS1_HIGH: u32,
    pub MAC_ADDRESS1_LOW: u32,
    pub MAC_ADDRESS2_HIGH: u32,
    pub MAC_ADDRESS2_LOW: u32,
    pub MAC_ADDRESS3_HIGH: u32,
    pub MAC_ADDRESS3_LOW: u32,
    pub MAC_ADDRESS4_HIGH: u32,
    pub MAC_ADDRESS4_LOW: u32,
    pub MAC_ADDRESS5_HIGH: u32,
    pub MAC_ADDRESS5_LOW: u32,
    pub MAC_ADDRESS6_HIGH: u32,
    pub MAC_ADDRESS6_LOW: u32,
    pub MAC_ADDRESS7_HIGH: u32,
    pub MAC_ADDRESS7_LOW: u32,
    pub MAC_ADDRESS8_HIGH: u32,
    pub MAC_ADDRESS8_LOW: u32,
    pub MAC_ADDRESS9_HIGH: u32,
    pub MAC_ADDRESS9_LOW: u32,
    pub MAC_ADDRESS10_HIGH: u32,
    pub MAC_ADDRESS10_LOW: u32,
    pub MAC_ADDRESS11_HIGH: u32,
    pub MAC_ADDRESS11_LOW: u32,
    pub MAC_ADDRESS12_HIGH: u32,
    pub MAC_ADDRESS12_LOW: u32,
    pub MAC_ADDRESS13_HIGH: u32,
    pub MAC_ADDRESS13_LOW: u32,
    pub MAC_ADDRESS14_HIGH: u32,
    pub MAC_ADDRESS14_LOW: u32,
    pub MAC_ADDRESS15_HIGH: u32,
    pub MAC_ADDRESS15_LOW: u32,
    pub MAC_ADDRESS16_HIGH: u32,
    pub MAC_ADDRESS16_LOW: u32,
    pub MAC_ADDRESS17_HIGH: u32,
    pub MAC_ADDRESS17_LOW: u32,
    pub MAC_ADDRESS18_HIGH: u32,
    pub MAC_ADDRESS18_LOW: u32,
    pub MAC_ADDRESS19_HIGH: u32,
    pub MAC_ADDRESS19_LOW: u32,
    pub MAC_ADDRESS20_HIGH: u32,
    pub MAC_ADDRESS20_LOW: u32,
    pub MAC_ADDRESS21_HIGH: u32,
    pub MAC_ADDRESS21_LOW: u32,
    pub MAC_ADDRESS22_HIGH: u32,
    pub MAC_ADDRESS22_LOW: u32,
    pub MAC_ADDRESS23_HIGH: u32,
    pub MAC_ADDRESS23_LOW: u32,
    pub MAC_ADDRESS24_HIGH: u32,
    pub MAC_ADDRESS24_LOW: u32,
    pub MAC_ADDRESS25_HIGH: u32,
    pub MAC_ADDRESS25_LOW: u32,
    pub MAC_ADDRESS26_HIGH: u32,
    pub MAC_ADDRESS26_LOW: u32,
    pub MAC_ADDRESS27_HIGH: u32,
    pub MAC_ADDRESS27_LOW: u32,
    pub MAC_ADDRESS28_HIGH: u32,
    pub MAC_ADDRESS28_LOW: u32,
    pub MAC_ADDRESS29_HIGH: u32,
    pub MAC_ADDRESS29_LOW: u32,
    pub MAC_ADDRESS30_HIGH: u32,
    pub MAC_ADDRESS30_LOW: u32,
    pub MAC_ADDRESS31_HIGH: u32,
    pub MAC_ADDRESS31_LOW: u32,
    pub MAC_ADDRESS32_HIGH: u32,
    pub MAC_ADDRESS32_LOW: u32,
    pub MAC_ADDRESS33_HIGH: u32,
    pub MAC_ADDRESS33_LOW: u32,
    pub MAC_ADDRESS34_HIGH: u32,
    pub MAC_ADDRESS34_LOW: u32,
    pub MAC_ADDRESS35_HIGH: u32,
    pub MAC_ADDRESS35_LOW: u32,
    pub MAC_ADDRESS36_HIGH: u32,
    pub MAC_ADDRESS36_LOW: u32,
    pub MAC_ADDRESS37_HIGH: u32,
    pub MAC_ADDRESS37_LOW: u32,
    pub MAC_ADDRESS38_HIGH: u32,
    pub MAC_ADDRESS38_LOW: u32,
    pub MAC_ADDRESS39_HIGH: u32,
    pub MAC_ADDRESS39_LOW: u32,
    pub MAC_ADDRESS40_HIGH: u32,
    pub MAC_ADDRESS40_LOW: u32,
    pub MAC_ADDRESS41_HIGH: u32,
    pub MAC_ADDRESS41_LOW: u32,
    pub MAC_ADDRESS42_HIGH: u32,
    pub MAC_ADDRESS42_LOW: u32,
    pub MAC_ADDRESS43_HIGH: u32,
    pub MAC_ADDRESS43_LOW: u32,
    pub MAC_ADDRESS44_HIGH: u32,
    pub MAC_ADDRESS44_LOW: u32,
    pub MAC_ADDRESS45_HIGH: u32,
    pub MAC_ADDRESS45_LOW: u32,
    pub MAC_ADDRESS46_HIGH: u32,
    pub MAC_ADDRESS46_LOW: u32,
    pub MAC_ADDRESS47_HIGH: u32,
    pub MAC_ADDRESS47_LOW: u32,
    pub MAC_ADDRESS48_HIGH: u32,
    pub MAC_ADDRESS48_LOW: u32,
    pub MAC_ADDRESS49_HIGH: u32,
    pub MAC_ADDRESS49_LOW: u32,
    pub MAC_ADDRESS50_HIGH: u32,
    pub MAC_ADDRESS50_LOW: u32,
    pub MAC_ADDRESS51_HIGH: u32,
    pub MAC_ADDRESS51_LOW: u32,
    pub MAC_ADDRESS52_HIGH: u32,
    pub MAC_ADDRESS52_LOW: u32,
    pub MAC_ADDRESS53_HIGH: u32,
    pub MAC_ADDRESS53_LOW: u32,
    pub MAC_ADDRESS54_HIGH: u32,
    pub MAC_ADDRESS54_LOW: u32,
    pub MAC_ADDRESS55_HIGH: u32,
    pub MAC_ADDRESS55_LOW: u32,
    pub MAC_ADDRESS56_HIGH: u32,
    pub MAC_ADDRESS56_LOW: u32,
    pub MAC_ADDRESS57_HIGH: u32,
    pub MAC_ADDRESS57_LOW: u32,
    pub MAC_ADDRESS58_HIGH: u32,
    pub MAC_ADDRESS58_LOW: u32,
    pub MAC_ADDRESS59_HIGH: u32,
    pub MAC_ADDRESS59_LOW: u32,
    pub MAC_ADDRESS60_HIGH: u32,
    pub MAC_ADDRESS60_LOW: u32,
    pub MAC_ADDRESS61_HIGH: u32,
    pub MAC_ADDRESS61_LOW: u32,
    pub MAC_ADDRESS62_HIGH: u32,
    pub MAC_ADDRESS62_LOW: u32,
    pub MAC_ADDRESS63_HIGH: u32,
    pub MAC_ADDRESS63_LOW: u32,
    pub MAC_MMC_CONTROL: u32,
    pub MAC_MMC_RX_INTERRUPT: u32,
    pub MAC_MMC_TX_INTERRUPT: u32,
    pub MAC_MMC_RX_INTERRUPT_MASK: u32,
    pub MAC_MMC_TX_INTERRUPT_MASK: u32,
    pub MAC_TX_OCTET_COUNT_GOOD_BAD: u32,
    pub MAC_TX_PACKET_COUNT_GOOD_BAD: u32,
    pub MAC_TX_BROADCAST_PACKETS_GOOD: u32,
    pub MAC_TX_MULTICAST_PACKETS_GOOD: u32,
    pub MAC_TX_64OCTETS_PACKETS_GOOD_BAD: u32,
    pub MAC_TX_65TO127OCTETS_PACKETS_GOOD_BAD: u32,
    pub MAC_TX_128TO255OCTETS_PACKETS_GOOD_BAD: u32,
    pub MAC_TX_256TO511OCTETS_PACKETS_GOOD_BAD: u32,
    pub MAC_TX_512TO1023OCTETS_PACKETS_GOOD_BAD: u32,
    pub MAC_TX_1024TOMAXOCTETS_PACKETS_GOOD_BAD: u32,
    pub MAC_TX_UNICAST_PACKETS_GOOD_BAD: u32,
    pub MAC_TX_MULTICAST_PACKETS_GOOD_BAD: u32,
    pub MAC_TX_BROADCAST_PACKETS_GOOD_BAD: u32,
    pub MAC_TX_UNDERFLOW_ERROR_PACKETS: u32,
    pub MAC_TX_SINGLE_COLLISION_GOOD_PACKETS: u32,
    pub MAC_TX_MULTIPLE_COLLISION_GOOD_PACKETS: u32,
    pub MAC_TX_DEFERRED_PACKETS: u32,
    pub MAC_TX_LATE_COLLISION_PACKETS: u32,
    pub MAC_TX_EXCESSIVE_COLLISION_PACKETS: u32,
    pub MAC_TX_CARRIER_ERROR_PACKETS: u32,
    pub MAC_TX_OCTET_COUNT_GOOD: u32,
    pub MAC_TX_PACKET_COUNT_GOOD: u32,
    pub MAC_TX_EXCESSIVE_DEFERRAL_ERROR: u32,
    pub MAC_TX_PAUSE_PACKETS: u32,
    pub MAC_TX_VLAN_PACKETS_GOOD: u32,
    pub MAC_TX_OSIZE_PACKETS_GOOD: u32,
    pub MAC_RX_PACKETS_COUNT_GOOD_BAD: u32,
    pub MAC_RX_OCTET_COUNT_GOOD_BAD: u32,
    pub MAC_RX_OCTET_COUNT_GOOD: u32,
    pub MAC_RX_BROADCAST_PACKETS_GOOD: u32,
    pub MAC_RX_MULTICAST_PACKETS_GOOD: u32,
    pub MAC_RX_CRC_ERROR_PACKETS: u32,
    pub MAC_RX_ALIGNMENT_ERROR_PACKETS: u32,
    pub MAC_RX_RUNT_ERROR_PACKETS: u32,
    pub MAC_RX_JABBER_ERROR_PACKETS: u32,
    pub MAC_RX_UNDERSIZE_PACKETS_GOOD: u32,
    pub MAC_RX_OVERSIZE_PACKETS_GOOD: u32,
    pub MAC_RX_64OCTETS_PACKETS_GOOD_BAD: u32,
    pub MAC_RX_65TO127OCTETS_PACKETS_GOOD_BAD: u32,
    pub MAC_RX_128TO255OCTETS_PACKETS_GOOD_BAD: u32,
    pub MAC_RX_256TO511OCTETS_PACKETS_GOOD_BAD: u32,
    pub MAC_RX_512TO1023OCTETS_PACKETS_GOOD_BAD: u32,
    pub MAC_RX_1024TOMAXOCTETS_PACKETS_GOOD_BAD: u32,
    pub MAC_RX_UNICAST_PACKETS_GOOD: u32,
    pub MAC_RX_LENGTH_ERROR_PACKETS: u32,
    pub MAC_RX_OUT_OF_RANGE_TYPE_PACKETS: u32,
    pub MAC_RX_PAUSE_PACKETS: u32,
    pub MAC_RX_FIFO_OVERFLOW_PACKETS: u32,
    pub MAC_RX_VLAN_PACKETS_GOOD_BAD: u32,
    pub MAC_RX_WATCHDOG_ERROR_PACKETS: u32,
    pub MAC_RX_RECEIVE_ERROR_PACKETS: u32,
    pub MAC_RX_CONTROL_PACKETS_GOOD: u32,
    pub MAC_TX_LPI_USEC_CNTR: u32,
    pub MAC_TX_LPI_TRAN_CNTR: u32,
    pub MAC_RX_LPI_USEC_CNTR: u32,
    pub MAC_RX_LPI_TRAN_CNTR: u32,
    pub MAC_MMC_IPC_RX_INTERRUPT_MASK: u32,
    pub MAC_MMC_IPC_RX_INTERRUPT: u32,
    pub MAC_RXIPV4_GOOD_PACKETS: u32,
    pub MAC_RXIPV4_HEADER_ERROR_PACKETS: u32,
    pub MAC_RXIPV4_NO_PAYLOAD_PACKETS: u32,
    pub MAC_RXIPV4_FRAGMENTED_PACKETS: u32,
    pub MAC_RXIPV4_UDP_CHECKSUM_DISABLED_PACKETS: u32,
    pub MAC_RXIPV6_GOOD_PACKETS: u32,
    pub MAC_RXIPV6_HEADER_ERROR_PACKETS: u32,
    pub MAC_RXIPV6_NO_PAYLOAD_PACKETS: u32,
    pub MAC_RXUDP_GOOD_PACKETS: u32,
    pub MAC_RXUDP_ERROR_PACKETS: u32,
    pub MAC_RXTCP_GOOD_PACKETS: u32,
    pub MAC_RXTCP_ERROR_PACKETS: u32,
    pub MAC_RXICMP_GOOD_PACKETS: u32,
    pub MAC_RXICMP_ERROR_PACKETS: u32,
    pub MAC_RXIPV4_GOOD_OCTETS: u32,
    pub MAC_RXIPV4_HEADER_ERROR_OCTETS: u32,
    pub MAC_RXIPV4_NO_PAYLOAD_OCTETS: u32,
    pub MAC_RXIPV4_FRAGMENTED_OCTETS: u32,
    pub MAC_RXIPV4_UDP_CHECKSUM_DISABLE_OCTETS: u32,
    pub MAC_RXIPV6_GOOD_OCTETS: u32,
    pub MAC_RXIPV6_HEADER_ERROR_OCTETS: u32,
    pub MAC_RXIPV6_NO_PAYLOAD_OCTETS: u32,
    pub MAC_RXUDP_GOOD_OCTETS: u32,
    pub MAC_RXUDP_ERROR_OCTETS: u32,
    pub MAC_RXTCP_GOOD_OCTETS: u32,
    pub MAC_RXTCP_ERROR_OCTETS: u32,
    pub MAC_RXICMP_GOOD_OCTETS: u32,
    pub MAC_RXICMP_ERROR_OCTETS: u32,
    pub MAC_MMC_FPE_TX_INTERRUPT: u32,
    pub MAC_MMC_FPE_TX_INTERRUPT_MASK: u32,
    pub MAC_MMC_TX_FPE_FRAGMENT_CNTR: u32,
    pub MAC_MMC_TX_HOLD_REQ_CNTR: u32,
    pub MAC_MMC_FPE_RX_INTERRUPT: u32,
    pub MAC_MMC_FPE_RX_INTERRUPT_MASK: u32,
    pub MAC_MMC_RX_PACKET_ASSEMBLY_ERR_CNTR: u32,
    pub MAC_MMC_RX_PACKET_SMD_ERR_CNTR: u32,
    pub MAC_MMC_RX_PACKET_ASSEMBLY_OK_CNTR: u32,
    pub MAC_MMC_RX_FPE_FRAGMENT_CNTR: u32,
    pub MAC_L3_L4_CONTROL0: u32,
    pub MAC_LAYER4_ADDRESS0: u32,
    pub MAC_LAYER3_ADDR0_REG0: u32,
    pub MAC_LAYER3_ADDR1_REG0: u32,
    pub MAC_LAYER3_ADDR2_REG0: u32,
    pub MAC_LAYER3_ADDR3_REG0: u32,
    pub MAC_L3_L4_CONTROL1: u32,
    pub MAC_LAYER4_ADDRESS1: u32,
    pub MAC_LAYER3_ADDR0_REG1: u32,
    pub MAC_LAYER3_ADDR1_REG1: u32,
    pub MAC_LAYER3_ADDR2_REG1: u32,
    pub MAC_LAYER3_ADDR3_REG1: u32,
    pub MAC_L3_L4_CONTROL2: u32,
    pub MAC_LAYER4_ADDRESS2: u32,
    pub MAC_LAYER3_ADDR0_REG2: u32,
    pub MAC_LAYER3_ADDR1_REG2: u32,
    pub MAC_LAYER3_ADDR2_REG2: u32,
    pub MAC_LAYER3_ADDR3_REG2: u32,
    pub MAC_L3_L4_CONTROL3: u32,
    pub MAC_LAYER4_ADDRESS3: u32,
    pub MAC_LAYER3_ADDR0_REG3: u32,
    pub MAC_LAYER3_ADDR1_REG3: u32,
    pub MAC_LAYER3_ADDR2_REG3: u32,
    pub MAC_LAYER3_ADDR3_REG3: u32,
    pub MAC_L3_L4_CONTROL4: u32,
    pub MAC_LAYER4_ADDRESS4: u32,
    pub MAC_LAYER3_ADDR0_REG4: u32,
    pub MAC_LAYER3_ADDR1_REG4: u32,
    pub MAC_LAYER3_ADDR2_REG4: u32,
    pub MAC_LAYER3_ADDR3_REG4: u32,
    pub MAC_L3_L4_CONTROL5: u32,
    pub MAC_LAYER4_ADDRESS5: u32,
    pub MAC_LAYER3_ADDR0_REG5: u32,
    pub MAC_LAYER3_ADDR1_REG5: u32,
    pub MAC_LAYER3_ADDR2_REG5: u32,
    pub MAC_LAYER3_ADDR3_REG5: u32,
    pub MAC_L3_L4_CONTROL6: u32,
    pub MAC_LAYER4_ADDRESS6: u32,
    pub MAC_LAYER3_ADDR0_REG6: u32,
    pub MAC_LAYER3_ADDR1_REG6: u32,
    pub MAC_LAYER3_ADDR2_REG6: u32,
    pub MAC_LAYER3_ADDR3_REG6: u32,
    pub MAC_L3_L4_CONTROL7: u32,
    pub MAC_LAYER4_ADDRESS7: u32,
    pub MAC_LAYER3_ADDR0_REG7: u32,
    pub MAC_LAYER3_ADDR1_REG7: u32,
    pub MAC_LAYER3_ADDR2_REG7: u32,
    pub MAC_LAYER3_ADDR3_REG7: u32,
    pub MAC_TIMESTAMP_CONTROL: u32,
    pub MAC_SUB_SECOND_INCREMENT: u32,
    pub MAC_SYSTEM_TIME_SECONDS: u32,
    pub MAC_SYSTEM_TIME_NANOSECONDS: u32,
    pub MAC_SYSTEM_TIME_SECONDS_UPDATE: u32,
    pub MAC_SYSTEM_TIME_NANOSECONDS_UPDATE: u32,
    pub MAC_TIMESTAMP_ADDEND: u32,
    pub MAC_SYSTEM_TIME_HIGHER_WORD_SECONDS: u32,
    pub MAC_TIMESTAMP_STATUS: u32,
    pub MAC_TX_TIMESTAMP_STATUS_NANOSECONDS: u32,
    pub MAC_TX_TIMESTAMP_STATUS_SECONDS: u32,
    pub MAC_AUXILIARY_CONTROL: u32,
    pub MAC_AUXILIARY_TIMESTAMP_NANOSECONDS: u32,
    pub MAC_AUXILIARY_TIMESTAMP_SECONDS: u32,
    pub MAC_TIMESTAMP_INGRESS_ASYM_CORR: u32,
    pub MAC_TIMESTAMP_EGRESS_ASYM_CORR: u32,
    pub MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND: u32,
    pub MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND: u32,
    pub MAC_TIMESTAMP_INGRESS_CORR_SUBNANOSEC: u32,
    pub MAC_TIMESTAMP_EGRESS_CORR_SUBNANOSEC: u32,
    pub MAC_TIMESTAMP_INGRESS_LATENCY: u32,
    pub MAC_TIMESTAMP_EGRESS_LATENCY: u32,
    pub MAC_PPS_CONTROL: u32,
    pub MAC_PPS0_TARGET_TIME_SECONDS: u32,
    pub MAC_PPS0_TARGET_TIME_NANOSECONDS: u32,
    pub MAC_PPS0_INTERVAL: u32,
    pub MAC_PPS0_WIDTH: u32,
    pub MAC_PPS1_TARGET_TIME_SECONDS: u32,
    pub MAC_PPS1_TARGET_TIME_NANOSECONDS: u32,
    pub MAC_PPS1_INTERVAL: u32,
    pub MAC_PPS1_WIDTH: u32,
    pub MAC_PPS2_TARGET_TIME_SECONDS: u32,
    pub MAC_PPS2_TARGET_TIME_NANOSECONDS: u32,
    pub MAC_PPS2_INTERVAL: u32,
    pub MAC_PPS2_WIDTH: u32,
    pub MAC_PPS3_TARGET_TIME_SECONDS: u32,
    pub MAC_PPS3_TARGET_TIME_NANOSECONDS: u32,
    pub MAC_PPS3_INTERVAL: u32,
    pub MAC_PPS3_WIDTH: u32,
    pub MAC_PTO_CONTROL: u32,
    pub MAC_SOURCE_PORT_IDENTITY0: u32,
    pub MAC_SOURCE_PORT_IDENTITY1: u32,
    pub MAC_SOURCE_PORT_IDENTITY2: u32,
    pub MAC_LOG_MESSAGE_INTERVAL: u32,
    pub MTL_OPERATION_MODE: u32,
    pub MTL_DBG_CTL: u32,
    pub MTL_DBG_STS: u32,
    pub MTL_FIFO_DEBUG_DATA: u32,
    pub MTL_INTERRUPT_STATUS: u32,
    pub MTL_RXQ_DMA_MAP0: u32,
    pub MTL_RXQ_DMA_MAP1: u32,
    pub MTL_TBS_CTRL: u32,
    pub MTL_EST_CONTROL: u32,
    pub MTL_EST_STATUS: u32,
    pub MTL_EST_SCH_ERROR: u32,
    pub MTL_EST_FRM_SIZE_ERROR: u32,
    pub MTL_EST_FRM_SIZE_CAPTURE: u32,
    pub MTL_EST_INTR_ENABLE: u32,
    pub MTL_EST_GCL_CONTROL: u32,
    pub MTL_EST_GCL_DATA: u32,
    pub MTL_FPE_CTRL_STS: u32,
    pub MTL_FPE_ADVANCE: u32,
    pub MTL_RXP_CONTROL_STATUS: u32,
    pub MTL_RXP_INTERRUPT_CONTROL_STATUS: u32,
    pub MTL_RXP_DROP_CNT: u32,
    pub MTL_RXP_ERROR_CNT: u32,
    pub MTL_RXP_INDIRECT_ACC_CONTROL_STATUS: u32,
    pub MTL_RXP_INDIRECT_ACC_DATA: u32,
    pub MTL_TXQ0_OPERATION_MODE: u32,
    pub MTL_TXQ0_UNDERFLOW: u32,
    pub MTL_TXQ0_DEBUG: u32,
    pub MTL_TXQ0_ETS_STATUS: u32,
    pub MTL_TXQ0_QUANTUM_WEIGHT: u32,
    pub MTL_Q0_INTERRUPT_CONTROL_STATUS: u32,
    pub MTL_RXQ0_OPERATION_MODE: u32,
    pub MTL_RXQ0_MISSED_PACKET_OVERFLOW_CNT: u32,
    pub MTL_RXQ0_DEBUG: u32,
    pub MTL_RXQ0_CONTROL: u32,
    pub MTL_TXQ1_OPERATION_MODE: u32,
    pub MTL_TXQ1_UNDERFLOW: u32,
    pub MTL_TXQ1_DEBUG: u32,
    pub MTL_TXQ1_ETS_CONTROL: u32,
    pub MTL_TXQ1_ETS_STATUS: u32,
    pub MTL_TXQ1_QUANTUM_WEIGHT: u32,
    pub MTL_TXQ1_SENDSLOPECREDIT: u32,
    pub MTL_TXQ1_HICREDIT: u32,
    pub MTL_TXQ1_LOCREDIT: u32,
    pub MTL_Q1_INTERRUPT_CONTROL_STATUS: u32,
    pub MTL_RXQ1_OPERATION_MODE: u32,
    pub MTL_RXQ1_MISSED_PACKET_OVERFLOW_CNT: u32,
    pub MTL_RXQ1_DEBUG: u32,
    pub MTL_RXQ1_CONTROL: u32,
    pub MTL_TXQ2_OPERATION_MODE: u32,
    pub MTL_TXQ2_UNDERFLOW: u32,
    pub MTL_TXQ2_DEBUG: u32,
    pub MTL_TXQ2_ETS_CONTROL: u32,
    pub MTL_TXQ2_ETS_STATUS: u32,
    pub MTL_TXQ2_QUANTUM_WEIGHT: u32,
    pub MTL_TXQ2_SENDSLOPECREDIT: u32,
    pub MTL_TXQ2_HICREDIT: u32,
    pub MTL_TXQ2_LOCREDIT: u32,
    pub MTL_Q2_INTERRUPT_CONTROL_STATUS: u32,
    pub MTL_RXQ2_OPERATION_MODE: u32,
    pub MTL_RXQ2_MISSED_PACKET_OVERFLOW_CNT: u32,
    pub MTL_RXQ2_DEBUG: u32,
    pub MTL_RXQ2_CONTROL: u32,
    pub MTL_TXQ3_OPERATION_MODE: u32,
    pub MTL_TXQ3_UNDERFLOW: u32,
    pub MTL_TXQ3_DEBUG: u32,
    pub MTL_TXQ3_ETS_CONTROL: u32,
    pub MTL_TXQ3_ETS_STATUS: u32,
    pub MTL_TXQ3_QUANTUM_WEIGHT: u32,
    pub MTL_TXQ3_SENDSLOPECREDIT: u32,
    pub MTL_TXQ3_HICREDIT: u32,
    pub MTL_TXQ3_LOCREDIT: u32,
    pub MTL_Q3_INTERRUPT_CONTROL_STATUS: u32,
    pub MTL_RXQ3_OPERATION_MODE: u32,
    pub MTL_RXQ3_MISSED_PACKET_OVERFLOW_CNT: u32,
    pub MTL_RXQ3_DEBUG: u32,
    pub MTL_RXQ3_CONTROL: u32,
    pub MTL_TXQ4_OPERATION_MODE: u32,
    pub MTL_TXQ4_UNDERFLOW: u32,
    pub MTL_TXQ4_DEBUG: u32,
    pub MTL_TXQ4_ETS_CONTROL: u32,
    pub MTL_TXQ4_ETS_STATUS: u32,
    pub MTL_TXQ4_QUANTUM_WEIGHT: u32,
    pub MTL_TXQ4_SENDSLOPECREDIT: u32,
    pub MTL_TXQ4_HICREDIT: u32,
    pub MTL_TXQ4_LOCREDIT: u32,
    pub MTL_Q4_INTERRUPT_CONTROL_STATUS: u32,
    pub MTL_RXQ4_OPERATION_MODE: u32,
    pub MTL_RXQ4_MISSED_PACKET_OVERFLOW_CNT: u32,
    pub MTL_RXQ4_DEBUG: u32,
    pub MTL_RXQ4_CONTROL: u32,
    pub DMA_MODE: u32,
    pub DMA_SYSBUS_MODE: u32,
    pub DMA_INTERRUPT_STATUS: u32,
    pub DMA_DEBUG_STATUS0: u32,
    pub DMA_DEBUG_STATUS1: u32,
    pub DMA_AXI_LPI_ENTRY_INTERVAL: u32,
    pub DMA_TBS_CTRL: u32,
    pub DMA_CH0_CONTROL: u32,
    pub DMA_CH0_TX_CONTROL: u32,
    pub DMA_CH0_RX_CONTROL: u32,
    pub DMA_CH0_TXDESC_LIST_ADDRESS: u32,
    pub DMA_CH0_RXDESC_LIST_ADDRESS: u32,
    pub DMA_CH0_TXDESC_TAIL_POINTER: u32,
    pub DMA_CH0_RXDESC_TAIL_POINTER: u32,
    pub DMA_CH0_TXDESC_RING_LENGTH: u32,
    pub DMA_CH0_RXDESC_RING_LENGTH: u32,
    pub DMA_CH0_INTERRUPT_ENABLE: u32,
    pub DMA_CH0_RX_INTERRUPT_WATCHDOG_TIMER: u32,
    pub DMA_CH0_SLOT_FUNCTION_CONTROL_STATUS: u32,
    pub DMA_CH0_CURRENT_APP_TXDESC: u32,
    pub DMA_CH0_CURRENT_APP_RXDESC: u32,
    pub DMA_CH0_CURRENT_APP_TXBUFFER: u32,
    pub DMA_CH0_CURRENT_APP_RXBUFFER: u32,
    pub DMA_CH0_STATUS: u32,
    pub DMA_CH0_MISS_FRAME_CNT: u32,
    pub DMA_CH0_RXP_ACCEPT_CNT: u32,
    pub DMA_CH0_RX_ERI_CNT: u32,
    pub DMA_CH1_CONTROL: u32,
    pub DMA_CH1_TX_CONTROL: u32,
    pub DMA_CH1_RX_CONTROL: u32,
    pub DMA_CH1_TXDESC_LIST_ADDRESS: u32,
    pub DMA_CH1_RXDESC_LIST_ADDRESS: u32,
    pub DMA_CH1_TXDESC_TAIL_POINTER: u32,
    pub DMA_CH1_RXDESC_TAIL_POINTER: u32,
    pub DMA_CH1_TXDESC_RING_LENGTH: u32,
    pub DMA_CH1_RXDESC_RING_LENGTH: u32,
    pub DMA_CH1_INTERRUPT_ENABLE: u32,
    pub DMA_CH1_RX_INTERRUPT_WATCHDOG_TIMER: u32,
    pub DMA_CH1_SLOT_FUNCTION_CONTROL_STATUS: u32,
    pub DMA_CH1_CURRENT_APP_TXDESC: u32,
    pub DMA_CH1_CURRENT_APP_RXDESC: u32,
    pub DMA_CH1_CURRENT_APP_TXBUFFER: u32,
    pub DMA_CH1_CURRENT_APP_RXBUFFER: u32,
    pub DMA_CH1_STATUS: u32,
    pub DMA_CH1_MISS_FRAME_CNT: u32,
    pub DMA_CH1_RXP_ACCEPT_CNT: u32,
    pub DMA_CH1_RX_ERI_CNT: u32,
    pub DMA_CH2_CONTROL: u32,
    pub DMA_CH2_TX_CONTROL: u32,
    pub DMA_CH2_RX_CONTROL: u32,
    pub DMA_CH2_TXDESC_LIST_ADDRESS: u32,
    pub DMA_CH2_RXDESC_LIST_ADDRESS: u32,
    pub DMA_CH2_TXDESC_TAIL_POINTER: u32,
    pub DMA_CH2_RXDESC_TAIL_POINTER: u32,
    pub DMA_CH2_TXDESC_RING_LENGTH: u32,
    pub DMA_CH2_RXDESC_RING_LENGTH: u32,
    pub DMA_CH2_INTERRUPT_ENABLE: u32,
    pub DMA_CH2_RX_INTERRUPT_WATCHDOG_TIMER: u32,
    pub DMA_CH2_SLOT_FUNCTION_CONTROL_STATUS: u32,
    pub DMA_CH2_CURRENT_APP_TXDESC: u32,
    pub DMA_CH2_CURRENT_APP_RXDESC: u32,
    pub DMA_CH2_CURRENT_APP_TXBUFFER: u32,
    pub DMA_CH2_CURRENT_APP_RXBUFFER: u32,
    pub DMA_CH2_STATUS: u32,
    pub DMA_CH2_MISS_FRAME_CNT: u32,
    pub DMA_CH2_RXP_ACCEPT_CNT: u32,
    pub DMA_CH2_RX_ERI_CNT: u32,
    pub DMA_CH3_CONTROL: u32,
    pub DMA_CH3_TX_CONTROL: u32,
    pub DMA_CH3_RX_CONTROL: u32,
    pub DMA_CH3_TXDESC_LIST_ADDRESS: u32,
    pub DMA_CH3_RXDESC_LIST_ADDRESS: u32,
    pub DMA_CH3_TXDESC_TAIL_POINTER: u32,
    pub DMA_CH3_RXDESC_TAIL_POINTER: u32,
    pub DMA_CH3_TXDESC_RING_LENGTH: u32,
    pub DMA_CH3_RXDESC_RING_LENGTH: u32,
    pub DMA_CH3_INTERRUPT_ENABLE: u32,
    pub DMA_CH3_RX_INTERRUPT_WATCHDOG_TIMER: u32,
    pub DMA_CH3_SLOT_FUNCTION_CONTROL_STATUS: u32,
    pub DMA_CH3_CURRENT_APP_TXDESC: u32,
    pub DMA_CH3_CURRENT_APP_RXDESC: u32,
    pub DMA_CH3_CURRENT_APP_TXBUFFER: u32,
    pub DMA_CH3_CURRENT_APP_RXBUFFER: u32,
    pub DMA_CH3_STATUS: u32,
    pub DMA_CH3_MISS_FRAME_CNT: u32,
    pub DMA_CH3_RXP_ACCEPT_CNT: u32,
    pub DMA_CH3_RX_ERI_CNT: u32,
    pub DMA_CH4_CONTROL: u32,
    pub DMA_CH4_TX_CONTROL: u32,
    pub DMA_CH4_RX_CONTROL: u32,
    pub DMA_CH4_TXDESC_LIST_ADDRESS: u32,
    pub DMA_CH4_RXDESC_LIST_ADDRESS: u32,
    pub DMA_CH4_TXDESC_TAIL_POINTER: u32,
    pub DMA_CH4_RXDESC_TAIL_POINTER: u32,
    pub DMA_CH4_TXDESC_RING_LENGTH: u32,
    pub DMA_CH4_RXDESC_RING_LENGTH: u32,
    pub DMA_CH4_INTERRUPT_ENABLE: u32,
    pub DMA_CH4_RX_INTERRUPT_WATCHDOG_TIMER: u32,
    pub DMA_CH4_SLOT_FUNCTION_CONTROL_STATUS: u32,
    pub DMA_CH4_CURRENT_APP_TXDESC: u32,
    pub DMA_CH4_CURRENT_APP_RXDESC: u32,
    pub DMA_CH4_CURRENT_APP_TXBUFFER: u32,
    pub DMA_CH4_CURRENT_APP_RXBUFFER: u32,
    pub DMA_CH4_STATUS: u32,
    pub DMA_CH4_MISS_FRAME_CNT: u32,
    pub DMA_CH4_RXP_ACCEPT_CNT: u32,
    pub DMA_CH4_RX_ERI_CNT: u32,
}
pub struct Instance<const N: u8> {
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) addr: u32,
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) intrs: &'static [crate::Interrupt],
}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> ::core::ops::Deref for Instance<N> {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

#[cfg(not(feature = "nosync"))]
unsafe impl<const N: u8> Send for Instance<N> {}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> Instance<N> {
    /// Return the interrupt signals associated with this
    /// peripheral instance
    ///
    /// Collection may be empty if there is no interrupt signal
    /// associated with the peripheral. There's no guarantee for
    /// interrupt signal ordering in the collection.
    #[inline(always)]
    pub const fn interrupts<'a>(&'a self) -> &'a [crate::Interrupt] {
        self.intrs
    }
}
