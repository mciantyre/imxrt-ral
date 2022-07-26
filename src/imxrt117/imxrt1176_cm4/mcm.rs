#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Core Platform Miscellaneous Control Module

use crate::{RORegister, RWRegister};

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// SoC-defined platform revision
pub mod PLREV {

    /// The PLREV\[15:0\] field is specified by an platform input signal to define a software-visible revision number.
    pub mod PLREV {
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

/// Processor core type
pub mod PCT {

    /// This MCM design supports the ARM Cortex M4 core. The following value identifies this core complex.
    pub mod PCT {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1010110001000000: ARM Cortex M4
            pub const PCT_44096: u16 = 0b1010110001000000;
        }
    }
}

/// Memory configuration
pub mod MEMCFG {

    /// TCRAMU size
    pub mod TCRAMUSZ {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (4 bits: 0b1111 << 2)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TCRAML size
    pub mod TCRAMLSZ {
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
}

/// Crossbar Switch (AXBS) Slave Configuration
pub mod PLASC {

    /// Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch's slave input port.
    pub mod ASC {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: A bus slave connection to AXBS input port n is absent
            pub const ASC_0: u16 = 0b00000000;

            /// 0b00000001: A bus slave connection to AXBS input port n is present
            pub const ASC_1: u16 = 0b00000001;
        }
    }
}

/// Crossbar Switch (AXBS) Master Configuration
pub mod PLAMC {

    /// Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port.
    pub mod AMC {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: A bus master connection to AXBS input port n is absent
            pub const AMC_0: u16 = 0b00000000;

            /// 0b00000001: A bus master connection to AXBS input port n is present
            pub const AMC_1: u16 = 0b00000001;
        }
    }
}

/// Control Register
pub mod CR {

    /// Status bits
    pub mod STATUS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Crossbar round-robin arbitration enable
    pub mod CBRR {
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

            /// 0b0: Fixed-priority arbitration
            pub const CBRR_0: u32 = 0b0;

            /// 0b1: Round-robin arbitration
            pub const CBRR_1: u32 = 0b1;
        }
    }

    /// System TCM arbitration priority
    pub mod STCMAP {
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

            /// 0b00: Round robin
            pub const STCMAP_0: u32 = 0b00;

            /// 0b01: Special round robin (favors TCM backoor accesses over the processor)
            pub const STCMAP_1: u32 = 0b01;

            /// 0b10: Fixed priority. Processor has highest, backdoor has lowest
            pub const STCMAP_2: u32 = 0b10;

            /// 0b11: Fixed priority. Backdoor has highest, processor has lowest
            pub const STCMAP_3: u32 = 0b11;
        }
    }

    /// System TCM write protect
    pub mod STCMWP {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Code TCM arbitration priority
    pub mod CTCMAP {
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

            /// 0b00: Round robin
            pub const CTCMAP_0: u32 = 0b00;

            /// 0b01: Special round robin (favors TCM backoor accesses over the processor)
            pub const CTCMAP_1: u32 = 0b01;

            /// 0b10: Fixed priority. Processor has highest, backdoor has lowest
            pub const CTCMAP_2: u32 = 0b10;

            /// 0b11: Fixed priority. Backdoor has highest, processor has lowest
            pub const CTCMAP_3: u32 = 0b11;
        }
    }

    /// Code TCM Write Protect
    pub mod CTCMWP {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Status and Control Register
pub mod ISCR {

    /// Cache write buffer error status
    pub mod CWBER {
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

            /// 0b0: No error
            pub const CWBER_0: u32 = 0b0;

            /// 0b1: Error occurred
            pub const CWBER_1: u32 = 0b1;
        }
    }

    /// FPU invalid operation interrupt status
    pub mod FIOC {
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

            /// 0b0: No interrupt
            pub const FIOC_0: u32 = 0b0;

            /// 0b1: Interrupt occurred
            pub const FIOC_1: u32 = 0b1;
        }
    }

    /// FPU divide-by-zero interrupt status
    pub mod FDZC {
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

            /// 0b0: No interrupt
            pub const FDZC_0: u32 = 0b0;

            /// 0b1: Interrupt occurred
            pub const FDZC_1: u32 = 0b1;
        }
    }

    /// FPU overflow interrupt status
    pub mod FOFC {
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

            /// 0b0: No interrupt
            pub const FOFC_0: u32 = 0b0;

            /// 0b1: Interrupt occurred
            pub const FOFC_1: u32 = 0b1;
        }
    }

    /// FPU underflow interrupt status
    pub mod FUFC {
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

            /// 0b0: No interrupt
            pub const FUFC_0: u32 = 0b0;

            /// 0b1: Interrupt occurred
            pub const FUFC_1: u32 = 0b1;
        }
    }

    /// FPU inexact interrupt status
    pub mod FIXC {
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

            /// 0b0: No interrupt
            pub const FIXC_0: u32 = 0b0;

            /// 0b1: Interrupt occurred
            pub const FIXC_1: u32 = 0b1;
        }
    }

    /// FPU input denormal interrupt status
    pub mod FIDC {
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

            /// 0b0: No interrupt
            pub const FIDC_0: u32 = 0b0;

            /// 0b1: Interrupt occurred
            pub const FIDC_1: u32 = 0b1;
        }
    }

    /// Cache write buffer error enable
    pub mod CWBEE {
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

            /// 0b0: Disable error interrupt
            pub const CWBEE_0: u32 = 0b0;

            /// 0b1: Enable error interrupt
            pub const CWBEE_1: u32 = 0b1;
        }
    }

    /// FPU invalid operation interrupt enable
    pub mod FIOCE {
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

            /// 0b0: Disable interrupt
            pub const FIOCE_0: u32 = 0b0;

            /// 0b1: Enable interrupt
            pub const FIOCE_1: u32 = 0b1;
        }
    }

    /// FPU divide-by-zero interrupt enable
    pub mod FDZCE {
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

            /// 0b0: Disable interrupt
            pub const FDZCE_0: u32 = 0b0;

            /// 0b1: Enable interrupt
            pub const FDZCE_1: u32 = 0b1;
        }
    }

    /// FPU overflow interrupt enable
    pub mod FOFCE {
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

            /// 0b0: Disable interrupt
            pub const FOFCE_0: u32 = 0b0;

            /// 0b1: Enable interrupt
            pub const FOFCE_1: u32 = 0b1;
        }
    }

    /// FPU underflow interrupt enable
    pub mod FUFCE {
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

            /// 0b0: Disable interrupt
            pub const FUFCE_0: u32 = 0b0;

            /// 0b1: Enable interrupt
            pub const FUFCE_1: u32 = 0b1;
        }
    }

    /// FPU inexact interrupt enable
    pub mod FIXCE {
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

            /// 0b0: Disable interrupt
            pub const FIXCE_0: u32 = 0b0;

            /// 0b1: Enable interrupt
            pub const FIXCE_1: u32 = 0b1;
        }
    }

    /// FPU input denormal interrupt enable
    pub mod FIDCE {
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

            /// 0b0: Disable interrupt
            pub const FIDCE_0: u32 = 0b0;

            /// 0b1: Enable interrupt
            pub const FIDCE_1: u32 = 0b1;
        }
    }
}

/// Fault address register
pub mod FADR {

    /// Fault address
    pub mod ADDRESS {
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

/// Fault attributes register
pub mod FATR {

    /// Bus error access type
    pub mod BEDA {
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

            /// 0b0: Instruction
            pub const BEDA_0: u32 = 0b0;

            /// 0b1: Data
            pub const BEDA_1: u32 = 0b1;
        }
    }

    /// Bus error privilege level
    pub mod BEMD {
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

            /// 0b0: User mode
            pub const BEMD_0: u32 = 0b0;

            /// 0b1: Supervisor/privileged mode
            pub const BEMD_1: u32 = 0b1;
        }
    }

    /// Bus error size
    pub mod BESZ {
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

            /// 0b00: 8-bit access
            pub const BESZ_0: u32 = 0b00;

            /// 0b01: 16-bit access
            pub const BESZ_1: u32 = 0b01;

            /// 0b10: 32-bit access
            pub const BESZ_2: u32 = 0b10;
        }
    }

    /// Bus error write
    pub mod BEWT {
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

            /// 0b0: Read access
            pub const BEWT_0: u32 = 0b0;

            /// 0b1: Write access
            pub const BEWT_1: u32 = 0b1;
        }
    }

    /// Bus error master number
    pub mod BEMN {
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

    /// Bus error overrun
    pub mod BEOVR {
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

            /// 0b0: No bus error overrun
            pub const BEOVR_0: u32 = 0b0;

            /// 0b1: Bus error overrun occurred. The FADR and FDR registers and the other FATR bits are not updated to reflect this new bus error.
            pub const BEOVR_1: u32 = 0b1;
        }
    }
}

/// Fault data register
pub mod FDR {

    /// Fault data
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

/// Local Memory Descriptor Register
pub mod LMDR0 {

    /// Control Field 0
    pub mod CF0 {
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

    /// Control Field 1 - for Cache Parity control functions
    pub mod CF1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Memory Type
    pub mod MT {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: code TCM
            pub const MT_0: u32 = 0b000;

            /// 0b001: system TCM
            pub const MT_1: u32 = 0b001;

            /// 0b010: PC Cache
            pub const MT_2: u32 = 0b010;

            /// 0b011: PS Cache
            pub const MT_3: u32 = 0b011;
        }
    }

    /// Read-Only
    pub mod RO {
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

            /// 0b0: Writes to the LMDRn\[7:0\] are allowed.
            pub const RO_0: u32 = 0b0;

            /// 0b1: Writes to the LMDRn\[7:0\] are ignored.
            pub const RO_1: u32 = 0b1;
        }
    }

    /// LMEM Data Path Width. This read-only field defines the width of the local memory.
    pub mod DPW {
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

            /// 0b010: LMEMn 32-bits wide
            pub const DPW_2: u32 = 0b010;

            /// 0b011: LMEMn 64-bits wide
            pub const DPW_3: u32 = 0b011;
        }
    }

    /// Level 1 Cache Ways
    pub mod WY {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No Cache
            pub const WY_0: u32 = 0b0000;

            /// 0b0010: 2-Way Set Associative
            pub const WY_2: u32 = 0b0010;

            /// 0b0100: 4-Way Set Associative
            pub const WY_4: u32 = 0b0100;
        }
    }

    /// LMEM Size
    pub mod LMSZ {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: no LMEMn (0 KB)
            pub const LMSZ_0: u32 = 0b0000;

            /// 0b0001: 1 KB LMEMn
            pub const LMSZ_1: u32 = 0b0001;

            /// 0b0010: 2 KB LMEMn
            pub const LMSZ_2: u32 = 0b0010;

            /// 0b0011: 4 KB LMEMn
            pub const LMSZ_3: u32 = 0b0011;

            /// 0b0100: 8 KB LMEMn
            pub const LMSZ_4: u32 = 0b0100;

            /// 0b0101: 16 KB LMEMn
            pub const LMSZ_5: u32 = 0b0101;

            /// 0b0110: 32 KB LMEMn
            pub const LMSZ_6: u32 = 0b0110;

            /// 0b0111: 64 KB LMEMn
            pub const LMSZ_7: u32 = 0b0111;

            /// 0b1000: 128 KB LMEMn
            pub const LMSZ_8: u32 = 0b1000;

            /// 0b1001: 256 KB LMEMn
            pub const LMSZ_9: u32 = 0b1001;

            /// 0b1010: 512 KB LMEMn
            pub const LMSZ_10: u32 = 0b1010;

            /// 0b1011: 1024 KB LMEMn
            pub const LMSZ_11: u32 = 0b1011;

            /// 0b1100: 2048 KB LMEMn
            pub const LMSZ_12: u32 = 0b1100;

            /// 0b1101: 4096 KB LMEMn
            pub const LMSZ_13: u32 = 0b1101;

            /// 0b1110: 8192 KB LMEMn
            pub const LMSZ_14: u32 = 0b1110;

            /// 0b1111: 16384 KB LMEMn
            pub const LMSZ_15: u32 = 0b1111;
        }
    }

    /// LMEM Size "Hole"
    pub mod LMSZH {
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

            /// 0b0: LMEMn is a power-of-2 capacity.
            pub const LMSZH_0: u32 = 0b0;

            /// 0b1: LMEMn is not a power-of-2, with a capacity is 0.75 * LMSZ.
            pub const LMSZH_1: u32 = 0b1;
        }
    }

    /// Local memory Valid bit. This read-only field defines the validity (presence) of the local memory.
    pub mod V {
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

            /// 0b0: LMEMn is not present.
            pub const V_0: u32 = 0b0;

            /// 0b1: LMEMn is present.
            pub const V_1: u32 = 0b1;
        }
    }
}

/// Local Memory Descriptor Register
pub mod LMDR1 {
    pub use super::LMDR0::CF0;
    pub use super::LMDR0::CF1;
    pub use super::LMDR0::DPW;
    pub use super::LMDR0::LMSZ;
    pub use super::LMDR0::LMSZH;
    pub use super::LMDR0::MT;
    pub use super::LMDR0::RO;
    pub use super::LMDR0::V;
    pub use super::LMDR0::WY;
}

/// Local Memory Descriptor Register
pub mod LMDR2 {
    pub use super::LMDR0::CF0;
    pub use super::LMDR0::CF1;
    pub use super::LMDR0::DPW;
    pub use super::LMDR0::LMSZ;
    pub use super::LMDR0::LMSZH;
    pub use super::LMDR0::MT;
    pub use super::LMDR0::RO;
    pub use super::LMDR0::V;
    pub use super::LMDR0::WY;
}

/// Local Memory Descriptor Register
pub mod LMDR3 {
    pub use super::LMDR0::CF0;
    pub use super::LMDR0::CF1;
    pub use super::LMDR0::DPW;
    pub use super::LMDR0::LMSZ;
    pub use super::LMDR0::LMSZH;
    pub use super::LMDR0::MT;
    pub use super::LMDR0::RO;
    pub use super::LMDR0::V;
    pub use super::LMDR0::WY;
}

/// LMEM Parity & ECC Control Register
pub mod LMPECR {

    /// Enable RAM ECC Non-correctable Reporting
    pub mod ERNCR {
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

            /// 0b0: reporting enabled
            pub const ERNCR_0: u32 = 0b0;

            /// 0b1: reporting disabled
            pub const ERNCR_1: u32 = 0b1;
        }
    }

    /// Enable RAM Non-correctable ECC Interrupt
    pub mod ERNCI {
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

            /// 0b0: Interrupt is disabled
            pub const ERNCI_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const ERNCI_1: u32 = 0b1;
        }
    }

    /// Enable RAM ECC 1-bit Reporting
    pub mod ER1BR {
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

            /// 0b0: reporting enabled
            pub const ER1BR_0: u32 = 0b0;

            /// 0b1: reporting disabled
            pub const ER1BR_1: u32 = 0b1;
        }
    }

    /// Enable RAM ECC 1-bit Interrupt
    pub mod ER1BI {
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

            /// 0b0: Interrupt is disabled
            pub const ER1BI_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const ER1BI_1: u32 = 0b1;
        }
    }

    /// Enable Cache Parity Reporting
    pub mod ECPR {
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

            /// 0b0: reporting enabled
            pub const ECPR_0: u32 = 0b0;

            /// 0b1: reporting disabled
            pub const ECPR_1: u32 = 0b1;
        }
    }

    /// Enable Cache Parity IRQ
    pub mod ECPI {
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

            /// 0b0: enabled
            pub const ECPI_0: u32 = 0b0;

            /// 0b1: disabled
            pub const ECPI_1: u32 = 0b1;
        }
    }
}

/// LMEM Parity & ECC Interrupt Register
pub mod LMPEIR {

    /// ENCn = ECC Non-correctable Error n
    pub mod ENC {
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

    /// E1Bn = ECC 1-bit Error n
    pub mod E1B {
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

    /// Parity Error
    pub mod PE {
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

    /// Parity or ECC Error Location
    pub mod PEELOC {
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

    /// Valid bit
    pub mod V {
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

/// LMEM Fault Address Register
pub mod LMFAR {

    /// ECC Fault Address
    pub mod EFADD {
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

/// LMEM Fault Attribute Register
pub mod LMFATR {

    /// Parity/ECC Fault Protection FATR\[3\] is Cacheable: 0=Non-cacheable, 1=Cacheable FATR\[2\] is Bufferable: 0=Non-bufferable, 1=Bufferable FATR\[1\] is Mode: 0=User mode, 1=Supervisor mode FATR\[0\] is Type: 0=I-Fetch, 1=Data
    pub mod PEFPRT {
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

    /// Parity/ECC Fault Master Size 3'b000 = 8-bit access 3'b001 = 16-bit access 3'b010 = 32-bit access 3'b011 = 64-bit access 3'b1xx = Reserved
    pub mod PEFSIZE {
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

    /// Parity/ECC Fault Write
    pub mod PEFW {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Parity/ECC Fault Master Number
    pub mod PEFMST {
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

    /// ID of the word which has ECC error
    pub mod WORDID {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overrun
    pub mod OVR {
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

/// LMEM Fault Data High Register
pub mod LMFDHR {

    /// Parity or ECC Fault Data High
    pub mod PEFDH {
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

/// LMEM Fault Data Low Register
pub mod LMFDLR {

    /// Parity or ECC Fault Data Low
    pub mod PEFDL {
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
#[repr(C)]
pub struct RegisterBlock {
    /// SoC-defined platform revision
    pub PLREV: RORegister<u16>,

    /// Processor core type
    pub PCT: RORegister<u16>,

    /// Memory configuration
    pub MEMCFG: RORegister<u32>,

    /// Crossbar Switch (AXBS) Slave Configuration
    pub PLASC: RORegister<u16>,

    /// Crossbar Switch (AXBS) Master Configuration
    pub PLAMC: RORegister<u16>,

    /// Control Register
    pub CR: RWRegister<u32>,

    /// Interrupt Status and Control Register
    pub ISCR: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// Fault address register
    pub FADR: RORegister<u32>,

    /// Fault attributes register
    pub FATR: RORegister<u32>,

    /// Fault data register
    pub FDR: RORegister<u32>,

    _reserved2: [u32; 245],

    /// Local Memory Descriptor Register
    pub LMDR0: RWRegister<u32>,

    /// Local Memory Descriptor Register
    pub LMDR1: RWRegister<u32>,

    /// Local Memory Descriptor Register
    pub LMDR2: RWRegister<u32>,

    /// Local Memory Descriptor Register
    pub LMDR3: RWRegister<u32>,

    _reserved3: [u32; 28],

    /// LMEM Parity & ECC Control Register
    pub LMPECR: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// LMEM Parity & ECC Interrupt Register
    pub LMPEIR: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// LMEM Fault Address Register
    pub LMFAR: RORegister<u32>,

    /// LMEM Fault Attribute Register
    pub LMFATR: RWRegister<u32>,

    _reserved6: [u32; 2],

    /// LMEM Fault Data High Register
    pub LMFDHR: RORegister<u32>,

    /// LMEM Fault Data Low Register
    pub LMFDLR: RORegister<u32>,
}
pub struct ResetValues {
    pub PLREV: u16,
    pub PCT: u16,
    pub MEMCFG: u32,
    pub PLASC: u16,
    pub PLAMC: u16,
    pub CR: u32,
    pub ISCR: u32,
    pub FADR: u32,
    pub FATR: u32,
    pub FDR: u32,
    pub LMDR0: u32,
    pub LMDR1: u32,
    pub LMDR2: u32,
    pub LMDR3: u32,
    pub LMPECR: u32,
    pub LMPEIR: u32,
    pub LMFAR: u32,
    pub LMFATR: u32,
    pub LMFDHR: u32,
    pub LMFDLR: u32,
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

/// The MCM peripheral instance.
#[cfg(not(feature = "doc"))]
pub type MCM = Instance<0>;

/// The MCM peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type MCM = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct MCM {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for MCM {}
impl crate::Valid for MCM {
    fn take() -> Option<Self> {
        <MCM>::take()
    }
    fn release(self) {
        <MCM>::release(self);
    }
    unsafe fn steal() -> Self {
        <MCM>::steal()
    }
}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static MCM_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the MCM peripheral instance
#[cfg(not(feature = "nosync"))]
impl MCM {
    const INSTANCE: Self = Self {
        addr: 0xe0080000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in MCM
    pub const reset: ResetValues = ResetValues {
        PLREV: 0x00000000,
        PCT: 0x0000AC40,
        MEMCFG: 0x00000000,
        PLASC: 0x0000001F,
        PLAMC: 0x0000001F,
        CR: 0x00000000,
        ISCR: 0x00020000,
        FADR: 0x00000000,
        FATR: 0x00000000,
        FDR: 0x00000000,
        LMDR0: 0x00000000,
        LMDR1: 0x00000000,
        LMDR2: 0x00000000,
        LMDR3: 0x00000000,
        LMPECR: 0x00300003,
        LMPEIR: 0x00000000,
        LMFAR: 0x00000000,
        LMFATR: 0x00000000,
        LMFDHR: 0x00000000,
        LMFDLR: 0x00000000,
    };

    /// Safe access to MCM
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
        let taken = MCM_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to MCM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = MCM_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal MCM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        MCM_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl MCM {
    /// The interrupts associated with MCM
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with MCM
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to MCM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const MCM: *const RegisterBlock = 0xe0080000 as *const _;
