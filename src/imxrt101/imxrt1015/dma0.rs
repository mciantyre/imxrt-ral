#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA

use crate::{RORegister, RWRegister};

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// Control Register
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

            /// 0b0: When in debug mode, the DMA continues to operate.
            pub const EDBG_0: u32 = 0b0;

            /// 0b1: When in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete. Channel execution resumes when the system exits debug mode or the EDBG bit is cleared.
            pub const EDBG_1: u32 = 0b1;
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

            /// 0b0: Fixed priority arbitration is used for channel selection within each group.
            pub const ERCA_0: u32 = 0b0;

            /// 0b1: Round robin arbitration is used for channel selection within each group.
            pub const ERCA_1: u32 = 0b1;
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

            /// 0b0: Fixed priority arbitration is used for selection among the groups.
            pub const ERGA_0: u32 = 0b0;

            /// 0b1: Round robin arbitration is used for selection among the groups.
            pub const ERGA_1: u32 = 0b1;
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
            pub const HOE_0: u32 = 0b0;

            /// 0b1: Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared.
            pub const HOE_1: u32 = 0b1;
        }
    }

    /// Halt DMA Operations
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
            pub const HALT_0: u32 = 0b0;

            /// 0b1: Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared.
            pub const HALT_1: u32 = 0b1;
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

            /// 0b0: A minor loop channel link made to itself goes through channel arbitration before being activated again.
            pub const CLM_0: u32 = 0b0;

            /// 0b1: A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop.
            pub const CLM_1: u32 = 0b1;
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

            /// 0b0: Disabled. TCDn.word2 is defined as a 32-bit NBYTES field.
            pub const EMLM_0: u32 = 0b0;

            /// 0b1: Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled.
            pub const EMLM_1: u32 = 0b1;
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
            pub const ECX_0: u32 = 0b0;

            /// 0b1: Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the Error Status register (DMAx_ES) and generating an optional error interrupt.
            pub const ECX_1: u32 = 0b1;
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const CX_0: u32 = 0b0;

            /// 0b1: Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed.
            pub const CX_1: u32 = 0b1;
        }
    }

    /// DMA Active Status
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

            /// 0b0: eDMA is idle.
            pub const ACTIVE_0: u32 = 0b0;

            /// 0b1: eDMA is executing a channel.
            pub const ACTIVE_1: u32 = 0b1;
        }
    }
}

/// Error Status Register
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

            /// 0b0: No destination bus error
            pub const DBE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a bus error on a destination write
            pub const DBE_1: u32 = 0b1;
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

            /// 0b0: No source bus error
            pub const SBE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a bus error on a source read
            pub const SBE_1: u32 = 0b1;
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

            /// 0b0: No scatter/gather configuration error
            pub const SGE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error detected in the TCDn_DLASTSGA field. This field is checked at the beginning of a scatter/gather operation after major loop completion if TCDn_CSR\[ESG\] is enabled. TCDn_DLASTSGA is not on a 32 byte boundary.
            pub const SGE_1: u32 = 0b1;
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

            /// 0b0: No NBYTES/CITER configuration error
            pub const NCE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields. TCDn_NBYTES is not a multiple of TCDn_ATTR\[SSIZE\] and TCDn_ATTR\[DSIZE\], or TCDn_CITER\[CITER\] is equal to zero, or TCDn_CITER\[ELINK\] is not equal to TCDn_BITER\[ELINK\]
            pub const NCE_1: u32 = 0b1;
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

            /// 0b0: No destination offset configuration error
            pub const DOE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR\[DSIZE\].
            pub const DOE_1: u32 = 0b1;
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

            /// 0b0: No destination address configuration error
            pub const DAE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR\[DSIZE\].
            pub const DAE_1: u32 = 0b1;
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

            /// 0b0: No source offset configuration error
            pub const SOE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR\[SSIZE\].
            pub const SOE_1: u32 = 0b1;
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
            pub const SAE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR\[SSIZE\].
            pub const SAE_1: u32 = 0b1;
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

            /// 0b0: No channel priority error
            pub const CPE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error in the channel priorities within a group. Channel priorities within a group are not unique.
            pub const CPE_1: u32 = 0b1;
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

            /// 0b0: No group priority error
            pub const GPE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error among the group priorities. All group priorities are not unique.
            pub const GPE_1: u32 = 0b1;
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
            pub const ECX_0: u32 = 0b0;

            /// 0b1: The last recorded entry was a canceled transfer by the error cancel transfer input
            pub const ECX_1: u32 = 0b1;
        }
    }

    /// VLD
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

            /// 0b0: No ERR bits are set.
            pub const VLD_0: u32 = 0b0;

            /// 0b1: At least one ERR bit is set indicating a valid error exists that has not been cleared.
            pub const VLD_1: u32 = 0b1;
        }
    }
}

/// Enable Request Register
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ0_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ0_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ1_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ1_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ2_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ2_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ3_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ3_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ4_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ4_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ5_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ5_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ6_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ6_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ7_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ7_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ8_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ8_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ9_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ9_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ10_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ10_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ11_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ11_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ12_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ12_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ13_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ13_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ14_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ14_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ15_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ15_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ16_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ16_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ17_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ17_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ18_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ18_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ19_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ19_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ20_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ20_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ21_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ21_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ22_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ22_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ23_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ23_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ24_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ24_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ25_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ25_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ26_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ26_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ27_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ27_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ28_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ28_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ29_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ29_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ30_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ30_1: u32 = 0b1;
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ31_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ31_1: u32 = 0b1;
        }
    }
}

/// Enable Error Interrupt Register
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI0_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI0_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI1_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI1_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI2_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI2_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI3_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI3_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI4_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI4_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI5_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI5_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI6_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI6_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI7_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI7_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI8_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI8_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI9_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI9_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI10_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI10_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI11_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI11_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI12_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI12_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI13_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI13_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI14_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI14_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI15_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI15_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI16_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI16_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI17_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI17_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI18_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI18_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI19_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI19_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI20_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI20_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI21_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI21_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI22_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI22_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI23_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI23_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI24_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI24_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI25_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI25_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI26_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI26_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI27_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI27_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI28_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI28_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI29_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI29_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI30_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI30_1: u32 = 0b1;
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI31_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI31_1: u32 = 0b1;
        }
    }
}

/// Clear Enable Error Interrupt Register
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

            /// 0b0: Clear only the EEI bit specified in the CEEI field
            pub const CAEE_0: u8 = 0b0;

            /// 0b1: Clear all bits in EEI
            pub const CAEE_1: u8 = 0b1;
        }
    }

    /// No Op enable
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
            pub const NOP_0: u8 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u8 = 0b1;
        }
    }
}

/// Set Enable Error Interrupt Register
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

    /// Sets All Enable Error Interrupts
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

            /// 0b0: Set only the EEI bit specified in the SEEI field.
            pub const SAEE_0: u8 = 0b0;

            /// 0b1: Sets all bits in EEI
            pub const SAEE_1: u8 = 0b1;
        }
    }

    /// No Op enable
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
            pub const NOP_0: u8 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u8 = 0b1;
        }
    }
}

/// Clear Enable Request Register
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

            /// 0b0: Clear only the ERQ bit specified in the CERQ field
            pub const CAER_0: u8 = 0b0;

            /// 0b1: Clear all bits in ERQ
            pub const CAER_1: u8 = 0b1;
        }
    }

    /// No Op enable
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
            pub const NOP_0: u8 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u8 = 0b1;
        }
    }
}

/// Set Enable Request Register
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

            /// 0b0: Set only the ERQ bit specified in the SERQ field
            pub const SAER_0: u8 = 0b0;

            /// 0b1: Set all bits in ERQ
            pub const SAER_1: u8 = 0b1;
        }
    }

    /// No Op enable
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
            pub const NOP_0: u8 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u8 = 0b1;
        }
    }
}

/// Clear DONE Status Bit Register
pub mod CDNE {

    /// Clear DONE Bit
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

    /// Clears All DONE Bits
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

            /// 0b0: Clears only the TCDn_CSR\[DONE\] bit specified in the CDNE field
            pub const CADN_0: u8 = 0b0;

            /// 0b1: Clears all bits in TCDn_CSR\[DONE\]
            pub const CADN_1: u8 = 0b1;
        }
    }

    /// No Op enable
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
            pub const NOP_0: u8 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u8 = 0b1;
        }
    }
}

/// Set START Bit Register
pub mod SSRT {

    /// Set START Bit
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

    /// Set All START Bits (activates all channels)
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

            /// 0b0: Set only the TCDn_CSR\[START\] bit specified in the SSRT field
            pub const SAST_0: u8 = 0b0;

            /// 0b1: Set all bits in TCDn_CSR\[START\]
            pub const SAST_1: u8 = 0b1;
        }
    }

    /// No Op enable
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
            pub const NOP_0: u8 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u8 = 0b1;
        }
    }
}

/// Clear Error Register
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

            /// 0b0: Clear only the ERR bit specified in the CERR field
            pub const CAEI_0: u8 = 0b0;

            /// 0b1: Clear all bits in ERR
            pub const CAEI_1: u8 = 0b1;
        }
    }

    /// No Op enable
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
            pub const NOP_0: u8 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u8 = 0b1;
        }
    }
}

/// Clear Interrupt Request Register
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

            /// 0b0: Clear only the INT bit specified in the CINT field
            pub const CAIR_0: u8 = 0b0;

            /// 0b1: Clear all bits in INT
            pub const CAIR_1: u8 = 0b1;
        }
    }

    /// No Op enable
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
            pub const NOP_0: u8 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u8 = 0b1;
        }
    }
}

/// Interrupt Request Register
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT0_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT0_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT1_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT1_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT2_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT2_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT3_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT3_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT4_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT4_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT5_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT5_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT6_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT6_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT7_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT7_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT8_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT8_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT9_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT9_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT10_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT10_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT11_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT11_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT12_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT12_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT13_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT13_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT14_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT14_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT15_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT15_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT16_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT16_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT17_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT17_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT18_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT18_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT19_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT19_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT20_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT20_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT21_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT21_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT22_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT22_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT23_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT23_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT24_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT24_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT25_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT25_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT26_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT26_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT27_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT27_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT28_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT28_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT29_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT29_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT30_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT30_1: u32 = 0b1;
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT31_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT31_1: u32 = 0b1;
        }
    }
}

/// Error Register
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR0_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR0_1: u32 = 0b1;
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR1_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR1_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR2_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR2_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR3_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR3_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR4_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR4_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR5_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR5_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR6_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR6_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR7_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR7_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR8_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR8_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR9_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR9_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR10_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR10_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR11_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR11_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR12_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR12_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR13_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR13_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR14_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR14_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR15_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR15_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR16_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR16_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR17_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR17_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR18_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR18_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR19_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR19_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR20_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR20_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR21_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR21_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR22_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR22_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR23_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR23_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR24_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR24_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR25_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR25_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR26_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR26_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR27_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR27_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR28_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR28_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR29_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR29_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR30_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR30_1: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: An error in this channel has not occurred
            pub const ERR31_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR31_1: u32 = 0b1;
        }
    }
}

/// Hardware Request Status Register
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
            pub const HRS0_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 0 is present
            pub const HRS0_1: u32 = 0b1;
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
            pub const HRS1_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 1 is present
            pub const HRS1_1: u32 = 0b1;
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
            pub const HRS2_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 2 is present
            pub const HRS2_1: u32 = 0b1;
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
            pub const HRS3_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 3 is present
            pub const HRS3_1: u32 = 0b1;
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
            pub const HRS4_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 4 is present
            pub const HRS4_1: u32 = 0b1;
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
            pub const HRS5_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 5 is present
            pub const HRS5_1: u32 = 0b1;
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
            pub const HRS6_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 6 is present
            pub const HRS6_1: u32 = 0b1;
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
            pub const HRS7_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 7 is present
            pub const HRS7_1: u32 = 0b1;
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
            pub const HRS8_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 8 is present
            pub const HRS8_1: u32 = 0b1;
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
            pub const HRS9_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 9 is present
            pub const HRS9_1: u32 = 0b1;
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
            pub const HRS10_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 10 is present
            pub const HRS10_1: u32 = 0b1;
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
            pub const HRS11_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 11 is present
            pub const HRS11_1: u32 = 0b1;
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
            pub const HRS12_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 12 is present
            pub const HRS12_1: u32 = 0b1;
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
            pub const HRS13_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 13 is present
            pub const HRS13_1: u32 = 0b1;
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
            pub const HRS14_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 14 is present
            pub const HRS14_1: u32 = 0b1;
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
            pub const HRS15_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 15 is present
            pub const HRS15_1: u32 = 0b1;
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
            pub const HRS16_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 16 is present
            pub const HRS16_1: u32 = 0b1;
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
            pub const HRS17_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 17 is present
            pub const HRS17_1: u32 = 0b1;
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
            pub const HRS18_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 18 is present
            pub const HRS18_1: u32 = 0b1;
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
            pub const HRS19_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 19 is present
            pub const HRS19_1: u32 = 0b1;
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
            pub const HRS20_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 20 is present
            pub const HRS20_1: u32 = 0b1;
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
            pub const HRS21_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 21 is present
            pub const HRS21_1: u32 = 0b1;
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
            pub const HRS22_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 22 is present
            pub const HRS22_1: u32 = 0b1;
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
            pub const HRS23_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 23 is present
            pub const HRS23_1: u32 = 0b1;
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
            pub const HRS24_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 24 is present
            pub const HRS24_1: u32 = 0b1;
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
            pub const HRS25_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 25 is present
            pub const HRS25_1: u32 = 0b1;
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
            pub const HRS26_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 26 is present
            pub const HRS26_1: u32 = 0b1;
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
            pub const HRS27_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 27 is present
            pub const HRS27_1: u32 = 0b1;
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
            pub const HRS28_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 28 is present
            pub const HRS28_1: u32 = 0b1;
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
            pub const HRS29_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 29 is present
            pub const HRS29_1: u32 = 0b1;
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
            pub const HRS30_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 30 is present
            pub const HRS30_1: u32 = 0b1;
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
            pub const HRS31_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 31 is present
            pub const HRS31_1: u32 = 0b1;
        }
    }
}

/// Enable Asynchronous Request in Stop Register
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

            /// 0b0: Disable asynchronous DMA request for channel 0.
            pub const EDREQ_0_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 0.
            pub const EDREQ_0_1: u32 = 0b1;
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
            pub const EDREQ_1_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 1.
            pub const EDREQ_1_1: u32 = 0b1;
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

            /// 0b0: Disable asynchronous DMA request for channel 2.
            pub const EDREQ_2_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 2.
            pub const EDREQ_2_1: u32 = 0b1;
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

            /// 0b0: Disable asynchronous DMA request for channel 3.
            pub const EDREQ_3_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 3.
            pub const EDREQ_3_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 4
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

            /// 0b0: Disable asynchronous DMA request for channel 4.
            pub const EDREQ_4_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 4.
            pub const EDREQ_4_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 5
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

            /// 0b0: Disable asynchronous DMA request for channel 5.
            pub const EDREQ_5_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 5.
            pub const EDREQ_5_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 6
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

            /// 0b0: Disable asynchronous DMA request for channel 6.
            pub const EDREQ_6_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 6.
            pub const EDREQ_6_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 7
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

            /// 0b0: Disable asynchronous DMA request for channel 7.
            pub const EDREQ_7_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 7.
            pub const EDREQ_7_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 8
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

            /// 0b0: Disable asynchronous DMA request for channel 8.
            pub const EDREQ_8_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 8.
            pub const EDREQ_8_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 9
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

            /// 0b0: Disable asynchronous DMA request for channel 9.
            pub const EDREQ_9_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 9.
            pub const EDREQ_9_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 10
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

            /// 0b0: Disable asynchronous DMA request for channel 10.
            pub const EDREQ_10_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 10.
            pub const EDREQ_10_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 11
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

            /// 0b0: Disable asynchronous DMA request for channel 11.
            pub const EDREQ_11_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 11.
            pub const EDREQ_11_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 12
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

            /// 0b0: Disable asynchronous DMA request for channel 12.
            pub const EDREQ_12_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 12.
            pub const EDREQ_12_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 13
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

            /// 0b0: Disable asynchronous DMA request for channel 13.
            pub const EDREQ_13_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 13.
            pub const EDREQ_13_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 14
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

            /// 0b0: Disable asynchronous DMA request for channel 14.
            pub const EDREQ_14_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 14.
            pub const EDREQ_14_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 15
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

            /// 0b0: Disable asynchronous DMA request for channel 15.
            pub const EDREQ_15_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 15.
            pub const EDREQ_15_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 16
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
            pub const EDREQ_16_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 16
            pub const EDREQ_16_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 17
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
            pub const EDREQ_17_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 17
            pub const EDREQ_17_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 18
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
            pub const EDREQ_18_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 18
            pub const EDREQ_18_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 19
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
            pub const EDREQ_19_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 19
            pub const EDREQ_19_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 20
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
            pub const EDREQ_20_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 20
            pub const EDREQ_20_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 21
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
            pub const EDREQ_21_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 21
            pub const EDREQ_21_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 22
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
            pub const EDREQ_22_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 22
            pub const EDREQ_22_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 23
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
            pub const EDREQ_23_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 23
            pub const EDREQ_23_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 24
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
            pub const EDREQ_24_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 24
            pub const EDREQ_24_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 25
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
            pub const EDREQ_25_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 25
            pub const EDREQ_25_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 26
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
            pub const EDREQ_26_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 26
            pub const EDREQ_26_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 27
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
            pub const EDREQ_27_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 27
            pub const EDREQ_27_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 28
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
            pub const EDREQ_28_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 28
            pub const EDREQ_28_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 29
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
            pub const EDREQ_29_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 29
            pub const EDREQ_29_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 30
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
            pub const EDREQ_30_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 30
            pub const EDREQ_30_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 31
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
            pub const EDREQ_31_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 31
            pub const EDREQ_31_1: u32 = 0b1;
        }
    }
}

/// Channel Priority Register
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

            /// 0b0: Channel n can suspend a lower priority channel.
            pub const DPA_0: u8 = 0b0;

            /// 0b1: Channel n cannot suspend any channel, regardless of channel priority.
            pub const DPA_1: u8 = 0b1;
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

            /// 0b0: Channel n cannot be suspended by a higher priority channel's service request.
            pub const ECP_0: u8 = 0b0;

            /// 0b1: Channel n can be temporarily suspended by the service request of a higher priority channel.
            pub const ECP_1: u8 = 0b1;
        }
    }
}

/// Channel Priority Register
pub mod DCHPRI2 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI1 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI0 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI7 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI6 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI5 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI4 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI11 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI10 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI9 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI8 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI15 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI14 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI13 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI12 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI19 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI18 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI17 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI16 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI23 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI22 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI21 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI20 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI27 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI26 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI25 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI24 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI31 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI30 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI29 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel Priority Register
pub mod DCHPRI28 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Master ID Register
pub mod DCHMID0 {

    /// Master ID
    pub mod MID {
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

    /// Privileged Access Level
    pub mod PAL {
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

            /// 0b0: User protection level for DMA transfers
            pub const PAL_0: u8 = 0b0;

            /// 0b1: Privileged protection level for DMA transfers
            pub const PAL_1: u8 = 0b1;
        }
    }

    /// Enable Master ID replication
    pub mod EMI {
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

            /// 0b0: Master ID replication is disabled
            pub const EMI_0: u8 = 0b0;

            /// 0b1: Master ID replication is enabled
            pub const EMI_1: u8 = 0b1;
        }
    }
}

/// Channel n Master ID Register
pub mod DCHMID1 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID2 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID3 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID4 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID5 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID6 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID7 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID8 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID9 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID10 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID11 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID12 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID13 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID14 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID15 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID16 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID17 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID18 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID19 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID20 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID21 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID22 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID23 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID24 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID25 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID26 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID27 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID28 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID29 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID30 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
}

/// Channel n Master ID Register
pub mod DCHMID31 {
    pub use super::DCHMID0::EMI;
    pub use super::DCHMID0::MID;
    pub use super::DCHMID0::PAL;
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
            pub const SSIZE_0: u16 = 0b000;

            /// 0b001: 16-bit
            pub const SSIZE_1: u16 = 0b001;

            /// 0b010: 32-bit
            pub const SSIZE_2: u16 = 0b010;

            /// 0b011: 64-bit
            pub const SSIZE_3: u16 = 0b011;

            /// 0b101: 32-byte burst (4 beats of 64 bits)
            pub const SSIZE_5: u16 = 0b101;
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

            /// 0b00000: Source address modulo feature is disabled
            pub const SMOD_0: u16 = 0b00000;

            /// 0b00001: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_1: u16 = 0b00001;

            /// 0b00010: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_2: u16 = 0b00010;

            /// 0b00011: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_3: u16 = 0b00011;

            /// 0b00100: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_4: u16 = 0b00100;

            /// 0b00101: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_5: u16 = 0b00101;

            /// 0b00110: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_6: u16 = 0b00110;

            /// 0b00111: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_7: u16 = 0b00111;

            /// 0b01000: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_8: u16 = 0b01000;

            /// 0b01001: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_9: u16 = 0b01001;
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

    /// Destination Minor Loop Offset enable
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
            pub const DMLOE_0: u32 = 0b0;

            /// 0b1: The minor loop offset is applied to the DADDR
            pub const DMLOE_1: u32 = 0b1;
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
            pub const SMLOE_0: u32 = 0b0;

            /// 0b1: The minor loop offset is applied to the SADDR
            pub const SMLOE_1: u32 = 0b1;
        }
    }

    /// If SMLOE or DMLOE is set, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes.
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

            /// 0b0: The channel-to-channel linking is disabled
            pub const ELINK_0: u16 = 0b0;

            /// 0b1: The channel-to-channel linking is enabled
            pub const ELINK_1: u16 = 0b1;
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

    /// DLASTSGA
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

            /// 0b0: The channel is not explicitly started.
            pub const START_0: u16 = 0b0;

            /// 0b1: The channel is explicitly started via a software initiated service request.
            pub const START_1: u16 = 0b1;
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

            /// 0b0: The end-of-major loop interrupt is disabled.
            pub const INTMAJOR_0: u16 = 0b0;

            /// 0b1: The end-of-major loop interrupt is enabled.
            pub const INTMAJOR_1: u16 = 0b1;
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

            /// 0b0: The half-point interrupt is disabled.
            pub const INTHALF_0: u16 = 0b0;

            /// 0b1: The half-point interrupt is enabled.
            pub const INTHALF_1: u16 = 0b1;
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

            /// 0b0: The channel's ERQ bit is not affected.
            pub const DREQ_0: u16 = 0b0;

            /// 0b1: The channel's ERQ bit is cleared when the major loop is complete.
            pub const DREQ_1: u16 = 0b1;
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

            /// 0b0: The current channel's TCD is normal format.
            pub const ESG_0: u16 = 0b0;

            /// 0b1: The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution.
            pub const ESG_1: u16 = 0b1;
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

            /// 0b0: The channel-to-channel linking is disabled.
            pub const MAJORELINK_0: u16 = 0b0;

            /// 0b1: The channel-to-channel linking is enabled.
            pub const MAJORELINK_1: u16 = 0b1;
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

            /// 0b00: No eDMA engine stalls.
            pub const BWC_0: u16 = 0b00;

            /// 0b10: eDMA engine stalls for 4 cycles after each R/W.
            pub const BWC_2: u16 = 0b10;

            /// 0b11: eDMA engine stalls for 8 cycles after each R/W.
            pub const BWC_3: u16 = 0b11;
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

            /// 0b0: The channel-to-channel linking is disabled
            pub const ELINK_0: u16 = 0b0;

            /// 0b1: The channel-to-channel linking is enabled
            pub const ELINK_1: u16 = 0b1;
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
    /// Control Register
    pub CR: RWRegister<u32>,

    /// Error Status Register
    pub ES: RORegister<u32>,

    _reserved1: [u32; 1],

    /// Enable Request Register
    pub ERQ: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// Enable Error Interrupt Register
    pub EEI: RWRegister<u32>,

    /// Clear Enable Error Interrupt Register
    pub CEEI: RWRegister<u8>,

    /// Set Enable Error Interrupt Register
    pub SEEI: RWRegister<u8>,

    /// Clear Enable Request Register
    pub CERQ: RWRegister<u8>,

    /// Set Enable Request Register
    pub SERQ: RWRegister<u8>,

    /// Clear DONE Status Bit Register
    pub CDNE: RWRegister<u8>,

    /// Set START Bit Register
    pub SSRT: RWRegister<u8>,

    /// Clear Error Register
    pub CERR: RWRegister<u8>,

    /// Clear Interrupt Request Register
    pub CINT: RWRegister<u8>,

    _reserved3: [u32; 1],

    /// Interrupt Request Register
    pub INT: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// Error Register
    pub ERR: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// Hardware Request Status Register
    pub HRS: RORegister<u32>,

    _reserved6: [u32; 3],

    /// Enable Asynchronous Request in Stop Register
    pub EARS: RWRegister<u32>,

    _reserved7: [u32; 46],

    /// Channel Priority Register
    pub DCHPRI3: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI2: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI1: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI0: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI7: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI6: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI5: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI4: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI11: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI10: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI9: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI8: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI15: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI14: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI13: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI12: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI19: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI18: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI17: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI16: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI23: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI22: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI21: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI20: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI27: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI26: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI25: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI24: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI31: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI30: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI29: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI28: RWRegister<u8>,

    _reserved8: [u32; 8],

    /// Channel n Master ID Register
    pub DCHMID0: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID1: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID2: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID3: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID4: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID5: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID6: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID7: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID8: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID9: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID10: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID11: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID12: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID13: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID14: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID15: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID16: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID17: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID18: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID19: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID20: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID21: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID22: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID23: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID24: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID25: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID26: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID27: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID28: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID29: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID30: RWRegister<u8>,

    /// Channel n Master ID Register
    pub DCHMID31: RWRegister<u8>,

    _reserved9: [u32; 936],

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
    pub DCHMID0: u8,
    pub DCHMID1: u8,
    pub DCHMID2: u8,
    pub DCHMID3: u8,
    pub DCHMID4: u8,
    pub DCHMID5: u8,
    pub DCHMID6: u8,
    pub DCHMID7: u8,
    pub DCHMID8: u8,
    pub DCHMID9: u8,
    pub DCHMID10: u8,
    pub DCHMID11: u8,
    pub DCHMID12: u8,
    pub DCHMID13: u8,
    pub DCHMID14: u8,
    pub DCHMID15: u8,
    pub DCHMID16: u8,
    pub DCHMID17: u8,
    pub DCHMID18: u8,
    pub DCHMID19: u8,
    pub DCHMID20: u8,
    pub DCHMID21: u8,
    pub DCHMID22: u8,
    pub DCHMID23: u8,
    pub DCHMID24: u8,
    pub DCHMID25: u8,
    pub DCHMID26: u8,
    pub DCHMID27: u8,
    pub DCHMID28: u8,
    pub DCHMID29: u8,
    pub DCHMID30: u8,
    pub DCHMID31: u8,
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

/// The DMA0 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type DMA0 = Instance<0>;

/// The DMA0 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type DMA0 = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct DMA0 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for DMA0 {}
impl crate::Valid for DMA0 {
    fn take() -> Option<Self> {
        <DMA0>::take()
    }
    fn release(self) {
        <DMA0>::release(self);
    }
    unsafe fn steal() -> Self {
        <DMA0>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static DMA0_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the DMA0 peripheral instance
#[cfg(not(feature = "nosync"))]
impl DMA0 {
    const INSTANCE: Self = Self {
        addr: 0x400e8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::DMA0_DMA16,
            crate::interrupt::DMA1_DMA17,
            crate::interrupt::DMA2_DMA18,
            crate::interrupt::DMA3_DMA19,
            crate::interrupt::DMA4_DMA20,
            crate::interrupt::DMA5_DMA21,
            crate::interrupt::DMA6_DMA22,
            crate::interrupt::DMA7_DMA23,
            crate::interrupt::DMA8_DMA24,
            crate::interrupt::DMA9_DMA25,
            crate::interrupt::DMA10_DMA26,
            crate::interrupt::DMA11_DMA27,
            crate::interrupt::DMA12_DMA28,
            crate::interrupt::DMA13_DMA29,
            crate::interrupt::DMA14_DMA30,
            crate::interrupt::DMA15_DMA31,
            crate::interrupt::DMA_ERROR,
        ],
    };

    /// Reset values for each field in DMA0
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000400,
        ES: 0x00000000,
        ERQ: 0x00000000,
        EEI: 0x00000000,
        CEEI: 0x00000000,
        SEEI: 0x00000000,
        CERQ: 0x00000000,
        SERQ: 0x00000000,
        CDNE: 0x00000000,
        SSRT: 0x00000000,
        CERR: 0x00000000,
        CINT: 0x00000000,
        INT: 0x00000000,
        ERR: 0x00000000,
        HRS: 0x00000000,
        EARS: 0x00000000,
        DCHPRI3: 0x00000003,
        DCHPRI2: 0x00000002,
        DCHPRI1: 0x00000001,
        DCHPRI0: 0x00000000,
        DCHPRI7: 0x00000007,
        DCHPRI6: 0x00000006,
        DCHPRI5: 0x00000005,
        DCHPRI4: 0x00000004,
        DCHPRI11: 0x0000000B,
        DCHPRI10: 0x0000000A,
        DCHPRI9: 0x00000009,
        DCHPRI8: 0x00000008,
        DCHPRI15: 0x0000000F,
        DCHPRI14: 0x0000000E,
        DCHPRI13: 0x0000000D,
        DCHPRI12: 0x0000000C,
        DCHPRI19: 0x00000013,
        DCHPRI18: 0x00000012,
        DCHPRI17: 0x00000011,
        DCHPRI16: 0x00000010,
        DCHPRI23: 0x00000017,
        DCHPRI22: 0x00000016,
        DCHPRI21: 0x00000015,
        DCHPRI20: 0x00000014,
        DCHPRI27: 0x0000001B,
        DCHPRI26: 0x0000001A,
        DCHPRI25: 0x00000019,
        DCHPRI24: 0x00000018,
        DCHPRI31: 0x0000001F,
        DCHPRI30: 0x0000001E,
        DCHPRI29: 0x0000001D,
        DCHPRI28: 0x0000001C,
        DCHMID0: 0x00000000,
        DCHMID1: 0x00000000,
        DCHMID2: 0x00000000,
        DCHMID3: 0x00000000,
        DCHMID4: 0x00000000,
        DCHMID5: 0x00000000,
        DCHMID6: 0x00000000,
        DCHMID7: 0x00000000,
        DCHMID8: 0x00000000,
        DCHMID9: 0x00000000,
        DCHMID10: 0x00000000,
        DCHMID11: 0x00000000,
        DCHMID12: 0x00000000,
        DCHMID13: 0x00000000,
        DCHMID14: 0x00000000,
        DCHMID15: 0x00000000,
        DCHMID16: 0x00000000,
        DCHMID17: 0x00000000,
        DCHMID18: 0x00000000,
        DCHMID19: 0x00000000,
        DCHMID20: 0x00000000,
        DCHMID21: 0x00000000,
        DCHMID22: 0x00000000,
        DCHMID23: 0x00000000,
        DCHMID24: 0x00000000,
        DCHMID25: 0x00000000,
        DCHMID26: 0x00000000,
        DCHMID27: 0x00000000,
        DCHMID28: 0x00000000,
        DCHMID29: 0x00000000,
        DCHMID30: 0x00000000,
        DCHMID31: 0x00000000,
        TCD_SADDR_0: 0x00000000,
        TCD_SOFF_0: 0x00000000,
        TCD_ATTR_0: 0x00000000,
        TCD_NBYTES_ML: 0x00000000,
        TCD_SLAST_0: 0x00000000,
        TCD_DADDR_0: 0x00000000,
        TCD_DOFF_0: 0x00000000,
        TCD_CITER_ELINK: 0x00000000,
        TCD_DLASTSGA_0: 0x00000000,
        TCD_CSR_0: 0x00000000,
        TCD_BITER_ELINK: 0x00000000,
        TCD_SADDR_1: 0x00000000,
        TCD_SOFF_1: 0x00000000,
        TCD_ATTR_1: 0x00000000,
        TCD_NBYTES_MLNO_1: 0x00000000,
        TCD_SLAST_1: 0x00000000,
        TCD_DADDR_1: 0x00000000,
        TCD_DOFF_1: 0x00000000,
        TCD_CITER_ELINKNO_1: 0x00000000,
        TCD_DLASTSGA_1: 0x00000000,
        TCD_CSR_1: 0x00000000,
        TCD_BITER_ELINKNO_1: 0x00000000,
        TCD_SADDR_2: 0x00000000,
        TCD_SOFF_2: 0x00000000,
        TCD_ATTR_2: 0x00000000,
        TCD_NBYTES_MLNO_2: 0x00000000,
        TCD_SLAST_2: 0x00000000,
        TCD_DADDR_2: 0x00000000,
        TCD_DOFF_2: 0x00000000,
        TCD_CITER_ELINKNO_2: 0x00000000,
        TCD_DLASTSGA_2: 0x00000000,
        TCD_CSR_2: 0x00000000,
        TCD_BITER_ELINKNO_2: 0x00000000,
        TCD_SADDR_3: 0x00000000,
        TCD_SOFF_3: 0x00000000,
        TCD_ATTR_3: 0x00000000,
        TCD_NBYTES_MLNO_3: 0x00000000,
        TCD_SLAST_3: 0x00000000,
        TCD_DADDR_3: 0x00000000,
        TCD_DOFF_3: 0x00000000,
        TCD_CITER_ELINKNO_3: 0x00000000,
        TCD_DLASTSGA_3: 0x00000000,
        TCD_CSR_3: 0x00000000,
        TCD_BITER_ELINKNO_3: 0x00000000,
        TCD_SADDR_4: 0x00000000,
        TCD_SOFF_4: 0x00000000,
        TCD_ATTR_4: 0x00000000,
        TCD_NBYTES_MLNO_4: 0x00000000,
        TCD_SLAST_4: 0x00000000,
        TCD_DADDR_4: 0x00000000,
        TCD_DOFF_4: 0x00000000,
        TCD_CITER_ELINKNO_4: 0x00000000,
        TCD_DLASTSGA_4: 0x00000000,
        TCD_CSR_4: 0x00000000,
        TCD_BITER_ELINKNO_4: 0x00000000,
        TCD_SADDR_5: 0x00000000,
        TCD_SOFF_5: 0x00000000,
        TCD_ATTR_5: 0x00000000,
        TCD_NBYTES_MLNO_5: 0x00000000,
        TCD_SLAST_5: 0x00000000,
        TCD_DADDR_5: 0x00000000,
        TCD_DOFF_5: 0x00000000,
        TCD_CITER_ELINKNO_5: 0x00000000,
        TCD_DLASTSGA_5: 0x00000000,
        TCD_CSR_5: 0x00000000,
        TCD_BITER_ELINKNO_5: 0x00000000,
        TCD_SADDR_6: 0x00000000,
        TCD_SOFF_6: 0x00000000,
        TCD_ATTR_6: 0x00000000,
        TCD_NBYTES_MLNO_6: 0x00000000,
        TCD_SLAST_6: 0x00000000,
        TCD_DADDR_6: 0x00000000,
        TCD_DOFF_6: 0x00000000,
        TCD_CITER_ELINKNO_6: 0x00000000,
        TCD_DLASTSGA_6: 0x00000000,
        TCD_CSR_6: 0x00000000,
        TCD_BITER_ELINKNO_6: 0x00000000,
        TCD_SADDR_7: 0x00000000,
        TCD_SOFF_7: 0x00000000,
        TCD_ATTR_7: 0x00000000,
        TCD_NBYTES_MLNO_7: 0x00000000,
        TCD_SLAST_7: 0x00000000,
        TCD_DADDR_7: 0x00000000,
        TCD_DOFF_7: 0x00000000,
        TCD_CITER_ELINKNO_7: 0x00000000,
        TCD_DLASTSGA_7: 0x00000000,
        TCD_CSR_7: 0x00000000,
        TCD_BITER_ELINKNO_7: 0x00000000,
        TCD_SADDR_8: 0x00000000,
        TCD_SOFF_8: 0x00000000,
        TCD_ATTR_8: 0x00000000,
        TCD_NBYTES_MLNO_8: 0x00000000,
        TCD_SLAST_8: 0x00000000,
        TCD_DADDR_8: 0x00000000,
        TCD_DOFF_8: 0x00000000,
        TCD_CITER_ELINKNO_8: 0x00000000,
        TCD_DLASTSGA_8: 0x00000000,
        TCD_CSR_8: 0x00000000,
        TCD_BITER_ELINKNO_8: 0x00000000,
        TCD_SADDR_9: 0x00000000,
        TCD_SOFF_9: 0x00000000,
        TCD_ATTR_9: 0x00000000,
        TCD_NBYTES_MLNO_9: 0x00000000,
        TCD_SLAST_9: 0x00000000,
        TCD_DADDR_9: 0x00000000,
        TCD_DOFF_9: 0x00000000,
        TCD_CITER_ELINKNO_9: 0x00000000,
        TCD_DLASTSGA_9: 0x00000000,
        TCD_CSR_9: 0x00000000,
        TCD_BITER_ELINKNO_9: 0x00000000,
        TCD_SADDR_10: 0x00000000,
        TCD_SOFF_10: 0x00000000,
        TCD_ATTR_10: 0x00000000,
        TCD_NBYTES_MLNO_10: 0x00000000,
        TCD_SLAST_10: 0x00000000,
        TCD_DADDR_10: 0x00000000,
        TCD_DOFF_10: 0x00000000,
        TCD_CITER_ELINKNO_10: 0x00000000,
        TCD_DLASTSGA_10: 0x00000000,
        TCD_CSR_10: 0x00000000,
        TCD_BITER_ELINKNO_10: 0x00000000,
        TCD_SADDR_11: 0x00000000,
        TCD_SOFF_11: 0x00000000,
        TCD_ATTR_11: 0x00000000,
        TCD_NBYTES_MLNO_11: 0x00000000,
        TCD_SLAST_11: 0x00000000,
        TCD_DADDR_11: 0x00000000,
        TCD_DOFF_11: 0x00000000,
        TCD_CITER_ELINKNO_11: 0x00000000,
        TCD_DLASTSGA_11: 0x00000000,
        TCD_CSR_11: 0x00000000,
        TCD_BITER_ELINKNO_11: 0x00000000,
        TCD_SADDR_12: 0x00000000,
        TCD_SOFF_12: 0x00000000,
        TCD_ATTR_12: 0x00000000,
        TCD_NBYTES_MLNO_12: 0x00000000,
        TCD_SLAST_12: 0x00000000,
        TCD_DADDR_12: 0x00000000,
        TCD_DOFF_12: 0x00000000,
        TCD_CITER_ELINKNO_12: 0x00000000,
        TCD_DLASTSGA_12: 0x00000000,
        TCD_CSR_12: 0x00000000,
        TCD_BITER_ELINKNO_12: 0x00000000,
        TCD_SADDR_13: 0x00000000,
        TCD_SOFF_13: 0x00000000,
        TCD_ATTR_13: 0x00000000,
        TCD_NBYTES_MLNO_13: 0x00000000,
        TCD_SLAST_13: 0x00000000,
        TCD_DADDR_13: 0x00000000,
        TCD_DOFF_13: 0x00000000,
        TCD_CITER_ELINKNO_13: 0x00000000,
        TCD_DLASTSGA_13: 0x00000000,
        TCD_CSR_13: 0x00000000,
        TCD_BITER_ELINKNO_13: 0x00000000,
        TCD_SADDR_14: 0x00000000,
        TCD_SOFF_14: 0x00000000,
        TCD_ATTR_14: 0x00000000,
        TCD_NBYTES_MLNO_14: 0x00000000,
        TCD_SLAST_14: 0x00000000,
        TCD_DADDR_14: 0x00000000,
        TCD_DOFF_14: 0x00000000,
        TCD_CITER_ELINKNO_14: 0x00000000,
        TCD_DLASTSGA_14: 0x00000000,
        TCD_CSR_14: 0x00000000,
        TCD_BITER_ELINKNO_14: 0x00000000,
        TCD_SADDR_15: 0x00000000,
        TCD_SOFF_15: 0x00000000,
        TCD_ATTR_15: 0x00000000,
        TCD_NBYTES_MLNO_15: 0x00000000,
        TCD_SLAST_15: 0x00000000,
        TCD_DADDR_15: 0x00000000,
        TCD_DOFF_15: 0x00000000,
        TCD_CITER_ELINKNO_15: 0x00000000,
        TCD_DLASTSGA_15: 0x00000000,
        TCD_CSR_15: 0x00000000,
        TCD_BITER_ELINKNO_15: 0x00000000,
        TCD_SADDR_16: 0x00000000,
        TCD_SOFF_16: 0x00000000,
        TCD_ATTR_16: 0x00000000,
        TCD_NBYTES_MLNO_16: 0x00000000,
        TCD_SLAST_16: 0x00000000,
        TCD_DADDR_16: 0x00000000,
        TCD_DOFF_16: 0x00000000,
        TCD_CITER_ELINKNO_16: 0x00000000,
        TCD_DLASTSGA_16: 0x00000000,
        TCD_CSR_16: 0x00000000,
        TCD_BITER_ELINKNO_16: 0x00000000,
        TCD_SADDR_17: 0x00000000,
        TCD_SOFF_17: 0x00000000,
        TCD_ATTR_17: 0x00000000,
        TCD_NBYTES_MLNO_17: 0x00000000,
        TCD_SLAST_17: 0x00000000,
        TCD_DADDR_17: 0x00000000,
        TCD_DOFF_17: 0x00000000,
        TCD_CITER_ELINKNO_17: 0x00000000,
        TCD_DLASTSGA_17: 0x00000000,
        TCD_CSR_17: 0x00000000,
        TCD_BITER_ELINKNO_17: 0x00000000,
        TCD_SADDR_18: 0x00000000,
        TCD_SOFF_18: 0x00000000,
        TCD_ATTR_18: 0x00000000,
        TCD_NBYTES_MLNO_18: 0x00000000,
        TCD_SLAST_18: 0x00000000,
        TCD_DADDR_18: 0x00000000,
        TCD_DOFF_18: 0x00000000,
        TCD_CITER_ELINKNO_18: 0x00000000,
        TCD_DLASTSGA_18: 0x00000000,
        TCD_CSR_18: 0x00000000,
        TCD_BITER_ELINKNO_18: 0x00000000,
        TCD_SADDR_19: 0x00000000,
        TCD_SOFF_19: 0x00000000,
        TCD_ATTR_19: 0x00000000,
        TCD_NBYTES_MLNO_19: 0x00000000,
        TCD_SLAST_19: 0x00000000,
        TCD_DADDR_19: 0x00000000,
        TCD_DOFF_19: 0x00000000,
        TCD_CITER_ELINKNO_19: 0x00000000,
        TCD_DLASTSGA_19: 0x00000000,
        TCD_CSR_19: 0x00000000,
        TCD_BITER_ELINKNO_19: 0x00000000,
        TCD_SADDR_20: 0x00000000,
        TCD_SOFF_20: 0x00000000,
        TCD_ATTR_20: 0x00000000,
        TCD_NBYTES_MLNO_20: 0x00000000,
        TCD_SLAST_20: 0x00000000,
        TCD_DADDR_20: 0x00000000,
        TCD_DOFF_20: 0x00000000,
        TCD_CITER_ELINKNO_20: 0x00000000,
        TCD_DLASTSGA_20: 0x00000000,
        TCD_CSR_20: 0x00000000,
        TCD_BITER_ELINKNO_20: 0x00000000,
        TCD_SADDR_21: 0x00000000,
        TCD_SOFF_21: 0x00000000,
        TCD_ATTR_21: 0x00000000,
        TCD_NBYTES_MLNO_21: 0x00000000,
        TCD_SLAST_21: 0x00000000,
        TCD_DADDR_21: 0x00000000,
        TCD_DOFF_21: 0x00000000,
        TCD_CITER_ELINKNO_21: 0x00000000,
        TCD_DLASTSGA_21: 0x00000000,
        TCD_CSR_21: 0x00000000,
        TCD_BITER_ELINKNO_21: 0x00000000,
        TCD_SADDR_22: 0x00000000,
        TCD_SOFF_22: 0x00000000,
        TCD_ATTR_22: 0x00000000,
        TCD_NBYTES_MLNO_22: 0x00000000,
        TCD_SLAST_22: 0x00000000,
        TCD_DADDR_22: 0x00000000,
        TCD_DOFF_22: 0x00000000,
        TCD_CITER_ELINKNO_22: 0x00000000,
        TCD_DLASTSGA_22: 0x00000000,
        TCD_CSR_22: 0x00000000,
        TCD_BITER_ELINKNO_22: 0x00000000,
        TCD_SADDR_23: 0x00000000,
        TCD_SOFF_23: 0x00000000,
        TCD_ATTR_23: 0x00000000,
        TCD_NBYTES_MLNO_23: 0x00000000,
        TCD_SLAST_23: 0x00000000,
        TCD_DADDR_23: 0x00000000,
        TCD_DOFF_23: 0x00000000,
        TCD_CITER_ELINKNO_23: 0x00000000,
        TCD_DLASTSGA_23: 0x00000000,
        TCD_CSR_23: 0x00000000,
        TCD_BITER_ELINKNO_23: 0x00000000,
        TCD_SADDR_24: 0x00000000,
        TCD_SOFF_24: 0x00000000,
        TCD_ATTR_24: 0x00000000,
        TCD_NBYTES_MLNO_24: 0x00000000,
        TCD_SLAST_24: 0x00000000,
        TCD_DADDR_24: 0x00000000,
        TCD_DOFF_24: 0x00000000,
        TCD_CITER_ELINKNO_24: 0x00000000,
        TCD_DLASTSGA_24: 0x00000000,
        TCD_CSR_24: 0x00000000,
        TCD_BITER_ELINKNO_24: 0x00000000,
        TCD_SADDR_25: 0x00000000,
        TCD_SOFF_25: 0x00000000,
        TCD_ATTR_25: 0x00000000,
        TCD_NBYTES_MLNO_25: 0x00000000,
        TCD_SLAST_25: 0x00000000,
        TCD_DADDR_25: 0x00000000,
        TCD_DOFF_25: 0x00000000,
        TCD_CITER_ELINKNO_25: 0x00000000,
        TCD_DLASTSGA_25: 0x00000000,
        TCD_CSR_25: 0x00000000,
        TCD_BITER_ELINKNO_25: 0x00000000,
        TCD_SADDR_26: 0x00000000,
        TCD_SOFF_26: 0x00000000,
        TCD_ATTR_26: 0x00000000,
        TCD_NBYTES_MLNO_26: 0x00000000,
        TCD_SLAST_26: 0x00000000,
        TCD_DADDR_26: 0x00000000,
        TCD_DOFF_26: 0x00000000,
        TCD_CITER_ELINKNO_26: 0x00000000,
        TCD_DLASTSGA_26: 0x00000000,
        TCD_CSR_26: 0x00000000,
        TCD_BITER_ELINKNO_26: 0x00000000,
        TCD_SADDR_27: 0x00000000,
        TCD_SOFF_27: 0x00000000,
        TCD_ATTR_27: 0x00000000,
        TCD_NBYTES_MLNO_27: 0x00000000,
        TCD_SLAST_27: 0x00000000,
        TCD_DADDR_27: 0x00000000,
        TCD_DOFF_27: 0x00000000,
        TCD_CITER_ELINKNO_27: 0x00000000,
        TCD_DLASTSGA_27: 0x00000000,
        TCD_CSR_27: 0x00000000,
        TCD_BITER_ELINKNO_27: 0x00000000,
        TCD_SADDR_28: 0x00000000,
        TCD_SOFF_28: 0x00000000,
        TCD_ATTR_28: 0x00000000,
        TCD_NBYTES_MLNO_28: 0x00000000,
        TCD_SLAST_28: 0x00000000,
        TCD_DADDR_28: 0x00000000,
        TCD_DOFF_28: 0x00000000,
        TCD_CITER_ELINKNO_28: 0x00000000,
        TCD_DLASTSGA_28: 0x00000000,
        TCD_CSR_28: 0x00000000,
        TCD_BITER_ELINKNO_28: 0x00000000,
        TCD_SADDR_29: 0x00000000,
        TCD_SOFF_29: 0x00000000,
        TCD_ATTR_29: 0x00000000,
        TCD_NBYTES_MLNO_29: 0x00000000,
        TCD_SLAST_29: 0x00000000,
        TCD_DADDR_29: 0x00000000,
        TCD_DOFF_29: 0x00000000,
        TCD_CITER_ELINKNO_29: 0x00000000,
        TCD_DLASTSGA_29: 0x00000000,
        TCD_CSR_29: 0x00000000,
        TCD_BITER_ELINKNO_29: 0x00000000,
        TCD_SADDR_30: 0x00000000,
        TCD_SOFF_30: 0x00000000,
        TCD_ATTR_30: 0x00000000,
        TCD_NBYTES_MLNO_30: 0x00000000,
        TCD_SLAST_30: 0x00000000,
        TCD_DADDR_30: 0x00000000,
        TCD_DOFF_30: 0x00000000,
        TCD_CITER_ELINKNO_30: 0x00000000,
        TCD_DLASTSGA_30: 0x00000000,
        TCD_CSR_30: 0x00000000,
        TCD_BITER_ELINKNO_30: 0x00000000,
        TCD_SADDR_31: 0x00000000,
        TCD_SOFF_31: 0x00000000,
        TCD_ATTR_31: 0x00000000,
        TCD_NBYTES_MLNO_31: 0x00000000,
        TCD_SLAST_31: 0x00000000,
        TCD_DADDR_31: 0x00000000,
        TCD_DOFF_31: 0x00000000,
        TCD_CITER_ELINKNO_31: 0x00000000,
        TCD_DLASTSGA_31: 0x00000000,
        TCD_CSR_31: 0x00000000,
        TCD_BITER_ELINKNO_31: 0x00000000,
    };

    /// Safe access to DMA0
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Self> {
        let taken = DMA0_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to DMA0
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = DMA0_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal DMA0
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        DMA0_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl DMA0 {
    /// The interrupts associated with DMA0
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 17] = [
        crate::interrupt::DMA0_DMA16,
        crate::interrupt::DMA1_DMA17,
        crate::interrupt::DMA2_DMA18,
        crate::interrupt::DMA3_DMA19,
        crate::interrupt::DMA4_DMA20,
        crate::interrupt::DMA5_DMA21,
        crate::interrupt::DMA6_DMA22,
        crate::interrupt::DMA7_DMA23,
        crate::interrupt::DMA8_DMA24,
        crate::interrupt::DMA9_DMA25,
        crate::interrupt::DMA10_DMA26,
        crate::interrupt::DMA11_DMA27,
        crate::interrupt::DMA12_DMA28,
        crate::interrupt::DMA13_DMA29,
        crate::interrupt::DMA14_DMA30,
        crate::interrupt::DMA15_DMA31,
        crate::interrupt::DMA_ERROR,
    ];

    /// The interrupts associated with DMA0
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to DMA0
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMA0: *const RegisterBlock = 0x400e8000 as *const _;
