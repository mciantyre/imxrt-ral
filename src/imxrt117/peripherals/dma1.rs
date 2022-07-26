#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA
//!
//! Used by: imxrt1176_cm4, imxrt1176_cm7

use crate::{RORegister, RWRegister};

/// Control
pub mod CR {

    /// Enable Debug
    pub mod EDBG {
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

            /// 0b0: When the chip is in Debug mode, the eDMA continues to operate.
            pub const DISABLED: u32 = 0b0;

            /// 0b1: When the chip is in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete.
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Enable Round Robin Channel Arbitration
    pub mod ERCA {
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

            /// 0b0: Fixed priority arbitration within each group
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Round robin arbitration within each group
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Enable Round Robin Group Arbitration
    pub mod ERGA {
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

            /// 0b0: Fixed priority arbitration
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Round robin arbitration
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Halt On Error
    pub mod HOE {
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

            /// 0b0: Normal operation
            pub const NORMAL_OPS: u32 = 0b0;

            /// 0b1: Error causes HALT field to be automatically set to 1
            pub const HALT_ON_ERROR: u32 = 0b1;
        }
    }

    /// Halt eDMA Operations
    pub mod HALT {
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

            /// 0b0: Normal operation
            pub const NORMAL_OPS: u32 = 0b0;

            /// 0b1: eDMA operations halted
            pub const HALT_DMA: u32 = 0b1;
        }
    }

    /// Continuous Link Mode
    pub mod CLM {
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

            /// 0b0: Continuous link mode is off
            pub const CLM_OFF: u32 = 0b0;

            /// 0b1: Continuous link mode is on
            pub const CLM_ON: u32 = 0b1;
        }
    }

    /// Enable Minor Loop Mapping
    pub mod EMLM {
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

            /// 0b0: Disabled
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Enabled
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Channel Group 0 Priority
    pub mod GRP0PRI {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel Group 1 Priority
    pub mod GRP1PRI {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Error Cancel Transfer
    pub mod ECX {
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

            /// 0b0: Normal operation
            pub const NORMAL_OPS: u32 = 0b0;

            /// 0b1: Cancel the remaining data transfer
            pub const CANCEL: u32 = 0b1;
        }
    }

    /// Cancel Transfer
    pub mod CX {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ECX::RW;
    }

    /// eDMA version number
    pub mod VERSION {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// eDMA Active Status
    pub mod ACTIVE {
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

            /// 0b0: eDMA is idle
            pub const IDLE: u32 = 0b0;

            /// 0b1: eDMA is executing a channel
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// Error Status
pub mod ES {

    /// Destination Bus Error
    pub mod DBE {
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

            /// 0b0: No destination bus error.
            pub const NO_ERROR: u32 = 0b0;

            /// 0b1: The most-recently recorded error was a bus error on a destination write.
            pub const ERROR: u32 = 0b1;
        }
    }

    /// Source Bus Error
    pub mod SBE {
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

            /// 0b0: No source bus error.
            pub const NO_ERROR: u32 = 0b0;

            /// 0b1: The most-recently recorded error was a bus error on a source read.
            pub const ERROR: u32 = 0b1;
        }
    }

    /// Scatter/Gather Configuration Error
    pub mod SGE {
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

            /// 0b0: No scatter/gather configuration error.
            pub const NO_ERROR: u32 = 0b0;

            /// 0b1: The most-recently recorded error was a configuration error detected in the TCDn_DLASTSGA field.
            pub const ERROR: u32 = 0b1;
        }
    }

    /// NBYTES/CITER Configuration Error
    pub mod NCE {
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

            /// 0b0: No NBYTES/CITER configuration error.
            pub const NO_ERROR: u32 = 0b0;

            /// 0b1: The most-recently recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields. TCDn_NBYTES is not a multiple of TCDn_ATTR\[SSIZE\] and TCDn_ATTR\[DSIZE\], or TCDn_CITER\[CITER\] = 0, or TCDn_CITER\[ELINK\] is not equal to TCDn_BITER\[ELINK\].
            pub const ERROR: u32 = 0b1;
        }
    }

    /// Destination Offset Error
    pub mod DOE {
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

            /// 0b0: No destination offset configuration error.
            pub const NO_ERROR: u32 = 0b0;

            /// 0b1: The most-recently recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR\[DSIZE\].
            pub const ERROR: u32 = 0b1;
        }
    }

    /// Destination Address Error
    pub mod DAE {
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

            /// 0b0: No destination address configuration error.
            pub const NO_ERROR: u32 = 0b0;

            /// 0b1: The most-recently recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR\[DSIZE\].
            pub const ERROR: u32 = 0b1;
        }
    }

    /// Source Offset Error
    pub mod SOE {
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

            /// 0b0: No source offset configuration error.
            pub const NO_ERROR: u32 = 0b0;

            /// 0b1: The most-recently recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR\[SSIZE\].
            pub const ERROR: u32 = 0b1;
        }
    }

    /// Source Address Error
    pub mod SAE {
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

            /// 0b0: No source address configuration error.
            pub const NO_ERROR: u32 = 0b0;

            /// 0b1: The most-recently recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR\[SSIZE\].
            pub const ERROR: u32 = 0b1;
        }
    }

    /// Error Channel Number or Canceled Channel Number
    pub mod ERRCHN {
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

    /// Channel Priority Error
    pub mod CPE {
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

            /// 0b0: No channel priority error.
            pub const NO_ERROR: u32 = 0b0;

            /// 0b1: The most-recently recorded error was a configuration error in the channel priorities within a group. Channel priorities within a group are not unique.
            pub const ERROR: u32 = 0b1;
        }
    }

    /// Group Priority Error
    pub mod GPE {
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

            /// 0b0: No group priority error.
            pub const NO_ERROR: u32 = 0b0;

            /// 0b1: The most-recently recorded error was a configuration error among the group priorities. All group priorities are not unique.
            pub const ERROR: u32 = 0b1;
        }
    }

    /// Transfer Canceled
    pub mod ECX {
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

            /// 0b0: No canceled transfers
            pub const NO_CANCELS: u32 = 0b0;

            /// 0b1: The most-recently recorded entry was a canceled transfer initiated by the error cancel transfer field
            pub const CANCELED: u32 = 0b1;
        }
    }

    /// Logical OR of all ERR status fields
    pub mod VLD {
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

            /// 0b0: No ERR fields are 1
            pub const NO_ERROR: u32 = 0b0;

            /// 0b1: At least one ERR field has a value of 1, indicating a valid error exists that has not been cleared
            pub const ERROR: u32 = 0b1;
        }
    }
}

/// Enable Request
pub mod ERQ {

    /// Enable DMA Request 0
    pub mod ERQ0 {
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

            /// 0b0: The DMA request signal for channel 0 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 0 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 1
    pub mod ERQ1 {
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

            /// 0b0: The DMA request signal for channel 1 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 1 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 2
    pub mod ERQ2 {
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

            /// 0b0: The DMA request signal for channel 2 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 2 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 3
    pub mod ERQ3 {
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

            /// 0b0: The DMA request signal for channel 3 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 3 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 4
    pub mod ERQ4 {
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

            /// 0b0: The DMA request signal for channel 4 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 4 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 5
    pub mod ERQ5 {
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

            /// 0b0: The DMA request signal for channel 5 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 5 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 6
    pub mod ERQ6 {
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

            /// 0b0: The DMA request signal for channel 6 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 6 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 7
    pub mod ERQ7 {
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

            /// 0b0: The DMA request signal for channel 7 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 7 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 8
    pub mod ERQ8 {
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

            /// 0b0: The DMA request signal for channel 8 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 8 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 9
    pub mod ERQ9 {
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

            /// 0b0: The DMA request signal for channel 9 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 9 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 10
    pub mod ERQ10 {
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

            /// 0b0: The DMA request signal for channel 10 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 10 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 11
    pub mod ERQ11 {
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

            /// 0b0: The DMA request signal for channel 11 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 11 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 12
    pub mod ERQ12 {
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

            /// 0b0: The DMA request signal for channel 12 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 12 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 13
    pub mod ERQ13 {
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

            /// 0b0: The DMA request signal for channel 13 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 13 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 14
    pub mod ERQ14 {
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

            /// 0b0: The DMA request signal for channel 14 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 14 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 15
    pub mod ERQ15 {
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

            /// 0b0: The DMA request signal for channel 15 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 15 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 16
    pub mod ERQ16 {
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

            /// 0b0: The DMA request signal for channel 16 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 16 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 17
    pub mod ERQ17 {
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

            /// 0b0: The DMA request signal for channel 17 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 17 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 18
    pub mod ERQ18 {
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

            /// 0b0: The DMA request signal for channel 18 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 18 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 19
    pub mod ERQ19 {
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

            /// 0b0: The DMA request signal for channel 19 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 19 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 20
    pub mod ERQ20 {
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

            /// 0b0: The DMA request signal for channel 20 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 20 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 21
    pub mod ERQ21 {
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

            /// 0b0: The DMA request signal for channel 21 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 21 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 22
    pub mod ERQ22 {
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

            /// 0b0: The DMA request signal for channel 22 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 22 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 23
    pub mod ERQ23 {
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

            /// 0b0: The DMA request signal for channel 23 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 23 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 24
    pub mod ERQ24 {
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

            /// 0b0: The DMA request signal for channel 24 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 24 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 25
    pub mod ERQ25 {
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

            /// 0b0: The DMA request signal for channel 25 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 25 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 26
    pub mod ERQ26 {
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

            /// 0b0: The DMA request signal for channel 26 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 26 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 27
    pub mod ERQ27 {
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

            /// 0b0: The DMA request signal for channel 27 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 27 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 28
    pub mod ERQ28 {
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

            /// 0b0: The DMA request signal for channel 28 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 28 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 29
    pub mod ERQ29 {
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

            /// 0b0: The DMA request signal for channel 29 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 29 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 30
    pub mod ERQ30 {
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

            /// 0b0: The DMA request signal for channel 30 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 30 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable DMA Request 31
    pub mod ERQ31 {
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

            /// 0b0: The DMA request signal for channel 31 is disabled
            pub const DISABLE: u32 = 0b0;

            /// 0b1: The DMA request signal for channel 31 is enabled
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Enable Error Interrupt
pub mod EEI {

    /// Enable Error Interrupt 0
    pub mod EEI0 {
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

            /// 0b0: An error on channel 0 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 0 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 1
    pub mod EEI1 {
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

            /// 0b0: An error on channel 1 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 1 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 2
    pub mod EEI2 {
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

            /// 0b0: An error on channel 2 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 2 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 3
    pub mod EEI3 {
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

            /// 0b0: An error on channel 3 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 3 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 4
    pub mod EEI4 {
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

            /// 0b0: An error on channel 4 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 4 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 5
    pub mod EEI5 {
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

            /// 0b0: An error on channel 5 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 5 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 6
    pub mod EEI6 {
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

            /// 0b0: An error on channel 6 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 6 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 7
    pub mod EEI7 {
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

            /// 0b0: An error on channel 7 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 7 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 8
    pub mod EEI8 {
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

            /// 0b0: An error on channel 8 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 8 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 9
    pub mod EEI9 {
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

            /// 0b0: An error on channel 9 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 9 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 10
    pub mod EEI10 {
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

            /// 0b0: An error on channel 10 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 10 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 11
    pub mod EEI11 {
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

            /// 0b0: An error on channel 11 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 11 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 12
    pub mod EEI12 {
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

            /// 0b0: An error on channel 12 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 12 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 13
    pub mod EEI13 {
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

            /// 0b0: An error on channel 13 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 13 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 14
    pub mod EEI14 {
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

            /// 0b0: An error on channel 14 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 14 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 15
    pub mod EEI15 {
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

            /// 0b0: An error on channel 15 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 15 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 16
    pub mod EEI16 {
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

            /// 0b0: An error on channel 16 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 16 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 17
    pub mod EEI17 {
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

            /// 0b0: An error on channel 17 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 17 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 18
    pub mod EEI18 {
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

            /// 0b0: An error on channel 18 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 18 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 19
    pub mod EEI19 {
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

            /// 0b0: An error on channel 19 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 19 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 20
    pub mod EEI20 {
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

            /// 0b0: An error on channel 20 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 20 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 21
    pub mod EEI21 {
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

            /// 0b0: An error on channel 21 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 21 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 22
    pub mod EEI22 {
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

            /// 0b0: An error on channel 22 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 22 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 23
    pub mod EEI23 {
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

            /// 0b0: An error on channel 23 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 23 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 24
    pub mod EEI24 {
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

            /// 0b0: An error on channel 24 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 24 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 25
    pub mod EEI25 {
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

            /// 0b0: An error on channel 25 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 25 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 26
    pub mod EEI26 {
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

            /// 0b0: An error on channel 26 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 26 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 27
    pub mod EEI27 {
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

            /// 0b0: An error on channel 27 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 27 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 28
    pub mod EEI28 {
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

            /// 0b0: An error on channel 28 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 28 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 29
    pub mod EEI29 {
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

            /// 0b0: An error on channel 29 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 29 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 30
    pub mod EEI30 {
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

            /// 0b0: An error on channel 30 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 30 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 31
    pub mod EEI31 {
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

            /// 0b0: An error on channel 31 does not generate an error interrupt
            pub const NO_INTERRUPT: u32 = 0b0;

            /// 0b1: An error on channel 31 generates an error interrupt request
            pub const INTERRUPT: u32 = 0b1;
        }
    }
}

/// Clear Enable Error Interrupt
pub mod CEEI {

    /// Clear Enable Error Interrupt
    pub mod CEEI {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u8 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear All Enable Error Interrupts
    pub mod CAEE {
        /// Offset (6 bits)
        pub const offset: u8 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write 0 only to the EEI field specified in the CEEI field
            pub const CLEAR_EEI: u8 = 0b0;

            /// 0b1: Write 0 to all fields in EEI
            pub const CLEAR_ALL: u8 = 0b1;
        }
    }

    /// No Op Enable
    pub mod NOP {
        /// Offset (7 bits)
        pub const offset: u8 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const NORMAL_OPS: u8 = 0b0;

            /// 0b1: No operation, ignore the other fields in this register
            pub const NO_OPS: u8 = 0b1;
        }
    }
}

/// Set Enable Error Interrupt
pub mod SEEI {

    /// Set Enable Error Interrupt
    pub mod SEEI {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u8 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set All Enable Error Interrupts
    pub mod SAEE {
        /// Offset (6 bits)
        pub const offset: u8 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write 1 only to the EEI field specified in the SEEI field
            pub const SET_EEI: u8 = 0b0;

            /// 0b1: Writes 1 to all fields in EEI
            pub const SET_ALL: u8 = 0b1;
        }
    }

    /// No Op Enable
    pub mod NOP {
        /// Offset (7 bits)
        pub const offset: u8 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const NORMAL_OPS: u8 = 0b0;

            /// 0b1: No operation, ignore the other fields in this register
            pub const NO_OPS: u8 = 0b1;
        }
    }
}

/// Clear Enable Request
pub mod CERQ {

    /// Clear Enable Request
    pub mod CERQ {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u8 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear All Enable Requests
    pub mod CAER {
        /// Offset (6 bits)
        pub const offset: u8 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write 0 to only the ERQ field specified in the CERQ field
            pub const CLEAR_ERQ: u8 = 0b0;

            /// 0b1: Write 0 to all fields in ERQ
            pub const CLEAR_ALL: u8 = 0b1;
        }
    }

    /// No Op Enable
    pub mod NOP {
        /// Offset (7 bits)
        pub const offset: u8 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const NORMAL_OPS: u8 = 0b0;

            /// 0b1: No operation, ignore the other fields in this register
            pub const NO_OPS: u8 = 0b1;
        }
    }
}

/// Set Enable Request
pub mod SERQ {

    /// Set Enable Request
    pub mod SERQ {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u8 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set All Enable Requests
    pub mod SAER {
        /// Offset (6 bits)
        pub const offset: u8 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write 1 to only the ERQ field specified in the SERQ field
            pub const SET_ERQ: u8 = 0b0;

            /// 0b1: Write 1 to all fields in ERQ
            pub const SET_ALL: u8 = 0b1;
        }
    }

    /// No Op Enable
    pub mod NOP {
        /// Offset (7 bits)
        pub const offset: u8 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const NORMAL_OPS: u8 = 0b0;

            /// 0b1: No operation, ignore the other fields in this register
            pub const NO_OPS: u8 = 0b1;
        }
    }
}

/// Clear DONE Status Bit
pub mod CDNE {

    /// Clear DONE field
    pub mod CDNE {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u8 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clears All DONE fields
    pub mod CADN {
        /// Offset (6 bits)
        pub const offset: u8 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Writes 0 to only the TCDn_CSR\[DONE\] field specified in the CDNE field
            pub const CLEAR_DONE: u8 = 0b0;

            /// 0b1: Writes 0 to all bits in TCDn_CSR\[DONE\]
            pub const CLEAR_ALL: u8 = 0b1;
        }
    }

    /// No Op Enable
    pub mod NOP {
        /// Offset (7 bits)
        pub const offset: u8 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const NORMAL_OPS: u8 = 0b0;

            /// 0b1: No operation; all other fields in this register are ignored.
            pub const NO_OPS: u8 = 0b1;
        }
    }
}

/// Set START Bit
pub mod SSRT {

    /// Set START field
    pub mod SSRT {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u8 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set All START fields (activates all channels)
    pub mod SAST {
        /// Offset (6 bits)
        pub const offset: u8 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write 1 to only the TCDn_CSR\[START\] field specified in the SSRT field
            pub const SET_START: u8 = 0b0;

            /// 0b1: Write 1 to all bits in TCDn_CSR\[START\]
            pub const SET_ALL: u8 = 0b1;
        }
    }

    /// No Op Enable
    pub mod NOP {
        /// Offset (7 bits)
        pub const offset: u8 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const NORMAL_OPS: u8 = 0b0;

            /// 0b1: No operation; all other fields in this register are ignored.
            pub const NO_OPS: u8 = 0b1;
        }
    }
}

/// Clear Error
pub mod CERR {

    /// Clear Error Indicator
    pub mod CERR {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u8 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear All Error Indicators
    pub mod CAEI {
        /// Offset (6 bits)
        pub const offset: u8 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write 0 to only the ERR field specified in the CERR field
            pub const CLEAR_ERR: u8 = 0b0;

            /// 0b1: Write 0 to all fields in ERR
            pub const CLEAR_ALL: u8 = 0b1;
        }
    }

    /// No Op Enable
    pub mod NOP {
        /// Offset (7 bits)
        pub const offset: u8 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const NORMAL_OPS: u8 = 0b0;

            /// 0b1: No operation; all other fields in this register are ignored.
            pub const NO_OPS: u8 = 0b1;
        }
    }
}

/// Clear Interrupt Request
pub mod CINT {

    /// Clear Interrupt Request
    pub mod CINT {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u8 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear All Interrupt Requests
    pub mod CAIR {
        /// Offset (6 bits)
        pub const offset: u8 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clear only the INT field specified in the CINT field
            pub const CLEAR_INT: u8 = 0b0;

            /// 0b1: Clear all bits in INT
            pub const CLEAR_ALL: u8 = 0b1;
        }
    }

    /// No Op Enable
    pub mod NOP {
        /// Offset (7 bits)
        pub const offset: u8 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const NORMAL_OPS: u8 = 0b0;

            /// 0b1: No operation; all other fields in this register are ignored.
            pub const NO_OPS: u8 = 0b1;
        }
    }
}

/// Interrupt Request
pub mod INT {

    /// Interrupt Request 0
    pub mod INT0 {
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

            /// 0b0: The interrupt request for channel 0 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 0 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 1
    pub mod INT1 {
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

            /// 0b0: The interrupt request for channel 1 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 1 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 2
    pub mod INT2 {
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

            /// 0b0: The interrupt request for channel 2 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 2 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 3
    pub mod INT3 {
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

            /// 0b0: The interrupt request for channel 3 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 3 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 4
    pub mod INT4 {
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

            /// 0b0: The interrupt request for channel 4 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 4 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 5
    pub mod INT5 {
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

            /// 0b0: The interrupt request for channel 5 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 5 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 6
    pub mod INT6 {
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

            /// 0b0: The interrupt request for channel 6 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 6 is active
            pub const CTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 7
    pub mod INT7 {
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

            /// 0b0: The interrupt request for channel 7 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 7 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 8
    pub mod INT8 {
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

            /// 0b0: The interrupt request for channel 8 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 8 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 9
    pub mod INT9 {
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

            /// 0b0: The interrupt request for channel 9 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 9 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 10
    pub mod INT10 {
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

            /// 0b0: The interrupt request for channel 10 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 10 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 11
    pub mod INT11 {
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

            /// 0b0: The interrupt request for channel 11 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 11 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 12
    pub mod INT12 {
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

            /// 0b0: The interrupt request for channel 12 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 12 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 13
    pub mod INT13 {
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

            /// 0b0: The interrupt request for channel 13 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 13 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 14
    pub mod INT14 {
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

            /// 0b0: The interrupt request for channel 14 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 14 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 15
    pub mod INT15 {
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

            /// 0b0: The interrupt request for channel 15 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 15 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 16
    pub mod INT16 {
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

            /// 0b0: The interrupt request for channel 16 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 16 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 17
    pub mod INT17 {
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

            /// 0b0: The interrupt request for channel 17 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 17 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 18
    pub mod INT18 {
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

            /// 0b0: The interrupt request for channel 18 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 18 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 19
    pub mod INT19 {
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

            /// 0b0: The interrupt request for channel 19 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 19 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 20
    pub mod INT20 {
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

            /// 0b0: The interrupt request for channel 20 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 20 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 21
    pub mod INT21 {
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

            /// 0b0: The interrupt request for channel 21 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 21 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 22
    pub mod INT22 {
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

            /// 0b0: The interrupt request for channel 22 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 22 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 23
    pub mod INT23 {
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

            /// 0b0: The interrupt request for channel 23 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 23 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 24
    pub mod INT24 {
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

            /// 0b0: The interrupt request for channel 24 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 24 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 25
    pub mod INT25 {
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

            /// 0b0: The interrupt request for channel 25 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 25 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 26
    pub mod INT26 {
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

            /// 0b0: The interrupt request for channel 26 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 26 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 27
    pub mod INT27 {
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

            /// 0b0: The interrupt request for channel 27 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 27 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 28
    pub mod INT28 {
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

            /// 0b0: The interrupt request for channel 28 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 28 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 29
    pub mod INT29 {
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

            /// 0b0: The interrupt request for channel 29 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 29 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 30
    pub mod INT30 {
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

            /// 0b0: The interrupt request for channel 30 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 30 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }

    /// Interrupt Request 31
    pub mod INT31 {
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

            /// 0b0: The interrupt request for channel 31 is cleared
            pub const NOT_ACTIVE: u32 = 0b0;

            /// 0b1: The interrupt request for channel 31 is active
            pub const ACTIVE: u32 = 0b1;
        }
    }
}

/// Error
pub mod ERR {

    /// Error In Channel 0
    pub mod ERR0 {
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

            /// 0b0: No error in this channel has occurred
            pub const NO_ERR: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR: u32 = 0b1;
        }
    }

    /// Error In Channel 1
    pub mod ERR1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 2
    pub mod ERR2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 3
    pub mod ERR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 4
    pub mod ERR4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 5
    pub mod ERR5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 6
    pub mod ERR6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 7
    pub mod ERR7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 8
    pub mod ERR8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 9
    pub mod ERR9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 10
    pub mod ERR10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 11
    pub mod ERR11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 12
    pub mod ERR12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 13
    pub mod ERR13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 14
    pub mod ERR14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 15
    pub mod ERR15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 16
    pub mod ERR16 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 17
    pub mod ERR17 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 18
    pub mod ERR18 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 19
    pub mod ERR19 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 20
    pub mod ERR20 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 21
    pub mod ERR21 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 22
    pub mod ERR22 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 23
    pub mod ERR23 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 24
    pub mod ERR24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 25
    pub mod ERR25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 26
    pub mod ERR26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 27
    pub mod ERR27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 28
    pub mod ERR28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 29
    pub mod ERR29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 30
    pub mod ERR30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }

    /// Error In Channel 31
    pub mod ERR31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ERR0::RW;
    }
}

/// Hardware Request Status
pub mod HRS {

    /// Hardware Request Status Channel 0
    pub mod HRS0 {
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

            /// 0b0: A hardware service request for channel 0 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 0 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 1
    pub mod HRS1 {
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

            /// 0b0: A hardware service request for channel 1 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 1 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 2
    pub mod HRS2 {
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

            /// 0b0: A hardware service request for channel 2 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 2 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 3
    pub mod HRS3 {
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

            /// 0b0: A hardware service request for channel 3 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 3 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 4
    pub mod HRS4 {
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

            /// 0b0: A hardware service request for channel 4 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 4 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 5
    pub mod HRS5 {
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

            /// 0b0: A hardware service request for channel 5 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 5 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 6
    pub mod HRS6 {
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

            /// 0b0: A hardware service request for channel 6 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 6 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 7
    pub mod HRS7 {
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

            /// 0b0: A hardware service request for channel 7 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 7 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 8
    pub mod HRS8 {
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

            /// 0b0: A hardware service request for channel 8 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 8 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 9
    pub mod HRS9 {
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

            /// 0b0: A hardware service request for channel 9 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 9 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 10
    pub mod HRS10 {
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

            /// 0b0: A hardware service request for channel 10 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 10 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 11
    pub mod HRS11 {
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

            /// 0b0: A hardware service request for channel 11 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 11 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 12
    pub mod HRS12 {
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

            /// 0b0: A hardware service request for channel 12 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 12 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 13
    pub mod HRS13 {
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

            /// 0b0: A hardware service request for channel 13 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 13 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 14
    pub mod HRS14 {
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

            /// 0b0: A hardware service request for channel 14 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 14 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 15
    pub mod HRS15 {
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

            /// 0b0: A hardware service request for channel 15 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 15 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 16
    pub mod HRS16 {
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

            /// 0b0: A hardware service request for channel 16 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 16 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 17
    pub mod HRS17 {
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

            /// 0b0: A hardware service request for channel 17 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 17 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 18
    pub mod HRS18 {
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

            /// 0b0: A hardware service request for channel 18 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 18 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 19
    pub mod HRS19 {
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

            /// 0b0: A hardware service request for channel 19 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 19 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 20
    pub mod HRS20 {
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

            /// 0b0: A hardware service request for channel 20 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 20 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 21
    pub mod HRS21 {
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

            /// 0b0: A hardware service request for channel 21 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 21 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 22
    pub mod HRS22 {
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

            /// 0b0: A hardware service request for channel 22 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 22 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 23
    pub mod HRS23 {
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

            /// 0b0: A hardware service request for channel 23 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 23 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 24
    pub mod HRS24 {
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

            /// 0b0: A hardware service request for channel 24 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 24 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 25
    pub mod HRS25 {
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

            /// 0b0: A hardware service request for channel 25 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 25 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 26
    pub mod HRS26 {
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

            /// 0b0: A hardware service request for channel 26 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 26 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 27
    pub mod HRS27 {
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

            /// 0b0: A hardware service request for channel 27 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 27 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 28
    pub mod HRS28 {
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

            /// 0b0: A hardware service request for channel 28 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 28 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 29
    pub mod HRS29 {
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

            /// 0b0: A hardware service request for channel 29 is not preset
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 29 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 30
    pub mod HRS30 {
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

            /// 0b0: A hardware service request for channel 30 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 30 is present
            pub const HWRQST: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 31
    pub mod HRS31 {
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

            /// 0b0: A hardware service request for channel 31 is not present
            pub const NO_HWRQST: u32 = 0b0;

            /// 0b1: A hardware service request for channel 31 is present
            pub const HWRQST: u32 = 0b1;
        }
    }
}

/// Enable Asynchronous Request in Stop
pub mod EARS {

    /// Enable asynchronous DMA request in stop mode for channel 0.
    pub mod EDREQ_0 {
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

            /// 0b0: Disable asynchronous DMA request for channel 0
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 0
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 1.
    pub mod EDREQ_1 {
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

            /// 0b0: Disable asynchronous DMA request for channel 1
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 1
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 2.
    pub mod EDREQ_2 {
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

            /// 0b0: Disable asynchronous DMA request for channel 2
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 2
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 3.
    pub mod EDREQ_3 {
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

            /// 0b0: Disable asynchronous DMA request for channel 3
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 3
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 4.
    pub mod EDREQ_4 {
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

            /// 0b0: Disable asynchronous DMA request for channel 4
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 4
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 5.
    pub mod EDREQ_5 {
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

            /// 0b0: Disable asynchronous DMA request for channel 5
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 5
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 6.
    pub mod EDREQ_6 {
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

            /// 0b0: Disable asynchronous DMA request for channel 6
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 6
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 7.
    pub mod EDREQ_7 {
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

            /// 0b0: Disable asynchronous DMA request for channel 7
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 7
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 8.
    pub mod EDREQ_8 {
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

            /// 0b0: Disable asynchronous DMA request for channel 8
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 8
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 9.
    pub mod EDREQ_9 {
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

            /// 0b0: Disable asynchronous DMA request for channel 9
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 9
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 10.
    pub mod EDREQ_10 {
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

            /// 0b0: Disable asynchronous DMA request for channel 10
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 10
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 11.
    pub mod EDREQ_11 {
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

            /// 0b0: Disable asynchronous DMA request for channel 11
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 11
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 12.
    pub mod EDREQ_12 {
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

            /// 0b0: Disable asynchronous DMA request for channel 12
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 12
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 13.
    pub mod EDREQ_13 {
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

            /// 0b0: Disable asynchronous DMA request for channel 13
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 13
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 14.
    pub mod EDREQ_14 {
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

            /// 0b0: Disable asynchronous DMA request for channel 14
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 14
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 15.
    pub mod EDREQ_15 {
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

            /// 0b0: Disable asynchronous DMA request for channel 15
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 15
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 16.
    pub mod EDREQ_16 {
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

            /// 0b0: Disable asynchronous DMA request for channel 16
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 16
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 17.
    pub mod EDREQ_17 {
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

            /// 0b0: Disable asynchronous DMA request for channel 17
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 17
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 18.
    pub mod EDREQ_18 {
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

            /// 0b0: Disable asynchronous DMA request for channel 18
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 18
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 19.
    pub mod EDREQ_19 {
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

            /// 0b0: Disable asynchronous DMA request for channel 19
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 19
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 20.
    pub mod EDREQ_20 {
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

            /// 0b0: Disable asynchronous DMA request for channel 20
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 20
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 21.
    pub mod EDREQ_21 {
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

            /// 0b0: Disable asynchronous DMA request for channel 21
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 21
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 22.
    pub mod EDREQ_22 {
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

            /// 0b0: Disable asynchronous DMA request for channel 22
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 22
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 23.
    pub mod EDREQ_23 {
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

            /// 0b0: Disable asynchronous DMA request for channel 23
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 23
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 24.
    pub mod EDREQ_24 {
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

            /// 0b0: Disable asynchronous DMA request for channel 24
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 24
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 25.
    pub mod EDREQ_25 {
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

            /// 0b0: Disable asynchronous DMA request for channel 25
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 25
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 26.
    pub mod EDREQ_26 {
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

            /// 0b0: Disable asynchronous DMA request for channel 26
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 26
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 27.
    pub mod EDREQ_27 {
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

            /// 0b0: Disable asynchronous DMA request for channel 27
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 27
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 28.
    pub mod EDREQ_28 {
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

            /// 0b0: Disable asynchronous DMA request for channel 28
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 28
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 29.
    pub mod EDREQ_29 {
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

            /// 0b0: Disable asynchronous DMA request for channel 29
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 29
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 30.
    pub mod EDREQ_30 {
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

            /// 0b0: Disable asynchronous DMA request for channel 30
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 30
            pub const ENABLE: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 31.
    pub mod EDREQ_31 {
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

            /// 0b0: Disable asynchronous DMA request for channel 31
            pub const DISABLE: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 31
            pub const ENABLE: u32 = 0b1;
        }
    }
}

/// Channel Priority
pub mod DCHPRI3 {

    /// Channel n Arbitration Priority
    pub mod CHPRI {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u8 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel n Current Group Priority
    pub mod GRPPRI {
        /// Offset (4 bits)
        pub const offset: u8 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u8 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Disable Preempt Ability. This field resets to 0.
    pub mod DPA {
        /// Offset (6 bits)
        pub const offset: u8 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Channel n can suspend a lower priority channel
            pub const ENABLED: u8 = 0b0;

            /// 0b1: Channel n cannot suspend any channel, regardless of channel priority
            pub const DISABLED: u8 = 0b1;
        }
    }

    /// Enable Channel Preemption. This field resets to 0.
    pub mod ECP {
        /// Offset (7 bits)
        pub const offset: u8 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Channel n cannot be suspended by a higher priority channel's service request
            pub const DISABLED: u8 = 0b0;

            /// 0b1: Channel n can be temporarily suspended by the service request of a higher priority channel
            pub const ENABLED: u8 = 0b1;
        }
    }
}

/// Channel Priority
pub mod DCHPRI2 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI1 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI0 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI7 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI6 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI5 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI4 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI11 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI10 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI9 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI8 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI15 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI14 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI13 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI12 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI19 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI18 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI17 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI16 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI23 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI22 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI21 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI20 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI27 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI26 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI25 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI24 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI31 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI30 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI29 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority
pub mod DCHPRI28 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// TCD Source Address
pub mod TCD_SADDR_0 {

    /// Source Address
    pub mod SADDR {
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

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_0 {

    /// Source address signed offset
    pub mod SOFF {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_0 {

    /// Destination data transfer size
    pub mod DSIZE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Destination Address Modulo
    pub mod DMOD {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (5 bits: 0b11111 << 3)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Source data transfer size
    pub mod SSIZE {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 8-bit
            pub const EIGHT: u16 = 0b000;

            /// 0b001: 16-bit
            pub const SIXTEEN_BIT: u16 = 0b001;

            /// 0b010: 32-bit
            pub const THIRTYTWO_BIT: u16 = 0b010;

            /// 0b011: 64-bit
            pub const SIXTYFOUR: u16 = 0b011;

            /// 0b101: 32-byte burst (4 beats of 64 bits)
            pub const THIRTYTWO_BYTE: u16 = 0b101;
        }
    }

    /// Source Address Modulo
    pub mod SMOD {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Disabled
            pub const DISABLED: u16 = 0b00000;

            /// 0b00001: ENABLED
            pub const ENABLED: u16 = 0b00001;
        }
    }
}

/// TCD_NBYTES_ML and TCD_NBYTES_MLOFFYES_0
/// TCD_NBYTES_ML: TCD_NBYTES_MLNO_0 and TCD_NBYTES_MLOFFNO_0
/// TCD_NBYTES_MLNO_0: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_0: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_0: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_ML {

    /// Minor Byte Transfer Count
    pub mod NBYTES {
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

    /// Destination Minor Loop Offset Enable
    pub mod DMLOE {
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

            /// 0b0: The minor loop offset is not applied to the DADDR
            pub const DISABLED: u32 = 0b0;

            /// 0b1: The minor loop offset is applied to the DADDR
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// Source Minor Loop Offset Enable
    pub mod SMLOE {
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

            /// 0b0: The minor loop offset is not applied to the SADDR
            pub const DISABLED: u32 = 0b0;

            /// 0b1: The minor loop offset is applied to the SADDR
            pub const ENABLED: u32 = 0b1;
        }
    }

    /// If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes.
    pub mod MLOFF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (20 bits: 0xfffff << 10)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_0 {

    /// Last Source Address Adjustment
    pub mod SLAST {
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

/// TCD Destination Address
pub mod TCD_DADDR_0 {

    /// Destination Address
    pub mod DADDR {
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

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_0 {

    /// Destination Address Signed Offset
    pub mod DOFF {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TCD_CITER_ELINKNO_0 and TCD_CITER_ELINKYES_0
/// TCD_CITER_ELINKNO_0: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_0: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINK {

    /// Current Major Iteration Count
    pub mod CITER {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (15 bits: 0x7fff << 0)
        pub const mask: u16 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable channel-to-channel linking on minor-loop complete
    pub mod ELINK {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Channel-to-channel linking is disabled
            pub const DISABLED: u16 = 0b0;

            /// 0b1: Channel-to-channel linking is enabled
            pub const ENABLED: u16 = 0b1;
        }
    }

    /// Minor Loop Link Channel Number
    pub mod LINKCH {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (5 bits: 0b11111 << 9)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_0 {

    /// Destination last address adjustment, or next memory address TCD for channel (scatter/gather)
    pub mod DLASTSGA {
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

/// TCD Control and Status
pub mod TCD_CSR_0 {

    /// Channel Start
    pub mod START {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Channel is not explicitly started
            pub const NO_START: u16 = 0b0;

            /// 0b1: Channel is explicitly started via a software initiated service request
            pub const START: u16 = 0b1;
        }
    }

    /// Enable an interrupt when major iteration count completes.
    pub mod INTMAJOR {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: End of major loop interrupt is disabled
            pub const DISABLED: u16 = 0b0;

            /// 0b1: End of major loop interrupt is enabled
            pub const ENABLED: u16 = 0b1;
        }
    }

    /// Enable an interrupt when major counter is half complete.
    pub mod INTHALF {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Half-point interrupt is disabled
            pub const DISABLED: u16 = 0b0;

            /// 0b1: Half-point interrupt is enabled
            pub const ENABLED: u16 = 0b1;
        }
    }

    /// Disable Request
    pub mod DREQ {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The channel's ERQ field is not affected
            pub const NO_CLEAR: u16 = 0b0;

            /// 0b1: The channel's ERQ field value changes to 0 when the major loop is complete
            pub const CLEAR: u16 = 0b1;
        }
    }

    /// Enable Scatter/Gather Processing
    pub mod ESG {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The current channel's TCD is normal format
            pub const NORMAL: u16 = 0b0;

            /// 0b1: The current channel's TCD specifies a scatter gather format
            pub const SCATTER: u16 = 0b1;
        }
    }

    /// Enable channel-to-channel linking on major loop complete
    pub mod MAJORELINK {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Channel-to-channel linking is disabled
            pub const DISABLED: u16 = 0b0;

            /// 0b1: Channel-to-channel linking is enabled
            pub const ENABLED: u16 = 0b1;
        }
    }

    /// Channel Active
    pub mod ACTIVE {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel Done
    pub mod DONE {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Major Loop Link Channel Number
    pub mod MAJORLINKCH {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bandwidth Control
    pub mod BWC {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No eDMA engine stalls
            pub const DISABLED: u16 = 0b00;

            /// 0b10: eDMA engine stalls for 4 cycles after each R/W
            pub const STALL4: u16 = 0b10;

            /// 0b11: eDMA engine stalls for 8 cycles after each R/W
            pub const STALL8: u16 = 0b11;
        }
    }
}

/// TCD_BITER_ELINKNO_0 and TCD_BITER_ELINKYES_0
/// TCD_BITER_ELINKNO_0: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_0: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINK {

    /// Starting Major Iteration Count
    pub mod BITER {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (15 bits: 0x7fff << 0)
        pub const mask: u16 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enables channel-to-channel linking on minor loop complete
    pub mod ELINK {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Channel-to-channel linking is disabled
            pub const DISABLED: u16 = 0b0;

            /// 0b1: Channel-to-channel linking is enabled
            pub const ENABLED: u16 = 0b1;
        }
    }

    /// Link Channel Number
    pub mod LINKCH {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (5 bits: 0b11111 << 9)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TCD Source Address
pub mod TCD_SADDR_1 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_1 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_1 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_1 and TCD_NBYTES_MLOFFYES_1
/// TCD_NBYTES_MLNO_1: TCD_NBYTES_MLNO_1 and TCD_NBYTES_MLOFFNO_1
/// TCD_NBYTES_MLNO_1: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_1: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_1: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_1 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_1 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_1 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_1 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_1 and TCD_CITER_ELINKYES_1
/// TCD_CITER_ELINKNO_1: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_1: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_1 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_1 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_1 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_1 and TCD_BITER_ELINKYES_1
/// TCD_BITER_ELINKNO_1: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_1: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_1 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_2 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_2 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_2 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_2 and TCD_NBYTES_MLOFFYES_2
/// TCD_NBYTES_MLNO_2: TCD_NBYTES_MLNO_2 and TCD_NBYTES_MLOFFNO_2
/// TCD_NBYTES_MLNO_2: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_2: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_2: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_2 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_2 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_2 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_2 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_2 and TCD_CITER_ELINKYES_2
/// TCD_CITER_ELINKNO_2: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_2: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_2 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_2 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_2 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_2 and TCD_BITER_ELINKYES_2
/// TCD_BITER_ELINKNO_2: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_2: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_2 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_3 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_3 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_3 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_3 and TCD_NBYTES_MLOFFYES_3
/// TCD_NBYTES_MLNO_3: TCD_NBYTES_MLNO_3 and TCD_NBYTES_MLOFFNO_3
/// TCD_NBYTES_MLNO_3: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_3: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_3: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_3 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_3 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_3 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_3 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_3 and TCD_CITER_ELINKYES_3
/// TCD_CITER_ELINKNO_3: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_3: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_3 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_3 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_3 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_3 and TCD_BITER_ELINKYES_3
/// TCD_BITER_ELINKNO_3: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_3: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_3 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_4 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_4 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_4 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_4 and TCD_NBYTES_MLOFFYES_4
/// TCD_NBYTES_MLNO_4: TCD_NBYTES_MLNO_4 and TCD_NBYTES_MLOFFNO_4
/// TCD_NBYTES_MLNO_4: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_4: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_4: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_4 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_4 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_4 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_4 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_4 and TCD_CITER_ELINKYES_4
/// TCD_CITER_ELINKNO_4: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_4: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_4 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_4 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_4 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_4 and TCD_BITER_ELINKYES_4
/// TCD_BITER_ELINKNO_4: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_4: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_4 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_5 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_5 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_5 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_5 and TCD_NBYTES_MLOFFYES_5
/// TCD_NBYTES_MLNO_5: TCD_NBYTES_MLNO_5 and TCD_NBYTES_MLOFFNO_5
/// TCD_NBYTES_MLNO_5: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_5: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_5: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_5 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_5 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_5 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_5 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_5 and TCD_CITER_ELINKYES_5
/// TCD_CITER_ELINKNO_5: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_5: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_5 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_5 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_5 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_5 and TCD_BITER_ELINKYES_5
/// TCD_BITER_ELINKNO_5: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_5: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_5 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_6 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_6 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_6 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_6 and TCD_NBYTES_MLOFFYES_6
/// TCD_NBYTES_MLNO_6: TCD_NBYTES_MLNO_6 and TCD_NBYTES_MLOFFNO_6
/// TCD_NBYTES_MLNO_6: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_6: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_6: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_6 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_6 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_6 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_6 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_6 and TCD_CITER_ELINKYES_6
/// TCD_CITER_ELINKNO_6: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_6: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_6 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_6 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_6 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_6 and TCD_BITER_ELINKYES_6
/// TCD_BITER_ELINKNO_6: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_6: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_6 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_7 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_7 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_7 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_7 and TCD_NBYTES_MLOFFYES_7
/// TCD_NBYTES_MLNO_7: TCD_NBYTES_MLNO_7 and TCD_NBYTES_MLOFFNO_7
/// TCD_NBYTES_MLNO_7: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_7: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_7: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_7 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_7 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_7 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_7 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_7 and TCD_CITER_ELINKYES_7
/// TCD_CITER_ELINKNO_7: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_7: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_7 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_7 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_7 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_7 and TCD_BITER_ELINKYES_7
/// TCD_BITER_ELINKNO_7: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_7: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_7 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_8 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_8 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_8 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_8 and TCD_NBYTES_MLOFFYES_8
/// TCD_NBYTES_MLNO_8: TCD_NBYTES_MLNO_8 and TCD_NBYTES_MLOFFNO_8
/// TCD_NBYTES_MLNO_8: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_8: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_8: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_8 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_8 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_8 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_8 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_8 and TCD_CITER_ELINKYES_8
/// TCD_CITER_ELINKNO_8: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_8: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_8 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_8 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_8 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_8 and TCD_BITER_ELINKYES_8
/// TCD_BITER_ELINKNO_8: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_8: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_8 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_9 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_9 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_9 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_9 and TCD_NBYTES_MLOFFYES_9
/// TCD_NBYTES_MLNO_9: TCD_NBYTES_MLNO_9 and TCD_NBYTES_MLOFFNO_9
/// TCD_NBYTES_MLNO_9: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_9: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_9: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_9 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_9 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_9 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_9 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_9 and TCD_CITER_ELINKYES_9
/// TCD_CITER_ELINKNO_9: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_9: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_9 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_9 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_9 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_9 and TCD_BITER_ELINKYES_9
/// TCD_BITER_ELINKNO_9: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_9: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_9 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_10 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_10 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_10 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_10 and TCD_NBYTES_MLOFFYES_10
/// TCD_NBYTES_MLNO_10: TCD_NBYTES_MLNO_10 and TCD_NBYTES_MLOFFNO_10
/// TCD_NBYTES_MLNO_10: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_10: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_10: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_10 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_10 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_10 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_10 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_10 and TCD_CITER_ELINKYES_10
/// TCD_CITER_ELINKNO_10: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_10: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_10 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_10 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_10 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_10 and TCD_BITER_ELINKYES_10
/// TCD_BITER_ELINKNO_10: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_10: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_10 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_11 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_11 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_11 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_11 and TCD_NBYTES_MLOFFYES_11
/// TCD_NBYTES_MLNO_11: TCD_NBYTES_MLNO_11 and TCD_NBYTES_MLOFFNO_11
/// TCD_NBYTES_MLNO_11: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_11: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_11: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_11 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_11 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_11 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_11 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_11 and TCD_CITER_ELINKYES_11
/// TCD_CITER_ELINKNO_11: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_11: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_11 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_11 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_11 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_11 and TCD_BITER_ELINKYES_11
/// TCD_BITER_ELINKNO_11: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_11: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_11 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_12 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_12 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_12 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_12 and TCD_NBYTES_MLOFFYES_12
/// TCD_NBYTES_MLNO_12: TCD_NBYTES_MLNO_12 and TCD_NBYTES_MLOFFNO_12
/// TCD_NBYTES_MLNO_12: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_12: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_12: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_12 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_12 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_12 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_12 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_12 and TCD_CITER_ELINKYES_12
/// TCD_CITER_ELINKNO_12: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_12: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_12 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_12 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_12 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_12 and TCD_BITER_ELINKYES_12
/// TCD_BITER_ELINKNO_12: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_12: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_12 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_13 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_13 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_13 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_13 and TCD_NBYTES_MLOFFYES_13
/// TCD_NBYTES_MLNO_13: TCD_NBYTES_MLNO_13 and TCD_NBYTES_MLOFFNO_13
/// TCD_NBYTES_MLNO_13: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_13: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_13: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_13 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_13 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_13 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_13 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_13 and TCD_CITER_ELINKYES_13
/// TCD_CITER_ELINKNO_13: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_13: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_13 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_13 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_13 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_13 and TCD_BITER_ELINKYES_13
/// TCD_BITER_ELINKNO_13: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_13: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_13 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_14 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_14 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_14 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_14 and TCD_NBYTES_MLOFFYES_14
/// TCD_NBYTES_MLNO_14: TCD_NBYTES_MLNO_14 and TCD_NBYTES_MLOFFNO_14
/// TCD_NBYTES_MLNO_14: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_14: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_14: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_14 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_14 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_14 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_14 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_14 and TCD_CITER_ELINKYES_14
/// TCD_CITER_ELINKNO_14: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_14: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_14 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_14 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_14 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_14 and TCD_BITER_ELINKYES_14
/// TCD_BITER_ELINKNO_14: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_14: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_14 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_15 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_15 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_15 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_15 and TCD_NBYTES_MLOFFYES_15
/// TCD_NBYTES_MLNO_15: TCD_NBYTES_MLNO_15 and TCD_NBYTES_MLOFFNO_15
/// TCD_NBYTES_MLNO_15: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_15: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_15: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_15 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_15 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_15 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_15 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_15 and TCD_CITER_ELINKYES_15
/// TCD_CITER_ELINKNO_15: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_15: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_15 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_15 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_15 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_15 and TCD_BITER_ELINKYES_15
/// TCD_BITER_ELINKNO_15: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_15: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_15 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_16 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_16 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_16 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_16 and TCD_NBYTES_MLOFFYES_16
/// TCD_NBYTES_MLNO_16: TCD_NBYTES_MLNO_16 and TCD_NBYTES_MLOFFNO_16
/// TCD_NBYTES_MLNO_16: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_16: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_16: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_16 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_16 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_16 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_16 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_16 and TCD_CITER_ELINKYES_16
/// TCD_CITER_ELINKNO_16: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_16: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_16 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_16 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_16 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_16 and TCD_BITER_ELINKYES_16
/// TCD_BITER_ELINKNO_16: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_16: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_16 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_17 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_17 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_17 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_17 and TCD_NBYTES_MLOFFYES_17
/// TCD_NBYTES_MLNO_17: TCD_NBYTES_MLNO_17 and TCD_NBYTES_MLOFFNO_17
/// TCD_NBYTES_MLNO_17: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_17: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_17: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_17 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_17 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_17 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_17 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_17 and TCD_CITER_ELINKYES_17
/// TCD_CITER_ELINKNO_17: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_17: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_17 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_17 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_17 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_17 and TCD_BITER_ELINKYES_17
/// TCD_BITER_ELINKNO_17: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_17: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_17 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_18 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_18 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_18 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_18 and TCD_NBYTES_MLOFFYES_18
/// TCD_NBYTES_MLNO_18: TCD_NBYTES_MLNO_18 and TCD_NBYTES_MLOFFNO_18
/// TCD_NBYTES_MLNO_18: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_18: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_18: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_18 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_18 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_18 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_18 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_18 and TCD_CITER_ELINKYES_18
/// TCD_CITER_ELINKNO_18: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_18: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_18 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_18 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_18 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_18 and TCD_BITER_ELINKYES_18
/// TCD_BITER_ELINKNO_18: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_18: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_18 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_19 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_19 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_19 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_19 and TCD_NBYTES_MLOFFYES_19
/// TCD_NBYTES_MLNO_19: TCD_NBYTES_MLNO_19 and TCD_NBYTES_MLOFFNO_19
/// TCD_NBYTES_MLNO_19: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_19: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_19: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_19 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_19 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_19 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_19 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_19 and TCD_CITER_ELINKYES_19
/// TCD_CITER_ELINKNO_19: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_19: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_19 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_19 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_19 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_19 and TCD_BITER_ELINKYES_19
/// TCD_BITER_ELINKNO_19: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_19: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_19 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_20 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_20 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_20 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_20 and TCD_NBYTES_MLOFFYES_20
/// TCD_NBYTES_MLNO_20: TCD_NBYTES_MLNO_20 and TCD_NBYTES_MLOFFNO_20
/// TCD_NBYTES_MLNO_20: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_20: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_20: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_20 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_20 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_20 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_20 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_20 and TCD_CITER_ELINKYES_20
/// TCD_CITER_ELINKNO_20: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_20: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_20 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_20 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_20 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_20 and TCD_BITER_ELINKYES_20
/// TCD_BITER_ELINKNO_20: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_20: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_20 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_21 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_21 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_21 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_21 and TCD_NBYTES_MLOFFYES_21
/// TCD_NBYTES_MLNO_21: TCD_NBYTES_MLNO_21 and TCD_NBYTES_MLOFFNO_21
/// TCD_NBYTES_MLNO_21: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_21: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_21: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_21 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_21 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_21 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_21 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_21 and TCD_CITER_ELINKYES_21
/// TCD_CITER_ELINKNO_21: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_21: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_21 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_21 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_21 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_21 and TCD_BITER_ELINKYES_21
/// TCD_BITER_ELINKNO_21: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_21: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_21 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_22 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_22 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_22 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_22 and TCD_NBYTES_MLOFFYES_22
/// TCD_NBYTES_MLNO_22: TCD_NBYTES_MLNO_22 and TCD_NBYTES_MLOFFNO_22
/// TCD_NBYTES_MLNO_22: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_22: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_22: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_22 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_22 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_22 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_22 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_22 and TCD_CITER_ELINKYES_22
/// TCD_CITER_ELINKNO_22: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_22: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_22 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_22 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_22 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_22 and TCD_BITER_ELINKYES_22
/// TCD_BITER_ELINKNO_22: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_22: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_22 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_23 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_23 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_23 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_23 and TCD_NBYTES_MLOFFYES_23
/// TCD_NBYTES_MLNO_23: TCD_NBYTES_MLNO_23 and TCD_NBYTES_MLOFFNO_23
/// TCD_NBYTES_MLNO_23: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_23: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_23: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_23 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_23 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_23 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_23 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_23 and TCD_CITER_ELINKYES_23
/// TCD_CITER_ELINKNO_23: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_23: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_23 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_23 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_23 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_23 and TCD_BITER_ELINKYES_23
/// TCD_BITER_ELINKNO_23: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_23: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_23 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_24 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_24 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_24 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_24 and TCD_NBYTES_MLOFFYES_24
/// TCD_NBYTES_MLNO_24: TCD_NBYTES_MLNO_24 and TCD_NBYTES_MLOFFNO_24
/// TCD_NBYTES_MLNO_24: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_24: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_24: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_24 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_24 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_24 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_24 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_24 and TCD_CITER_ELINKYES_24
/// TCD_CITER_ELINKNO_24: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_24: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_24 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_24 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_24 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_24 and TCD_BITER_ELINKYES_24
/// TCD_BITER_ELINKNO_24: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_24: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_24 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_25 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_25 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_25 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_25 and TCD_NBYTES_MLOFFYES_25
/// TCD_NBYTES_MLNO_25: TCD_NBYTES_MLNO_25 and TCD_NBYTES_MLOFFNO_25
/// TCD_NBYTES_MLNO_25: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_25: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_25: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_25 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_25 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_25 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_25 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_25 and TCD_CITER_ELINKYES_25
/// TCD_CITER_ELINKNO_25: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_25: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_25 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_25 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_25 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_25 and TCD_BITER_ELINKYES_25
/// TCD_BITER_ELINKNO_25: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_25: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_25 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_26 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_26 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_26 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_26 and TCD_NBYTES_MLOFFYES_26
/// TCD_NBYTES_MLNO_26: TCD_NBYTES_MLNO_26 and TCD_NBYTES_MLOFFNO_26
/// TCD_NBYTES_MLNO_26: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_26: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_26: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_26 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_26 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_26 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_26 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_26 and TCD_CITER_ELINKYES_26
/// TCD_CITER_ELINKNO_26: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_26: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_26 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_26 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_26 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_26 and TCD_BITER_ELINKYES_26
/// TCD_BITER_ELINKNO_26: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_26: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_26 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_27 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_27 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_27 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_27 and TCD_NBYTES_MLOFFYES_27
/// TCD_NBYTES_MLNO_27: TCD_NBYTES_MLNO_27 and TCD_NBYTES_MLOFFNO_27
/// TCD_NBYTES_MLNO_27: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_27: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_27: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_27 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_27 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_27 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_27 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_27 and TCD_CITER_ELINKYES_27
/// TCD_CITER_ELINKNO_27: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_27: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_27 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_27 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_27 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_27 and TCD_BITER_ELINKYES_27
/// TCD_BITER_ELINKNO_27: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_27: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_27 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_28 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_28 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_28 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_28 and TCD_NBYTES_MLOFFYES_28
/// TCD_NBYTES_MLNO_28: TCD_NBYTES_MLNO_28 and TCD_NBYTES_MLOFFNO_28
/// TCD_NBYTES_MLNO_28: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_28: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_28: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_28 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_28 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_28 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_28 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_28 and TCD_CITER_ELINKYES_28
/// TCD_CITER_ELINKNO_28: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_28: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_28 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_28 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_28 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_28 and TCD_BITER_ELINKYES_28
/// TCD_BITER_ELINKNO_28: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_28: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_28 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_29 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_29 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_29 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_29 and TCD_NBYTES_MLOFFYES_29
/// TCD_NBYTES_MLNO_29: TCD_NBYTES_MLNO_29 and TCD_NBYTES_MLOFFNO_29
/// TCD_NBYTES_MLNO_29: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_29: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_29: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_29 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_29 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_29 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_29 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_29 and TCD_CITER_ELINKYES_29
/// TCD_CITER_ELINKNO_29: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_29: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_29 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_29 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_29 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_29 and TCD_BITER_ELINKYES_29
/// TCD_BITER_ELINKNO_29: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_29: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_29 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_30 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_30 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_30 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_30 and TCD_NBYTES_MLOFFYES_30
/// TCD_NBYTES_MLNO_30: TCD_NBYTES_MLNO_30 and TCD_NBYTES_MLOFFNO_30
/// TCD_NBYTES_MLNO_30: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_30: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_30: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_30 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_30 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_30 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_30 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_30 and TCD_CITER_ELINKYES_30
/// TCD_CITER_ELINKNO_30: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_30: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_30 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_30 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_30 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_30 and TCD_BITER_ELINKYES_30
/// TCD_BITER_ELINKNO_30: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_30: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_30 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR_31 {
    pub use super::TCD_SADDR_0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF_31 {
    pub use super::TCD_SOFF_0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR_31 {
    pub use super::TCD_ATTR_0::DMOD;
    pub use super::TCD_ATTR_0::DSIZE;
    pub use super::TCD_ATTR_0::SMOD;
    pub use super::TCD_ATTR_0::SSIZE;
}

/// TCD_NBYTES_MLNO_31 and TCD_NBYTES_MLOFFYES_31
/// TCD_NBYTES_MLNO_31: TCD_NBYTES_MLNO_31 and TCD_NBYTES_MLOFFNO_31
/// TCD_NBYTES_MLNO_31: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO_31: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES_31: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO_31 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST_31 {
    pub use super::TCD_SLAST_0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR_31 {
    pub use super::TCD_DADDR_0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF_31 {
    pub use super::TCD_DOFF_0::DOFF;
}

/// TCD_CITER_ELINKNO_31 and TCD_CITER_ELINKYES_31
/// TCD_CITER_ELINKNO_31: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES_31: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO_31 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA_31 {
    pub use super::TCD_DLASTSGA_0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR_31 {
    pub use super::TCD_CSR_0::ACTIVE;
    pub use super::TCD_CSR_0::BWC;
    pub use super::TCD_CSR_0::DONE;
    pub use super::TCD_CSR_0::DREQ;
    pub use super::TCD_CSR_0::ESG;
    pub use super::TCD_CSR_0::INTHALF;
    pub use super::TCD_CSR_0::INTMAJOR;
    pub use super::TCD_CSR_0::MAJORELINK;
    pub use super::TCD_CSR_0::MAJORLINKCH;
    pub use super::TCD_CSR_0::START;
}

/// TCD_BITER_ELINKNO_31 and TCD_BITER_ELINKYES_31
/// TCD_BITER_ELINKNO_31: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES_31: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO_31 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Control
    pub CR: RWRegister<u32>,

    /// Error Status
    pub ES: RORegister<u32>,

    _reserved1: [u32; 1],

    /// Enable Request
    pub ERQ: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// Enable Error Interrupt
    pub EEI: RWRegister<u32>,

    /// Clear Enable Error Interrupt
    pub CEEI: RWRegister<u8>,

    /// Set Enable Error Interrupt
    pub SEEI: RWRegister<u8>,

    /// Clear Enable Request
    pub CERQ: RWRegister<u8>,

    /// Set Enable Request
    pub SERQ: RWRegister<u8>,

    /// Clear DONE Status Bit
    pub CDNE: RWRegister<u8>,

    /// Set START Bit
    pub SSRT: RWRegister<u8>,

    /// Clear Error
    pub CERR: RWRegister<u8>,

    /// Clear Interrupt Request
    pub CINT: RWRegister<u8>,

    _reserved3: [u32; 1],

    /// Interrupt Request
    pub INT: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// Error
    pub ERR: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// Hardware Request Status
    pub HRS: RORegister<u32>,

    _reserved6: [u32; 3],

    /// Enable Asynchronous Request in Stop
    pub EARS: RWRegister<u32>,

    _reserved7: [u32; 46],

    /// Channel Priority
    pub DCHPRI3: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI2: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI1: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI0: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI7: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI6: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI5: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI4: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI11: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI10: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI9: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI8: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI15: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI14: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI13: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI12: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI19: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI18: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI17: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI16: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI23: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI22: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI21: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI20: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI27: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI26: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI25: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI24: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI31: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI30: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI29: RWRegister<u8>,

    /// Channel Priority
    pub DCHPRI28: RWRegister<u8>,

    _reserved8: [u32; 952],

    /// TCD Source Address
    pub TCD_SADDR_0: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_0: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_0: RWRegister<u16>,

    /// TCD_NBYTES_ML and TCD_NBYTES_MLOFFYES_0
    /// TCD_NBYTES_ML: TCD_NBYTES_MLNO_0 and TCD_NBYTES_MLOFFNO_0
    /// TCD_NBYTES_MLNO_0: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_0: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_0: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_0: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_0: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_0: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_0 and TCD_CITER_ELINKYES_0
    /// TCD_CITER_ELINKNO_0: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_0: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_0: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_0: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_0 and TCD_BITER_ELINKYES_0
    /// TCD_BITER_ELINKNO_0: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_0: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_1: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_1: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_1: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_1 and TCD_NBYTES_MLOFFYES_1
    /// TCD_NBYTES_MLNO_1: TCD_NBYTES_MLNO_1 and TCD_NBYTES_MLOFFNO_1
    /// TCD_NBYTES_MLNO_1: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_1: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_1: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_1: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_1: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_1: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_1: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_1 and TCD_CITER_ELINKYES_1
    /// TCD_CITER_ELINKNO_1: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_1: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_1: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_1: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_1: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_1 and TCD_BITER_ELINKYES_1
    /// TCD_BITER_ELINKNO_1: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_1: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_1: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_2: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_2: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_2: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_2 and TCD_NBYTES_MLOFFYES_2
    /// TCD_NBYTES_MLNO_2: TCD_NBYTES_MLNO_2 and TCD_NBYTES_MLOFFNO_2
    /// TCD_NBYTES_MLNO_2: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_2: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_2: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_2: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_2: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_2: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_2: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_2 and TCD_CITER_ELINKYES_2
    /// TCD_CITER_ELINKNO_2: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_2: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_2: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_2: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_2: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_2 and TCD_BITER_ELINKYES_2
    /// TCD_BITER_ELINKNO_2: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_2: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_2: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_3: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_3: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_3: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_3 and TCD_NBYTES_MLOFFYES_3
    /// TCD_NBYTES_MLNO_3: TCD_NBYTES_MLNO_3 and TCD_NBYTES_MLOFFNO_3
    /// TCD_NBYTES_MLNO_3: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_3: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_3: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_3: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_3: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_3: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_3: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_3 and TCD_CITER_ELINKYES_3
    /// TCD_CITER_ELINKNO_3: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_3: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_3: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_3: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_3: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_3 and TCD_BITER_ELINKYES_3
    /// TCD_BITER_ELINKNO_3: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_3: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_3: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_4: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_4: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_4: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_4 and TCD_NBYTES_MLOFFYES_4
    /// TCD_NBYTES_MLNO_4: TCD_NBYTES_MLNO_4 and TCD_NBYTES_MLOFFNO_4
    /// TCD_NBYTES_MLNO_4: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_4: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_4: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_4: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_4: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_4: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_4: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_4 and TCD_CITER_ELINKYES_4
    /// TCD_CITER_ELINKNO_4: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_4: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_4: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_4: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_4: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_4 and TCD_BITER_ELINKYES_4
    /// TCD_BITER_ELINKNO_4: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_4: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_4: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_5: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_5: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_5: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_5 and TCD_NBYTES_MLOFFYES_5
    /// TCD_NBYTES_MLNO_5: TCD_NBYTES_MLNO_5 and TCD_NBYTES_MLOFFNO_5
    /// TCD_NBYTES_MLNO_5: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_5: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_5: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_5: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_5: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_5: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_5: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_5 and TCD_CITER_ELINKYES_5
    /// TCD_CITER_ELINKNO_5: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_5: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_5: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_5: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_5: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_5 and TCD_BITER_ELINKYES_5
    /// TCD_BITER_ELINKNO_5: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_5: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_5: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_6: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_6: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_6: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_6 and TCD_NBYTES_MLOFFYES_6
    /// TCD_NBYTES_MLNO_6: TCD_NBYTES_MLNO_6 and TCD_NBYTES_MLOFFNO_6
    /// TCD_NBYTES_MLNO_6: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_6: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_6: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_6: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_6: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_6: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_6: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_6 and TCD_CITER_ELINKYES_6
    /// TCD_CITER_ELINKNO_6: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_6: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_6: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_6: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_6: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_6 and TCD_BITER_ELINKYES_6
    /// TCD_BITER_ELINKNO_6: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_6: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_6: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_7: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_7: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_7: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_7 and TCD_NBYTES_MLOFFYES_7
    /// TCD_NBYTES_MLNO_7: TCD_NBYTES_MLNO_7 and TCD_NBYTES_MLOFFNO_7
    /// TCD_NBYTES_MLNO_7: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_7: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_7: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_7: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_7: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_7: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_7: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_7 and TCD_CITER_ELINKYES_7
    /// TCD_CITER_ELINKNO_7: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_7: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_7: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_7: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_7: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_7 and TCD_BITER_ELINKYES_7
    /// TCD_BITER_ELINKNO_7: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_7: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_7: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_8: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_8: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_8: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_8 and TCD_NBYTES_MLOFFYES_8
    /// TCD_NBYTES_MLNO_8: TCD_NBYTES_MLNO_8 and TCD_NBYTES_MLOFFNO_8
    /// TCD_NBYTES_MLNO_8: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_8: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_8: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_8: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_8: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_8: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_8: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_8 and TCD_CITER_ELINKYES_8
    /// TCD_CITER_ELINKNO_8: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_8: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_8: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_8: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_8: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_8 and TCD_BITER_ELINKYES_8
    /// TCD_BITER_ELINKNO_8: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_8: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_8: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_9: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_9: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_9: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_9 and TCD_NBYTES_MLOFFYES_9
    /// TCD_NBYTES_MLNO_9: TCD_NBYTES_MLNO_9 and TCD_NBYTES_MLOFFNO_9
    /// TCD_NBYTES_MLNO_9: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_9: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_9: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_9: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_9: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_9: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_9: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_9 and TCD_CITER_ELINKYES_9
    /// TCD_CITER_ELINKNO_9: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_9: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_9: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_9: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_9: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_9 and TCD_BITER_ELINKYES_9
    /// TCD_BITER_ELINKNO_9: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_9: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_9: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_10: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_10: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_10: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_10 and TCD_NBYTES_MLOFFYES_10
    /// TCD_NBYTES_MLNO_10: TCD_NBYTES_MLNO_10 and TCD_NBYTES_MLOFFNO_10
    /// TCD_NBYTES_MLNO_10: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_10: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_10: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_10: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_10: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_10: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_10: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_10 and TCD_CITER_ELINKYES_10
    /// TCD_CITER_ELINKNO_10: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_10: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_10: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_10: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_10: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_10 and TCD_BITER_ELINKYES_10
    /// TCD_BITER_ELINKNO_10: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_10: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_10: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_11: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_11: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_11: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_11 and TCD_NBYTES_MLOFFYES_11
    /// TCD_NBYTES_MLNO_11: TCD_NBYTES_MLNO_11 and TCD_NBYTES_MLOFFNO_11
    /// TCD_NBYTES_MLNO_11: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_11: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_11: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_11: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_11: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_11: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_11: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_11 and TCD_CITER_ELINKYES_11
    /// TCD_CITER_ELINKNO_11: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_11: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_11: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_11: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_11: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_11 and TCD_BITER_ELINKYES_11
    /// TCD_BITER_ELINKNO_11: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_11: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_11: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_12: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_12: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_12: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_12 and TCD_NBYTES_MLOFFYES_12
    /// TCD_NBYTES_MLNO_12: TCD_NBYTES_MLNO_12 and TCD_NBYTES_MLOFFNO_12
    /// TCD_NBYTES_MLNO_12: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_12: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_12: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_12: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_12: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_12: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_12: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_12 and TCD_CITER_ELINKYES_12
    /// TCD_CITER_ELINKNO_12: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_12: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_12: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_12: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_12: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_12 and TCD_BITER_ELINKYES_12
    /// TCD_BITER_ELINKNO_12: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_12: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_12: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_13: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_13: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_13: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_13 and TCD_NBYTES_MLOFFYES_13
    /// TCD_NBYTES_MLNO_13: TCD_NBYTES_MLNO_13 and TCD_NBYTES_MLOFFNO_13
    /// TCD_NBYTES_MLNO_13: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_13: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_13: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_13: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_13: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_13: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_13: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_13 and TCD_CITER_ELINKYES_13
    /// TCD_CITER_ELINKNO_13: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_13: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_13: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_13: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_13: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_13 and TCD_BITER_ELINKYES_13
    /// TCD_BITER_ELINKNO_13: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_13: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_13: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_14: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_14: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_14: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_14 and TCD_NBYTES_MLOFFYES_14
    /// TCD_NBYTES_MLNO_14: TCD_NBYTES_MLNO_14 and TCD_NBYTES_MLOFFNO_14
    /// TCD_NBYTES_MLNO_14: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_14: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_14: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_14: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_14: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_14: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_14: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_14 and TCD_CITER_ELINKYES_14
    /// TCD_CITER_ELINKNO_14: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_14: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_14: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_14: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_14: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_14 and TCD_BITER_ELINKYES_14
    /// TCD_BITER_ELINKNO_14: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_14: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_14: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_15: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_15: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_15: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_15 and TCD_NBYTES_MLOFFYES_15
    /// TCD_NBYTES_MLNO_15: TCD_NBYTES_MLNO_15 and TCD_NBYTES_MLOFFNO_15
    /// TCD_NBYTES_MLNO_15: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_15: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_15: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_15: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_15: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_15: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_15: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_15 and TCD_CITER_ELINKYES_15
    /// TCD_CITER_ELINKNO_15: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_15: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_15: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_15: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_15: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_15 and TCD_BITER_ELINKYES_15
    /// TCD_BITER_ELINKNO_15: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_15: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_15: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_16: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_16: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_16: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_16 and TCD_NBYTES_MLOFFYES_16
    /// TCD_NBYTES_MLNO_16: TCD_NBYTES_MLNO_16 and TCD_NBYTES_MLOFFNO_16
    /// TCD_NBYTES_MLNO_16: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_16: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_16: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_16: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_16: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_16: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_16: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_16 and TCD_CITER_ELINKYES_16
    /// TCD_CITER_ELINKNO_16: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_16: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_16: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_16: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_16: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_16 and TCD_BITER_ELINKYES_16
    /// TCD_BITER_ELINKNO_16: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_16: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_16: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_17: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_17: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_17: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_17 and TCD_NBYTES_MLOFFYES_17
    /// TCD_NBYTES_MLNO_17: TCD_NBYTES_MLNO_17 and TCD_NBYTES_MLOFFNO_17
    /// TCD_NBYTES_MLNO_17: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_17: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_17: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_17: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_17: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_17: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_17: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_17 and TCD_CITER_ELINKYES_17
    /// TCD_CITER_ELINKNO_17: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_17: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_17: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_17: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_17: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_17 and TCD_BITER_ELINKYES_17
    /// TCD_BITER_ELINKNO_17: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_17: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_17: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_18: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_18: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_18: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_18 and TCD_NBYTES_MLOFFYES_18
    /// TCD_NBYTES_MLNO_18: TCD_NBYTES_MLNO_18 and TCD_NBYTES_MLOFFNO_18
    /// TCD_NBYTES_MLNO_18: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_18: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_18: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_18: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_18: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_18: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_18: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_18 and TCD_CITER_ELINKYES_18
    /// TCD_CITER_ELINKNO_18: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_18: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_18: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_18: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_18: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_18 and TCD_BITER_ELINKYES_18
    /// TCD_BITER_ELINKNO_18: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_18: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_18: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_19: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_19: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_19: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_19 and TCD_NBYTES_MLOFFYES_19
    /// TCD_NBYTES_MLNO_19: TCD_NBYTES_MLNO_19 and TCD_NBYTES_MLOFFNO_19
    /// TCD_NBYTES_MLNO_19: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_19: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_19: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_19: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_19: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_19: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_19: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_19 and TCD_CITER_ELINKYES_19
    /// TCD_CITER_ELINKNO_19: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_19: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_19: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_19: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_19: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_19 and TCD_BITER_ELINKYES_19
    /// TCD_BITER_ELINKNO_19: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_19: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_19: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_20: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_20: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_20: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_20 and TCD_NBYTES_MLOFFYES_20
    /// TCD_NBYTES_MLNO_20: TCD_NBYTES_MLNO_20 and TCD_NBYTES_MLOFFNO_20
    /// TCD_NBYTES_MLNO_20: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_20: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_20: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_20: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_20: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_20: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_20: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_20 and TCD_CITER_ELINKYES_20
    /// TCD_CITER_ELINKNO_20: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_20: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_20: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_20: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_20: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_20 and TCD_BITER_ELINKYES_20
    /// TCD_BITER_ELINKNO_20: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_20: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_20: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_21: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_21: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_21: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_21 and TCD_NBYTES_MLOFFYES_21
    /// TCD_NBYTES_MLNO_21: TCD_NBYTES_MLNO_21 and TCD_NBYTES_MLOFFNO_21
    /// TCD_NBYTES_MLNO_21: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_21: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_21: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_21: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_21: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_21: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_21: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_21 and TCD_CITER_ELINKYES_21
    /// TCD_CITER_ELINKNO_21: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_21: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_21: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_21: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_21: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_21 and TCD_BITER_ELINKYES_21
    /// TCD_BITER_ELINKNO_21: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_21: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_21: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_22: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_22: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_22: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_22 and TCD_NBYTES_MLOFFYES_22
    /// TCD_NBYTES_MLNO_22: TCD_NBYTES_MLNO_22 and TCD_NBYTES_MLOFFNO_22
    /// TCD_NBYTES_MLNO_22: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_22: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_22: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_22: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_22: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_22: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_22: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_22 and TCD_CITER_ELINKYES_22
    /// TCD_CITER_ELINKNO_22: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_22: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_22: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_22: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_22: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_22 and TCD_BITER_ELINKYES_22
    /// TCD_BITER_ELINKNO_22: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_22: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_22: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_23: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_23: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_23: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_23 and TCD_NBYTES_MLOFFYES_23
    /// TCD_NBYTES_MLNO_23: TCD_NBYTES_MLNO_23 and TCD_NBYTES_MLOFFNO_23
    /// TCD_NBYTES_MLNO_23: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_23: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_23: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_23: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_23: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_23: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_23: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_23 and TCD_CITER_ELINKYES_23
    /// TCD_CITER_ELINKNO_23: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_23: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_23: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_23: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_23: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_23 and TCD_BITER_ELINKYES_23
    /// TCD_BITER_ELINKNO_23: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_23: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_23: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_24: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_24: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_24: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_24 and TCD_NBYTES_MLOFFYES_24
    /// TCD_NBYTES_MLNO_24: TCD_NBYTES_MLNO_24 and TCD_NBYTES_MLOFFNO_24
    /// TCD_NBYTES_MLNO_24: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_24: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_24: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_24: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_24: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_24: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_24: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_24 and TCD_CITER_ELINKYES_24
    /// TCD_CITER_ELINKNO_24: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_24: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_24: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_24: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_24: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_24 and TCD_BITER_ELINKYES_24
    /// TCD_BITER_ELINKNO_24: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_24: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_24: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_25: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_25: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_25: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_25 and TCD_NBYTES_MLOFFYES_25
    /// TCD_NBYTES_MLNO_25: TCD_NBYTES_MLNO_25 and TCD_NBYTES_MLOFFNO_25
    /// TCD_NBYTES_MLNO_25: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_25: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_25: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_25: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_25: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_25: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_25: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_25 and TCD_CITER_ELINKYES_25
    /// TCD_CITER_ELINKNO_25: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_25: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_25: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_25: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_25: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_25 and TCD_BITER_ELINKYES_25
    /// TCD_BITER_ELINKNO_25: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_25: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_25: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_26: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_26: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_26: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_26 and TCD_NBYTES_MLOFFYES_26
    /// TCD_NBYTES_MLNO_26: TCD_NBYTES_MLNO_26 and TCD_NBYTES_MLOFFNO_26
    /// TCD_NBYTES_MLNO_26: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_26: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_26: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_26: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_26: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_26: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_26: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_26 and TCD_CITER_ELINKYES_26
    /// TCD_CITER_ELINKNO_26: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_26: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_26: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_26: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_26: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_26 and TCD_BITER_ELINKYES_26
    /// TCD_BITER_ELINKNO_26: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_26: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_26: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_27: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_27: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_27: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_27 and TCD_NBYTES_MLOFFYES_27
    /// TCD_NBYTES_MLNO_27: TCD_NBYTES_MLNO_27 and TCD_NBYTES_MLOFFNO_27
    /// TCD_NBYTES_MLNO_27: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_27: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_27: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_27: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_27: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_27: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_27: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_27 and TCD_CITER_ELINKYES_27
    /// TCD_CITER_ELINKNO_27: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_27: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_27: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_27: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_27: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_27 and TCD_BITER_ELINKYES_27
    /// TCD_BITER_ELINKNO_27: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_27: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_27: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_28: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_28: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_28: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_28 and TCD_NBYTES_MLOFFYES_28
    /// TCD_NBYTES_MLNO_28: TCD_NBYTES_MLNO_28 and TCD_NBYTES_MLOFFNO_28
    /// TCD_NBYTES_MLNO_28: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_28: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_28: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_28: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_28: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_28: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_28: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_28 and TCD_CITER_ELINKYES_28
    /// TCD_CITER_ELINKNO_28: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_28: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_28: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_28: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_28: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_28 and TCD_BITER_ELINKYES_28
    /// TCD_BITER_ELINKNO_28: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_28: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_28: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_29: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_29: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_29: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_29 and TCD_NBYTES_MLOFFYES_29
    /// TCD_NBYTES_MLNO_29: TCD_NBYTES_MLNO_29 and TCD_NBYTES_MLOFFNO_29
    /// TCD_NBYTES_MLNO_29: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_29: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_29: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_29: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_29: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_29: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_29: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_29 and TCD_CITER_ELINKYES_29
    /// TCD_CITER_ELINKNO_29: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_29: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_29: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_29: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_29: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_29 and TCD_BITER_ELINKYES_29
    /// TCD_BITER_ELINKNO_29: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_29: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_29: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_30: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_30: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_30: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_30 and TCD_NBYTES_MLOFFYES_30
    /// TCD_NBYTES_MLNO_30: TCD_NBYTES_MLNO_30 and TCD_NBYTES_MLOFFNO_30
    /// TCD_NBYTES_MLNO_30: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_30: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_30: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_30: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_30: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_30: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_30: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_30 and TCD_CITER_ELINKYES_30
    /// TCD_CITER_ELINKNO_30: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_30: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_30: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_30: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_30: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_30 and TCD_BITER_ELINKYES_30
    /// TCD_BITER_ELINKNO_30: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_30: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_30: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR_31: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF_31: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR_31: RWRegister<u16>,

    /// TCD_NBYTES_MLNO_31 and TCD_NBYTES_MLOFFYES_31
    /// TCD_NBYTES_MLNO_31: TCD_NBYTES_MLNO_31 and TCD_NBYTES_MLOFFNO_31
    /// TCD_NBYTES_MLNO_31: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO_31: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES_31: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO_31: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST_31: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR_31: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF_31: RWRegister<u16>,

    /// TCD_CITER_ELINKNO_31 and TCD_CITER_ELINKYES_31
    /// TCD_CITER_ELINKNO_31: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES_31: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO_31: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA_31: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR_31: RWRegister<u16>,

    /// TCD_BITER_ELINKNO_31 and TCD_BITER_ELINKYES_31
    /// TCD_BITER_ELINKNO_31: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES_31: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO_31: RWRegister<u16>,
}
pub struct ResetValues {
    pub CR: u32,
    pub ES: u32,
    pub ERQ: u32,
    pub EEI: u32,
    pub CEEI: u8,
    pub SEEI: u8,
    pub CERQ: u8,
    pub SERQ: u8,
    pub CDNE: u8,
    pub SSRT: u8,
    pub CERR: u8,
    pub CINT: u8,
    pub INT: u32,
    pub ERR: u32,
    pub HRS: u32,
    pub EARS: u32,
    pub DCHPRI3: u8,
    pub DCHPRI2: u8,
    pub DCHPRI1: u8,
    pub DCHPRI0: u8,
    pub DCHPRI7: u8,
    pub DCHPRI6: u8,
    pub DCHPRI5: u8,
    pub DCHPRI4: u8,
    pub DCHPRI11: u8,
    pub DCHPRI10: u8,
    pub DCHPRI9: u8,
    pub DCHPRI8: u8,
    pub DCHPRI15: u8,
    pub DCHPRI14: u8,
    pub DCHPRI13: u8,
    pub DCHPRI12: u8,
    pub DCHPRI19: u8,
    pub DCHPRI18: u8,
    pub DCHPRI17: u8,
    pub DCHPRI16: u8,
    pub DCHPRI23: u8,
    pub DCHPRI22: u8,
    pub DCHPRI21: u8,
    pub DCHPRI20: u8,
    pub DCHPRI27: u8,
    pub DCHPRI26: u8,
    pub DCHPRI25: u8,
    pub DCHPRI24: u8,
    pub DCHPRI31: u8,
    pub DCHPRI30: u8,
    pub DCHPRI29: u8,
    pub DCHPRI28: u8,
    pub TCD_SADDR_0: u32,
    pub TCD_SOFF_0: u16,
    pub TCD_ATTR_0: u16,
    pub TCD_NBYTES_ML: u32,
    pub TCD_SLAST_0: u32,
    pub TCD_DADDR_0: u32,
    pub TCD_DOFF_0: u16,
    pub TCD_CITER_ELINK: u16,
    pub TCD_DLASTSGA_0: u32,
    pub TCD_CSR_0: u16,
    pub TCD_BITER_ELINK: u16,
    pub TCD_SADDR_1: u32,
    pub TCD_SOFF_1: u16,
    pub TCD_ATTR_1: u16,
    pub TCD_NBYTES_MLNO_1: u32,
    pub TCD_SLAST_1: u32,
    pub TCD_DADDR_1: u32,
    pub TCD_DOFF_1: u16,
    pub TCD_CITER_ELINKNO_1: u16,
    pub TCD_DLASTSGA_1: u32,
    pub TCD_CSR_1: u16,
    pub TCD_BITER_ELINKNO_1: u16,
    pub TCD_SADDR_2: u32,
    pub TCD_SOFF_2: u16,
    pub TCD_ATTR_2: u16,
    pub TCD_NBYTES_MLNO_2: u32,
    pub TCD_SLAST_2: u32,
    pub TCD_DADDR_2: u32,
    pub TCD_DOFF_2: u16,
    pub TCD_CITER_ELINKNO_2: u16,
    pub TCD_DLASTSGA_2: u32,
    pub TCD_CSR_2: u16,
    pub TCD_BITER_ELINKNO_2: u16,
    pub TCD_SADDR_3: u32,
    pub TCD_SOFF_3: u16,
    pub TCD_ATTR_3: u16,
    pub TCD_NBYTES_MLNO_3: u32,
    pub TCD_SLAST_3: u32,
    pub TCD_DADDR_3: u32,
    pub TCD_DOFF_3: u16,
    pub TCD_CITER_ELINKNO_3: u16,
    pub TCD_DLASTSGA_3: u32,
    pub TCD_CSR_3: u16,
    pub TCD_BITER_ELINKNO_3: u16,
    pub TCD_SADDR_4: u32,
    pub TCD_SOFF_4: u16,
    pub TCD_ATTR_4: u16,
    pub TCD_NBYTES_MLNO_4: u32,
    pub TCD_SLAST_4: u32,
    pub TCD_DADDR_4: u32,
    pub TCD_DOFF_4: u16,
    pub TCD_CITER_ELINKNO_4: u16,
    pub TCD_DLASTSGA_4: u32,
    pub TCD_CSR_4: u16,
    pub TCD_BITER_ELINKNO_4: u16,
    pub TCD_SADDR_5: u32,
    pub TCD_SOFF_5: u16,
    pub TCD_ATTR_5: u16,
    pub TCD_NBYTES_MLNO_5: u32,
    pub TCD_SLAST_5: u32,
    pub TCD_DADDR_5: u32,
    pub TCD_DOFF_5: u16,
    pub TCD_CITER_ELINKNO_5: u16,
    pub TCD_DLASTSGA_5: u32,
    pub TCD_CSR_5: u16,
    pub TCD_BITER_ELINKNO_5: u16,
    pub TCD_SADDR_6: u32,
    pub TCD_SOFF_6: u16,
    pub TCD_ATTR_6: u16,
    pub TCD_NBYTES_MLNO_6: u32,
    pub TCD_SLAST_6: u32,
    pub TCD_DADDR_6: u32,
    pub TCD_DOFF_6: u16,
    pub TCD_CITER_ELINKNO_6: u16,
    pub TCD_DLASTSGA_6: u32,
    pub TCD_CSR_6: u16,
    pub TCD_BITER_ELINKNO_6: u16,
    pub TCD_SADDR_7: u32,
    pub TCD_SOFF_7: u16,
    pub TCD_ATTR_7: u16,
    pub TCD_NBYTES_MLNO_7: u32,
    pub TCD_SLAST_7: u32,
    pub TCD_DADDR_7: u32,
    pub TCD_DOFF_7: u16,
    pub TCD_CITER_ELINKNO_7: u16,
    pub TCD_DLASTSGA_7: u32,
    pub TCD_CSR_7: u16,
    pub TCD_BITER_ELINKNO_7: u16,
    pub TCD_SADDR_8: u32,
    pub TCD_SOFF_8: u16,
    pub TCD_ATTR_8: u16,
    pub TCD_NBYTES_MLNO_8: u32,
    pub TCD_SLAST_8: u32,
    pub TCD_DADDR_8: u32,
    pub TCD_DOFF_8: u16,
    pub TCD_CITER_ELINKNO_8: u16,
    pub TCD_DLASTSGA_8: u32,
    pub TCD_CSR_8: u16,
    pub TCD_BITER_ELINKNO_8: u16,
    pub TCD_SADDR_9: u32,
    pub TCD_SOFF_9: u16,
    pub TCD_ATTR_9: u16,
    pub TCD_NBYTES_MLNO_9: u32,
    pub TCD_SLAST_9: u32,
    pub TCD_DADDR_9: u32,
    pub TCD_DOFF_9: u16,
    pub TCD_CITER_ELINKNO_9: u16,
    pub TCD_DLASTSGA_9: u32,
    pub TCD_CSR_9: u16,
    pub TCD_BITER_ELINKNO_9: u16,
    pub TCD_SADDR_10: u32,
    pub TCD_SOFF_10: u16,
    pub TCD_ATTR_10: u16,
    pub TCD_NBYTES_MLNO_10: u32,
    pub TCD_SLAST_10: u32,
    pub TCD_DADDR_10: u32,
    pub TCD_DOFF_10: u16,
    pub TCD_CITER_ELINKNO_10: u16,
    pub TCD_DLASTSGA_10: u32,
    pub TCD_CSR_10: u16,
    pub TCD_BITER_ELINKNO_10: u16,
    pub TCD_SADDR_11: u32,
    pub TCD_SOFF_11: u16,
    pub TCD_ATTR_11: u16,
    pub TCD_NBYTES_MLNO_11: u32,
    pub TCD_SLAST_11: u32,
    pub TCD_DADDR_11: u32,
    pub TCD_DOFF_11: u16,
    pub TCD_CITER_ELINKNO_11: u16,
    pub TCD_DLASTSGA_11: u32,
    pub TCD_CSR_11: u16,
    pub TCD_BITER_ELINKNO_11: u16,
    pub TCD_SADDR_12: u32,
    pub TCD_SOFF_12: u16,
    pub TCD_ATTR_12: u16,
    pub TCD_NBYTES_MLNO_12: u32,
    pub TCD_SLAST_12: u32,
    pub TCD_DADDR_12: u32,
    pub TCD_DOFF_12: u16,
    pub TCD_CITER_ELINKNO_12: u16,
    pub TCD_DLASTSGA_12: u32,
    pub TCD_CSR_12: u16,
    pub TCD_BITER_ELINKNO_12: u16,
    pub TCD_SADDR_13: u32,
    pub TCD_SOFF_13: u16,
    pub TCD_ATTR_13: u16,
    pub TCD_NBYTES_MLNO_13: u32,
    pub TCD_SLAST_13: u32,
    pub TCD_DADDR_13: u32,
    pub TCD_DOFF_13: u16,
    pub TCD_CITER_ELINKNO_13: u16,
    pub TCD_DLASTSGA_13: u32,
    pub TCD_CSR_13: u16,
    pub TCD_BITER_ELINKNO_13: u16,
    pub TCD_SADDR_14: u32,
    pub TCD_SOFF_14: u16,
    pub TCD_ATTR_14: u16,
    pub TCD_NBYTES_MLNO_14: u32,
    pub TCD_SLAST_14: u32,
    pub TCD_DADDR_14: u32,
    pub TCD_DOFF_14: u16,
    pub TCD_CITER_ELINKNO_14: u16,
    pub TCD_DLASTSGA_14: u32,
    pub TCD_CSR_14: u16,
    pub TCD_BITER_ELINKNO_14: u16,
    pub TCD_SADDR_15: u32,
    pub TCD_SOFF_15: u16,
    pub TCD_ATTR_15: u16,
    pub TCD_NBYTES_MLNO_15: u32,
    pub TCD_SLAST_15: u32,
    pub TCD_DADDR_15: u32,
    pub TCD_DOFF_15: u16,
    pub TCD_CITER_ELINKNO_15: u16,
    pub TCD_DLASTSGA_15: u32,
    pub TCD_CSR_15: u16,
    pub TCD_BITER_ELINKNO_15: u16,
    pub TCD_SADDR_16: u32,
    pub TCD_SOFF_16: u16,
    pub TCD_ATTR_16: u16,
    pub TCD_NBYTES_MLNO_16: u32,
    pub TCD_SLAST_16: u32,
    pub TCD_DADDR_16: u32,
    pub TCD_DOFF_16: u16,
    pub TCD_CITER_ELINKNO_16: u16,
    pub TCD_DLASTSGA_16: u32,
    pub TCD_CSR_16: u16,
    pub TCD_BITER_ELINKNO_16: u16,
    pub TCD_SADDR_17: u32,
    pub TCD_SOFF_17: u16,
    pub TCD_ATTR_17: u16,
    pub TCD_NBYTES_MLNO_17: u32,
    pub TCD_SLAST_17: u32,
    pub TCD_DADDR_17: u32,
    pub TCD_DOFF_17: u16,
    pub TCD_CITER_ELINKNO_17: u16,
    pub TCD_DLASTSGA_17: u32,
    pub TCD_CSR_17: u16,
    pub TCD_BITER_ELINKNO_17: u16,
    pub TCD_SADDR_18: u32,
    pub TCD_SOFF_18: u16,
    pub TCD_ATTR_18: u16,
    pub TCD_NBYTES_MLNO_18: u32,
    pub TCD_SLAST_18: u32,
    pub TCD_DADDR_18: u32,
    pub TCD_DOFF_18: u16,
    pub TCD_CITER_ELINKNO_18: u16,
    pub TCD_DLASTSGA_18: u32,
    pub TCD_CSR_18: u16,
    pub TCD_BITER_ELINKNO_18: u16,
    pub TCD_SADDR_19: u32,
    pub TCD_SOFF_19: u16,
    pub TCD_ATTR_19: u16,
    pub TCD_NBYTES_MLNO_19: u32,
    pub TCD_SLAST_19: u32,
    pub TCD_DADDR_19: u32,
    pub TCD_DOFF_19: u16,
    pub TCD_CITER_ELINKNO_19: u16,
    pub TCD_DLASTSGA_19: u32,
    pub TCD_CSR_19: u16,
    pub TCD_BITER_ELINKNO_19: u16,
    pub TCD_SADDR_20: u32,
    pub TCD_SOFF_20: u16,
    pub TCD_ATTR_20: u16,
    pub TCD_NBYTES_MLNO_20: u32,
    pub TCD_SLAST_20: u32,
    pub TCD_DADDR_20: u32,
    pub TCD_DOFF_20: u16,
    pub TCD_CITER_ELINKNO_20: u16,
    pub TCD_DLASTSGA_20: u32,
    pub TCD_CSR_20: u16,
    pub TCD_BITER_ELINKNO_20: u16,
    pub TCD_SADDR_21: u32,
    pub TCD_SOFF_21: u16,
    pub TCD_ATTR_21: u16,
    pub TCD_NBYTES_MLNO_21: u32,
    pub TCD_SLAST_21: u32,
    pub TCD_DADDR_21: u32,
    pub TCD_DOFF_21: u16,
    pub TCD_CITER_ELINKNO_21: u16,
    pub TCD_DLASTSGA_21: u32,
    pub TCD_CSR_21: u16,
    pub TCD_BITER_ELINKNO_21: u16,
    pub TCD_SADDR_22: u32,
    pub TCD_SOFF_22: u16,
    pub TCD_ATTR_22: u16,
    pub TCD_NBYTES_MLNO_22: u32,
    pub TCD_SLAST_22: u32,
    pub TCD_DADDR_22: u32,
    pub TCD_DOFF_22: u16,
    pub TCD_CITER_ELINKNO_22: u16,
    pub TCD_DLASTSGA_22: u32,
    pub TCD_CSR_22: u16,
    pub TCD_BITER_ELINKNO_22: u16,
    pub TCD_SADDR_23: u32,
    pub TCD_SOFF_23: u16,
    pub TCD_ATTR_23: u16,
    pub TCD_NBYTES_MLNO_23: u32,
    pub TCD_SLAST_23: u32,
    pub TCD_DADDR_23: u32,
    pub TCD_DOFF_23: u16,
    pub TCD_CITER_ELINKNO_23: u16,
    pub TCD_DLASTSGA_23: u32,
    pub TCD_CSR_23: u16,
    pub TCD_BITER_ELINKNO_23: u16,
    pub TCD_SADDR_24: u32,
    pub TCD_SOFF_24: u16,
    pub TCD_ATTR_24: u16,
    pub TCD_NBYTES_MLNO_24: u32,
    pub TCD_SLAST_24: u32,
    pub TCD_DADDR_24: u32,
    pub TCD_DOFF_24: u16,
    pub TCD_CITER_ELINKNO_24: u16,
    pub TCD_DLASTSGA_24: u32,
    pub TCD_CSR_24: u16,
    pub TCD_BITER_ELINKNO_24: u16,
    pub TCD_SADDR_25: u32,
    pub TCD_SOFF_25: u16,
    pub TCD_ATTR_25: u16,
    pub TCD_NBYTES_MLNO_25: u32,
    pub TCD_SLAST_25: u32,
    pub TCD_DADDR_25: u32,
    pub TCD_DOFF_25: u16,
    pub TCD_CITER_ELINKNO_25: u16,
    pub TCD_DLASTSGA_25: u32,
    pub TCD_CSR_25: u16,
    pub TCD_BITER_ELINKNO_25: u16,
    pub TCD_SADDR_26: u32,
    pub TCD_SOFF_26: u16,
    pub TCD_ATTR_26: u16,
    pub TCD_NBYTES_MLNO_26: u32,
    pub TCD_SLAST_26: u32,
    pub TCD_DADDR_26: u32,
    pub TCD_DOFF_26: u16,
    pub TCD_CITER_ELINKNO_26: u16,
    pub TCD_DLASTSGA_26: u32,
    pub TCD_CSR_26: u16,
    pub TCD_BITER_ELINKNO_26: u16,
    pub TCD_SADDR_27: u32,
    pub TCD_SOFF_27: u16,
    pub TCD_ATTR_27: u16,
    pub TCD_NBYTES_MLNO_27: u32,
    pub TCD_SLAST_27: u32,
    pub TCD_DADDR_27: u32,
    pub TCD_DOFF_27: u16,
    pub TCD_CITER_ELINKNO_27: u16,
    pub TCD_DLASTSGA_27: u32,
    pub TCD_CSR_27: u16,
    pub TCD_BITER_ELINKNO_27: u16,
    pub TCD_SADDR_28: u32,
    pub TCD_SOFF_28: u16,
    pub TCD_ATTR_28: u16,
    pub TCD_NBYTES_MLNO_28: u32,
    pub TCD_SLAST_28: u32,
    pub TCD_DADDR_28: u32,
    pub TCD_DOFF_28: u16,
    pub TCD_CITER_ELINKNO_28: u16,
    pub TCD_DLASTSGA_28: u32,
    pub TCD_CSR_28: u16,
    pub TCD_BITER_ELINKNO_28: u16,
    pub TCD_SADDR_29: u32,
    pub TCD_SOFF_29: u16,
    pub TCD_ATTR_29: u16,
    pub TCD_NBYTES_MLNO_29: u32,
    pub TCD_SLAST_29: u32,
    pub TCD_DADDR_29: u32,
    pub TCD_DOFF_29: u16,
    pub TCD_CITER_ELINKNO_29: u16,
    pub TCD_DLASTSGA_29: u32,
    pub TCD_CSR_29: u16,
    pub TCD_BITER_ELINKNO_29: u16,
    pub TCD_SADDR_30: u32,
    pub TCD_SOFF_30: u16,
    pub TCD_ATTR_30: u16,
    pub TCD_NBYTES_MLNO_30: u32,
    pub TCD_SLAST_30: u32,
    pub TCD_DADDR_30: u32,
    pub TCD_DOFF_30: u16,
    pub TCD_CITER_ELINKNO_30: u16,
    pub TCD_DLASTSGA_30: u32,
    pub TCD_CSR_30: u16,
    pub TCD_BITER_ELINKNO_30: u16,
    pub TCD_SADDR_31: u32,
    pub TCD_SOFF_31: u16,
    pub TCD_ATTR_31: u16,
    pub TCD_NBYTES_MLNO_31: u32,
    pub TCD_SLAST_31: u32,
    pub TCD_DADDR_31: u32,
    pub TCD_DOFF_31: u16,
    pub TCD_CITER_ELINKNO_31: u16,
    pub TCD_DLASTSGA_31: u32,
    pub TCD_CSR_31: u16,
    pub TCD_BITER_ELINKNO_31: u16,
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
